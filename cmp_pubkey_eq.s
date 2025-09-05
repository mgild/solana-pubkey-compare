.section .text
.globl __solana_pubkey_compare__fast_eq
.type __solana_pubkey_compare__fast_eq, @function

__solana_pubkey_compare__fast_eq:
    // r1 = lhs_ptr, r2 = rhs_ptr

    // Load and compare first 8 bytes
    ldxdw r3, [r1+0]
    ldxdw r4, [r2+0]
    jne r3, r4, not_equal

    // Load and compare second 8 bytes
    ldxdw r3, [r1+8]
    ldxdw r4, [r2+8]
    jne r3, r4, not_equal

    // Load and compare third 8 bytes
    ldxdw r3, [r1+16]
    ldxdw r4, [r2+16]
    jne r3, r4, not_equal

    // Load and compare fourth 8 bytes
    ldxdw r3, [r1+24]
    ldxdw r4, [r2+24]
    jne r3, r4, not_equal

    // All equal - return true (1)
    lddw r0, 1
    exit

not_equal:
    // Mismatch - return false (0)
    lddw r0, 0
    exit

.size __solana_pubkey_compare__fast_eq, .-__solana_pubkey_compare__fast_eq
