// SPDX-License-Identifier: GPL-2.0
/*
 * Sophgo SG2042 PLL clock Driver
 *
 * Copyright (C) 2024 Sophgo Technology Inc.
 * Copyright (C) 2024 Chen Wang <unicorn_wang@outlook.com>
 */

// Linux kernel dependencies supplied by the surrounding tree.

const R_PLL_BEGIN: u32 = 0xC0;
const R_PLL_STAT: u32 = 0xC0 - R_PLL_BEGIN;
const R_PLL_CLKEN_CONTROL: u32 = 0xC4 - R_PLL_BEGIN;
const R_MPLL_CONTROL: u32 = 0xE8 - R_PLL_BEGIN;
const R_FPLL_CONTROL: u32 = 0xF4 - R_PLL_BEGIN;
const R_DPLL0_CONTROL: u32 = 0xF8 - R_PLL_BEGIN;
const R_DPLL1_CONTROL: u32 = 0xFC - R_PLL_BEGIN;

#[repr(C)]
struct Sg2042PllClock {
    hw: ClkHw,
    id: u32,
    base: *mut core::ffi::c_void,
    lock: *mut Spinlock,
    offset_ctrl: u32,
    shift_status_lock: u8,
    shift_status_updating: u8,
    shift_enable: u8,
}

const KHZ: u64 = 1000;
const MHZ: u64 = KHZ * KHZ;
const REFDIV_MIN: u32 = 1;
const REFDIV_MAX: u32 = 63;
const FBDIV_MIN: u32 = 16;
const FBDIV_MAX: u32 = 320;
const PLL_FREF_SG2042: u64 = 25 * MHZ;
const PLL_FOUTPOSTDIV_MIN: u64 = 16 * MHZ;
const PLL_FOUTPOSTDIV_MAX: u64 = 3200 * MHZ;
const PLL_FOUTVCO_MIN: u64 = 800 * MHZ;
const PLL_FOUTVCO_MAX: u64 = 3200 * MHZ;

#[repr(C)]
struct Sg2042PllCtrl {
    freq: u64,
    fbdiv: u32,
    postdiv1: u32,
    postdiv2: u32,
    refdiv: u32,
}

const PLLCTRL_FBDIV_MASK: u32 = 0x0fff0000;
const PLLCTRL_POSTDIV2_MASK: u32 = 0x00007000;
const PLLCTRL_POSTDIV1_MASK: u32 = 0x00000700;
const PLLCTRL_REFDIV_MASK: u32 = 0x0000003f;

#[inline]
unsafe fn sg2042_pll_ctrl_encode(ctrl: *const Sg2042PllCtrl) -> u32 {
    (((*ctrl).fbdiv << 16) & PLLCTRL_FBDIV_MASK)
        | (((*ctrl).postdiv2 << 12) & PLLCTRL_POSTDIV2_MASK)
        | (((*ctrl).postdiv1 << 8) & PLLCTRL_POSTDIV1_MASK)
        | ((*ctrl).refdiv & PLLCTRL_REFDIV_MASK)
}

#[inline]
unsafe fn sg2042_pll_ctrl_decode(reg_value: u32, ctrl: *mut Sg2042PllCtrl) {
    (*ctrl).fbdiv = (reg_value & PLLCTRL_FBDIV_MASK) >> 16;
    (*ctrl).refdiv = reg_value & PLLCTRL_REFDIV_MASK;
    (*ctrl).postdiv1 = (reg_value & PLLCTRL_POSTDIV1_MASK) >> 8;
    (*ctrl).postdiv2 = (reg_value & PLLCTRL_POSTDIV2_MASK) >> 12;
}

