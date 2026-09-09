// SPDX-License-Identifier: GPL-2.0-only
/*
 * Joshua Henderson <joshua.henderson@microchip.com>
 * Copyright (C) 2015 Microchip Technology Inc.  All rights reserved.
 */

use core::ffi::c_void;

// Dependency supplied by asm/io.h.
extern "C" {
    fn ioremap(phys_addr: usize, size: usize) -> *mut c_void;
    fn iounmap(addr: *mut c_void);
    fn __raw_writel(value: u32, addr: *mut c_void);
}

const PPS_BASE: usize = 0x1f800000;

const INT1R: usize = 0x1404; const INT2R: usize = 0x1408; const INT3R: usize = 0x140c; const INT4R: usize = 0x1410;
const T2CKR: usize = 0x1418; const T3CKR: usize = 0x141c; const T4CKR: usize = 0x1420; const T5CKR: usize = 0x1424;
const T6CKR: usize = 0x1428; const T7CKR: usize = 0x142c; const T8CKR: usize = 0x1430; const T9CKR: usize = 0x1434;
const IC1R: usize = 0x1438; const IC2R: usize = 0x143c; const IC3R: usize = 0x1440; const IC4R: usize = 0x1444;
const IC5R: usize = 0x1448; const IC6R: usize = 0x144c; const IC7R: usize = 0x1450; const IC8R: usize = 0x1454; const IC9R: usize = 0x1458;
const OCFAR: usize = 0x1460; const U1RXR: usize = 0x1468; const U1CTSR: usize = 0x146c; const U2RXR: usize = 0x1470; const U2CTSR: usize = 0x1474;
const U3RXR: usize = 0x1478; const U3CTSR: usize = 0x147c; const U4RXR: usize = 0x1480; const U4CTSR: usize = 0x1484;
const U5RXR: usize = 0x1488; const U5CTSR: usize = 0x148c; const U6RXR: usize = 0x1490; const U6CTSR: usize = 0x1494;
const SDI1R: usize = 0x149c; const SS1R: usize = 0x14a0; const SDI2R: usize = 0x14a8; const SS2R: usize = 0x14ac;
const SDI3R: usize = 0x14b4; const SS3R: usize = 0x14b8; const SDI4R: usize = 0x14c0; const SS4R: usize = 0x14c4;
const SDI5R: usize = 0x14cc; const SS5R: usize = 0x14d0; const SDI6R: usize = 0x14d8; const SS6R: usize = 0x14dc;
const C1RXR: usize = 0x14e0; const C2RXR: usize = 0x14e4; const REFCLKI1R: usize = 0x14e8; const REFCLKI3R: usize = 0x14f0; const REFCLKI4R: usize = 0x14f4;

struct PinReg { function: i32, reg: usize }

