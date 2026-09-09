/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/arm/probes/kprobes/checkers.h
 *
 * Copyright (C) 2014 Huawei Inc.
 */

// C dependencies:
// #include <linux/kernel.h>
// #include <linux/types.h>
// #include "../decode.h"

extern "C" {
    pub static mut checker_stack_use_none: probes_check_t;
    pub static mut checker_stack_use_unknown: probes_check_t;
    #[cfg(feature = "CONFIG_THUMB2_KERNEL")]
    pub static mut checker_stack_use_imm_0xx: probes_check_t;
    #[cfg(not(feature = "CONFIG_THUMB2_KERNEL"))]
    pub static mut checker_stack_use_imm_x0x: probes_check_t;
    pub static mut checker_stack_use_imm_xxx: probes_check_t;
    pub static mut checker_stack_use_stmdx: probes_check_t;
}

#[repr(C)]
pub enum StackUseType {
    STACK_USE_NONE,
    STACK_USE_UNKNOWN,
    #[cfg(feature = "CONFIG_THUMB2_KERNEL")]
    STACK_USE_FIXED_0XX,
    #[cfg(feature = "CONFIG_THUMB2_KERNEL")]
    STACK_USE_T32STRD,
    #[cfg(not(feature = "CONFIG_THUMB2_KERNEL"))]
    STACK_USE_FIXED_X0X,
    STACK_USE_FIXED_XXX,
    STACK_USE_STMDX,
    NUM_STACK_USE_TYPES,
}

extern "C" {
    pub static stack_check_actions: [decode_action; 0];

    #[cfg(not(feature = "CONFIG_THUMB2_KERNEL"))]
    pub static arm_stack_checker: [decode_checker; 0];
    #[cfg(not(feature = "CONFIG_THUMB2_KERNEL"))]
    pub static arm_regs_checker: [decode_checker; 0];

    pub static t32_stack_checker: [decode_checker; 0];
    pub static t16_stack_checker: [decode_checker; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
