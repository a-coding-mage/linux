/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Cryptographic API.
 *
 * SHA1 Secure Hash Algorithm.
 *
 * Adapted for OCTEON by Aaro Koskinen <aaro.koskinen@iki.fi>.
 *
 * Based on crypto/sha1_generic.c, which is:
 *
 * Copyright (c) Alan Smithee.
 * Copyright (c) Andrew McDonald <andrew@mcdonald.org.uk>
 * Copyright (c) Jean-Francois Dive <jef@linuxbe.org>
 */

// External dependencies supplied by asm/octeon/crypto.h, asm/octeon/octeon.h,
// and the SHA-1 implementation are intentionally not defined here.

#[repr(C)]
union Sha1HashTail {
    word: [u32; 2],
    dword: u64,
}

unsafe fn octeon_sha1_store_hash(state: *mut sha1_block_state) {
    let hash = (*state).h.as_mut_ptr() as *mut u64;
    let hash_tail = Sha1HashTail {
        word: [(*state).h[4], 0],
    };

    write_octeon_64bit_hash_dword(hash.read_unaligned(), 0);
    write_octeon_64bit_hash_dword(hash.add(1).read_unaligned(), 1);
    write_octeon_64bit_hash_dword(hash_tail.dword, 2);
    memzero_explicit(
        core::ptr::addr_of!((*(&hash_tail)).word[0]) as *mut core::ffi::c_void,
        core::mem::size_of::<u32>(),
    );
}

unsafe fn octeon_sha1_read_hash(state: *mut sha1_block_state) {
    let hash = (*state).h.as_mut_ptr() as *mut u64;
    let mut hash_tail = Sha1HashTail { dword: 0 };

    hash.write_unaligned(read_octeon_64bit_hash_dword(0));
    hash.add(1).write_unaligned(read_octeon_64bit_hash_dword(1));
    hash_tail.dword = read_octeon_64bit_hash_dword(2);
    (*state).h[4] = hash_tail.word[0];
    memzero_explicit(
        core::ptr::addr_of_mut!(hash_tail.dword) as *mut core::ffi::c_void,
        core::mem::size_of::<u64>(),
    );
}

unsafe fn sha1_blocks(
    state: *mut sha1_block_state,
    mut data: *const u8,
    mut nblocks: usize,
) {
    let mut cop2_state: octeon_cop2_state = core::mem::zeroed();
    let mut flags: usize;

    if octeon_has_crypto() == 0 {
        return sha1_blocks_generic(state, data, nblocks);
    }

    flags = octeon_crypto_enable(&mut cop2_state);
    octeon_sha1_store_hash(state);

    loop {
        let block = data as *const u64;

        write_octeon_64bit_block_dword(block.add(0).read_unaligned(), 0);
        write_octeon_64bit_block_dword(block.add(1).read_unaligned(), 1);
        write_octeon_64bit_block_dword(block.add(2).read_unaligned(), 2);
        write_octeon_64bit_block_dword(block.add(3).read_unaligned(), 3);
        write_octeon_64bit_block_dword(block.add(4).read_unaligned(), 4);
        write_octeon_64bit_block_dword(block.add(5).read_unaligned(), 5);
        write_octeon_64bit_block_dword(block.add(6).read_unaligned(), 6);
        octeon_sha1_start(block.add(7).read_unaligned());

        data = data.add(SHA1_BLOCK_SIZE);
        nblocks -= 1;
        if nblocks == 0 {
            break;
        }
    }

    octeon_sha1_read_hash(state);
    octeon_crypto_disable(&mut cop2_state, flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
