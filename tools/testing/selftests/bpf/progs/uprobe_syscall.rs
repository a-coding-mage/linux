// SPDX-License-Identifier: GPL-2.0
// C dependencies: "vmlinux.h", <bpf/bpf_helpers.h>, <string.h>

use core::mem::size_of;
use core::ptr;

use crate::vmlinux::pt_regs;

#[no_mangle]
pub static mut regs: pt_regs = unsafe { core::mem::zeroed() };

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
#[link_section = "uprobe"]
pub unsafe extern "C" fn probe(ctx: *mut pt_regs) -> i32 {
    ptr::copy_nonoverlapping(
        ctx as *const u8,
        ptr::addr_of_mut!(regs) as *mut u8,
        size_of::<pt_regs>(),
    );
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
