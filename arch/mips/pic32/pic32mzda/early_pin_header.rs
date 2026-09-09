/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Joshua Henderson <joshua.henderson@microchip.com>
 * Copyright (C) 2015 Microchip Technology Inc.  All rights reserved.
 */

/*
 * This is a complete, yet overly simplistic and unoptimized, PIC32MZDA PPS
 * configuration only useful before we have full pinctrl initialized.
 */

/* Input PPS Functions */
#[repr(i32)]
pub enum InputPpsFunction {
    IN_FUNC_INT3,
    IN_FUNC_T2CK,
    IN_FUNC_T6CK,
    IN_FUNC_IC3,
    IN_FUNC_IC7,
    IN_FUNC_U1RX,
    IN_FUNC_U2CTS,
    IN_FUNC_U5RX,
    IN_FUNC_U6CTS,
    IN_FUNC_SDI1,
    IN_FUNC_SDI3,
    IN_FUNC_SDI5,
    IN_FUNC_SS6,
    IN_FUNC_REFCLKI1,
    IN_FUNC_INT4,
    IN_FUNC_T5CK,
    IN_FUNC_T7CK,
    IN_FUNC_IC4,
    IN_FUNC_IC8,
    IN_FUNC_U3RX,
    IN_FUNC_U4CTS,
    IN_FUNC_SDI2,
    IN_FUNC_SDI4,
    IN_FUNC_C1RX,
    IN_FUNC_REFCLKI4,
    IN_FUNC_INT2,
    IN_FUNC_T3CK,
    IN_FUNC_T8CK,
    IN_FUNC_IC2,
    IN_FUNC_IC5,
    IN_FUNC_IC9,
    IN_FUNC_U1CTS,
    IN_FUNC_U2RX,
    IN_FUNC_U5CTS,
    IN_FUNC_SS1,
    IN_FUNC_SS3,
    IN_FUNC_SS4,
    IN_FUNC_SS5,
    IN_FUNC_C2RX,
    IN_FUNC_INT1,
    IN_FUNC_T4CK,
    IN_FUNC_T9CK,
    IN_FUNC_IC1,
    IN_FUNC_IC6,
    IN_FUNC_U3CTS,
    IN_FUNC_U4RX,
    IN_FUNC_U6RX,
    IN_FUNC_SS2,
    IN_FUNC_SDI6,
    IN_FUNC_OCFA,
    IN_FUNC_REFCLKI3,
}

/* Input PPS Pins. Duplicate C macro names are retained as their final values. */
pub const IN_RPD2: i32 = 0x00;
pub const IN_RPG8: i32 = 0x01;
pub const IN_RPF4: i32 = 0x02;
pub const IN_RPD10: i32 = 0x03;
pub const IN_RPF1: i32 = 0x04;
pub const IN_RPB9: i32 = 0x05;
pub const IN_RPB10: i32 = 0x06;
pub const IN_RPC14: i32 = 0x07;
pub const IN_RPB5: i32 = 0x08;
pub const IN_RPC1: i32 = 0x0A;
pub const IN_RPD14: i32 = 0x0B;
pub const IN_RPG1: i32 = 0x0C;
pub const IN_RPA14: i32 = 0x0D;
pub const IN_RPD6: i32 = 0x0E;
pub const IN_RPD3: i32 = 0x00;
pub const IN_RPG7: i32 = 0x01;
pub const IN_RPF5: i32 = 0x02;
pub const IN_RPD11: i32 = 0x03;
pub const IN_RPF0: i32 = 0x04;
pub const IN_RPB1: i32 = 0x05;
pub const IN_RPE5: i32 = 0x06;
pub const IN_RPC13: i32 = 0x07;
pub const IN_RPB3: i32 = 0x08;
pub const IN_RPC4: i32 = 0x0A;
pub const IN_RPD15: i32 = 0x0B;
pub const IN_RPG0: i32 = 0x0C;
pub const IN_RPA15: i32 = 0x0D;
pub const IN_RPD7: i32 = 0x0E;
pub const IN_RPD9: i32 = 0x00;
pub const IN_RPG6: i32 = 0x01;
pub const IN_RPB8: i32 = 0x02;
pub const IN_RPB15: i32 = 0x03;
pub const IN_RPD4: i32 = 0x04;
pub const IN_RPB0: i32 = 0x05;
pub const IN_RPE3: i32 = 0x06;
pub const IN_RPB7: i32 = 0x07;
pub const IN_RPF12: i32 = 0x09;
pub const IN_RPD12: i32 = 0x0A;
pub const IN_RPF8: i32 = 0x0B;
pub const IN_RPC3: i32 = 0x0C;
pub const IN_RPE9: i32 = 0x0D;
pub const IN_RPD1: i32 = 0x00;
pub const IN_RPG9: i32 = 0x01;
pub const IN_RPB14: i32 = 0x02;
pub const IN_RPD0: i32 = 0x03;
pub const IN_RPB6: i32 = 0x05;
pub const IN_RPD5: i32 = 0x06;
pub const IN_RPB2: i32 = 0x07;
pub const IN_RPF3: i32 = 0x08;
pub const IN_RPF13: i32 = 0x09;
pub const IN_RPF2: i32 = 0x0B;
pub const IN_RPC2: i32 = 0x0C;
pub const IN_RPE8: i32 = 0x0D;

