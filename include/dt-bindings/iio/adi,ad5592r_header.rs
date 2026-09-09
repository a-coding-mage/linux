/* SPDX-License-Identifier: GPL-2.0 */

// Header guard: _DT_BINDINGS_ADI_AD5592R_H

pub const CH_MODE_UNUSED: i32 = 0;
pub const CH_MODE_ADC: i32 = 1;
pub const CH_MODE_DAC: i32 = 2;
pub const CH_MODE_DAC_AND_ADC: i32 = 3;
pub const CH_MODE_GPIO: i32 = 8;

pub const CH_OFFSTATE_PULLDOWN: i32 = 0;
pub const CH_OFFSTATE_OUT_LOW: i32 = 1;
pub const CH_OFFSTATE_OUT_HIGH: i32 = 2;
pub const CH_OFFSTATE_OUT_TRISTATE: i32 = 3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
