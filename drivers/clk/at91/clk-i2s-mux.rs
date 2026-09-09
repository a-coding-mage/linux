// SPDX-License-Identifier: GPL-2.0
/*
 *  Copyright (C) 2018 Microchip Technology Inc,
 *                     Codrin Ciubotariu <codrin.ciubotariu@microchip.com>
 *
 *
 */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/clk-provider.h, linux/of.h, linux/mfd/syscon.h, linux/regmap.h,
// linux/slab.h, soc/at91/atmel-sfr.h, and pmc.h.

#[repr(C)]
struct clk_i2s_mux {
    hw: clk_hw,
    regmap: *mut regmap,
    bus_id: u8,
}

#[inline]
unsafe fn to_clk_i2s_mux(hw: *mut clk_hw) -> *mut clk_i2s_mux {
    // C equivalent: container_of(hw, struct clk_i2s_mux, hw)
    (hw as *mut u8).sub(core::mem::offset_of!(clk_i2s_mux, hw)) as *mut clk_i2s_mux
}

unsafe extern "C" fn clk_i2s_mux_get_parent(hw: *mut clk_hw) -> u8 {
    let mux: *mut clk_i2s_mux = to_clk_i2s_mux(hw);
    let mut val: u32 = 0;

    regmap_read((*mux).regmap, AT91_SFR_I2SCLKSEL, &mut val);

    ((val & (1u32 << (*mux).bus_id)) >> (*mux).bus_id) as u8
}

unsafe extern "C" fn clk_i2s_mux_set_parent(hw: *mut clk_hw, index: u8) -> i32 {
    let mux: *mut clk_i2s_mux = to_clk_i2s_mux(hw);

    regmap_update_bits(
        (*mux).regmap,
        AT91_SFR_I2SCLKSEL,
        1u32 << (*mux).bus_id,
        (index as u32) << (*mux).bus_id,
    )
}

static clk_i2s_mux_ops: clk_ops = clk_ops {
    get_parent: Some(clk_i2s_mux_get_parent),
    set_parent: Some(clk_i2s_mux_set_parent),
    determine_rate: Some(__clk_mux_determine_rate),
};

unsafe extern "C" fn at91_clk_i2s_mux_register(
    regmap: *mut regmap,
    name: *const core::ffi::c_char,
    parent_names: *const *const core::ffi::c_char,
    num_parents: u32,
    bus_id: u8,
) -> *mut clk_hw {
    let mut init: clk_init_data = core::mem::zeroed();
    let mut i2s_ck: *mut clk_i2s_mux;
    let ret: i32;

    i2s_ck = kzalloc(core::mem::size_of::<clk_i2s_mux>()) as *mut clk_i2s_mux;
    if i2s_ck.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    init.name = name;
    init.ops = &clk_i2s_mux_ops;
    init.parent_names = parent_names;
    init.num_parents = num_parents;

    (*i2s_ck).hw.init = &init;
    (*i2s_ck).bus_id = bus_id;
    (*i2s_ck).regmap = regmap;

    ret = clk_hw_register(core::ptr::null_mut(), &mut (*i2s_ck).hw);
    if ret != 0 {
        kfree(i2s_ck as *mut core::ffi::c_void);
        return ERR_PTR(ret);
    }

    &mut (*i2s_ck).hw
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
