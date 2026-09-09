// SPDX-License-Identifier: GPL-2.0-only
/* -*- linux-c -*- ------------------------------------------------------- *
 *
 *   Copyright (C) 1991, 1992 Linus Torvalds
 *   Copyright 2007 rPath, Inc. - All Rights Reserved
 *
 * ----------------------------------------------------------------------- */

/*
 * Check for obligatory CPU features and abort if the features are not
 * present.  This code should be compilable as 16-, 32- or 64-bit
 * code, so be very careful with types and inline assembly.
 *
 * This code should not contain any messages; that requires an
 * additional wrapper.
 *
 * As written, this code is not safe for inclusion into the kernel
 * proper (after FPU initialization, in particular).
 */

// C headers and build-time configuration are supplied by the surrounding tree.

const NCAPINTS: usize = 17;

#[repr(C)]
pub struct Msr {
    pub l: u64,
    pub h: u64,
}

#[repr(C)]
pub struct CpuInfo {
    pub flags: [u32; NCAPINTS],
    pub level: i32,
    pub family: i32,
    pub model: i32,
}

extern "C" {
    static mut cpu: CpuInfo;
    static cpu_vendor: [u32; 3];
    static REQUIRED_MASK0: u32;
    static REQUIRED_MASK1: u32;
    static REQUIRED_MASK4: u32;
    static REQUIRED_MASK6: u32;
    static REQUIRED_MASK16: u32;
    static CONFIG_X86_MINIMUM_CPU_FAMILY: i32;
    static X86_EFLAGS_AC: u32;
    static X86_FEATURE_LM: u32;
    static X86_FEATURE_XMM: u32;
    static X86_FEATURE_XMM2: u32;
    static X86_FEATURE_CX8: u32;
    static X86_FEATURE_PAE: u32;
    static MSR_K7_HWCR: u32;
    static MSR_VIA_FCR: u32;
    static IS_ENABLED_CONFIG_X86_64: i32;
    static IS_ENABLED_CONFIG_X86_PAE: i32;

    fn memset(s: *mut core::ffi::c_void, c: i32, n: usize) -> *mut core::ffi::c_void;
    fn has_eflag(flag: u32) -> i32;
    fn get_cpuflags();
    fn test_bit(bit: u32, addr: *const u32) -> i32;
    fn set_bit(bit: u32, addr: *mut u32);
    fn raw_rdmsr(msr: u32, m: *mut Msr);
    fn raw_wrmsr(msr: u32, m: *const Msr);
    fn cmdline_find_option_bool(option: *const u8) -> i32;
    fn puts(s: *const u8);
}

static mut err_flags: [u32; NCAPINTS] = [0; NCAPINTS];

fn req_level() -> i32 {
    unsafe { CONFIG_X86_MINIMUM_CPU_FAMILY }
}

fn req_flags() -> [u32; NCAPINTS] {
    unsafe {
        [
            REQUIRED_MASK0, REQUIRED_MASK1, 0, 0, REQUIRED_MASK4, 0,
            REQUIRED_MASK6, 0, 0, 0, 0, 0, 0, 0, 0, 0, REQUIRED_MASK16,
        ]
    }
}

fn a32(a: u32, b: u32, c: u32, d: u32) -> u32 {
    (d << 24) + (c << 16) + (b << 8) + a
}

unsafe fn is_amd() -> i32 {
    (cpu_vendor[0] == a32(b'A' as u32, b'u' as u32, b't' as u32, b'h' as u32)
        && cpu_vendor[1] == a32(b'e' as u32, b'n' as u32, b't' as u32, b'i' as u32)
        && cpu_vendor[2] == a32(b'c' as u32, b'A' as u32, b'M' as u32, b'D' as u32)) as i32
}

unsafe fn is_centaur() -> i32 {
    (cpu_vendor[0] == a32(b'C' as u32, b'e' as u32, b'n' as u32, b't' as u32)
        && cpu_vendor[1] == a32(b'a' as u32, b'u' as u32, b'r' as u32, b'H' as u32)
        && cpu_vendor[2] == a32(b'a' as u32, b'u' as u32, b'l' as u32, b's' as u32)) as i32
}

unsafe fn is_transmeta() -> i32 {
    (cpu_vendor[0] == a32(b'G' as u32, b'e' as u32, b'n' as u32, b'u' as u32)
        && cpu_vendor[1] == a32(b'i' as u32, b'n' as u32, b'e' as u32, b'T' as u32)
        && cpu_vendor[2] == a32(b'M' as u32, b'x' as u32, b'8' as u32, b'6' as u32)) as i32
}

