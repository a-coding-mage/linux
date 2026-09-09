/*
 * Copyright 2015 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

/** PowerPlay table header file. */

pub const ATOM_TONGA_PP_FANPARAMETERS_TACHOMETER_PULSES_PER_REVOLUTION_MASK: u8 = 0x0f;
pub const ATOM_TONGA_PP_FANPARAMETERS_NOFAN: u8 = 0x80;
pub const ATOM_TONGA_PP_THERMALCONTROLLER_NONE: u8 = 0;
pub const ATOM_TONGA_PP_THERMALCONTROLLER_LM96163: u8 = 17;
pub const ATOM_TONGA_PP_THERMALCONTROLLER_TONGA: u8 = 21;
pub const ATOM_TONGA_PP_THERMALCONTROLLER_FIJI: u8 = 22;
pub const ATOM_TONGA_PP_THERMALCONTROLLER_ADT7473_WITH_INTERNAL: u8 = 0x89;
pub const ATOM_TONGA_PP_THERMALCONTROLLER_EMC2103_WITH_INTERNAL: u8 = 0x8d;

pub const ATOM_TONGA_PP_PLATFORM_CAP_VDDGFX_CONTROL: u32 = 0x1;
pub const ATOM_TONGA_PP_PLATFORM_CAP_POWERPLAY: u32 = 0x2;
pub const ATOM_TONGA_PP_PLATFORM_CAP_SBIOSPOWERSOURCE: u32 = 0x4;
pub const ATOM_TONGA_PP_PLATFORM_CAP_DISABLE_VOLTAGE_ISLAND: u32 = 0x8;
pub const ____RETIRE16____: u32 = 0x10;
pub const ATOM_TONGA_PP_PLATFORM_CAP_HARDWAREDC: u32 = 0x20;
pub const ____RETIRE64____: u32 = 0x40;
pub const ____RETIRE128____: u32 = 0x80;
pub const ____RETIRE256____: u32 = 0x100;
pub const ____RETIRE512____: u32 = 0x200;
pub const ____RETIRE1024____: u32 = 0x400;
pub const ____RETIRE2048____: u32 = 0x800;
pub const ATOM_TONGA_PP_PLATFORM_CAP_MVDD_CONTROL: u32 = 0x1000;
pub const ____RETIRE2000____: u32 = 0x2000;
pub const ____RETIRE4000____: u32 = 0x4000;
pub const ATOM_TONGA_PP_PLATFORM_CAP_VDDCI_CONTROL: u32 = 0x8000;
pub const ____RETIRE10000____: u32 = 0x10000;
pub const ATOM_TONGA_PP_PLATFORM_CAP_BACO: u32 = 0x20000;
pub const ATOM_TONGA_PP_PLATFORM_CAP_OUTPUT_THERMAL2GPIO17: u32 = 0x100000;
pub const ATOM_TONGA_PP_PLATFORM_COMBINE_PCC_WITH_THERMAL_SIGNAL: u32 = 0x1000000;
pub const ATOM_TONGA_PLATFORM_LOAD_POST_PRODUCTION_FIRMWARE: u32 = 0x2000000;

pub const ATOM_PPLIB_CLASSIFICATION_UI_MASK: u16 = 0x0007;
pub const ATOM_PPLIB_CLASSIFICATION_UI_SHIFT: u16 = 0;
pub const ATOM_PPLIB_CLASSIFICATION_UI_NONE: u16 = 0;
pub const ATOM_PPLIB_CLASSIFICATION_UI_BATTERY: u16 = 1;
pub const ATOM_PPLIB_CLASSIFICATION_UI_BALANCED: u16 = 3;
pub const ATOM_PPLIB_CLASSIFICATION_UI_PERFORMANCE: u16 = 5;
pub const ATOM_PPLIB_CLASSIFICATION_BOOT: u16 = 0x0008;
pub const ATOM_PPLIB_CLASSIFICATION_THERMAL: u16 = 0x0010;
pub const ATOM_PPLIB_CLASSIFICATION_LIMITEDPOWERSOURCE: u16 = 0x0020;
pub const ATOM_PPLIB_CLASSIFICATION_REST: u16 = 0x0040;
pub const ATOM_PPLIB_CLASSIFICATION_FORCED: u16 = 0x0080;
pub const ATOM_PPLIB_CLASSIFICATION_ACPI: u16 = 0x1000;
pub const ATOM_PPLIB_CLASSIFICATION2_LIMITEDPOWERSOURCE_2: u16 = 0x0001;
pub const ATOM_Tonga_DISALLOW_ON_DC: u16 = 0x00004000;
pub const ATOM_Tonga_ENABLE_VARIBRIGHT: u16 = 0x00008000;
pub const ATOM_Tonga_TABLE_REVISION_TONGA: u8 = 7;