/* Output PPS Pins */
#[repr(i32)]
pub enum OutputPpsPin {
    OUT_RPD2,
    OUT_RPG8,
    OUT_RPF4,
    OUT_RPD10,
    OUT_RPF1,
    OUT_RPB9,
    OUT_RPB10,
    OUT_RPC14,
    OUT_RPB5,
    OUT_RPC1,
    OUT_RPD14,
    OUT_RPG1,
    OUT_RPA14,
    OUT_RPD6,
    OUT_RPD3,
    OUT_RPG7,
    OUT_RPF5,
    OUT_RPD11,
    OUT_RPF0,
    OUT_RPB1,
    OUT_RPE5,
    OUT_RPC13,
    OUT_RPB3,
    OUT_RPC4,
    OUT_RPD15,
    OUT_RPG0,
    OUT_RPA15,
    OUT_RPD7,
    OUT_RPD9,
    OUT_RPG6,
    OUT_RPB8,
    OUT_RPB15,
    OUT_RPD4,
    OUT_RPB0,
    OUT_RPE3,
    OUT_RPB7,
    OUT_RPF12,
    OUT_RPD12,
    OUT_RPF8,
    OUT_RPC3,
    OUT_RPE9,
    OUT_RPD1,
    OUT_RPG9,
    OUT_RPB14,
    OUT_RPD0,
    OUT_RPB6,
    OUT_RPD5,
    OUT_RPB2,
    OUT_RPF3,
    OUT_RPF13,
    OUT_RPC2,
    OUT_RPE8,
    OUT_RPF2,
}

/* Output PPS Functions (the final C macro definitions determine these names). */
pub const OUT_FUNC_U3TX: i32 = 0x01;
pub const OUT_FUNC_U4RTS: i32 = 0x02;
pub const OUT_FUNC_SDO1: i32 = 0x05;
pub const OUT_FUNC_SDO2: i32 = 0x06;
pub const OUT_FUNC_SDO3: i32 = 0x07;
pub const OUT_FUNC_SDO5: i32 = 0x09;
pub const OUT_FUNC_SS6: i32 = 0x0A;
pub const OUT_FUNC_OC3: i32 = 0x0B;
pub const OUT_FUNC_OC6: i32 = 0x0C;
pub const OUT_FUNC_REFCLKO4: i32 = 0x0D;
pub const OUT_FUNC_C2OUT: i32 = 0x0E;
pub const OUT_FUNC_C1TX: i32 = 0x0F;
pub const OUT_FUNC_U1TX: i32 = 0x01;
pub const OUT_FUNC_U2RTS: i32 = 0x02;
pub const OUT_FUNC_U5TX: i32 = 0x03;
pub const OUT_FUNC_U6RTS: i32 = 0x04;
pub const OUT_FUNC_SDO4: i32 = 0x08;
pub const OUT_FUNC_OC4: i32 = 0x0B;
pub const OUT_FUNC_OC7: i32 = 0x0C;
pub const OUT_FUNC_REFCLKO1: i32 = 0x0F;
pub const OUT_FUNC_U3RTS: i32 = 0x01;
pub const OUT_FUNC_U4TX: i32 = 0x02;
pub const OUT_FUNC_U6TX: i32 = 0x04;
pub const OUT_FUNC_SS1: i32 = 0x05;
pub const OUT_FUNC_SS3: i32 = 0x07;
pub const OUT_FUNC_SS4: i32 = 0x08;
pub const OUT_FUNC_SS5: i32 = 0x09;
pub const OUT_FUNC_SDO6: i32 = 0x0A;
pub const OUT_FUNC_OC5: i32 = 0x0B;
pub const OUT_FUNC_OC8: i32 = 0x0C;
pub const OUT_FUNC_C1OUT: i32 = 0x0E;
pub const OUT_FUNC_REFCLKO3: i32 = 0x0F;
pub const OUT_FUNC_U1RTS: i32 = 0x01;
pub const OUT_FUNC_U2TX: i32 = 0x02;
pub const OUT_FUNC_U5RTS: i32 = 0x03;
pub const OUT_FUNC_SS2: i32 = 0x06;
pub const OUT_FUNC_OC2: i32 = 0x0B;
pub const OUT_FUNC_OC1: i32 = 0x0C;
pub const OUT_FUNC_OC9: i32 = 0x0D;
pub const OUT_FUNC_C2TX: i32 = 0x0F;

unsafe extern "C" {
    pub fn pic32_pps_input(function: i32, pin: i32);
    pub fn pic32_pps_output(function: i32, pin: i32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
