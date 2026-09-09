// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2016 Rafał Miłecki <rafal@milecki.pl>
 */

// Dependency declarations corresponding to the Linux kernel headers used by the C source.

#[repr(C)]
pub struct ClkHw {
    pub init: *const ClkInitData,
}

#[repr(C)]
pub struct Regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct DeviceNode {
    pub name: *const core::ffi::c_char,
    pub parent: *mut DeviceNode,
}

pub type ClkEnable = unsafe extern "C" fn(*mut ClkHw) -> i32;
pub type ClkDisable = unsafe extern "C" fn(*mut ClkHw);
pub type ClkRecalcRate = unsafe extern "C" fn(*mut ClkHw, usize) -> usize;

#[repr(C)]
pub struct ClkOps {
    pub enable: Option<ClkEnable>,
    pub disable: Option<ClkDisable>,
    pub recalc_rate: Option<ClkRecalcRate>,
}

#[repr(C)]
pub struct ClkInitData {
    pub name: *const core::ffi::c_char,
    pub ops: *const ClkOps,
    pub parent_names: *const *const core::ffi::c_char,
    pub num_parents: u32,
}

unsafe extern "C" {
    fn regmap_write(map: *mut Regmap, reg: u32, val: u32) -> i32;
    fn regmap_read(map: *mut Regmap, reg: u32, val: *mut u32) -> i32;
    fn cpu_relax();
    fn of_clk_get_parent_name(np: *mut DeviceNode, index: u32) -> *const core::ffi::c_char;
    fn syscon_node_to_regmap(np: *mut DeviceNode) -> *mut Regmap;
    fn clk_hw_register(dev: *mut core::ffi::c_void, hw: *mut ClkHw) -> i32;
    fn of_clk_add_hw_provider(
        np: *mut DeviceNode,
        get: unsafe extern "C" fn(*mut DeviceNode, *const core::ffi::c_void) -> *mut ClkHw,
        data: *mut ClkHw,
    ) -> i32;
    fn clk_hw_unregister(hw: *mut ClkHw);
    fn kfree(ptr: *mut bcm53573_ilp);
    fn pr_err(fmt: *const core::ffi::c_char, ...);
}

const PMU_XTAL_FREQ_RATIO: u32 = 0x66c;
const XTAL_ALP_PER_4ILP: u32 = 0x00001fff;
const XTAL_CTL_EN: u32 = 0x80000000;
const PMU_SLOW_CLK_PERIOD: u32 = 0x6dc;

#[repr(C)]
pub struct bcm53573_ilp {
    pub hw: ClkHw,
    pub regmap: *mut Regmap,
}

#[inline]
unsafe fn bcm53573_ilp_from_hw(hw: *mut ClkHw) -> *mut bcm53573_ilp {
    hw as *mut bcm53573_ilp
}

unsafe extern "C" fn bcm53573_ilp_enable(hw: *mut ClkHw) -> i32 {
    let ilp = &mut *bcm53573_ilp_from_hw(hw);

    regmap_write(ilp.regmap, PMU_SLOW_CLK_PERIOD, 0x10199);
    regmap_write(ilp.regmap, 0x674, 0x10000);

    0
}

unsafe extern "C" fn bcm53573_ilp_disable(hw: *mut ClkHw) {
    let ilp = &mut *bcm53573_ilp_from_hw(hw);

    regmap_write(ilp.regmap, PMU_SLOW_CLK_PERIOD, 0);
    regmap_write(ilp.regmap, 0x674, 0);
}

unsafe extern "C" fn bcm53573_ilp_recalc_rate(
    hw: *mut ClkHw,
    parent_rate: usize,
) -> usize {
    let ilp = &mut *bcm53573_ilp_from_hw(hw);
    let regmap = ilp.regmap;
    let mut last_val: u32 = 0;
    let mut cur_val: u32;
    let mut sum: i32 = 0;
    let mut num: i32 = 0;
    let mut loop_num: i32 = 0;
    let avg: i32;

    /* Enable measurement */
    regmap_write(regmap, PMU_XTAL_FREQ_RATIO, XTAL_CTL_EN);

    /* Read initial value */
    regmap_read(regmap, PMU_XTAL_FREQ_RATIO, &mut last_val);
    last_val &= XTAL_ALP_PER_4ILP;

    /*
     * At minimum we should loop for a bit to let hardware do the
     * measurement. This isn't very accurate however, so for a better
     * precision let's try getting 20 different values and use average.
     */
    while num < 20 {
        cur_val = 0;
        regmap_read(regmap, PMU_XTAL_FREQ_RATIO, &mut cur_val);
        cur_val &= XTAL_ALP_PER_4ILP;

        if cur_val != last_val {
            /* Got different value, use it */
            sum += cur_val as i32;
            num += 1;
            loop_num = 0;
            last_val = cur_val;
        } else {
            loop_num += 1;
            if loop_num > 5000 {
                /* Same value over and over, give up */
                sum += cur_val as i32;
                num += 1;
                break;
            }
        }

        cpu_relax();
    }

    /* Disable measurement to save power */
    regmap_write(regmap, PMU_XTAL_FREQ_RATIO, 0x0);

    avg = sum / num;

    parent_rate * 4 / avg as usize
}

static BCM53573_ILP_CLK_OPS: ClkOps = ClkOps {
    enable: Some(bcm53573_ilp_enable),
    disable: Some(bcm53573_ilp_disable),
    recalc_rate: Some(bcm53573_ilp_recalc_rate),
};

unsafe extern "C" fn bcm53573_ilp_init(np: *mut DeviceNode) {
    let ilp = {
        let ptr = libc::calloc(1, core::mem::size_of::<bcm53573_ilp>()) as *mut bcm53573_ilp;
        if ptr.is_null() {
            return;
        }
        ptr
    };
    let mut init: ClkInitData = core::mem::zeroed();
    let parent_name: *const core::ffi::c_char;
    let err: i32;

    parent_name = of_clk_get_parent_name(np, 0);
    if parent_name.is_null() {
        err = -2;
        goto_err_free_ilp(ilp, err);
        return;
    }

    (*ilp).regmap = syscon_node_to_regmap((*np).parent);

    init.name = (*np).name;
    init.ops = &BCM53573_ILP_CLK_OPS;
    init.parent_names = &parent_name;
    init.num_parents = 1;

    (*ilp).hw.init = &init;
    let register_err = clk_hw_register(core::ptr::null_mut(), &mut (*ilp).hw);
    if register_err != 0 {
        goto_err_free_ilp(ilp, register_err);
        return;
    }

    let provider_err = of_clk_add_hw_provider(np, of_clk_hw_simple_get, &mut (*ilp).hw);
    if provider_err != 0 {
        clk_hw_unregister(&mut (*ilp).hw);
        goto_err_free_ilp(ilp, provider_err);
    }
}

unsafe extern "C" fn goto_err_free_ilp(ilp: *mut bcm53573_ilp, err: i32) {
    kfree(ilp);
    // pr_err("Failed to init ILP clock: %d\n", err);
    let _ = err;
}

unsafe extern "C" fn of_clk_hw_simple_get(
    _np: *mut DeviceNode,
    data: *const core::ffi::c_void,
) -> *mut ClkHw {
    data as *mut ClkHw
}

// We need it very early for arch code, before device model gets ready
// CLK_OF_DECLARE(bcm53573_ilp_clk, "brcm,bcm53573-ilp", bcm53573_ilp_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
