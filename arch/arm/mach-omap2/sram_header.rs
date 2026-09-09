/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Interface for functions that need to be run in internal SRAM
 */

unsafe extern "C" {
    pub fn omap2_sram_ddr_init(
        slow_dll_ctrl: *mut u32,
        fast_dll_ctrl: u32,
        base_cs: u32,
        force_unlock: u32,
    );
    pub fn omap2_sram_reprogram_sdrc(perf_level: u32, dll_val: u32, mem_type: u32);
    pub fn omap2_set_prcm(dpll_ctrl_val: u32, sdrc_rfr_val: u32, bypass: i32) -> u32;

    pub fn omap3_sram_restore_context();

    /* C declaration uses the __init section attribute. */
    pub fn omap_sram_init() -> i32;

    pub fn omap_sram_push(funcp: *mut core::ffi::c_void, size: usize)
        -> *mut core::ffi::c_void;

    pub fn omap242x_sram_ddr_init(
        slow_dll_ctrl: *mut u32,
        fast_dll_ctrl: u32,
        base_cs: u32,
        force_unlock: u32,
    );
    pub static mut omap242x_sram_ddr_init_sz: usize;

    pub fn omap242x_sram_set_prcm(dpll_ctrl_val: u32, sdrc_rfr_val: u32, bypass: i32) -> u32;
    pub static mut omap242x_sram_set_prcm_sz: usize;

    pub fn omap242x_sram_reprogram_sdrc(perf_level: u32, dll_val: u32, mem_type: u32);
    pub static mut omap242x_sram_reprogram_sdrc_sz: usize;

    pub fn omap243x_sram_ddr_init(
        slow_dll_ctrl: *mut u32,
        fast_dll_ctrl: u32,
        base_cs: u32,
        force_unlock: u32,
    );
    pub static mut omap243x_sram_ddr_init_sz: usize;

    pub fn omap243x_sram_set_prcm(dpll_ctrl_val: u32, sdrc_rfr_val: u32, bypass: i32) -> u32;
    pub static mut omap243x_sram_set_prcm_sz: usize;

    pub fn omap243x_sram_reprogram_sdrc(perf_level: u32, dll_val: u32, mem_type: u32);
    pub static mut omap243x_sram_reprogram_sdrc_sz: usize;
}

/* CONFIG_PM conditional: enable the external idle function when configured. */
#[cfg(feature = "CONFIG_PM")]
pub unsafe extern "C" {
    pub fn omap_push_sram_idle();
}

#[cfg(not(feature = "CONFIG_PM"))]
#[inline]
pub fn omap_push_sram_idle() {}

/*
 * OMAP2+: define the SRAM PA addresses.
 * Used by the SRAM management code and the idle sleep code.
 */
pub const OMAP2_SRAM_PA: u32 = 0x4020_0000;
pub const OMAP3_SRAM_PA: u32 = 0x4020_0000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
