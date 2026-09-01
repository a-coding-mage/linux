// SPDX-License-Identifier: GPL-2.0
// Original C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>
// #include <bpf/bpf_tracing.h>

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

// C source condition: #if defined(__TARGET_ARCH_x86)
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod target_arch_x86 {
    extern "C" {
        pub type pt_regs;
    }

    #[unsafe(link_section = "kprobe")]
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn kprobe_write_ctx(ctx: *mut pt_regs) -> i32 {
        (*(ctx as *mut PtRegsAx)).ax = 0;
        0
    }

    #[unsafe(link_section = "kprobe.multi")]
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn kprobe_multi_write_ctx(ctx: *mut pt_regs) -> i32 {
        (*(ctx as *mut PtRegsAx)).ax = 0;
        0
    }

    #[unsafe(link_section = "?kprobe")]
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn kprobe_dummy(_regs: *mut pt_regs) -> i32 {
        0
    }

    #[unsafe(link_section = "?freplace")]
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn freplace_kprobe(regs: *mut pt_regs) -> i32 {
        (*(regs as *mut PtRegsDi)).di = 0;
        0
    }

    #[unsafe(link_section = "?fentry/bpf_fentry_test1")]
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn fentry() -> i32 {
        0
    }

    #[repr(C)]
    struct PtRegsAx {
        ax: u64,
    }

    #[repr(C)]
    struct PtRegsDi {
        di: u64,
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
