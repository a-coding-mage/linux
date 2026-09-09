/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * SHA-512 accelerated using the sparc64 sha512 opcodes
 *
 * Copyright (c) Jean-Luc Cooke <jlcooke@certainkey.com>
 * Copyright (c) Andrew McDonald <andrew@mcdonald.org.uk>
 * Copyright (c) 2003 Kyle McMartin <kyle@debian.org>
 */

// The following types, constants, globals, and functions are supplied by
// other translation units or by the target architecture environment.
extern "C" {
    static mut have_sha512_opcodes: StaticKey;

    fn sha512_sparc64_transform(
        state: *mut sha512_block_state,
        data: *const u8,
        nblocks: usize,
    );

    fn sha512_blocks_generic(
        state: *mut sha512_block_state,
        data: *const u8,
        nblocks: usize,
    );

    fn static_branch_likely(key: *const StaticKey) -> bool;
    fn static_branch_enable(key: *mut StaticKey);

    static sparc64_elf_hwcap: usize;
    static HWCAP_SPARC_CRYPTO: usize;
    static CFR_SHA512: usize;

    fn pr_info(format: *const u8, ...);
}

// Opaque external types corresponding to the kernel declarations used here.
#[repr(C)]
pub struct sha512_block_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct StaticKey {
    _private: [u8; 0],
}

unsafe fn sha512_blocks(
    state: *mut sha512_block_state,
    data: *const u8,
    nblocks: usize,
) {
    if static_branch_likely(unsafe { &have_sha512_opcodes }) {
        unsafe { sha512_sparc64_transform(state, data, nblocks) };
    } else {
        unsafe { sha512_blocks_generic(state, data, nblocks) };
    }
}

// #define sha512_mod_init_arch sha512_mod_init_arch
unsafe fn sha512_mod_init_arch() {
    let mut cfr: usize;

    if unsafe { sparc64_elf_hwcap } & unsafe { HWCAP_SPARC_CRYPTO } == 0 {
        return;
    }

    unsafe {
        core::arch::asm!("rd %asr26, {0}", out(reg) cfr);
    }
    if cfr & unsafe { CFR_SHA512 } == 0 {
        return;
    }

    unsafe { static_branch_enable(&mut have_sha512_opcodes) };
    unsafe {
        pr_info(
            b"Using sparc64 sha512 opcode optimized SHA-512/SHA-384 implementation\0"
                .as_ptr(),
        );
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
