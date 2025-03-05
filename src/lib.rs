//! # Bitbite
//! Bitbite is a simple trait that would help you interact bytes with flags easily
//!
//! # How to use
//! All you need to do is declare the flags and implement the trait.
//! ```rust
//! use bitbite::{Flag, Bitbite};
//! use std::ops::{Deref, DerefMut};
//!
//! struct NesCartridgeF6(pub u8);
//! impl NesCartridgeF6 {
//!   pub const LOWER_MAPPER: Flag<u8> = Flag::<u8>::new(0b1111_0000);
//! }
//!
//!
//! impl Deref for NesCartridgeF6 {
//!     type Target = u8;
//!
//!     fn deref(&self) -> &Self::Target {
//!       &self.0    
//!     }       
//! }
//!
//! impl DerefMut for NesCartridgeF6 {
//!     fn deref_mut(&mut self) -> &mut Self::Target {
//!       &mut self.0    
//!     }       
//! }
//!
//! impl Bitbite for NesCartridgeF6 {
//!     type Unit = u8;
//! }
//!  
//! let mut t = NesCartridgeF6(0b0110_0000);
//! let lower_mapper = t.get_flag(&NesCartridgeF6::LOWER_MAPPER);
//! assert_eq!(lower_mapper, 0b0110);
//! ```
//!
//!  Getting the flag value will always be shifted so you won't need to shift the data yourself
//!
//! # Bigger flags
//! The trait `BitBite` and the struct `Flag` are generics and can hold any primitive integer, so you could have a struct that hold 1,2,4 bytes of data and still use the flags seamlessly.
//!
//! ## Mutability  
//! You can now edit your bytes with flags with ease, you can use `set_flag` to override the entire flag's mask and change it as you please. \
//!  And in case you don't want to override the previous values you can `set_on` different bits to hold their state as it is.
use num_traits::PrimInt;
use std::ops::DerefMut;

#[derive(Debug)]
pub struct Flag<T: PrimInt> {
    pub(crate) mask: T,
}

impl<T: PrimInt> Flag<T>
where
    T: PrimInt,
{
    pub const fn new(mask: T) -> Self {
        Self { mask }
    }

    pub(crate) fn shift(&self) -> usize {
        self.mask.trailing_zeros() as usize
    }
}

/// [Bitbite] Bitbite is a simple trait that would help you interact bytes with flags easily
pub trait Bitbite: DerefMut<Target = Self::Unit> {
    type Unit: PrimInt;
    /// `get_flag` will return the value of the flag shifted
    /// ## Example
    /// ```
    ///  # use std::ops::{Deref, DerefMut};
    ///  use bitbite::{Flag, Bitbite};
    ///  struct NesCartridgeF6(pub u8);
    ///  impl NesCartridgeF6 {
    ///    pub const LOWER_MAPPER: Flag<u8> = Flag::<u8>::new(0b1111_0000);
    ///  }
    ///
    ///  impl Bitbite for NesCartridgeF6 {
    ///     type Unit = u8;
    ///  }
    ///
    ///  # impl Deref for NesCartridgeF6 {
    ///  #     type Target = u8;
    ///  #     fn deref(&self) -> &Self::Target{
    ///  #       &self.0    
    ///  #     }       
    ///  # }
    ///  #
    ///  # impl DerefMut for NesCartridgeF6 {
    ///  #     fn deref_mut(&mut self) -> &mut Self::Target {
    ///  #       &mut self.0    
    ///  #     }       
    ///  # }
    ///  
    ///  let mut t = NesCartridgeF6(0b0110_0000);
    ///  let lower_mapper = t.get_flag(&NesCartridgeF6::LOWER_MAPPER);
    ///  assert_eq!(lower_mapper, 0b0110);
    /// ```
    fn get_flag(&self, flag: &Flag<Self::Unit>) -> Self::Unit {
        (*self.deref() & flag.mask) >> flag.shift()
    }

    /// `set_flag` will set the flag to the given value, overriding the previous value
    /// ## Example
    /// ```
    ///  # use std::ops::{Deref, DerefMut};
    ///  use bitbite::{Flag, Bitbite};
    ///
    ///  struct NesCartridgeF6(pub u8);
    ///  impl NesCartridgeF6 {
    ///    pub const LOWER_MAPPER: Flag<u8> = Flag::<u8>::new(0b1111_0000);
    ///  }
    ///  
    ///  # impl Deref for NesCartridgeF6 {
    ///  #     type Target = u8;
    ///  #     fn deref(&self) -> &Self::Target{
    ///  #       &self.0    
    ///  #     }       
    ///  # }
    ///  #
    ///  # impl DerefMut for NesCartridgeF6 {
    ///  #     fn deref_mut(&mut self) -> &mut Self::Target {
    ///  #       &mut self.0    
    ///  #     }       
    ///  # }
    ///
    ///  impl Bitbite for NesCartridgeF6 {
    ///      type Unit = u8;
    ///  }
    ///  
    ///  let mut t = NesCartridgeF6(0b0100_0000);
    ///  t.set_flag(0b0010, &NesCartridgeF6::LOWER_MAPPER);
    ///  assert_eq!(t.get_flag(&NesCartridgeF6::LOWER_MAPPER), 0b0010);
    /// ```
    fn set_flag(&mut self, value: Self::Unit, flag: &Flag<Self::Unit>) {
        self.reset_flag(flag);
        let inner = self.deref_mut();
        *inner = *inner | (value << flag.shift())
    }

