// SPDX-License-Identifier: GPL-2.0
// C dependencies: "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "license")]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut pids: [u32; 3] = [0; 3];

#[unsafe(no_mangle)]
pub static mut test: [[u32; 2]; 3] = [[0; 2]; 3];

unsafe fn update_pid(idx: i32) {
    let pid: u32 = (unsafe { bpf_get_current_pid_tgid() } >> 32) as u32;
    let idx = idx as usize;

    if pid == unsafe { pids[idx] } {
        unsafe {
            test[idx][0] = test[idx][0].wrapping_add(1);
        }
    } else {
        unsafe {
            test[idx][1] = test[idx][1].wrapping_add(1);
        }
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "uprobe.multi")]
pub unsafe extern "C" fn uprobe_multi_0(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;
    unsafe {
        update_pid(0);
    }
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "uprobe.multi")]
pub unsafe extern "C" fn uprobe_multi_1(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;
    unsafe {
        update_pid(1);
    }
    0
}

#[unsafe(no_mangle)]
#[unsafe(link_section = "uprobe.multi")]
pub unsafe extern "C" fn uprobe_multi_2(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;
    unsafe {
        update_pid(2);
    }
    0
}
