/* SPDX-License-Identifier: GPL-2.0-only */
/* include/video/ili9320.c
 *
 * ILI9320 LCD controller configuration control.
 *
 * Copyright 2007 Simtec Electronics
 *	Ben Dooks <ben@simtec.co.uk>
 *
 * http://armlinux.simtec.co.uk/
 */

macro_rules! ILI9320_REG { ($x:expr) => { $x }; }

pub const ILI9320_INDEX: u32 = 0x00;
pub const ILI9320_OSCILATION: u32 = 0x00;
pub const ILI9320_DRIVER: u32 = 0x01;
pub const ILI9320_DRIVEWAVE: u32 = 0x02;
pub const ILI9320_ENTRYMODE: u32 = 0x03;
pub const ILI9320_RESIZING: u32 = 0x04;
pub const ILI9320_DISPLAY1: u32 = 0x07;
pub const ILI9320_DISPLAY2: u32 = 0x08;
pub const ILI9320_DISPLAY3: u32 = 0x09;
pub const ILI9320_DISPLAY4: u32 = 0x0A;
pub const ILI9320_RGB_IF1: u32 = 0x0C;
pub const ILI9320_FRAMEMAKER: u32 = 0x0D;
pub const ILI9320_RGB_IF2: u32 = 0x0F;
pub const ILI9320_POWER1: u32 = 0x10;
pub const ILI9320_POWER2: u32 = 0x11;
pub const ILI9320_POWER3: u32 = 0x12;
pub const ILI9320_POWER4: u32 = 0x13;
pub const ILI9320_GRAM_HORIZ_ADDR: u32 = 0x20;
pub const ILI9320_GRAM_VERT_ADD: u32 = 0x21;
pub const ILI9320_POWER7: u32 = 0x29;
pub const ILI9320_FRAME_RATE_COLOUR: u32 = 0x2B;
pub const ILI9320_GAMMA1: u32 = 0x30;
pub const ILI9320_GAMMA2: u32 = 0x31;
pub const ILI9320_GAMMA3: u32 = 0x32;
pub const ILI9320_GAMMA4: u32 = 0x35;
pub const ILI9320_GAMMA5: u32 = 0x36;
pub const ILI9320_GAMMA6: u32 = 0x37;
pub const ILI9320_GAMMA7: u32 = 0x38;
pub const ILI9320_GAMMA8: u32 = 0x39;
pub const ILI9320_GAMMA9: u32 = 0x3C;
pub const ILI9320_GAMMA10: u32 = 0x3D;
pub const ILI9320_HORIZ_START: u32 = 0x50;
pub const ILI9320_HORIZ_END: u32 = 0x51;
pub const ILI9320_VERT_START: u32 = 0x52;
pub const ILI9320_VERT_END: u32 = 0x53;
pub const ILI9320_DRIVER2: u32 = 0x60;
pub const ILI9320_BASE_IMAGE: u32 = 0x61;
pub const ILI9320_VERT_SCROLL: u32 = 0x6a;
pub const ILI9320_PARTIAL1_POSITION: u32 = 0x80;
pub const ILI9320_PARTIAL1_START: u32 = 0x81;
pub const ILI9320_PARTIAL1_END: u32 = 0x82;
pub const ILI9320_PARTIAL2_POSITION: u32 = 0x83;
pub const ILI9320_PARTIAL2_START: u32 = 0x84;
pub const ILI9320_PARTIAL2_END: u32 = 0x85;
pub const ILI9320_INTERFACE1: u32 = 0x90;
pub const ILI9320_INTERFACE2: u32 = 0x92;
pub const ILI9320_INTERFACE3: u32 = 0x93;
pub const ILI9320_INTERFACE4: u32 = 0x95;
pub const ILI9320_INTERFACE5: u32 = 0x97;
pub const ILI9320_INTERFACE6: u32 = 0x98;

macro_rules! bit { ($n:expr) => { 1u32 << $n }; }
pub const ILI9320_OSCILATION_OSC: u32 = 1 << 0;
pub const ILI9320_DRIVER_SS: u32 = 1 << 8;
pub const ILI9320_DRIVER_SM: u32 = 1 << 10;
pub const ILI9320_DRIVEWAVE_EOR: u32 = 1 << 8;
pub const ILI9320_DRIVEWAVE_BC: u32 = 1 << 9;
pub const ILI9320_DRIVEWAVE_MUSTSET: u32 = 1 << 10;
pub const ILI9320_ENTRYMODE_AM: u32 = 1 << 3;
pub const ILI9320_ENTRYMODE_ORG: u32 = 1 << 7;
pub const ILI9320_ENTRYMODE_HWM: u32 = 1 << 8;
pub const ILI9320_ENTRYMODE_BGR: u32 = 1 << 12;
pub const ILI9320_ENTRYMODE_DFM: u32 = 1 << 14;
pub const ILI9320_ENTRYMODE_TRI: u32 = 1 << 15;

