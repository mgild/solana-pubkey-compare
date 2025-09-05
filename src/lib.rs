#![allow(unexpected_cfgs)]
use solana_program::pubkey::Pubkey;

/// High-performance pubkey comparison using BPF assembly
///
/// This library provides optimized pubkey comparison for Solana programs
/// using pure BPF assembly for maximum performance.


/// Pure assembly pubkey equality check - maximum performance
///
/// # Safety
///
/// Both pointers must point to valid 32-byte pubkey data aligned as u64 arrays.
#[inline(always)]
pub fn fast_eq<T>(lhs: &T, rhs: &T) -> bool
where
    T: AsRef<[u8]> + PartialEq,
{
    #[cfg(target_os = "solana")]
    {
        unsafe {
            let lhs_ptr = lhs.as_ref().as_ptr() as *const u64;
            let rhs_ptr = rhs.as_ref().as_ptr() as *const u64;
            extern "C" {
                fn __solana_pubkey_compare__cmp(lhs_ptr: *const u64, rhs_ptr: *const u64) -> bool;
            }
            __solana_pubkey_compare__cmp(lhs_ptr, rhs_ptr)
        }
    }

    #[cfg(not(target_os = "solana"))]
    {
        // Fallback implementation for native testing
        lhs == rhs
    }
}
