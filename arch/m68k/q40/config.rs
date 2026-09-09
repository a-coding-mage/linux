// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  arch/m68k/q40/config.c
 *
 *  Copyright (C) 1999 Richard Zidlicky
 *
 * originally based on:
 *
 *  linux/bvme/config.c
 */

// C header dependencies are supplied by the surrounding kernel translation.

use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    static mut ql_ticks: c_int;
    static mut q40_mem_cptr: *mut c_char;
    static mut halted: c_int;

    fn q40_sched_init();
    fn q40_init_IRQ();
    fn q40_mksound();
    fn q40_hwclk(op: c_int, t: *mut rtc_time) -> c_int;
    fn q40_get_rtc_pll(pll: *mut rtc_pll_info) -> c_int;
    fn q40_set_rtc_pll(pll: *mut rtc_pll_info) -> c_int;
    fn q40_led_on();
    fn q40_led_off();
    fn outb(value: u8, port: c_uint);
    fn master_outb(value: u8, port: c_uint);
    fn register_console(co: *mut console);
    fn platform_device_register_simple(name: *const c_char, id: c_int,
                                       resources: *const resource, num_resources: usize) -> c_int;
    fn printk_info(format: *const c_char, ...);
    fn bin2bcd(value: c_int) -> c_int;
    fn bcd2bin(value: c_int) -> c_int;
}

#[repr(C)]
pub struct rtc_time {
    pub tm_sec: c_int, pub tm_min: c_int, pub tm_hour: c_int,
    pub tm_mday: c_int, pub tm_mon: c_int, pub tm_year: c_int,
    pub tm_wday: c_int,
}

#[repr(C)] pub struct rtc_pll_info { pub pll_ctrl: c_int, pub pll_value: c_int,
    pub pll_max: c_int, pub pll_min: c_int, pub pll_posmult: c_int,
    pub pll_negmult: c_int, pub pll_clock: c_int }
#[repr(C)] pub struct console { pub name: *const c_char, pub write: Option<unsafe extern "C" fn(*mut console, *const c_char, c_uint), pub flags: c_uint, pub index: c_int }
#[repr(C)] pub struct bi_record { _private: [u8; 0] }
#[repr(C)] pub struct resource { _private: [u8; 0] }

extern "C" {
    static mut mach_sched_init: Option<unsafe extern "C" fn()>;
    static mut mach_init_IRQ: Option<unsafe extern "C" fn()>;
    static mut mach_hwclk: Option<unsafe extern "C" fn(c_int, *mut rtc_time) -> c_int>;
    static mut mach_get_rtc_pll: Option<unsafe extern "C" fn(*mut rtc_pll_info) -> c_int>;
    static mut mach_set_rtc_pll: Option<unsafe extern "C" fn(*mut rtc_pll_info) -> c_int>;
    static mut mach_reset: Option<unsafe extern "C" fn()>;
    static mut mach_get_model: Option<unsafe extern "C" fn(*mut c_char)>;
    static mut mach_halt: Option<unsafe extern "C" fn()>;
    static q40_isa_io_base: c_uint;
}

static mut cpleft: c_int = 0;

unsafe extern "C" fn q40_mem_console_write(_co: *mut console, s: *const c_char, mut count: c_uint) {
    let mut p = s;
    if count < cpleft as c_uint {
        while count > 0 {
            *q40_mem_cptr = *p;
            p = p.add(1);
            q40_mem_cptr = q40_mem_cptr.add(4);
            count -= 1;
            cpleft -= 1;
        }
    }
}

static mut q40_console_driver: console = console { name: b"debug\0".as_ptr() as *const c_char,
    write: Some(q40_mem_console_write), flags: 1, index: -1 };

unsafe extern "C" fn q40_debug_setup(arg: *mut c_char) -> c_int {
    // useful for early debugging stages - writes kernel messages into SRAM
    if MACH_IS_Q40 && core::slice::from_raw_parts(arg as *const u8, 3) == *b"mem" {
        cpleft = 2000 - ((q40_mem_cptr as isize - 0xff020000isize) / 4) as c_int;
        register_console(&raw mut q40_console_driver);
    }
    0
}

unsafe extern "C" fn q40_heartbeat(on: c_int) {
    if halted != 0 { return; }
    if on != 0 { q40_led_on(); } else { q40_led_off(); }
}

