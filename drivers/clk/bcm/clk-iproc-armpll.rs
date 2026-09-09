// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2014 Broadcom Corporation

// Linux dependencies supplied by the surrounding translation unit.

const IPROC_CLK_MAX_FREQ_POLICY: u32 = 0x3;
const IPROC_CLK_POLICY_FREQ_OFFSET: usize = 0x008;
const IPROC_CLK_POLICY_FREQ_POLICY_FREQ_SHIFT: u32 = 8;
const IPROC_CLK_POLICY_FREQ_POLICY_FREQ_MASK: u32 = 0x7;

const IPROC_CLK_PLLARMA_OFFSET: usize = 0xc00;
const IPROC_CLK_PLLARMA_LOCK_SHIFT: u32 = 28;
const IPROC_CLK_PLLARMA_PDIV_SHIFT: u32 = 24;
const IPROC_CLK_PLLARMA_PDIV_MASK: u32 = 0xf;
const IPROC_CLK_PLLARMA_NDIV_INT_SHIFT: u32 = 8;
const IPROC_CLK_PLLARMA_NDIV_INT_MASK: u32 = 0x3ff;

const IPROC_CLK_PLLARMB_OFFSET: usize = 0xc04;
const IPROC_CLK_PLLARMB_NDIV_FRAC_MASK: u32 = 0xfffff;

const IPROC_CLK_PLLARMC_OFFSET: usize = 0xc08;
const IPROC_CLK_PLLARMC_BYPCLK_EN_SHIFT: u32 = 8;
const IPROC_CLK_PLLARMC_MDIV_MASK: u32 = 0xff;

const IPROC_CLK_PLLARMCTL5_OFFSET: usize = 0xc20;
const IPROC_CLK_PLLARMCTL5_H_MDIV_MASK: u32 = 0xff;

const IPROC_CLK_PLLARM_OFFSET_OFFSET: usize = 0xc24;
const IPROC_CLK_PLLARM_SW_CTL_SHIFT: u32 = 29;
const IPROC_CLK_PLLARM_NDIV_INT_OFFSET_SHIFT: u32 = 20;
const IPROC_CLK_PLLARM_NDIV_INT_OFFSET_MASK: u32 = 0xff;
const IPROC_CLK_PLLARM_NDIV_FRAC_OFFSET_MASK: u32 = 0xfffff;

const IPROC_CLK_ARM_DIV_OFFSET: usize = 0xe00;
const IPROC_CLK_ARM_DIV_PLL_SELECT_OVERRIDE_SHIFT: u32 = 4;
const IPROC_CLK_ARM_DIV_ARM_PLL_SELECT_MASK: u32 = 0xf;

const IPROC_CLK_POLICY_DBG_OFFSET: usize = 0xec0;
const IPROC_CLK_POLICY_DBG_ACT_FREQ_SHIFT: u32 = 12;
const IPROC_CLK_POLICY_DBG_ACT_FREQ_MASK: u32 = 0x7;

#[repr(u32)]
enum iproc_arm_pll_fid {
    ARM_PLL_FID_CRYSTAL_CLK = 0,
    ARM_PLL_FID_SYS_CLK = 2,
    ARM_PLL_FID_CH0_SLOW_CLK = 6,
    ARM_PLL_FID_CH1_FAST_CLK = 7,
}

#[repr(C)]
struct iproc_arm_pll {
    hw: clk_hw,
    base: *mut core::ffi::c_void,
    rate: libc::c_ulong,
}

unsafe fn to_iproc_arm_pll(hw: *mut clk_hw) -> *mut iproc_arm_pll {
    (hw as *mut u8).sub(core::mem::offset_of!(iproc_arm_pll, hw)) as *mut iproc_arm_pll
}

unsafe fn __get_fid(pll: *mut iproc_arm_pll) -> u32 {
    let mut val: u32;
    let mut policy: u32;
    let mut fid: u32;
    let mut active_fid: u32;

    val = readl((*pll).base.add(IPROC_CLK_ARM_DIV_OFFSET));
    if val & (1u32 << IPROC_CLK_ARM_DIV_PLL_SELECT_OVERRIDE_SHIFT) != 0 {
        policy = val & IPROC_CLK_ARM_DIV_ARM_PLL_SELECT_MASK;
    } else {
        policy = 0;
    }

    BUG_ON(policy > IPROC_CLK_MAX_FREQ_POLICY);

    val = readl((*pll).base.add(IPROC_CLK_POLICY_FREQ_OFFSET));
    fid = (val >> (IPROC_CLK_POLICY_FREQ_POLICY_FREQ_SHIFT * policy))
        & IPROC_CLK_POLICY_FREQ_POLICY_FREQ_MASK;

    val = readl((*pll).base.add(IPROC_CLK_POLICY_DBG_OFFSET));
    active_fid = IPROC_CLK_POLICY_DBG_ACT_FREQ_MASK
        & (val >> IPROC_CLK_POLICY_DBG_ACT_FREQ_SHIFT);
    if fid != active_fid {
        pr_debug!("%s: fid override %u->%u\n", "__get_fid", fid, active_fid);
        fid = active_fid;
    }

    pr_debug!("%s: active fid: %u\n", "__get_fid", fid);
    fid
}

