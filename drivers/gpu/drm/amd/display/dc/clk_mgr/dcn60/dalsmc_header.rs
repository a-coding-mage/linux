// SPDX-License-Identifier: MIT
//
// Copyright 2026 Advanced Micro Devices, Inc.

//! VBIOS and DAL to PMFW Interface.
//!
//! This is a direct Rust translation of `dalsmc.h`. C preprocessor include and
//! header-guard directives are intentionally omitted.

pub const DALSMC_VERSION: u32 = 0x1;

pub const DALSMC_Result_OK: u32 = 0x01;
pub const DALSMC_Result_Failed: u32 = 0xFF;
pub const DALSMC_Result_UnknownCmd: u32 = 0xFE;
pub const DALSMC_Result_CmdRejectedPrereq: u32 = 0xFD;
pub const DALSMC_Result_CmdRejectedBusy: u32 = 0xFC;

/// Generic register overlay — four 32-bit C2PMSG argument registers.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DALSMC_args_t { pub Reg0: u32, pub Reg1: u32, pub Reg2: u32, pub Reg3: u32 }

pub const DALSMC_MSG_TestMessage: u32 = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DALSMC_TestMessage_fields { pub TestValue: u32, pub Reserved: [u32; 3] }
#[repr(C)]
#[derive(Copy, Clone)]
pub union DALSMC_TestMessage_arg_t { pub fields: DALSMC_TestMessage_fields, pub Args: DALSMC_args_t }

pub const DALSMC_MSG_GetMsgHeaderVersion: u32 = 0x02;

