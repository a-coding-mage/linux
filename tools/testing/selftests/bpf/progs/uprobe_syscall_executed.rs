// SPDX-License-Identifier: GPL-2.0
// Translated from C. Original dependencies:
// "vmlinux.h", <bpf/bpf_helpers.h>, <bpf/bpf_tracing.h>, <bpf/usdt.bpf.h>,
// and <string.h>.

extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[no_mangle]
pub static mut regs: pt_regs = pt_regs { _private: [] };

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

#[no_mangle]
pub static mut executed: i32 = 0;

#[no_mangle]
pub static mut pid: i32 = 0;

#[no_mangle]
#[link_section = "uprobe"]
pub unsafe extern "C" fn test_uprobe(_ctx: *mut pt_regs) -> i32 {
    if (bpf_get_current_pid_tgid() >> 32) != pid as u64 {
        return 0;
    }

    executed += 1;
    0
}

#[no_mangle]
#[link_section = "uretprobe"]
pub unsafe extern "C" fn test_uretprobe(_ctx: *mut pt_regs) -> i32 {
    if (bpf_get_current_pid_tgid() >> 32) != pid as u64 {
        return 0;
    }

    executed += 1;
    0
}

#[no_mangle]
#[link_section = "uprobe.multi"]
pub unsafe extern "C" fn test_uprobe_multi(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;

    if (bpf_get_current_pid_tgid() >> 32) != pid as u64 {
        return 0;
    }

    executed += 1;
    0
}

#[no_mangle]
#[link_section = "uretprobe.multi"]
pub unsafe extern "C" fn test_uretprobe_multi(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;

    if (bpf_get_current_pid_tgid() >> 32) != pid as u64 {
        return 0;
    }

    executed += 1;
    0
}

#[no_mangle]
#[link_section = "uprobe.session"]
pub unsafe extern "C" fn test_uprobe_session(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;

    if (bpf_get_current_pid_tgid() >> 32) != pid as u64 {
        return 0;
    }

    executed += 1;
    0
}

#[no_mangle]
#[link_section = "usdt"]
pub unsafe extern "C" fn test_usdt(ctx: *mut pt_regs) -> i32 {
    let _ = ctx;

    if (bpf_get_current_pid_tgid() >> 32) != pid as u64 {
        return 0;
    }

    executed += 1;
    0
}
