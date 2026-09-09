/*
 * IP32 basic setup
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2000 Harald Koerfgen
 * Copyright (C) 2002, 2003, 2005 Ilya A. Volynets
 * Copyright (C) 2006 Ralf Baechle <ralf@linux-mips.org>
 */

use core::ffi::{c_char, c_int, c_uint, c_uchar, c_void};

// Linux and MIPS headers supply the declarations referenced below.

#[cfg(feature = "CONFIG_SGI_O2MACE_ETH")]
extern "C" {
    static mut o2meth_eaddr: [c_uchar; 8];
}

#[cfg(feature = "CONFIG_SGI_O2MACE_ETH")]
#[inline]
unsafe fn str2hexnum(c: c_uchar) -> c_uchar {
    if c >= b'0' && c <= b'9' {
        return c.wrapping_sub(b'0');
    }
    if c >= b'a' && c <= b'f' {
        return c.wrapping_sub(b'a').wrapping_add(10);
    }
    0 // foo
}

#[cfg(feature = "CONFIG_SGI_O2MACE_ETH")]
#[inline]
unsafe fn str2eaddr(ea: *mut c_uchar, mut string: *mut c_uchar) {
    for i in 0..6 {
        if *string == b':' {
            string = string.add(1);
        }
        let mut num = str2hexnum(*string.add(0)) << 4;
        string = string.add(1);
        num |= str2hexnum(*string.add(0));
        string = string.add(1);
        *ea.add(i) = num;
    }
}

// An arbitrary time; this can be decreased if reliability looks good.
const WAIT_MS: u32 = 10;

// The concrete definitions are supplied by the platform headers and other
// translation units.
#[repr(C)]
pub struct Crime {
    pub timer: u32,
}

extern "C" {
    static mut crime: *mut Crime;
    static mut mips_hpt_frequency: u32;
    static mut board_be_init: unsafe extern "C" fn();

    fn write_c0_count(value: u32);
    fn read_c0_count() -> u32;
    fn ip32_be_init();
    fn printk(format: *const c_char, ...);
    fn ArcGetEnvironmentVariable(name: *const c_char) -> *mut c_char;
    fn strscpy(dest: *mut c_char, src: *const c_char);
    fn add_preferred_console(
        name: *const c_char,
        index: c_uint,
        options: *const c_char,
    ) -> c_int;
}

#[cfg(feature = "CONFIG_SGI_O2MACE_ETH")]
pub unsafe extern "C" fn plat_mem_setup_eth() {
    let mac = ArcGetEnvironmentVariable(b"eaddr\0".as_ptr() as *const c_char);
    str2eaddr(o2meth_eaddr.as_mut_ptr(), mac as *mut c_uchar);
}

pub unsafe extern "C" fn plat_time_init() {
    printk(b"Calibrating system timer... \0".as_ptr() as *const c_char);
    write_c0_count(0);
    (*crime).timer = 0;
    while (*crime).timer < CRIME_MASTER_FREQ * WAIT_MS / 1000 {}
    mips_hpt_frequency = read_c0_count()
        .wrapping_mul(1000)
        .wrapping_div(WAIT_MS);
    printk(
        b"%d MHz CPU detected\n\0".as_ptr() as *const c_char,
        mips_hpt_frequency.wrapping_mul(2).wrapping_div(1_000_000),
    );
}

pub unsafe extern "C" fn plat_mem_setup() {
    board_be_init = ip32_be_init;

    #[cfg(feature = "CONFIG_SGI_O2MACE_ETH")]
    plat_mem_setup_eth();

    #[cfg(feature = "CONFIG_SERIAL_CORE_CONSOLE")]
    {
        let con = ArcGetEnvironmentVariable(b"console\0".as_ptr() as *const c_char);
        if !con.is_null() && *con == b'd' as c_char {
            static mut OPTIONS: [c_char; 8] = [0; 8];
            let baud = ArcGetEnvironmentVariable(b"dbaud\0".as_ptr() as *const c_char);
            if !baud.is_null() {
                strscpy(OPTIONS.as_mut_ptr(), baud);
            }
            add_preferred_console(
                b"ttyS\0".as_ptr() as *const c_char,
                if *con.add(1) == b'2' as c_char { 1 } else { 0 },
                if !baud.is_null() { OPTIONS.as_ptr() } else { core::ptr::null() },
            );
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
