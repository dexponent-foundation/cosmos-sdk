use ixc_core::{Context, Result};
use ixc_schema::state_object::ObjectKey;
use crate::Map;

/// A map from keys to 128-bit unsigned integers.
pub struct UIntMap<K, V: UInt> {
    map: Map<K, V>,
}

pub trait UInt: Sized {
    fn add(self, other: Self) -> Option<Self>;
    fn sub(self, other: Self) -> Option<Self>;
}

impl UInt for u64 {
    fn add(self, other: Self) -> Option<Self> {
        self.checked_add(other)
    }

    fn sub(self, other: Self) -> Option<Self> {
        self.checked_sub(other)
    }
}
impl UInt for u128 {
    fn add(self, other: Self) -> Option<Self> {
        self.checked_add(other)
    }

    fn sub(self, other: Self) -> Option<Self> {
        self.checked_sub(other)
    }
}

impl<'a, K: ObjectKey, V: UInt> UIntMap<K, V> {
    /// Gets the current value for the given key, defaulting always to 0.
    pub fn get(&self, ctx: &Context, key: K::In<'_>) -> Result<u128> {
        // let value = self.map.get(ctx, key)?;
        // Ok(value.unwrap_or(0))
        todo!()
    }

    /// Adds the given value to the current value for the given key.
    pub fn add(&self, ctx: &mut Context, key: K::In<'_>, value: u128) -> Result<u128> {
        todo!()
    }

    /// Subtracts the given value from the current value for the given key,
    /// returning an error if the subtraction would result in a negative value.
    pub fn safe_sub(&self, ctx: &mut Context, key: K::In<'_>, value: u128) -> Result<u128> {
        todo!()
    }
}
