// SPDX-License-Identifier: GPL-2.0-only
/*
 * Zynq PLL driver
 *
 *  Copyright (C) 2013 Xilinx
 *
 *  Sören Brinkmann <soren.brinkmann@xilinx.com>
 */

// Dependencies supplied by the surrounding kernel environment.

#[repr(C)]
pub struct zynq_pll {
    pub hw: clk_hw,
    pub pll_ctrl: *mut core::ffi::c_void,
    pub pll_status: *mut core::ffi::c_void,
    pub lock: *mut spinlock_t,
    pub lockbit: u8,
}

// Register bitfield defines
const PLLCTRL_FBDIV_MASK: u32 = 0x7f000;
const PLLCTRL_FBDIV_SHIFT: u32 = 12;
const PLLCTRL_BPQUAL_MASK: u32 = 1 << 3;
const PLLCTRL_PWRDWN_MASK: u32 = 2;
const PLLCTRL_PWRDWN_SHIFT: u32 = 1;
const PLLCTRL_RESET_MASK: u32 = 1;
const PLLCTRL_RESET_SHIFT: u32 = 0;

const PLL_FBDIV_MIN: u32 = 13;
const PLL_FBDIV_MAX: u32 = 66;

unsafe fn zynq_pll_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> i32 {
    let mut fbdiv: u32;

    fbdiv = ((*req).rate / (*req).best_parent_rate) as u32;
    if fbdiv < PLL_FBDIV_MIN {
        fbdiv = PLL_FBDIV_MIN;
    } else if fbdiv > PLL_FBDIV_MAX {
        fbdiv = PLL_FBDIV_MAX;
    }

    (*req).rate = (*req).best_parent_rate * fbdiv as u64;

    0
}

unsafe fn zynq_pll_recalc_rate(hw: *mut clk_hw, parent_rate: u64) -> u64 {
    let clk = hw as *mut zynq_pll;
    let fbdiv: u32;

    /*
     * makes probably sense to redundantly save fbdiv in the struct
     * zynq_pll to save the IO access.
     */
    fbdiv = (readl((*clk).pll_ctrl) & PLLCTRL_FBDIV_MASK) >> PLLCTRL_FBDIV_SHIFT;

    parent_rate * fbdiv as u64
}

unsafe fn zynq_pll_is_enabled(hw: *mut clk_hw) -> i32 {
    let mut flags: c_ulong = 0;
    let reg: u32;
    let clk = hw as *mut zynq_pll;

    spin_lock_irqsave((*clk).lock, &mut flags);
    reg = readl((*clk).pll_ctrl);
    spin_unlock_irqrestore((*clk).lock, flags);

    if reg & (PLLCTRL_RESET_MASK | PLLCTRL_PWRDWN_MASK) == 0 { 1 } else { 0 }
}

unsafe fn zynq_pll_enable(hw: *mut clk_hw) -> i32 {
    let mut flags: c_ulong = 0;
    let mut reg: u32;
    let clk = hw as *mut zynq_pll;

    if zynq_pll_is_enabled(hw) != 0 {
        return 0;
    }

    pr_info!("PLL: enable\n");

    /* Power up PLL and wait for lock */
    spin_lock_irqsave((*clk).lock, &mut flags);
    reg = readl((*clk).pll_ctrl);
    reg &= !(PLLCTRL_RESET_MASK | PLLCTRL_PWRDWN_MASK);
    writel(reg, (*clk).pll_ctrl);
    while readl((*clk).pll_status) & (1u32 << (*clk).lockbit) == 0 {}
    spin_unlock_irqrestore((*clk).lock, flags);

    0
}

unsafe fn zynq_pll_disable(hw: *mut clk_hw) {
    let mut flags: c_ulong = 0;
    let mut reg: u32;
    let clk = hw as *mut zynq_pll;

    if zynq_pll_is_enabled(hw) == 0 {
        return;
    }

    pr_info!("PLL: shutdown\n");

    /* shut down PLL */
    spin_lock_irqsave((*clk).lock, &mut flags);
    reg = readl((*clk).pll_ctrl);
    reg |= PLLCTRL_RESET_MASK | PLLCTRL_PWRDWN_MASK;
    writel(reg, (*clk).pll_ctrl);
    spin_unlock_irqrestore((*clk).lock, flags);
}

static zynq_pll_ops: clk_ops = clk_ops {
    enable: Some(zynq_pll_enable),
    disable: Some(zynq_pll_disable),
    is_enabled: Some(zynq_pll_is_enabled),
    determine_rate: Some(zynq_pll_determine_rate),
    recalc_rate: Some(zynq_pll_recalc_rate),
};

pub unsafe fn clk_register_zynq_pll(
    name: *const c_char,
    parent: *const c_char,
    pll_ctrl: *mut core::ffi::c_void,
    pll_status: *mut core::ffi::c_void,
    lock_index: u8,
    lock: *mut spinlock_t,
) -> *mut clk {
    let mut pll: *mut zynq_pll;
    let mut clk: *mut clk;
    let mut reg: u32;
    let parent_arr: [*const c_char; 1] = [parent];
    let mut flags: c_ulong = 0;
    let initd = clk_init_data {
        name,
        parent_names: parent_arr.as_ptr(),
        ops: &zynq_pll_ops,
        num_parents: 1,
        flags: 0,
    };

    pll = kmalloc_obj::<zynq_pll>();
    if pll.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    /* Populate the struct */
    (*pll).hw.init = &initd;
    (*pll).pll_ctrl = pll_ctrl;
    (*pll).pll_status = pll_status;
    (*pll).lockbit = lock_index;
    (*pll).lock = lock;

    spin_lock_irqsave((*pll).lock, &mut flags);
    reg = readl((*pll).pll_ctrl);
    reg &= !PLLCTRL_BPQUAL_MASK;
    writel(reg, (*pll).pll_ctrl);
    spin_unlock_irqrestore((*pll).lock, flags);

    clk = clk_register(core::ptr::null_mut(), &mut (*pll).hw);
    if WARN_ON(IS_ERR(clk)) {
        kfree(pll);
    }

    clk
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
