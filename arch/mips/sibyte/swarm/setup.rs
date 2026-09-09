// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2000, 2001, 2002, 2003, 2004 Broadcom Corporation
 * Copyright (C) 2004 by Ralf Baechle (ralf@linux-mips.org)
 */

/*
 * Setup code for the SWARM board
 */

// Linux and MIPS headers supply the declarations and constants referenced below.

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct timespec64 {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

extern "C" {
    #[cfg(any(CONFIG_SIBYTE_BCM1x80))]
    fn bcm1480_setup();
    #[cfg(any(CONFIG_SIBYTE_SB1250, CONFIG_SIBYTE_BCM112X))]
    fn sb1250_setup();

    fn xicor_probe() -> i32;
    fn xicor_set_time(value: i64) -> i32;
    fn xicor_get_time() -> i64;

    fn m41t81_probe() -> i32;
    fn m41t81_set_time(value: i64) -> i32;
    fn m41t81_get_time() -> i64;

    fn mips_set_be_handler(handler: unsafe extern "C" fn(*mut pt_regs, i32) -> i32);
    fn vgacon_register_screen(info: *mut screen_info);
    fn writeb(value: u8, address: *mut core::ffi::c_void);
    fn __read_64bit_c0_register(reg: i32, sel: i32) -> u64;
    fn printk(format: *const core::ffi::c_char, ...) -> i32;
    fn mktime64(year: i32, month: i32, day: i32, hour: i32, minute: i32, second: i32) -> i64;
}

#[repr(C)]
pub struct screen_info {
    pub orig_video_page: u16,
    pub orig_video_mode: u8,
    pub orig_video_cols: u8,
    pub flags: u16,
    pub orig_video_ega_bx: u16,
    pub orig_video_lines: u8,
    pub orig_video_isVGA: u8,
    pub orig_video_points: u16,
}

pub const MIPS_BE_FIXUP: i32 = 0;
pub const MIPS_BE_FATAL: i32 = 1;

pub unsafe extern "C" fn get_system_type() -> *const core::ffi::c_char {
    concat!("SiByte ", SIBYTE_BOARD_NAME).as_ptr() as *const core::ffi::c_char
}

pub unsafe extern "C" fn swarm_be_handler(regs: *mut pt_regs, is_fixup: i32) -> i32 {
    if is_fixup == 0 && ((*regs).cp0_cause & 4) != 0 {
        /* Data bus error - print PA */
        printk(
            b"DBE physical address: %010Lx\n\0".as_ptr() as *const core::ffi::c_char,
            __read_64bit_c0_register(26, 1),
        );
    }
    if is_fixup != 0 { MIPS_BE_FIXUP } else { MIPS_BE_FATAL }
}

#[repr(i32)]
pub enum swarm_rtc_type {
    RTC_NONE,
    RTC_XICOR,
    RTC_M41T81,
}

pub static mut swarm_rtc_type: swarm_rtc_type = swarm_rtc_type::RTC_NONE;

pub unsafe extern "C" fn read_persistent_clock64(ts: *mut timespec64) {
    let sec: i64;
    match swarm_rtc_type {
        swarm_rtc_type::RTC_XICOR => sec = xicor_get_time(),
        swarm_rtc_type::RTC_M41T81 => sec = m41t81_get_time(),
        swarm_rtc_type::RTC_NONE => sec = mktime64(2000, 1, 1, 0, 0, 0),
    }
    (*ts).tv_sec = sec;
    (*ts).tv_nsec = 0;
}

pub unsafe extern "C" fn update_persistent_clock64(now: timespec64) -> i32 {
    let sec = now.tv_sec;
    match swarm_rtc_type {
        swarm_rtc_type::RTC_XICOR => xicor_set_time(sec),
        swarm_rtc_type::RTC_M41T81 => m41t81_set_time(sec),
        swarm_rtc_type::RTC_NONE => -1,
    }
}

#[cfg(CONFIG_VGA_CONSOLE)]
static mut vgacon_screen_info: screen_info = screen_info {
    orig_video_page: 52,
    orig_video_mode: 3,
    orig_video_cols: 80,
    flags: 12,
    orig_video_ega_bx: 3,
    orig_video_lines: 25,
    orig_video_isVGA: 0x22,
    orig_video_points: 16,
};

pub unsafe extern "C" fn plat_mem_setup() {
    #[cfg(CONFIG_SIBYTE_BCM1x80)]
    bcm1480_setup();
    #[cfg(any(CONFIG_SIBYTE_SB1250, CONFIG_SIBYTE_BCM112X))]
    sb1250_setup();

    mips_set_be_handler(swarm_be_handler);

    if xicor_probe() != 0 {
        swarm_rtc_type = swarm_rtc_type::RTC_XICOR;
    }
    if m41t81_probe() != 0 {
        swarm_rtc_type = swarm_rtc_type::RTC_M41T81;
    }

    #[cfg(CONFIG_VGA_CONSOLE)]
    {
        vgacon_register_screen(&mut vgacon_screen_info);
        /* XXXKW for CFE, get lines/cols from environment */
    }
}

#[cfg(LEDS_PHYS)]
pub unsafe extern "C" fn setleds(str_: *const u8) {
    for i in 0..4 {
        let reg = (IOADDR(LEDS_PHYS) as usize + 0x20 + ((3 - i) << 3)) as *mut core::ffi::c_void;
        if *str_.add(i as usize) == 0 {
            writeb(b' ', reg);
        } else {
            writeb(*str_.add(i as usize), reg);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
