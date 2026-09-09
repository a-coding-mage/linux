// SPDX-License-Identifier: GPL-2.0-only
/* OMAP2/3 PRM module functions */

// Kernel and local header declarations are supplied by the surrounding crate.

extern "C" {
    fn omap2_prm_read_mod_bits_shift(prm_mod: i16, offset: u16, mask: u32) -> i32;
    fn omap2_prm_rmw_mod_reg_bits(mask: u32, bits: u32, prm_mod: i16, offset: u16);
    fn omap2_prm_read_mod_reg(prm_mod: i16, offset: u16) -> u32;
    fn omap2_prm_set_mod_reg_bits(bits: u32, prm_mod: i16, offset: u16);
    fn omap2_prm_clear_mod_reg_bits(bits: u32, prm_mod: i16, offset: u16);
    fn udelay(usecs: u32);
    fn pr_err(fmt: *const u8, ...);
    fn pr_debug(fmt: *const u8, ...);
}

#[repr(C)]
pub struct powerdomain {
    pub prcm_offs: i16,
    pub name: *const u8,
}

#[repr(C)]
pub struct clockdomain {
    pub pwrdm: *mut clockdomain_powerdomain,
    pub wkdep_srcs: *mut clkdm_dep,
}

#[repr(C)]
pub struct clockdomain_powerdomain { pub ptr: *mut powerdomain }

#[repr(C)]
pub struct clkdm_dep {
    pub clkdm_name: *const u8,
    pub clkdm: *mut clockdomain_dep_target,
    pub wkdep_usecount: u32,
}

#[repr(C)]
pub struct clockdomain_dep_target { pub dep_bit: u8 }

pub unsafe fn omap2_prm_is_hardreset_asserted(shift: u8, _part: u8, prm_mod: i16, _offset: u16) -> i32 {
    omap2_prm_read_mod_bits_shift(prm_mod, OMAP2_RM_RSTCTRL, 1u32 << shift)
}

pub unsafe fn omap2_prm_assert_hardreset(shift: u8, _part: u8, prm_mod: i16, _offset: u16) -> i32 {
    let mask = 1u32 << shift;
    omap2_prm_rmw_mod_reg_bits(mask, mask, prm_mod, OMAP2_RM_RSTCTRL);
    0
}

pub unsafe fn omap2_prm_deassert_hardreset(rst_shift: u8, st_shift: u8, _part: u8,
                                           prm_mod: i16, _rst_offset: u16, _st_offset: u16) -> i32 {
    let rst = 1u32 << rst_shift;
    let st = 1u32 << st_shift;
    if omap2_prm_read_mod_bits_shift(prm_mod, OMAP2_RM_RSTCTRL, rst) == 0 { return -EEXIST; }
    omap2_prm_rmw_mod_reg_bits(0xffff_ffff, st, prm_mod, OMAP2_RM_RSTST);
    omap2_prm_rmw_mod_reg_bits(rst, 0, prm_mod, OMAP2_RM_RSTCTRL);
    let mut c = 0;
    while omap2_prm_read_mod_bits_shift(prm_mod, OMAP2_RM_RSTST, st) != 0 && c < MAX_MODULE_HARDRESET_WAIT { c += 1; }
    if c == MAX_MODULE_HARDRESET_WAIT { -EBUSY } else { 0 }
}