unsafe extern "C" fn q40_reset() -> ! {
    halted = 1;
    printk_info(b"*******************************************\nCalled q40_reset : press the RESET button!!\n*******************************************\n\0".as_ptr() as *const c_char);
    q40_led_on();
    loop {}
}

unsafe extern "C" fn q40_halt() -> ! {
    halted = 1;
    printk_info(b"*******************\n  Called q40_halt\n*******************\n\0".as_ptr() as *const c_char);
    q40_led_on();
    loop {}
}

unsafe extern "C" fn q40_get_model(model: *mut c_char) {
    *model = b'Q' as c_char; *model.add(1) = b'4' as c_char; *model.add(2) = b'0' as c_char; *model.add(3) = 0;
}

static serports: [c_uint; 5] = [0x3f8, 0x2f8, 0x3e8, 0x2e8, 0];

unsafe extern "C" fn q40_disable_irqs() {
    let mut j = 0;
    while serports[j] != 0 { outb(0, serports[j] + UART_IER); j += 1; }
    master_outb(0, EXT_ENABLE_REG); master_outb(0, KEY_IRQ_ENABLE_REG);
}

pub unsafe extern "C" fn config_q40() {
    mach_sched_init = Some(q40_sched_init); mach_init_IRQ = Some(q40_init_IRQ);
    mach_hwclk = Some(q40_hwclk); mach_get_rtc_pll = Some(q40_get_rtc_pll); mach_set_rtc_pll = Some(q40_set_rtc_pll);
    mach_reset = Some(q40_reset); mach_get_model = Some(q40_get_model); mach_halt = Some(q40_halt);
    q40_disable_irqs();
}

pub unsafe extern "C" fn q40_parse_bootinfo(_rec: *const bi_record) -> c_int { 1 }

const Q40_RTC_PLL_MASK: c_int = (1 << 5) - 1;
const Q40_RTC_PLL_SIGN: c_int = 1 << 5;

// RTC register and machine constants/macros are supplied by the architecture dependencies.
pub unsafe extern "C" fn q40_get_rtc_pll(pll: *mut rtc_pll_info) -> c_int {
    let tmp = Q40_RTC_CTRL;
    (*pll).pll_ctrl = 0; (*pll).pll_value = tmp & Q40_RTC_PLL_MASK;
    if tmp & Q40_RTC_PLL_SIGN != 0 { (*pll).pll_value = -(*pll).pll_value; }
    (*pll).pll_max = 31; (*pll).pll_min = -31; (*pll).pll_posmult = 512; (*pll).pll_negmult = 256; (*pll).pll_clock = 125829120; 0
}

pub unsafe extern "C" fn q40_set_rtc_pll(pll: *mut rtc_pll_info) -> c_int {
    if (*pll).pll_ctrl == 0 {
        let tmp = ((*pll).pll_value & 31) | if (*pll).pll_value < 0 { 32 } else { 0 } | Q40_RTC_WRITE;
        Q40_RTC_CTRL |= Q40_RTC_WRITE; Q40_RTC_CTRL = tmp; Q40_RTC_CTRL &= !Q40_RTC_WRITE; 0
    } else { -22 }
}

// The resource initializers and platform initcall are kernel macros retained as external build dependencies.
const PCIDE_BASE1: c_uint = 0x1f0; const PCIDE_BASE2: c_uint = 0x170; const PCIDE_CTL: c_uint = 0x206;

// DEFINE_RES_MEM/DEFINE_RES_IO/DEFINE_RES_IRQ expand to kernel resource literals.
static q40_pata_rsrc_0: [resource; 5] = [resource { _private: [] }, resource { _private: [] }, resource { _private: [] }, resource { _private: [] }, resource { _private: [] }];
static q40_pata_rsrc_1: [resource; 5] = [resource { _private: [] }, resource { _private: [] }, resource { _private: [] }, resource { _private: [] }, resource { _private: [] }];

unsafe extern "C" fn q40_platform_init() -> c_int {
    if !MACH_IS_Q40 { return -19; }
    platform_device_register_simple(b"q40kbd\0".as_ptr() as *const c_char, -1, core::ptr::null(), 0);
    platform_device_register_simple(b"atari-falcon-ide\0".as_ptr() as *const c_char, 0, q40_pata_rsrc_0.as_ptr(), q40_pata_rsrc_0.len());
    platform_device_register_simple(b"atari-falcon-ide\0".as_ptr() as *const c_char, 1, q40_pata_rsrc_1.as_ptr(), q40_pata_rsrc_1.len());
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
