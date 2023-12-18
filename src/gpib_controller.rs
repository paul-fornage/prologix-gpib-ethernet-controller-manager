use std::io::{Read, Write};
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::time::Duration;
use crate::errors::BatTestError;
use crate::gpib_device::GpibDevice;

pub struct GpibController{
    /// TcpStream where the controller is
    tcp_stream: TcpStream,
    /// the GPIB address the controller currently has selected.
    ///
    /// This is tracked locally to avoid selecting a new address everytime a command is sent if you
    /// are only using one device. 255 means the address has not yet been set although this should never appear.
    current_gpib_addr: u8,
    ///list of devices on this bus
    devices: Vec<GpibDevice>
}

impl GpibController{
    pub fn send_raw_data(&mut self, message: &str) -> Result<usize, BatTestError>{
        self.tcp_stream.write(message.as_ref()).map_err(|e| {BatTestError::TcpIoError(e)})
    }

    pub fn read_data(&mut self) -> Result<String, BatTestError>{
        let mut buffer_vec: Vec<u8> = Vec::with_capacity(32);
        self.tcp_stream.read_to_end(&mut buffer_vec).map_err(|e| {BatTestError::TcpIoError(e)})?;
        Ok(String::from_utf8(buffer_vec)?)
    }

    pub fn try_new_from(ip_addr: IpAddr) -> Result<GpibController, BatTestError>{
        let sock_addr: SocketAddr = SocketAddr::from((ip_addr, 1234u16));
        let mut temp_controller = GpibController{
            tcp_stream: TcpStream::connect_timeout(&sock_addr, Duration::from_millis(500)).map_err(|e| {BatTestError::TcpIoError(e)})?,
            current_gpib_addr: 255,
            devices: vec![],
        };
        temp_controller.send_raw_data("++addr\n")?;
        let addr: u8 = temp_controller.read_data()?.parse()?;
        temp_controller.current_gpib_addr = addr;
        temp_controller.send_raw_data("++auto 1;++mode 1\n")?;

        Ok(temp_controller)
    }

}

// Commands for the controller: (not GPIB protocol)

//The following commands are available:
// ++addr 0-30 [96-126]  -- specify GPIB address
// ++addr                -- query GPIB address
// ++auto 0|1            -- enable (1) or disable (0) read-after-write
// ++auto                -- query read-after-write setting
// ++clr                 -- issue device clear
// ++eoi 0|1             -- enable (1) or disable (0) EOI with last byte
// ++eoi                 -- query eoi setting
// ++eos 0|1|2|3         -- EOS terminator - 0:CR+LF, 1:CR, 2:LF, 3:None
// ++eos                 -- query eos setting
// ++eot_enable 0|1      -- enable (1) or disable (0) appending eot_char on EOI
// ++eot_enable          -- query eot_enable setting
// ++eot_char <char>     -- specify eot character in decimal
// ++eot_char            -- query eot_char character
// ++ifc                 -- issue interface clear
// ++loc                 -- set device to local
// ++lon                 -- enable (1) or disable (0) listen only mode
// ++mode 0|1            -- set mode - 0:DEVICE, 1:CONTROLLER
// ++mode                -- query current mode
// ++read [eoi|<char>]   -- read until EOI, <char>, or timeout
// ++read_tmo_ms 1-3000  -- set read timeout in millisec
// ++read_tmo_ms         -- query timeout
// ++rst                 -- reset controller
// ++savecfg 0|1         -- enable (1) or disable (0) saving configuration to EPROM
// ++savecfg             -- query savecfg setting
// ++spoll               -- serial poll currently addressed device
// ++spoll 0-30 [96-126] -- serial poll device at specified address
// ++srq                 -- query SRQ status
// ++status 0-255        -- specify serial poll status byte
// ++status              -- query serial poll status byte
// ++trg                 -- issue device trigger
// ++ver                 -- query controller version
// ++help                -- display this help