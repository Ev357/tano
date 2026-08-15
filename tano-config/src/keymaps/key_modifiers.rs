use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
    pub struct KeyModifiers: u8 {
        const SHIFT   = 1 << 0;
        const ALT     = 1 << 1;
        const CONTROL = 1 << 2;
        const SUPER   = 1 << 3;
        const HYPER   = 1 << 4;
        const META    = 1 << 5;
    }
}
