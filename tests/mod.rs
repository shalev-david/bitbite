#[cfg(test)]
mod test {

    use std::ops::{Deref, DerefMut};

    use bitbite::{Bitbite, Flag};

    /// In this test I'm trying to work with a byte from the Nes Cartridge header
    struct NesCartridgeF6(pub u8);
    impl NesCartridgeF6 {
        pub const MIRRORING: Flag<u8> = Flag::new(0b0000_0001);
        pub const HAS_BATTERY_RAM: Flag<u8> = Flag::new(0b0000_0010);
        pub const TRAINER: Flag<u8> = Flag::new(0b0000_0100);
        pub const FOUR_SCREEN: Flag<u8> = Flag::new(0b0000_1000);
        pub const LOWER_MAPPER: Flag<u8> = Flag::new(0b1111_0000);
    }

    impl Deref for NesCartridgeF6 {
        type Target = u8;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for NesCartridgeF6 {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl Bitbite for NesCartridgeF6 {
        type Unit = u8;
    }

    struct U16Flags(pub u16);
    impl U16Flags {
        pub const FIRST_NIBBLE: Flag<u16> = Flag::new(0b0000_0000_0000_1111);
        pub const MIDDLE_NIBBLE: Flag<u16> = Flag::new(0b0000_0011_1100_0000);
        pub const LAST_NIBBLE: Flag<u16> = Flag::new(0b1111_0000_0000_0000);
    }

    impl Deref for U16Flags {
        type Target = u16;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for U16Flags {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    impl Bitbite for U16Flags {
        type Unit = u16;
    }

    #[test]
    fn test_u8() {
        let mut t = NesCartridgeF6(0b0100_1011);
        assert_eq!(t.get_flag(&NesCartridgeF6::MIRRORING), 0b1);
        assert_eq!(t.get_flag(&NesCartridgeF6::HAS_BATTERY_RAM), 0b1);
        assert_eq!(t.get_flag(&NesCartridgeF6::TRAINER), 0b0);
        assert_eq!(t.get_flag(&NesCartridgeF6::FOUR_SCREEN), 0b1);
        assert_eq!(t.get_flag(&NesCartridgeF6::LOWER_MAPPER), 0b0100);
        t.set_flag(0b0010, &NesCartridgeF6::LOWER_MAPPER);
        assert_eq!(t.get_flag(&NesCartridgeF6::LOWER_MAPPER), 0b0010);
        t.set_on(0b0100, &NesCartridgeF6::LOWER_MAPPER);
        assert_eq!(t.get_flag(&NesCartridgeF6::LOWER_MAPPER), 0b0110);
    }
    #[test]
    fn test_u16() {
        let mut t = U16Flags(0b0110_0010_1000_0011);
        assert_eq!(t.get_flag(&U16Flags::FIRST_NIBBLE), 0b0011);
        assert_eq!(t.get_flag(&U16Flags::MIDDLE_NIBBLE), 0b1010);
        assert_eq!(t.get_flag(&U16Flags::LAST_NIBBLE), 0b0110);
        t.set_flag(0b0101, &U16Flags::MIDDLE_NIBBLE);
        assert_eq!(t.get_flag(&U16Flags::MIDDLE_NIBBLE), 0b0101);
        t.set_on(0b0010, &U16Flags::MIDDLE_NIBBLE);
        assert_eq!(t.get_flag(&U16Flags::MIDDLE_NIBBLE), 0b0111);
    }
}
