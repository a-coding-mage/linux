// SPDX-License-Identifier: GPL-2.0+
/*
 * CMOS/NV-RAM driver for Atari. Adapted from drivers/char/nvram.c.
 * Copyright (C) 1997 Roman Hodek <Roman.Hodek@informatik.uni-erlangen.de>
 * idea by and with help from Richard Jelinek <rj@suse.de>
 * Portions copyright (c) 2001,2002 Sun Microsystems (thockin@sun.com)
 * Further contributions from Cesar Barros, Erik Gilling, Tim Hockin and
 * Wim Van Sebroeck.
 */

// Kernel dependencies supplied externally:
// linux/errno.h, linux/init.h, linux/mc146818rtc.h, linux/module.h,
// linux/nvram.h, linux/proc_fs.h, linux/seq_file.h, linux/spinlock.h,
// linux/string_choices.h, linux/types.h, asm/atarihw.h, asm/atariints.h

const NVRAM_BYTES: usize = 50;

// These functions access general-purpose NVRAM bytes, adding NVRAM_FIRST_BYTE.
// CMOS_READ and CMOS_WRITE, rtc_lock, and related kernel symbols are external.

unsafe extern "C" {
    static mut rtc_lock: core::ffi::c_void;
    fn CMOS_READ(index: i32) -> u8;
    fn CMOS_WRITE(value: u8, index: i32);
}

unsafe fn __nvram_read_byte(i: i32) -> u8 {
    CMOS_READ(NVRAM_FIRST_BYTE + i)
}

// This races nicely with trying to read with checksum checking.
unsafe fn __nvram_write_byte(c: u8, i: i32) {
    CMOS_WRITE(c, NVRAM_FIRST_BYTE + i);
}

const ATARI_CKS_RANGE_START: i32 = 0;
const ATARI_CKS_RANGE_END: i32 = 47;
const ATARI_CKS_LOC: i32 = 48;

unsafe fn __nvram_check_checksum() -> bool {
    let mut sum: u8 = 0;
    for i in ATARI_CKS_RANGE_START..=ATARI_CKS_RANGE_END {
        sum = sum.wrapping_add(__nvram_read_byte(i));
    }
    __nvram_read_byte(ATARI_CKS_LOC) == !sum
        && __nvram_read_byte(ATARI_CKS_LOC + 1) == sum
}

unsafe fn __nvram_set_checksum() {
    let mut sum: u8 = 0;
    for i in ATARI_CKS_RANGE_START..=ATARI_CKS_RANGE_END {
        sum = sum.wrapping_add(__nvram_read_byte(i));
    }
    __nvram_write_byte(!sum, ATARI_CKS_LOC);
    __nvram_write_byte(sum, ATARI_CKS_LOC + 1);
}

pub unsafe fn atari_nvram_set_checksum() -> i64 {
    spin_lock_irq(&raw mut rtc_lock);
    __nvram_set_checksum();
    spin_unlock_irq(&raw mut rtc_lock);
    0
}

pub unsafe fn atari_nvram_initialize() -> i64 {
    spin_lock_irq(&raw mut rtc_lock);
    for i in 0..NVRAM_BYTES as i32 {
        __nvram_write_byte(0, i);
    }
    __nvram_set_checksum();
    spin_unlock_irq(&raw mut rtc_lock);
    0
}

pub unsafe fn atari_nvram_read(buf: *mut i8, count: usize, ppos: *mut i64) -> isize {
    let mut p = buf;
    let mut remaining = count;
    let mut i = *ppos;
    spin_lock_irq(&raw mut rtc_lock);
    if !__nvram_check_checksum() {
        spin_unlock_irq(&raw mut rtc_lock);
        return -(EIO as isize);
    }
    while remaining > 0 && i < NVRAM_BYTES as i64 {
        *p = __nvram_read_byte(i as i32) as i8;
        remaining -= 1;
        i += 1;
        p = p.add(1);
    }
    spin_unlock_irq(&raw mut rtc_lock);
    *ppos = i;
    p.offset_from(buf)
}

