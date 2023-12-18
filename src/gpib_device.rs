use crate::hp606n_x::Hp606nX;

pub enum GpibDevice{
    Hp606nX(Hp606nX),
}