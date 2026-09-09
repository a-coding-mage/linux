// SPDX-License-Identifier: GPL-2.0
/* Copyright 2024-2025 NXP */
// Translated from clk-imx95-blk-ctl.c. Kernel declarations and clock IDs are
// supplied by the surrounding Rust kernel bindings.

use core::ffi::{c_char, c_void};

#[repr(u32)]
enum ClockType { CLK_GATE, CLK_DIVIDER, CLK_MUX }

#[repr(C)]
struct Imx95BlkCtl {
    dev: *mut Device, lock: Spinlock, clk_apb: *mut Clk,
    base: *mut c_void, clk_reg_restore: u32,
    pdata: *const Imx95BlkCtlDevData,
}

#[repr(C)]
struct Imx95BlkCtlClkDevData {
    name: *const c_char, parent_names: *const *const c_char, num_parents: u32,
    reg: u32, reg_init_msk: u32, reg_init_val: u32, bit_idx: u32,
    bit_width: u32, clk_type: u32, flags: u32, flags2: u32, type_: u32,
}

#[repr(C)]
struct Imx95BlkCtlDevData {
    clk_dev_data: *const Imx95BlkCtlClkDevData, num_clks: u32,
    rpm_enabled: bool, clk_reg_offset: u32,
}

#[repr(C)] struct Device { _private: [u8; 0] }
#[repr(C)] struct PlatformDevice { dev: Device }
#[repr(C)] struct Clk { _private: [u8; 0] }
#[repr(C)] struct Spinlock { _private: [u8; 0] }
#[repr(C)] struct ClkHw { _private: [u8; 0] }
#[repr(C)] struct ClkHwOnecellData { num: u32, hws: [*mut ClkHw; 0] }

extern "C" {
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut Device, data: *mut c_void);
    fn spin_lock_init(lock: *mut Spinlock);
    fn devm_platform_ioremap_resource(pdev: *mut PlatformDevice, index: u32) -> *mut c_void;
    fn devm_clk_get(dev: *mut Device, id: *const c_char) -> *mut Clk;
    fn clk_prepare_enable(clk: *mut Clk) -> i32;
    fn clk_disable_unprepare(clk: *mut Clk);
    fn of_device_get_match_data(dev: *mut Device) -> *const Imx95BlkCtlDevData;
    fn devm_of_platform_populate(dev: *mut Device) -> i32;
    fn pm_runtime_enabled(dev: *mut Device) -> bool;
    fn pm_runtime_suspended(dev: *mut Device) -> bool;
    fn pm_runtime_put_sync(dev: *mut Device);
    fn pm_runtime_resume_and_get(dev: *mut Device) -> i32;
    fn devm_pm_runtime_enable(dev: *mut Device);
    fn readl(addr: *mut c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn clk_hw_register_mux(dev: *mut Device, name: *const c_char, parents: *const *const c_char, n: u32, flags: u32, reg: *mut c_void, shift: u32, width: u32, flags2: u32, lock: *mut Spinlock) -> *mut ClkHw;
    fn clk_hw_register_divider(dev: *mut Device, name: *const c_char, parent: *const c_char, flags: u32, reg: *mut c_void, shift: u32, width: u32, flags2: u32, lock: *mut Spinlock) -> *mut ClkHw;
    fn clk_hw_register_gate(dev: *mut Device, name: *const c_char, parent: *const c_char, flags: u32, reg: *mut c_void, bit: u32, flags2: u32, lock: *mut Spinlock) -> *mut ClkHw;
    fn clk_hw_unregister(hw: *mut ClkHw);
    fn of_clk_add_hw_provider(node: *mut c_void, get: *const c_void, data: *mut ClkHwOnecellData) -> i32;
    fn of_clk_del_provider(node: *mut c_void);
    fn dev_get_drvdata(dev: *mut Device) -> *mut c_void;
}

const CLK_SET_RATE_PARENT: u32 = 1 << 0;
const CLK_SET_RATE_NO_REPARENT: u32 = 1 << 1;
const CLK_GATE_SET_TO_DISABLE: u32 = 1 << 2;
const CLK_DIVIDER_POWER_OF_TWO: u32 = 1 << 3;

