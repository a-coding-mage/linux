/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * SHA-256 accelerated using the sparc64 sha256 opcodes
 *
 * Copyright (c) Jean-Luc Cooke <jlcooke@certainkey.com>
 * Copyright (c) Andrew McDonald <andrew@mcdonald.org.uk>
 * Copyright (c) 2002 James Morris <jmorris@intercode.com.au>
 * SHA224 Support Copyright 2007 Intel Corporation <jonathan.lynch@intel.com>
 */

// Dependencies supplied by the surrounding kernel translation.

static mut have_sha256_opcodes: core::ffi::c_void = core::ffi::c_void {};

extern "C" {
    fn sha256_sparc64_transform(
        state: *mut sha256_block_state,
        data: *const u8,
        nblocks: usize,
    );
}

unsafe fn sha256_blocks(
    state: *mut sha256_block_state,
    data: *const u8,
    nblocks: usize,
) {
    if static_branch_likely(&mut have_sha256_opcodes) {
        sha256_sparc64_transform(state, data, nblocks);
    } else {
        sha256_blocks_generic(state, data, nblocks);
    }
}

macro_rules! sha256_mod_init_arch {
    ($($tokens:tt)*) => { sha256_mod_init_arch($($tokens)*) };
}

unsafe fn sha256_mod_init_arch() {
    let mut cfr: usize;

    if (sparc64_elf_hwcap & HWCAP_SPARC_CRYPTO) == 0 {
        return;
    }

    core::arch::asm!("rd %asr26, {0}", out(reg) cfr);
    if (cfr & CFR_SHA256) == 0 {
        return;
    }

    static_branch_enable(&mut have_sha256_opcodes);
    pr_info!("Using sparc64 sha256 opcode optimized SHA-256/SHA-224 implementation\n");
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