pub unsafe fn atari_nvram_write(buf: *mut i8, count: usize, ppos: *mut i64) -> isize {
    let mut p = buf;
    let mut remaining = count;
    let mut i = *ppos;
    spin_lock_irq(&raw mut rtc_lock);
    if !__nvram_check_checksum() {
        spin_unlock_irq(&raw mut rtc_lock);
        return -(EIO as isize);
    }
    while remaining > 0 && i < NVRAM_BYTES as i64 {
        __nvram_write_byte(*p as u8, i as i32);
        remaining -= 1;
        i += 1;
        p = p.add(1);
    }
    __nvram_set_checksum();
    spin_unlock_irq(&raw mut rtc_lock);
    *ppos = i;
    p.offset_from(buf)
}

pub fn atari_nvram_get_size() -> isize {
    NVRAM_BYTES as isize
}

// The CONFIG_PROC_FS section is retained below; kernel proc/seq symbols are external.
#[cfg(CONFIG_PROC_FS)]
mod proc_fs {
    use super::*;

    struct BootPref { val: u8, name: &'static str }
    static BOOT_PREFS: [BootPref; 5] = [
        BootPref { val: 0x80, name: "TOS" },
        BootPref { val: 0x40, name: "ASV" },
        BootPref { val: 0x20, name: "NetBSD (?)" },
        BootPref { val: 0x10, name: "Linux" },
        BootPref { val: 0x00, name: "unspecified" },
    ];
    static LANGUAGES: [&str; 9] = ["English (US)", "German", "French", "English (UK)", "Spanish", "Italian", "6 (undefined)", "Swiss (French)", "Swiss (German)"];
    static DATEFORMAT: [&str; 8] = ["MM%cDD%cYY", "DD%cMM%cYY", "YY%cMM%cDD", "YY%cDD%cMM", "4 (undefined)", "5 (undefined)", "6 (undefined)", "7 (undefined)"];
    static COLORS: [&str; 8] = ["2", "4", "16", "256", "65536", "??", "??", "??"];

    #[repr(C)] pub struct SeqFile { _private: [u8; 0] }
    unsafe extern "C" {
        fn seq_printf(seq: *mut SeqFile, fmt: *const i8, ...);
        fn seq_puts(seq: *mut SeqFile, s: *const i8);
        fn proc_create_single(name: *const i8, mode: u32, parent: *mut core::ffi::c_void, show: unsafe extern "C" fn(*mut SeqFile, *mut core::ffi::c_void) -> i32) -> *mut core::ffi::c_void;
        fn pr_err(fmt: *const i8, ...);
        fn spin_lock_irq(lock: *mut core::ffi::c_void);
        fn spin_unlock_irq(lock: *mut core::ffi::c_void);
        fn str_on_off(value: bool) -> *const i8;
    }
    const NVRAM_FIRST_BYTE: i32 = 0; // supplied by the Atari kernel headers
    const MACH_IS_FALCON: bool = true; // supplied by the Atari kernel headers

