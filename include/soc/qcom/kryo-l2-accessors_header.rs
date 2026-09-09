/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2018, The Linux Foundation. All rights reserved.
 */

unsafe extern "C" {
    pub fn kryo_l2_set_indirect_reg(reg: u64, val: u64);
    pub fn kryo_l2_get_indirect_reg(reg: u64) -> u64;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
