/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * This header provides constants for binding aspeed,*-gpio.
 *
 * The first cell in Aspeed's GPIO specifier is the GPIO ID. The macros below
 * provide names for this.
 *
 * The second cell contains standard flag values specified in gpio.h.
 */

// Dependency intent: <dt-bindings/gpio/gpio.h> is supplied by other bindings.

pub const ASPEED_GPIO_PORT_A: i32 = 0;
pub const ASPEED_GPIO_PORT_B: i32 = 1;
pub const ASPEED_GPIO_PORT_C: i32 = 2;
pub const ASPEED_GPIO_PORT_D: i32 = 3;
pub const ASPEED_GPIO_PORT_E: i32 = 4;
pub const ASPEED_GPIO_PORT_F: i32 = 5;
pub const ASPEED_GPIO_PORT_G: i32 = 6;
pub const ASPEED_GPIO_PORT_H: i32 = 7;
pub const ASPEED_GPIO_PORT_I: i32 = 8;
pub const ASPEED_GPIO_PORT_J: i32 = 9;
pub const ASPEED_GPIO_PORT_K: i32 = 10;
pub const ASPEED_GPIO_PORT_L: i32 = 11;
pub const ASPEED_GPIO_PORT_M: i32 = 12;
pub const ASPEED_GPIO_PORT_N: i32 = 13;
pub const ASPEED_GPIO_PORT_O: i32 = 14;
pub const ASPEED_GPIO_PORT_P: i32 = 15;
pub const ASPEED_GPIO_PORT_Q: i32 = 16;
pub const ASPEED_GPIO_PORT_R: i32 = 17;
pub const ASPEED_GPIO_PORT_S: i32 = 18;
pub const ASPEED_GPIO_PORT_T: i32 = 19;
pub const ASPEED_GPIO_PORT_U: i32 = 20;
pub const ASPEED_GPIO_PORT_V: i32 = 21;
pub const ASPEED_GPIO_PORT_W: i32 = 22;
pub const ASPEED_GPIO_PORT_X: i32 = 23;
pub const ASPEED_GPIO_PORT_Y: i32 = 24;
pub const ASPEED_GPIO_PORT_Z: i32 = 25;
pub const ASPEED_GPIO_PORT_AA: i32 = 26;
pub const ASPEED_GPIO_PORT_AB: i32 = 27;
pub const ASPEED_GPIO_PORT_AC: i32 = 28;

#[inline]
pub const fn ASPEED_GPIO(port: i32, offset: i32) -> i32 {
    (port * 8) + offset
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
