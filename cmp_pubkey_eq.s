.section .text
.globl __solana_pubkey_compare__cmp
.type __solana_pubkey_compare__cmp, @function

__solana_pubkey_compare__cmp:
    // r1 = lhs_ptr, r2 = rhs_ptr

    // Load and compare first 8 bytes
    ldxdw r3, [r1+0]
    ldxdw r4, [r2+0]
    jne r3, r4, fail

    // Load and compare second 8 bytes
    ldxdw r3, [r1+8]
    ldxdw r4, [r2+8]
    jne r3, r4, fail

    // Load and compare third 8 bytes
    ldxdw r3, [r1+16]
    ldxdw r4, [r2+16]
    jne r3, r4, fail

    // Load and compare fourth 8 bytes
    ldxdw r3, [r1+24]
    ldxdw r4, [r2+24]
    jne r3, r4, fail

    // All equal - success, return 0
    lddw r0, 0
    exit

fail:
    // Mismatch - return 1
    lddw r0, 1
    exit

.size __solana_pubkey_compare__cmp, .-__solana_pubkey_compare__cmp
