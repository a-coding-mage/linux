/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Defines macros and constants for Renesas RZ/A2 pin controller pin
 * muxing functions.
 */

pub const RZA2_PINS_PER_PORT: i32 = 8;

/* Port names as labeled in the Hardware Manual */
pub const PORT0: i32 = 0;
pub const PORT1: i32 = 1;
pub const PORT2: i32 = 2;
pub const PORT3: i32 = 3;
pub const PORT4: i32 = 4;
pub const PORT5: i32 = 5;
pub const PORT6: i32 = 6;
pub const PORT7: i32 = 7;
pub const PORT8: i32 = 8;
pub const PORT9: i32 = 9;
pub const PORTA: i32 = 10;
pub const PORTB: i32 = 11;
pub const PORTC: i32 = 12;
pub const PORTD: i32 = 13;
pub const PORTE: i32 = 14;
pub const PORTF: i32 = 15;
pub const PORTG: i32 = 16;
pub const PORTH: i32 = 17;
/* No I */
pub const PORTJ: i32 = 18;
pub const PORTK: i32 = 19;
pub const PORTL: i32 = 20;
pub const PORTM: i32 = 21; /* Pins PM_0/1 are labeled JP_0/1 in HW manual */

/*
 * Create the pin index from its bank and position numbers and store in
 * the upper 16 bits the alternate function identifier
 */
#[inline]
pub const fn RZA2_PINMUX(b: i32, p: i32, f: i32) -> i32 {
    (b * RZA2_PINS_PER_PORT + p) | (f << 16)
}

/*
 * Convert a port and pin label to its global pin index
 */
#[inline]
pub const fn RZA2_PIN(port: i32, pin: i32) -> i32 {
    port * RZA2_PINS_PER_PORT + pin
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
