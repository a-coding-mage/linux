/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2015-2017  Dialog Semiconductor
 */

// Dependencies supplied by the corresponding Linux headers:
// linux/interrupt.h and linux/mfd/da9062/registers.h

#[repr(C)]
pub enum da9062_compatible_types {
    COMPAT_TYPE_DA9061 = 1,
    COMPAT_TYPE_DA9062,
}

#[repr(C)]
pub enum da9061_irqs {
    /* IRQ A */
    DA9061_IRQ_ONKEY,
    DA9061_IRQ_WDG_WARN,
    DA9061_IRQ_SEQ_RDY,
    /* IRQ B*/
    DA9061_IRQ_TEMP,
    DA9061_IRQ_LDO_LIM,
    DA9061_IRQ_DVC_RDY,
    DA9061_IRQ_VDD_WARN,
    /* IRQ C */
    DA9061_IRQ_GPI0,
    DA9061_IRQ_GPI1,
    DA9061_IRQ_GPI2,
    DA9061_IRQ_GPI3,
    DA9061_IRQ_GPI4,

    DA9061_NUM_IRQ,
}

#[repr(C)]
pub enum da9062_irqs {
    /* IRQ A */
    DA9062_IRQ_ONKEY,
    DA9062_IRQ_ALARM,
    DA9062_IRQ_TICK,
    DA9062_IRQ_WDG_WARN,
    DA9062_IRQ_SEQ_RDY,
    /* IRQ B*/
    DA9062_IRQ_TEMP,
    DA9062_IRQ_LDO_LIM,
    DA9062_IRQ_DVC_RDY,
    DA9062_IRQ_VDD_WARN,
    /* IRQ C */
    DA9062_IRQ_GPI0,
    DA9062_IRQ_GPI1,
    DA9062_IRQ_GPI2,
    DA9062_IRQ_GPI3,
    DA9062_IRQ_GPI4,

    DA9062_NUM_IRQ,
}

#[repr(C)]
pub struct da9062 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub regmap_irq: *mut regmap_irq_chip_data,
    pub chip_type: da9062_compatible_types,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
