/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * SHA-256 Secure Hash Algorithm.
 *
 * Adapted for OCTEON by Aaro Koskinen <aaro.koskinen@iki.fi>.
 *
 * Based on crypto/sha256_generic.c, which is:
 *
 * Copyright (c) Jean-Luc Cooke <jlcooke@certainkey.com>
 * Copyright (c) Andrew McDonald <andrew@mcdonald.org.uk>
 * Copyright (c) 2002 James Morris <jmorris@intercode.com.au>
 * SHA224 Support Copyright 2007 Intel Corporation <jonathan.lynch@intel.com>
 */

/* Dependencies supplied by the surrounding kernel translation. */

/*
 * We pass everything as 64-bit. OCTEON can handle misaligned data.
 */

unsafe fn sha256_blocks(
    state: *mut sha256_block_state,
    mut data: *const u8,
    mut nblocks: usize,
) {
    let mut cop2_state: octeon_cop2_state = core::mem::zeroed();
    let state64 = state as *mut u64;
    let flags: c_ulong;

    if !octeon_has_crypto() {
        return sha256_blocks_generic(state, data, nblocks);
    }

    flags = octeon_crypto_enable(&mut cop2_state);
    write_octeon_64bit_hash_dword(*state64.add(0), 0);
    write_octeon_64bit_hash_dword(*state64.add(1), 1);
    write_octeon_64bit_hash_dword(*state64.add(2), 2);
    write_octeon_64bit_hash_dword(*state64.add(3), 3);

    loop {
        let block = data as *const u64;

        write_octeon_64bit_block_dword(*block.add(0), 0);
        write_octeon_64bit_block_dword(*block.add(1), 1);
        write_octeon_64bit_block_dword(*block.add(2), 2);
        write_octeon_64bit_block_dword(*block.add(3), 3);
        write_octeon_64bit_block_dword(*block.add(4), 4);
        write_octeon_64bit_block_dword(*block.add(5), 5);
        write_octeon_64bit_block_dword(*block.add(6), 6);
        octeon_sha256_start(*block.add(7));

        data = data.add(SHA256_BLOCK_SIZE);
        nblocks = nblocks.wrapping_sub(1);
        if nblocks == 0 {
            break;
        }
    }

    *state64.add(0) = read_octeon_64bit_hash_dword(0);
    *state64.add(1) = read_octeon_64bit_hash_dword(1);
    *state64.add(2) = read_octeon_64bit_hash_dword(2);
    *state64.add(3) = read_octeon_64bit_hash_dword(3);
    octeon_crypto_disable(&mut cop2_state, flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
