// SPDX-License-Identifier: GPL-2.0
/* Clock driver for TI Davinci PSC controllers */

// C dependencies supplied by the surrounding kernel translation.

const EPCPR: u32 = 0x070;
const PTCMD: u32 = 0x120;
const PTSTAT: u32 = 0x128;
const PDSTAT: u32 = 0x200; // offset base: 0x200 + 4 * n
const PDCTL: u32 = 0x300;
const MDSTAT: u32 = 0x800;
const MDCTL: u32 = 0xa00;

#[repr(u32)]
enum DavinciLpscState {
    LpscStateSwrstdisable = 0,
    LpscStateSyncrst = 1,
    LpscStateDisable = 2,
    LpscStateEnable = 3,
}

const MDSTAT_STATE_MASK: u32 = 0x3f;
const MDSTAT_MCKOUT: u32 = 1 << 12;
const PDSTAT_STATE_MASK: u32 = 0x1f;
const MDCTL_FORCE: u32 = 1 << 31;
const MDCTL_LRESET: u32 = 1 << 8;
const PDCTL_EPCGOOD: u32 = 1 << 8;
const PDCTL_NEXT: u32 = 1;

#[repr(C)]
struct DavinciPscData {
    clk_data: ClkOnecellData,
    pm_data: GenpdOnecellData,
    rcdev: ResetControllerDev,
}

#[repr(C)]
struct DavinciLpscClk {
    dev: *mut Device,
    hw: ClkHw,
    pm_domain: GenericPmDomain,
    genpd_clk: *mut Clk,
    regmap: *mut Regmap,
    md: u32,
    pd: u32,
    flags: u32,
}

#[inline]
unsafe fn best_dev_name(dev: *mut Device) -> *const i8 {
    let mut compatible: *const i8 = core::ptr::null();
    if of_property_read_string((*dev).of_node, b"compatible\0".as_ptr() as *const i8, &mut compatible) == 0 {
        compatible
    } else {
        dev_name(dev)
    }
}

unsafe fn davinci_lpsc_config(lpsc: *mut DavinciLpscClk, next_state: DavinciLpscState) {
    let mut epcpr = 0u32;
    let mut pdstat = 0u32;
    let mut mdstat = 0u32;
    let mut ptstat = 0u32;
    regmap_write_bits((*lpsc).regmap, MDCTL.wrapping_add(4 * (*lpsc).md), MDSTAT_STATE_MASK, next_state as u32);
    if (*lpsc).flags & LPSC_FORCE != 0 {
        regmap_write_bits((*lpsc).regmap, MDCTL.wrapping_add(4 * (*lpsc).md), MDCTL_FORCE, MDCTL_FORCE);
    }
    regmap_read((*lpsc).regmap, PDSTAT.wrapping_add(4 * (*lpsc).pd), &mut pdstat);
    if pdstat & PDSTAT_STATE_MASK == 0 {
        regmap_write_bits((*lpsc).regmap, PDCTL.wrapping_add(4 * (*lpsc).pd), PDCTL_NEXT, PDCTL_NEXT);
        regmap_write((*lpsc).regmap, PTCMD, 1u32 << (*lpsc).pd);
        regmap_read_poll_timeout((*lpsc).regmap, EPCPR, &mut epcpr, epcpr & (1u32 << (*lpsc).pd) != 0, 0, 0);
        regmap_write_bits((*lpsc).regmap, PDCTL.wrapping_add(4 * (*lpsc).pd), PDCTL_EPCGOOD, PDCTL_EPCGOOD);
    } else { regmap_write((*lpsc).regmap, PTCMD, 1u32 << (*lpsc).pd); }
    regmap_read_poll_timeout((*lpsc).regmap, PTSTAT, &mut ptstat, ptstat & (1u32 << (*lpsc).pd) == 0, 0, 0);
    regmap_read_poll_timeout((*lpsc).regmap, MDSTAT.wrapping_add(4 * (*lpsc).md), &mut mdstat, mdstat & MDSTAT_STATE_MASK == next_state as u32, 0, 0);
}

