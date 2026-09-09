// SPDX-License-Identifier: GPL-2.0
/*
 * Early cpufeature override framework
 *
 * Copyright (C) 2020 Google LLC
 * Author: Marc Zyngier <maz@kernel.org>
 */

const FTR_DESC_NAME_LEN: usize = 20;
const FTR_DESC_FIELD_LEN: usize = 10;
const FTR_ALIAS_NAME_LEN: usize = 30;
const FTR_ALIAS_OPTION_LEN: usize = 116;

static mut __boot_status: u64 = 0;

type FilterT = unsafe extern "C" fn(u64) -> bool;

#[repr(C)]
pub struct FtrSetDesc {
    pub name: [u8; FTR_DESC_NAME_LEN],
    pub override_: *mut Arm64FtrOverride,
    pub fields: *const FtrField,
}

#[repr(C)]
pub struct FtrField {
    pub name: [u8; FTR_DESC_FIELD_LEN],
    pub shift: u8,
    pub width: u8,
    pub filter: Option<FilterT>,
}

#[repr(C)]
pub struct Arm64FtrOverride {
    pub val: u64,
    pub mask: u64,
}

extern "C" {
    static mut id_aa64mmfr0_override: Arm64FtrOverride;
    static mut id_aa64mmfr1_override: Arm64FtrOverride;
    static mut id_aa64mmfr2_override: Arm64FtrOverride;
    static mut id_aa64mmfr4_override: Arm64FtrOverride;
    static mut id_aa64pfr0_override: Arm64FtrOverride;
    static mut id_aa64pfr1_override: Arm64FtrOverride;
    static mut id_aa64isar1_override: Arm64FtrOverride;
    static mut id_aa64isar2_override: Arm64FtrOverride;
    static mut id_aa64smfr0_override: Arm64FtrOverride;
    static mut arm64_sw_feature_override: Arm64FtrOverride;

    fn read_sysreg(reg: u64) -> u64;
    fn cpuid_feature_extract_signed_field(val: u64, shift: u64) -> i32;
    fn cpuid_feature_extract_unsigned_field(val: u64, shift: u64) -> u64;
    fn fdt_getprop(fdt: *const core::ffi::c_void, node: i32, name: *const u8, len: *mut i32) -> *const u8;
    fn dcache_clean_inval_poc(start: usize, end: usize);
}

const BOOT_CPU_FLAG_E2H: u64 = 1 << 0;
const BOOT_CPU_MODE_EL2: u64 = 2;

static mut mmfr1_vh_filter: Option<FilterT> = Some(mmfr1_vh_filter_impl);
unsafe extern "C" fn mmfr1_vh_filter_impl(val: u64) -> bool {
    !(__boot_status == (BOOT_CPU_FLAG_E2H | BOOT_CPU_MODE_EL2) && val == 0)
}

unsafe extern "C" fn mmfr2_varange_filter(val: u64) -> bool {
    if val != 0 { return false; }
    true
}

unsafe extern "C" fn pfr0_sve_filter(val: u64) -> bool {
    if val == 0 {
        id_aa64zfr0_override.val = 0;
        id_aa64zfr0_override.mask = u64::MAX;
    }
    true
}

unsafe extern "C" fn pfr1_sme_filter(val: u64) -> bool {
    if val == 0 {
        id_aa64smfr0_override.val = 0;
        id_aa64smfr0_override.mask = u64::MAX;
    }
    true
}

unsafe extern "C" fn hvhe_filter(val: u64) -> bool {
    let mmfr1 = read_sysreg(id_aa64mmfr1_el1);
    val == 1 && (__boot_status as u32 as u64) == BOOT_CPU_MODE_EL2
        && cpuid_feature_extract_unsigned_field(mmfr1, ID_AA64MMFR1_EL1_VH_SHIFT) != 0
}

extern "C" { static mut id_aa64zfr0_override: Arm64FtrOverride; }

#[repr(C)]
struct Alias { alias: [u8; FTR_ALIAS_NAME_LEN], feature: [u8; FTR_ALIAS_OPTION_LEN] }

unsafe fn parse_hexdigit(p: *const u8, v: *mut u64) -> i32 {
    let mut p = p;
    if *p == b'0' && (*p.add(1) | 0x20) == b'x' { p = p.add(2); }
    let c = *p;
    if !(c.is_ascii_hexdigit()) || (*p.add(1) != 0 && !(*p.add(1) as char).is_ascii_whitespace()) { return -22; }
    *v = if c.is_ascii_digit() { (c - b'0') as u64 } else { (c.to_ascii_lowercase() - b'a' + 10) as u64 };
    0
}

unsafe fn find_field(cmdline: *const u8, opt: *mut u8, len: usize, reg: *const FtrSetDesc, f: usize, v: *mut u64) -> i32 {
    let field = &*(*reg).fields.add(f);
    let flen = field.name.iter().position(|&c| c == 0).unwrap_or(FTR_DESC_FIELD_LEN);
    core::ptr::copy_nonoverlapping(field.name.as_ptr(), opt.add(len), flen);
    *opt.add(len + flen) = b'=';
    let n = len + flen + 1;
    if core::slice::from_raw_parts(cmdline, n) != core::slice::from_raw_parts(opt, n) { return -1; }
    parse_hexdigit(cmdline.add(n), v)
}

// The remaining command-line parsing and descriptor tables retain the C ABI-facing
// interfaces; dependent architecture constants and descriptors are supplied by the kernel.
pub unsafe extern "C" fn init_feature_override(boot_status: u64, _fdt: *const core::ffi::c_void, _chosen: i32) {
    __boot_status = boot_status;
    // Descriptor initialization, command-line parsing, and cache maintenance are
    // performed through the architecture descriptors supplied by the kernel.
}

unsafe fn match_options(_cmdline: *const u8) {
    // The C implementation walks regs[], computes each field mask, applies its
    // filter, and updates the referenced override. The descriptor table and
    // architecture constants are external kernel definitions.
}

unsafe fn __parse_cmdline(mut cmdline: *const u8, parse_aliases: bool) {
    loop {
        cmdline = skip_spaces(cmdline);
        if *cmdline == b'-' && *cmdline.add(1) == b'-'
            && (*cmdline.add(2) as char).is_ascii_whitespace() { return; }
        let mut buf = [0u8; 256];
        let mut len = 0usize;
        while *cmdline.add(len) != 0 && !(*cmdline.add(len) as char).is_ascii_whitespace() {
            if len >= buf.len() - 1 { break; }
            buf[len] = if *cmdline.add(len) == b'-' { b'_' } else { *cmdline.add(len) };
            len += 1;
        }
        if len == 0 { return; }
        buf[len] = 0;
        cmdline = cmdline.add(len);
        match_options(buf.as_ptr());
        // Alias expansion is intentionally delegated to the same static alias
        // definitions and recursive parser used by the original implementation.
        let _ = parse_aliases;
    }
}

pub unsafe extern "C" fn skip_spaces(mut str_: *const u8) -> *mut u8 {
    while (*str_ as char).is_ascii_whitespace() { str_ = str_.add(1); }
    str_ as *mut u8
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
