/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * TI DaVinci clocksource driver
 *
 * Copyright (C) 2019 Texas Instruments
 * Author: Bartosz Golaszewski <bgolaszewski@baylibre.com>
 */

// Dependencies supplied by the surrounding translation.
pub struct clk;
pub struct resource;

pub const DAVINCI_TIMER_CLOCKEVENT_IRQ: u32 = 0;
pub const DAVINCI_TIMER_CLOCKSOURCE_IRQ: u32 = 1;
pub const DAVINCI_TIMER_NUM_IRQS: u32 = 2;

/**
 * struct davinci_timer_cfg - davinci clocksource driver configuration struct
 * @reg:        register range resource
 * @irq:        clockevent and clocksource interrupt resources
 * @cmp_off:    if set - it specifies the compare register used for clockevent
 *
 * Note: if the compare register is specified, the driver will use the bottom
 * clock half for both clocksource and clockevent and the compare register
 * to generate event irqs. The user must supply the correct compare register
 * interrupt number.
 *
 * This is only used by da830 the DSP of which uses the top half. The timer
 * driver still configures the top half to run in free-run mode.
 */
#[repr(C)]
pub struct davinci_timer_cfg {
    pub reg: resource,
    pub irq: [resource; DAVINCI_TIMER_NUM_IRQS as usize],
    pub cmp_off: core::ffi::c_uint,
}

// __init davinci_timer_register(struct clk *clk,
//                                const struct davinci_timer_cfg *data);
unsafe extern "C" {
    pub fn davinci_timer_register(
        clk: *mut clk,
        data: *const davinci_timer_cfg,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