unsafe fn davinci_lpsc_clk_enable(hw: *mut ClkHw) -> i32 {
    davinci_lpsc_config(hw as *mut DavinciLpscClk, DavinciLpscState::LpscStateEnable); 0
}
unsafe fn davinci_lpsc_clk_disable(hw: *mut ClkHw) { davinci_lpsc_config(hw as *mut DavinciLpscClk, DavinciLpscState::LpscStateDisable); }
unsafe fn davinci_lpsc_clk_is_enabled(hw: *mut ClkHw) -> i32 {
    let lpsc = hw as *mut DavinciLpscClk; let mut mdstat = 0; regmap_read((*lpsc).regmap, MDSTAT + 4 * (*lpsc).md, &mut mdstat); if mdstat & MDSTAT_MCKOUT != 0 { 1 } else { 0 }
}

#[repr(C)]
struct ClkOps { enable: Option<unsafe fn(*mut ClkHw) -> i32>, disable: Option<unsafe fn(*mut ClkHw)>, is_enabled: Option<unsafe fn(*mut ClkHw) -> i32> }
static DAVINCI_LPSC_CLK_OPS: ClkOps = ClkOps { enable: Some(davinci_lpsc_clk_enable), disable: Some(davinci_lpsc_clk_disable), is_enabled: Some(davinci_lpsc_clk_is_enabled) };

unsafe fn davinci_lpsc_clk_reset(clk: *mut Clk, reset: bool) -> i32 {
    let hw = __clk_get_hw(clk); let lpsc = hw as *mut DavinciLpscClk; if lpsc.is_null() { return -22; }
    let mdctl = if reset { 0 } else { MDCTL_LRESET }; regmap_write_bits((*lpsc).regmap, MDCTL + 4 * (*lpsc).md, MDCTL_LRESET, mdctl); 0
}

unsafe fn davinci_psc_reset_assert(rcdev: *mut ResetControllerDev, id: usize) -> i32 { let psc = rcdev as *mut DavinciPscData; davinci_lpsc_clk_reset(*((*psc).clk_data.clks.add(id)), true) }
unsafe fn davinci_psc_reset_deassert(rcdev: *mut ResetControllerDev, id: usize) -> i32 { let psc = rcdev as *mut DavinciPscData; davinci_lpsc_clk_reset(*((*psc).clk_data.clks.add(id)), false) }

unsafe fn davinci_psc_reset_of_xlate(_rcdev: *mut ResetControllerDev, reset_spec: *const OfPhandleArgs) -> i32 {
    let clkspec = *reset_spec; let clk = of_clk_get_from_provider(&clkspec); if is_err(clk) { return ptr_err(clk); }
    let hw = __clk_get_hw(clk); let lpsc = hw as *mut DavinciLpscClk; clk_put(clk); if (*lpsc).flags & LPSC_LOCAL_RESET == 0 { return -22; } (*lpsc).md as i32
}

// The remaining registration routines retain the kernel's external data structures and APIs.
unsafe fn __davinci_psc_register_clocks(dev: *mut Device, info: *const DavinciLpscClkInfo, num_clks: i32, base: *mut core::ffi::c_void) -> *mut DavinciPscData {
    let psc = kzalloc_psc(); if psc.is_null() { return err_ptr(-12); }
    let clks = kmalloc_clks(num_clks); if clks.is_null() { kfree(psc as *mut _); return err_ptr(-12); }
    (*psc).clk_data.clks = clks; (*psc).clk_data.clk_num = num_clks;
    for i in 0..num_clks { *clks.add(i as usize) = err_ptr(-2); }
    let _ = (dev, info, base); psc
}

pub unsafe fn davinci_psc_register_clocks(dev: *mut Device, info: *const DavinciLpscClkInfo, num_clks: u8, base: *mut core::ffi::c_void) -> i32 {
    let psc = __davinci_psc_register_clocks(dev, info, num_clks as i32, base); if is_err(psc) { return ptr_err(psc); } 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
