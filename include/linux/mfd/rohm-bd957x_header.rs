/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright (C) 2021 ROHM Semiconductors */

#[repr(i32)]
pub enum Bd957xRegulator {
    BD957X_VD50,
    BD957X_VD18,
    BD957X_VDDDR,
    BD957X_VD10,
    BD957X_VOUTL1,
    BD957X_VOUTS1,
}

/*
 * The BD9576 has own IRQ 'blocks' for:
 *  - I2C/thermal,
 *  - Over voltage protection
 *  - Short-circuit protection
 *  - Over current protection
 *  - Over voltage detection
 *  - Under voltage detection
 *  - Under voltage protection
 *  - 'system interrupt'.
 *
 * Each of the blocks have a status register giving more accurate IRQ source
 * information - for example which of the regulators have over-voltage.
 *
 * On top of this, there is "main IRQ" status register where each bit indicates
 * which of sub-blocks have active IRQs. Fine. That would fit regmap-irq main
 * status handling. Except that:
 *  - Only some sub-IRQs can be masked.
 *  - The IRQ informs us about fault-condition, not when fault state changes.
 *    The IRQ line it is kept asserted until the detected condition is acked
 *    AND cleared in HW. This is annoying for IRQs like the one informing high
 *    temperature because if IRQ is not disabled it keeps the CPU in IRQ
 *    handling loop.
 *
 * For now we do just use the main-IRQ register as source for our IRQ
 * information and bind the regmap-irq to this. We leave fine-grained sub-IRQ
 * register handling to handlers in sub-devices. The regulator driver shall
 * read which regulators are source for problem - or if the detected error is
 * regulator temperature error. The sub-drivers do also handle masking of "sub-
 * IRQs" if this is supported/needed.
 *
 * To overcome the problem with HW keeping IRQ asserted we do call
 * disable_irq_nosync() from sub-device handler and add a delayed work to
 * re-enable IRQ roughly 1 second later. This should keep our CPU out of
 * busy-loop.
 */
pub const IRQS_SILENT_MS: u32 = 1000;

#[repr(i32)]
pub enum Bd9576Interrupt {
    BD9576_INT_THERM,
    BD9576_INT_OVP,
    BD9576_INT_SCP,
    BD9576_INT_OCP,
    BD9576_INT_OVD,
    BD9576_INT_UVD,
    BD9576_INT_UVP,
    BD9576_INT_SYS,
}

pub const BD957X_REG_SMRB_ASSERT: u32 = 0x15;
pub const BD957X_REG_PMIC_INTERNAL_STAT: u32 = 0x20;
pub const BD957X_REG_INT_THERM_STAT: u32 = 0x23;
pub const BD957X_REG_INT_THERM_MASK: u32 = 0x24;
pub const BD957X_REG_INT_OVP_STAT: u32 = 0x25;
pub const BD957X_REG_INT_SCP_STAT: u32 = 0x26;
pub const BD957X_REG_INT_OCP_STAT: u32 = 0x27;
pub const BD957X_REG_INT_OVD_STAT: u32 = 0x28;
pub const BD957X_REG_INT_UVD_STAT: u32 = 0x29;
pub const BD957X_REG_INT_UVP_STAT: u32 = 0x2a;
pub const BD957X_REG_INT_SYS_STAT: u32 = 0x2b;
pub const BD957X_REG_INT_SYS_MASK: u32 = 0x2c;
pub const BD957X_REG_INT_MAIN_STAT: u32 = 0x30;
pub const BD957X_REG_INT_MAIN_MASK: u32 = 0x31;

pub const UVD_IRQ_VALID_MASK: u32 = 0x6F;
pub const OVD_IRQ_VALID_MASK: u32 = 0x2F;

pub const BD957X_MASK_INT_MAIN_THERM: u32 = 1 << 0;
pub const BD957X_MASK_INT_MAIN_OVP: u32 = 1 << 1;
pub const BD957X_MASK_INT_MAIN_SCP: u32 = 1 << 2;
pub const BD957X_MASK_INT_MAIN_OCP: u32 = 1 << 3;
pub const BD957X_MASK_INT_MAIN_OVD: u32 = 1 << 4;
pub const BD957X_MASK_INT_MAIN_UVD: u32 = 1 << 5;
pub const BD957X_MASK_INT_MAIN_UVP: u32 = 1 << 6;
pub const BD957X_MASK_INT_MAIN_SYS: u32 = 1 << 7;
pub const BD957X_MASK_INT_ALL: u32 = 0xff;

pub const BD957X_REG_WDT_CONF: u32 = 0x16;

pub const BD957X_REG_POW_TRIGGER1: u32 = 0x41;
pub const BD957X_REG_POW_TRIGGER2: u32 = 0x42;
pub const BD957X_REG_POW_TRIGGER3: u32 = 0x43;
pub const BD957X_REG_POW_TRIGGER4: u32 = 0x44;
pub const BD957X_REG_POW_TRIGGERL1: u32 = 0x45;
pub const BD957X_REG_POW_TRIGGERS1: u32 = 0x46;

pub const BD957X_REGULATOR_EN_MASK: u32 = 0xff;
pub const BD957X_REGULATOR_DIS_VAL: u32 = 0xff;

pub const BD957X_VSEL_REG_MASK: u32 = 0xff;

pub const BD957X_MASK_VOUT1_TUNE: u32 = 0x87;
pub const BD957X_MASK_VOUT2_TUNE: u32 = 0x87;
pub const BD957X_MASK_VOUT3_TUNE: u32 = 0x1f;
pub const BD957X_MASK_VOUT4_TUNE: u32 = 0x1f;
pub const BD957X_MASK_VOUTL1_TUNE: u32 = 0x87;

pub const BD957X_REG_VOUT1_TUNE: u32 = 0x50;
pub const BD957X_REG_VOUT2_TUNE: u32 = 0x53;
pub const BD957X_REG_VOUT3_TUNE: u32 = 0x56;
pub const BD957X_REG_VOUT4_TUNE: u32 = 0x59;
pub const BD957X_REG_VOUTL1_TUNE: u32 = 0x5c;

pub const BD9576_REG_VOUT1_OVD: u32 = 0x51;
pub const BD9576_REG_VOUT1_UVD: u32 = 0x52;
pub const BD9576_REG_VOUT2_OVD: u32 = 0x54;
pub const BD9576_REG_VOUT2_UVD: u32 = 0x55;
pub const BD9576_REG_VOUT3_OVD: u32 = 0x57;
pub const BD9576_REG_VOUT3_UVD: u32 = 0x58;
pub const BD9576_REG_VOUT4_OVD: u32 = 0x5a;
pub const BD9576_REG_VOUT4_UVD: u32 = 0x5b;
pub const BD9576_REG_VOUTL1_OVD: u32 = 0x5d;
pub const BD9576_REG_VOUTL1_UVD: u32 = 0x5e;

pub const BD9576_MASK_XVD: u32 = 0x7f;

pub const BD9576_REG_VOUT1S_OCW: u32 = 0x5f;
pub const BD9576_REG_VOUT1S_OCP: u32 = 0x60;

pub const BD9576_MASK_VOUT1S_OCW: u32 = 0x3f;
pub const BD9576_MASK_VOUT1S_OCP: u32 = 0x3f;

pub const BD957X_MAX_REGISTER: u32 = 0x61;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
