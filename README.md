# solana-pubkey-compare

High-performance Solana public key comparison using optimized BPF assembly.

## Overview

This crate provides ultra-fast public key comparison for Solana blockchain programs, achieving significant performance improvements through hand-optimized BPF assembly. Perfect for performance-critical Solana programs where compute unit efficiency matters.

## Performance

- **Assembly implementation**: 19 compute units on Solana BPF
- **Standard comparison**: 28 compute units on Solana BPF  
- **Improvement**: ~32% reduction in compute units

## Features

- ✅ Zero dependencies and `#[no_std]` compatible
- ✅ Hand-optimized BPF assembly for Solana runtime  
- ✅ Automatic fallback to standard comparison for native testing
- ✅ Generic interface supporting any 32-byte key types
- ✅ Compile-time safety with Rust's type system
- ✅ Early exit optimization for maximum efficiency

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
solana-pubkey-compare = "0.1.0"
```

## Usage

### Basic Example

```rust
use solana_pubkey_compare::fast_eq;
use solana_program::pubkey::Pubkey;

// Compare Solana Pubkeys
let key1 = Pubkey::new_unique();  
let key2 = Pubkey::new_unique();

if fast_eq(&key1, &key2) {
    // Keys are equal - this is very fast!
}

// Works with any 32-byte types
let bytes1: [u8; 32] = [0; 32];
let bytes2: [u8; 32] = [1; 32]; 
assert!(!fast_eq(&bytes1, &bytes2));
```

### In Solana Programs

```rust
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};
use solana_pubkey_compare::fast_eq;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let expected_authority = Pubkey::from([/* your authority key */]);
    
    // Fast authority check - saves compute units!
    if !fast_eq(&accounts[0].key, &expected_authority) {
        return Err(/* unauthorized error */);
    }
    
    // Continue processing...
    Ok(())
}
```

### Performance-Critical Loops

```rust
use solana_pubkey_compare::fast_eq;

fn find_account_index(accounts: &[AccountInfo], target: &Pubkey) -> Option<usize> {
    accounts.iter().position(|account| {
        // Each comparison saves ~9 compute units
        fast_eq(&account.key, target)
    })
}
```

## How It Works

### Assembly Implementation

The core of this crate is a hand-optimized BPF assembly function that:

1. **Loads 64-bit chunks**: Uses `ldxdw` to load 8 bytes at a time
2. **Early exit**: Uses conditional jumps (`jne`) to exit immediately on first mismatch  
3. **Minimal instructions**: Only 11-19 instructions depending on where difference is found
4. **Register-optimized**: Uses minimal register pressure for maximum efficiency

### Instruction Sequence

```assembly
// Compare bytes 0-7
ldxdw r3, [r1+0]      // Load first 8 bytes of key1
ldxdw r4, [r2+0]      // Load first 8 bytes of key2  
jne r3, r4, not_equal // Exit early if different

// Repeat for bytes 8-15, 16-23, 24-31...
// Return 1 if all equal, 0 if any different
```

### Fallback Implementation

On non-Solana platforms, the function falls back to the standard `PartialEq` implementation for compatibility with testing and development workflows.

## Benchmarks

Performance measurements on Solana BPF runtime:

| Operation | Standard `PartialEq` | `fast_eq` | Improvement |
|-----------|---------------------|-----------|-------------|
| Equal keys | 28 CU | 19 CU | 32% faster |
| Different keys (first byte) | 28 CU | 11 CU | 61% faster |
| Different keys (last byte) | 28 CU | 19 CU | 32% faster |

*CU = Compute Units*

## Type Requirements

The generic type `T` must implement:
- `AsRef<[u8]>` - For accessing the underlying byte data  
- `PartialEq` - For the native fallback implementation

This includes:
- `solana_program::pubkey::Pubkey`
- `[u8; 32]` 
- `Vec<u8>` (if exactly 32 bytes)
- Any custom types that dereference to 32-byte arrays

## Safety

This function is completely safe to call. While it uses `unsafe` internally to interface with the assembly function, all safety invariants are maintained:

- References are valid for the duration of the call
- Data alignment is handled by the BPF runtime  
- No memory is mutated - this is a pure comparison
- All bounds are compile-time verified

## Development

### Building

```bash
# Build for native (testing)
cargo build

# Build for Solana BPF  
cargo build-sbf
```

### Testing

```bash
# Run tests (uses fallback implementation)
cargo test

# Test on Solana BPF runtime
cargo test-sbf
```

## Contributing

Contributions are welcome! Please ensure:

- All tests pass on both native and BPF targets
- Assembly changes are documented and benchmarked
- New features maintain the `#[no_std]` compatibility

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Credits

Created by [Switchboard](https://switchboard.xyz) for high-performance Solana applications.