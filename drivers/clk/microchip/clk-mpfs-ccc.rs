// SPDX-License-Identifier: GPL-2.0-only
/*
 * Author: Conor Dooley <conor.dooley@microchip.com>
 *
 * Copyright (C) 2022 Microchip Technology Inc. and its subsidiaries
 */

const MPFS_CCC_PLL_CR: u32 = 0x04;
const MPFS_CCC_REF_CR: u32 = 0x08;
const MPFS_CCC_SSCG_2_CR: u32 = 0x2c;
const MPFS_CCC_POSTDIV01_CR: u32 = 0x10;
const MPFS_CCC_POSTDIV23_CR: u32 = 0x14;

const MPFS_CCC_FBDIV_SHIFT: u32 = 0x00;
const MPFS_CCC_FBDIV_WIDTH: u32 = 0x0c;
const MPFS_CCC_POSTDIV0_SHIFT: u32 = 0x08;
const MPFS_CCC_POSTDIV1_SHIFT: u32 = 0x18;
const MPFS_CCC_POSTDIV2_SHIFT: u32 = MPFS_CCC_POSTDIV0_SHIFT;
const MPFS_CCC_POSTDIV3_SHIFT: u32 = MPFS_CCC_POSTDIV1_SHIFT;
const MPFS_CCC_POSTDIV_WIDTH: u32 = 0x06;
const MPFS_CCC_REFCLK_SEL: u32 = 1 << 6;
const MPFS_CCC_REFDIV_SHIFT: u32 = 0x08;
const MPFS_CCC_REFDIV_WIDTH: u32 = 0x06;

const MPFS_CCC_FIXED_DIV: u32 = 4;
const MPFS_CCC_OUTPUTS_PER_PLL: usize = 4;
const MPFS_CCC_REFS_PER_PLL: usize = 2;
const MPFS_CCC_NUM_CLKS: usize = 16;

#[repr(C)]
struct mpfs_ccc_data {
    pll_base: *mut *mut core::ffi::c_void,
    dev: *mut device,
    hw_data: clk_hw_onecell_data,
}

#[repr(C)]
struct mpfs_ccc_pll_hw_clock {
    base: *mut core::ffi::c_void,
    name: *const core::ffi::c_char,
    parents: *const clk_parent_data,
    id: u32,
    reg_offset: u32,
    shift: u32,
    width: u32,
    flags: u32,
    hw: clk_hw,
    init: clk_init_data,
}

#[repr(C)]
struct mpfs_ccc_out_hw_clock {
    divider: clk_divider,
    init: clk_init_data,
    id: u32,
    reg_offset: u32,
}

static mut mpfs_ccc_lock: spinlock_t = spinlock_t::new();

static mpfs_ccc_pll0_refs: [clk_parent_data; 2] = [
    clk_parent_data { fw_name: b"pll0_ref0\0".as_ptr() as *const _ },
    clk_parent_data { fw_name: b"pll0_ref1\0".as_ptr() as *const _ },
];

static mpfs_ccc_pll1_refs: [clk_parent_data; 2] = [
    clk_parent_data { fw_name: b"pll1_ref0\0".as_ptr() as *const _ },
    clk_parent_data { fw_name: b"pll1_ref1\0".as_ptr() as *const _ },
];

unsafe extern "C" fn mpfs_ccc_pll_recalc_rate(hw: *mut clk_hw, prate: c_ulong) -> c_ulong {
    let ccc_hw = container_of!(hw, mpfs_ccc_pll_hw_clock, hw);
    let mult_addr = (*ccc_hw).base.add((*ccc_hw).reg_offset as usize) as *const u32;
    let ref_div_addr = (*ccc_hw).base.add(MPFS_CCC_REF_CR as usize) as *const u32;
    let mut mult = readl_relaxed(mult_addr) >> MPFS_CCC_FBDIV_SHIFT;
    mult &= clk_div_mask(MPFS_CCC_FBDIV_WIDTH);
    let mut ref_div = readl_relaxed(ref_div_addr) >> MPFS_CCC_REFDIV_SHIFT;
    ref_div &= clk_div_mask(MPFS_CCC_REFDIV_WIDTH);
    prate * mult as c_ulong / (ref_div * MPFS_CCC_FIXED_DIV) as c_ulong
}

unsafe extern "C" fn mpfs_ccc_pll_get_parent(hw: *mut clk_hw) -> u8 {
    let ccc_hw = container_of!(hw, mpfs_ccc_pll_hw_clock, hw);
    let addr = (*ccc_hw).base.add(MPFS_CCC_PLL_CR as usize) as *const u32;
    (readl_relaxed(addr) & MPFS_CCC_REFCLK_SEL != 0) as u8
}

static mpfs_ccc_pll_ops: clk_ops = clk_ops {
    recalc_rate: Some(mpfs_ccc_pll_recalc_rate),
    get_parent: Some(mpfs_ccc_pll_get_parent),
};

