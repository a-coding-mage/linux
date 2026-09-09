// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding translation unit:
// cpuflags.h, string.h, io.h, error.h, vdso/limits.h, uapi/asm/vmx.h,
// and asm/shared/tdx.h.

use crate::{
    cpuid_count, error, hcall_func, memcmp, pio_ops, __tdx_hypercall,
    tdx_module_args, EXIT_REASON_IO_INSTRUCTION, TDX_CPUID_LEAF_ID,
    TDX_HYPERCALL_STANDARD, TDX_IDENT,
};

/* Called from __tdx_hypercall() for unrecoverable failure */
pub unsafe fn __tdx_hypercall_failed() {
    error("TDVMCALL failed. TDX module bug?");
}

unsafe fn tdx_io_in(size: u32, port: u16) -> u32 {
    let mut args = tdx_module_args {
        r10: TDX_HYPERCALL_STANDARD,
        r11: hcall_func(EXIT_REASON_IO_INSTRUCTION),
        r12: size,
        r13: 0,
        r14: port,
        ..core::mem::zeroed()
    };

    if __tdx_hypercall(&mut args) != 0 {
        return u32::MAX;
    }

    args.r11
}

unsafe fn tdx_io_out(size: u32, port: u16, value: u32) {
    let mut args = tdx_module_args {
        r10: TDX_HYPERCALL_STANDARD,
        r11: hcall_func(EXIT_REASON_IO_INSTRUCTION),
        r12: size,
        r13: 1,
        r14: port,
        r15: value,
        ..core::mem::zeroed()
    };

    __tdx_hypercall(&mut args);
}

unsafe fn tdx_inb(port: u16) -> u8 {
    tdx_io_in(1, port) as u8
}

unsafe fn tdx_outb(value: u8, port: u16) {
    tdx_io_out(1, port, value as u32);
}

unsafe fn tdx_outw(value: u16, port: u16) {
    tdx_io_out(2, port, value as u32);
}

pub unsafe fn early_tdx_detect() {
    let mut eax: u32 = 0;
    let mut sig = [0u32; 3];

    cpuid_count(
        TDX_CPUID_LEAF_ID,
        0,
        &mut eax,
        &mut sig[0],
        &mut sig[2],
        &mut sig[1],
    );

    if memcmp(TDX_IDENT.as_ptr(), sig.as_ptr(), core::mem::size_of_val(&sig)) != 0 {
        return;
    }

    /* Use hypercalls instead of I/O instructions */
    pio_ops.f_inb = Some(tdx_inb);
    pio_ops.f_outb = Some(tdx_outb);
    pio_ops.f_outw = Some(tdx_outw);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
