/*
 * Direct Rust ABI translation of dmub_cmd.h.
 * C-only preprocessing and includes are represented by comments; external
 * symbols are intentionally left as dependencies of the containing crate.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

pub const SET_ABM_PIPE_GRADUALLY_DISABLE: u8 = 0;
pub const SET_ABM_PIPE_IMMEDIATELY_DISABLE: u8 = 255;
pub const SET_ABM_PIPE_IMMEDIATE_KEEP_GAIN_DISABLE: u8 = 254;
pub const SET_ABM_PIPE_NORMAL: u8 = 1;
pub const NUM_AMBI_LEVEL: usize = 5;
pub const NUM_AGGR_LEVEL: usize = 4;
pub const NUM_POWER_FN_SEGS: usize = 8;
pub const NUM_BL_CURVE_SEGS: usize = 16;
pub const ABM_MAX_NUM_OF_ACE_SEGMENTS: usize = 64;
pub const ABM_MAX_NUM_OF_HG_BINS: usize = 64;
pub const SET_CACP_PIPE_GRADUALLY_DISABLE: u8 = 0;
pub const SET_CACP_PIPE_IMMEDIATELY_DISABLE: u8 = 255;
pub const SET_CACP_PIPE_IMMEDIATE_KEEP_GAIN_DISABLE: u8 = 254;
pub const SET_CACP_PIPE_IMMEDIATE_ON_NEXT_DISABLE: u8 = 253;
pub const SET_CACP_PIPE_NORMAL: u8 = 1;
pub const DMUB_MAX_SUBVP_STREAMS: usize = 2;
pub const DMUB_MAX_FPO_STREAMS: usize = 4;
pub const DMUB_MAX_STREAMS: usize = 6;
pub const DMUB_MAX_PLANES: usize = 6;
pub const DMUB_MAX_PHANTOM_PLANES: usize = DMUB_MAX_PLANES / 2;
pub const TRACE_BUFFER_ENTRY_OFFSET: usize = 16;
pub const DMUB_MAX_DIRTY_RECTS: usize = 3;
pub const DMUB_CMD_PSR_CONTROL_VERSION_UNKNOWN: u32 = 0x0;
pub const DMUB_CMD_PSR_CONTROL_VERSION_1: u32 = 0x1;
pub const DMUB_CMD_DIRTY_RECTS_VERSION_UNKNOWN: u32 = 0x0;
pub const DMUB_CMD_DIRTY_RECTS_VERSION_1: u32 = 0x1;
pub const DMUB_CMD_DIRTY_RECTS_VERSION_2: u32 = 0x2;
pub const DMUB_CMD_CURSOR_UPDATE_VERSION_UNKNOWN: u32 = 0x0;
pub const DMUB_CMD_CURSOR_UPDATE_VERSION_1: u32 = 0x1;
pub const DMUB_CMD_CURSOR_UPDATE_VERSION_2: u32 = 0x2;
pub const DMUB_CMD_ABM_CONTROL_VERSION_UNKNOWN: u32 = 0x0;
pub const DMUB_CMD_ABM_CONTROL_VERSION_1: u32 = 0x1;
pub const DMUB_CMD_CACP_CONTROL_VERSION_UNKNOWN: u32 = 0x0;
pub const DMUB_CMD_CACP_CONTROL_VERSION_1: u32 = 0x1;
pub const DMUB_CMD_CACP_CONTROL_MODE_0: u32 = 0x0;
pub const DMUB_CMD_CACP_CONTROL_MODE_1: u32 = 0x1;
pub const ABM_NUM_OF_ACE_SEGMENTS: usize = 5;
pub const DMUB_DEBUG_FW_STATE_OFFSET: usize = 0x300;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct abm_flags_bitfields { pub bits: u32 }
#[repr(C)]
pub union abm_flags { pub bitfields: abm_flags_bitfields, pub u32All: u32 }

#[repr(C, packed)]
pub struct abm_save_restore {
    pub flags: abm_flags,
    pub pause: u32,
    pub next_ace_slope: [u32; ABM_NUM_OF_ACE_SEGMENTS],
    pub next_ace_thresh: [u32; ABM_NUM_OF_ACE_SEGMENTS],
    pub next_ace_offset: [u32; ABM_NUM_OF_ACE_SEGMENTS],
    pub knee_threshold: u32,
    pub current_gain: u32,
    pub curr_bl_level: u16,
    pub curr_user_bl_level: u16,
}

#[repr(C)]
pub union dmub_addr {
    pub u: dmub_addr_parts,
    pub quad_part: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct dmub_addr_parts { pub low_part: u32, pub high_part: u32 }

#[repr(C, packed)]
pub struct dmub_soc_bb_params {
    pub dram_clk_change_blackout_ns: u32,
    pub dram_clk_change_read_only_ns: u32,
    pub dram_clk_change_write_only_ns: u32,
    pub fclk_change_blackout_ns: u32,
    pub g7_ppt_blackout_ns: u32,
    pub stutter_enter_plus_exit_latency_ns: u32,
    pub stutter_exit_latency_ns: u32,
    pub z8_stutter_enter_plus_exit_latency_ns: u32,
    pub z8_stutter_exit_latency_ns: u32,
    pub z8_min_idle_time_ns: u32,
    pub type_b_dram_clk_change_blackout_ns: u32,
    pub type_b_ppt_blackout_ns: u32,
    pub vmin_limit_dispclk_khz: u32,
    pub vmin_limit_dcfclk_khz: u32,
    pub g7_temperature_read_blackout_ns: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct dmub_rect { pub x: u32, pub y: u32, pub width: u32, pub height: u32 }

#[repr(C)]
pub union dmub_psr_debug_flags { pub bitfields: dmub_psr_debug_flags_bits, pub u32All: u32 }
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct dmub_psr_debug_flags_bits { pub bits: u32 }

#[repr(C)]
pub union replay_debug_flags { pub bitfields: replay_debug_flags_bits, pub u32All: u32 }
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct replay_debug_flags_bits { pub bits: u32 }

#[repr(C)]
pub union replay_visual_confirm_error_state_flags { pub bitfields: replay_visual_confirm_error_state_flags_bits, pub u32All: u32 }
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct replay_visual_confirm_error_state_flags_bits { pub bits: u32 }

#[repr(C)]
pub union replay_hw_flags { pub bitfields: replay_hw_flags_bits, pub u32All: u32 }
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct replay_hw_flags_bits { pub bits: u32 }

#[repr(C)]
pub union pr_debug_flags { pub bitfields: pr_debug_flags_bits, pub u32All: u32 }
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct pr_debug_flags_bits { pub bits: u32 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
