#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, strum::Display, strum::IntoStaticStr)]
pub enum Target {
    #[strum(serialize = "mayon::backend")]
    Backend,
}
