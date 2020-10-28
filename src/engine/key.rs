#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Key {
    Key0 = 0x0,
    Key1 = 0x1,
    Key2 = 0x2,
    Key3 = 0x3,
    Key4 = 0x4,
    Key5 = 0x5,
    Key6 = 0x6,
    Key7 = 0x7,
    Key8 = 0x8,
    Key9 = 0x9,
    KeyA = 0xa,
    KeyB = 0xb,
    KeyC = 0xc,
    KeyD = 0xd,
    KeyE = 0xe,
    KeyF = 0xf,
}

impl Key {
    pub fn from(key: minifb::Key) -> Option<Self> {
        match key {
            minifb::Key::Key1 => Some(Self::Key1),
            minifb::Key::Key2 => Some(Self::Key2),
            minifb::Key::Key3 => Some(Self::Key3),
            minifb::Key::Key4 => Some(Self::KeyC),

            minifb::Key::Q => Some(Self::Key4),
            minifb::Key::W => Some(Self::Key5),
            minifb::Key::E => Some(Self::Key6),
            minifb::Key::R => Some(Self::KeyD),

            minifb::Key::A => Some(Self::Key7),
            minifb::Key::S => Some(Self::Key8),
            minifb::Key::D => Some(Self::Key9),
            minifb::Key::F => Some(Self::KeyE),

            minifb::Key::Z => Some(Self::KeyA),
            minifb::Key::X => Some(Self::Key0),
            minifb::Key::C => Some(Self::KeyB),
            minifb::Key::V => Some(Self::KeyF),

            _ => None,
        }
    }
}
