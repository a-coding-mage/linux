/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Interrupt support for Cirrus Logic Madera codecs
 *
 * Copyright (C) 2016-2018 Cirrus Logic, Inc. and
 *                         Cirrus Logic International Semiconductor Ltd.
 */

// Dependencies supplied by the Linux interrupt and Madera core interfaces.

pub const MADERA_IRQ_FLL1_LOCK: i32 = 0;
pub const MADERA_IRQ_FLL2_LOCK: i32 = 1;
pub const MADERA_IRQ_FLL3_LOCK: i32 = 2;
pub const MADERA_IRQ_FLLAO_LOCK: i32 = 3;
pub const MADERA_IRQ_CLK_SYS_ERR: i32 = 4;
pub const MADERA_IRQ_CLK_ASYNC_ERR: i32 = 5;
pub const MADERA_IRQ_CLK_DSP_ERR: i32 = 6;
pub const MADERA_IRQ_HPDET: i32 = 7;
pub const MADERA_IRQ_MICDET1: i32 = 8;
pub const MADERA_IRQ_MICDET2: i32 = 9;
pub const MADERA_IRQ_JD1_RISE: i32 = 10;
pub const MADERA_IRQ_JD1_FALL: i32 = 11;
pub const MADERA_IRQ_JD2_RISE: i32 = 12;
pub const MADERA_IRQ_JD2_FALL: i32 = 13;
pub const MADERA_IRQ_MICD_CLAMP_RISE: i32 = 14;
pub const MADERA_IRQ_MICD_CLAMP_FALL: i32 = 15;
pub const MADERA_IRQ_DRC2_SIG_DET: i32 = 16;
pub const MADERA_IRQ_DRC1_SIG_DET: i32 = 17;
pub const MADERA_IRQ_ASRC1_IN1_LOCK: i32 = 18;
pub const MADERA_IRQ_ASRC1_IN2_LOCK: i32 = 19;
pub const MADERA_IRQ_ASRC2_IN1_LOCK: i32 = 20;
pub const MADERA_IRQ_ASRC2_IN2_LOCK: i32 = 21;
pub const MADERA_IRQ_DSP_IRQ1: i32 = 22;
pub const MADERA_IRQ_DSP_IRQ2: i32 = 23;
pub const MADERA_IRQ_DSP_IRQ3: i32 = 24;
pub const MADERA_IRQ_DSP_IRQ4: i32 = 25;
pub const MADERA_IRQ_DSP_IRQ5: i32 = 26;
pub const MADERA_IRQ_DSP_IRQ6: i32 = 27;
pub const MADERA_IRQ_DSP_IRQ7: i32 = 28;
pub const MADERA_IRQ_DSP_IRQ8: i32 = 29;
pub const MADERA_IRQ_DSP_IRQ9: i32 = 30;
pub const MADERA_IRQ_DSP_IRQ10: i32 = 31;
pub const MADERA_IRQ_DSP_IRQ11: i32 = 32;
pub const MADERA_IRQ_DSP_IRQ12: i32 = 33;
pub const MADERA_IRQ_DSP_IRQ13: i32 = 34;
pub const MADERA_IRQ_DSP_IRQ14: i32 = 35;
pub const MADERA_IRQ_DSP_IRQ15: i32 = 36;
pub const MADERA_IRQ_DSP_IRQ16: i32 = 37;
pub const MADERA_IRQ_HP1L_SC: i32 = 38;
pub const MADERA_IRQ_HP1R_SC: i32 = 39;
pub const MADERA_IRQ_HP2L_SC: i32 = 40;
pub const MADERA_IRQ_HP2R_SC: i32 = 41;
pub const MADERA_IRQ_HP3L_SC: i32 = 42;
pub const MADERA_IRQ_HP3R_SC: i32 = 43;
pub const MADERA_IRQ_SPKOUTL_SC: i32 = 44;
pub const MADERA_IRQ_SPKOUTR_SC: i32 = 45;
pub const MADERA_IRQ_HP1L_ENABLE_DONE: i32 = 46;
pub const MADERA_IRQ_HP1R_ENABLE_DONE: i32 = 47;
pub const MADERA_IRQ_HP2L_ENABLE_DONE: i32 = 48;
pub const MADERA_IRQ_HP2R_ENABLE_DONE: i32 = 49;
pub const MADERA_IRQ_HP3L_ENABLE_DONE: i32 = 50;
pub const MADERA_IRQ_HP3R_ENABLE_DONE: i32 = 51;
pub const MADERA_IRQ_SPKOUTL_ENABLE_DONE: i32 = 52;
pub const MADERA_IRQ_SPKOUTR_ENABLE_DONE: i32 = 53;
pub const MADERA_IRQ_SPK_SHUTDOWN: i32 = 54;
pub const MADERA_IRQ_SPK_OVERHEAT: i32 = 55;
pub const MADERA_IRQ_SPK_OVERHEAT_WARN: i32 = 56;
pub const MADERA_IRQ_GPIO1: i32 = 57;
pub const MADERA_IRQ_GPIO2: i32 = 58;
pub const MADERA_IRQ_GPIO3: i32 = 59;
pub const MADERA_IRQ_GPIO4: i32 = 60;
pub const MADERA_IRQ_GPIO5: i32 = 61;
pub const MADERA_IRQ_GPIO6: i32 = 62;
pub const MADERA_IRQ_GPIO7: i32 = 63;
pub const MADERA_IRQ_GPIO8: i32 = 64;
pub const MADERA_IRQ_DSP1_BUS_ERR: i32 = 65;
pub const MADERA_IRQ_DSP2_BUS_ERR: i32 = 66;
pub const MADERA_IRQ_DSP3_BUS_ERR: i32 = 67;
pub const MADERA_IRQ_DSP4_BUS_ERR: i32 = 68;
pub const MADERA_IRQ_DSP5_BUS_ERR: i32 = 69;
pub const MADERA_IRQ_DSP6_BUS_ERR: i32 = 70;
pub const MADERA_IRQ_DSP7_BUS_ERR: i32 = 71;

pub const MADERA_NUM_IRQ: i32 = 72;

/* These wrapper functions are for use by other child drivers of the same parent MFD. */
pub unsafe fn madera_get_irq_mapping(madera: *mut madera, irq: i32) -> i32 {
    if (*madera).irq_dev.is_null() {
        return -ENODEV;
    }
    regmap_irq_get_virq((*madera).irq_data, irq)
}

pub unsafe fn madera_request_irq(
    madera: *mut madera,
    mut irq: i32,
    name: *const core::ffi::c_char,
    handler: irq_handler_t,
    data: *mut core::ffi::c_void,
) -> i32 {
    irq = madera_get_irq_mapping(madera, irq);
    if irq < 0 {
        return irq;
    }
    request_threaded_irq(irq, core::ptr::null_mut(), handler, IRQF_ONESHOT, name, data)
}

pub unsafe fn madera_free_irq(madera: *mut madera, mut irq: i32, data: *mut core::ffi::c_void) {
    irq = madera_get_irq_mapping(madera, irq);
    if irq < 0 {
        return;
    }
    free_irq(irq, data);
}

pub unsafe fn madera_set_irq_wake(madera: *mut madera, mut irq: i32, on: i32) -> i32 {
    irq = madera_get_irq_mapping(madera, irq);
    if irq < 0 {
        return irq;
    }
    irq_set_irq_wake(irq, on)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