static INPUT_PIN_REG: &[PinReg] = &[
    PinReg { function: IN_FUNC_INT3, reg: INT3R }, PinReg { function: IN_FUNC_T2CK, reg: T2CKR }, PinReg { function: IN_FUNC_T6CK, reg: T6CKR }, PinReg { function: IN_FUNC_IC3, reg: IC3R }, PinReg { function: IN_FUNC_IC7, reg: IC7R },
    PinReg { function: IN_FUNC_U1RX, reg: U1RXR }, PinReg { function: IN_FUNC_U2CTS, reg: U2CTSR }, PinReg { function: IN_FUNC_U5RX, reg: U5RXR }, PinReg { function: IN_FUNC_U6CTS, reg: U6CTSR }, PinReg { function: IN_FUNC_SDI1, reg: SDI1R },
    PinReg { function: IN_FUNC_SDI3, reg: SDI3R }, PinReg { function: IN_FUNC_SDI5, reg: SDI5R }, PinReg { function: IN_FUNC_SS6, reg: SS6R }, PinReg { function: IN_FUNC_REFCLKI1, reg: REFCLKI1R }, PinReg { function: IN_FUNC_INT4, reg: INT4R },
    PinReg { function: IN_FUNC_T5CK, reg: T5CKR }, PinReg { function: IN_FUNC_T7CK, reg: T7CKR }, PinReg { function: IN_FUNC_IC4, reg: IC4R }, PinReg { function: IN_FUNC_IC8, reg: IC8R }, PinReg { function: IN_FUNC_U3RX, reg: U3RXR },
    PinReg { function: IN_FUNC_U4CTS, reg: U4CTSR }, PinReg { function: IN_FUNC_SDI2, reg: SDI2R }, PinReg { function: IN_FUNC_SDI4, reg: SDI4R }, PinReg { function: IN_FUNC_C1RX, reg: C1RXR }, PinReg { function: IN_FUNC_REFCLKI4, reg: REFCLKI4R },
    PinReg { function: IN_FUNC_INT2, reg: INT2R }, PinReg { function: IN_FUNC_T3CK, reg: T3CKR }, PinReg { function: IN_FUNC_T8CK, reg: T8CKR }, PinReg { function: IN_FUNC_IC2, reg: IC2R }, PinReg { function: IN_FUNC_IC5, reg: IC5R },
    PinReg { function: IN_FUNC_IC9, reg: IC9R }, PinReg { function: IN_FUNC_U1CTS, reg: U1CTSR }, PinReg { function: IN_FUNC_U2RX, reg: U2RXR }, PinReg { function: IN_FUNC_U5CTS, reg: U5CTSR }, PinReg { function: IN_FUNC_SS1, reg: SS1R },
    PinReg { function: IN_FUNC_SS3, reg: SS3R }, PinReg { function: IN_FUNC_SS4, reg: SS4R }, PinReg { function: IN_FUNC_SS5, reg: SS5R }, PinReg { function: IN_FUNC_C2RX, reg: C2RXR }, PinReg { function: IN_FUNC_INT1, reg: INT1R },
    PinReg { function: IN_FUNC_T4CK, reg: T4CKR }, PinReg { function: IN_FUNC_T9CK, reg: T9CKR }, PinReg { function: IN_FUNC_IC1, reg: IC1R }, PinReg { function: IN_FUNC_IC6, reg: IC6R }, PinReg { function: IN_FUNC_U3CTS, reg: U3CTSR },
    PinReg { function: IN_FUNC_U4RX, reg: U4RXR }, PinReg { function: IN_FUNC_U6RX, reg: U6RXR }, PinReg { function: IN_FUNC_SS2, reg: SS2R }, PinReg { function: IN_FUNC_SDI6, reg: SDI6R }, PinReg { function: IN_FUNC_OCFA, reg: OCFAR }, PinReg { function: IN_FUNC_REFCLKI3, reg: REFCLKI3R },
];

pub unsafe fn pic32_pps_input(function: i32, pin: i32) {
    let pps_base = ioremap(PPS_BASE, 0xF4);
    for entry in INPUT_PIN_REG {
        if entry.function == function {
            __raw_writel(pin as u32, pps_base.add(entry.reg));
            return;
        }
    }
    iounmap(pps_base);
}

const RPA14R: usize = 0x1538; const RPA15R: usize = 0x153c; const RPB0R: usize = 0x1540; const RPB1R: usize = 0x1544; const RPB2R: usize = 0x1548; const RPB3R: usize = 0x154c; const RPB5R: usize = 0x1554; const RPB6R: usize = 0x1558; const RPB7R: usize = 0x155c; const RPB8R: usize = 0x1560; const RPB9R: usize = 0x1564; const RPB10R: usize = 0x1568; const RPB14R: usize = 0x1578; const RPB15R: usize = 0x157c;
const RPC1R: usize = 0x1584; const RPC2R: usize = 0x1588; const RPC3R: usize = 0x158c; const RPC4R: usize = 0x1590; const RPC13R: usize = 0x15b4; const RPC14R: usize = 0x15b8;
const RPD0R: usize = 0x15c0; const RPD1R: usize = 0x15c4; const RPD2R: usize = 0x15c8; const RPD3R: usize = 0x15cc; const RPD4R: usize = 0x15d0; const RPD5R: usize = 0x15d4; const RPD6R: usize = 0x15d8; const RPD7R: usize = 0x15dc; const RPD9R: usize = 0x15e4; const RPD10R: usize = 0x15e8; const RPD11R: usize = 0x15ec; const RPD12R: usize = 0x15f0; const RPD14R: usize = 0x15f8; const RPD15R: usize = 0x15fc;
const RPE3R: usize = 0x160c; const RPE5R: usize = 0x1614; const RPE8R: usize = 0x1620; const RPE9R: usize = 0x1624; const RPF0R: usize = 0x1640; const RPF1R: usize = 0x1644; const RPF2R: usize = 0x1648; const RPF3R: usize = 0x164c; const RPF4R: usize = 0x1650; const RPF5R: usize = 0x1654; const RPF8R: usize = 0x1660; const RPF12R: usize = 0x1670; const RPF13R: usize = 0x1674; const RPG0R: usize = 0x1680; const RPG1R: usize = 0x1684; const RPG6R: usize = 0x1698; const RPG7R: usize = 0x169c; const RPG8R: usize = 0x16a0; const RPG9R: usize = 0x16a4;

