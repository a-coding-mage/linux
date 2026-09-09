/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * wm8960.h  --  WM8960 Soc Audio driver platform data
 */

pub const WM8960_DRES_400R: u32 = 0;
pub const WM8960_DRES_200R: u32 = 1;
pub const WM8960_DRES_600R: u32 = 2;
pub const WM8960_DRES_150R: u32 = 3;
pub const WM8960_DRES_MAX: u32 = 3;

#[repr(C)]
pub struct wm8960_data {
    pub capless: bool, /* Headphone outputs configured in capless mode */

    pub shared_lrclk: bool, /* DAC and ADC LRCLKs are wired together */

    /*
     * Setup for headphone detection
     *
     * hp_cfg[0]: HPSEL[1:0] of R48 (Additional Control 4)
     * hp_cfg[1]: {HPSWEN:HPSWPOL} of R24 (Additional Control 2).
     * hp_cfg[2]: {TOCLKSEL:TOEN} of R23 (Additional Control 1).
     */
    pub hp_cfg: [u32; 3],

    /*
     * Setup for gpio configuration
     *
     * gpio_cfg[0]: ALRCGPIO of R9 (Audio interface)
     * gpio_cfg[1]: {GPIOPOL:GPIOSEL[2:0]} of R48 (Additional Control 4).
     */
    pub gpio_cfg: [u32; 2],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
