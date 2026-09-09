// SPDX-License-Identifier: GPL-2.0-only
/* Linux performance counter support for MIPS. Rust translation of the
 * corresponding kernel implementation; external kernel symbols are supplied
 * by the surrounding kernel translation. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

const MIPS_MAX_HWEVENTS: usize = 4;
const MIPS_TCS_PER_COUNTER: usize = 2;
const MIPS_CPUID_TO_COUNTER_MASK: usize = MIPS_TCS_PER_COUNTER - 1;
const CNTR_EVEN: u32 = 0x5555_5555;
const CNTR_ODD: u32 = 0xaaaa_aaaa;
const CNTR_ALL: u32 = 0xffff_ffff;

#[repr(C)]
pub struct cpu_hw_events {
    pub events: [*mut perf_event; MIPS_MAX_HWEVENTS],
    pub used_mask: [usize; 1],
    pub saved_ctrl: [u32; MIPS_MAX_HWEVENTS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mips_perf_event {
    pub event_id: u32,
    pub cntr_mask: u32,
    pub range: u32,
}

pub const T: u32 = 0;
pub const V: u32 = 1;
pub const P: u32 = 2;

#[repr(C)]
pub struct mips_pmu {
    pub max_period: u64,
    pub valid_count: u64,
    pub overflow: u64,
    pub name: *const u8,
    pub irq: i32,
    pub read_counter: Option<unsafe extern "C" fn(u32) -> u64>,
    pub write_counter: Option<unsafe extern "C" fn(u32, u64)>,
    pub map_raw_event: Option<unsafe extern "C" fn(u64) -> *const mips_perf_event>,
    pub num_counters: u32,
}

extern "C" {
    static mut counter_bits: i32;
    static mut mipspmu: mips_pmu;
    fn read_c0_perfcntr0() -> u32; fn read_c0_perfcntr1() -> u32;
    fn read_c0_perfcntr2() -> u32; fn read_c0_perfcntr3() -> u32;
    fn read_c0_perfcntr0_64() -> u64; fn read_c0_perfcntr1_64() -> u64;
    fn read_c0_perfcntr2_64() -> u64; fn read_c0_perfcntr3_64() -> u64;
    fn write_c0_perfcntr0(v: u64); fn write_c0_perfcntr1(v: u64);
    fn write_c0_perfcntr2(v: u64); fn write_c0_perfcntr3(v: u64);
    fn write_c0_perfcntr0_64(v: u64); fn write_c0_perfcntr1_64(v: u64);
    fn write_c0_perfcntr2_64(v: u64); fn write_c0_perfcntr3_64(v: u64);
    fn read_c0_perfctrl0() -> u32; fn read_c0_perfctrl1() -> u32;
    fn read_c0_perfctrl2() -> u32; fn read_c0_perfctrl3() -> u32;
    fn write_c0_perfctrl0(v: u32); fn write_c0_perfctrl1(v: u32);
    fn write_c0_perfctrl2(v: u32); fn write_c0_perfctrl3(v: u32);
}

#[repr(C)] pub struct perf_event { pub hw: hw_perf_event }
#[repr(C)] pub struct hw_perf_event {
    pub event_base: u64, pub config_base: u64, pub idx: i32,
    pub sample_period: u64, pub last_period: u64, pub period_left: u64,
    pub prev_count: u64, pub state: u64,
}

#[inline] fn cntr_bit_mask(n: i32) -> u64 {
    if n == 64 { u64::MAX } else { (1u64 << n) - 1 }
}

unsafe fn swizzle(mut idx: u32) -> u32 {
    // vpe_id() is supplied by the MIPS CPU support.
    if vpe_id() == 1 { idx = (idx + 2) & 3; }
    idx
}

unsafe fn mipsxx_pmu_read_counter(mut idx: u32) -> u64 {
    idx = swizzle(idx);
    match idx { 0 => read_c0_perfcntr0() as u64, 1 => read_c0_perfcntr1() as u64,
        2 => read_c0_perfcntr2() as u64, 3 => read_c0_perfcntr3() as u64, _ => 0 }
}
unsafe fn mipsxx_pmu_read_counter_64(mut idx: u32) -> u64 {
    let mask = cntr_bit_mask(counter_bits); idx = swizzle(idx);
    (match idx { 0 => read_c0_perfcntr0_64(), 1 => read_c0_perfcntr1_64(),
        2 => read_c0_perfcntr2_64(), 3 => read_c0_perfcntr3_64(), _ => 0 }) & mask
}
unsafe fn mipsxx_pmu_write_counter(mut idx: u32, val: u64) {
    idx = swizzle(idx); match idx { 0 => write_c0_perfcntr0(val), 1 => write_c0_perfcntr1(val),
        2 => write_c0_perfcntr2(val), 3 => write_c0_perfcntr3(val), _ => {} }
}
unsafe fn mipsxx_pmu_write_counter_64(mut idx: u32, mut val: u64) {
    val &= cntr_bit_mask(counter_bits); idx = swizzle(idx);
    match idx { 0 => write_c0_perfcntr0_64(val), 1 => write_c0_perfcntr1_64(val),
        2 => write_c0_perfcntr2_64(val), 3 => write_c0_perfcntr3_64(val), _ => {} }
}

extern "C" { fn vpe_id() -> u32; }

/* The remaining kernel callbacks and architecture-specific event tables retain
 * the C implementation's externally supplied perf/kernel interfaces. */
pub unsafe fn mipsxx_pmu_read_control(mut idx: u32) -> u32 {
    idx = swizzle(idx); match idx { 0 => read_c0_perfctrl0(), 1 => read_c0_perfctrl1(),
        2 => read_c0_perfctrl2(), 3 => read_c0_perfctrl3(), _ => 0 }
}
pub unsafe fn mipsxx_pmu_write_control(mut idx: u32, val: u32) {
    idx = swizzle(idx); match idx { 0 => write_c0_perfctrl0(val), 1 => write_c0_perfctrl1(val),
        2 => write_c0_perfctrl2(val), 3 => write_c0_perfctrl3(val), _ => {} }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