unsafe fn __get_mdiv(pll: *mut iproc_arm_pll) -> i32 {
    let fid = __get_fid(pll);
    let mut mdiv: i32;
    let mut val: u32;

    match fid {
        x if x == iproc_arm_pll_fid::ARM_PLL_FID_CRYSTAL_CLK as u32
            || x == iproc_arm_pll_fid::ARM_PLL_FID_SYS_CLK as u32 => mdiv = 1,
        x if x == iproc_arm_pll_fid::ARM_PLL_FID_CH0_SLOW_CLK as u32 => {
            val = readl((*pll).base.add(IPROC_CLK_PLLARMC_OFFSET));
            mdiv = (val & IPROC_CLK_PLLARMC_MDIV_MASK) as i32;
            if mdiv == 0 { mdiv = 256; }
        }
        x if x == iproc_arm_pll_fid::ARM_PLL_FID_CH1_FAST_CLK as u32 => {
            val = readl((*pll).base.add(IPROC_CLK_PLLARMCTL5_OFFSET));
            mdiv = (val & IPROC_CLK_PLLARMCTL5_H_MDIV_MASK) as i32;
            if mdiv == 0 { mdiv = 256; }
        }
        _ => mdiv = -EFAULT,
    }
    mdiv
}

unsafe fn __get_ndiv(pll: *mut iproc_arm_pll) -> u32 {
    let mut val = readl((*pll).base.add(IPROC_CLK_PLLARM_OFFSET_OFFSET));
    let (ndiv_int, ndiv_frac): (u32, u32);
    if val & (1u32 << IPROC_CLK_PLLARM_SW_CTL_SHIFT) != 0 {
        let mut n = (val >> IPROC_CLK_PLLARM_NDIV_INT_OFFSET_SHIFT)
            & IPROC_CLK_PLLARM_NDIV_INT_OFFSET_MASK;
        if n == 0 { n = 256; }
        ndiv_int = n;
        ndiv_frac = val & IPROC_CLK_PLLARM_NDIV_FRAC_OFFSET_MASK;
    } else {
        val = readl((*pll).base.add(IPROC_CLK_PLLARMA_OFFSET));
        let mut n = (val >> IPROC_CLK_PLLARMA_NDIV_INT_SHIFT)
            & IPROC_CLK_PLLARMA_NDIV_INT_MASK;
        if n == 0 { n = 1024; }
        ndiv_int = n;
        val = readl((*pll).base.add(IPROC_CLK_PLLARMB_OFFSET));
        ndiv_frac = val & IPROC_CLK_PLLARMB_NDIV_FRAC_MASK;
    }
    (ndiv_int << 20) | ndiv_frac
}

unsafe extern "C" fn iproc_arm_pll_recalc_rate(
    hw: *mut clk_hw,
    parent_rate: libc::c_ulong,
) -> libc::c_ulong {
    let pll = to_iproc_arm_pll(hw);
    let mut val = readl((*pll).base.add(IPROC_CLK_PLLARMC_OFFSET));
    if val & (1u32 << IPROC_CLK_PLLARMC_BYPCLK_EN_SHIFT) != 0 {
        (*pll).rate = parent_rate;
        return (*pll).rate;
    }
    val = readl((*pll).base.add(IPROC_CLK_PLLARMA_OFFSET));
    if val & (1u32 << IPROC_CLK_PLLARMA_LOCK_SHIFT) == 0 {
        (*pll).rate = 0;
        return 0;
    }
    let mut pdiv = (val >> IPROC_CLK_PLLARMA_PDIV_SHIFT) & IPROC_CLK_PLLARMA_PDIV_MASK;
    if pdiv == 0 { pdiv = 16; }
    let ndiv = __get_ndiv(pll) as u64;
    let mdiv = __get_mdiv(pll);
    if mdiv <= 0 {
        (*pll).rate = 0;
        return 0;
    }
    (*pll).rate = (((ndiv * parent_rate as u64) >> 20) / pdiv as u64 / mdiv as u64) as libc::c_ulong;
    pr_debug!("%s: ARM PLL rate: %lu. parent rate: %lu\n", "iproc_arm_pll_recalc_rate", (*pll).rate, parent_rate);
    pr_debug!("%s: ndiv_int: %u, pdiv: %u, mdiv: %d\n", "iproc_arm_pll_recalc_rate", (ndiv >> 20) as u32, pdiv, mdiv);
    (*pll).rate
}

static iproc_arm_pll_ops: clk_ops = clk_ops {
    recalc_rate: Some(iproc_arm_pll_recalc_rate),
};

unsafe extern "C" fn iproc_armpll_setup(node: *mut device_node) {
    let mut ret: i32;
    let pll = kzalloc_obj::<iproc_arm_pll>();
    if WARN_ON(pll.is_null()) { return; }
    (*pll).base = of_iomap(node, 0);
    if WARN_ON((*pll).base.is_null()) { goto_err_free_pll(pll); return; }
    let parent_name = of_clk_get_parent_name(node, 0);
    let init = clk_init_data {
        name: (*node).name,
        ops: &iproc_arm_pll_ops,
        flags: 0,
        parent_names: if !parent_name.is_null() { &parent_name } else { core::ptr::null() },
        num_parents: if !parent_name.is_null() { 1 } else { 0 },
    };
    (*pll).hw.init = &init;
    ret = clk_hw_register(core::ptr::null_mut(), &mut (*pll).hw);
    if WARN_ON(ret != 0) { iounmap((*pll).base); kfree(pll); return; }
    ret = of_clk_add_hw_provider(node, of_clk_hw_simple_get, &mut (*pll).hw);
    if WARN_ON(ret != 0) { clk_hw_unregister(&mut (*pll).hw); iounmap((*pll).base); kfree(pll); }
}

unsafe fn goto_err_free_pll(pll: *mut iproc_arm_pll) { kfree(pll); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