#[repr(C, packed)]
pub struct ATOM_Tonga_POWERPLAYTABLE {
    pub sHeader: ATOM_COMMON_TABLE_HEADER,
    pub ucTableRevision: u8, pub usTableSize: u16,
    pub ulGoldenPPID: u32, pub ulGoldenRevision: u32, pub usFormatID: u16,
    pub usVoltageTime: u16, pub ulPlatformCaps: u32,
    pub ulMaxODEngineClock: u32, pub ulMaxODMemoryClock: u32,
    pub usPowerControlLimit: u16, pub usUlvVoltageOffset: u16,
    pub usStateArrayOffset: u16, pub usFanTableOffset: u16,
    pub usThermalControllerOffset: u16, pub usReserv: u16,
    pub usMclkDependencyTableOffset: u16, pub usSclkDependencyTableOffset: u16,
    pub usVddcLookupTableOffset: u16, pub usVddgfxLookupTableOffset: u16,
    pub usMMDependencyTableOffset: u16, pub usVCEStateTableOffset: u16,
    pub usPPMTableOffset: u16, pub usPowerTuneTableOffset: u16,
    pub usHardLimitTableOffset: u16, pub usPCIETableOffset: u16,
    pub usGPIOTableOffset: u16, pub usReserved: [u16; 6],
}

#[repr(C, packed)]
pub struct ATOM_Tonga_State { pub ucEngineClockIndexHigh: u8, pub ucEngineClockIndexLow: u8, pub ucMemoryClockIndexHigh: u8, pub ucMemoryClockIndexLow: u8, pub ucPCIEGenLow: u8, pub ucPCIEGenHigh: u8, pub ucPCIELaneLow: u8, pub ucPCIELaneHigh: u8, pub usClassification: u16, pub ulCapsAndSettings: u32, pub usClassification2: u16, pub ucUnused: [u8; 4] }
#[repr(C, packed)]
pub struct ATOM_Tonga_State_Array { pub ucRevId: u8, pub ucNumEntries: u8, pub entries: [ATOM_Tonga_State; 0] }

#[repr(C, packed)]
pub struct ATOM_Tonga_MCLK_Dependency_Record { pub ucVddcInd: u8, pub usVddci: u16, pub usVddgfxOffset: u16, pub usMvdd: u16, pub ulMclk: u32, pub usReserved: u16 }
#[repr(C, packed)]
pub struct ATOM_Tonga_MCLK_Dependency_Table { pub ucRevId: u8, pub ucNumEntries: u8, pub entries: [ATOM_Tonga_MCLK_Dependency_Record; 0] }
#[repr(C, packed)]
pub struct ATOM_Tonga_SCLK_Dependency_Record { pub ucVddInd: u8, pub usVddcOffset: u16, pub ulSclk: u32, pub usEdcCurrent: u16, pub ucReliabilityTemperature: u8, pub ucCKSVOffsetandDisable: u8 }
#[repr(C, packed)]
pub struct ATOM_Tonga_SCLK_Dependency_Table { pub ucRevId: u8, pub ucNumEntries: u8, pub entries: [ATOM_Tonga_SCLK_Dependency_Record; 0] }
#[repr(C, packed)]
pub struct ATOM_Polaris_SCLK_Dependency_Record { pub ucVddInd: u8, pub usVddcOffset: u16, pub ulSclk: u32, pub usEdcCurrent: u16, pub ucReliabilityTemperature: u8, pub ucCKSVOffsetandDisable: u8, pub ulSclkOffset: u32 }
#[repr(C, packed)]
pub struct ATOM_Polaris_SCLK_Dependency_Table { pub ucRevId: u8, pub ucNumEntries: u8, pub entries: [ATOM_Polaris_SCLK_Dependency_Record; 0] }

#[repr(C, packed)]
pub struct ATOM_Tonga_PCIE_Record { pub ucPCIEGenSpeed: u8, pub usPCIELaneWidth: u8, pub ucReserved: [u8; 2] }
#[repr(C, packed)]
pub struct ATOM_Tonga_PCIE_Table { pub ucRevId: u8, pub ucNumEntries: u8, pub entries: [ATOM_Tonga_PCIE_Record; 0] }
#[repr(C, packed)]
pub struct ATOM_Polaris10_PCIE_Record { pub ucPCIEGenSpeed: u8, pub usPCIELaneWidth: u8, pub ucReserved: [u8; 2], pub ulPCIE_Sclk: u32 }
#[repr(C, packed)]
pub struct ATOM_Polaris10_PCIE_Table { pub ucRevId: u8, pub ucNumEntries: u8, pub entries: [ATOM_Polaris10_PCIE_Record; 0] }

