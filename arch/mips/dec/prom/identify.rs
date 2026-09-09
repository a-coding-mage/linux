// SPDX-License-Identifier: GPL-2.0
/*
 * identify.c: machine identification code.
 *
 * Copyright (C) 1998 Harald Koerfgen and Paul M. Antoine
 * Copyright (C) 2002, 2003, 2004, 2005  Maciej W. Rozycki
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

// C headers supplied by the surrounding kernel translation.

extern "C" {
    static mut mips_machtype: c_int;
    static mut dec_kn_slot_base: c_uint;
    static mut dec_kn_slot_size: c_uint;
    static mut dec_tc_bus: c_uint;
    static mut ioasic_base: *mut c_void;

    fn prom_is_rex(magic: u32) -> c_int;
    fn prom_getenv(name: *const c_char) -> *mut c_char;
    fn simple_strtoul(cp: *const c_char, endp: *mut *mut c_char, base: c_uint) -> u32;
    fn rex_getsysid() -> u32;
    fn printk(format: *const c_char, ...) -> c_int;
    fn snprintf(buf: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;
    fn ioasic_read(reg: c_uint) -> u32;
}

static DEC_SYSTEM_STRINGS: [*const c_char; 11] = [
    b"unknown DECstation\0".as_ptr() as *const c_char,
    b"DECstation 2100/3100\0".as_ptr() as *const c_char,
    b"DECsystem 5100\0".as_ptr() as *const c_char,
    b"DECstation 5000/200\0".as_ptr() as *const c_char,
    b"DECstation 5000/1xx\0".as_ptr() as *const c_char,
    b"Personal DECstation 5000/xx\0".as_ptr() as *const c_char,
    b"DECstation 5000/2x0\0".as_ptr() as *const c_char,
    b"DECsystem 5400\0".as_ptr() as *const c_char,
    b"DECsystem 5500\0".as_ptr() as *const c_char,
    b"DECsystem 5800\0".as_ptr() as *const c_char,
    b"DECsystem 5900\0".as_ptr() as *const c_char,
];

pub unsafe fn get_system_type() -> *const c_char {
    const STR_BUF_LEN: usize = 64;
    static mut SYSTEM: [u8; STR_BUF_LEN] = [0; STR_BUF_LEN];
    static mut CALLED: c_int = 0;

    if CALLED == 0 {
        CALLED = 1;
        snprintf(
            SYSTEM.as_mut_ptr() as *mut c_char,
            STR_BUF_LEN,
            b"Digital %s\0".as_ptr() as *const c_char,
            DEC_SYSTEM_STRINGS[mips_machtype as usize],
        );
    }

    SYSTEM.as_ptr() as *const c_char
}

/* Setup essential system-specific memory addresses. */
pub static mut dec_rtc_base: *mut u8 = core::ptr::null_mut();

#[inline]
unsafe fn prom_init_kn01() {
    dec_kn_slot_base = KN01_SLOT_BASE;
    dec_kn_slot_size = KN01_SLOT_SIZE;
    dec_rtc_base = CKSEG1ADDR(dec_kn_slot_base.wrapping_add(KN01_RTC)) as *mut u8;
}

#[inline]
unsafe fn prom_init_kn230() {
    dec_kn_slot_base = KN01_SLOT_BASE;
    dec_kn_slot_size = KN01_SLOT_SIZE;
    dec_rtc_base = CKSEG1ADDR(dec_kn_slot_base.wrapping_add(KN01_RTC)) as *mut u8;
}

#[inline]
unsafe fn prom_init_kn02() {
    dec_kn_slot_base = KN02_SLOT_BASE;
    dec_kn_slot_size = KN02_SLOT_SIZE;
    dec_tc_bus = 1;
    dec_rtc_base = CKSEG1ADDR(dec_kn_slot_base.wrapping_add(KN02_RTC)) as *mut u8;
}

#[inline]
unsafe fn prom_init_kn02xa() {
    dec_kn_slot_base = KN02XA_SLOT_BASE;
    dec_kn_slot_size = IOASIC_SLOT_SIZE;
    dec_tc_bus = 1;
    ioasic_base = CKSEG1ADDR(dec_kn_slot_base.wrapping_add(IOASIC_IOCTL)) as *mut c_void;
    dec_rtc_base = CKSEG1ADDR(dec_kn_slot_base.wrapping_add(IOASIC_TOY)) as *mut u8;
}

#[inline]
unsafe fn prom_init_kn03() {
    dec_kn_slot_base = KN03_SLOT_BASE;
    dec_kn_slot_size = IOASIC_SLOT_SIZE;
    dec_tc_bus = 1;
    ioasic_base = CKSEG1ADDR(dec_kn_slot_base.wrapping_add(IOASIC_IOCTL)) as *mut c_void;
    dec_rtc_base = CKSEG1ADDR(dec_kn_slot_base.wrapping_add(IOASIC_TOY)) as *mut u8;
}

pub unsafe fn prom_identify_arch(magic: u32) {
    let mut dec_cpunum: u8;
    let mut dec_firmrev: u8;
    let mut dec_etc: u8;
    let mut dec_systype: u8;
    let mut dec_sysid: u32;

    if prom_is_rex(magic) == 0 {
        dec_sysid = simple_strtoul(prom_getenv(b"systype\0".as_ptr() as *const c_char), core::ptr::null_mut(), 0);
    } else {
        dec_sysid = rex_getsysid();
        if dec_sysid == 0 {
            printk(b"Zero sysid returned from PROM! Assuming a PMAX-like machine.\n\0".as_ptr() as *const c_char);
            dec_sysid = 1;
        }
    }

    dec_cpunum = ((dec_sysid & 0xff000000) >> 24) as u8;
    dec_systype = ((dec_sysid & 0xff0000) >> 16) as u8;
    dec_firmrev = ((dec_sysid & 0xff00) >> 8) as u8;
    dec_etc = (dec_sysid & 0xff) as u8;

    match dec_systype {
        DS2100_3100 => { mips_machtype = MACH_DS23100; prom_init_kn01(); }
        DS5100 => { mips_machtype = MACH_DS5100; prom_init_kn230(); }
        DS5000_200 => { mips_machtype = MACH_DS5000_200; prom_init_kn02(); }
        DS5000_1XX => { mips_machtype = MACH_DS5000_1XX; prom_init_kn02xa(); }
        DS5000_2X0 => {
            mips_machtype = MACH_DS5000_2X0;
            prom_init_kn03();
            if ioasic_read(IO_REG_SIR) & KN03_IO_INR_3MAXP == 0 { mips_machtype = MACH_DS5900; }
        }
        DS5000_XX => { mips_machtype = MACH_DS5000_XX; prom_init_kn02xa(); }
        DS5800 => { mips_machtype = MACH_DS5800; }
        DS5400 => { mips_machtype = MACH_DS5400; }
        DS5500 => { mips_machtype = MACH_DS5500; }
        _ => { mips_machtype = MACH_DSUNKNOWN; }
    }

    if mips_machtype == MACH_DSUNKNOWN {
        printk(b"This is an %s, id is %x\n\0".as_ptr() as *const c_char, DEC_SYSTEM_STRINGS[mips_machtype as usize], dec_systype);
    } else {
        printk(b"This is a %s\n\0".as_ptr() as *const c_char, DEC_SYSTEM_STRINGS[mips_machtype as usize]);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
