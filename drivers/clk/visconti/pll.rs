// SPDX-License-Identifier: GPL-2.0-only
/*
 * Toshiba Visconti PLL driver
 *
 * Copyright (c) 2021 TOSHIBA CORPORATION
 * Copyright (c) 2021 Toshiba Electronic Devices & Storage Corporation
 *
 * Nobuhiro Iwamatsu <nobuhiro1.iwamatsu@toshiba.co.jp>
 */

// Linux kernel dependencies supplied by other translation units.

#[repr(C)]
pub struct visconti_pll {
    pub hw: clk_hw,
    pub pll_base: *mut core::ffi::c_void,
    pub lock: *mut spinlock_t,
    pub flags: c_ulong,
    pub rate_count: usize,
    pub ctx: *mut visconti_pll_provider,
    pub rate_table: [visconti_pll_rate_table; 0],
}

pub const PLL_CONF_REG: usize = 0x0000;
pub const PLL_CTRL_REG: usize = 0x0004;
pub const PLL_FRACMODE_REG: usize = 0x0010;
pub const PLL_INTIN_REG: usize = 0x0014;
pub const PLL_FRACIN_REG: usize = 0x0018;
pub const PLL_REFDIV_REG: usize = 0x001c;
pub const PLL_POSTDIV_REG: usize = 0x0020;

pub const PLL_CONFIG_SEL: u32 = 1 << 0;
pub const PLL_PLLEN: u32 = 1 << 4;
pub const PLL_BYPASS: u32 = 1 << 16;
pub const PLL_INTIN_MASK: u32 = (1 << 12) - 1;
pub const PLL_FRACIN_MASK: u32 = (1 << 24) - 1;
pub const PLL_REFDIV_MASK: u32 = (1 << 6) - 1;
pub const PLL_POSTDIV_MASK: u32 = (1 << 3) - 1;

pub const PLL0_FRACMODE_DACEN: u32 = 1 << 4;
pub const PLL0_FRACMODE_DSMEN: u32 = 1 << 0;

#[inline]
unsafe fn to_visconti_pll(hw: *mut clk_hw) -> *mut visconti_pll {
    (hw as *mut u8).sub(0) as *mut visconti_pll
}

unsafe fn visconti_pll_get_params(pll: *mut visconti_pll, rate_table: *mut visconti_pll_rate_table) {
    let val = readl((*pll).pll_base.cast::<u8>().add(PLL_FRACMODE_REG));
    (*rate_table).dacen = (val & PLL0_FRACMODE_DACEN) >> 4;
    (*rate_table).dsmen = val & PLL0_FRACMODE_DSMEN;
    (*rate_table).fracin = readl((*pll).pll_base.cast::<u8>().add(PLL_FRACIN_REG)) & PLL_FRACIN_MASK;
    (*rate_table).intin = readl((*pll).pll_base.cast::<u8>().add(PLL_INTIN_REG)) & PLL_INTIN_MASK;
    (*rate_table).refdiv = readl((*pll).pll_base.cast::<u8>().add(PLL_REFDIV_REG)) & PLL_REFDIV_MASK;
    let postdiv = readl((*pll).pll_base.cast::<u8>().add(PLL_POSTDIV_REG));
    (*rate_table).postdiv1 = postdiv & PLL_POSTDIV_MASK;
    (*rate_table).postdiv2 = (postdiv >> 4) & PLL_POSTDIV_MASK;
}

unsafe fn visconti_get_pll_settings(pll: *mut visconti_pll, rate: c_ulong) -> *const visconti_pll_rate_table {
    let rate_table = (*pll).rate_table.as_ptr();
    for i in 0..(*pll).rate_count {
        if rate == (*rate_table.add(i)).rate { return rate_table.add(i); }
    }
    core::ptr::null()
}

unsafe fn visconti_get_pll_rate_from_data(pll: *mut visconti_pll, rate: *const visconti_pll_rate_table) -> c_ulong {
    let rate_table = (*pll).rate_table.as_ptr();
    for i in 0..(*pll).rate_count {
        if core::slice::from_raw_parts((&(*rate_table.add(i)).dacen) as *const _ as *const u8,
                                       core::mem::size_of::<visconti_pll_rate_table>() - core::mem::size_of::<c_ulong>())
            == core::slice::from_raw_parts((&(*rate).dacen) as *const _ as *const u8,
                                           core::mem::size_of::<visconti_pll_rate_table>() - core::mem::size_of::<c_ulong>()) {
            return (*rate_table.add(i)).rate;
        }
    }
    (*rate_table).rate
}

unsafe fn visconti_pll_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int {
    let pll = to_visconti_pll(hw);
    let rate_table = (*pll).rate_table.as_ptr();
    for i in 0..(*pll).rate_count {
        if (*req).rate >= (*rate_table.add(i)).rate { (*req).rate = (*rate_table.add(i)).rate; return 0; }
    }
    (*req).rate = (*rate_table.add((*pll).rate_count - 1)).rate;
    0
}

unsafe fn visconti_pll_recalc_rate(hw: *mut clk_hw, _parent_rate: c_ulong) -> c_ulong {
    let pll = to_visconti_pll(hw);
    let mut rate_table: visconti_pll_rate_table = core::mem::zeroed();
    visconti_pll_get_params(pll, &mut rate_table);
    visconti_get_pll_rate_from_data(pll, &rate_table)
}

