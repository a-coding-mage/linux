// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for Digigram VX soundcards
 *
 * DSP commands
 *
 * Copyright (c) 2002 by Takashi Iwai <tiwai@suse.de>
 */

/* Dependencies from the original C includes:
 * <sound/core.h>
 * <sound/pcm.h>
 * <sound/vx_core.h>
 * "vx_cmd.h"
 */

/*
 * Array of DSP commands
 */
static vx_dsp_cmds: [vx_cmd_info; CMD_LAST_INDEX as usize] = {
    let mut cmds = [vx_cmd_info {
        opcode: 0,
        length: 0,
        st_type: 0,
        st_length: 0,
    }; CMD_LAST_INDEX as usize];

    cmds[CMD_VERSION as usize] = vx_cmd_info { opcode: 0x010000, length: 2, st_type: RMH_SSIZE_FIXED, st_length: 1 };
    cmds[CMD_SUPPORTED as usize] = vx_cmd_info { opcode: 0x020000, length: 1, st_type: RMH_SSIZE_FIXED, st_length: 2 };
    cmds[CMD_TEST_IT as usize] = vx_cmd_info { opcode: 0x040000, length: 1, st_type: RMH_SSIZE_FIXED, st_length: 1 };
    cmds[CMD_SEND_IRQA as usize] = vx_cmd_info { opcode: 0x070001, length: 1, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_IBL as usize] = vx_cmd_info { opcode: 0x080000, length: 1, st_type: RMH_SSIZE_FIXED, st_length: 4 };
    cmds[CMD_ASYNC as usize] = vx_cmd_info { opcode: 0x0A0000, length: 1, st_type: RMH_SSIZE_ARG, st_length: 0 };
    cmds[CMD_RES_PIPE as usize] = vx_cmd_info { opcode: 0x400000, length: 1, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_FREE_PIPE as usize] = vx_cmd_info { opcode: 0x410000, length: 1, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_CONF_PIPE as usize] = vx_cmd_info { opcode: 0x42A101, length: 2, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_ABORT_CONF_PIPE as usize] = vx_cmd_info { opcode: 0x42A100, length: 2, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_PARAM_OUTPUT_PIPE as usize] = vx_cmd_info { opcode: 0x43A000, length: 2, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_STOP_PIPE as usize] = vx_cmd_info { opcode: 0x470004, length: 1, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_PIPE_STATE as usize] = vx_cmd_info { opcode: 0x480000, length: 1, st_type: RMH_SSIZE_FIXED, st_length: 1 };
    cmds[CMD_PIPE_SPL_COUNT as usize] = vx_cmd_info { opcode: 0x49A000, length: 2, st_type: RMH_SSIZE_FIXED, st_length: 2 };
    cmds[CMD_CAN_START_PIPE as usize] = vx_cmd_info { opcode: 0x4b0000, length: 1, st_type: RMH_SSIZE_FIXED, st_length: 1 };
    cmds[CMD_SIZE_HBUFFER as usize] = vx_cmd_info { opcode: 0x4C0000, length: 1, st_type: RMH_SSIZE_FIXED, st_length: 1 };
    cmds[CMD_START_STREAM as usize] = vx_cmd_info { opcode: 0x80A000, length: 2, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_START_ONE_STREAM as usize] = vx_cmd_info { opcode: 0x800000, length: 1, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_PAUSE_STREAM as usize] = vx_cmd_info { opcode: 0x81A000, length: 2, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_PAUSE_ONE_STREAM as usize] = vx_cmd_info { opcode: 0x810000, length: 1, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_STREAM_OUT_LEVEL_ADJUST as usize] = vx_cmd_info { opcode: 0x828000, length: 2, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_STOP_STREAM as usize] = vx_cmd_info { opcode: 0x830000, length: 1, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_FORMAT_STREAM_OUT as usize] = vx_cmd_info { opcode: 0x868000, length: 1, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_FORMAT_STREAM_IN as usize] = vx_cmd_info { opcode: 0x878800, length: 1, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_GET_STREAM_STATE as usize] = vx_cmd_info { opcode: 0x890001, length: 2, st_type: RMH_SSIZE_FIXED, st_length: 1 };
    cmds[CMD_DROP_BYTES_AWAY as usize] = vx_cmd_info { opcode: 0x8A8000, length: 2, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_GET_REMAINING_BYTES as usize] = vx_cmd_info { opcode: 0x8D0800, length: 1, st_type: RMH_SSIZE_FIXED, st_length: 2 };
    cmds[CMD_CONNECT_AUDIO as usize] = vx_cmd_info { opcode: 0xC10000, length: 1, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_AUDIO_LEVEL_ADJUST as usize] = vx_cmd_info { opcode: 0xC2A000, length: 3, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_AUDIO_VU_PIC_METER as usize] = vx_cmd_info { opcode: 0xC3A003, length: 2, st_type: RMH_SSIZE_FIXED, st_length: 1 };
    cmds[CMD_GET_AUDIO_LEVELS as usize] = vx_cmd_info { opcode: 0xC4A000, length: 2, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_GET_NOTIFY_EVENT as usize] = vx_cmd_info { opcode: 0x4D0000, length: 1, st_type: RMH_SSIZE_ARG, st_length: 0 };
    cmds[CMD_INFO_NOTIFIED as usize] = vx_cmd_info { opcode: 0x0B0000, length: 1, st_type: RMH_SSIZE_FIXED, st_length: 2 };
    cmds[CMD_ACCESS_IO_FCT as usize] = vx_cmd_info { opcode: 0x098000, length: 1, st_type: RMH_SSIZE_ARG, st_length: 0 };
    cmds[CMD_STATUS_R_BUFFERS as usize] = vx_cmd_info { opcode: 0x440000, length: 1, st_type: RMH_SSIZE_ARG, st_length: 0 };
    cmds[CMD_UPDATE_R_BUFFERS as usize] = vx_cmd_info { opcode: 0x848000, length: 4, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_LOAD_EFFECT_CONTEXT as usize] = vx_cmd_info { opcode: 0x0c8000, length: 3, st_type: RMH_SSIZE_FIXED, st_length: 1 };
    cmds[CMD_EFFECT_ONE_PIPE as usize] = vx_cmd_info { opcode: 0x458000, length: 0, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_MODIFY_CLOCK as usize] = vx_cmd_info { opcode: 0x0d0000, length: 1, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_STREAM1_OUT_SET_N_LEVELS as usize] = vx_cmd_info { opcode: 0x858000, length: 3, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_PURGE_STREAM_DCMDS as usize] = vx_cmd_info { opcode: 0x8b8000, length: 3, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_NOTIFY_PIPE_TIME as usize] = vx_cmd_info { opcode: 0x4e0000, length: 1, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_LOAD_EFFECT_CONTEXT_PACKET as usize] = vx_cmd_info { opcode: 0x0c8000, length: 1, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_RELIC_R_BUFFER as usize] = vx_cmd_info { opcode: 0x8e0800, length: 1, st_type: RMH_SSIZE_FIXED, st_length: 1 };
    cmds[CMD_RESYNC_AUDIO_INPUTS as usize] = vx_cmd_info { opcode: 0x0e0000, length: 1, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_NOTIFY_STREAM_TIME as usize] = vx_cmd_info { opcode: 0x8f0000, length: 1, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_STREAM_SAMPLE_COUNT as usize] = vx_cmd_info { opcode: 0x900000, length: 1, st_type: RMH_SSIZE_FIXED, st_length: 2 };
    cmds[CMD_CONFIG_TIME_CODE as usize] = vx_cmd_info { opcode: 0x050000, length: 2, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_GET_TIME_CODE as usize] = vx_cmd_info { opcode: 0x060000, length: 1, st_type: RMH_SSIZE_FIXED, st_length: 5 };
    cmds[CMD_MANAGE_SIGNAL as usize] = vx_cmd_info { opcode: 0x0f0000, length: 1, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_PARAMETER_STREAM_OUT as usize] = vx_cmd_info { opcode: 0x91A000, length: 3, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_READ_BOARD_FREQ as usize] = vx_cmd_info { opcode: 0x030000, length: 1, st_type: RMH_SSIZE_FIXED, st_length: 2 };
    cmds[CMD_GET_STREAM_LEVELS as usize] = vx_cmd_info { opcode: 0x8c0000, length: 1, st_type: RMH_SSIZE_FIXED, st_length: 3 };
    cmds[CMD_PURGE_PIPE_DCMDS as usize] = vx_cmd_info { opcode: 0x4f8000, length: 3, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    // [CMD_SET_STREAM_OUT_EFFECTS] = { 0x888000, 34, RMH_SSIZE_FIXED, 0 },
    // [CMD_GET_STREAM_OUT_EFFECTS] = { 0x928000, 2, RMH_SSIZE_FIXED, 32 },
    cmds[CMD_CONNECT_MONITORING as usize] = vx_cmd_info { opcode: 0xC00000, length: 1, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_STREAM2_OUT_SET_N_LEVELS as usize] = vx_cmd_info { opcode: 0x938000, length: 3, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_CANCEL_R_BUFFERS as usize] = vx_cmd_info { opcode: 0x948000, length: 4, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_NOTIFY_END_OF_BUFFER as usize] = vx_cmd_info { opcode: 0x950000, length: 1, st_type: RMH_SSIZE_FIXED, st_length: 0 };
    cmds[CMD_GET_STREAM_VU_METER as usize] = vx_cmd_info { opcode: 0x95A000, length: 2, st_type: RMH_SSIZE_ARG, st_length: 0 };

    cmds
};

/**
 * vx_init_rmh - initialize the RMH instance
 * @rmh: the rmh pointer to be initialized
 * @cmd: the rmh command to be set
 */
pub unsafe extern "C" fn vx_init_rmh(rmh: *mut vx_rmh, cmd: ::core::ffi::c_uint) {
    if snd_BUG_ON(cmd >= CMD_LAST_INDEX) {
        return;
    }
    (*rmh).LgCmd = vx_dsp_cmds[cmd as usize].length;
    (*rmh).LgStat = vx_dsp_cmds[cmd as usize].st_length;
    (*rmh).DspStat = vx_dsp_cmds[cmd as usize].st_type;
    (*rmh).Cmd[0] = vx_dsp_cmds[cmd as usize].opcode;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
