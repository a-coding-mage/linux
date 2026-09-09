// SPDX-License-Identifier: GPL-2.0
/*
 * dma_timer.c -- Freescale ColdFire DMA Timer.
 *
 * Copyright (C) 2007, Benedikt Spranger <b.spranger@linutronix.de>
 * Copyright (C) 2008. Sebastian Siewior, Linutronix
 *
 */

// Linux and ColdFire header dependencies are supplied by other translation units.

const DMA_TIMER_0: usize = 0x00;
const DMA_TIMER_1: usize = 0x40;
const DMA_TIMER_2: usize = 0x80;
const DMA_TIMER_3: usize = 0xc0;

const DTMR0: usize = MCF_IPSBAR + DMA_TIMER_0 + 0x400;
const DTXMR0: usize = MCF_IPSBAR + DMA_TIMER_0 + 0x402;
const DTER0: usize = MCF_IPSBAR + DMA_TIMER_0 + 0x403;
const DTRR0: usize = MCF_IPSBAR + DMA_TIMER_0 + 0x404;
const DTCR0: usize = MCF_IPSBAR + DMA_TIMER_0 + 0x408;
const DTCN0: usize = MCF_IPSBAR + DMA_TIMER_0 + 0x40c;

const DMA_FREQ: u32 = (MCF_CLK / 2) / 16;

/* DTMR */
const DMA_DTMR_RESTART: u16 = 1 << 3;
const DMA_DTMR_CLK_DIV_1: u16 = 1 << 1;
const DMA_DTMR_CLK_DIV_16: u16 = 2 << 1;
const DMA_DTMR_ENABLE: u16 = 1 << 0;

extern "C" {
    static MCF_IPSBAR: usize;
    static MCF_CLK: u32;

    fn mcf_read32(address: usize) -> u32;
    fn mcf_write8(value: u8, address: usize);
    fn mcf_write16(value: u16, address: usize);
    fn mcf_write32(value: u32, address: usize);
    fn clocksource_register_hz(cs: *mut clocksource, hz: u32) -> i32;
}

#[repr(C)]
struct clocksource {
    name: *const u8,
    rating: i32,
    read: Option<unsafe extern "C" fn(cs: *mut clocksource) -> u64>,
    mask: u64,
    flags: u32,
}

const CLOCKSOURCE_MASK_32: u64 = 0xffff_ffff;
const CLOCK_SOURCE_IS_CONTINUOUS: u32 = 1 << 1;

unsafe extern "C" fn cf_dt_get_cycles(_cs: *mut clocksource) -> u64 {
    mcf_read32(DTCN0) as u64
}

static mut CLOCKSOURCE_CF_DT: clocksource = clocksource {
    name: b"coldfire_dma_timer\0".as_ptr(),
    rating: 200,
    read: Some(cf_dt_get_cycles),
    mask: CLOCKSOURCE_MASK_32,
    flags: CLOCK_SOURCE_IS_CONTINUOUS,
};

unsafe extern "C" fn init_cf_dt_clocksource() -> i32 {
    /*
     * We setup DMA timer 0 in free run mode. This incrementing counter is
     * used as a highly precious clock source. With MCF_CLOCK = 150 MHz we
     * get a ~213 ns resolution and the 32bit register will overflow almost
     * every 15 minutes.
     */
    mcf_write8(0x00, DTXMR0);
    mcf_write8(0x00, DTER0);
    mcf_write32(0x00000000, DTRR0);
    mcf_write16(DMA_DTMR_CLK_DIV_16 | DMA_DTMR_ENABLE, DTMR0);
    clocksource_register_hz(&raw mut CLOCKSOURCE_CF_DT, DMA_FREQ)
}

// arch_initcall(init_cf_dt_clocksource);

const CYC2NS_SCALE_FACTOR: u32 = 10; /* 2^10, carefully chosen */
const CYC2NS_SCALE: u64 = ((1_000_000u64 << CYC2NS_SCALE_FACTOR) /
    (DMA_FREQ as u64 / 1000));

unsafe fn cycles2ns(cycl: u64) -> u64 {
    (cycl.wrapping_mul(CYC2NS_SCALE)) >> CYC2NS_SCALE_FACTOR
}

#[no_mangle]
pub unsafe extern "C" fn sched_clock() -> u64 {
    let cycl: u64 = mcf_read32(DTCN0) as u64;

    cycles2ns(cycl)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
