// SPDX-License-Identifier: GPL-2.0
// Dependencies from the original C source:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>

/*
 * A single initialized global, so the generated loader has one internal
 * (.data) map that it seeds with an initial value while loading.
 * prog_tests/signed_loader.c uses this to check that a signed loader
 * keeps the attested contents and ignores a ctx-supplied initial_value:
 * the host cannot re-seed a signed program's maps through the loader ctx.
 */
#[no_mangle]
pub static mut magic: u64 = 0x5eed1234abad1dea_u64;

#[no_mangle]
#[link_section = "socket"]
pub unsafe extern "C" fn probe(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    magic as i32
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
