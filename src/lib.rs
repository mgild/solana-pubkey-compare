/// High-performance pubkey comparison using BPF assembly
///
/// This library provides optimized pubkey comparison for Solana programs
/// using pure BPF assembly for maximum performance.

use solana_program::pubkey::Pubkey;


/// Pure assembly pubkey equality check - maximum performance
///
/// # Safety
///
/// Both pointers must point to valid 32-byte pubkey data aligned as u64 arrays.
#[inline(always)]
pub fn cmp(lhs_ptr: &Pubkey, rhs_ptr: &Pubkey) -> bool {
    unsafe {
        let lhs_ptr = lhs_ptr.as_ref() as *const _ as *const u64;
        let rhs_ptr = rhs_ptr.as_ref() as *const _ as *const u64;
        unsafe extern "C" {
            fn __solana_pubkey_compare__cmp(lhs_ptr: *const u64, rhs_ptr: *const u64) -> u64;
        }
        __solana_pubkey_compare__cmp(lhs_ptr, rhs_ptr) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pubkey_eq() {
        let key1: [u8; 32] = [1; 32];
        let key2: [u8; 32] = [1; 32];
        let key3: [u8; 32] = [2; 32];

        let pubkey1 = Pubkey::new_from_array(key1);
        let pubkey2 = Pubkey::new_from_array(key2);
        let pubkey3 = Pubkey::new_from_array(key3);

        // Should be equal
        assert!(cmp(&pubkey1, &pubkey2));
        
        // Should not be equal
        assert!(!cmp(&pubkey1, &pubkey3));
    }
}
