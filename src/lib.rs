//! An implementation of a 64-bit [DualHashKey].
//! 
//! For use with hierarchical ordered collections,
//! to quickly find subkeys and permit range queries.

pub use core::num::NonZeroU64;

/// The fnv1a hasher used internally.
pub use const_fnv1a_hash;

/// Shorthand alias for [DualHashKey].
pub type DHK = DualHashKey;

/// A mask for the low-half of a [DualHashKey].
/// 
/// The maximum value of an [u32], zero-extended into a [u64].
pub const LOW_MASK: u64 = u32::MAX as u64;

/// A mask for the high-half of a [DualHashKey].
pub const HIGH_MASK: u64 = !(u32::MAX as u64);

/// The offset of the high-half in a [DualHashKey].
pub const HIGH_SHIFT: u64 = 32;

/// The lowest possible [DualHashKey].
pub const MIN: DualHashKey = DualHashKey {hash: NonZeroU64::MIN};

/// The highest possible [DualHashKey].
pub const MAX: DualHashKey = DualHashKey {hash: NonZeroU64::MAX};

#[cfg(test)]
mod test;

/// A 64-bit key made of two hashes, whose raw value is never zero.
/// 
/// The HIGH-half source should be a superset-or-parent of the LOW-half source,  
/// such that any `ORDEREDMAP<DualHashKey, _>` can be walked in hierarchical order,  
/// by performing range-queries using the [`Self::get_hash_low_half_min`] and [`Self::get_hash_low_half_max`] functions.
/// 
/// For example, passing in `root/mid/low` as HIGH and `root/mid/low/name` as LOW,
/// results in a dual-hash of `E05F2E55.0CB0216D`.
/// 
/// 
/// Print formats:
/// - Display: `DualHashKey({HIGH:0>8X}.{LOW:0>8X})`
/// - Debug: `{HIGH:0>8X}.{LOW:0>8X}`
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct DualHashKey {
    pub hash: NonZeroU64
}

/// Hash-implementation: Writes the hash via `write_u64`. That's it.
/// 
/// One should use a passthru/nohash-hasher when using the [DualHashKey].
impl core::hash::Hash for DualHashKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.get_hash_raw())
    }
}

impl core::fmt::Debug for DualHashKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(
            format_args!("{:0>8X?}.{:0>8X?}",
                self.get_hash_high_half(),
                self.get_hash_low_half()
            )
        )
    }
}

impl core::fmt::Display for DualHashKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(
            format_args!("DualHashKey({:0>8X?}.{:0>8X?})",
                self.get_hash_high_half(),
                self.get_hash_low_half()
            )
        )
    }
}

/// Functions/Methods for the dual form of the DHK.
impl DualHashKey {
    /// Creates a new [DualHashKey] from the pair of high and low sequences of bytes.
    pub const fn from_dual_bytes(high: &[u8], low: &[u8]) -> Option<Self> {
        Self::from_raw_dual(
            const_fnv1a_hash::fnv1a_hash_32(high, None),
            const_fnv1a_hash::fnv1a_hash_32(low, None)
        )
    }
    
    /// Creates a new [DualHashKey] from the pair of high and low strings.
    pub const fn from_dual_str(high: &str, low: &str) -> Option<Self> {
        Self::from_raw_dual(
            const_fnv1a_hash::fnv1a_hash_str_32(high),
            const_fnv1a_hash::fnv1a_hash_str_32(low)
        )
    }
    
    /// Creates a new [DualHashKey] from the high sequence of bytes, with the low-half zeroed.
    pub const fn from_high_bytes(high: &[u8]) -> Option<Self> {
        Self::from_raw_high(const_fnv1a_hash::fnv1a_hash_32(high, None))
    }
    
    /// Creates a new [DualHashKey] from the high string, with the low-half zeroed.
    pub const fn from_high_str(high: &str) -> Option<Self> {
        Self::from_raw_high(const_fnv1a_hash::fnv1a_hash_str_32(high))
    }
    
    /// Creates a copy with the high-half replaced.
    pub const fn with_high_half_bytes(&self, high: &[u8]) -> Option<Self> {
        self.with_high_half_raw(const_fnv1a_hash::fnv1a_hash_32(high, None))
    }
    
    /// Creates a copy with the high-half replaced.
    pub const fn with_high_half_str(&self, high: &str) -> Option<Self> {
        self.with_high_half_raw(const_fnv1a_hash::fnv1a_hash_str_32(high))
    }
    
    /// Creates a copy with the low-half replaced.
    pub const fn with_low_half_bytes(&self, low: &[u8]) -> Option<Self> {
        self.with_low_half_raw(const_fnv1a_hash::fnv1a_hash_32(low, None))
    }
    
    /// Creates a copy with the low-half replaced.
    pub const fn with_low_half_str(&self, low: &str) -> Option<Self> {
        self.with_low_half_raw(const_fnv1a_hash::fnv1a_hash_str_32(low))
    }
}

