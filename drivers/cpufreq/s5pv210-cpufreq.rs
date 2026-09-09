// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2010 Samsung Electronics Co., Ltd. */

// Linux kernel includes and build-time symbols are supplied externally.
use core::ffi::c_void;

static mut clk_base: *mut c_void = core::ptr::null_mut();
static mut dmc_base: [*mut c_void; 2] = [core::ptr::null_mut(); 2];
macro_rules! S5P_CLKREG { ($x:expr) => { unsafe { (clk_base as *mut u8).add($x) as *mut c_void } }; }
const S5P_APLL_LOCK: *mut c_void = S5P_CLKREG!(0x00);
const S5P_APLL_CON: *mut c_void = S5P_CLKREG!(0x100);
const S5P_CLK_SRC0: *mut c_void = S5P_CLKREG!(0x200);
const S5P_CLK_SRC2: *mut c_void = S5P_CLKREG!(0x208);
const S5P_CLK_DIV0: *mut c_void = S5P_CLKREG!(0x300);
const S5P_CLK_DIV2: *mut c_void = S5P_CLKREG!(0x308);
const S5P_CLK_DIV6: *mut c_void = S5P_CLKREG!(0x318);
const S5P_CLKDIV_STAT0: *mut c_void = S5P_CLKREG!(0x1000);
const S5P_CLKDIV_STAT1: *mut c_void = S5P_CLKREG!(0x1004);
const S5P_CLKMUX_STAT0: *mut c_void = S5P_CLKREG!(0x1100);
const S5P_CLKMUX_STAT1: *mut c_void = S5P_CLKREG!(0x1104);
const S5P_ARM_MCS_CON: *mut c_void = S5P_CLKREG!(0x6100);
const S5P_CLKSRC0_MUX200_SHIFT: u32 = 16;
const S5P_CLKSRC0_MUX200_MASK: u32 = 1 << 16;
const S5P_CLKSRC2_G3D_SHIFT: u32 = 0;
const S5P_CLKSRC2_G3D_MASK: u32 = 3;
const S5P_CLKSRC2_MFC_SHIFT: u32 = 4;
const S5P_CLKSRC2_MFC_MASK: u32 = 3 << 4;
const S5P_CLKDIV0_APLL_MASK: u32 = 7;
const S5P_CLKDIV0_A2M_MASK: u32 = 7 << 4;
const S5P_CLKDIV0_HCLK200_MASK: u32 = 7 << 8;
const S5P_CLKDIV0_PCLK100_MASK: u32 = 7 << 12;
const S5P_CLKDIV0_HCLK166_MASK: u32 = 15 << 16;
const S5P_CLKDIV0_PCLK83_MASK: u32 = 7 << 20;
const S5P_CLKDIV0_HCLK133_MASK: u32 = 15 << 24;
const S5P_CLKDIV0_PCLK66_MASK: u32 = 7 << 28;
const S5P_CLKDIV2_G3D_MASK: u32 = 15;
const S5P_CLKDIV2_MFC_MASK: u32 = 15 << 4;
const S5P_CLKDIV6_ONEDRAM_MASK: u32 = 15 << 28;
const APLL_VAL_1000: u32 = (1 << 31) | (125 << 16) | (3 << 8) | 1;
const APLL_VAL_800: u32 = (1 << 31) | (100 << 16) | (3 << 8) | 1;
const SLEEP_FREQ: u32 = 800 * 1000;

static mut dmc0_clk: *mut c_void = core::ptr::null_mut();
static mut dmc1_clk: *mut c_void = core::ptr::null_mut();
static mut no_cpufreq_access: bool = false;

#[repr(C)]
struct dram_conf { freq: usize, refresh: usize }
static mut s5pv210_dram_conf: [dram_conf; 2] = [dram_conf { freq: 0, refresh: 0 }; 2];
#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
enum perf_level { L0, L1, L2, L3, L4 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
enum s5pv210_mem_type { LPDDR = 1, LPDDR2 = 2, DDR2 = 4 }
#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
enum s5pv210_dmc_port { DMC0 = 0, DMC1 }
#[repr(C)]
struct s5pv210_dvs_conf { arm_volt: i32, int_volt: i32 }
static mut dvs_conf: [s5pv210_dvs_conf; 5] = [
    s5pv210_dvs_conf { arm_volt: 1250000, int_volt: 1100000 },
    s5pv210_dvs_conf { arm_volt: 1200000, int_volt: 1100000 },
    s5pv210_dvs_conf { arm_volt: 1050000, int_volt: 1100000 },
    s5pv210_dvs_conf { arm_volt: 950000, int_volt: 1100000 },
    s5pv210_dvs_conf { arm_volt: 950000, int_volt: 1000000 },
];
const arm_volt_max: i32 = 1350000;
const int_volt_max: i32 = 1250000;
static mut clkdiv_val: [[u32; 11]; 5] = [
    [0,4,4,1,3,1,4,1,3,0,0], [0,3,3,1,3,1,4,1,3,0,0],
    [1,3,1,1,3,1,4,1,3,0,0], [3,3,1,1,3,1,4,1,3,0,0],
    [7,7,0,0,7,0,9,0,7,0,0],
];

extern "C" {
    fn readl_relaxed(addr: *mut c_void) -> u32;
    fn writel_relaxed(value: u32, addr: *mut c_void);
    fn regulator_set_voltage(reg: *mut c_void, min: i32, max: i32) -> i32;
}

unsafe fn s5pv210_set_refresh(ch: s5pv210_dmc_port, freq: usize) {
    let reg = match ch { s5pv210_dmc_port::DMC0 => (dmc_base[0] as *mut u8).add(0x30) as *mut c_void,
        s5pv210_dmc_port::DMC1 => (dmc_base[1] as *mut u8).add(0x30) as *mut c_void };
    let tmp = s5pv210_dram_conf[ch as usize].freq / freq;
    writel_relaxed((s5pv210_dram_conf[ch as usize].refresh / tmp) as u32, reg);
}

unsafe fn check_mem_type(dmc_reg: *mut c_void) -> i32 {
    ((readl_relaxed((dmc_reg as *mut u8).add(4) as *mut c_void) & (0xf << 8)) >> 8) as i32
}

// Source-level kernel entry points and driver registration, with external kernel types preserved.
extern "C" {
    fn s5pv210_target(policy: *mut c_void, index: u32) -> i32;
    fn s5pv210_cpu_init(policy: *mut c_void) -> i32;
    fn s5pv210_cpufreq_reboot_notifier_event(this: *mut c_void, event: usize, ptr: *mut c_void) -> i32;
    fn s5pv210_cpufreq_probe(pdev: *mut c_void) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
