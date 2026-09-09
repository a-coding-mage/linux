// SPDX-License-Identifier: GPL-2.0-only
/*
 * Marvell PXA family clocks
 *
 * Copyright (C) 2014 Robert Jarzmik
 *
 * Common clock code for PXA clocks ("CKEN" type clocks + DT)
 */

// Kernel dependencies supplied by the surrounding translation unit.

const KHz: u32 = 1000;
const MHz: u32 = 1000 * 1000;

const MDREFR_K0DB4: u32 = 1 << 29; // SDCLK0 Divide by 4 Control/Status
const MDREFR_K2FREE: u32 = 1 << 25; // SDRAM Free-Running Control
const MDREFR_K1FREE: u32 = 1 << 24; // SDRAM Free-Running Control
const MDREFR_K0FREE: u32 = 1 << 23; // SDRAM Free-Running Control
const MDREFR_SLFRSH: u32 = 1 << 22; // SDRAM Self-Refresh Control/Status
const MDREFR_APD: u32 = 1 << 20; // SDRAM/SSRAM Auto-Power-Down Enable
const MDREFR_K2DB2: u32 = 1 << 19; // SDCLK2 Divide by 2 Control/Status
const MDREFR_K2RUN: u32 = 1 << 18; // SDCLK2 Run Control/Status
const MDREFR_K1DB2: u32 = 1 << 17; // SDCLK1 Divide by 2 Control/Status
const MDREFR_K1RUN: u32 = 1 << 16; // SDCLK1 Run Control/Status
const MDREFR_E1PIN: u32 = 1 << 15; // SDCKE1 Level Control/Status
const MDREFR_K0DB2: u32 = 1 << 14; // SDCLK0 Divide by 2 Control/Status
const MDREFR_K0RUN: u32 = 1 << 13; // SDCLK0 Run Control/Status
const MDREFR_E0PIN: u32 = 1 << 12; // SDCKE0 Level Control/Status
const MDREFR_DB2_MASK: u32 = MDREFR_K2DB2 | MDREFR_K1DB2;
const MDREFR_DRI_MASK: u32 = 0xFFF;

static mut pxa_clk_lock: spinlock_t = DEFINE_SPINLOCK!();
static mut pxa_clocks: [*mut clk; CLK_MAX as usize] = [core::ptr::null_mut(); CLK_MAX as usize];
static mut onecell_data: clk_onecell_data = clk_onecell_data {
    clks: pxa_clocks.as_mut_ptr(),
    clk_num: CLK_MAX,
};

#[repr(C)]
struct pxa_clk {
    hw: clk_hw,
    lp: clk_fixed_factor,
    hp: clk_fixed_factor,
    gate: clk_gate,
    is_in_low_power: Option<unsafe extern "C" fn() -> bool>,
}

unsafe fn to_pxa_clk(hw: *mut clk_hw) -> *mut pxa_clk {
    container_of!(hw, pxa_clk, hw)
}

unsafe extern "C" fn cken_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    let pclk = to_pxa_clk(hw);
    let fix: *mut clk_fixed_factor;
    if (*pclk).is_in_low_power.is_none()
        || ((*pclk).is_in_low_power.unwrap())()
    {
        fix = &mut (*pclk).lp;
    } else {
        fix = &mut (*pclk).hp;
    }
    __clk_hw_set_clk(&mut (*fix).hw, hw);
    clk_fixed_factor_ops.recalc_rate.unwrap()(&mut (*fix).hw, parent_rate)
}

static cken_rate_ops: clk_ops = clk_ops {
    recalc_rate: Some(cken_recalc_rate),
};

unsafe extern "C" fn cken_get_parent(hw: *mut clk_hw) -> u8 {
    let pclk = to_pxa_clk(hw);
    if (*pclk).is_in_low_power.is_none() {
        return 0;
    }
    if ((*pclk).is_in_low_power.unwrap())() { 0 } else { 1 }
}

static cken_mux_ops: clk_ops = clk_ops {
    determine_rate: Some(clk_hw_determine_rate_no_reparent),
    get_parent: Some(cken_get_parent),
    set_parent: Some(dummy_clk_set_parent),
};

pub unsafe extern "C" fn clkdev_pxa_register(
    ckid: c_int, con_id: *const c_char, dev_id: *const c_char, clk: *mut clk,
) {
    if !IS_ERR!(clk) && ckid != CLK_NONE { pxa_clocks[ckid as usize] = clk; }
    if !IS_ERR!(clk) { clk_register_clkdev(clk, con_id, dev_id); }
}