macro_rules! ILI9320_ENTRYMODE_ID { ($x:expr) => { ($x) << 4 }; }
macro_rules! ILI9320_RESIZING_RSZ { ($x:expr) => { ($x) << 0 }; }
macro_rules! ILI9320_RESIZING_RCH { ($x:expr) => { ($x) << 4 }; }
macro_rules! ILI9320_RESIZING_RCV { ($x:expr) => { ($x) << 8 }; }
macro_rules! ILI9320_DISPLAY1_D { ($x:expr) => { ($x) << 0 }; }
pub const ILI9320_DISPLAY1_CL: u32 = 1 << 3;
pub const ILI9320_DISPLAY1_DTE: u32 = 1 << 4;
pub const ILI9320_DISPLAY1_GON: u32 = 1 << 5;
pub const ILI9320_DISPLAY1_BASEE: u32 = 1 << 8;
macro_rules! ILI9320_DISPLAY1_PTDE { ($x:expr) => { ($x) << 12 }; }
macro_rules! ILI9320_DISPLAY2_BP { ($x:expr) => { ($x) << 0 }; }
macro_rules! ILI9320_DISPLAY2_FP { ($x:expr) => { ($x) << 8 }; }

pub const ILI9320_RGBIF1_RIM_RGB18: u32 = 0 << 0;
pub const ILI9320_RGBIF1_RIM_RGB16: u32 = 1 << 0;
pub const ILI9320_RGBIF1_RIM_RGB6: u32 = 2 << 0;
pub const ILI9320_RGBIF1_CLK_INT: u32 = 0 << 4;
pub const ILI9320_RGBIF1_CLK_RGBIF: u32 = 1 << 4;
pub const ILI9320_RGBIF1_CLK_VSYNC: u32 = 2 << 4;
pub const ILI9320_RGBIF1_RM: u32 = 1 << 8;
macro_rules! ILI9320_RGBIF1_ENC_FRAMES { ($x:expr) => { (($x) - 1) << 13 }; }
pub const ILI9320_RGBIF2_DPL: u32 = 1 << 0;
pub const ILI9320_RGBIF2_EPL: u32 = 1 << 1;
pub const ILI9320_RGBIF2_HSPL: u32 = 1 << 3;
pub const ILI9320_RGBIF2_VSPL: u32 = 1 << 4;

macro_rules! ILI9320_POWER1_AP { ($x:expr) => { ($x) << 4 }; }
macro_rules! ILI9320_POWER1_BT { ($x:expr) => { ($x) << 8 }; }
pub const ILI9320_POWER1_SLP: u32 = 1 << 1;
pub const ILI9320_POWER1_DSTB: u32 = 1 << 2;
pub const ILI9320_POWER1_APE: u32 = 1 << 7;
pub const ILI9320_POWER1_SAP: u32 = 1 << 12;
macro_rules! ILI9320_POWER2_VC { ($x:expr) => { ($x) << 0 }; }
macro_rules! ILI9320_POWER2_DC0 { ($x:expr) => { ($x) << 4 }; }
macro_rules! ILI9320_POWER2_DC1 { ($x:expr) => { ($x) << 8 }; }
macro_rules! ILI9320_POWER3_VRH { ($x:expr) => { ($x) << 0 }; }
pub const ILI9320_POWER3_PON: u32 = 1 << 4;
pub const ILI9320_POWER3_VCMR: u32 = 1 << 8;
macro_rules! ILI9320_POWER4_VREOUT { ($x:expr) => { ($x) << 8 }; }
macro_rules! ILI9320_DRIVER2_SCNL { ($x:expr) => { ($x) << 0 }; }
macro_rules! ILI9320_DRIVER2_NL { ($x:expr) => { ($x) << 8 }; }
pub const ILI9320_DRIVER2_GS: u32 = 1 << 15;
pub const ILI9320_BASEIMAGE_REV: u32 = 1 << 0;
pub const ILI9320_BASEIMAGE_VLE: u32 = 1 << 1;
pub const ILI9320_BASEIMAGE_NDL: u32 = 1 << 2;
macro_rules! ILI9320_INTERFACE4_RTNE { ($x:expr) => { $x }; }
macro_rules! ILI9320_INTERFACE4_DIVE { ($x:expr) => { ($x) << 8 }; }

pub const ILI9320_SPI_IDCODE: u32 = 0x70;
macro_rules! ILI9320_SPI_ID { ($x:expr) => { ($x) << 2 }; }
pub const ILI9320_SPI_READ: u32 = 0x01;
pub const ILI9320_SPI_WRITE: u32 = 0x00;
pub const ILI9320_SPI_DATA: u32 = 0x02;
pub const ILI9320_SPI_INDEX: u32 = 0x00;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ili9320_suspend {
    ILI9320_SUSPEND_OFF,
    ILI9320_SUSPEND_DEEP,
}

#[repr(C)]
pub struct ili9320_platdata {
    pub hsize: u16,
    pub vsize: u16,
    pub suspend: ili9320_suspend,
    pub reset: Option<unsafe extern "C" fn(val: u32)>,
    pub entry_mode: u16,
    pub display2: u16,
    pub display3: u16,
    pub display4: u16,
    pub rgb_if1: u16,
    pub rgb_if2: u16,
    pub interface2: u16,
    pub interface3: u16,
    pub interface4: u16,
    pub interface5: u16,
    pub interface6: u16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