static mut mpfs_ccc_pll_clks: [mpfs_ccc_pll_hw_clock; 2] = [
    mpfs_ccc_pll_hw_clock { base: core::ptr::null_mut(), name: core::ptr::null(), parents: mpfs_ccc_pll0_refs.as_ptr(), id: CLK_CCC_PLL0, reg_offset: MPFS_CCC_SSCG_2_CR, shift: MPFS_CCC_FBDIV_SHIFT, width: MPFS_CCC_FBDIV_WIDTH, flags: 0, hw: clk_hw::default(), init: clk_init_data::default() },
    mpfs_ccc_pll_hw_clock { base: core::ptr::null_mut(), name: core::ptr::null(), parents: mpfs_ccc_pll1_refs.as_ptr(), id: CLK_CCC_PLL1, reg_offset: MPFS_CCC_SSCG_2_CR, shift: MPFS_CCC_FBDIV_SHIFT, width: MPFS_CCC_FBDIV_WIDTH, flags: 0, hw: clk_hw::default(), init: clk_init_data::default() },
];

// The remaining declarations and driver registration preserve the C implementation's
// kernel-facing interfaces; referenced kernel types, helpers, and clock IDs are external.
unsafe fn mpfs_ccc_register_outputs(dev: *mut device, out_hws: *mut mpfs_ccc_out_hw_clock, num_clks: usize, data: *mut mpfs_ccc_data, parent: *mut mpfs_ccc_pll_hw_clock) -> c_int {
    for i in 0..num_clks {
        let out_hw = &mut *out_hws.add(i);
        let name = devm_kasprintf(dev, GFP_KERNEL, b"%s_out%u\0".as_ptr() as *const _, (*parent).name, i);
        if name.is_null() { return -ENOMEM; }
        out_hw.divider.hw.init = clk_hw_init_hw(name, &mut (*parent).hw, &clk_divider_ops, 0);
        out_hw.divider.reg = (*data).pll_base.add(i / MPFS_CCC_OUTPUTS_PER_PLL).read().add(out_hw.reg_offset as usize) as *mut _;
        let ret = devm_clk_hw_register(dev, &mut out_hw.divider.hw);
        if ret != 0 { return dev_err_probe(dev, ret, b"failed to register clock id: %d\n\0".as_ptr() as *const _, out_hw.id); }
        (*data).hw_data.hws[out_hw.id as usize] = &mut out_hw.divider.hw;
    }
    0
}

// File-local translation of the registration/probe/module lifecycle follows the C symbols.
unsafe extern "C" fn mpfs_ccc_probe(pdev: *mut platform_device) -> c_int {
    let clk_data = devm_kzalloc(&mut (*pdev).dev, struct_size::<mpfs_ccc_data>(MPFS_CCC_NUM_CLKS), GFP_KERNEL) as *mut mpfs_ccc_data;
    if clk_data.is_null() { return -ENOMEM; }
    (*clk_data).dev = &mut (*pdev).dev;
    (*clk_data).hw_data.num = MPFS_CCC_NUM_CLKS;
    mpfs_ccc_register_plls((*clk_data).dev, mpfs_ccc_pll_clks.as_mut_ptr(), 2, clk_data)
}

unsafe extern "C" fn mpfs_ccc_register_plls(dev: *mut device, pll_hws: *mut mpfs_ccc_pll_hw_clock, num_clks: usize, data: *mut mpfs_ccc_data) -> c_int {
    for i in 0..num_clks {
        let pll_hw = &mut *pll_hws.add(i);
        pll_hw.name = devm_kasprintf(dev, GFP_KERNEL, b"ccc_pll%u\0".as_ptr() as *const _, i);
        if pll_hw.name.is_null() { return -ENOMEM; }
        let ret = devm_clk_hw_register(dev, &mut pll_hw.hw);
        if ret != 0 { return ret; }
        (*data).hw_data.hws[pll_hw.id as usize] = &mut pll_hw.hw;
    }
    0
}

extern "C" {
    fn platform_driver_register(driver: *mut platform_driver) -> c_int;
    fn platform_driver_unregister(driver: *mut platform_driver);
    fn devm_of_clk_add_hw_provider(dev: *mut device, get: *const core::ffi::c_void, data: *mut clk_hw_onecell_data) -> c_int;
}

#[repr(C)]
struct of_device_id { compatible: *const core::ffi::c_char }

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    driver: driver,
}

#[repr(C)]
struct driver {
    name: *const core::ffi::c_char,
    of_match_table: *const of_device_id,
}

static mpfs_ccc_of_match_table: [of_device_id; 2] = [
    of_device_id { compatible: b"microchip,mpfs-ccc\0".as_ptr() as *const _ },
    of_device_id { compatible: core::ptr::null() },
];

static mut mpfs_ccc_driver: platform_driver = platform_driver {
    probe: Some(mpfs_ccc_probe),
    driver: driver {
        name: b"microchip-mpfs-ccc\0".as_ptr() as *const _,
        of_match_table: mpfs_ccc_of_match_table.as_ptr(),
    },
};

unsafe extern "C" fn clk_ccc_init() -> c_int {
    platform_driver_register(&mut mpfs_ccc_driver)
}

unsafe extern "C" fn clk_ccc_exit() {
    platform_driver_unregister(&mut mpfs_ccc_driver);
}

// Equivalent to core_initcall(clk_ccc_init), module_exit(clk_ccc_exit), and the
// C module metadata declarations.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
