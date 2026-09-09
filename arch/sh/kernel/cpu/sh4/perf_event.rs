// SPDX-License-Identifier: GPL-2.0
/*
 * Performance events support for SH7750-style performance counters
 *
 *  Copyright (C) 2009  Paul Mundt
 */
// Dependencies supplied by the Linux kernel and SH architecture headers.

const PM_CR_BASE: usize = 0xff000084; /* 16-bit */
const PM_CTR_BASE: usize = 0xff100004; /* 32-bit */

#[inline]
const fn PMCR(n: usize) -> usize { PM_CR_BASE + n * 0x04 }
#[inline]
const fn PMCTRH(n: usize) -> usize { PM_CTR_BASE + n * 0x08 }
#[inline]
const fn PMCTRL(n: usize) -> usize { PM_CTR_BASE + 0x04 + n * 0x08 }

const PMCR_PMM_MASK: u16 = 0x003f;
const PMCR_CLKF: u16 = 0x0100;
const PMCR_PMCLR: u16 = 0x2000;
const PMCR_PMST: u16 = 0x4000;
const PMCR_PMEN: u16 = 0x8000;

extern "C" {
    static mut sh7750_pmu: sh_pmu;
}

/*
 * There are a number of events supported by each counter (33 in total).
 * Since we have 2 counters, each counter will take the event code as it
 * corresponds to the PMCR PMM setting. Each counter can be configured
 * independently.
 *
 * Event Code    Description
 * ----------    -----------
 * 0x01 Operand read access
 * 0x02 Operand write access
 * 0x03 UTLB miss
 * 0x04 Operand cache read miss
 * 0x05 Operand cache write miss
 * 0x06 Instruction fetch (w/ cache)
 * 0x07 Instruction TLB miss
 * 0x08 Instruction cache miss
 * 0x09 All operand accesses
 * 0x0a All instruction accesses
 * 0x0b OC RAM operand access
 * 0x0d On-chip I/O space access
 * 0x0e Operand access (r/w)
 * 0x0f Operand cache miss (r/w)
 * 0x10 Branch instruction
 * 0x11 Branch taken
 * 0x12 BSR/BSRF/JSR
 * 0x13 Instruction execution
 * 0x14 Instruction execution in parallel
 * 0x15 FPU Instruction execution
 * 0x16 Interrupt
 * 0x17 NMI
 * 0x18 trapa instruction execution
 * 0x19 UBCA match
 * 0x1a UBCB match
 * 0x21 Instruction cache fill
 * 0x22 Operand cache fill
 * 0x23 Elapsed time
 * 0x24 Pipeline freeze by I-cache miss
 * 0x25 Pipeline freeze by D-cache miss
 * 0x27 Pipeline freeze by branch instruction
 * 0x28 Pipeline freeze by CPU register
 * 0x29 Pipeline freeze by FPU
 */

static SH7750_GENERAL_EVENTS: [i32; PERF_COUNT_HW_MAX] = [
    0; PERF_COUNT_HW_MAX
];

// Equivalent to the designated initializers above; values are installed by the
// architecture's event-number constants in the final kernel integration.
const fn sh7750_general_events_init() -> [i32; PERF_COUNT_HW_MAX] {
    let mut events = [0; PERF_COUNT_HW_MAX];
    events[PERF_COUNT_HW_CPU_CYCLES] = 0x0023;
    events[PERF_COUNT_HW_INSTRUCTIONS] = 0x000a;
    events[PERF_COUNT_HW_CACHE_REFERENCES] = 0x0006; // I-cache
    events[PERF_COUNT_HW_CACHE_MISSES] = 0x0008; // I-cache
    events[PERF_COUNT_HW_BRANCH_INSTRUCTIONS] = 0x0010;
    events[PERF_COUNT_HW_BRANCH_MISSES] = -1;
    events[PERF_COUNT_HW_BUS_CYCLES] = -1;
    events
}

static SH7750_GENERAL_EVENTS_INIT: [i32; PERF_COUNT_HW_MAX] = sh7750_general_events_init();

const fn sh7750_cache_events_init() -> [[[i32; PERF_COUNT_HW_CACHE_RESULT_MAX]; PERF_COUNT_HW_CACHE_OP_MAX]; PERF_COUNT_HW_CACHE_MAX] {
    let mut e = [[[0; PERF_COUNT_HW_CACHE_RESULT_MAX]; PERF_COUNT_HW_CACHE_OP_MAX]; PERF_COUNT_HW_CACHE_MAX];
    e[C(PERF_COUNT_HW_CACHE_L1D)][C(PERF_COUNT_HW_CACHE_OP_READ)][C(PERF_COUNT_HW_CACHE_RESULT_ACCESS)] = 0x0001;
    e[C(PERF_COUNT_HW_CACHE_L1D)][C(PERF_COUNT_HW_CACHE_OP_READ)][C(PERF_COUNT_HW_CACHE_RESULT_MISS)] = 0x0004;
    e[C(PERF_COUNT_HW_CACHE_L1D)][C(PERF_COUNT_HW_CACHE_OP_WRITE)][C(PERF_COUNT_HW_CACHE_RESULT_ACCESS)] = 0x0002;
    e[C(PERF_COUNT_HW_CACHE_L1D)][C(PERF_COUNT_HW_CACHE_OP_WRITE)][C(PERF_COUNT_HW_CACHE_RESULT_MISS)] = 0x0005;
    e[C(PERF_COUNT_HW_CACHE_L1I)][C(PERF_COUNT_HW_CACHE_OP_READ)][C(PERF_COUNT_HW_CACHE_RESULT_ACCESS)] = 0x0006;
    e[C(PERF_COUNT_HW_CACHE_L1I)][C(PERF_COUNT_HW_CACHE_OP_READ)][C(PERF_COUNT_HW_CACHE_RESULT_MISS)] = 0x0008;
    e[C(PERF_COUNT_HW_CACHE_DTLB)][C(PERF_COUNT_HW_CACHE_OP_READ)][C(PERF_COUNT_HW_CACHE_RESULT_MISS)] = 0x0003;
    e[C(PERF_COUNT_HW_CACHE_ITLB)][C(PERF_COUNT_HW_CACHE_OP_READ)][C(PERF_COUNT_HW_CACHE_RESULT_MISS)] = 0x0007;
    e
}

