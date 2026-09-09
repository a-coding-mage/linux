// SPDX-License-Identifier: GPL-2.0
/*
 *  Floating proportions with flexible aging period
 *
 *   Copyright (C) 2011, SUSE, Jan Kara <jack@suse.cz>
 *
 * The goal of this code is: Given different types of event, measure proportion
 * of each type of event over time. The proportions are measured with
 * exponentially decaying history to give smooth transitions.
 */

use core::ffi::c_int;

// Types and operations supplied by linux/flex_proportions.h and other kernel headers.
pub type GfpT = usize;
pub type S64 = i64;
pub type U64 = u64;

#[repr(C)]
pub struct PercpuCounter { _private: [u8; 0] }
#[repr(C)]
pub struct Seqcount { _private: [u8; 0] }
#[repr(C)]
pub struct RawSpinlock { _private: [u8; 0] }

#[repr(C)]
pub struct FpropGlobal {
    pub events: PercpuCounter,
    pub period: u32,
    pub sequence: Seqcount,
}

#[repr(C)]
pub struct FpropLocalPercpu {
    pub events: PercpuCounter,
    pub period: u32,
    pub lock: RawSpinlock,
}

extern "C" {
    fn percpu_counter_init(c: *mut PercpuCounter, amount: i64, gfp: GfpT) -> c_int;
    fn percpu_counter_destroy(c: *mut PercpuCounter);
    fn percpu_counter_sum(c: *mut PercpuCounter) -> i64;
    fn percpu_counter_read(c: *mut PercpuCounter) -> i64;
    fn percpu_counter_read_positive(c: *mut PercpuCounter) -> i64;
    fn percpu_counter_add(c: *mut PercpuCounter, amount: i64);
    fn percpu_counter_add_batch(c: *mut PercpuCounter, amount: i64, batch: i64);
    fn percpu_counter_set(c: *mut PercpuCounter, amount: i64);
    fn seqcount_init(s: *mut Seqcount);
    fn write_seqcount_begin(s: *mut Seqcount);
    fn write_seqcount_end(s: *mut Seqcount);
    fn read_seqcount_begin(s: *mut Seqcount) -> u32;
    fn read_seqcount_retry(s: *mut Seqcount, seq: u32) -> bool;
    fn raw_spin_lock_irqsave(lock: *mut RawSpinlock, flags: *mut usize);
    fn raw_spin_unlock_irqrestore(lock: *mut RawSpinlock, flags: usize);
    fn raw_spin_lock_init(lock: *mut RawSpinlock);
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
}

const FPROP_FRAC_BASE: i64 = 1 << FPROP_FRAC_SHIFT;
const FPROP_FRAC_SHIFT: u32 = 8;
const BITS_PER_LONG: u32 = usize::BITS;
const NR_CPU_IDS: i64 = 1;
const PROP_BATCH: i64 = 8 * (1 + 0);

pub unsafe fn fprop_global_init(p: *mut FpropGlobal, gfp: GfpT) -> c_int {
    (*p).period = 0;
    let err = percpu_counter_init(&mut (*p).events, 1, gfp);
    if err != 0 { return err; }
    seqcount_init(&mut (*p).sequence);
    0
}

pub unsafe fn fprop_global_destroy(p: *mut FpropGlobal) {
    percpu_counter_destroy(&mut (*p).events);
}

pub unsafe fn fprop_new_period(p: *mut FpropGlobal, periods: u32) -> bool {
    let mut events = percpu_counter_sum(&mut (*p).events);
    let mut flags = 0usize;
    if events <= 1 { return false; }
    local_irq_save(&mut flags);
    write_seqcount_begin(&mut (*p).sequence);
    if periods < 64 { events -= events >> periods; }
    percpu_counter_add(&mut (*p).events, -events);
    (*p).period = (*p).period.wrapping_add(periods);
    write_seqcount_end(&mut (*p).sequence);
    local_irq_restore(flags);
    true
}

unsafe fn fprop_reflect_period_percpu(p: *mut FpropGlobal, pl: *mut FpropLocalPercpu) {
    let period = (*p).period;
    let mut flags = 0usize;
    if (*pl).period == period { return; }
    raw_spin_lock_irqsave(&mut (*pl).lock, &mut flags);
    if (*pl).period >= period {
        raw_spin_unlock_irqrestore(&mut (*pl).lock, flags);
        return;
    }
    if period - (*pl).period < BITS_PER_LONG {
        let mut val = percpu_counter_read(&mut (*pl).events);
        if val < NR_CPU_IDS * PROP_BATCH { val = percpu_counter_sum(&mut (*pl).events); }
        percpu_counter_add_batch(&mut (*pl).events, -val + (val >> (period - (*pl).period)), PROP_BATCH);
    } else { percpu_counter_set(&mut (*pl).events, 0); }
    (*pl).period = period;
    raw_spin_unlock_irqrestore(&mut (*pl).lock, flags);
}

pub unsafe fn fprop_local_init_percpu(pl: *mut FpropLocalPercpu, gfp: GfpT) -> c_int {
    let err = percpu_counter_init(&mut (*pl).events, 0, gfp);
    if err != 0 { return err; }
    (*pl).period = 0;
    raw_spin_lock_init(&mut (*pl).lock);
    0
}

pub unsafe fn fprop_local_destroy_percpu(pl: *mut FpropLocalPercpu) {
    percpu_counter_destroy(&mut (*pl).events);
}

pub unsafe fn __fprop_add_percpu(p: *mut FpropGlobal, pl: *mut FpropLocalPercpu, nr: i64) {
    fprop_reflect_period_percpu(p, pl);
    percpu_counter_add_batch(&mut (*pl).events, nr, PROP_BATCH);
    percpu_counter_add(&mut (*p).events, nr);
}

pub unsafe fn fprop_fraction_percpu(p: *mut FpropGlobal, pl: *mut FpropLocalPercpu, numerator: *mut usize, denominator: *mut usize) {
    let (num, den);
    loop {
        let seq = read_seqcount_begin(&mut (*p).sequence);
        fprop_reflect_period_percpu(p, pl);
        num = percpu_counter_read_positive(&mut (*pl).events);
        den = percpu_counter_read_positive(&mut (*p).events);
        if !read_seqcount_retry(&mut (*p).sequence, seq) { break; }
    }
    let mut den = den;
    if den <= num { den = if num != 0 { num } else { 1 }; }
    *denominator = den as usize;
    *numerator = num as usize;
}

pub unsafe fn __fprop_add_percpu_max(p: *mut FpropGlobal, pl: *mut FpropLocalPercpu, max_frac: i64, mut nr: i64) {
    if max_frac < FPROP_FRAC_BASE {
        let mut numerator = 0usize;
        let mut denominator = 0usize;
        fprop_fraction_percpu(p, pl, &mut numerator, &mut denominator);
        let tmp = (denominator as U64) * max_frac as U64 - ((numerator as U64) << FPROP_FRAC_SHIFT);
        if (tmp as i64) < 0 { return; }
        if tmp < (nr as U64) * (FPROP_FRAC_BASE - max_frac) as U64 {
            nr = ((tmp + (FPROP_FRAC_BASE - max_frac - 1) as U64) / (FPROP_FRAC_BASE - max_frac) as U64) as i64;
        }
    }
    __fprop_add_percpu(p, pl, nr);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
