# Bitbite derive
The Bitbite derive is used to save you a bit of code and make the usage cleaner. \
For the cleanest code I recommend using [deref_derive](https://crates.io/crates/derive_deref)
# Usage:
```rust
use bitbite::*;
use bitbite_derive::Bitbite;
use deref_derive::*;

#[derive(Bitbite, Deref, DerefMut)]
struct Header(pub u8);

impl Header {
    pub const FIRST_NIBBLE_FLAG : Flag<u8> = Flag::new(0b0000_1111);
}

let h = Header(0b1100_1010);
assert_eq!(h.get_flag(&Header::FIRST_NIBBLE_FLAG), 0b1010);
```
# Multi fielded structs
In case you have more than one field you can use the `bitbite` attribute
# Usage:
```rust
use bitbite::*;
use bitbite_derive::Bitbite;
use deref_derive::*;

#[derive(Bitbite, Deref, DerefMut)]
#[bitbite(u16)]
struct Header {
    #[deref]
    pub field: u16,
    pub count:u8
}

impl Header {
    pub const MIDDLE_NIBBLE : Flag<u16> = Flag::new(0b0000_0011_1100_0000);
}

let h = Header {
    field: 0b0010_1010_1000_0110,
    count : 0,
};
assert_eq!(h.get_flag(&Header::MIDDLE_NIBBLE), 0b1010);
```