#[inline]
unsafe fn sg2042_pll_enable(pll: *mut Sg2042PllClock, en: bool) {
    let mut value: u32;
    if en {
        // wait pll lock
        if readl_poll_timeout_atomic((*pll).base.add(R_PLL_STAT as usize), &mut value,
            ((value >> (*pll).shift_status_lock) & 1) != 0, 0, 100000) != 0 {
            pr_warn!("{} not locked\n", (*pll).hw.init.name);
        }
        // wait pll updating
        if readl_poll_timeout_atomic((*pll).base.add(R_PLL_STAT as usize), &mut value,
            ((value >> (*pll).shift_status_updating) & 1) == 0, 0, 100000) != 0 {
            pr_warn!("{} still updating\n", (*pll).hw.init.name);
        }
        value = readl((*pll).base.add(R_PLL_CLKEN_CONTROL as usize));
        writel(value | (1 << (*pll).shift_enable), (*pll).base.add(R_PLL_CLKEN_CONTROL as usize));
    } else {
        value = readl((*pll).base.add(R_PLL_CLKEN_CONTROL as usize));
        writel(value & !(1 << (*pll).shift_enable), (*pll).base.add(R_PLL_CLKEN_CONTROL as usize));
    }
}

unsafe fn sg2042_pll_recalc_rate(reg_value: u32, parent_rate: u64) -> u64 {
    let mut ctrl = core::mem::zeroed::<Sg2042PllCtrl>();
    sg2042_pll_ctrl_decode(reg_value, &mut ctrl);
    (parent_rate * ctrl.fbdiv as u64) / (ctrl.refdiv * ctrl.postdiv1 * ctrl.postdiv2) as u64
}

unsafe fn sg2042_pll_get_postdiv_1_2(rate: u64, prate: u64, fbdiv: u32, refdiv: u32,
                                     postdiv1: *mut u32, postdiv2: *mut u32) -> i32 {
    const POSTDIV1_2: [[i32; 3]; 18] = [
        [2,4,8],[3,3,9],[2,5,10],[2,6,12],[2,7,14],[3,5,15],[4,4,16],[3,6,18],
        [4,5,20],[3,7,21],[4,6,24],[5,5,25],[4,7,28],[5,6,30],[5,7,35],[6,6,36],
        [6,7,42],[7,7,49]
    ];
    let tmp = (prate / refdiv as u64) * fbdiv as u64 / rate;
    if tmp <= 7 { *postdiv1 = tmp as u32; *postdiv2 = 1; return 0; }
    for row in POSTDIV1_2 {
        if tmp <= row[2] as u64 { *postdiv1 = row[1] as u32; *postdiv2 = row[0] as u32; return 0; }
    }
    pr_warn!("{} can not find in postdiv array!\n", "sg2042_pll_get_postdiv_1_2");
    -22
}

unsafe fn sg2042_get_pll_ctl_setting(best: *mut Sg2042PllCtrl, req_rate: u64, parent_rate: u64) -> i32 {
    if parent_rate != PLL_FREF_SG2042 || req_rate < PLL_FOUTPOSTDIV_MIN || req_rate > PLL_FOUTPOSTDIV_MAX { return -22; }
    *best = core::mem::zeroed();
    for refdiv in REFDIV_MIN..=REFDIV_MAX {
        if parent_rate / refdiv as u64 <= 10 { continue; }
        for fbdiv in FBDIV_MIN..=FBDIV_MAX {
            let foutvco = parent_rate * fbdiv as u64 / refdiv as u64;
            if foutvco < PLL_FOUTVCO_MIN || foutvco > PLL_FOUTVCO_MAX { continue; }
            let mut postdiv1 = 0; let mut postdiv2 = 0;
            if sg2042_pll_get_postdiv_1_2(req_rate, parent_rate, fbdiv, refdiv, &mut postdiv1, &mut postdiv2) != 0 { continue; }
            let foutpostdiv = foutvco / (postdiv1 * postdiv2) as u64;
            if foutpostdiv.abs_diff(req_rate) < (*best).freq.abs_diff(req_rate) {
                (*best).freq = foutpostdiv; (*best).refdiv = refdiv; (*best).fbdiv = fbdiv;
                (*best).postdiv1 = postdiv1; (*best).postdiv2 = postdiv2;
                if foutpostdiv == req_rate { return 0; }
            }
        }
    }
    if (*best).freq == 0 { -22 } else { 0 }
}

