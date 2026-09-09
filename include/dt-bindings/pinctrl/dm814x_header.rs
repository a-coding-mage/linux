/* SPDX-License-Identifier: GPL-2.0 */
/*
 * This header provides constants specific to DM814X pinctrl bindings.
 */

// Dependency: <dt-bindings/pinctrl/omap.h>
// The following symbols replace the corresponding definitions from omap.h.

// INPUT_EN, PULL_UP, and PULL_ENA are undefined here before being redefined.

/*
 * Note that dm814x silicon revision 2.1 and older require input enabled
 * (bit 18 set) for all 3.3V I/Os to avoid cumulative hardware damage. For
 * more info, see errata advisory 2.1.87. We leave bit 18 out of
 * function-mask in dm814x.h and rely on the bootloader for it.
 */
pub const INPUT_EN: i32 = 1 << 18;
pub const PULL_UP: i32 = 1 << 17;
pub const PULL_DISABLE: i32 = 1 << 16;

// Update macros depending on INPUT_EN and PULL_ENA.
// PIN_OUTPUT, PIN_OUTPUT_PULLUP, PIN_OUTPUT_PULLDOWN, PIN_INPUT,
// PIN_INPUT_PULLUP, and PIN_INPUT_PULLDOWN replace the corresponding
// definitions from omap.h.
pub const PIN_OUTPUT: i32 = PULL_DISABLE;
pub const PIN_OUTPUT_PULLUP: i32 = PULL_UP;
pub const PIN_OUTPUT_PULLDOWN: i32 = 0;
pub const PIN_INPUT: i32 = INPUT_EN | PULL_DISABLE;
pub const PIN_INPUT_PULLUP: i32 = INPUT_EN | PULL_UP;
pub const PIN_INPUT_PULLDOWN: i32 = INPUT_EN;

// Non-existing modes undefined here: PIN_OFF_NONE, PIN_OFF_OUTPUT_HIGH,
// PIN_OFF_OUTPUT_LOW, PIN_OFF_INPUT_PULLUP, PIN_OFF_INPUT_PULLDOWN, and
// PIN_OFF_WAKEUPENABLE.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