    /// `set_on` will set the given bits in the flag on without changing it's previous state
    /// ## Example
    /// ```
    ///  # use std::ops::{Deref, DerefMut};
    ///  use bitbite::{Flag, Bitbite};
    ///
    ///  struct NesCartridgeF6(pub u8);
    ///  impl NesCartridgeF6 {
    ///    pub const LOWER_MAPPER: Flag<u8> = Flag::<u8>::new(0b1111_0000);
    ///  }
    ///
    ///  # impl Deref for NesCartridgeF6 {
    ///  #     type Target = u8;
    ///  #     fn deref(&self) -> &Self::Target{
    ///  #       &self.0    
    ///  #     }       
    ///  # }
    ///  #
    ///  # impl DerefMut for NesCartridgeF6 {
    ///  #     fn deref_mut(&mut self) -> &mut Self::Target {
    ///  #       &mut self.0    
    ///  #     }       
    ///  # }
    ///
    ///  impl Bitbite for NesCartridgeF6 {
    ///      type Unit = u8;
    ///  }
    ///  
    ///  let mut t = NesCartridgeF6(0b0100_0000);
    ///  t.set_on(0b0010, &NesCartridgeF6::LOWER_MAPPER);
    ///  assert_eq!(t.get_flag(&NesCartridgeF6::LOWER_MAPPER), 0b0110);
    /// ```
    fn set_on(&mut self, value: Self::Unit, flag: &Flag<Self::Unit>) {
        let inner = self.deref_mut();
        *inner = *inner | (value << flag.shift())
    }

    /// `reset_flag` will reset all the bits of the specified flag to 0
    /// ## Example
    /// ```
    /// # use std::ops::{Deref, DerefMut};
    ///  use bitbite::{Flag, Bitbite};
    ///
    ///  struct NesCartridgeF6(pub u8);
    ///  impl NesCartridgeF6 {
    ///    pub const LOWER_MAPPER: Flag<u8> = Flag::<u8>::new(0b1111_0000);
    ///  }
    ///
    ///  # impl Deref for NesCartridgeF6 {
    ///  #     type Target = u8;
    ///  #     fn deref(&self) -> &Self::Target {
    ///  #       &self.0    
    ///  #     }       
    ///  # }
    ///  #
    ///  # impl DerefMut for NesCartridgeF6 {
    ///  #     fn deref_mut(&mut self) -> &mut Self::Target {
    ///  #       &mut self.0    
    ///  #     }       
    ///  # }
    ///
    ///  impl Bitbite for NesCartridgeF6 {
    ///      type Unit = u8;
    ///  }
    ///  
    ///  let mut t = NesCartridgeF6(0b0110_0000);
    ///  t.reset_flag(&NesCartridgeF6::LOWER_MAPPER);
    ///  assert_eq!(t.get_flag(&NesCartridgeF6::LOWER_MAPPER), 0b0000);
    /// ```
    fn reset_flag(&mut self, flag: &Flag<Self::Unit>) {
        let inner = self.deref_mut();
        *inner = (*inner) & !flag.mask;
    }

    /// `set_off` will turn off specific bits of the flag \
    /// You pass in the bits you want to set off \
    /// For example we have this flag `0b1110`, and we want to set off the second bit from the right we would have to pass \
    /// `set_off(0b0100)` - this will turn off only the second bit and all other bits will remain the same \
    /// **Output** - `0b1011`
    ///
    /// ## Usage:
    /// ```
    /// # use std::ops::{Deref, DerefMut};
    ///  use bitbite::{Flag, Bitbite};
    ///
    ///  struct NesCartridgeF6(pub u8);
    ///  impl NesCartridgeF6 {
    ///    pub const LOWER_MAPPER: Flag<u8> = Flag::<u8>::new(0b1111_0000);
    ///  }
    ///
    ///  # impl Deref for NesCartridgeF6 {
    ///  #     type Target = u8;
    ///  #     fn deref(&self) -> &Self::Target {
    ///  #       &self.0    
    ///  #     }       
    ///  # }
    ///  #
    ///  # impl DerefMut for NesCartridgeF6 {
    ///  #     fn deref_mut(&mut self) -> &mut Self::Target {
    ///  #       &mut self.0    
    ///  #     }       
    ///  # }
    ///
    ///  impl Bitbite for NesCartridgeF6 {
    ///      type Unit = u8;
    ///  }
    ///  
    ///  let mut t = NesCartridgeF6(0b1110_0000);
    ///  t.set_off(0b0100, &NesCartridgeF6::LOWER_MAPPER);
    ///  assert_eq!(t.get_flag(&NesCartridgeF6::LOWER_MAPPER), 0b1010);
    /// ```
    fn set_off(&mut self, value: Self::Unit, flag: &Flag<Self::Unit>) {
        let new_value = !value & self.get_flag(flag);
        self.set_flag(new_value, flag);
    }
}
