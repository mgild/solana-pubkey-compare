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
pub fn fast_cmp(lhs_ptr: &Pubkey, rhs_ptr: &Pubkey) -> bool {
    #[cfg(target_os = "solana")]
    {
        unsafe {
            let lhs_ptr = lhs_ptr.as_ref() as *const _ as *const u64;
            let rhs_ptr = rhs_ptr.as_ref() as *const _ as *const u64;
            unsafe extern "C" {
                fn __solana_pubkey_compare__cmp(lhs_ptr: *const u64, rhs_ptr: *const u64) -> u64;
            }
            __solana_pubkey_compare__cmp(lhs_ptr, rhs_ptr) != 0
        }
    }

    #[cfg(not(target_os = "solana"))]
    {
        // Fallback implementation for native testing
        lhs_ptr == rhs_ptr
    }
}
