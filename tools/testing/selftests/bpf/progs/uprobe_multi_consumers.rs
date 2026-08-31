// SPDX-License-Identifier: GPL-2.0
// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>,
// <stdbool.h>, "bpf_kfuncs.h", "bpf_misc.h"

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut uprobe_result: [u64; 4] = [0; 4];

#[unsafe(link_section = "uprobe.multi")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn uprobe_0(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;
    unsafe {
        uprobe_result[0] = uprobe_result[0].wrapping_add(1);
    }
    0
}

#[unsafe(link_section = "uprobe.multi")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn uprobe_1(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;
    unsafe {
        uprobe_result[1] = uprobe_result[1].wrapping_add(1);
    }
    0
}

#[unsafe(link_section = "uprobe.session")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn uprobe_2(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;
    unsafe {
        uprobe_result[2] = uprobe_result[2].wrapping_add(1);
    }
    0
}

#[unsafe(link_section = "uprobe.session")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn uprobe_3(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;
    unsafe {
        uprobe_result[3] = uprobe_result[3].wrapping_add(1);
    }
    1
}
