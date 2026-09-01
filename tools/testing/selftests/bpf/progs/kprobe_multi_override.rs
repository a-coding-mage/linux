// SPDX-License-Identifier: GPL-2.0
// C dependencies: <linux/bpf.h>, <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_override_return(ctx: *mut pt_regs, rc: u64) -> i32;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[unsafe(no_mangle)]
pub static mut pid: i32 = 0;

#[unsafe(link_section = "kprobe.multi")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_override(ctx: *mut pt_regs) -> i32 {
    if (unsafe { bpf_get_current_pid_tgid() } >> 32) != unsafe { pid } as u64 {
        return 0;
    }

    unsafe {
        bpf_override_return(ctx, 123);
    }
    0
}

#[unsafe(link_section = "kprobe")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_kprobe_override(ctx: *mut pt_regs) -> i32 {
    if (unsafe { bpf_get_current_pid_tgid() } >> 32) != unsafe { pid } as u64 {
        return 0;
    }

    unsafe {
        bpf_override_return(ctx, 123);
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
