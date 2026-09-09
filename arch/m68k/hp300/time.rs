// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/arch/m68k/hp300/time.c
 *
 *  Copyright (C) 1998 Philip Blundell <philb@gnu.org>
 *
 *  This file contains the HP300-specific time handling code.
 */

// C dependencies supplied by the surrounding kernel build are intentionally
// referenced here as external Rust items.

use core::ffi::c_void;

type U64 = u64;
type U32 = u32;
type U8 = u8;
type Irqreturn = i32;

#[repr(C)]
pub struct Clocksource {
    pub name: *const u8,
    pub rating: i32,
    pub read: Option<unsafe extern "C" fn(*mut Clocksource) -> U64>,
    pub mask: U32,
    pub flags: U32,
}

unsafe extern "C" {
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn in_8(addr: usize) -> U8;
    fn out_8(addr: usize, value: U8);
    fn legacy_timer_tick(ticks: i32);
    fn timer_heartbeat();
    fn blinken_leds(a: i32, b: i32);
    fn request_irq(
        irq: i32,
        handler: unsafe extern "C" fn(i32, *mut c_void) -> Irqreturn,
        flags: U32,
        name: *const u8,
        dev_id: *mut c_void,
    ) -> i32;
    fn pr_err(message: *const u8);
    fn clocksource_register_hz(cs: *mut Clocksource, hz: U32) -> i32;
}

const CLOCKSOURCE_MASK_32: U32 = 0xffff_ffff;
const CLOCK_SOURCE_IS_CONTINUOUS: U32 = 1 << 0;
const IRQ_HANDLED: Irqreturn = 1;
const IRQ_AUTO_6: i32 = 6;
const IRQF_TIMER: U32 = 0x0000_0001;
const HZ: U32 = 100;

static mut hp300_clk: Clocksource = Clocksource {
    name: b"timer\0".as_ptr(),
    rating: 250,
    read: Some(hp300_read_clk),
    mask: CLOCKSOURCE_MASK_32,
    flags: CLOCK_SOURCE_IS_CONTINUOUS,
};

static mut clk_total: U32 = 0;
static mut clk_offset: U32 = 0;

// Clock hardware definitions
const CLOCKBASE: usize = 0xf05f8000;
const CLKCR1: usize = 0x1;
const CLKCR2: usize = 0x3;
const CLKCR3: usize = CLKCR1;
const CLKSR: usize = CLKCR2;
const CLKMSB1: usize = 0x5;
const CLKLSB1: usize = 0x7;
const CLKMSB2: usize = 0x9;
const CLKMSB3: usize = 0xd;

const CLKSR_INT1: U8 = 1 << 0;

// This is for machines which generate the exact clock.
const HP300_TIMER_CLOCK_FREQ: U32 = 250_000;
const HP300_TIMER_CYCLES: U32 = HP300_TIMER_CLOCK_FREQ / HZ;
const INTVAL: U32 = HP300_TIMER_CYCLES - 1;

unsafe extern "C" fn hp300_tick(_irq: i32, _dev_id: *mut c_void) -> Irqreturn {
    let mut flags: usize = 0;
    let mut tmp: usize;

    local_irq_save(&mut flags);
    in_8(CLOCKBASE + CLKSR);
    core::arch::asm!("movpw 5({base}), {tmp}", base = in(reg) CLOCKBASE, tmp = out(reg) tmp);
    clk_total = clk_total.wrapping_add(INTVAL);
    clk_offset = 0;
    legacy_timer_tick(1);
    timer_heartbeat();
    local_irq_restore(flags);

    // Turn off the network and SCSI leds
    blinken_leds(0, 0xe0);
    IRQ_HANDLED
}

unsafe extern "C" fn hp300_read_clk(_cs: *mut Clocksource) -> U64 {
    let mut flags: usize = 0;
    let mut lsb: U8;
    let mut msb: U8;
    let mut msb_new: U8;
    let ticks: U32;

    local_irq_save(&mut flags);
    // Read current timer 1 value
    msb = in_8(CLOCKBASE + CLKMSB1);
    loop {
        if (in_8(CLOCKBASE + CLKSR) & CLKSR_INT1) != 0 && msb > 0 {
            clk_offset = INTVAL;
        }
        lsb = in_8(CLOCKBASE + CLKLSB1);
        msb_new = in_8(CLOCKBASE + CLKMSB1);
        if msb_new == msb {
            break;
        }
        msb = msb_new;
    }

    ticks = INTVAL
        .wrapping_sub(((msb as U32) << 8) | lsb as U32)
        .wrapping_add(clk_offset)
        .wrapping_add(clk_total);
    local_irq_restore(flags);

    ticks as U64
}

pub unsafe extern "C" fn hp300_sched_init() {
    out_8(CLOCKBASE + CLKCR2, 0x1); // select CR1
    out_8(CLOCKBASE + CLKCR1, 0x1); // reset

    core::arch::asm!("movpw {intval}, 5({base})", intval = in(reg) INTVAL, base = in(reg) CLOCKBASE);

    if request_irq(IRQ_AUTO_6, hp300_tick, IRQF_TIMER, b"timer tick\0".as_ptr(), core::ptr::null_mut()) != 0 {
        pr_err(b"Couldn't register timer interrupt\n\0".as_ptr());
    }

    out_8(CLOCKBASE + CLKCR2, 0x1); // select CR1
    out_8(CLOCKBASE + CLKCR1, 0x40); // enable irq

    clocksource_register_hz(&mut hp300_clk, HP300_TIMER_CLOCK_FREQ);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
