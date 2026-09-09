/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 *  Copyright (C) 2010 John Crispin <john@phrozen.org>
 */

// C header guard: _LANTIQ_XWAY_IRQ_H__

pub const INT_NUM_IRQ0: i32 = 8;
pub const INT_NUM_IM0_IRL0: i32 = INT_NUM_IRQ0 + 0;
pub const INT_NUM_IM1_IRL0: i32 = INT_NUM_IRQ0 + 32;
pub const INT_NUM_IM2_IRL0: i32 = INT_NUM_IRQ0 + 64;
pub const INT_NUM_IM3_IRL0: i32 = INT_NUM_IRQ0 + 96;
pub const INT_NUM_IM4_IRL0: i32 = INT_NUM_IRQ0 + 128;
pub const INT_NUM_IM_OFFSET: i32 = INT_NUM_IM1_IRL0 - INT_NUM_IM0_IRL0;

pub const LTQ_DMA_CH0_INT: i32 = INT_NUM_IM2_IRL0;

pub const MAX_IM: i32 = 5;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
