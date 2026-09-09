// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * MIPS SPRAM support
 *
 * Copyright (C) 2007, 2008 MIPS Technologies, Inc.
 */

const SPRAM_TAG0_ENABLE: u32 = 0x00000080;
const SPRAM_TAG0_PA_MASK: u32 = 0xfffff000;
const SPRAM_TAG1_SIZE_MASK: u32 = 0xfffff000;
const SPRAM_TAG_STRIDE: u32 = 8;
const ERRCTL_SPRAM: u32 = 1 << 28;

extern "C" {
    fn read_c0_errctl() -> u32;
    fn write_c0_errctl(value: u32);
    fn read_c0_taglo() -> u32;
    fn write_c0_taglo(value: u32);
    fn read_c0_dtaglo() -> u32;
    fn write_c0_dtaglo(value: u32);
    fn ehb();
    fn cache_op(op: u32, address: u32);
    fn read_c0_config() -> u32;
    fn current_cpu_type() -> u32;
    fn mb();
    fn strcmp(a: *const i8, b: *const i8) -> i32;
    fn pr_debug(format: *const i8, ...);
    fn pr_info(format: *const i8, ...);
    fn printk(format: *const i8, ...);
}

extern "C" {
    static CKSEG0: u32;
    static CKSEG1: u32;
    static Index_Store_Tag_I: u32;
    static Index_Load_Tag_I: u32;
    static Index_Store_Tag_D: u32;
    static Index_Load_Tag_D: u32;
    static MIPS_CONF_ISP: u32;
    static MIPS_CONF_DSP: u32;
    static CPU_24K: u32;
    static CPU_34K: u32;
    static CPU_74K: u32;
    static CPU_1004K: u32;
    static CPU_1074K: u32;
    static CPU_INTERAPTIV: u32;
    static CPU_PROAPTIV: u32;
    static CPU_P5600: u32;
    static CPU_QEMU_GENERIC: u32;
    static CPU_I6400: u32;
    static CPU_P6600: u32;
}

unsafe fn bis_c0_errctl(set: u32) -> u32 {
    let res = read_c0_errctl();
    write_c0_errctl(res | set);
    res
}

unsafe fn ispram_store_tag(offset: u32, data: u32) {
    let errctl = bis_c0_errctl(ERRCTL_SPRAM);
    ehb();
    write_c0_taglo(data);
    ehb();
    cache_op(Index_Store_Tag_I, CKSEG0 | offset);
    ehb();
    write_c0_errctl(errctl);
    ehb();
}

unsafe fn ispram_load_tag(offset: u32) -> u32 {
    let errctl = bis_c0_errctl(ERRCTL_SPRAM);
    ehb();
    cache_op(Index_Load_Tag_I, CKSEG0 | offset);
    ehb();
    let data = read_c0_taglo();
    ehb();
    write_c0_errctl(errctl);
    ehb();
    data
}

unsafe fn dspram_store_tag(offset: u32, data: u32) {
    let errctl = bis_c0_errctl(ERRCTL_SPRAM);
    ehb();
    write_c0_dtaglo(data);
    ehb();
    cache_op(Index_Store_Tag_D, CKSEG0 | offset);
    ehb();
    write_c0_errctl(errctl);
    ehb();
}

unsafe fn dspram_load_tag(offset: u32) -> u32 {
    let errctl = bis_c0_errctl(ERRCTL_SPRAM);
    ehb();
    cache_op(Index_Load_Tag_D, CKSEG0 | offset);
    ehb();
    let data = read_c0_dtaglo();
    ehb();
    write_c0_errctl(errctl);
    ehb();
    data
}

unsafe fn probe_spram(
    type_: *const i8,
    mut base: u32,
    read: unsafe fn(u32) -> u32,
    write: unsafe fn(u32, u32),
) {
    let mut firstsize = 0;
    let mut lastsize = 0;
    let mut firstpa = 0;
    let mut lastpa = 0;
    let mut pa = 0;
    let mut offset = 0;
    let mut i = 0;
    while i < 8 {
        let mut tag0 = read(offset);
        let tag1 = read(offset + SPRAM_TAG_STRIDE);
        pr_debug(b"DBG %s%d: tag0=%08x tag1=%08x\0".as_ptr() as *const i8, type_, i, tag0, tag1);
        let size = tag1 & SPRAM_TAG1_SIZE_MASK;
        if size == 0 { break; }
        if i != 0 && ((pa == firstpa && size == firstsize) || (pa == lastpa && size == lastsize)) { break; }
        base = (base.wrapping_add(size).wrapping_sub(1)) & !(size.wrapping_sub(1));
        tag0 = (base & SPRAM_TAG0_PA_MASK) | SPRAM_TAG0_ENABLE;
        write(offset, tag0);
        base = base.wrapping_add(size);
        tag0 = read(offset);
        pa = tag0 & SPRAM_TAG0_PA_MASK;
        let enabled = tag0 & SPRAM_TAG0_ENABLE;
        if i == 0 { firstpa = pa; firstsize = size; }
        lastpa = pa; lastsize = size;
        if strcmp(type_, b"DSPRAM\0".as_ptr() as *const i8) == 0 {
            let vp = (CKSEG1 | pa) as *mut u32;
            const TDAT: u32 = 0x5a5aa5a5;
            core::ptr::write_volatile(vp, TDAT);
            core::ptr::write_volatile(vp.add(1), !TDAT);
            mb();
            let v = core::ptr::read_volatile(vp);
            if v != TDAT { printk(b"vp=%p wrote=%08x got=%08x\n\0".as_ptr() as *const i8, vp, TDAT, v); }
            let v = core::ptr::read_volatile(vp.add(1));
            if v != !TDAT { printk(b"vp=%p wrote=%08x got=%08x\n\0".as_ptr() as *const i8, vp.add(1), !TDAT, v); }
        }
        pr_info(b"%s%d: PA=%08x,Size=%08x%s\n\0".as_ptr() as *const i8, type_, i, pa, size, if enabled != 0 { b",enabled\0".as_ptr() } else { b"\0".as_ptr() });
        offset += 2 * SPRAM_TAG_STRIDE;
        i += 1;
    }
}

pub unsafe fn spram_config() {
    let config0 = read_c0_config();
    match current_cpu_type() {
        x if x == CPU_24K || x == CPU_34K || x == CPU_74K || x == CPU_1004K || x == CPU_1074K || x == CPU_INTERAPTIV || x == CPU_PROAPTIV || x == CPU_P5600 || x == CPU_QEMU_GENERIC || x == CPU_I6400 || x == CPU_P6600 => {
            if config0 & MIPS_CONF_ISP != 0 { probe_spram(b"ISPRAM\0".as_ptr() as *const i8, 0x1c000000, ispram_load_tag, ispram_store_tag); }
            if config0 & MIPS_CONF_DSP != 0 { probe_spram(b"DSPRAM\0".as_ptr() as *const i8, 0x1c100000, dspram_load_tag, dspram_store_tag); }
        }
        _ => {}
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
