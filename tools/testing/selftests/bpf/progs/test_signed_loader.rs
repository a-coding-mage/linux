// SPDX-License-Identifier: GPL-2.0
// Original dependencies: "vmlinux.h" and <bpf/bpf_helpers.h>

/*
 * Minimal, map-less program. Driven through libbpf's gen_loader (gen_hash)
 * by prog_tests/signed_loader.c so the generated light-skeleton loader can be
 * exercised against good and tampered metadata, which the kernel now verifies
 * at load time via the insns||metadata signature. A socket filter needs no
 * load-time attach resolution, and having no maps keeps the generated loader's
 * ctx trivial (0 maps, 1 prog).
 */
#[no_mangle]
#[link_section = "socket"]
pub unsafe extern "C" fn probe(ctx: *mut core::ffi::c_void) -> i32 {
    let _ = ctx;
    return 0;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";