pub unsafe extern "C" fn clk_pxa_cken_init(
    clks: *const desc_clk_cken, nb_clks: c_int, clk_regs: *mut c_void,
) -> c_int {
    for i in 0..nb_clks {
        let pxa_clk = kzalloc_obj!(pxa_clk);
        if pxa_clk.is_null() { return -ENOMEM; }
        (*pxa_clk).is_in_low_power = (*clks.add(i as usize)).is_in_low_power;
        (*pxa_clk).lp = (*clks.add(i as usize)).lp;
        (*pxa_clk).hp = (*clks.add(i as usize)).hp;
        (*pxa_clk).gate = (*clks.add(i as usize)).gate;
        (*pxa_clk).gate.reg = (clk_regs as *mut u8).add((*clks.add(i as usize)).cken_reg as usize) as *mut c_void;
        (*pxa_clk).gate.lock = &raw mut pxa_clk_lock;
        let c = clk_register_composite(core::ptr::null_mut(), (*clks.add(i as usize)).name,
            (*clks.add(i as usize)).parent_names, 2, &mut (*pxa_clk).hw, &cken_mux_ops,
            &mut (*pxa_clk).hw, &cken_rate_ops, &mut (*pxa_clk).gate.hw, &clk_gate_ops,
            (*clks.add(i as usize)).flags);
        clkdev_pxa_register((*clks.add(i as usize)).ckid, (*clks.add(i as usize)).con_id,
            (*clks.add(i as usize)).dev_id, c);
    }
    0
}

pub unsafe extern "C" fn clk_pxa_dt_common_init(np: *mut device_node) {
    of_clk_add_provider(np, of_clk_src_onecell_get, &raw mut onecell_data);
}

pub unsafe extern "C" fn pxa2xx_core_turbo_switch(on: bool) {
    let mut flags: c_ulong = 0;
    let mut clkcfg: u32;
    let mut unused: u32;
    local_irq_save(&mut flags);
    // ARM inline assembly from the C source: read CLKCFG, update TURBO/FCS,
    // then write it back through the required aligned branch sequence.
    asm!("mrc p14, 0, {0}, c6, c0, 0", out(reg) clkcfg);
    clkcfg &= !CLKCFG_TURBO & !CLKCFG_HALFTURBO;
    if on { clkcfg |= CLKCFG_TURBO; }
    clkcfg |= CLKCFG_FCS;
    asm!("b 2f; .align 5; 1: mcr p14, 0, {1}, c6, c0, 0; b 3f; 2: b 1b; 3: nop",
        out(reg) unused, in(reg) clkcfg);
    local_irq_restore(flags);
}

pub unsafe extern "C" fn pxa2xx_cpll_change(
    freq: *mut pxa2xx_freq, mdrefr_dri: Option<unsafe extern "C" fn(c_uint) -> u32>,
    cccr: *mut c_void,
) {
    let clkcfg = (*freq).clkcfg;
    let mut flags: c_ulong = 0;
    let mdrefr = pxa_smemc_get_mdrefr();
    local_irq_save(&mut flags);
    let mut preset_mdrefr = readl(mdrefr);
    let mut postset_mdrefr = preset_mdrefr;
    let dri = mdrefr_dri.unwrap()((*freq).membus_khz);
    if (preset_mdrefr & MDREFR_DRI_MASK) > dri {
        preset_mdrefr = (preset_mdrefr & !MDREFR_DRI_MASK) | dri;
    }
    postset_mdrefr = (postset_mdrefr & !MDREFR_DRI_MASK) | dri;
    if (*freq).div2 { preset_mdrefr |= MDREFR_DB2_MASK; postset_mdrefr |= MDREFR_DB2_MASK; }
    else { postset_mdrefr &= !MDREFR_DB2_MASK; }
    writel((*freq).cccr, cccr);
    // ARM inline assembly performs the preset/write CLKCFG/postset sequence.
    asm!("ldr r4, [{0}]; b 2f; .align 5; 1: str {2}, [{0}]; mcr p14, 0, {1}, c6, c0, 0; str {3}, [{0}]; b 3f; 2: b 1b; 3: nop",
        in(reg) mdrefr, in(reg) clkcfg, in(reg) preset_mdrefr, in(reg) postset_mdrefr,
        out("r4") _, options(nostack));
    local_irq_restore(flags);
}

pub unsafe extern "C" fn pxa2xx_determine_rate(
    req: *mut clk_rate_request, freqs: *mut pxa2xx_freq, nb_freqs: c_int,
) -> c_int {
    let mut closest_below = -1;
    let mut closest_above = -1;
    let mut i = 0;
    while i < nb_freqs {
        let rate = (*freqs.add(i as usize)).cpll;
        if rate == (*req).rate { break; }
        if rate < (*req).min_rate || rate > (*req).max_rate { i += 1; continue; }
        if rate <= (*req).rate { closest_below = i; }
        if rate >= (*req).rate && closest_above == -1 { closest_above = i; }
        i += 1;
    }
    (*req).best_parent_hw = core::ptr::null_mut();
    let rate;
    if i < nb_freqs { rate = (*req).rate; }
    else if closest_below >= 0 { rate = (*freqs.add(closest_below as usize)).cpll; }
    else if closest_above >= 0 { rate = (*freqs.add(closest_above as usize)).cpll; }
    else { pr_debug!("%s(rate=%lu) no match\n", "pxa2xx_determine_rate", (*req).rate); return -EINVAL; }
    pr_debug!("%s(rate=%lu) rate=%lu\n", "pxa2xx_determine_rate", (*req).rate, rate);
    (*req).rate = rate;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