struct OutputPinReg { pin: i32, reg: usize }
static OUTPUT_PIN_REG: &[OutputPinReg] = &[
    OutputPinReg { pin: OUT_RPD2, reg: RPD2R }, OutputPinReg { pin: OUT_RPG8, reg: RPG8R }, OutputPinReg { pin: OUT_RPF4, reg: RPF4R }, OutputPinReg { pin: OUT_RPD10, reg: RPD10R }, OutputPinReg { pin: OUT_RPF1, reg: RPF1R }, OutputPinReg { pin: OUT_RPB9, reg: RPB9R }, OutputPinReg { pin: OUT_RPB10, reg: RPB10R }, OutputPinReg { pin: OUT_RPC14, reg: RPC14R }, OutputPinReg { pin: OUT_RPB5, reg: RPB5R }, OutputPinReg { pin: OUT_RPC1, reg: RPC1R }, OutputPinReg { pin: OUT_RPD14, reg: RPD14R }, OutputPinReg { pin: OUT_RPG1, reg: RPG1R }, OutputPinReg { pin: OUT_RPA14, reg: RPA14R }, OutputPinReg { pin: OUT_RPD6, reg: RPD6R }, OutputPinReg { pin: OUT_RPD3, reg: RPD3R }, OutputPinReg { pin: OUT_RPG7, reg: RPG7R }, OutputPinReg { pin: OUT_RPF5, reg: RPF5R }, OutputPinReg { pin: OUT_RPD11, reg: RPD11R }, OutputPinReg { pin: OUT_RPF0, reg: RPF0R }, OutputPinReg { pin: OUT_RPB1, reg: RPB1R }, OutputPinReg { pin: OUT_RPE5, reg: RPE5R }, OutputPinReg { pin: OUT_RPC13, reg: RPC13R }, OutputPinReg { pin: OUT_RPB3, reg: RPB3R }, OutputPinReg { pin: OUT_RPC4, reg: RPC4R }, OutputPinReg { pin: OUT_RPD15, reg: RPD15R }, OutputPinReg { pin: OUT_RPG0, reg: RPG0R }, OutputPinReg { pin: OUT_RPA15, reg: RPA15R }, OutputPinReg { pin: OUT_RPD7, reg: RPD7R }, OutputPinReg { pin: OUT_RPD9, reg: RPD9R }, OutputPinReg { pin: OUT_RPG6, reg: RPG6R }, OutputPinReg { pin: OUT_RPB8, reg: RPB8R }, OutputPinReg { pin: OUT_RPB15, reg: RPB15R }, OutputPinReg { pin: OUT_RPD4, reg: RPD4R }, OutputPinReg { pin: OUT_RPB0, reg: RPB0R }, OutputPinReg { pin: OUT_RPE3, reg: RPE3R }, OutputPinReg { pin: OUT_RPB7, reg: RPB7R }, OutputPinReg { pin: OUT_RPF12, reg: RPF12R }, OutputPinReg { pin: OUT_RPD12, reg: RPD12R }, OutputPinReg { pin: OUT_RPF8, reg: RPF8R }, OutputPinReg { pin: OUT_RPC3, reg: RPC3R }, OutputPinReg { pin: OUT_RPE9, reg: RPE9R }, OutputPinReg { pin: OUT_RPD1, reg: RPD1R }, OutputPinReg { pin: OUT_RPG9, reg: RPG9R }, OutputPinReg { pin: OUT_RPB14, reg: RPB14R }, OutputPinReg { pin: OUT_RPD0, reg: RPD0R }, OutputPinReg { pin: OUT_RPB6, reg: RPB6R }, OutputPinReg { pin: OUT_RPD5, reg: RPD5R }, OutputPinReg { pin: OUT_RPB2, reg: RPB2R }, OutputPinReg { pin: OUT_RPF3, reg: RPF3R }, OutputPinReg { pin: OUT_RPF13, reg: RPF13R }, OutputPinReg { pin: OUT_RPC2, reg: RPC2R }, OutputPinReg { pin: OUT_RPE8, reg: RPE8R }, OutputPinReg { pin: OUT_RPF2, reg: RPF2R },
];

pub unsafe fn pic32_pps_output(function: i32, pin: i32) {
    let pps_base = ioremap(PPS_BASE, 0x170);
    for entry in OUTPUT_PIN_REG {
        if entry.pin == pin {
            __raw_writel(function as u32, pps_base.add(entry.reg));
            return;
        }
    }
    iounmap(pps_base);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
