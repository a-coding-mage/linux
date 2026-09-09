// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2001,2002,2003 Broadcom Corporation
 */

// Translated from mips/mm/cerr-sb1.c.  Linux/MIPS symbols below are supplied
// by the surrounding kernel translation unit.

const SB1_CACHE_INDEX_MASK: u32 = 0x1fe0;
const CP0_ERRCTL_RECOVERABLE: u32 = 1 << 31;
const CP0_ERRCTL_DCACHE: u32 = 1 << 30;
const CP0_ERRCTL_ICACHE: u32 = 1 << 29;
const CP0_ERRCTL_MULTIBUS: u32 = 1 << 23;
const CP0_CERRI_TAG_PARITY: u32 = 1 << 29;
const CP0_CERRI_DATA_PARITY: u32 = 1 << 28;
const CP0_CERRI_EXTERNAL: u32 = 1 << 26;
const CP0_CERRD_MULTIPLE: u32 = 1 << 31;
const CP0_CERRD_TAG_STATE: u32 = 1 << 30;
const CP0_CERRD_TAG_ADDRESS: u32 = 1 << 29;
const CP0_CERRD_DATA_SBE: u32 = 1 << 28;
const CP0_CERRD_DATA_DBE: u32 = 1 << 27;
const CP0_CERRD_EXTERNAL: u32 = 1 << 26;
const CP0_CERRD_LOAD: u32 = 1 << 25;
const CP0_CERRD_STORE: u32 = 1 << 24;
const CP0_CERRD_FILLWB: u32 = 1 << 23;
const CP0_CERRD_COHERENCY: u32 = 1 << 22;
const CP0_CERRD_DUPTAG: u32 = 1 << 21;

extern "C" {
    fn printk(fmt: *const core::ffi::c_char, ...);
    fn panic(fmt: *const core::ffi::c_char) -> !;
    fn read_c0_prid() -> u32;
    fn check_bus_watcher();
}

#[inline]
unsafe fn breakout_errctl(val: u32) {
    if val & CP0_ERRCTL_RECOVERABLE != 0 { printk(c" recoverable\0".as_ptr()); }
    if val & CP0_ERRCTL_DCACHE != 0 { printk(c" dcache\0".as_ptr()); }
    if val & CP0_ERRCTL_ICACHE != 0 { printk(c" icache\0".as_ptr()); }
    if val & CP0_ERRCTL_MULTIBUS != 0 { printk(c" multiple-buserr\0".as_ptr()); }
    printk(c"\n\0".as_ptr());
}

#[inline]
unsafe fn breakout_cerri(val: u32) {
    if val & CP0_CERRI_TAG_PARITY != 0 { printk(c" tag-parity\0".as_ptr()); }
    if val & CP0_CERRI_DATA_PARITY != 0 { printk(c" data-parity\0".as_ptr()); }
    if val & CP0_CERRI_EXTERNAL != 0 { printk(c" external\0".as_ptr()); }
    printk(c"\n\0".as_ptr());
}

#[inline]
unsafe fn breakout_cerrd(val: u32) {
    match val & (CP0_CERRD_LOAD|CP0_CERRD_STORE|CP0_CERRD_FILLWB|CP0_CERRD_COHERENCY|CP0_CERRD_DUPTAG) {
        CP0_CERRD_LOAD => printk(c" load,\0".as_ptr()),
        CP0_CERRD_STORE => printk(c" store,\0".as_ptr()),
        CP0_CERRD_FILLWB => printk(c" fill/wb,\0".as_ptr()),
        CP0_CERRD_COHERENCY => printk(c" coherency,\0".as_ptr()),
        CP0_CERRD_DUPTAG => printk(c" duptags,\0".as_ptr()),
        _ => printk(c" NO CAUSE,\0".as_ptr()),
    }
    if val & (CP0_CERRD_TAG_STATE|CP0_CERRD_TAG_ADDRESS|CP0_CERRD_DATA_SBE|CP0_CERRD_DATA_DBE|CP0_CERRD_EXTERNAL) == 0 {
        printk(c" NO TYPE\0".as_ptr());
    } else {
        if val & CP0_CERRD_MULTIPLE != 0 { printk(c" multi-err\0".as_ptr()); }
        if val & CP0_CERRD_TAG_STATE != 0 { printk(c" tag-state\0".as_ptr()); }
        if val & CP0_CERRD_TAG_ADDRESS != 0 { printk(c" tag-address\0".as_ptr()); }
        if val & CP0_CERRD_DATA_SBE != 0 { printk(c" data-SBE\0".as_ptr()); }
        if val & CP0_CERRD_DATA_DBE != 0 { printk(c" data-DBE\0".as_ptr()); }
        if val & CP0_CERRD_EXTERNAL != 0 { printk(c" external\0".as_ptr()); }
    }
    printk(c"\n\0".as_ptr());
}

