// SPDX-License-Identifier: GPL-2.0-only
/*
 * OMAP2xxx DVFS virtual clock functions
 *
 * Copyright (C) 2005-2008, 2012 Texas Instruments, Inc.
 * Copyright (C) 2004-2010 Nokia Corporation
 *
 * Contacts:
 * Richard Woodruff <r-woodruff2@ti.com>
 * Paul Walmsley
 *
 * Based on earlier work by Tuukka Tikkanen, Tony Lindgren,
 * Gordon McNutt and RidgeRun, Inc.
 *
 * XXX Some of this code should be replaceable by the upcoming OPP layer
 * code.  However, some notion of "rate set" is probably still necessary
 * for OMAP2xxx at least.  Rate sets should be generalized so they can be
 * used for any OMAP chip, not just OMAP2xxx.  In particular, Richard Woodruff
 * has in the past expressed a preference to use rate sets for OPP changes,
 * rather than dynamically recalculating the clock tree, so if someone wants
 * this badly enough to write the code to handle it, we should support it
 * as an option.
 */

// External kernel and SoC declarations supplied by other translation units.
#[repr(C)]
pub struct prcm_config {
    pub mpu_speed: libc::c_ulong,
    pub flags: u16,
    pub xtal_speed: libc::c_ulong,
    pub dpll_speed: u32,
    pub cm_clksel2_pll: u32,
    pub cm_clksel_mpu: u32,
    pub cm_clksel_dsp: u32,
    pub cm_clksel_gfx: u32,
    pub cm_clksel1_core: u32,
    pub cm_clksel_mdm: u32,
    pub cm_clksel1_pll: u32,
    pub base_sdrc_rfr: u32,
}

#[repr(C)] pub struct clk_hw { _private: [u8; 0] }
#[repr(C)] pub struct clk_rate_request { pub rate: libc::c_ulong }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct clk_hw_omap { pub hw: clk_hw, pub init: *mut clk_init_data }
#[repr(C)] pub struct clk_init_data {
    pub name: *const libc::c_char,
    pub ops: *const clk_ops,
    pub parent_names: *const *const libc::c_char,
    pub num_parents: u8,
}
#[repr(C)] pub struct clk_ops {
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, libc::c_ulong) -> libc::c_ulong>,
    pub set_rate: Option<unsafe extern "C" fn(*mut clk_hw, libc::c_ulong, libc::c_ulong) -> libc::c_int>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> libc::c_int>,
}

extern "C" {
    fn omap2xxx_clk_get_core_rate() -> libc::c_ulong;
    fn omap2xxx_sdrc_reprogram(source: u32, dll: u32);
    fn omap2xxx_cm_set_mod_dividers(mpu: u32, dsp: u32, gfx: u32, core: u32, mdm: u32);
    fn omap2_set_prcm(pll: u32, rfr: u32, bypass: u32);
    fn omap2xxx_sdrc_init_params(unlocked: u32);
    fn omap2xxx_sdrc_dll_is_unlocked() -> u32;
    fn clk_get(dev: *mut libc::c_void, name: *const libc::c_char) -> *mut clk;
    fn clk_get_rate(clk: *mut clk) -> libc::c_ulong;
    fn clk_put(clk: *mut clk);
    fn kzalloc(size: usize, flags: libc::c_uint) -> *mut libc::c_void;
    fn kfree(ptr: *mut libc::c_void);
    fn clk_register(dev: *mut libc::c_void, hw: *mut clk_hw) -> *mut clk;
    fn clkdev_create(clk: *mut clk, con_id: *const libc::c_char, dev_fmt: *const libc::c_char) -> *mut libc::c_void;
}

static mut CPU_MASK: u16 = 0;
pub static mut CURR_PRCM_SET: *const prcm_config = core::ptr::null();
pub static mut RATE_TABLE: *const prcm_config = core::ptr::null();
static mut SYS_CK_RATE: libc::c_ulong = 0;

unsafe extern "C" fn omap2_table_mpu_recalc(_clk: *mut clk_hw, _parent_rate: libc::c_ulong) -> libc::c_ulong {
    (*CURR_PRCM_SET).mpu_speed
}

unsafe extern "C" fn omap2_determine_rate_to_table(_hw: *mut clk_hw, req: *mut clk_rate_request) -> libc::c_int {
    let mut ptr = RATE_TABLE;
    let mut highest_rate: libc::c_long = -22;
    while (*ptr).mpu_speed != 0 {
        if ((*ptr).flags & CPU_MASK) == 0 { ptr = ptr.add(1); continue; }
        if (*ptr).xtal_speed != SYS_CK_RATE { ptr = ptr.add(1); continue; }
        highest_rate = (*ptr).mpu_speed as libc::c_long;
        if (*ptr).mpu_speed <= (*req).rate { break; }
        ptr = ptr.add(1);
    }
    (*req).rate = highest_rate as libc::c_ulong;
    0
}

