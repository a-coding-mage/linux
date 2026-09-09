/* SPDX-License-Identifier: GPL-2.0 */
/*
 * S390 version
 * Copyright IBM Corp. 1999
 * Derived from include/asm-i386/timex.h
 */

// Dependencies supplied by the surrounding kernel translation.

pub const TOD_UNIX_EPOCH: u64 = 0x7d91048bca000000;

extern "C" {
    pub static mut clock_comparator_max: u64;
    pub static mut ptff_function_mask: [u8; 16];
    pub fn clock_comparator_work();
    pub fn time_early_init();
    pub fn get_phys_clock(clock: *mut c_ulong) -> c_int;
    pub fn init_cpu_timer();
    pub static mut tod_clock_base: tod_clock;
}

pub type c_ulong = usize;
pub type c_int = i32;
pub type u16 = u16;
pub type u64_ = u64;
pub type u128_ = u128;
pub type cycles_t = c_ulong;

#[repr(C)]
pub union tod_clock {
    pub tod: c_ulong,
    pub bytes: [u8; 16],
}

/* Inline functions for clock register access. */
#[inline]
pub unsafe fn set_tod_clock(time: u64) -> c_int {
    let mut cc: c_int = 0;
    core::arch::asm!("sck {time}", time = in(reg) time, lateout("0") cc);
    cc
}

#[inline]
pub unsafe fn store_tod_clock_ext_cc(clk: *mut tod_clock) -> c_int {
    let mut cc: c_int = 0;
    core::arch::asm!("stcke {clk}", clk = in(reg) clk, lateout("0") cc);
    cc
}

#[inline(always)]
pub unsafe fn store_tod_clock_ext(tod: *mut tod_clock) {
    core::arch::asm!("stcke {tod}", tod = in(reg) tod, options(nostack));
}

#[inline]
pub unsafe fn set_clock_comparator(time: u64) {
    core::arch::asm!("sckc {time}", time = in(reg) time, options(nostack));
}

#[inline]
pub unsafe fn set_tod_programmable_field(val: u16) {
    core::arch::asm!("lgr 0,{val}\n sckpf", val = in(reg) val as c_ulong);
}

pub const PTFF_QAF: u32 = 0x00;
pub const PTFF_QTO: u32 = 0x01;
pub const PTFF_QSI: u32 = 0x02;
pub const PTFF_QPT: u32 = 0x03;
pub const PTFF_QUI: u32 = 0x04;
pub const PTFF_ATO: u32 = 0x40;
pub const PTFF_STO: u32 = 0x41;
pub const PTFF_SFS: u32 = 0x42;
pub const PTFF_SGS: u32 = 0x43;

#[repr(C, packed)]
pub struct ptff_qto {
    pub physical_clock: c_ulong,
    pub tod_offset: c_ulong,
    pub logical_tod_offset: c_ulong,
    pub tod_epoch_difference: c_ulong,
}

#[inline]
pub unsafe fn ptff_query(nr: u32) -> bool {
    let ptr = ptff_function_mask.as_ptr().add((nr >> 3) as usize);
    (*ptr & (0x80 >> (nr & 7))) != 0
}

#[repr(C, packed)]
pub struct ptff_qui {
    /* C bitfields tm:2, ts:2, and an unnamed 28-bit field. */
    pub tm_ts_reserved: u32,
    pub pad_0x04: u32,
    pub leap_event: c_ulong,
    pub old_leap: i16,
    pub new_leap: i16,
    pub pad_0x14: u32,
    pub prt: [c_ulong; 5],
    pub cst: [c_ulong; 3],
    pub skew: u32,
    pub pad_0x5c: [u32; 41],
}

/* ptff - perform timing facility function; returns the condition code. */
#[inline]
pub unsafe fn ptff<T>(ptff_block: *mut T, len: usize, func: u32) -> c_int {
    let _ = len;
    let mut rc: c_int = 0;
    core::arch::asm!(
        "lgr 0,{reg0}\n lgr 1,{reg1}\n ptff",
        reg0 = in(reg) func as c_ulong,
        reg1 = in(reg) ptff_block as c_ulong,
        lateout("0") rc,
        inout("1") _ => _,
    );
    rc
}

#[inline]
pub unsafe fn local_tick_disable() -> c_ulong {
    let old = (*get_lowcore()).clock_comparator;
    (*get_lowcore()).clock_comparator = clock_comparator_max;
    set_clock_comparator((*get_lowcore()).clock_comparator);
    old
}

#[inline]
pub unsafe fn local_tick_enable(comp: c_ulong) {
    (*get_lowcore()).clock_comparator = comp;
    set_clock_comparator((*get_lowcore()).clock_comparator);
}

extern "C" {
    fn get_lowcore() -> *mut lowcore;
    fn preempt_disable_notrace();
    fn preempt_enable_notrace();
}

#[repr(C)]
pub struct lowcore {
    pub clock_comparator: c_ulong,
}

#[inline(always)]
pub unsafe fn get_tod_clock() -> c_ulong {
    let mut clk = tod_clock { tod: 0 };
    store_tod_clock_ext(&mut clk);
    clk.tod
}

#[inline]
pub unsafe fn get_tod_clock_fast() -> c_ulong {
    let mut clk: c_ulong;
    core::arch::asm!("stckf {clk}", clk = lateout(reg) clk);
    clk
}

#[inline(always)]
pub unsafe fn __get_tod_clock_monotonic() -> c_ulong {
    get_tod_clock().wrapping_sub(tod_clock_base.tod)
}

#[inline]
pub unsafe fn get_tod_clock_monotonic() -> c_ulong {
    preempt_disable_notrace();
    let tod = __get_tod_clock_monotonic();
    preempt_enable_notrace();
    tod
}

#[inline]
pub unsafe fn get_cycles() -> cycles_t {
    get_tod_clock_monotonic() >> 2
}

#[inline(always)]
pub const fn tod_to_ns(todval: c_ulong) -> c_ulong {
    ((todval >> 9) * 125) + (((todval & 0x1ff) * 125) >> 9)
}

#[inline(always)]
pub const fn eitod_to_ns(todval: u128) -> u128 {
    (todval * 125) >> 9
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
