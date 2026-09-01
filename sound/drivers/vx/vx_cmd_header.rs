/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Driver for Digigram VX soundcards
 *
 * Definitions of DSP commands
 *
 * Copyright (c) 2002 by Takashi Iwai <tiwai@suse.de>
 */

// C header dependencies intentionally left external:
// struct vx_rmh and its Cmd field are supplied by other translated files.

pub const CMD_VERSION: u32 = 0;
pub const CMD_SUPPORTED: u32 = 1;
pub const CMD_TEST_IT: u32 = 2;
pub const CMD_SEND_IRQA: u32 = 3;
pub const CMD_IBL: u32 = 4;
pub const CMD_ASYNC: u32 = 5;
pub const CMD_RES_PIPE: u32 = 6;
pub const CMD_FREE_PIPE: u32 = 7;
pub const CMD_CONF_PIPE: u32 = 8;
pub const CMD_ABORT_CONF_PIPE: u32 = 9;
pub const CMD_PARAM_OUTPUT_PIPE: u32 = 10;
pub const CMD_STOP_PIPE: u32 = 11;
pub const CMD_PIPE_STATE: u32 = 12;
pub const CMD_PIPE_SPL_COUNT: u32 = 13;
pub const CMD_CAN_START_PIPE: u32 = 14;
pub const CMD_SIZE_HBUFFER: u32 = 15;
pub const CMD_START_STREAM: u32 = 16;
pub const CMD_START_ONE_STREAM: u32 = 17;
pub const CMD_PAUSE_STREAM: u32 = 18;
pub const CMD_PAUSE_ONE_STREAM: u32 = 19;
pub const CMD_STREAM_OUT_LEVEL_ADJUST: u32 = 20;
pub const CMD_STOP_STREAM: u32 = 21;
pub const CMD_FORMAT_STREAM_OUT: u32 = 22;
pub const CMD_FORMAT_STREAM_IN: u32 = 23;
pub const CMD_GET_STREAM_STATE: u32 = 24;
pub const CMD_DROP_BYTES_AWAY: u32 = 25;
pub const CMD_GET_REMAINING_BYTES: u32 = 26;
pub const CMD_CONNECT_AUDIO: u32 = 27;
pub const CMD_AUDIO_LEVEL_ADJUST: u32 = 28;
pub const CMD_AUDIO_VU_PIC_METER: u32 = 29;
pub const CMD_GET_AUDIO_LEVELS: u32 = 30;
pub const CMD_GET_NOTIFY_EVENT: u32 = 31;
pub const CMD_INFO_NOTIFIED: u32 = 32;
pub const CMD_ACCESS_IO_FCT: u32 = 33;
pub const CMD_STATUS_R_BUFFERS: u32 = 34;
pub const CMD_UPDATE_R_BUFFERS: u32 = 35;
pub const CMD_LOAD_EFFECT_CONTEXT: u32 = 36;
pub const CMD_EFFECT_ONE_PIPE: u32 = 37;
pub const CMD_MODIFY_CLOCK: u32 = 38;
pub const CMD_STREAM1_OUT_SET_N_LEVELS: u32 = 39;
pub const CMD_PURGE_STREAM_DCMDS: u32 = 40;
pub const CMD_NOTIFY_PIPE_TIME: u32 = 41;
pub const CMD_LOAD_EFFECT_CONTEXT_PACKET: u32 = 42;
pub const CMD_RELIC_R_BUFFER: u32 = 43;
pub const CMD_RESYNC_AUDIO_INPUTS: u32 = 44;
pub const CMD_NOTIFY_STREAM_TIME: u32 = 45;
pub const CMD_STREAM_SAMPLE_COUNT: u32 = 46;
pub const CMD_CONFIG_TIME_CODE: u32 = 47;
pub const CMD_GET_TIME_CODE: u32 = 48;
pub const CMD_MANAGE_SIGNAL: u32 = 49;
pub const CMD_PARAMETER_STREAM_OUT: u32 = 50;
pub const CMD_READ_BOARD_FREQ: u32 = 51;
pub const CMD_GET_STREAM_LEVELS: u32 = 52;
pub const CMD_PURGE_PIPE_DCMDS: u32 = 53;
// CMD_SET_STREAM_OUT_EFFECTS
// CMD_GET_STREAM_OUT_EFFECTS
pub const CMD_CONNECT_MONITORING: u32 = 54;
pub const CMD_STREAM2_OUT_SET_N_LEVELS: u32 = 55;
pub const CMD_CANCEL_R_BUFFERS: u32 = 56;
pub const CMD_NOTIFY_END_OF_BUFFER: u32 = 57;
pub const CMD_GET_STREAM_VU_METER: u32 = 58;
pub const CMD_LAST_INDEX: u32 = 59;

