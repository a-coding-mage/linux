// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * High speed xor_block operation for RAID4/5 utilizing the
 * UltraSparc Visual Instruction Set and Niagara block-init
 * twin-load instructions.
 *
 * Copyright (C) 1997, 1999 Jakub Jelinek (jj@ultra.linux.cz)
 * Copyright (C) 2006 David S. Miller <davem@davemloft.net>
 */

// Dependencies supplied by xor_impl.h and xor_arch.h are provided by the
// surrounding translation unit.

use core::ffi::c_char;

extern "C" {
    pub fn xor_vis_2(
        bytes: usize,
        p1: *mut usize,
        p2: *const usize,
    );
    pub fn xor_vis_3(
        bytes: usize,
        p1: *mut usize,
        p2: *const usize,
        p3: *const usize,
    );
    pub fn xor_vis_4(
        bytes: usize,
        p1: *mut usize,
        p2: *const usize,
        p3: *const usize,
        p4: *const usize,
    );
    pub fn xor_vis_5(
        bytes: usize,
        p1: *mut usize,
        p2: *const usize,
        p3: *const usize,
        p4: *const usize,
        p5: *const usize,
    );
}

/* XXX Ugh, write cheetah versions... -DaveM */

// DO_XOR_BLOCKS(vis, xor_vis_2, xor_vis_3, xor_vis_4, xor_vis_5);
// The declaration above is the Rust equivalent of the source macro
// expansion supplied by xor_impl.h.

extern "C" {
    pub fn xor_gen_vis();
}

#[repr(C)]
pub struct xor_block_template {
    pub name: *const c_char,
    pub xor_gen: unsafe extern "C" fn(),
}

#[no_mangle]
pub static mut xor_block_VIS: xor_block_template = xor_block_template {
    name: b"VIS\0".as_ptr() as *const c_char,
    xor_gen: xor_gen_vis,
};

extern "C" {
    pub fn xor_niagara_2(
        bytes: usize,
        p1: *mut usize,
        p2: *const usize,
    );
    pub fn xor_niagara_3(
        bytes: usize,
        p1: *mut usize,
        p2: *const usize,
        p3: *const usize,
    );
    pub fn xor_niagara_4(
        bytes: usize,
        p1: *mut usize,
        p2: *const usize,
        p3: *const usize,
        p4: *const usize,
    );
    pub fn xor_niagara_5(
        bytes: usize,
        p1: *mut usize,
        p2: *const usize,
        p3: *const usize,
        p4: *const usize,
        p5: *const usize,
    );
}

// DO_XOR_BLOCKS(niagara, xor_niagara_2, xor_niagara_3, xor_niagara_4,
//               xor_niagara_5);

extern "C" {
    pub fn xor_gen_niagara();
}

#[no_mangle]
pub static mut xor_block_niagara: xor_block_template = xor_block_template {
    name: b"Niagara\0".as_ptr() as *const c_char,
    xor_gen: xor_gen_niagara,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
