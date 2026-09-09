/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  include/linux/irqchip/irq-st.h
 *
 *  Copyright (C) 2014 STMicroelectronics – All Rights Reserved
 *
 *  Author: Lee Jones <lee.jones@linaro.org>
 */

// C header guard: _DT_BINDINGS_INTERRUPT_CONTROLLER_ST_H

pub const ST_IRQ_SYSCFG_EXT_0: u32 = 0;
pub const ST_IRQ_SYSCFG_EXT_1: u32 = 1;
pub const ST_IRQ_SYSCFG_EXT_2: u32 = 2;
pub const ST_IRQ_SYSCFG_CTI_0: u32 = 3;
pub const ST_IRQ_SYSCFG_CTI_1: u32 = 4;
pub const ST_IRQ_SYSCFG_PMU_0: u32 = 5;
pub const ST_IRQ_SYSCFG_PMU_1: u32 = 6;
pub const ST_IRQ_SYSCFG_pl310_L2: u32 = 7;
pub const ST_IRQ_SYSCFG_DISABLED: u32 = 0xFFFF_FFFF;

pub const ST_IRQ_SYSCFG_EXT_1_INV: u32 = 0x1;
pub const ST_IRQ_SYSCFG_EXT_2_INV: u32 = 0x2;
pub const ST_IRQ_SYSCFG_EXT_3_INV: u32 = 0x4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
