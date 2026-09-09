/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Cryptographic API.
 *
 * SHA-512 and SHA-384 Secure Hash Algorithm.
 *
 * Adapted for OCTEON by Aaro Koskinen <aaro.koskinen@iki.fi>.
 *
 * Based on crypto/sha512_generic.c, which is:
 *
 * Copyright (c) Jean-Luc Cooke <jlcooke@certainkey.com>
 * Copyright (c) Andrew McDonald <andrew@mcdonald.org.uk>
 * Copyright (c) 2003 Kyle McMartin <kyle@debian.org>
 */

// Dependencies supplied by the surrounding translation unit:
// asm/octeon/crypto.h, asm/octeon/octeon.h

/*
 * We pass everything as 64-bit. OCTEON can handle misaligned data.
 */

unsafe fn sha512_blocks(
    state: *mut sha512_block_state,
    mut data: *const u8,
    mut nblocks: usize,
) {
    let mut cop2_state: octeon_cop2_state;
    let flags: libc::c_ulong;

    if !octeon_has_crypto() {
        return sha512_blocks_generic(state, data, nblocks);
    }

    flags = octeon_crypto_enable(&mut cop2_state);
    write_octeon_64bit_hash_sha512((*state).h[0], 0);
    write_octeon_64bit_hash_sha512((*state).h[1], 1);
    write_octeon_64bit_hash_sha512((*state).h[2], 2);
    write_octeon_64bit_hash_sha512((*state).h[3], 3);
    write_octeon_64bit_hash_sha512((*state).h[4], 4);
    write_octeon_64bit_hash_sha512((*state).h[5], 5);
    write_octeon_64bit_hash_sha512((*state).h[6], 6);
    write_octeon_64bit_hash_sha512((*state).h[7], 7);

    loop {
        let block = data as *const u64;

        write_octeon_64bit_block_sha512(*block.add(0), 0);
        write_octeon_64bit_block_sha512(*block.add(1), 1);
        write_octeon_64bit_block_sha512(*block.add(2), 2);
        write_octeon_64bit_block_sha512(*block.add(3), 3);
        write_octeon_64bit_block_sha512(*block.add(4), 4);
        write_octeon_64bit_block_sha512(*block.add(5), 5);
        write_octeon_64bit_block_sha512(*block.add(6), 6);
        write_octeon_64bit_block_sha512(*block.add(7), 7);
        write_octeon_64bit_block_sha512(*block.add(8), 8);
        write_octeon_64bit_block_sha512(*block.add(9), 9);
        write_octeon_64bit_block_sha512(*block.add(10), 10);
        write_octeon_64bit_block_sha512(*block.add(11), 11);
        write_octeon_64bit_block_sha512(*block.add(12), 12);
        write_octeon_64bit_block_sha512(*block.add(13), 13);
        write_octeon_64bit_block_sha512(*block.add(14), 14);
        octeon_sha512_start(*block.add(15));

        data = data.add(SHA512_BLOCK_SIZE);
        nblocks = nblocks.wrapping_sub(1);
        if nblocks == 0 {
            break;
        }
    }

    (*state).h[0] = read_octeon_64bit_hash_sha512(0);
    (*state).h[1] = read_octeon_64bit_hash_sha512(1);
    (*state).h[2] = read_octeon_64bit_hash_sha512(2);
    (*state).h[3] = read_octeon_64bit_hash_sha512(3);
    (*state).h[4] = read_octeon_64bit_hash_sha512(4);
    (*state).h[5] = read_octeon_64bit_hash_sha512(5);
    (*state).h[6] = read_octeon_64bit_hash_sha512(6);
    (*state).h[7] = read_octeon_64bit_hash_sha512(7);
    octeon_crypto_disable(&mut cop2_state, flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