#[repr(C, packed)]
pub struct ATOM_Tonga_MM_Dependency_Record { pub ucVddcInd: u8, pub usVddgfxOffset: u16, pub ulDClk: u32, pub ulVClk: u32, pub ulEClk: u32, pub ulAClk: u32, pub ulSAMUClk: u32 }
#[repr(C, packed)]
pub struct ATOM_Tonga_MM_Dependency_Table { pub ucRevId: u8, pub ucNumEntries: u8, pub entries: [ATOM_Tonga_MM_Dependency_Record; 0] }
#[repr(C, packed)]
pub struct ATOM_Tonga_Voltage_Lookup_Record { pub usVdd: u16, pub usCACLow: u16, pub usCACMid: u16, pub usCACHigh: u16 }
#[repr(C, packed)]
pub struct ATOM_Tonga_Voltage_Lookup_Table { pub ucRevId: u8, pub ucNumEntries: u8, pub entries: [ATOM_Tonga_Voltage_Lookup_Record; 0] }

#[repr(C, packed)]
pub struct ATOM_Tonga_Fan_Table { pub ucRevId:u8, pub ucTHyst:u8, pub usTMin:u16, pub usTMed:u16, pub usTHigh:u16, pub usPWMMin:u16, pub usPWMMed:u16, pub usPWMHigh:u16, pub usTMax:u16, pub ucFanControlMode:u8, pub usFanPWMMax:u16, pub usFanOutputSensitivity:u16, pub usFanRPMMax:u16, pub ulMinFanSCLKAcousticLimit:u32, pub ucTargetTemperature:u8, pub ucMinimumPWMLimit:u8, pub usReserved:u16 }
#[repr(C, packed)]
pub struct ATOM_Fiji_Fan_Table { pub ucRevId:u8, pub ucTHyst:u8, pub usTMin:u16, pub usTMed:u16, pub usTHigh:u16, pub usPWMMin:u16, pub usPWMMed:u16, pub usPWMHigh:u16, pub usTMax:u16, pub ucFanControlMode:u8, pub usFanPWMMax:u16, pub usFanOutputSensitivity:u16, pub usFanRPMMax:u16, pub ulMinFanSCLKAcousticLimit:u32, pub ucTargetTemperature:u8, pub ucMinimumPWMLimit:u8, pub usFanGainEdge:u16, pub usFanGainHotspot:u16, pub usFanGainLiquid:u16, pub usFanGainVrVddc:u16, pub usFanGainVrMvdd:u16, pub usFanGainPlx:u16, pub usFanGainHbm:u16, pub usReserved:u16 }
#[repr(C, packed)]
pub struct ATOM_Polaris_Fan_Table { pub ucRevId:u8, pub ucTHyst:u8, pub usTMin:u16, pub usTMed:u16, pub usTHigh:u16, pub usPWMMin:u16, pub usPWMMed:u16, pub usPWMHigh:u16, pub usTMax:u16, pub ucFanControlMode:u8, pub usFanPWMMax:u16, pub usFanOutputSensitivity:u16, pub usFanRPMMax:u16, pub ulMinFanSCLKAcousticLimit:u32, pub ucTargetTemperature:u8, pub ucMinimumPWMLimit:u8, pub usFanGainEdge:u16, pub usFanGainHotspot:u16, pub usFanGainLiquid:u16, pub usFanGainVrVddc:u16, pub usFanGainVrMvdd:u16, pub usFanGainPlx:u16, pub usFanGainHbm:u16, pub ucEnableZeroRPM:u8, pub ucFanStopTemperature:u8, pub ucFanStartTemperature:u8, pub usReserved:u16 }
#[repr(C, packed)]
pub struct ATOM_Tonga_Thermal_Controller { pub ucRevId:u8, pub ucType:u8, pub ucI2cLine:u8, pub ucI2cAddress:u8, pub ucFanParameters:u8, pub ucFanMinRPM:u8, pub ucFanMaxRPM:u8, pub ucReserved:u8, pub ucFlags:u8 }
#[repr(C, packed)]
pub struct ATOM_Tonga_VCE_State_Record { pub ucVCEClockIndex:u8, pub ucFlag:u8, pub ucSCLKIndex:u8, pub ucMCLKIndex:u8 }
#[repr(C, packed)]
pub struct ATOM_Tonga_VCE_State_Table { pub ucRevId:u8, pub ucNumEntries:u8, pub entries:[ATOM_Tonga_VCE_State_Record;0] }