/// Functions/Methods for the raw form of the DHK.
impl DualHashKey {
    /// Safely creates a new [DualHashKey] from two raw [u32] values.
    #[inline(always)]
    pub const fn from_raw_dual(high: u32, low: u32) -> Option<Self> {
        Self::from_raw((high as u64) << HIGH_SHIFT | (low as u64))
    }
    
    /// Safely creates a new [DualHashKey] from a raw [u32] value for the high-half,
    /// leaving the low-half zeroed out.
    #[inline(always)]
    pub const fn from_raw_high(high: u32) -> Option<Self> {
        Self::from_raw((high as u64) << HIGH_SHIFT)
    }
    
    /// Safely creates a new [DualHashKey] from a raw [u64] value.
    #[inline(always)]
    pub const fn from_raw(hash: u64) -> Option<Self> {
        match NonZeroU64::new(hash) {
            Some(hash) => Some(Self {hash}),
            None => None,
        }
    }
    
    /// Directly creates a new [DualHashKey] from a raw [u64] value.
    /// 
    /// # Safety
    /// This function is safe to call if-and-only-if the provided `hash` value is non-zero.
    #[inline(always)]
    pub const unsafe fn from_raw_unchecked(hash: u64) -> Self {
        Self {hash: NonZeroU64::new_unchecked(hash)}
    }
    
    /// Swaps the low and high halfes.
    #[inline(always)]
    pub const fn swapped(&self) -> Option<Self> {
        Self::from_raw_dual(
            self.get_hash_high_half(), 
            self.get_hash_low_half()
        )
    }
    
    /// Creates a copy with the high-half replaced.
    #[inline(always)]
    pub const fn with_high_half_raw(&self, high: u32) -> Option<Self> {
        Self::from_raw((self.hash.get() & LOW_MASK) | ((high as u64) << HIGH_SHIFT) )
    }
    
    /// Creates a copy with the low-half replaced.
    #[inline(always)]
    pub const fn with_low_half_raw(&self, low: u32) -> Option<Self> {
        Self::from_raw((self.hash.get() & HIGH_MASK) | (low as u64) )
    }
    
    /// Gets the wrapped hash value.
    #[inline(always)]
    pub const fn get_hash(&self) -> NonZeroU64 {
        self.hash
    }
    
    /// Gets the wrapped hash value as [u64].
    /// 
    /// This is the same as `dhk.get_hash().get()`.
    #[inline(always)]
    pub const fn get_hash_raw(&self) -> u64 {
        self.hash.get()
    }
    
    /// Gets the high-half of the hash.
    #[inline(always)]
    pub const fn get_hash_high_half(&self) -> u32 {
        (self.get_hash_raw() >> HIGH_SHIFT) as u32
    }
    
    /// Gets the low-half of the hash.
    #[inline(always)]
    pub const fn get_hash_low_half(&self) -> u32 {
        (self.get_hash_raw() & LOW_MASK) as u32
    }
    
    /// Checks if the low-half of the hash has any of its bits set.
    #[inline(always)]
    pub const fn is_hash_low_half_set(&self) -> bool {
        self.get_hash_low_half() != 0
    }
    
    /// Checks if the low-half of the hash has none of its bits set.
    #[inline(always)]
    pub const fn is_hash_low_half_clear(&self) -> bool {
        self.get_hash_low_half() == 0
    }
    
    /// Returns the hash with the low-half cleared.
    #[inline(always)]
    pub const fn get_hash_low_half_min_raw(&self) -> u64 {
        self.get_hash_raw() & HIGH_MASK
    }
    
    /// Returns the hash with the low-half filled.
    #[inline(always)]
    pub const fn get_hash_low_half_max_raw(&self) -> u64 {
        self.get_hash_raw() | LOW_MASK
    }
    
    /// Returns the hash with the low-half cleared.
    /// 
    /// Since this *may* result in an all-zero value, an [`Option<DualHashKey>`] is returned.
    #[inline(always)]
    pub const fn get_hash_low_half_min(&self) -> Option<Self> {
        Self::from_raw(self.get_hash_low_half_min_raw())
    }
    
    /// Returns the hash with the low-half filled.
    /// 
    /// Since the low-half is filled with bits, making the [`DualHashKey`]s value non-zero, this method can never fail.
    #[inline(always)]
    pub const fn get_hash_low_half_max(&self) -> Self {
        // # Safety
        // The `| U32_MAX` operation *forces* the low-half bits to be set.
        // As such, the raw DHK **cannot** be zero, so no check is needed.
        unsafe {
            Self::from_raw_unchecked(self.get_hash_low_half_max_raw())
        }
    }
}

impl core::convert::TryFrom<u64> for DualHashKey {
    type Error = &'static str;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Self::from_raw(value).ok_or("value given to DHK::from_raw is zero")
    }
}

impl core::convert::TryFrom<&[u8]> for DualHashKey {
    type Error = &'static str;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::from_high_bytes(value).ok_or("generated hash of high-half bytes is zero")
    }
}

impl core::convert::TryFrom<&str> for DualHashKey {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_high_str(value).ok_or("generated hash of high-half string is zero")
    }
}

impl core::convert::From<NonZeroU64> for DualHashKey {
    fn from(hash: NonZeroU64) -> Self {
        Self { hash }
    }
}
