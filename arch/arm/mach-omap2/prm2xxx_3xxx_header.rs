/* SPDX-License-Identifier: GPL-2.0-only */
/* OMAP2xxx/3xxx-common Power/Reset Management (PRM) register definitions. */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

pub const OMAP2_RM_RSTCTRL: u32 = 0x0050;
pub const OMAP2_RM_RSTTIME: u32 = 0x0054;
pub const OMAP2_RM_RSTST: u32 = 0x0058;
pub const OMAP2_PM_PWSTCTRL: u32 = 0x00e0;
pub const OMAP2_PM_PWSTST: u32 = 0x00e4;

pub const PM_WKEN: u32 = 0x00a0;
pub const PM_WKEN1: u32 = PM_WKEN;
pub const PM_WKST: u32 = 0x00b0;
pub const PM_WKST1: u32 = PM_WKST;
pub const PM_WKDEP: u32 = 0x00c8;
pub const PM_EVGENCTRL: u32 = 0x00d4;
pub const PM_EVGENONTIM: u32 = 0x00d8;
pub const PM_EVGENOFFTIM: u32 = 0x00dc;

#[repr(C)]
pub struct PrmBase { pub va: *mut u8 }
#[repr(C)] pub struct powerdomain { _private: [u8; 0] }
#[repr(C)] pub struct clockdomain { _private: [u8; 0] }

extern "C" {
    pub static mut prm_base: PrmBase;
    fn readl_relaxed(addr: *mut c_void) -> u32;
    fn writel_relaxed(value: u32, addr: *mut c_void);
    fn __ffs(value: u32) -> u32;
}

pub unsafe fn omap2_prm_read_mod_reg(module: i16, idx: u16) -> u32 {
    readl_relaxed(prm_base.va.offset(module as isize + idx as isize) as *mut c_void)
}

pub unsafe fn omap2_prm_write_mod_reg(val: u32, module: i16, idx: u16) {
    writel_relaxed(val, prm_base.va.offset(module as isize + idx as isize) as *mut c_void);
}

pub unsafe fn omap2_prm_rmw_mod_reg_bits(mask: u32, bits: u32, module: i16, idx: i16) -> u32 {
    let mut v = omap2_prm_read_mod_reg(module, idx as u16);
    v &= !mask;
    v |= bits;
    omap2_prm_write_mod_reg(v, module, idx as u16);
    v
}

pub unsafe fn omap2_prm_read_mod_bits_shift(domain: i16, idx: i16, mask: u32) -> u32 {
    let mut v = omap2_prm_read_mod_reg(domain, idx as u16);
    v &= mask;
    v >>= __ffs(mask);
    v
}

pub unsafe fn omap2_prm_set_mod_reg_bits(bits: u32, module: i16, idx: i16) -> u32 {
    omap2_prm_rmw_mod_reg_bits(bits, bits, module, idx)
}

pub unsafe fn omap2_prm_clear_mod_reg_bits(bits: u32, module: i16, idx: i16) -> u32 {
    omap2_prm_rmw_mod_reg_bits(bits, 0x0, module, idx)
}

extern "C" {
    pub fn omap2_prm_is_hardreset_asserted(shift: u8, part: u8, prm_mod: i16, offset: u16) -> i32;
    pub fn omap2_prm_assert_hardreset(shift: u8, part: u8, prm_mod: i16, offset: u16) -> i32;
    pub fn omap2_prm_deassert_hardreset(rst_shift: u8, st_shift: u8, part: u8,
        prm_mod: i16, reset_offset: u16, st_offset: u16) -> i32;
    pub fn omap2_pwrdm_set_mem_onst(pwrdm: *mut powerdomain, bank: u8, pwrst: u8) -> i32;
    pub fn omap2_pwrdm_set_mem_retst(pwrdm: *mut powerdomain, bank: u8, pwrst: u8) -> i32;
    pub fn omap2_pwrdm_read_mem_pwrst(pwrdm: *mut powerdomain, bank: u8) -> i32;
    pub fn omap2_pwrdm_read_mem_retst(pwrdm: *mut powerdomain, bank: u8) -> i32;
    pub fn omap2_pwrdm_set_logic_retst(pwrdm: *mut powerdomain, pwrst: u8) -> i32;
    pub fn omap2_pwrdm_wait_transition(pwrdm: *mut powerdomain) -> i32;
    pub fn omap2_clkdm_add_wkdep(clkdm1: *mut clockdomain, clkdm2: *mut clockdomain) -> i32;
    pub fn omap2_clkdm_del_wkdep(clkdm1: *mut clockdomain, clkdm2: *mut clockdomain) -> i32;
    pub fn omap2_clkdm_read_wkdep(clkdm1: *mut clockdomain, clkdm2: *mut clockdomain) -> i32;
    pub fn omap2_clkdm_clear_all_wkdeps(clkdm: *mut clockdomain) -> i32;
}

pub const OMAP_ONTIMEVAL_SHIFT: u32 = 0;
pub const OMAP_ONTIMEVAL_MASK: u32 = 0xffffffff << 0;
pub const OMAP_OFFTIMEVAL_SHIFT: u32 = 0;
pub const OMAP_OFFTIMEVAL_MASK: u32 = 0xffffffff << 0;
pub const OMAP_SETUP_TIME_SHIFT: u32 = 0;
pub const OMAP_SETUP_TIME_MASK: u32 = 0xffff << 0;
pub const OMAP_SYSCLKDIV_SHIFT: u32 = 6;
pub const OMAP_SYSCLKDIV_MASK: u32 = 0x3 << 6;
pub const OMAP_SYSCLKDIV_WIDTH: u32 = 2;
pub const OMAP_AUTOEXTCLKMODE_SHIFT: u32 = 3;
pub const OMAP_AUTOEXTCLKMODE_MASK: u32 = 0x3 << 3;
pub const OMAP_SYSCLKSEL_SHIFT: u32 = 0;
pub const OMAP_SYSCLKSEL_MASK: u32 = 0x3 << 0;
pub const OMAP_OFFLOADMODE_SHIFT: u32 = 3;
pub const OMAP_OFFLOADMODE_MASK: u32 = 0x3 << 3;
pub const OMAP_ONLOADMODE_SHIFT: u32 = 1;
pub const OMAP_ONLOADMODE_MASK: u32 = 0x3 << 1;
pub const OMAP_ENABLE_MASK: u32 = 1 << 0;
pub const OMAP_RSTTIME2_SHIFT: u32 = 8;
pub const OMAP_RSTTIME2_MASK: u32 = 0x1f << 8;
pub const OMAP_RSTTIME1_SHIFT: u32 = 0;
pub const OMAP_RSTTIME1_MASK: u32 = 0xff << 0;
pub const OMAP_RST_DPLL3_MASK: u32 = 1 << 2;
pub const OMAP_RST_GS_MASK: u32 = 1 << 1;
pub const OMAP_COREDOMAINWKUP_RST_MASK: u32 = 1 << 3;
pub const OMAP_DOMAINWKUP_RST_MASK: u32 = 1 << 2;
pub const OMAP_GLOBALWARM_RST_SHIFT: u32 = 1;
pub const OMAP_GLOBALWARM_RST_MASK: u32 = 1 << 1;
pub const OMAP_GLOBALCOLD_RST_SHIFT: u32 = 0;
pub const OMAP_GLOBALCOLD_RST_MASK: u32 = 1 << 0;
pub const OMAP_EN_WKUP_SHIFT: u32 = 4;
pub const OMAP_EN_WKUP_MASK: u32 = 1 << 4;
pub const OMAP_LOGICRETSTATE_MASK: u32 = 1 << 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
