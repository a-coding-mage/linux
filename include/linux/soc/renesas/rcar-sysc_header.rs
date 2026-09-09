/* SPDX-License-Identifier: GPL-2.0 */

extern "C" {
    pub fn rcar_sysc_power_down_cpu(cpu: u32) -> i32;
    pub fn rcar_sysc_power_up_cpu(cpu: u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
