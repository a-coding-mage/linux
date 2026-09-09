/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies:
// #include <linux/init.h>
// #include <bcm63xx_cpu.h>

unsafe extern "C" {
    pub fn bcm63xx_gpio_init() -> ::core::ffi::c_int;
}

#[inline]
pub unsafe fn bcm63xx_gpio_count() -> ::core::ffi::c_ulong {
    match bcm63xx_get_cpu_id() {
        BCM6328_CPU_ID => 32,
        BCM3368_CPU_ID => 40,
        BCM6338_CPU_ID => 8,
        BCM6345_CPU_ID => 16,
        BCM6358_CPU_ID | BCM6368_CPU_ID => 38,
        BCM6362_CPU_ID => 48,
        BCM6348_CPU_ID => 37,
        _ => 37,
    }
}

pub const BCM63XX_GPIO_DIR_OUT: ::core::ffi::c_uint = 0x0;
pub const BCM63XX_GPIO_DIR_IN: ::core::ffi::c_uint = 0x1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
