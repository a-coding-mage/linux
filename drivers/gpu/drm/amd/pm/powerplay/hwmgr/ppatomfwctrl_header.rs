/*
 * Copyright 2016 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependency supplied by the surrounding translation unit: hwmgr.h

pub type BIOS_CLKID = atom_smu9_syspll0_clock_id;

#[macro_export]
macro_rules! GetIndexIntoMasterCmdTable {
    ($FieldName:ident) => {
        std::mem::offset_of!(atom_master_list_of_command_functions_v2_1, $FieldName)
            / std::mem::size_of::<u16>()
    };
}

#[macro_export]
macro_rules! GetIndexIntoMasterDataTable {
    ($FieldName:ident) => {
        std::mem::offset_of!(atom_master_list_of_data_tables_v2_1, $FieldName)
            / std::mem::size_of::<u16>()
    };
}

pub const PP_ATOMFWCTRL_MAX_VOLTAGE_ENTRIES: usize = 32;

#[repr(C)]
pub struct pp_atomfwctrl_voltage_table_entry {
    pub value: u16,
    pub smio_low: u32,
}

#[repr(C)]
pub struct pp_atomfwctrl_voltage_table {
    pub count: u32,
    pub mask_low: u32,
    pub phase_delay: u32,
    pub psi0_enable: u8,
    pub psi1_enable: u8,
    pub max_vid_step: u8,
    pub telemetry_offset: u8,
    pub telemetry_slope: u8,
    pub entries: [pp_atomfwctrl_voltage_table_entry; PP_ATOMFWCTRL_MAX_VOLTAGE_ENTRIES],
}

#[repr(C)]
pub struct pp_atomfwctrl_gpio_pin_assignment {
    pub us_gpio_pin_aindex: u16,
    pub uc_gpio_pin_bit_shift: u8,
}

#[repr(C)]
pub struct pp_atomfwctrl_clock_dividers_soc15 {
    pub ulClock: u32,
    pub ulDid: u32,
    pub ulPll_fb_mult: u32,
    pub ulPll_ss_fbsmult: u32,
    pub usPll_ss_slew_frac: u16,
    pub ucPll_ss_enable: u8,
    pub ucReserve: u8,
    pub ulReserve: [u32; 2],
}

#[repr(C)]
pub struct pp_atomfwctrl_avfs_parameters {
    pub ulMaxVddc: u32,
    pub ulMinVddc: u32,
    pub ulMeanNsigmaAcontant0: u32,
    pub ulMeanNsigmaAcontant1: u32,
    pub ulMeanNsigmaAcontant2: u32,
    pub usMeanNsigmaDcTolSigma: u16,
    pub usMeanNsigmaPlatformMean: u16,
    pub usMeanNsigmaPlatformSigma: u16,
    pub ulGbVdroopTableCksoffA0: u32,
    pub ulGbVdroopTableCksoffA1: u32,
    pub ulGbVdroopTableCksoffA2: u32,
    pub ulGbVdroopTableCksonA0: u32,
    pub ulGbVdroopTableCksonA1: u32,
    pub ulGbVdroopTableCksonA2: u32,
    pub ulGbFuseTableCksoffM1: u32,
    pub ulGbFuseTableCksoffM2: u32,
    pub ulGbFuseTableCksoffB: u32,
    pub ulGbFuseTableCksonM1: u32,
    pub ulGbFuseTableCksonM2: u32,
    pub ulGbFuseTableCksonB: u32,
    pub ucEnableGbVdroopTableCkson: u8,
    pub ucEnableGbFuseTableCkson: u8,
    pub usPsmAgeComfactor: u16,
    pub ulDispclk2GfxclkM1: u32,
    pub ulDispclk2GfxclkM2: u32,
    pub ulDispclk2GfxclkB: u32,
    pub ulDcefclk2GfxclkM1: u32,
    pub ulDcefclk2GfxclkM2: u32,
    pub ulDcefclk2GfxclkB: u32,
    pub ulPixelclk2GfxclkM1: u32,
    pub ulPixelclk2GfxclkM2: u32,
    pub ulPixelclk2GfxclkB: u32,
    pub ulPhyclk2GfxclkM1: u32,
    pub ulPhyclk2GfxclkM2: u32,
    pub ulPhyclk2GfxclkB: u32,
    pub ulAcgGbVdroopTableA0: u32,
    pub ulAcgGbVdroopTableA1: u32,
    pub ulAcgGbVdroopTableA2: u32,
    pub ulAcgGbFuseTableM1: u32,
    pub ulAcgGbFuseTableM2: u32,
    pub ulAcgGbFuseTableB: u32,
    pub ucAcgEnableGbVdroopTable: u32,
    pub ucAcgEnableGbFuseTable: u32,
}

#[repr(C)]
pub struct pp_atomfwctrl_gpio_parameters {
    pub ucAcDcGpio: u8,
    pub ucAcDcPolarity: u8,
    pub ucVR0HotGpio: u8,
    pub ucVR0HotPolarity: u8,
    pub ucVR1HotGpio: u8,
    pub ucVR1HotPolarity: u8,
    pub ucFwCtfGpio: u8,
    pub ucFwCtfPolarity: u8,
}

#[repr(C)]
pub struct pp_atomfwctrl_bios_boot_up_values {
    pub ulRevision: u32,
    pub ulGfxClk: u32,
    pub ulUClk: u32,
    pub ulSocClk: u32,
    pub ulDCEFClk: u32,
    pub ulEClk: u32,
    pub ulVClk: u32,
    pub ulDClk: u32,
    pub ulFClk: u32,
    pub usVddc: u16,
    pub usVddci: u16,
    pub usMvddc: u16,
    pub usVddGfx: u16,
    pub ucCoolingID: u8,
}

#[repr(C)]
pub struct pp_atomfwctrl_smc_dpm_parameters {
    pub liquid1_i2c_address: u8, pub liquid2_i2c_address: u8, pub vr_i2c_address: u8, pub plx_i2c_address: u8,
    pub liquid_i2c_linescl: u8, pub liquid_i2c_linesda: u8, pub vr_i2c_linescl: u8, pub vr_i2c_linesda: u8,
    pub plx_i2c_linescl: u8, pub plx_i2c_linesda: u8, pub vrsensorpresent: u8, pub liquidsensorpresent: u8,
    pub maxvoltagestepgfx: u16, pub maxvoltagestepsoc: u16,
    pub vddgfxvrmapping: u8, pub vddsocvrmapping: u8, pub vddmem0vrmapping: u8, pub vddmem1vrmapping: u8,
    pub gfxulvphasesheddingmask: u8, pub soculvphasesheddingmask: u8,
    pub gfxmaxcurrent: u16, pub gfxoffset: u8, pub padding_telemetrygfx: u8,
    pub socmaxcurrent: u16, pub socoffset: u8, pub padding_telemetrysoc: u8,
    pub mem0maxcurrent: u16, pub mem0offset: u8, pub padding_telemetrymem0: u8,
    pub mem1maxcurrent: u16, pub mem1offset: u8, pub padding_telemetrymem1: u8,
    pub acdcgpio: u8, pub acdcpolarity: u8, pub vr0hotgpio: u8, pub vr0hotpolarity: u8,
    pub vr1hotgpio: u8, pub vr1hotpolarity: u8, pub padding1: u8, pub padding2: u8,
    pub ledpin0: u8, pub ledpin1: u8, pub ledpin2: u8,
    pub pllgfxclkspreadenabled: u8, pub pllgfxclkspreadpercent: u8, pub pllgfxclkspreadfreq: u16,
    pub uclkspreadenabled: u8, pub uclkspreadpercent: u8, pub uclkspreadfreq: u16,
    pub socclkspreadenabled: u8, pub socclkspreadpercent: u8, pub socclkspreadfreq: u16,
    pub acggfxclkspreadenabled: u8, pub acggfxclkspreadpercent: u8, pub acggfxclkspreadfreq: u16,
    pub Vr2_I2C_address: u8,
}

extern "C" {
    pub fn pp_atomfwctrl_get_gpu_pll_dividers_vega10(hwmgr: *mut pp_hwmgr, clock_type: u32, clock_value: u32, dividers: *mut pp_atomfwctrl_clock_dividers_soc15) -> i32;
    pub fn pp_atomfwctrl_get_voltage_table_v4(hwmgr: *mut pp_hwmgr, voltage_type: u8, voltage_mode: u8, voltage_table: *mut pp_atomfwctrl_voltage_table) -> i32;
    pub fn pp_atomfwctrl_is_voltage_controlled_by_gpio_v4(hwmgr: *mut pp_hwmgr, voltage_type: u8, voltage_mode: u8) -> bool;
    pub fn pp_atomfwctrl_get_avfs_information(hwmgr: *mut pp_hwmgr, param: *mut pp_atomfwctrl_avfs_parameters) -> i32;
    pub fn pp_atomfwctrl_get_gpio_information(hwmgr: *mut pp_hwmgr, param: *mut pp_atomfwctrl_gpio_parameters) -> i32;
    pub fn pp_atomfwctrl_get_vbios_bootup_values(hwmgr: *mut pp_hwmgr, boot_values: *mut pp_atomfwctrl_bios_boot_up_values) -> i32;
    pub fn pp_atomfwctrl_get_smc_dpm_information(hwmgr: *mut pp_hwmgr, param: *mut pp_atomfwctrl_smc_dpm_parameters) -> i32;
    pub fn pp_atomfwctrl_get_clk_information_by_clkid(hwmgr: *mut pp_hwmgr, clk_id: u8, syspll_id: u8, frequency: *mut u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
