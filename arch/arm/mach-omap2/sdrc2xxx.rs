// SPDX-License-Identifier: GPL-2.0-only
/*
 * SDRAM timing related functions for OMAP2xxx
 *
 * Copyright (C) 2005, 2008 Texas Instruments Inc.
 * Copyright (C) 2005, 2008 Nokia Corporation
 *
 * Tony Lindgren <tony@atomide.com>
 * Paul Walmsley
 * Richard Woodruff <r-woodruff2@ti.com>
 */

// Linux kernel and OMAP dependencies are supplied by the surrounding crate.

/* Memory timing, DLL mode flags */
const M_DDR: u32 = 1;
const M_LOCK_CTRL: u32 = 1 << 2;
const M_UNLOCK: u32 = 0;
const M_LOCK: u32 = 1;

#[repr(C)]
pub struct MemoryTimings {
    pub slow_dll_ctrl: u32,
    pub fast_dll_ctrl: u32,
    pub m_type: u32,
    pub base_cs: u32,
    pub dll_mode: u32,
}

static mut MEM_TIMINGS: MemoryTimings = MemoryTimings {
    slow_dll_ctrl: 0,
    fast_dll_ctrl: 0,
    m_type: 0,
    base_cs: 0,
    dll_mode: 0,
};
// Initialized from CORE_CLK_SRC_DPLL_X2 by the surrounding OMAP definitions.
static mut CURR_PERF_LEVEL: u32 = 0;

extern "C" {
    static CORE_CLK_SRC_DPLL: u32;
    static CORE_CLK_SRC_DPLL_X2: u32;
    fn sdrc_read_reg(reg: u32) -> u32;
    fn writel_relaxed(value: u32, address: usize);
    fn cpu_is_omap2420() -> bool;
    fn cpu_is_omap2422() -> bool;
    fn omap2_sram_reprogram_sdrc(level: u32, dll_ctrl: u32, m_type: u32);
    fn omap2_sram_ddr_init(
        slow_dll_ctrl: *mut u32,
        fast_dll_ctrl: u32,
        base_cs: u32,
        force_lock_to_unlock_mode: u32,
    );
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
}

// Register and address constants are supplied by the OMAP headers.
extern "C" {
    static SDRC_MR_0: u32;
    static SDRC_DLLA_CTRL: u32;
    static SDRC_DLLA_STATUS: u32;
    static SDRC_DLLB_CTRL: u32;
    static SDRC_DLLB_STATUS: u32;
    static OMAP2420_PRCM_VOLTSETUP: usize;
    static OMAP2430_PRCM_VOLTSETUP: usize;
}

unsafe fn omap2xxx_sdrc_get_slow_dll_ctrl() -> u32 {
    MEM_TIMINGS.slow_dll_ctrl
}

unsafe fn omap2xxx_sdrc_get_fast_dll_ctrl() -> u32 {
    MEM_TIMINGS.fast_dll_ctrl
}

unsafe fn omap2xxx_sdrc_get_type() -> u32 {
    MEM_TIMINGS.m_type
}

/*
 * Check the DLL lock state, and return tue if running in unlock mode.
 * This is needed to compensate for the shifted DLL value in unlock mode.
 */
pub unsafe fn omap2xxx_sdrc_dll_is_unlocked() -> u32 {
    /* dlla and dllb are a set */
    let dll_state = sdrc_read_reg(SDRC_DLLA_CTRL);

    if (dll_state & (1 << 2)) == (1 << 2) { 1 } else { 0 }
}

/* Used by the clock framework during CORE DPLL changes */
pub unsafe fn omap2xxx_sdrc_reprogram(level: u32, force: u32) -> u32 {
    let dll_ctrl;
    let m_type;
    let prev = CURR_PERF_LEVEL;
    let mut flags: usize = 0;

    if (CURR_PERF_LEVEL == level) && force == 0 { return prev; }

    if level == CORE_CLK_SRC_DPLL {
        dll_ctrl = omap2xxx_sdrc_get_slow_dll_ctrl();
    } else if level == CORE_CLK_SRC_DPLL_X2 {
        dll_ctrl = omap2xxx_sdrc_get_fast_dll_ctrl();
    } else {
        return prev;
    }

    m_type = omap2xxx_sdrc_get_type();
    local_irq_save(&mut flags);
    /* XXX These calls should be abstracted out through a prm2xxx.c function */
    if cpu_is_omap2420() {
        writel_relaxed(0xffff, OMAP2420_PRCM_VOLTSETUP);
    } else {
        writel_relaxed(0xffff, OMAP2430_PRCM_VOLTSETUP);
    }
    omap2_sram_reprogram_sdrc(level, dll_ctrl, m_type);
    CURR_PERF_LEVEL = level;
    local_irq_restore(flags);
    prev
}

/* Used by the clock framework during CORE DPLL changes */
pub unsafe fn omap2xxx_sdrc_init_params(force_lock_to_unlock_mode: u32) {
    let dll_cnt;
    let mut fast_dll: u32 = 0;

    /* DDR = 1, SDR = 0 */
    MEM_TIMINGS.m_type = if (sdrc_read_reg(SDRC_MR_0) & 0x3) == 0x1 { 0 } else { 1 };

    /* 2422 es2.05 and beyond has a single SIP DDR instead of 2 like others. */
    if cpu_is_omap2422() { MEM_TIMINGS.base_cs = 1; } else { MEM_TIMINGS.base_cs = 0; }
    if MEM_TIMINGS.m_type != M_DDR { return; }

    if (MEM_TIMINGS.fast_dll_ctrl & (1 << 2)) == M_LOCK_CTRL {
        MEM_TIMINGS.dll_mode = M_UNLOCK;
    } else {
        MEM_TIMINGS.dll_mode = M_LOCK;
    }

    if MEM_TIMINGS.base_cs == 0 {
        fast_dll = sdrc_read_reg(SDRC_DLLA_CTRL);
        dll_cnt = sdrc_read_reg(SDRC_DLLA_STATUS) & 0xff00;
    } else {
        fast_dll = sdrc_read_reg(SDRC_DLLB_CTRL);
        dll_cnt = sdrc_read_reg(SDRC_DLLB_STATUS) & 0xff00;
    }
    if force_lock_to_unlock_mode != 0 {
        fast_dll &= !0xff00;
        fast_dll |= dll_cnt;
    }
    /* set fast timings with DLL filter disabled */
    MEM_TIMINGS.fast_dll_ctrl = fast_dll | (3 << 8);

    /* No disruptions, DDR will be offline & C-ABI not followed */
    omap2_sram_ddr_init(
        &mut MEM_TIMINGS.slow_dll_ctrl,
        MEM_TIMINGS.fast_dll_ctrl,
        MEM_TIMINGS.base_cs,
        force_lock_to_unlock_mode,
    );
    MEM_TIMINGS.slow_dll_ctrl &= 0xff00; /* Keep lock value */
    /* Turn status into unlock ctrl */
    MEM_TIMINGS.slow_dll_ctrl |= (MEM_TIMINGS.fast_dll_ctrl & 0xF) | (1 << 2);
    /* 90 degree phase for anything below 133MHz + disable DLL filter */
    MEM_TIMINGS.slow_dll_ctrl |= (1 << 1) | (3 << 8);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
