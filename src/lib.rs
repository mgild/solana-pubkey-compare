/// High-performance pubkey comparison using BPF assembly
///
/// This library provides optimized pubkey comparison for Solana programs
/// using pure BPF assembly for maximum performance.

use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
};


/// Pure assembly pubkey equality check - maximum performance
///
/// # Safety
///
/// Both pointers must point to valid 32-byte pubkey data aligned as u64 arrays.
#[inline(always)]
pub fn cmp(lhs_ptr: &Pubkey, rhs_ptr: &Pubkey) -> bool {
    #[cfg(target_arch = "bpf")]
    {
        unsafe {
            let lhs_ptr = lhs_ptr.as_ref() as *const _ as *const u64;
            let rhs_ptr = rhs_ptr.as_ref() as *const _ as *const u64;
            extern "C" {
                fn __solana_pubkey_compare__cmp(lhs_ptr: *const u64, rhs_ptr: *const u64) -> u64;
            }
            __solana_pubkey_compare__cmp(lhs_ptr, rhs_ptr) != 0
        }
    }
    
    #[cfg(not(target_arch = "bpf"))]
    {
        // Fallback implementation for native testing
        lhs_ptr == rhs_ptr
    }
}

/// Program entrypoint for testing
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if accounts.len() < 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let account1 = &accounts[0];
    let account2 = &accounts[1];

    match instruction_data.get(0) {
        Some(0) => {
            // Standard comparison
            if account1.key == account2.key {
                // Keys match
                Ok(())
            } else {
                // Keys don't match  
                Ok(())
            }
        }
        Some(1) => {
            // Assembly comparison
            if cmp(account1.key, account2.key) {
                // Keys match
                Ok(())
            } else {
                // Keys don't match - this should panic for demonstration
                panic!("Assembly comparison: keys don't match!");
            }
        }
        _ => Err(ProgramError::InvalidInstructionData),
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
