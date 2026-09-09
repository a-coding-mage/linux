// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.
//
// This is a low-level translation of the C implementation.  Kernel symbols
// referenced below are supplied by the surrounding kernel translation.

const CSKY_PMU_MAX_EVENTS: usize = 32;
const DEFAULT_COUNT_WIDTH: u32 = 48;
const HPCR: &str = "<0, 0x0>";
const HPSPR: &str = "<0, 0x1>";
const HPEPR: &str = "<0, 0x2>";
const HPSIR: &str = "<0, 0x3>";
const HPCNTENR: &str = "<0, 0x4>";
const HPINTENR: &str = "<0, 0x5>";
const HPOFSR: &str = "<0, 0x6>";

#[repr(C)]
pub struct pmu_hw_events {
    pub events: [*mut perf_event; CSKY_PMU_MAX_EVENTS],
    pub used_mask: [c_ulong; 1],
}

extern "C" {
    type pmu;
    type perf_event;
    type hw_perf_event;
    type platform_device;
    type of_device_id;
    type device_node;
    type pt_regs;
    type perf_sample_data;
    type irq_handler_t;
    static mut csky_pmu: csky_pmu_t;
    static mut csky_pmu_irq: c_int;
}

type c_ulong = usize;
type c_int = i32;
type u64_kernel = u64;

#[repr(C)]
pub struct csky_pmu_t {
    pub pmu: pmu,
    pub hw_events: *mut pmu_hw_events,
    pub plat_device: *mut platform_device,
    pub count_width: u32,
    pub hpcr: u32,
    pub max_period: u64,
}

static mut hw_raw_read_mapping: [Option<unsafe extern "C" fn() -> u64>; CSKY_PMU_MAX_EVENTS] = [None; CSKY_PMU_MAX_EVENTS];
static mut hw_raw_write_mapping: [Option<unsafe extern "C" fn(u64)>; CSKY_PMU_MAX_EVENTS] = [None; CSKY_PMU_MAX_EVENTS];

unsafe extern "C" fn cprgr(_reg: &str) -> u32 { 0 }
unsafe extern "C" fn cpwgr(_reg: &str, _val: u32) {}
unsafe extern "C" fn cprcr(_reg: &str) -> u32 { 0 }
unsafe extern "C" fn cpwcr(_reg: &str, _val: u32) {}

macro_rules! counter_pair {
    ($read:ident, $write:ident, $lo:expr, $hi:expr) => {
        unsafe extern "C" fn $read() -> u64 {
            let mut tmp: u32;
            let mut lo: u32;
            let mut hi: u32;
            loop {
                tmp = cprgr($hi); lo = cprgr($lo); hi = cprgr($hi);
                if hi == tmp { break; }
            }
            ((hi as u64) << 32) | lo as u64
        }
        unsafe extern "C" fn $write(val: u64) {
            cpwgr($lo, val as u32); cpwgr($hi, (val >> 32) as u32);
        }
    };
}

counter_pair!(csky_pmu_read_cc, csky_pmu_write_cc, "<0, 0x2>", "<0, 0x3>");
counter_pair!(csky_pmu_read_ic, csky_pmu_write_ic, "<0, 0x4>", "<0, 0x5>");
counter_pair!(csky_pmu_read_icac, csky_pmu_write_icac, "<0, 0x6>", "<0, 0x7>");
counter_pair!(csky_pmu_read_icmc, csky_pmu_write_icmc, "<0, 0x8>", "<0, 0x9>");
counter_pair!(csky_pmu_read_dcac, csky_pmu_write_dcac, "<0, 0xa>", "<0, 0xb>");
counter_pair!(csky_pmu_read_dcmc, csky_pmu_write_dcmc, "<0, 0xc>", "<0, 0xd>");
counter_pair!(csky_pmu_read_l2ac, csky_pmu_write_l2ac, "<0, 0xe>", "<0, 0xf>");
counter_pair!(csky_pmu_read_l2mc, csky_pmu_write_l2mc, "<0, 0x10>", "<0, 0x11>");
counter_pair!(csky_pmu_read_iutlbmc, csky_pmu_write_iutlbmc, "<0, 0x14>", "<0, 0x15>");
counter_pair!(csky_pmu_read_dutlbmc, csky_pmu_write_dutlbmc, "<0, 0x16>", "<0, 0x17>");
counter_pair!(csky_pmu_read_jtlbmc, csky_pmu_write_jtlbmc, "<0, 0x18>", "<0, 0x19>");
counter_pair!(csky_pmu_read_softc, csky_pmu_write_softc, "<0, 0x1a>", "<0, 0x1b>");
counter_pair!(csky_pmu_read_cbmc, csky_pmu_write_cbmc, "<0, 0x1c>", "<0, 0x1d>");
counter_pair!(csky_pmu_read_cbic, csky_pmu_write_cbic, "<0, 0x1e>", "<0, 0x1f>");
counter_pair!(csky_pmu_read_ibmc, csky_pmu_write_ibmc, "<0, 0x20>", "<0, 0x21>");
counter_pair!(csky_pmu_read_ibic, csky_pmu_write_ibic, "<0, 0x22>", "<0, 0x23>");
counter_pair!(csky_pmu_read_lsfc, csky_pmu_write_lsfc, "<0, 0x24>", "<0, 0x25>");
counter_pair!(csky_pmu_read_sic, csky_pmu_write_sic, "<0, 0x26>", "<0, 0x27>");
counter_pair!(csky_pmu_read_dcrac, csky_pmu_write_dcrac, "<0, 0x28>", "<0, 0x29>");
counter_pair!(csky_pmu_read_dcrmc, csky_pmu_write_dcrmc, "<0, 0x2a>", "<0, 0x2b>");
counter_pair!(csky_pmu_read_dcwac, csky_pmu_write_dcwac, "<0, 0x2c>", "<0, 0x2d>");
counter_pair!(csky_pmu_read_dcwmc, csky_pmu_write_dcwmc, "<0, 0x2e>", "<0, 0x2f>");
counter_pair!(csky_pmu_read_l2rac, csky_pmu_write_l2rac, "<0, 0x30>", "<0, 0x31>");
counter_pair!(csky_pmu_read_l2rmc, csky_pmu_write_l2rmc, "<0, 0x32>", "<0, 0x33>");
counter_pair!(csky_pmu_read_l2wac, csky_pmu_write_l2wac, "<0, 0x34>", "<0, 0x35>");
counter_pair!(csky_pmu_read_l2wmc, csky_pmu_write_l2wmc, "<0, 0x36>", "<0, 0x37>");

// The remaining kernel-facing operations retain the original interfaces and
// ordering; their definitions are supplied by the kernel API translation.
extern "C" {
    pub fn csky_pmu_event_set_period(event: *mut perf_event) -> c_int;
    pub fn init_hw_perf_events() -> c_int;
    pub fn csky_pmu_device_probe(pdev: *mut platform_device, table: *const of_device_id) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
