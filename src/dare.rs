use crate::magnetic_fields::MagneticField;

pub type Hertz = f64;
pub trait <Split: MagneticField>Dare<MagneticField> {
     fn access(&self, mf: Split, hz: Hertz) -> Split;
}

pub struct Daddy;

impl <Split>Dare<MagneticField> for Daddy where Split: MagneticField{
    fn access(&self, sp: Split, hz: Hertz) -> Daddy {
        sp.access(sp, 101)
    }
}
