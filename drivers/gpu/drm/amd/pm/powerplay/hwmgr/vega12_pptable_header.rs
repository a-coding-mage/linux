/*
 * Copyright 2017 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

pub const ATOM_VEGA12_PP_THERMALCONTROLLER_NONE: u32 = 0;
pub const ATOM_VEGA12_PP_THERMALCONTROLLER_VEGA12: u32 = 25;

pub const ATOM_VEGA12_PP_PLATFORM_CAP_POWERPLAY: u32 = 0x1;
pub const ATOM_VEGA12_PP_PLATFORM_CAP_SBIOSPOWERSOURCE: u32 = 0x2;
pub const ATOM_VEGA12_PP_PLATFORM_CAP_HARDWAREDC: u32 = 0x4;
pub const ATOM_VEGA12_PP_PLATFORM_CAP_BACO: u32 = 0x8;
pub const ATOM_VEGA12_PP_PLATFORM_CAP_BAMACO: u32 = 0x10;
pub const ATOM_VEGA12_PP_PLATFORM_CAP_ENABLESHADOWPSTATE: u32 = 0x20;

pub const ATOM_VEGA12_TABLE_REVISION_VEGA12: u32 = 9;

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ATOM_VEGA12_ODSETTING_ID {
    ATOM_VEGA12_ODSETTING_GFXCLKFMAX = 0,
    ATOM_VEGA12_ODSETTING_GFXCLKFMIN,
    ATOM_VEGA12_ODSETTING_VDDGFXCURVEFREQ_P1,
    ATOM_VEGA12_ODSETTING_VDDGFXCURVEVOLTAGEOFFSET_P1,
    ATOM_VEGA12_ODSETTING_VDDGFXCURVEFREQ_P2,
    ATOM_VEGA12_ODSETTING_VDDGFXCURVEVOLTAGEOFFSET_P2,
    ATOM_VEGA12_ODSETTING_VDDGFXCURVEFREQ_P3,
    ATOM_VEGA12_ODSETTING_VDDGFXCURVEVOLTAGEOFFSET_P3,
    ATOM_VEGA12_ODSETTING_UCLKFMAX,
    ATOM_VEGA12_ODSETTING_POWERPERCENTAGE,
    ATOM_VEGA12_ODSETTING_FANRPMMIN,
    ATOM_VEGA12_ODSETTING_FANRPMACOUSTICLIMIT,
    ATOM_VEGA12_ODSETTING_FANTARGETTEMPERATURE,
    ATOM_VEGA12_ODSETTING_OPERATINGTEMPMAX,
    ATOM_VEGA12_ODSETTING_COUNT,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ATOM_VEGA12_PPCLOCK_ID {
    ATOM_VEGA12_PPCLOCK_GFXCLK = 0,
    ATOM_VEGA12_PPCLOCK_VCLK,
    ATOM_VEGA12_PPCLOCK_DCLK,
    ATOM_VEGA12_PPCLOCK_ECLK,
    ATOM_VEGA12_PPCLOCK_SOCCLK,
    ATOM_VEGA12_PPCLOCK_UCLK,
    ATOM_VEGA12_PPCLOCK_DCEFCLK,
    ATOM_VEGA12_PPCLOCK_DISPCLK,
    ATOM_VEGA12_PPCLOCK_PIXCLK,
    ATOM_VEGA12_PPCLOCK_PHYCLK,
    ATOM_VEGA12_PPCLOCK_COUNT,
}

#[repr(C, packed)]
pub struct ATOM_Vega12_POWERPLAYTABLE {
    pub sHeader: atom_common_table_header,
    pub ucTableRevision: u8,
    pub usTableSize: u16,
    pub ulGoldenPPID: u32,
    pub ulGoldenRevision: u32,
    pub usFormatID: u16,
    pub ulPlatformCaps: u32,
    pub ucThermalControllerType: u8,
    pub usSmallPowerLimit1: u16,
    pub usSmallPowerLimit2: u16,
    pub usBoostPowerLimit: u16,
    pub usODTurboPowerLimit: u16,
    pub usODPowerSavePowerLimit: u16,
    pub usSoftwareShutdownTemp: u16,
    pub PowerSavingClockMax: [u32; ATOM_VEGA12_PPCLOCK_ID::ATOM_VEGA12_PPCLOCK_COUNT as usize],
    pub PowerSavingClockMin: [u32; ATOM_VEGA12_PPCLOCK_ID::ATOM_VEGA12_PPCLOCK_COUNT as usize],
    pub ODSettingsMax: [u32; ATOM_VEGA12_ODSETTING_ID::ATOM_VEGA12_ODSETTING_COUNT as usize],
    pub ODSettingsMin: [u32; ATOM_VEGA12_ODSETTING_ID::ATOM_VEGA12_ODSETTING_COUNT as usize],
    pub usReserve: [u16; 5],
    pub smcPPTable: PPTable_t,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