#[repr(C)]
pub struct vx_cmd_info {
    pub opcode: u32,    /* command word */
    pub length: i32,    /* command length (in words) */
    pub st_type: i32,   /* status type (RMH_SSIZE_XXX) */
    pub st_length: i32, /* fixed length */
}

/* Family and code op of some DSP requests. */
pub const CODE_OP_PIPE_TIME: u32 = 0x004e0000;
pub const CODE_OP_START_STREAM: u32 = 0x00800000;
pub const CODE_OP_PAUSE_STREAM: u32 = 0x00810000;
pub const CODE_OP_OUT_STREAM_LEVEL: u32 = 0x00820000;
pub const CODE_OP_UPDATE_R_BUFFERS: u32 = 0x00840000;
pub const CODE_OP_OUT_STREAM1_LEVEL_CURVE: u32 = 0x00850000;
pub const CODE_OP_OUT_STREAM2_LEVEL_CURVE: u32 = 0x00930000;
pub const CODE_OP_OUT_STREAM_FORMAT: u32 = 0x00860000;
pub const CODE_OP_STREAM_TIME: u32 = 0x008f0000;
pub const CODE_OP_OUT_STREAM_EXTRAPARAMETER: u32 = 0x00910000;
pub const CODE_OP_OUT_AUDIO_LEVEL: u32 = 0x00c20000;

pub const NOTIFY_LAST_COMMAND: u32 = 0x00400000;

/* Values for a user delay */
pub const DC_DIFFERED_DELAY: u32 = 1 << BIT_DIFFERED_COMMAND;
pub const DC_NOTIFY_DELAY: u32 = 1 << BIT_NOTIFIED_COMMAND;
pub const DC_HBUFFER_DELAY: u32 = 1 << BIT_TIME_RELATIVE_TO_BUFFER;
pub const DC_MULTIPLE_DELAY: u32 = 1 << BIT_RESERVED;
pub const DC_STREAM_TIME_DELAY: u32 = 1 << BIT_STREAM_TIME;
pub const DC_CANCELLED_DELAY: u32 = 1 << BIT_CANCELLED_COMMAND;

/* Values for tiDelayed field in TIME_INFO structure,
 * and for pbPause field in PLAY_BUFFER_INFO structure
 */
pub const BIT_DIFFERED_COMMAND: u32 = 0;
pub const BIT_NOTIFIED_COMMAND: u32 = 1;
pub const BIT_TIME_RELATIVE_TO_BUFFER: u32 = 2;
pub const BIT_RESERVED: u32 = 3;
pub const BIT_STREAM_TIME: u32 = 4;
pub const BIT_CANCELLED_COMMAND: u32 = 5;

/* Access to the "Size" field of the response of the CMD_GET_NOTIFY_EVENT request. */
pub const GET_NOTIFY_EVENT_SIZE_FIELD_MASK: u32 = 0x000000ff;

/* DSP commands general masks */
pub const OPCODE_MASK: u32 = 0x00ff0000;
pub const DSP_DIFFERED_COMMAND_MASK: u32 = 0x0000C000;

/* Notifications (NOTIFY_INFO) */
pub const ALL_CMDS_NOTIFIED: u32 = 0x0000; /* reserved */
pub const START_STREAM_NOTIFIED: u32 = 0x0001;
pub const PAUSE_STREAM_NOTIFIED: u32 = 0x0002;
pub const OUT_STREAM_LEVEL_NOTIFIED: u32 = 0x0003;
pub const OUT_STREAM_PARAMETER_NOTIFIED: u32 = 0x0004; /* left for backward compatibility */
pub const OUT_STREAM_FORMAT_NOTIFIED: u32 = 0x0004;
pub const PIPE_TIME_NOTIFIED: u32 = 0x0005;
pub const OUT_AUDIO_LEVEL_NOTIFIED: u32 = 0x0006;
pub const OUT_STREAM_LEVEL_CURVE_NOTIFIED: u32 = 0x0007;
pub const STREAM_TIME_NOTIFIED: u32 = 0x0008;
pub const OUT_STREAM_EXTRAPARAMETER_NOTIFIED: u32 = 0x0009;
pub const UNKNOWN_COMMAND_NOTIFIED: u32 = 0xffff;

/* Output pipe parameters setting */
pub const MASK_VALID_PIPE_MPEG_PARAM: u32 = 0x000040;
pub const MASK_VALID_PIPE_BACKWARD_PARAM: u32 = 0x000020;
pub const MASK_SET_PIPE_MPEG_PARAM: u32 = 0x000002;
pub const MASK_SET_PIPE_BACKWARD_PARAM: u32 = 0x000001;

