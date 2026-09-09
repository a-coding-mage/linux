/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * SHA-1 accelerated using the sparc64 crypto opcodes
 *
 * Copyright (c) Alan Smithee.
 * Copyright (c) Andrew McDonald <andrew@mcdonald.org.uk>
 * Copyright (c) Jean-Francois Dive <jef@linuxbe.org>
 * Copyright (c) Mathias Krause <minipli@googlemail.com>
 */

// C dependencies supplied by other translation units:
// asm/elf.h, asm/opcodes.h, asm/pstate.h

// External types, globals, and functions supplied by other translation units.
extern "C" {
    pub static mut have_sha1_opcodes: StaticKey;

    pub fn sha1_sparc64_transform(
        state: *mut sha1_block_state,
        data: *const u8,
        nblocks: usize,
    );

    pub fn sha1_blocks_generic(
        state: *mut sha1_block_state,
        data: *const u8,
        nblocks: usize,
    );

    pub static mut sparc64_elf_hwcap: c_ulong;

    pub fn static_branch_likely(key: *const StaticKey) -> bool;
    pub fn static_branch_enable(key: *mut StaticKey);
    pub fn pr_info(fmt: *const core::ffi::c_char, ...);
}

// Build-time definitions supplied by the surrounding kernel translation.
// The original header includes these architecture definitions.
pub type c_ulong = core::ffi::c_ulong;

#[repr(C)]
pub struct StaticKey {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sha1_block_state {
    _private: [u8; 0],
}

// Architecture constants supplied by asm/elf.h and asm/pstate.h:
// HWCAP_SPARC_CRYPTO and CFR_SHA1.

// static __ro_after_init DEFINE_STATIC_KEY_FALSE(have_sha1_opcodes);

pub fn sha1_blocks(
    state: *mut sha1_block_state,
    data: *const u8,
    nblocks: usize,
) {
    unsafe {
        if static_branch_likely(&have_sha1_opcodes) {
            sha1_sparc64_transform(state, data, nblocks);
        } else {
            sha1_blocks_generic(state, data, nblocks);
        }
    }
}

// #define sha1_mod_init_arch sha1_mod_init_arch
pub fn sha1_mod_init_arch() {
    let mut cfr: c_ulong;

    unsafe {
        if (sparc64_elf_hwcap & HWCAP_SPARC_CRYPTO) == 0 {
            return;
        }

        // __asm__ __volatile__("rd %%asr26, %0" : "=r" (cfr));
        core::arch::asm!("rd %asr26, {0}", out(reg) cfr);
        if (cfr & CFR_SHA1) == 0 {
            return;
        }

        static_branch_enable(&mut have_sha1_opcodes);
        pr_info(b"Using sparc64 sha1 opcode optimized SHA-1 implementation\n\0".as_ptr() as *const core::ffi::c_char);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