pub unsafe fn omap2_pwrdm_set_mem_onst(pwrdm: *mut powerdomain, bank: u8, pwrst: u8) -> i32 {
    let m = omap2_pwrdm_get_mem_bank_onstate_mask(bank);
    omap2_prm_rmw_mod_reg_bits(m, (pwrst as u32) << __ffs(m), (*pwrdm).prcm_offs, OMAP2_PM_PWSTCTRL); 0
}
pub unsafe fn omap2_pwrdm_set_mem_retst(pwrdm: *mut powerdomain, bank: u8, pwrst: u8) -> i32 {
    let m = omap2_pwrdm_get_mem_bank_retst_mask(bank);
    omap2_prm_rmw_mod_reg_bits(m, (pwrst as u32) << __ffs(m), (*pwrdm).prcm_offs, OMAP2_PM_PWSTCTRL); 0
}
pub unsafe fn omap2_pwrdm_read_mem_pwrst(pwrdm: *mut powerdomain, bank: u8) -> i32 {
    omap2_prm_read_mod_bits_shift((*pwrdm).prcm_offs, OMAP2_PM_PWSTST, omap2_pwrdm_get_mem_bank_stst_mask(bank))
}
pub unsafe fn omap2_pwrdm_read_mem_retst(pwrdm: *mut powerdomain, bank: u8) -> i32 {
    omap2_prm_read_mod_bits_shift((*pwrdm).prcm_offs, OMAP2_PM_PWSTCTRL, omap2_pwrdm_get_mem_bank_retst_mask(bank))
}
pub unsafe fn omap2_pwrdm_set_logic_retst(pwrdm: *mut powerdomain, pwrst: u8) -> i32 {
    let v = (pwrst as u32) << __ffs(OMAP_LOGICRETSTATE_MASK);
    omap2_prm_rmw_mod_reg_bits(OMAP_LOGICRETSTATE_MASK, v, (*pwrdm).prcm_offs, OMAP2_PM_PWSTCTRL); 0
}
pub unsafe fn omap2_pwrdm_wait_transition(pwrdm: *mut powerdomain) -> i32 {
    let mut c = 0u32;
    while (omap2_prm_read_mod_reg((*pwrdm).prcm_offs, OMAP2_PM_PWSTST) & OMAP_INTRANSITION_MASK) != 0 && { c += 1; c <= PWRDM_TRANSITION_BAILOUT } { udelay(1); }
    if c > PWRDM_TRANSITION_BAILOUT { pr_err(b"powerdomain: %s: waited too long to complete transition\n\0".as_ptr(), (*pwrdm).name); return -EAGAIN; }
    pr_debug(b"powerdomain: completed transition in %d loops\n\0".as_ptr(), c); 0
}

pub unsafe fn omap2_clkdm_add_wkdep(clkdm1: *mut clockdomain, clkdm2: *mut clockdomain_dep_target) -> i32 { omap2_prm_set_mod_reg_bits(1u32 << (*clkdm2).dep_bit, (*(*(*clkdm1).pwrdm).ptr).prcm_offs, PM_WKDEP); 0 }
pub unsafe fn omap2_clkdm_del_wkdep(clkdm1: *mut clockdomain, clkdm2: *mut clockdomain_dep_target) -> i32 { omap2_prm_clear_mod_reg_bits(1u32 << (*clkdm2).dep_bit, (*(*(*clkdm1).pwrdm).ptr).prcm_offs, PM_WKDEP); 0 }
pub unsafe fn omap2_clkdm_read_wkdep(clkdm1: *mut clockdomain, clkdm2: *mut clockdomain_dep_target) -> i32 { omap2_prm_read_mod_bits_shift((*(*(*clkdm1).pwrdm).ptr).prcm_offs, PM_WKDEP, 1u32 << (*clkdm2).dep_bit) }

pub unsafe fn omap2_clkdm_clear_all_wkdeps(clkdm: *mut clockdomain) -> i32 {
    let mut cd = (*clkdm).wkdep_srcs; let mut mask = 0u32;
    while !cd.is_null() && !(*cd).clkdm_name.is_null() { if (*cd).clkdm.is_null() { cd = cd.add(1); continue; } mask |= 1u32 << (*(*cd).clkdm).dep_bit; (*cd).wkdep_usecount = 0; cd = cd.add(1); }
    omap2_prm_clear_mod_reg_bits(mask, (*(*(*clkdm).pwrdm).ptr).prcm_offs, PM_WKDEP); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
