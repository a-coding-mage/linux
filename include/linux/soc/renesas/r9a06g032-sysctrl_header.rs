/* SPDX-License-Identifier: GPL-2.0 */

// CONFIG_CLK_R9A06G032 selects the implementation provided by the build.
#[cfg(CONFIG_CLK_R9A06G032)]
extern "C" {
    pub fn r9a06g032_sysctrl_set_dmamux(mask: u32, val: u32) -> i32;
}

#[cfg(not(CONFIG_CLK_R9A06G032))]
pub fn r9a06g032_sysctrl_set_dmamux(_mask: u32, _val: u32) -> i32 {
    -ENODEV
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