unsafe extern "C" fn omap2_select_table_rate(_hw: *mut clk_hw, rate: libc::c_ulong, _parent_rate: libc::c_ulong) -> libc::c_int {
    let mut prcm = RATE_TABLE;
    let mut found_speed = 0;
    while (*prcm).mpu_speed != 0 {
        if ((*prcm).flags & CPU_MASK) == 0 { prcm = prcm.add(1); continue; }
        if (*prcm).xtal_speed != SYS_CK_RATE { prcm = prcm.add(1); continue; }
        if (*prcm).mpu_speed <= rate { found_speed = (*prcm).mpu_speed; break; }
        prcm = prcm.add(1);
    }
    if found_speed == 0 { return -22; }
    CURR_PRCM_SET = prcm;
    let cur_rate = omap2xxx_clk_get_core_rate();
    if (*prcm).dpll_speed as libc::c_ulong == cur_rate / 2 {
        omap2xxx_sdrc_reprogram(0, 1);
    } else if (*prcm).dpll_speed as libc::c_ulong == cur_rate * 2 {
        omap2xxx_sdrc_reprogram(1, 1);
    } else if (*prcm).dpll_speed as libc::c_ulong != cur_rate {
        let mut bypass = 0;
        if (*prcm).dpll_speed as libc::c_ulong == (*prcm).xtal_speed { bypass = 1; }
        let done_rate = if ((*prcm).cm_clksel2_pll & 0x3) == 1 { 1 } else { 0 };
        omap2xxx_cm_set_mod_dividers((*prcm).cm_clksel_mpu, (*prcm).cm_clksel_dsp, (*prcm).cm_clksel_gfx, (*prcm).cm_clksel1_core, (*prcm).cm_clksel_mdm);
        omap2xxx_sdrc_reprogram(1, 1);
        omap2_set_prcm((*prcm).cm_clksel1_pll, (*prcm).base_sdrc_rfr, bypass);
        omap2xxx_sdrc_init_params(omap2xxx_sdrc_dll_is_unlocked());
        omap2xxx_sdrc_reprogram(done_rate, 0);
    }
    0
}

unsafe fn omap2xxx_clkt_vps_check_bootloader_rates() {
    let mut prcm = RATE_TABLE;
    let rate = omap2xxx_clk_get_core_rate();
    while (*prcm).mpu_speed != 0 {
        if ((*prcm).flags & CPU_MASK) == 0 || (*prcm).xtal_speed != SYS_CK_RATE { prcm = prcm.add(1); continue; }
        if (*prcm).dpll_speed as libc::c_ulong <= rate { break; }
        prcm = prcm.add(1);
    }
    CURR_PRCM_SET = prcm;
}

unsafe fn omap2xxx_clkt_vps_late_init() {
    let name = b"sys_ck\0";
    let c = clk_get(core::ptr::null_mut(), name.as_ptr() as *const libc::c_char);
    if !c.is_null() { SYS_CK_RATE = clk_get_rate(c); clk_put(c); }
}

static VIRT_PRCM_SET_OPS: clk_ops = clk_ops {
    recalc_rate: Some(omap2_table_mpu_recalc),
    set_rate: Some(omap2_select_table_rate),
    determine_rate: Some(omap2_determine_rate_to_table),
};

#[no_mangle]
pub unsafe extern "C" fn omap2xxx_clkt_vps_init() {
    omap2xxx_clkt_vps_late_init();
    omap2xxx_clkt_vps_check_bootloader_rates();
    let parent_name = b"mpu_ck\0";
    let mut hw = kzalloc(core::mem::size_of::<clk_hw_omap>(), 0) as *mut clk_hw_omap;
    if hw.is_null() { return; }
    let name = b"virt_prcm_set\0";
    let init = Box::new(clk_init_data {
        name: name.as_ptr() as *const libc::c_char,
        ops: &VIRT_PRCM_SET_OPS,
        parent_names: &parent_name.as_ptr() as *const *const u8 as *const *const libc::c_char,
        num_parents: 1,
    });
    (*hw).init = Box::into_raw(init);
    let clk = clk_register(core::ptr::null_mut(), &mut (*hw).hw);
    if clk.is_null() { kfree(hw as *mut libc::c_void); return; }
    let con_id = b"cpufreq_ck\0";
    clkdev_create(clk, con_id.as_ptr() as *const libc::c_char, core::ptr::null());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
