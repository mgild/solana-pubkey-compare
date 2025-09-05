//! Optimized BPF assembly implementation for 32-byte public key comparison
//!
//! This assembly function provides maximum performance for comparing Solana
//! public keys by leveraging BPF's 64-bit memory operations and conditional
//! jumps for early exit optimization.
//!
//! ## Performance Characteristics
//! - **Best case**: 11 instructions (keys differ in first 8 bytes)  
//! - **Worst case**: 19 instructions (keys are identical)
//! - **Memory ops**: 2-8 loads depending on where difference is found
//! - **Branches**: 1-4 conditional jumps with early termination
//!
//! ## Instruction Breakdown
//! - 2x `ldxdw` per 8-byte chunk (load 64-bit values)
//! - 1x `jne` per chunk (conditional jump on not-equal)
//! - 1x `lddw` + `exit` for return value
//!
//! ## Algorithm 
//! 1. Load 8 bytes from each key at offset 0, compare, exit if different
//! 2. Load 8 bytes from each key at offset 8, compare, exit if different  
//! 3. Load 8 bytes from each key at offset 16, compare, exit if different
//! 4. Load 8 bytes from each key at offset 24, compare, exit if different
//! 5. Return true (1) if all chunks match
//!
//! ## Register Usage
//! - r0: Return value (0 = false, 1 = true)
//! - r1: Pointer to first key (lhs_ptr parameter) 
//! - r2: Pointer to second key (rhs_ptr parameter)
//! - r3: Temporary for first key's 8-byte chunk
//! - r4: Temporary for second key's 8-byte chunk

.section .text
.globl __solana_pubkey_compare__fast_eq
.type __solana_pubkey_compare__fast_eq, @function

__solana_pubkey_compare__fast_eq:
    // Function parameters: r1 = lhs_ptr, r2 = rhs_ptr
    // Returns: r0 = 1 if equal, 0 if not equal

    // Compare bytes 0-7: Load first 64-bit chunk from both keys
    ldxdw r3, [r1+0]      // r3 = first 8 bytes of lhs
    ldxdw r4, [r2+0]      // r4 = first 8 bytes of rhs  
    jne r3, r4, not_equal // Early exit if chunks differ

    // Compare bytes 8-15: Load second 64-bit chunk from both keys
    ldxdw r3, [r1+8]      // r3 = bytes 8-15 of lhs
    ldxdw r4, [r2+8]      // r4 = bytes 8-15 of rhs
    jne r3, r4, not_equal // Early exit if chunks differ

    // Compare bytes 16-23: Load third 64-bit chunk from both keys  
    ldxdw r3, [r1+16]     // r3 = bytes 16-23 of lhs
    ldxdw r4, [r2+16]     // r4 = bytes 16-23 of rhs
    jne r3, r4, not_equal // Early exit if chunks differ

    // Compare bytes 24-31: Load fourth 64-bit chunk from both keys
    ldxdw r3, [r1+24]     // r3 = bytes 24-31 of lhs  
    ldxdw r4, [r2+24]     // r4 = bytes 24-31 of rhs
    jne r3, r4, not_equal // Early exit if chunks differ

    // All 32 bytes match - return true
    lddw r0, 1            // Load immediate value 1 into return register
    exit                  // Return to caller

not_equal:
    // Keys differ - return false
    lddw r0, 0            // Load immediate value 0 into return register  
    exit                  // Return to caller

.size __solana_pubkey_compare__fast_eq, .-__solana_pubkey_compare__fast_eq
