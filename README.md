# Bitbite
Bitbite is a simple trait that would help you interact bytes with flags easily

# How to use
All you need to do is declare the flags and implement the trait.
```rust
use bitbite::{Flag, Bitbite};

struct NesCartridgeF6(pub u8);
impl NesCartridgeF6 {
  pub const LOWER_MAPPER: Flag = Flag::new(0b1111_0000);
}

impl Bitbite<u8> for NesCartridgeF6 { }
 
let mut t = NesCartridgeF6(0b0110_0000);
let lower_mapper = t.get_flag(&NesCartridgeF6::LOWER_MAPPER);
assert_eq!(lower_mapper, 0b0110);
```

 Getting the flag value will always be shifted so you won't need to shift the data yourself

# Bigger flags
The trait `BitBite` and the struct `Flag` are generics and can hold any primitive integer, so you could have a struct that hold 1,2,4 bytes of data and still use the flags seamlessly.

## Mutability  
You can now edit your bytes with flags with ease, you can use `set_flag` to override the entire flag's mask and change it as you please. \
 And in case you don't want to override the previous values you can `set_on` different bits to hold their state as it is.
 