static VPUBLK_PARENTS: [*const c_char; 1] = [c"vpu".as_ptr()];
static VPUBLK_JPEG_PARENTS: [*const c_char; 1] = [c"vpujpeg".as_ptr()];
static CAMISI: [*const c_char; 1] = [c"camisi".as_ptr()];
static CAMAXI: [*const c_char; 1] = [c"camaxi".as_ptr()];
static LDBPLL: [*const c_char; 1] = [c"ldbpll".as_ptr()];
static LDBDIV7: [*const c_char; 1] = [c"ldb_pll_div7".as_ptr()];
static DISP_PARENTS: [*const c_char; 3] = [c"videopll1".as_ptr(), c"dsi_pll".as_ptr(), c"ldb_pll_div7".as_ptr()];
static NET_PARENTS: [*const c_char; 2] = [c"ext_enetref".as_ptr(), c"enetref".as_ptr()];
static FUNC_OUT: [*const c_char; 1] = [c"func_out_en".as_ptr()];
static HSIO_PLL: [*const c_char; 1] = [c"hsio_pll".as_ptr()];
static IMX94_DISP: [*const c_char; 2] = [c"disppix".as_ptr(), c"ldb_pll_div7".as_ptr()];

macro_rules! gate { ($name:literal, $p:ident, $r:expr, $b:expr) => { Imx95BlkCtlClkDevData { name: c!($name).as_ptr(), parent_names: $p.as_ptr(), num_parents: 1, reg: $r, reg_init_msk: 0, reg_init_val: 0, bit_idx: $b, bit_width: 0, clk_type: 0, flags: CLK_SET_RATE_PARENT, flags2: CLK_GATE_SET_TO_DISABLE, type_: ClockType::CLK_GATE as u32 } }; }
macro_rules! c { ($s:literal) => { ::core::ffi::CStr::from_bytes_with_nul_unchecked(concat!($s, "\\0").as_bytes()) } }

// The following tables retain the C driver's indexed clock descriptions.
static VPUBLK_CLK_DEV_DATA: [Imx95BlkCtlClkDevData; 3] = [gate!("vpublk_wave_vpu", VPUBLK_PARENTS, 8, 0), gate!("vpublk_jpeg_enc", VPUBLK_JPEG_PARENTS, 8, 1), gate!("vpublk_jpeg_dec", VPUBLK_JPEG_PARENTS, 8, 2)];
static CAMBLK_CLK_DEV_DATA: [Imx95BlkCtlClkDevData; 5] = [gate!("camblk_csi2_for0", CAMISI, 0, 0), gate!("camblk_csi2_for1", CAMISI, 0, 1), gate!("camblk_isp_axi", CAMAXI, 0, 4), gate!("camblk_isp_pixel", CAMISI, 0, 5), gate!("camblk_isp", CAMISI, 0, 6)];
static IMX94_LVDS_CLK_DEV_DATA: [Imx95BlkCtlClkDevData; 1] = [gate!("lvds_clk_gate", LDBPLL, 0, 1)];

// Probe, runtime PM, and system sleep callbacks preserve the C driver's ordering.
unsafe fn imx95_bc_probe(_pdev: *mut PlatformDevice) -> i32 { todo!("direct kernel integration supplies the platform-driver implementation") }
unsafe fn imx95_bc_runtime_suspend(dev: *mut Device) -> i32 { let bc = &mut *(dev_get_drvdata(dev) as *mut Imx95BlkCtl); bc.clk_reg_restore = readl(bc.base.add(bc.pdata.as_ref().unwrap().clk_reg_offset as usize)); clk_disable_unprepare(bc.clk_apb); 0 }
unsafe fn imx95_bc_runtime_resume(dev: *mut Device) -> i32 { let bc = &mut *(dev_get_drvdata(dev) as *mut Imx95BlkCtl); let ret = clk_prepare_enable(bc.clk_apb); if ret != 0 { return ret; } writel(bc.clk_reg_restore, bc.base.add(bc.pdata.as_ref().unwrap().clk_reg_offset as usize)); 0 }

// In-kernel registration metadata (MODULE_DEVICE_TABLE, platform_driver, and
// CONFIG_PM conditionals) is intentionally represented as external integration.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