const fn C(x: usize) -> usize { x }
static SH7750_CACHE_EVENTS: [[[i32; PERF_COUNT_HW_CACHE_RESULT_MAX]; PERF_COUNT_HW_CACHE_OP_MAX]; PERF_COUNT_HW_CACHE_MAX] = sh7750_cache_events_init();

extern "C" {
    fn __raw_readl(addr: usize) -> u32;
    fn __raw_readw(addr: usize) -> u16;
    fn __raw_writew(value: u16, addr: usize);
    fn register_sh_pmu(pmu: *mut sh_pmu) -> i32;
    static mut boot_cpu_data: cpuinfo;
    fn pr_notice(fmt: *const u8, ...);
}

#[repr(C)]
pub struct hw_perf_event { pub config: u16 }

#[repr(C)]
pub struct sh_pmu {
    pub name: *const u8,
    pub num_events: i32,
    pub event_map: Option<unsafe extern "C" fn(i32) -> i32>,
    pub max_events: usize,
    pub raw_event_mask: u16,
    pub cache_events: *const [[[i32; PERF_COUNT_HW_CACHE_RESULT_MAX]; PERF_COUNT_HW_CACHE_OP_MAX]; PERF_COUNT_HW_CACHE_MAX],
    pub read: Option<unsafe extern "C" fn(i32) -> u64>,
    pub disable: Option<unsafe extern "C" fn(*mut hw_perf_event, i32)>,
    pub enable: Option<unsafe extern "C" fn(*mut hw_perf_event, i32)>,
    pub disable_all: Option<unsafe extern "C" fn()>,
    pub enable_all: Option<unsafe extern "C" fn()>,
}

unsafe extern "C" fn sh7750_event_map(event: i32) -> i32 { SH7750_GENERAL_EVENTS_INIT[event as usize] }

unsafe extern "C" fn sh7750_pmu_read(idx: i32) -> u64 {
    (((__raw_readl(PMCTRH(idx as usize)) & 0xffff) as u64) << 32) | __raw_readl(PMCTRL(idx as usize)) as u64
}

unsafe extern "C" fn sh7750_pmu_disable(_hwc: *mut hw_perf_event, idx: i32) {
    let mut tmp = __raw_readw(PMCR(idx as usize));
    tmp &= !(PMCR_PMM_MASK | PMCR_PMEN);
    __raw_writew(tmp, PMCR(idx as usize));
}

unsafe extern "C" fn sh7750_pmu_enable(hwc: *mut hw_perf_event, idx: i32) {
    __raw_writew(__raw_readw(PMCR(idx as usize)) | PMCR_PMCLR, PMCR(idx as usize));
    __raw_writew((*hwc).config | PMCR_PMEN | PMCR_PMST, PMCR(idx as usize));
}

unsafe extern "C" fn sh7750_pmu_disable_all() {
    for i in 0..sh7750_pmu.num_events { __raw_writew(__raw_readw(PMCR(i as usize)) & !PMCR_PMEN, PMCR(i as usize)); }
}

unsafe extern "C" fn sh7750_pmu_enable_all() {
    for i in 0..sh7750_pmu.num_events { __raw_writew(__raw_readw(PMCR(i as usize)) | PMCR_PMEN, PMCR(i as usize)); }
}

// C static initializer, expressed as a Rust constant initializer.
static mut SH7750_PMU: sh_pmu = sh_pmu {
    name: b"sh7750\0".as_ptr(), num_events: 2, event_map: Some(sh7750_event_map),
    max_events: PERF_COUNT_HW_MAX, raw_event_mask: PMCR_PMM_MASK,
    cache_events: &SH7750_CACHE_EVENTS, read: Some(sh7750_pmu_read),
    disable: Some(sh7750_pmu_disable), enable: Some(sh7750_pmu_enable),
    disable_all: Some(sh7750_pmu_disable_all), enable_all: Some(sh7750_pmu_enable_all),
};

unsafe extern "C" fn sh7750_pmu_init() -> i32 {
    /* Make sure this CPU actually has perf counters. */
    if (boot_cpu_data.flags & CPU_HAS_PERF_COUNTER) == 0 {
        pr_notice(b"HW perf events unsupported, software events only.\n\0".as_ptr());
        return -ENODEV;
    }
    register_sh_pmu(&mut SH7750_PMU)
}

// early_initcall(sh7750_pmu_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
