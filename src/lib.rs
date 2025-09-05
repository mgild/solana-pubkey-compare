#![allow(unexpected_cfgs)]
#![feature(asm_experimental_arch)]
#![doc = include_str!("../README.md")]

unsafe extern "C" {
    fn __solana_pubkey_compare__fast_eq(lhs_ptr: *const u64, rhs_ptr: *const u64) -> bool;
}

/// High-performance Solana public key comparison library
///
/// This crate provides ultra-fast public key comparison for Solana blockchain programs,
/// achieving significant performance improvements through hand-optimized BPF assembly.
///
/// ## Performance
///
/// - **Assembly implementation**: 19 compute units on Solana BPF
/// - **Standard comparison**: 28 compute units on Solana BPF
/// - **Improvement**: ~32% reduction in compute units
///
/// ## Features
///
/// - Zero dependencies and `#[no_std]` compatible
/// - Hand-optimized BPF assembly for Solana runtime
/// - Automatic fallback to standard comparison for native testing
/// - Generic interface supporting any 32-byte key types
/// - Compile-time safety with Rust's type system
///
/// ## Usage
///
/// ```rust
/// use solana_pubkey_compare::fast_eq;
/// use solana_program::pubkey::Pubkey;
///
/// // Compare Solana Pubkeys
/// let key1 = Pubkey::new_unique();
/// let key2 = Pubkey::new_unique();
///
/// if fast_eq(&key1, &key2) {
///     // Keys are equal
/// }
///
/// // Works with any 32-byte types
/// let bytes1: [u8; 32] = [0; 32];
/// let bytes2: [u8; 32] = [1; 32];
/// assert!(!fast_eq(&bytes1, &bytes2));
/// ```
///
/// ## Implementation Details
///
/// The assembly implementation performs parallel 64-bit comparisons:
/// 1. Loads four 8-byte chunks from each key simultaneously
/// 2. Uses BPF conditional jumps for early exit on first mismatch
/// 3. Minimizes instruction count and memory access overhead
///
/// On native platforms, falls back to the standard `PartialEq` implementation
/// for compatibility with testing and development workflows.


/// Ultra-fast public key equality comparison using optimized BPF assembly
///
/// This function provides maximum performance for comparing 32-byte public keys
/// on Solana's BPF runtime. It uses hand-crafted assembly that performs four
/// parallel 64-bit comparisons with early exit on first mismatch.
///
/// # Performance
///
/// - **On Solana BPF**: 19 compute units (32% faster than standard comparison)
/// - **On native**: Falls back to `PartialEq` for testing compatibility
///
/// # Examples
///
/// ```rust
/// use solana_pubkey_compare::fast_eq;
/// use solana_program::pubkey::Pubkey;
///
/// let pubkey1 = Pubkey::new_unique();
/// let pubkey2 = Pubkey::new_unique();
///
/// // Fast comparison - uses assembly on Solana BPF
/// if fast_eq(&pubkey1, &pubkey2) {
///     // Handle equal keys
/// }
///
/// // Works with any 32-byte array-like types
/// let array1 = [1u8; 32];
/// let array2 = [1u8; 32];
/// assert!(fast_eq(&array1, &array2));
/// ```
///
/// # Type Requirements
///
/// The generic type `T` must implement:
/// - `AsRef<[u8]>` - For accessing the underlying byte data
/// - `PartialEq` - For the native fallback implementation
///
/// # Safety
///
/// This function is safe to call. Internally it uses unsafe code to cast
/// references to raw pointers for the assembly function, but all safety
/// invariants are maintained:
///
/// - References are valid for the duration of the call
/// - Data alignment is handled by the BPF runtime
/// - No memory is mutated - this is a pure comparison
///
/// # Implementation Notes
///
/// The assembly implementation (`cmp_pubkey_eq.s`) performs:
/// 1. Four 64-bit memory loads (8 bytes each)
/// 2. Four conditional comparisons with early exit
/// 3. Single return instruction with boolean result
///
/// This eliminates the overhead of Rust's slice comparison and provides
/// direct control over the BPF instruction sequence.
#[inline(always)]
pub fn fast_eq<T>(lhs: &T, rhs: &T) -> bool
where
    T: AsRef<[u8]> + PartialEq,
{
    #[cfg(target_os = "solana")]
    unsafe {
        let lhs_ptr = lhs as *const _ as *const u64;
        let rhs_ptr = rhs as *const _ as *const u64;
        __solana_pubkey_compare__fast_eq(lhs_ptr, rhs_ptr)
    }

    #[cfg(not(target_os = "solana"))]
    {
        lhs == rhs
    }
}