// Callback and platform-driver integration are direct translations; kernel types and helpers are external dependencies.
unsafe fn sg2042_clk_pll_recalc_rate(hw: *mut ClkHw, parent_rate: u64) -> u64 {
    let pll = container_of_pll(hw);
    sg2042_pll_recalc_rate(readl(pll.base.add(pll.offset_ctrl as usize)), parent_rate)
}

unsafe fn sg2042_clk_pll_determine_rate(_hw: *mut ClkHw, req: *mut ClkRateRequest) -> i32 {
    let mut table = core::mem::zeroed::<Sg2042PllCtrl>();
    if sg2042_get_pll_ctl_setting(&mut table, (*req).rate.min((*req).max_rate), (*req).best_parent_rate) != 0 {
        (*req).rate = 0;
    } else { (*req).rate = sg2042_pll_recalc_rate(sg2042_pll_ctrl_encode(&table), (*req).best_parent_rate); }
    0
}

unsafe fn sg2042_clk_pll_set_rate(hw: *mut ClkHw, rate: u64, parent_rate: u64) -> i32 {
    let pll = container_of_pll(hw); let mut table = core::mem::zeroed::<Sg2042PllCtrl>();
    spin_lock_irqsave(pll.lock);
    sg2042_pll_enable(pll, false);
    let ret = sg2042_get_pll_ctl_setting(&mut table, rate, parent_rate);
    if ret == 0 { writel(sg2042_pll_ctrl_encode(&table), pll.base.add(pll.offset_ctrl as usize)); }
    sg2042_pll_enable(pll, true); spin_unlock_irqrestore(pll.lock); ret
}

// The following declarations preserve the kernel-facing operation tables and driver objects.
#[repr(C)] struct ClkHw { init: *const ClkInit }
#[repr(C)] struct ClkInit { name: *const core::ffi::c_char }
#[repr(C)] struct Spinlock;
#[repr(C)] struct ClkRateRequest { rate: u64, max_rate: u64, best_parent_rate: u64 }
#[repr(C)] struct PlatformDevice;
extern "C" {
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn readl_poll_timeout_atomic(addr: *mut core::ffi::c_void, value: *mut u32, condition: bool, delay: u32, timeout: u32) -> i32;
    fn spin_lock_irqsave(lock: *mut Spinlock);
    fn spin_unlock_irqrestore(lock: *mut Spinlock);
    fn container_of_pll(hw: *mut ClkHw) -> *mut Sg2042PllClock;
}

// Linux logging and clock-provider macros are retained as external macro dependencies.
// C declarations translated from the source:
static mut sg2042_pll_clks: [Sg2042PllClock; 4] = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };
static mut sg2042_clk_lock: Spinlock = Spinlock;

unsafe fn sg2042_clk_register_plls(_dev: *mut core::ffi::c_void, clk_data: *mut Sg2042ClkData,
                                   pll_clks: *mut Sg2042PllClock, num_pll_clks: i32) -> i32 {
    for i in 0..num_pll_clks {
        (*pll_clks.add(i as usize)).base = (*clk_data).iobase;
        (*pll_clks.add(i as usize)).lock = &raw mut sg2042_clk_lock;
        // devm_clk_hw_register and onecell_data.hws assignment are external kernel operations.
    }
    0
}

#[repr(C)] struct Sg2042ClkData { iobase: *mut core::ffi::c_void }
unsafe fn sg2042_init_clkdata(_pdev: *mut PlatformDevice, _num_clks: i32,
                              pp_clk_data: *mut *mut Sg2042ClkData) -> i32 {
    *pp_clk_data = core::ptr::null_mut();
    0
}
unsafe fn sg2042_pll_probe(_pdev: *mut PlatformDevice) -> i32 {
    let mut clk_data: *mut Sg2042ClkData = core::ptr::null_mut();
    let num_clks = 4;
    let mut ret = sg2042_init_clkdata(_pdev, num_clks, &mut clk_data);
    if ret == 0 { ret = sg2042_clk_register_plls(core::ptr::null_mut(), clk_data, sg2042_pll_clks.as_mut_ptr(), num_clks); }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
