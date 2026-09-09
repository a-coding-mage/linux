// SPDX-License-Identifier: GPL-2.0
/*
 * Performance events support for SH-4A performance counters
 *
 *  Copyright (C) 2009, 2010  Paul Mundt
 */

const fn ppc_ccbr(idx: usize) -> usize { 0xff200800 + core::mem::size_of::<u32>() * idx }
const fn ppc_pmctr(idx: usize) -> usize { 0xfc100000 + core::mem::size_of::<u32>() * idx }

const CCBR_CIT_MASK: u32 = 0x7ff << 6;
const CCBR_DUC: u32 = 1 << 3;
const CCBR_CMDS: u32 = 1 << 1;
const CCBR_PPCE: u32 = 1;

#[cfg(CONFIG_CPU_SHX3)]
const PPC_PMCAT: usize = 0xfc100240;
#[cfg(not(CONFIG_CPU_SHX3))]
const PPC_PMCAT: usize = 0xfc100080;

const PMCAT_OVF3: u32 = 1 << 27;
const PMCAT_CNN3: u32 = 1 << 26;
const PMCAT_CLR3: u32 = 1 << 25;
const PMCAT_OVF2: u32 = 1 << 19;
const PMCAT_CLR2: u32 = 1 << 17;
const PMCAT_OVF1: u32 = 1 << 11;
const PMCAT_CNN1: u32 = 1 << 10;
const PMCAT_CLR1: u32 = 1 << 9;
const PMCAT_OVF0: u32 = 1 << 3;
const PMCAT_CLR0: u32 = 1 << 1;
const PMCAT_EMU_CLR_MASK: u32 = (1 << 24) | (1 << 16) | (1 << 8) | (1 << 0);

static mut SH4A_PMU: sh_pmu = sh_pmu {
    name: "sh4a",
    num_events: 2,
    event_map: sh4a_event_map,
    max_events: SH4A_GENERAL_EVENTS.len(),
    raw_event_mask: 0x3ff,
    cache_events: &SH4A_CACHE_EVENTS,
    read: sh4a_pmu_read,
    disable: sh4a_pmu_disable,
    enable: sh4a_pmu_enable,
    disable_all: sh4a_pmu_disable_all,
    enable_all: sh4a_pmu_enable_all,
};

/* Supported raw event codes are documented in the original C source. */
static SH4A_GENERAL_EVENTS: [i32; 7] = [0x0000, 0x0202, 0x0029, 0x002a, 0x0204, -1, -1];

static SH4A_CACHE_EVENTS: [[[i32; 2]; 3]; 7] = [
    [[0x0031, 0x0032], [0x0039, 0x003a], [0, 0]],
    [[0x0029, 0x002a], [-1, -1], [0, 0]],
    [[0x0030, 0], [0x0038, 0], [0, 0]],
    [[0x0222, 0x0220], [0, 0], [0, 0]],
    [[0, 0x02a0], [-1, -1], [-1, -1]],
    [[-1, -1], [-1, -1], [-1, -1]],
    [[-1, -1], [-1, -1], [-1, -1]],
];

unsafe extern "C" {
    fn __raw_readl(addr: usize) -> u32;
    fn __raw_writel(value: u32, addr: usize);
    fn register_sh_pmu(pmu: *mut sh_pmu) -> i32;
    static mut boot_cpu_data: processor;
    fn pr_notice(fmt: *const u8, ...);
}

#[repr(C)]
pub struct hw_perf_event { pub config: u64 }
#[repr(C)]
pub struct processor { pub flags: u32 }
#[repr(C)]
pub struct sh_pmu {
    pub name: &'static str,
    pub num_events: i32,
    pub event_map: fn(i32) -> i32,
    pub max_events: usize,
    pub raw_event_mask: u64,
    pub cache_events: &'static [[[i32; 2]; 3]; 7],
    pub read: fn(i32) -> u64,
    pub disable: fn(*mut hw_perf_event, i32),
    pub enable: fn(*mut hw_perf_event, i32),
    pub disable_all: fn(),
    pub enable_all: fn(),
}

fn sh4a_event_map(event: i32) -> i32 { unsafe { SH4A_GENERAL_EVENTS[event as usize] } }

fn sh4a_pmu_read(idx: i32) -> u64 { unsafe { __raw_readl(ppc_pmctr(idx as usize)) as u64 } }

fn sh4a_pmu_disable(_hwc: *mut hw_perf_event, idx: i32) {
    unsafe {
        let mut tmp = __raw_readl(ppc_ccbr(idx as usize));
        tmp &= !(CCBR_CIT_MASK | CCBR_DUC);
        __raw_writel(tmp, ppc_ccbr(idx as usize));
    }
}

fn sh4a_pmu_enable(hwc: *mut hw_perf_event, idx: i32) {
    unsafe {
        let mut tmp = __raw_readl(PPC_PMCAT);
        tmp &= !PMCAT_EMU_CLR_MASK;
        tmp |= if idx != 0 { PMCAT_CLR1 } else { PMCAT_CLR0 };
        __raw_writel(tmp, PPC_PMCAT);
        tmp = __raw_readl(ppc_ccbr(idx as usize));
        tmp |= ((*hwc).config as u32) << 6 | CCBR_CMDS | CCBR_PPCE;
        __raw_writel(tmp, ppc_ccbr(idx as usize));
        __raw_writel(__raw_readl(ppc_ccbr(idx as usize)) | CCBR_DUC, ppc_ccbr(idx as usize));
    }
}

fn sh4a_pmu_disable_all() {
    unsafe { for i in 0..SH4A_PMU.num_events { __raw_writel(__raw_readl(ppc_ccbr(i as usize)) & !CCBR_DUC, ppc_ccbr(i as usize)); } }
}

fn sh4a_pmu_enable_all() {
    unsafe { for i in 0..SH4A_PMU.num_events { __raw_writel(__raw_readl(ppc_ccbr(i as usize)) | CCBR_DUC, ppc_ccbr(i as usize)); } }
}

fn sh4a_pmu_init() -> i32 {
    unsafe {
        if boot_cpu_data.flags & CPU_HAS_PERF_COUNTER == 0 {
            pr_notice(b"HW perf events unsupported, software events only.\0".as_ptr());
            return -ENODEV;
        }
        register_sh_pmu(&raw mut SH4A_PMU)
    }
}

const CPU_HAS_PERF_COUNTER: u32 = 0;
const ENODEV: i32 = 19;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