unsafe fn is_intel() -> i32 {
    (cpu_vendor[0] == a32(b'G' as u32, b'e' as u32, b'n' as u32, b'u' as u32)
        && cpu_vendor[1] == a32(b'i' as u32, b'n' as u32, b'e' as u32, b'I' as u32)
        && cpu_vendor[2] == a32(b'n' as u32, b't' as u32, b'e' as u32, b'l' as u32)) as i32
}

unsafe fn check_cpuflags() -> i32 {
    let mut err: u32 = 0;
    let flags = req_flags();
    for i in 0..NCAPINTS {
        err_flags[i] = flags[i] & !cpu.flags[i];
        if err_flags[i] != 0 { err |= 1u32 << i; }
    }
    err as i32
}

pub unsafe fn check_cpu(cpu_level_ptr: *mut i32, req_level_ptr: *mut i32, err_flags_ptr: *mut *mut u32) -> i32 {
    let mut err: i32;
    memset(cpu.flags.as_mut_ptr() as *mut _, 0, core::mem::size_of_val(&cpu.flags));
    cpu.level = 3;
    if has_eflag(X86_EFLAGS_AC) != 0 { cpu.level = 4; }
    get_cpuflags();
    err = check_cpuflags();
    if test_bit(X86_FEATURE_LM, cpu.flags.as_ptr()) != 0 { cpu.level = 64; }

    if err == 0x01 && (err_flags[0] & !((1 << X86_FEATURE_XMM) | (1 << X86_FEATURE_XMM2))) == 0 && is_amd() != 0 {
        let mut m = Msr { l: 0, h: 0 }; raw_rdmsr(MSR_K7_HWCR, &mut m); m.l &= !(1 << 15); raw_wrmsr(MSR_K7_HWCR, &m); get_cpuflags(); err = check_cpuflags();
    } else if err == 0x01 && (err_flags[0] & !(1 << X86_FEATURE_CX8)) == 0 && is_centaur() != 0 && cpu.model >= 6 {
        let mut m = Msr { l: 0, h: 0 }; raw_rdmsr(MSR_VIA_FCR, &mut m); m.l |= (1 << 1) | (1 << 7); raw_wrmsr(MSR_VIA_FCR, &m); set_bit(X86_FEATURE_CX8, cpu.flags.as_mut_ptr()); err = check_cpuflags();
    } else if err == 0x01 && is_transmeta() != 0 {
        let mut m = Msr { l: 0, h: 0 }; let mut m_tmp; let mut level: u32 = 1; raw_rdmsr(0x80860004, &mut m); m_tmp = m; m_tmp.l = !0; raw_wrmsr(0x80860004, &m_tmp);
        core::arch::asm!("cpuid", inout("eax") level, lateout("edx") cpu.flags[0], lateout("ecx") _, lateout("ebx") _);
        raw_wrmsr(0x80860004, &m); err = check_cpuflags();
    } else if err == 0x01 && (err_flags[0] & !(1 << X86_FEATURE_PAE)) == 0 && is_intel() != 0 && cpu.level == 6 && (cpu.model == 9 || cpu.model == 13) {
        if cmdline_find_option_bool(b"forcepae\0".as_ptr()) != 0 { puts(b"WARNING: Forcing PAE in CPU flags\n\0".as_ptr()); set_bit(X86_FEATURE_PAE, cpu.flags.as_mut_ptr()); err = check_cpuflags(); } else { puts(b"WARNING: PAE disabled. Use parameter 'forcepae' to enable at your own risk!\n\0".as_ptr()); }
    }
    if err == 0 { err = check_knl_erratum(); }
    if !err_flags_ptr.is_null() { *err_flags_ptr = if err != 0 { err_flags.as_mut_ptr() } else { core::ptr::null_mut() }; }
    if !cpu_level_ptr.is_null() { *cpu_level_ptr = cpu.level; }
    if !req_level_ptr.is_null() { *req_level_ptr = req_level(); }
    if cpu.level < req_level() || err != 0 { -1 } else { 0 }
}

pub unsafe fn check_knl_erratum() -> i32 {
    if is_intel() == 0 || cpu.family != 6 || cpu.model != 0x57 { return 0; }
    // CONFIG_X86_64 or CONFIG_X86_PAE enables the required PTE width.
    if IS_ENABLED_CONFIG_X86_64 != 0 || IS_ENABLED_CONFIG_X86_PAE != 0 { return 0; }
    puts(b"This 32-bit kernel can not run on this Xeon Phi x200\nprocessor due to a processor erratum.  Use a 64-bit\nkernel, or enable PAE in this 32-bit kernel.\n\n\0".as_ptr());
    -1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
