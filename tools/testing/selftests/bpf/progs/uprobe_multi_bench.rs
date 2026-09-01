// SPDX-License-Identifier: GPL-2.0
// Dependencies from the original C source:
// #include <linux/bpf.h>
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut count: i32 = 0;

#[no_mangle]
#[link_section = "uprobe.multi/./uprobe_multi:uprobe_multi_func_*"]
pub unsafe extern "C" fn uprobe_bench(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;
    count += 1;
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
