// SPDX-License-Identifier: GPL-2.0

// C dependencies removed from executable Rust:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

/* This is an unused struct_ops program, it lacks corresponding
 * struct_ops map, which provides attachment information.
 * W/o additional configuration attempt to load such
 * BPF object file would fail.
 */
#[no_mangle]
#[link_section = "struct_ops/foo"]
pub extern "C" fn foo() {}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
