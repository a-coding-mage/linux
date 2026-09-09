/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Copyright (c) 2012 Samsung Electronics.
 *
 * Exynos - SMC Call
 */

pub const SMC_CMD_INIT: i32 = -1;
pub const SMC_CMD_INFO: i32 = -2;
/* For Power Management */
pub const SMC_CMD_SLEEP: i32 = -3;
pub const SMC_CMD_CPU1BOOT: i32 = -4;
pub const SMC_CMD_CPU0AFTR: i32 = -5;
pub const SMC_CMD_SAVE: i32 = -6;
pub const SMC_CMD_SHUTDOWN: i32 = -7;
/* For CP15 Access */
pub const SMC_CMD_C15RESUME: i32 = -11;
/* For L2 Cache Access */
pub const SMC_CMD_L2X0CTRL: i32 = -21;
pub const SMC_CMD_L2X0SETUP1: i32 = -22;
pub const SMC_CMD_L2X0SETUP2: i32 = -23;
pub const SMC_CMD_L2X0INVALL: i32 = -24;
pub const SMC_CMD_L2X0DEBUG: i32 = -25;

/* For Accessing CP15/SFR (General) */
pub const SMC_CMD_REG: i32 = -101;

/* defines for SMC_CMD_REG */
pub const SMC_REG_CLASS_SFR_W: u32 = 0x1u32 << 30;

#[inline]
pub const fn SMC_REG_ID_SFR_W(addr: u32) -> u32 {
    SMC_REG_CLASS_SFR_W | (addr >> 2)
}

extern "C" {
    pub fn exynos_smc(cmd: u32, arg1: u32, arg2: u32, arg3: u32);
}

/* op type for SMC_CMD_SAVE and SMC_CMD_SHUTDOWN */
pub const OP_TYPE_CORE: u32 = 0x0;
pub const OP_TYPE_CLUSTER: u32 = 0x1;

/* Power State required for SMC_CMD_SAVE and SMC_CMD_SHUTDOWN */
pub const SMC_POWERSTATE_IDLE: u32 = 0x1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
