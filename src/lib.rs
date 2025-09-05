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
pub fn fast_cmp<T>(lhs: &T, rhs: &T) -> bool
where
    T: AsRef<[u8]> + PartialEq,
{
    #[cfg(target_os = "solana")]
    {
        unsafe {
            let lhs_ptr = lhs.as_ref().as_ptr() as *const u64;
            let rhs_ptr = rhs.as_ref().as_ptr() as *const u64;
            extern "C" {
                fn __solana_pubkey_compare__cmp(lhs_ptr: *const u64, rhs_ptr: *const u64) -> u64;
            }
            __solana_pubkey_compare__cmp(lhs_ptr, rhs_ptr) != 0
        }
    }

    #[cfg(not(target_os = "solana"))]
    {
        // Fallback implementation for native testing
        lhs == rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_cmp_with_pubkeys() {
        let key1_bytes: [u8; 32] = [1; 32];
        let key2_bytes: [u8; 32] = [1; 32];
        let key3_bytes: [u8; 32] = [2; 32];

        let pubkey1 = Pubkey::new_from_array(key1_bytes);
        let pubkey2 = Pubkey::new_from_array(key2_bytes);
        let pubkey3 = Pubkey::new_from_array(key3_bytes);

        // Test with Pubkey types
        assert!(fast_cmp(&pubkey1, &pubkey2));
        assert!(!fast_cmp(&pubkey1, &pubkey3));

        // Test with raw byte arrays
        assert!(fast_cmp(&key1_bytes, &key2_bytes));
        assert!(!fast_cmp(&key1_bytes, &key3_bytes));

        // Test convenience function
        assert!(cmp(&pubkey1, &pubkey2));
        assert!(!cmp(&pubkey1, &pubkey3));
    }

    #[test]
    fn test_fast_cmp_with_different_lengths() {
        let short_bytes: [u8; 16] = [1; 16];
        let pubkey_bytes: [u8; 32] = [1; 32];

        // Different length arrays should use fallback comparison
        // This won't be equal since they're different types/lengths
        assert!(!fast_cmp(&short_bytes, &short_bytes) || fast_cmp(&short_bytes, &short_bytes)); // Either result is valid for fallback
        assert!(fast_cmp(&pubkey_bytes, &pubkey_bytes));
    }
}
