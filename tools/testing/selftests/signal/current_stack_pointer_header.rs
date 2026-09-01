/* SPDX-License-Identifier: GPL-2.0 */

/*
 * C source intent:
 *
 * register unsigned long sp asm("<arch stack pointer register>");
 *
 * Rust has no direct stable equivalent for a file-scope C global register
 * variable. Preserve the externally meaningful local name by exposing `sp()`,
 * which reads the architecture stack pointer register selected by the same
 * conditions.
 */

#[cfg(target_arch = "alpha")]
pub unsafe fn sp() -> core::ffi::c_ulong {
    let sp: core::ffi::c_ulong;
    unsafe {
        core::arch::asm!("mov {}, $30", out(reg) sp);
    }
    sp
}

#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "csky",
    target_arch = "m68k",
    target_arch = "mips",
    target_arch = "riscv32",
    target_arch = "riscv64"
))]
pub unsafe fn sp() -> core::ffi::c_ulong {
    let sp: core::ffi::c_ulong;
    unsafe {
        core::arch::asm!("mov {}, sp", out(reg) sp);
    }
    sp
}

#[cfg(target_arch = "x86")]
pub unsafe fn sp() -> core::ffi::c_ulong {
    let sp: core::ffi::c_ulong;
    unsafe {
        core::arch::asm!("mov {}, esp", out(reg) sp);
    }
    sp
}

#[cfg(target_arch = "loongarch64")]
pub unsafe fn sp() -> core::ffi::c_ulong {
    let sp: core::ffi::c_ulong;
    unsafe {
        core::arch::asm!("move {}, $sp", out(reg) sp);
    }
    sp
}

#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
pub unsafe fn sp() -> core::ffi::c_ulong {
    let sp: core::ffi::c_ulong;
    unsafe {
        core::arch::asm!("mr {}, 1", out(reg) sp);
    }
    sp
}

#[cfg(target_arch = "s390x")]
pub unsafe fn sp() -> core::ffi::c_ulong {
    let sp: core::ffi::c_ulong;
    unsafe {
        core::arch::asm!("lgr {}, %r15", out(reg) sp);
    }
    sp
}

#[cfg(target_arch = "sh")]
pub unsafe fn sp() -> core::ffi::c_ulong {
    let sp: core::ffi::c_ulong;
    unsafe {
        core::arch::asm!("mov {}, r15", out(reg) sp);
    }
    sp
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn sp() -> core::ffi::c_ulong {
    let sp: core::ffi::c_ulong;
    unsafe {
        core::arch::asm!("mov {}, rsp", out(reg) sp);
    }
    sp
}

#[cfg(target_arch = "xtensa")]
pub unsafe fn sp() -> core::ffi::c_ulong {
    let sp: core::ffi::c_ulong;
    unsafe {
        core::arch::asm!("mov {}, a1", out(reg) sp);
    }
    sp
}

#[cfg(not(any(
    target_arch = "alpha",
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "csky",
    target_arch = "m68k",
    target_arch = "mips",
    target_arch = "riscv32",
    target_arch = "riscv64",
    target_arch = "x86",
    target_arch = "loongarch64",
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "s390x",
    target_arch = "sh",
    target_arch = "x86_64",
    target_arch = "xtensa"
)))]
compile_error!("implement current_stack_pointer equivalent");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