pub const DALSMC_MSG_TransferTableSmu2Dram: u32 = 0x03;
pub const DALSMC_MSG_TransferTableDram2Smu: u32 = 0x04;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DALSMC_TransferTable_fields { pub TableId: u32, pub AddrLow: u32, pub AddrHigh: u32, pub Reserved: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub union DALSMC_TransferTable_arg_t { pub fields: DALSMC_TransferTable_fields, pub Args: DALSMC_args_t }

pub const DALSMC_MSG_SetHardMinByFreq: u32 = 0x05;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DALSMC_SetHardMinByFreq_fields { pub FreqKhz: u32, pub Ppclk: u32, pub Reserved: [u32; 3] }
#[repr(C)]
#[derive(Copy, Clone)]
pub union DALSMC_SetHardMinByFreq_arg_t { pub fields: DALSMC_SetHardMinByFreq_fields, pub Args: DALSMC_args_t }

pub const DALSMC_MSG_SetMinDeepSleepDcfclk: u32 = 0x06;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DALSMC_SetMinDeepSleepDcfclk_fields { pub MinDcfclkMhz: u32, pub Reserved: [u32; 3] }
#[repr(C)]
#[derive(Copy, Clone)]
pub union DALSMC_SetMinDeepSleepDcfclk_arg_t { pub fields: DALSMC_SetMinDeepSleepDcfclk_fields, pub Args: DALSMC_args_t }

pub const DALSMC_MSG_BacoAudioD3PME: u32 = 0x07;
pub const DALSMC_MSG_ReturnHardMinStatus: u32 = 0x08;

pub const DALSMC_MSG_IndicatePstateStatus: u32 = 0x09;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DALSMC_IndicatePstateStatus_fields { pub Flags: u32, pub Reserved2: [u32; 3] }
#[repr(C)]
#[derive(Copy, Clone)]
pub union DALSMC_IndicatePstateStatus_arg_t { pub fields: DALSMC_IndicatePstateStatus_fields, pub Args: DALSMC_args_t }

pub const DALSMC_MSG_UpdateUTMQoSRequest: u32 = 0x0A;
#[repr(C)]
#[derive(Copy, Clone)]
pub union DALSMC_UpdateUTMQoSRequest_arg_t { pub fields: DALSMC_args_t, pub Args: DALSMC_args_t }

pub const DALSMC_MSG_SetDisplayIdleOptimizations: u32 = 0x0B;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DALSMC_SetDisplayIdleOptimizations_fields { pub Flags: u32, pub Reserved1: [u32; 3] }
#[repr(C)]
#[derive(Copy, Clone)]
pub union DALSMC_SetDisplayIdleOptimizations_arg_t { pub fields: DALSMC_SetDisplayIdleOptimizations_fields, pub Args: DALSMC_args_t }

pub const DALSMC_MSG_SetStutterEfficiency: u32 = 0x0C;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DALSMC_SetStutterEfficiency_fields { pub Efficiencies: u32, pub Reserved1: [u32; 3] }
#[repr(C)]
#[derive(Copy, Clone)]
pub union DALSMC_SetStutterEfficiency_arg_t { pub fields: DALSMC_SetStutterEfficiency_fields, pub Args: DALSMC_args_t }

pub const DALSMC_Message_Count: u32 = 0x0D;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SocUtmTableHeader_t { pub Flags: u32 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SocUtmSopEntry_t {
    pub UrgentRampPs: u32, pub TripPs: u32, pub MetaTripToMemPs: u32,
    pub MaxReqLatencyUrgPs: u32, pub AvgReqLatencyUrgPs: u32,
    pub MaxReqLatencyNonUrgPs: u32, pub AvgReqLatencyNonUrgPs: u32,
    pub DfResponseTimePs: u32, pub UrgentBandwidthKBps: u32,
    pub NominalBandwidthKBps: u32, pub LsdmaBandwidthKBps: u32, pub Reserved: [u32; 1],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DalInitHeader_t { pub SmuVersion: u32, pub SmuDriverIfVersion: u32, pub Reserved: [u32; 2] }
pub const NUM_CLOCK_LEVELS: usize = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DpmClock_t { pub Clocks: [u32; NUM_CLOCK_LEVELS], pub DcMaxClock: u32, pub NumClocks: u32, pub Reserved: [u32; 2] }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryConfig_t { pub NumUmcChannels: u32, pub Reserved: [u32; 3] }

pub const TABLE_SOC_UTM: u32 = 0xC;
pub const DALSMC_MAX_UTM_SOP_COUNT: usize = 16;
pub const MAX_UTM_LOAD_LEVEL_COUNT: usize = 16;
pub const UTM_LOAD_LEVEL_INDEX_IDLE: usize = 0;
pub const UTM_LOAD_LEVEL_INDEX_ACTIVE_ALTERNATE_PSTATE: usize = 1;
pub const UTM_LOAD_LEVEL_INDEX_ACTIVE: usize = 2;
#[inline] pub const fn UTM_SOP_ENTRIES_OFFSET(load_level: usize, sop_index: usize) -> usize { std::mem::size_of::<SocUtmTableHeader_t>() + (load_level * DALSMC_MAX_UTM_SOP_COUNT + sop_index) * std::mem::size_of::<SocUtmSopEntry_t>() }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SocUtmTable_t { pub Header: SocUtmTableHeader_t, pub Sops: [[SocUtmSopEntry_t; DALSMC_MAX_UTM_SOP_COUNT]; MAX_UTM_LOAD_LEVEL_COUNT] }

pub const TABLE_DAL_INIT: u32 = 0xD;
pub const MAX_PPCLK_COUNT: usize = 12;
#[inline] pub const fn DPM_CLOCK_OFFSET(ppclk: usize) -> usize { std::mem::size_of::<DalInitHeader_t>() + ppclk * std::mem::size_of::<DpmClock_t>() }
#[inline] pub const fn UTM_TABLE_OFFSET() -> usize { std::mem::size_of::<DalInitHeader_t>() + MAX_PPCLK_COUNT * std::mem::size_of::<DpmClock_t>() }
#[inline] pub const fn MEMORY_CONFIG_OFFSET() -> usize { std::mem::size_of::<DalInitHeader_t>() + MAX_PPCLK_COUNT * std::mem::size_of::<DpmClock_t>() + std::mem::size_of::<SocUtmTable_t>() }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DalInitTable_t { pub Header: DalInitHeader_t, pub PPClocks: [DpmClock_t; MAX_PPCLK_COUNT], pub UtmTable: SocUtmTable_t, pub MemoryConfig: MemoryConfig_t }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
