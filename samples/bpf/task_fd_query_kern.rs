// SPDX-License-Identifier: GPL-2.0
// External kernel/BPF declarations and build-time constants are supplied by
// the surrounding translation environment.

#[no_mangle]
#[link_section = "kprobe/blk_mq_start_request"]
pub unsafe extern "C" fn bpf_prog1(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;
    0
}

#[no_mangle]
#[link_section = "kretprobe/__blk_account_io_done"]
pub unsafe extern "C" fn bpf_prog2(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;
    0
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
#[link_section = "version"]
pub static mut _version: u32 = LINUX_VERSION_CODE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
