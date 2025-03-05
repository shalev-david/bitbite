# Bitbite &emsp; [![Build Status]][actions] [![Latest Version]][crates.io]

[Build Status]: https://img.shields.io/github/actions/workflow/status/shalev-david/bitbite/rust.yml?branch=main
[actions]: https://github.com/shalev-david/bitbite/actions?query=branch%3Amain
[Latest Version]: https://img.shields.io/crates/v/bitbite.svg
[Latest Version Derive]: https://img.shields.io/crates/v/bitbite_derive.svg
[crates.io]: https://crates.io/crates/bitbite

Bitbite is a simple trait that would help you interact bytes with flags easily

- [How to use](#how-to-use)
- [Bigger flags](#bigger-flags)
- [Mutability](#mutability)
- [Derive](#derive)

# How to use
All you need to do is declare the flags and implement the trait.
```rust
use bitbite::{Flag, Bitbite};
use std::ops::{Deref, DerefMut};

struct NesCartridgeF6(pub u8);
impl NesCartridgeF6 {
  pub const LOWER_MAPPER: Flag<u8> = Flag::<u8>::new(0b1111_0000);
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
 
let mut t = NesCartridgeF6(0b0110_0000);
let lower_mapper = t.get_flag(&NesCartridgeF6::LOWER_MAPPER);
assert_eq!(lower_mapper, 0b0110);
```

 Getting the flag value will always be shifted so you won't need to shift the data yourself

# Bigger flags
The trait `BitBite` and the struct `Flag` are generics and can hold any primitive integer, so you could have a struct that hold 1,2,4 bytes of data and still use the flags seamlessly.

# Mutability  
You can now edit your bytes with flags with ease, you can use `set_flag` to override the entire flag's mask and change it as you please. \
 And in case you don't want to override the previous values you can `set_on` different bits to hold their state as it is.

# Derive &emsp; ![Latest Version Derive]
For easier use you can add the [bitbite_derive](https://crates.io/crates/bitbite_derive) crate \
I also recommend using [defer_derive](https://crates.io/crates/deref_derive) for maximum ease
```rust
use bitbite::*;
use bitbite_derive::Bitbite;
use deref_derive::*;

#[derive(Bitbite, Deref, DerefMut)]
struct Header(pub u8);
```