/* SPDX-License-Identifier: GPL-2.0 */

// Translation of the Linux kernel private BCM47xx header.
// The original `__init` annotations are linker/build-time attributes and have
// no direct Rust equivalent.

// Equivalent of the C preprocessor formatting macro:
// #define pr_fmt(fmt) "bcm47xx: " fmt
macro_rules! pr_fmt {
    ($fmt:literal) => {
        concat!("bcm47xx: ", $fmt)
    };
}

/* prom.c */
extern "C" {
    pub fn bcm47xx_prom_highmem_init();
}

/* buttons.c */
extern "C" {
    pub fn bcm47xx_buttons_register() -> ::core::ffi::c_int;
}

/* leds.c */
extern "C" {
    pub fn bcm47xx_leds_register();
}

/* setup.c */
extern "C" {
    pub fn bcm47xx_bus_setup();
}

/* workarounds.c */
extern "C" {
    pub fn bcm47xx_workarounds();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