pub const MASK_DSP_WORD: u32 = 0x00FFFFFF;
pub const MASK_ALL_STREAM: u32 = 0x00FFFFFF;
pub const MASK_DSP_WORD_LEVEL: u32 = 0x000001FF;
pub const MASK_FIRST_FIELD: u32 = 0x0000001F;
pub const FIELD_SIZE: u32 = 5;

pub const COMMAND_RECORD_MASK: u32 = 0x000800;

/* PipeManagement definition bits (PIPE_DECL_INFO) */
pub const P_UNDERRUN_SKIP_SOUND_MASK: u32 = 0x01;
pub const P_PREPARE_FOR_MPEG3_MASK: u32 = 0x02;
pub const P_DO_NOT_RESET_ANALOG_LEVELS: u32 = 0x04;
pub const P_ALLOW_UNDER_ALLOCATION_MASK: u32 = 0x08;
pub const P_DATA_MODE_MASK: u32 = 0x10;
pub const P_ASIO_BUFFER_MANAGEMENT_MASK: u32 = 0x20;

pub const BIT_SKIP_SOUND: u32 = 0x08; /* bit 3 */
pub const BIT_DATA_MODE: u32 = 0x10; /* bit 4 */

/* Bits in the CMD_MODIFY_CLOCK request. */
pub const CMD_MODIFY_CLOCK_FD_BIT: u32 = 0x00000001;
pub const CMD_MODIFY_CLOCK_T_BIT: u32 = 0x00000002;
pub const CMD_MODIFY_CLOCK_S_BIT: u32 = 0x00000004;

/* Access to the results of the CMD_GET_TIME_CODE RMH. */
pub const TIME_CODE_V_MASK: u32 = 0x00800000;
pub const TIME_CODE_N_MASK: u32 = 0x00400000;
pub const TIME_CODE_B_MASK: u32 = 0x00200000;
pub const TIME_CODE_W_MASK: u32 = 0x00100000;

/* Values for the CMD_MANAGE_SIGNAL RMH. */
pub const MANAGE_SIGNAL_TIME_CODE: u32 = 0x01;
pub const MANAGE_SIGNAL_MIDI: u32 = 0x02;

/* Values for the CMD_CONFIG_TIME_CODE RMH. */
pub const CONFIG_TIME_CODE_CANCEL: u32 = 0x00001000;

/* Mask to get only the effective time from the
 * high word out of the 2 returned by the DSP
 */
pub const PCX_TIME_HI_MASK: u32 = 0x000fffff;

/* Values for setting a H-Buffer time */
pub const HBUFFER_TIME_HIGH: u32 = 0x00200000;
pub const HBUFFER_TIME_LOW: u32 = 0x00000000;

pub const NOTIFY_MASK_TIME_HIGH: u32 = 0x00400000;
pub const MULTIPLE_MASK_TIME_HIGH: u32 = 0x00100000;
pub const STREAM_MASK_TIME_HIGH: u32 = 0x00800000;

extern "C" {
    pub fn vx_init_rmh(rmh: *mut vx_rmh, cmd: u32);
}

/**
 * vx_set_pipe_cmd_params - fill first command word for pipe commands
 * @rmh: the rmh to be modified
 * @is_capture: 0 = playback, 1 = capture operation
 * @param1: first pipe-parameter
 * @param2: second pipe-parameter
 */
pub unsafe fn vx_set_pipe_cmd_params(
    rmh: *mut vx_rmh,
    is_capture: i32,
    param1: i32,
    param2: i32,
) {
    if is_capture != 0 {
        (*rmh).Cmd[0] |= COMMAND_RECORD_MASK;
    }
    (*rmh).Cmd[0] |= (((param1 as u32) & MASK_FIRST_FIELD) << FIELD_SIZE) & MASK_DSP_WORD;

    if param2 != 0 {
        (*rmh).Cmd[0] |= ((param2 as u32) & MASK_FIRST_FIELD) & MASK_DSP_WORD;
    }
}

/**
 * vx_set_stream_cmd_params - fill first command word for stream commands
 * @rmh: the rmh to be modified
 * @is_capture: 0 = playback, 1 = capture operation
 * @pipe: the pipe index (zero-based)
 */
pub unsafe fn vx_set_stream_cmd_params(rmh: *mut vx_rmh, is_capture: i32, pipe: i32) {
    if is_capture != 0 {
        (*rmh).Cmd[0] |= COMMAND_RECORD_MASK;
    }
    (*rmh).Cmd[0] |= (((pipe as u32) & MASK_FIRST_FIELD) << FIELD_SIZE) & MASK_DSP_WORD;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