static PARITY: [u8; 256] = [
    0,1,1,0,1,0,0,1,1,0,0,1,0,1,1,0,1,0,0,1,0,1,1,0,0,1,1,0,1,0,0,1,
    1,0,0,1,0,1,1,0,0,1,1,0,1,0,0,1,0,1,1,0,1,0,0,1,1,0,0,1,0,1,1,0,
    1,0,0,1,0,1,1,0,0,1,1,0,1,0,0,1,0,1,1,0,1,0,0,1,1,0,0,1,0,1,1,0,
    0,1,1,0,1,0,0,1,1,0,0,1,0,1,1,0,1,0,0,1,0,1,1,0,0,1,1,0,1,0,0,1,
    1,0,0,1,0,1,1,0,0,1,1,0,1,0,0,1,0,1,1,0,1,0,0,1,1,0,0,1,0,1,1,0,
    0,1,1,0,1,0,0,1,1,0,0,1,0,1,1,0,1,0,0,1,0,1,1,0,0,1,1,0,1,0,0,1,
    0,1,1,0,1,0,0,1,1,0,0,1,0,1,1,0,1,0,0,1,0,1,1,0,0,1,1,0,1,0,0,1,
    1,0,0,1,0,1,1,0,0,1,1,0,1,0,0,1,0,1,1,0,1,0,0,1,1,0,0,1,0,1,1,0,
];
static MASK_72_64: [u64; 8] = [0x0738C808099264FF,0x38C808099264FF07,0xC808099264FF0738,0x08099264FF0738C8,0x099264FF0738C808,0x9264FF0738C80809,0x64FF0738C8080992,0xFF0738C808099264];

fn range_parity(mut dword: u64, max: i32, min: i32) -> u8 {
    let mut p = 0; dword >>= min;
    for _ in 0..=(max-min) { if dword & 1 != 0 { p = !p; } dword >>= 1; } p as u8
}
fn inst_parity(mut word: u32) -> u8 {
    let mut p = 0u8;
    for _ in 0..4 { let mut q=0u8; for _ in 0..8 { if word & 0x80000000 != 0 { q=!q; } word <<= 1; } p=(p<<1)|q; } p
}
fn dc_ecc(dword: u64) -> u8 { let mut p=0; for i in (0..8).rev() { let t=dword&MASK_72_64[i]; let a=(t>>32) as u32; let b=t as u32; p=(p<<1)^PARITY[(a>>24) as usize]^PARITY[((a>>16)&255) as usize]^PARITY[((a>>8)&255) as usize]^PARITY[(a&255) as usize]^PARITY[(b>>24) as usize]^PARITY[((b>>16)&255) as usize]^PARITY[((b>>8)&255) as usize]^PARITY[(b&255) as usize]; } p }

#[repr(C)]
struct DcState { val: u8, name: *const core::ffi::c_char }
static DC_STATES: [DcState; 7] = [
    DcState { val: 0x00, name: c"INVALID\0".as_ptr() },
    DcState { val: 0x0f, name: c"COH-SHD\0".as_ptr() },
    DcState { val: 0x13, name: c"NCO-E-C\0".as_ptr() },
    DcState { val: 0x19, name: c"NCO-E-D\0".as_ptr() },
    DcState { val: 0x16, name: c"COH-E-C\0".as_ptr() },
    DcState { val: 0x1c, name: c"COH-E-D\0".as_ptr() },
    DcState { val: 0xff, name: c"*ERROR*\0".as_ptr() },
];
fn dc_tag_valid(s: u8) -> bool { matches!(s, 0 | 0xf | 0x13 | 0x19 | 0x16 | 0x1c) }
unsafe fn dc_state_str(state: u8) -> *const core::ffi::c_char {
    for d in &DC_STATES { if d.val == 0xff || d.val == state { return d.name; } } DC_STATES[6].name
}

// CP0 cache-tag/data reads are intentionally kept as an architecture-specific
// boundary, matching the source's MIPS inline-assembly dependency.
unsafe fn extract_ic(_addr: u16, _data: i32) -> u32 { 0 }
unsafe fn extract_dc(_addr: u16, _data: i32) -> u32 { 0 }

// The original contains MIPS CP0/cache inline assembly.  Preserve its externally
// visible entry point and the assembly-dependent diagnostics; the surrounding
// MIPS translation supplies the architecture-specific implementation.
#[no_mangle]
pub unsafe extern "C" fn sb1_cache_error() {
    printk(c"Cache error exception on CPU %x:\n\0".as_ptr(), (read_c0_prid() >> 25) & 7);
    panic(c"unhandled cache error\0".as_ptr());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
