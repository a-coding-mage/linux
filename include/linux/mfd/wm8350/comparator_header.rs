/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * comparator.h -- Comparator Aux ADC for Wolfson WM8350 PMIC
 *
 * Copyright 2007 Wolfson Microelectronics PLC
 */

/* Registers */
pub const WM8350_DIGITISER_CONTROL_1: u16 = 0x90;
pub const WM8350_DIGITISER_CONTROL_2: u16 = 0x91;
pub const WM8350_AUX1_READBACK: u16 = 0x98;
pub const WM8350_AUX2_READBACK: u16 = 0x99;
pub const WM8350_AUX3_READBACK: u16 = 0x9A;
pub const WM8350_AUX4_READBACK: u16 = 0x9B;
pub const WM8350_CHIP_TEMP_READBACK: u16 = 0x9F;
pub const WM8350_GENERIC_COMPARATOR_CONTROL: u16 = 0xA3;
pub const WM8350_GENERIC_COMPARATOR_1: u16 = 0xA4;
pub const WM8350_GENERIC_COMPARATOR_2: u16 = 0xA5;
pub const WM8350_GENERIC_COMPARATOR_3: u16 = 0xA6;
pub const WM8350_GENERIC_COMPARATOR_4: u16 = 0xA7;

/* R144 (0x90) - Digitiser Control (1) */
pub const WM8350_AUXADC_CTC: u16 = 0x4000;
pub const WM8350_AUXADC_POLL: u16 = 0x2000;
pub const WM8350_AUXADC_HIB_MODE: u16 = 0x1000;
pub const WM8350_AUXADC_SEL8: u16 = 0x0080;
pub const WM8350_AUXADC_SEL7: u16 = 0x0040;
pub const WM8350_AUXADC_SEL6: u16 = 0x0020;
pub const WM8350_AUXADC_SEL5: u16 = 0x0010;
pub const WM8350_AUXADC_SEL4: u16 = 0x0008;
pub const WM8350_AUXADC_SEL3: u16 = 0x0004;
pub const WM8350_AUXADC_SEL2: u16 = 0x0002;
pub const WM8350_AUXADC_SEL1: u16 = 0x0001;

/* R145 (0x91) - Digitiser Control (2) */
pub const WM8350_AUXADC_MASKMODE_MASK: u16 = 0x3000;
pub const WM8350_AUXADC_CRATE_MASK: u16 = 0x0700;
pub const WM8350_AUXADC_CAL: u16 = 0x0004;
pub const WM8350_AUX_RBMODE: u16 = 0x0002;
pub const WM8350_AUXADC_WAIT: u16 = 0x0001;

/* R152-R155 (0x98-0x9B) - AUX Readback */
pub const WM8350_AUXADC_SCALE1_MASK: u16 = 0x6000;
pub const WM8350_AUXADC_REF1: u16 = 0x1000;
pub const WM8350_AUXADC_DATA1_MASK: u16 = 0x0FFF;
pub const WM8350_AUXADC_SCALE2_MASK: u16 = 0x6000;
pub const WM8350_AUXADC_REF2: u16 = 0x1000;
pub const WM8350_AUXADC_DATA2_MASK: u16 = 0x0FFF;
pub const WM8350_AUXADC_SCALE3_MASK: u16 = 0x6000;
pub const WM8350_AUXADC_REF3: u16 = 0x1000;
pub const WM8350_AUXADC_DATA3_MASK: u16 = 0x0FFF;
pub const WM8350_AUXADC_SCALE4_MASK: u16 = 0x6000;
pub const WM8350_AUXADC_REF4: u16 = 0x1000;
pub const WM8350_AUXADC_DATA4_MASK: u16 = 0x0FFF;

/* R156-R158 (0x9C-0x9E) - USB, LINE and BATT Voltage Readback */
pub const WM8350_AUXADC_DATA_USB_MASK: u16 = 0x0FFF;
pub const WM8350_AUXADC_DATA_LINE_MASK: u16 = 0x0FFF;
pub const WM8350_AUXADC_DATA_BATT_MASK: u16 = 0x0FFF;

/* R159 (0x9F) - Chip Temp Readback */
pub const WM8350_AUXADC_DATA_CHIPTEMP_MASK: u16 = 0x0FFF;

/* R163-R167 (0xA3-0xA7) - Generic Comparators */
pub const WM8350_DCMP4_ENA: u16 = 0x0008;
pub const WM8350_DCMP3_ENA: u16 = 0x0004;
pub const WM8350_DCMP2_ENA: u16 = 0x0002;
pub const WM8350_DCMP1_ENA: u16 = 0x0001;
pub const WM8350_DCMP1_SRCSEL_MASK: u16 = 0xE000;
pub const WM8350_DCMP1_GT: u16 = 0x1000;
pub const WM8350_DCMP1_THR_MASK: u16 = 0x0FFF;
pub const WM8350_DCMP2_SRCSEL_MASK: u16 = 0xE000;
pub const WM8350_DCMP2_GT: u16 = 0x1000;
pub const WM8350_DCMP2_THR_MASK: u16 = 0x0FFF;
pub const WM8350_DCMP3_SRCSEL_MASK: u16 = 0xE000;
pub const WM8350_DCMP3_GT: u16 = 0x1000;
pub const WM8350_DCMP3_THR_MASK: u16 = 0x0FFF;
pub const WM8350_DCMP4_SRCSEL_MASK: u16 = 0xE000;
pub const WM8350_DCMP4_GT: u16 = 0x1000;
pub const WM8350_DCMP4_THR_MASK: u16 = 0x0FFF;

/* Interrupts. */
pub const WM8350_IRQ_AUXADC_DATARDY: i32 = 16;
pub const WM8350_IRQ_AUXADC_DCOMP4: i32 = 17;
pub const WM8350_IRQ_AUXADC_DCOMP3: i32 = 18;
pub const WM8350_IRQ_AUXADC_DCOMP2: i32 = 19;
pub const WM8350_IRQ_AUXADC_DCOMP1: i32 = 20;
pub const WM8350_IRQ_SYS_HYST_COMP_FAIL: i32 = 21;
pub const WM8350_IRQ_SYS_CHIP_GT115: i32 = 22;
pub const WM8350_IRQ_SYS_CHIP_GT140: i32 = 23;

/* USB/2, LINE & BATT = ((VRTC * 2) / 4095)) * 10e6 uV, where VRTC = 2.7 V. */
pub const WM8350_AUX_COEFF: i32 = 1319;

pub const WM8350_AUXADC_AUX1: i32 = 0;
pub const WM8350_AUXADC_AUX2: i32 = 1;
pub const WM8350_AUXADC_AUX3: i32 = 2;
pub const WM8350_AUXADC_AUX4: i32 = 3;
pub const WM8350_AUXADC_USB: i32 = 4;
pub const WM8350_AUXADC_LINE: i32 = 5;
pub const WM8350_AUXADC_BATT: i32 = 6;
pub const WM8350_AUXADC_TEMP: i32 = 7;

#[repr(C)]
pub struct wm8350 {
    _private: [u8; 0],
}

/* AUX ADC Readback */
extern "C" {
    pub fn wm8350_read_auxadc(
        wm8350: *mut wm8350,
        channel: i32,
        scale: i32,
        vref: i32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
