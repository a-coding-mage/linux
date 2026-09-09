// SPDX-License-Identifier: (GPL-2.0 OR MIT)
/*
 * Copyright (c) 2018 BayLibre, SAS.
 * Author: Jerome Brunet <jbrunet@baylibre.com>
 */

// Dependencies supplied by the Linux clock-provider and Meson clock headers.

#[inline]
fn phase_step(width: u32) -> u32 {
    360 / (1u32 << width)
}

#[inline]
unsafe fn meson_clk_phase_data(clk: *mut clk_regmap) -> *mut meson_clk_phase_data {
    (*clk).data as *mut meson_clk_phase_data
}

unsafe fn meson_clk_degrees_from_val(val: u32, width: u32) -> i32 {
    (phase_step(width) * val) as i32
}

unsafe fn meson_clk_degrees_to_val(degrees: i32, width: u32) -> u32 {
    let val = div_round_closest(degrees, phase_step(width) as i32) as u32;

    /*
     * This last calculation is here for cases when degrees is rounded
     * to 360, in which case val == (1 << width).
     */
    val % (1u32 << width)
}

unsafe fn meson_clk_phase_get_phase(hw: *mut clk_hw) -> i32 {
    let clk = to_clk_regmap(hw);
    let phase = meson_clk_phase_data(clk);
    let val: u32;

    val = meson_parm_read((*clk).map, &(*phase).ph);

    meson_clk_degrees_from_val(val, (*phase).ph.width)
}

unsafe fn meson_clk_phase_set_phase(hw: *mut clk_hw, degrees: i32) -> i32 {
    let clk = to_clk_regmap(hw);
    let phase = meson_clk_phase_data(clk);
    let val: u32;

    val = meson_clk_degrees_to_val(degrees, (*phase).ph.width);
    meson_parm_write((*clk).map, &(*phase).ph, val);

    0
}

pub static meson_clk_phase_ops: clk_ops = clk_ops {
    init: Some(clk_regmap_init),
    get_phase: Some(meson_clk_phase_get_phase),
    set_phase: Some(meson_clk_phase_set_phase),
};

/*
 * This is a special clock for the audio controller.
 * The phase of mst_sclk clock output can be controlled independently
 * for the outside world (ph0), the tdmout (ph1) and tdmin (ph2).
 * Controlling these 3 phases as just one makes things simpler and
 * give the same clock view to all the element on the i2s bus.
 * If necessary, we can still control the phase in the tdm block
 * which makes these independent control redundant.
 */
#[inline]
unsafe fn meson_clk_triphase_data(clk: *mut clk_regmap) -> *mut meson_clk_triphase_data {
    (*clk).data as *mut meson_clk_triphase_data
}

unsafe fn meson_clk_triphase_sync(hw: *mut clk_hw) -> i32 {
    let clk = to_clk_regmap(hw);
    let tph = meson_clk_triphase_data(clk);
    let val: u32;
    let ret: i32;

    ret = clk_regmap_init(hw);
    if ret != 0 {
        return ret;
    }

    /* Get phase 0 and sync it to phase 1 and 2 */
    val = meson_parm_read((*clk).map, &(*tph).ph0);
    meson_parm_write((*clk).map, &(*tph).ph1, val);
    meson_parm_write((*clk).map, &(*tph).ph2, val);

    0
}

unsafe fn meson_clk_triphase_get_phase(hw: *mut clk_hw) -> i32 {
    let clk = to_clk_regmap(hw);
    let tph = meson_clk_triphase_data(clk);
    let val: u32;

    /* Phase are in sync, reading phase 0 is enough */
    val = meson_parm_read((*clk).map, &(*tph).ph0);

    meson_clk_degrees_from_val(val, (*tph).ph0.width)
}

unsafe fn meson_clk_triphase_set_phase(hw: *mut clk_hw, degrees: i32) -> i32 {
    let clk = to_clk_regmap(hw);
    let tph = meson_clk_triphase_data(clk);
    let val: u32;

    val = meson_clk_degrees_to_val(degrees, (*tph).ph0.width);
    meson_parm_write((*clk).map, &(*tph).ph0, val);
    meson_parm_write((*clk).map, &(*tph).ph1, val);
    meson_parm_write((*clk).map, &(*tph).ph2, val);

    0
}

pub static meson_clk_triphase_ops: clk_ops = clk_ops {
    init: Some(meson_clk_triphase_sync),
    get_phase: Some(meson_clk_triphase_get_phase),
    set_phase: Some(meson_clk_triphase_set_phase),
};

/*
 * This is a special clock for the audio controller.
 * This drive a bit clock inverter for which the
 * opposite value of the inverter bit needs to be manually
 * set into another bit
 */
#[inline]
unsafe fn meson_sclk_ws_inv_data(clk: *mut clk_regmap) -> *mut meson_sclk_ws_inv_data {
    (*clk).data as *mut meson_sclk_ws_inv_data
}

unsafe fn meson_sclk_ws_inv_sync(hw: *mut clk_hw) -> i32 {
    let clk = to_clk_regmap(hw);
    let tph = meson_sclk_ws_inv_data(clk);
    let val: u32;
    let ret: i32;

    ret = clk_regmap_init(hw);
    if ret != 0 {
        return ret;
    }

    /* Get phase and sync the inverted value to ws */
    val = meson_parm_read((*clk).map, &(*tph).ph);
    meson_parm_write((*clk).map, &(*tph).ws, if val != 0 { 0 } else { 1 });

    0
}

unsafe fn meson_sclk_ws_inv_get_phase(hw: *mut clk_hw) -> i32 {
    let clk = to_clk_regmap(hw);
    let tph = meson_sclk_ws_inv_data(clk);
    let val: u32;

    val = meson_parm_read((*clk).map, &(*tph).ph);

    meson_clk_degrees_from_val(val, (*tph).ph.width)
}

unsafe fn meson_sclk_ws_inv_set_phase(hw: *mut clk_hw, degrees: i32) -> i32 {
    let clk = to_clk_regmap(hw);
    let tph = meson_sclk_ws_inv_data(clk);
    let val: u32;

    val = meson_clk_degrees_to_val(degrees, (*tph).ph.width);
    meson_parm_write((*clk).map, &(*tph).ph, val);
    meson_parm_write((*clk).map, &(*tph).ws, if val != 0 { 0 } else { 1 });
    0
}

pub static meson_sclk_ws_inv_ops: clk_ops = clk_ops {
    init: Some(meson_sclk_ws_inv_sync),
    get_phase: Some(meson_sclk_ws_inv_get_phase),
    set_phase: Some(meson_sclk_ws_inv_set_phase),
};

// MODULE_DESCRIPTION("Amlogic phase driver");
// MODULE_AUTHOR("Jerome Brunet <jbrunet@baylibre.com>");
// MODULE_LICENSE("GPL");
// MODULE_IMPORT_NS("CLK_MESON");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
