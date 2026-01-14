#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, strum::IntoStaticStr, strum::Display)]
pub enum TargetPlatform {
    Wayland = 1,
    Xcb = 2,
    Xlib = 3,
    Win32 = 4,
    Android = 5,
    Metal = 6,
}