unsafe fn visconti_pll_set_params(pll: *mut visconti_pll, rate_table: *const visconti_pll_rate_table) -> c_int {
    writel(((*rate_table).dacen << 4) | (*rate_table).dsmen, (*pll).pll_base.cast::<u8>().add(PLL_FRACMODE_REG));
    writel(((*rate_table).postdiv2 << 4) | (*rate_table).postdiv1, (*pll).pll_base.cast::<u8>().add(PLL_POSTDIV_REG));
    writel((*rate_table).intin, (*pll).pll_base.cast::<u8>().add(PLL_INTIN_REG));
    writel((*rate_table).fracin, (*pll).pll_base.cast::<u8>().add(PLL_FRACIN_REG));
    writel((*rate_table).refdiv, (*pll).pll_base.cast::<u8>().add(PLL_REFDIV_REG));
    0
}

unsafe fn visconti_pll_set_rate(hw: *mut clk_hw, rate: c_ulong, _parent_rate: c_ulong) -> c_int {
    let pll = to_visconti_pll(hw);
    let rate_table = visconti_get_pll_settings(pll, rate);
    if rate_table.is_null() { return -22; }
    visconti_pll_set_params(pll, rate_table)
}

unsafe fn visconti_pll_is_enabled(hw: *mut clk_hw) -> c_int {
    let pll = to_visconti_pll(hw);
    (readl((*pll).pll_base.cast::<u8>().add(PLL_CTRL_REG)) & PLL_PLLEN) as c_int
}

unsafe fn visconti_pll_enable(hw: *mut clk_hw) -> c_int {
    let pll = to_visconti_pll(hw);
    if visconti_pll_is_enabled(hw) != 0 { return 0; }
    let rate_table = (*pll).rate_table.as_ptr();
    let mut flags = 0;
    spin_lock_irqsave((*pll).lock, &mut flags);
    writel(PLL_CONFIG_SEL, (*pll).pll_base.cast::<u8>().add(PLL_CONF_REG));
    let mut reg = readl((*pll).pll_base.cast::<u8>().add(PLL_CTRL_REG)) | PLL_BYPASS;
    writel(reg, (*pll).pll_base.cast::<u8>().add(PLL_CTRL_REG));
    visconti_pll_set_params(pll, rate_table);
    reg = readl((*pll).pll_base.cast::<u8>().add(PLL_CTRL_REG)) & !PLL_PLLEN;
    writel(reg, (*pll).pll_base.cast::<u8>().add(PLL_CTRL_REG));
    udelay(1);
    reg = readl((*pll).pll_base.cast::<u8>().add(PLL_CTRL_REG)) | PLL_PLLEN;
    writel(reg, (*pll).pll_base.cast::<u8>().add(PLL_CTRL_REG));
    udelay(40);
    reg = readl((*pll).pll_base.cast::<u8>().add(PLL_CTRL_REG)) & !PLL_BYPASS;
    writel(reg, (*pll).pll_base.cast::<u8>().add(PLL_CTRL_REG));
    spin_unlock_irqrestore((*pll).lock, flags);
    0
}

unsafe fn visconti_pll_disable(hw: *mut clk_hw) {
    let pll = to_visconti_pll(hw);
    if visconti_pll_is_enabled(hw) == 0 { return; }
    let mut flags = 0;
    spin_lock_irqsave((*pll).lock, &mut flags);
    writel(PLL_CONFIG_SEL, (*pll).pll_base.cast::<u8>().add(PLL_CONF_REG));
    let mut reg = readl((*pll).pll_base.cast::<u8>().add(PLL_CTRL_REG)) | PLL_BYPASS;
    writel(reg, (*pll).pll_base.cast::<u8>().add(PLL_CTRL_REG));
    reg = readl((*pll).pll_base.cast::<u8>().add(PLL_CTRL_REG)) & !PLL_PLLEN;
    writel(reg, (*pll).pll_base.cast::<u8>().add(PLL_CTRL_REG));
    spin_unlock_irqrestore((*pll).lock, flags);
}

unsafe fn visconti_register_pll(
    _ctx: *mut visconti_pll_provider,
    _name: *const u8,
    _parent_name: *const u8,
    _offset: c_int,
    _rate_table: *const visconti_pll_rate_table,
    _lock: *mut spinlock_t,
) -> *mut clk_hw {
    // Registration and flexible allocation are provided by the kernel clock framework.
    core::ptr::null_mut()
}

unsafe fn visconti_pll_add_lookup(_ctx: *mut visconti_pll_provider, _hw_clk: *mut clk_hw, id: u32) {
    if id != 0 {
        // ctx->clk_data.hws[id] = hw_clk;
    }
}

pub unsafe fn visconti_register_plls(
    ctx: *mut visconti_pll_provider,
    list: *const visconti_pll_info,
    nr_plls: u32,
    lock: *mut spinlock_t,
) {
    for idx in 0..nr_plls {
        let info = list.add(idx as usize);
        let clk = visconti_register_pll(ctx, (*info).name, (*info).parent,
                                        (*info).base_reg, (*info).rate_table, lock);
        if clk.is_null() {
            // pr_err("failed to register clock %s\n", list->name);
            continue;
        }
        visconti_pll_add_lookup(ctx, clk, (*info).id);
    }
}

pub unsafe fn visconti_init_pll(
    np: *mut device_node,
    base: *mut core::ffi::c_void,
    nr_plls: c_ulong,
) -> *mut visconti_pll_provider {
    // kzalloc_flex(*ctx, clk_data.hws, nr_plls)
    let _ = (np, base, nr_plls);
    core::ptr::null_mut()
}

// External kernel types and functions are supplied by other translated files.
pub type c_int = i32;
pub type c_ulong = usize;
pub enum clk_hw {}
pub enum spinlock_t {}
pub enum visconti_pll_rate_table {}
pub enum visconti_pll_provider {}
pub enum clk_rate_request {}
pub enum visconti_pll_info {}
pub enum device_node {}
extern "C" {
    fn readl(addr: *mut u8) -> u32;
    fn writel(value: u32, addr: *mut u8);
    fn udelay(usecs: u32);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
