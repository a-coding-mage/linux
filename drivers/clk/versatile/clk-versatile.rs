// SPDX-License-Identifier: GPL-2.0-only
/*
 * Clock driver for the ARM Integrator/AP, Integrator/CP, Versatile AB and
 * Versatile PB boards.
 * Copyright (C) 2012 Linus Walleij
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_void};

const INTEGRATOR_HDR_LOCK_OFFSET: usize = 0x14;
const VERSATILE_SYS_OSCCLCD_OFFSET: usize = 0x1c;
const VERSATILE_SYS_LOCK_OFFSET: usize = 0x20;

#[repr(C)]
pub struct device_node {
    pub name: *const c_char,
}

#[repr(C)]
pub struct clk;

#[repr(C)]
pub struct icst_params {
    pub vco_max: u32,
    pub vco_min: u32,
    pub vd_min: u32,
    pub vd_max: u32,
    pub rd_min: u32,
    pub rd_max: u32,
    pub s2div: Option<unsafe extern "C" fn(u32) -> u32>,
    pub idx2s: Option<unsafe extern "C" fn(u32) -> u32>,
}

#[repr(C)]
pub struct clk_icst_desc {
    pub params: *const icst_params,
    pub vco_offset: usize,
    pub lock_offset: usize,
}

extern "C" {
    static ICST525_VCO_MAX_5V: u32;
    static ICST525_VCO_MIN: u32;
    static ICST307_VCO_MAX: u32;
    static ICST307_VCO_MIN: u32;
    static icst525_s2div: unsafe extern "C" fn(u32) -> u32;
    static icst525_idx2s: unsafe extern "C" fn(u32) -> u32;
    static icst307_s2div: unsafe extern "C" fn(u32) -> u32;
    static icst307_idx2s: unsafe extern "C" fn(u32) -> u32;

    fn of_get_parent(np: *mut device_node) -> *mut device_node;
    fn of_iomap(np: *mut device_node, index: i32) -> *mut c_void;
    fn of_node_put(np: *mut device_node);
    fn of_clk_get_parent_name(np: *mut device_node, index: i32) -> *const c_char;
    fn icst_clk_register(
        dev: *mut c_void,
        desc: *const clk_icst_desc,
        name: *const c_char,
        parent_name: *const c_char,
        base: *mut c_void,
    ) -> *mut clk;
    fn of_clk_add_provider(
        np: *mut device_node,
        get: Option<unsafe extern "C" fn(*mut device_node, *const c_void) -> *mut clk>,
        data: *mut clk,
    ) -> i32;
    fn of_clk_src_simple_get(np: *mut device_node, data: *const c_void) -> *mut clk;
    fn pr_err(message: *const c_char, ...);
    fn IS_ERR(ptr: *mut clk) -> bool;
}

static mut cm_base: *mut c_void = core::ptr::null_mut();

static cp_auxosc_params: icst_params = icst_params {
    vco_max: unsafe { ICST525_VCO_MAX_5V },
    vco_min: unsafe { ICST525_VCO_MIN },
    vd_min: 8,
    vd_max: 263,
    rd_min: 3,
    rd_max: 65,
    s2div: Some(unsafe { icst525_s2div }),
    idx2s: Some(unsafe { icst525_idx2s }),
};

static cm_auxosc_desc: clk_icst_desc = clk_icst_desc {
    params: &cp_auxosc_params,
    vco_offset: 0x1c,
    lock_offset: INTEGRATOR_HDR_LOCK_OFFSET,
};

static versatile_auxosc_params: icst_params = icst_params {
    vco_max: unsafe { ICST307_VCO_MAX },
    vco_min: unsafe { ICST307_VCO_MIN },
    vd_min: 4 + 8,
    vd_max: 511 + 8,
    rd_min: 1 + 2,
    rd_max: 127 + 2,
    s2div: Some(unsafe { icst307_s2div }),
    idx2s: Some(unsafe { icst307_idx2s }),
};

static versatile_auxosc_desc: clk_icst_desc = clk_icst_desc {
    params: &versatile_auxosc_params,
    vco_offset: VERSATILE_SYS_OSCCLCD_OFFSET,
    lock_offset: VERSATILE_SYS_LOCK_OFFSET,
};

unsafe extern "C" fn cm_osc_setup(np: *mut device_node, desc: *const clk_icst_desc) {
    let clk: *mut clk;
    let clk_name: *const c_char = (*np).name;
    let parent_name: *const c_char;

    if cm_base.is_null() {
        // Remap the core module base if not done yet
        let parent: *mut device_node = of_get_parent(np);
        if parent.is_null() {
            pr_err(b"no parent on core module clock\0".as_ptr() as *const c_char);
            return;
        }
        cm_base = of_iomap(parent, 0);
        of_node_put(parent);
        if cm_base.is_null() {
            pr_err(b"could not remap core module base\0".as_ptr() as *const c_char);
            return;
        }
    }

    parent_name = of_clk_get_parent_name(np, 0);
    clk = icst_clk_register(core::ptr::null_mut(), desc, clk_name, parent_name, cm_base);
    if !IS_ERR(clk) {
        of_clk_add_provider(np, Some(of_clk_src_simple_get), clk);
    }
}

unsafe extern "C" fn of_integrator_cm_osc_setup(np: *mut device_node) {
    cm_osc_setup(np, &cm_auxosc_desc);
}

// CLK_OF_DECLARE(integrator_cm_auxosc_clk, "arm,integrator-cm-auxosc", of_integrator_cm_osc_setup);

unsafe extern "C" fn of_versatile_cm_osc_setup(np: *mut device_node) {
    cm_osc_setup(np, &versatile_auxosc_desc);
}

// CLK_OF_DECLARE(versatile_cm_auxosc_clk, "arm,versatile-cm-auxosc", of_versatile_cm_osc_setup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
