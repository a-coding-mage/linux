/*
 * Header for code common to all OMAP2+ machines.
 * Translated from the C header; included kernel dependencies are external.
 */

use core::ffi::{c_char, c_void};

// C dependencies: linux/irq.h, linux/delay.h, linux/i2c.h,
// linux/mfd/twl.h, linux/platform_data/i2c-omap.h, linux/reboot.h,
// linux/irqchip/irq-omap-intc.h, asm/proc-fns.h, asm/hardware/cache-l2x0.h,
// and i2c.h.

pub const OMAP_INTC_START: u32 = NR_IRQS;

pub type RebootMode = i32;
pub type U32 = u32;
pub type U8 = u8;

pub enum Device {}
pub enum DeviceNode {}
pub enum OfDeviceId {}
pub enum OmapSystemDmaPlatInfo {}
pub enum OmapSdrcParams {}
pub enum OmapHwmod {}
pub enum SmpOperations {}
pub enum PlatformDevice {}

extern "C" {
    pub static mut NR_IRQS: u32;
    pub static mut omap_pm_soc_init: Option<unsafe extern "C" fn() -> i32>;
    pub fn omap_pm_nop_init() -> i32;

    pub fn omap3_pm_init() -> i32;
    pub fn omap4_pm_init() -> i32;
    pub fn omap4_pm_init_early() -> i32;
    pub fn amx3_common_pm_init() -> i32;

    pub fn omap_l2_cache_init() -> i32;
    pub fn omap4_l2c310_write_sec(val: c_ulong, reg: u32);
    pub fn omap5_realtime_timer_init();

    pub fn omap2420_init_early();
    pub fn omap2430_init_early();
    pub fn omap3430_init_early();
    pub fn omap3630_init_early();
    pub fn am33xx_init_early();
    pub fn am35xx_init_early();
    pub fn ti814x_init_early();
    pub fn ti816x_init_early();
    pub fn am43xx_init_early();
    pub fn am43xx_init_late();
    pub fn omap4430_init_early();
    pub fn omap5_init_early();
    pub fn omap3_init_late();
    pub fn omap4430_init_late();
    pub fn ti81xx_init_late();
    pub fn am33xx_init_late();
    pub fn omap5_init_late();
    pub fn dra7xx_init_early();
    pub fn dra7xx_init_late();
    pub fn omap_soc_device_init();

    pub fn omap2xxx_restart(mode: RebootMode, cmd: *const c_char);
    pub fn am33xx_restart(mode: RebootMode, cmd: *const c_char);
    pub fn omap3xxx_restart(mode: RebootMode, cmd: *const c_char);
    pub fn ti81xx_restart(mode: RebootMode, cmd: *const c_char);
    pub fn omap44xx_restart(mode: RebootMode, cmd: *const c_char);
    pub fn omap_barrier_reserve_memblock();
    pub fn omap_barriers_init();
    pub fn omap2_set_globals_tap(class: u32, tap: *mut c_void);
    pub fn omap242x_map_io();
    pub fn omap243x_map_io();
    pub fn omap3_map_io();
    pub fn am33xx_map_io();
    pub fn omap4_map_io();
    pub fn omap5_map_io();
    pub fn dra7xx_map_io();
    pub fn ti81xx_map_io();
    pub fn omap_gic_of_init();
    pub fn omap4_get_l2cache_base() -> *mut c_void;
    pub fn omap4_get_scu_base() -> *mut c_void;
    pub fn gic_dist_disable();
    pub fn gic_dist_enable();
    pub fn gic_dist_disabled() -> bool;
    pub fn gic_timer_retrigger();
    pub fn _omap_smc1(fn_: u32, arg: u32);
    pub fn omap4_sar_ram_init();
    pub fn omap4_get_sar_ram_base() -> *mut c_void;
    pub fn omap4_mpuss_early_init();
    pub fn omap_do_wfi();
    pub fn omap_interconnect_sync();
    pub fn omap_modify_auxcoreboot0(set_mask: u32, clear_mask: u32) -> u32;
    pub fn omap_auxcoreboot_addr(cpu_addr: u32);
    pub fn omap_read_auxcoreboot0() -> u32;
    pub fn omap4_cpu_die(cpu: u32);
    pub fn omap4_cpu_kill(cpu: u32) -> i32;
    pub static omap4_smp_ops: SmpOperations;
    pub fn omap4_get_cpu1_ns_pa_addr() -> u32;
    pub fn omap4_mpuss_init() -> i32;
    pub fn omap4_enter_lowpower(cpu: u32, power_state: u32, rcuidle: bool) -> i32;
    pub fn omap4_hotplug_cpu(cpu: u32, power_state: u32) -> i32;
    pub fn omap4_secondary_startup();
    pub fn omap4460_secondary_startup();
    pub fn omap4_finish_suspend(cpu_state: c_ulong) -> i32;
    pub fn omap4_cpu_resume();
    pub fn omap5_secondary_startup();
    pub fn omap5_secondary_hyp_startup();
    pub fn pdata_quirks_init(id: *const OfDeviceId);
    pub fn omap_auxdata_legacy_init(dev: *mut Device);
    pub fn omap_pcs_legacy_init(irq: i32, rearm: Option<unsafe extern "C" fn()>);
    pub static mut dma_plat_info: OmapSystemDmaPlatInfo;
    pub fn omap_sdrc_init(cs0: *mut OmapSdrcParams, cs1: *mut OmapSdrcParams);
    pub fn omap_reserve();
    pub fn omap_dss_reset(hw: *mut OmapHwmod) -> i32;
    pub fn omap_clk_init() -> i32;
    pub fn omap_iommu_set_pwrdm_constraint(pdev: *mut PlatformDevice, request: bool, pwrst: *mut u8) -> i32;
    pub fn udelay(usecs: u32);
    pub fn cpu_do_idle();
}