#[repr(C, packed)]
pub struct ATOM_Tonga_PowerTune_Table { pub ucRevId:u8, pub usTDP:u16, pub usConfigurableTDP:u16, pub usTDC:u16, pub usBatteryPowerLimit:u16, pub usSmallPowerLimit:u16, pub usLowCACLeakage:u16, pub usHighCACLeakage:u16, pub usMaximumPowerDeliveryLimit:u16, pub usTjMax:u16, pub usPowerTuneDataSetID:u16, pub usEDCLimit:u16, pub usSoftwareShutdownTemp:u16, pub usClockStretchAmount:u16, pub usReserve:[u16;2] }
#[repr(C, packed)]
pub struct ATOM_Fiji_PowerTune_Table { pub ucRevId:u8, pub usTDP:u16, pub usConfigurableTDP:u16, pub usTDC:u16, pub usBatteryPowerLimit:u16, pub usSmallPowerLimit:u16, pub usLowCACLeakage:u16, pub usHighCACLeakage:u16, pub usMaximumPowerDeliveryLimit:u16, pub usTjMax:u16, pub usPowerTuneDataSetID:u16, pub usEDCLimit:u16, pub usSoftwareShutdownTemp:u16, pub usClockStretchAmount:u16, pub usTemperatureLimitHotspot:u16, pub usTemperatureLimitLiquid1:u16, pub usTemperatureLimitLiquid2:u16, pub usTemperatureLimitVrVddc:u16, pub usTemperatureLimitVrMvdd:u16, pub usTemperatureLimitPlx:u16, pub ucLiquid1_I2C_address:u8, pub ucLiquid2_I2C_address:u8, pub ucLiquid_I2C_Line:u8, pub ucVr_I2C_address:u8, pub ucVr_I2C_Line:u8, pub ucPlx_I2C_address:u8, pub ucPlx_I2C_Line:u8, pub usReserved:u16 }
#[repr(C, packed)]
pub struct ATOM_Polaris_PowerTune_Table { pub ucRevId:u8, pub usTDP:u16, pub usConfigurableTDP:u16, pub usTDC:u16, pub usBatteryPowerLimit:u16, pub usSmallPowerLimit:u16, pub usLowCACLeakage:u16, pub usHighCACLeakage:u16, pub usMaximumPowerDeliveryLimit:u16, pub usTjMax:u16, pub usPowerTuneDataSetID:u16, pub usEDCLimit:u16, pub usSoftwareShutdownTemp:u16, pub usClockStretchAmount:u16, pub usTemperatureLimitHotspot:u16, pub usTemperatureLimitLiquid1:u16, pub usTemperatureLimitLiquid2:u16, pub usTemperatureLimitVrVddc:u16, pub usTemperatureLimitVrMvdd:u16, pub usTemperatureLimitPlx:u16, pub ucLiquid1_I2C_address:u8, pub ucLiquid2_I2C_address:u8, pub ucLiquid_I2C_Line:u8, pub ucVr_I2C_address:u8, pub ucVr_I2C_Line:u8, pub ucPlx_I2C_address:u8, pub ucPlx_I2C_Line:u8, pub usBoostPowerLimit:u16, pub ucCKS_LDO_REFSEL:u8, pub ucHotSpotOnly:u8, pub ucReserve:u8, pub usReserve:u16 }

pub const ATOM_PPM_A_A: u8 = 1;
pub const ATOM_PPM_A_I: u8 = 2;
#[repr(C, packed)]
pub struct ATOM_Tonga_PPM_Table { pub ucRevId:u8, pub ucPpmDesign:u8, pub usCpuCoreNumber:u16, pub ulPlatformTDP:u32, pub ulSmallACPlatformTDP:u32, pub ulPlatformTDC:u32, pub ulSmallACPlatformTDC:u32, pub ulApuTDP:u32, pub ulDGpuTDP:u32, pub ulDGpuUlvPower:u32, pub ulTjmax:u32 }
#[repr(C, packed)]
pub struct ATOM_Tonga_Hard_Limit_Record { pub ulSCLKLimit:u32, pub ulMCLKLimit:u32, pub usVddcLimit:u16, pub usVddciLimit:u16, pub usVddgfxLimit:u16 }
#[repr(C, packed)]
pub struct ATOM_Tonga_Hard_Limit_Table { pub ucRevId:u8, pub ucNumEntries:u8, pub entries:[ATOM_Tonga_Hard_Limit_Record;0] }
#[repr(C, packed)]
pub struct ATOM_Tonga_GPIO_Table { pub ucRevId:u8, pub ucVRHotTriggeredSclkDpmIndex:u8, pub ucReserve:[u8;5] }
#[repr(C, packed)]
pub struct PPTable_Generic_SubTable_Header { pub ucRevId:u8 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