    unsafe fn atari_nvram_proc_read(nvram: *const u8, seq: *mut SeqFile, _offset: *mut core::ffi::c_void) {
        let checksum;
        spin_lock_irq(&raw mut rtc_lock);
        checksum = __nvram_check_checksum();
        spin_unlock_irq(&raw mut rtc_lock);
        seq_printf(seq, c"Checksum status  : %svalid\n".as_ptr(), if checksum { c"".as_ptr() } else { c"not ".as_ptr() });
        seq_puts(seq, c"Boot preference  : ".as_ptr());
        let mut i = BOOT_PREFS.len() as isize - 1;
        while i >= 0 {
            if *nvram.add(1) == BOOT_PREFS[i as usize].val {
                seq_printf(seq, c"%s\n".as_ptr(), BOOT_PREFS[i as usize].name.as_ptr());
                break;
            }
            i -= 1;
        }
        if i < 0 { seq_printf(seq, c"0x%02x (undefined)\n".as_ptr(), *nvram.add(1)); }
        seq_printf(seq, c"SCSI arbitration : %s\n".as_ptr(), str_on_off((*nvram.add(16) & 0x80) != 0));
        seq_puts(seq, c"SCSI host ID     : ".as_ptr());
        if *nvram.add(16) & 0x80 != 0 { seq_printf(seq, c"%d\n".as_ptr(), *nvram.add(16) & 7); } else { seq_puts(seq, c"n/a\n".as_ptr()); }
        if !MACH_IS_FALCON { return; }
        seq_puts(seq, c"OS language      : ".as_ptr());
        if (*nvram.add(6) as usize) < LANGUAGES.len() { seq_printf(seq, c"%s\n".as_ptr(), LANGUAGES[*nvram.add(6) as usize].as_ptr()); } else { seq_printf(seq, c"%u (undefined)\n".as_ptr(), *nvram.add(6)); }
        seq_puts(seq, c"Keyboard language: ".as_ptr());
        if (*nvram.add(7) as usize) < LANGUAGES.len() { seq_printf(seq, c"%s\n".as_ptr(), LANGUAGES[*nvram.add(7) as usize].as_ptr()); } else { seq_printf(seq, c"%u (undefined)\n".as_ptr(), *nvram.add(7)); }
        seq_puts(seq, c"Date format      : ".as_ptr());
        seq_printf(seq, DATEFORMAT[(*nvram.add(8) & 7) as usize].as_ptr(), if *nvram.add(9) != 0 { *nvram.add(9) } else { b'/' }, if *nvram.add(9) != 0 { *nvram.add(9) } else { b'/' });
        seq_printf(seq, c", %dh clock\n".as_ptr(), if *nvram.add(8) & 16 != 0 { 24 } else { 12 });
        seq_puts(seq, c"Boot delay       : ".as_ptr());
        if *nvram.add(10) == 0 { seq_puts(seq, c"default\n".as_ptr()); } else { seq_printf(seq, c"%ds%s\n".as_ptr(), *nvram.add(10), if *nvram.add(10) < 8 { c", no memory test".as_ptr() } else { c"".as_ptr() }); }
        let vmode = ((*nvram.add(14) as u32) << 8) | *nvram.add(15) as u32;
        seq_printf(seq, c"Video mode       : %s colors, %d columns, %s %s monitor\n".as_ptr(), COLORS[(vmode & 7) as usize].as_ptr(), if vmode & 8 != 0 { 80 } else { 40 }, if vmode & 16 != 0 { c"VGA".as_ptr() } else { c"TV".as_ptr() }, if vmode & 32 != 0 { c"PAL".as_ptr() } else { c"NTSC".as_ptr() });
        seq_printf(seq, c"                   %soverscan, compat. mode %s%s\n".as_ptr(), if vmode & 64 != 0 { c"".as_ptr() } else { c"no ".as_ptr() }, str_on_off(vmode & 128 != 0), if vmode & 256 != 0 { if vmode & 16 != 0 { c", line doubling".as_ptr() } else { c", half screen".as_ptr() } } else { c"".as_ptr() });
    }

    #[allow(dead_code)]
    unsafe extern "C" fn nvram_proc_read(seq: *mut SeqFile, offset: *mut core::ffi::c_void) -> i32 {
        let mut contents = [0u8; NVRAM_BYTES];
        spin_lock_irq(&raw mut rtc_lock);
        for i in 0..NVRAM_BYTES { contents[i] = __nvram_read_byte(i as i32); }
        spin_unlock_irq(&raw mut rtc_lock);
        atari_nvram_proc_read(contents.as_ptr(), seq, offset);
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
