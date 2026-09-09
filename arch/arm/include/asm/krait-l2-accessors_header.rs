/* SPDX-License-Identifier: GPL-2.0 */

unsafe extern "C" {
    pub fn krait_set_l2_indirect_reg(addr: u32, val: u32);
    pub fn krait_get_l2_indirect_reg(addr: u32) -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