pub const OMAP_L2C_AUX_CTRL: u32 = L2C_AUX_CTRL_SHARED_OVERRIDE | L310_AUX_CTRL_DATA_PREFETCH | L310_AUX_CTRL_INSTR_PREFETCH;
pub const L2C_AUX_CTRL_SHARED_OVERRIDE: u32 = 0;
pub const L310_AUX_CTRL_DATA_PREFETCH: u32 = 0;
pub const L310_AUX_CTRL_INSTR_PREFETCH: u32 = 0;

pub type CLong = core::ffi::c_ulong;
pub type c_ulong = usize;

#[inline]
pub unsafe fn omap_test_timeout<F: Fn() -> bool>(cond: F, timeout: u32, index: &mut u32) {
    *index = 0;
    while *index < timeout {
        if cond() { break; }
        udelay(1);
        *index = index.wrapping_add(1);
    }
}

#[inline] pub unsafe fn omap3_pm_init_fallback() -> i32 { 0 }
#[inline] pub unsafe fn omap4_pm_init_fallback() -> i32 { 0 }
#[inline] pub unsafe fn omap4_pm_init_early_fallback() -> i32 { 0 }
#[inline] pub unsafe fn amx3_common_pm_init_fallback() -> i32 { 0 }
#[inline] pub unsafe fn omap_l2_cache_init_fallback() -> i32 { 0 }
#[inline] pub unsafe fn omap5_realtime_timer_init_fallback() {}
#[inline] pub unsafe fn omap_soc_device_init_fallback() {}
#[inline] pub unsafe fn omap4_get_scu_base_fallback() -> *mut c_void { core::ptr::null_mut() }
#[inline] pub unsafe fn omap4_enter_lowpower_fallback(_: u32, _: u32, _: bool) -> i32 { cpu_do_idle(); 0 }
#[inline] pub unsafe fn omap4_hotplug_cpu_fallback(_: u32, _: u32) -> i32 { cpu_do_idle(); 0 }
#[inline] pub unsafe fn omap4_mpuss_init_fallback() -> i32 { 0 }
#[inline] pub unsafe fn omap4_secondary_startup_fallback() {}
#[inline] pub unsafe fn omap4460_secondary_startup_fallback() {}
#[inline] pub unsafe fn omap4_finish_suspend_fallback(_: c_ulong) -> i32 { 0 }
#[inline] pub unsafe fn omap4_cpu_resume_fallback() {}
#[inline] pub unsafe fn omap5_secondary_startup_fallback() {}
#[inline] pub unsafe fn omap5_secondary_hyp_startup_fallback() {}
#[inline] pub unsafe fn omap_barrier_reserve_memblock_fallback() {}
#[inline] pub unsafe fn omap2xxx_restart_fallback(_: RebootMode, _: *const c_char) {}
#[inline] pub unsafe fn am33xx_restart_fallback(_: RebootMode, _: *const c_char) {}
#[inline] pub unsafe fn omap3xxx_restart_fallback(_: RebootMode, _: *const c_char) {}
#[inline] pub unsafe fn ti81xx_restart_fallback(_: RebootMode, _: *const c_char) {}
#[inline] pub unsafe fn omap44xx_restart_fallback(_: RebootMode, _: *const c_char) {}
#[inline] pub unsafe fn omap_iommu_set_pwrdm_constraint_fallback(_: *mut PlatformDevice, _: bool, _: *mut u8) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
