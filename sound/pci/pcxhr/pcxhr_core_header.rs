/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Driver for Digigram pcxhr compatible soundcards
 *
 * low level interface with interrupt and message handling
 *
 * Copyright (c) 2004 by Digigram <alsa@digigram.com>
 */

/* Opaque declarations corresponding to C forward declarations. */
#[repr(C)]
pub struct firmware {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pcxhr_mgr {
    _private: [u8; 0],
}

/* init and firmware download commands */
unsafe extern "C" {
    pub fn pcxhr_reset_xilinx_com(mgr: *mut pcxhr_mgr);
    pub fn pcxhr_reset_dsp(mgr: *mut pcxhr_mgr);
    pub fn pcxhr_enable_dsp(mgr: *mut pcxhr_mgr);
    pub fn pcxhr_load_xilinx_binary(
        mgr: *mut pcxhr_mgr,
        xilinx: *const firmware,
        second: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn pcxhr_load_eeprom_binary(
        mgr: *mut pcxhr_mgr,
        eeprom: *const firmware,
    ) -> ::core::ffi::c_int;
    pub fn pcxhr_load_boot_binary(
        mgr: *mut pcxhr_mgr,
        boot: *const firmware,
    ) -> ::core::ffi::c_int;
    pub fn pcxhr_load_dsp_binary(
        mgr: *mut pcxhr_mgr,
        dsp: *const firmware,
    ) -> ::core::ffi::c_int;
}

/* DSP time available on MailBox4 register : 24 bit time samples() */
pub const PCXHR_DSP_TIME_MASK: u32 = 0x00ffffff;
pub const PCXHR_DSP_TIME_INVALID: u32 = 0x10000000;

pub const PCXHR_SIZE_MAX_CMD: usize = 8;
pub const PCXHR_SIZE_MAX_STATUS: usize = 16;
pub const PCXHR_SIZE_MAX_LONG_STATUS: usize = 256;

#[repr(C)]
pub struct pcxhr_rmh {
    pub cmd_len: u16,  /* length of the command to send (WORDs) */
    pub stat_len: u16, /* length of the status received (WORDs) */
    pub dsp_stat: u16, /* status type, RMP_SSIZE_XXX */
    pub cmd_idx: u16,  /* index of the command */
    pub cmd: [u32; PCXHR_SIZE_MAX_CMD],
    pub stat: [u32; PCXHR_SIZE_MAX_STATUS],
}

pub const CMD_VERSION: ::core::ffi::c_int = 0; /* cmd_len = 2	stat_len = 1 */
pub const CMD_SUPPORTED: ::core::ffi::c_int = 1; /* cmd_len = 1	stat_len = 4 */
pub const CMD_TEST_IT: ::core::ffi::c_int = 2; /* cmd_len = 1	stat_len = 1 */
pub const CMD_SEND_IRQA: ::core::ffi::c_int = 3; /* cmd_len = 1	stat_len = 0 */
pub const CMD_ACCESS_IO_WRITE: ::core::ffi::c_int = 4; /* cmd_len >= 1	stat_len >= 1 */
pub const CMD_ACCESS_IO_READ: ::core::ffi::c_int = 5; /* cmd_len >= 1	stat_len >= 1 */
pub const CMD_ASYNC: ::core::ffi::c_int = 6; /* cmd_len = 1	stat_len = 1 */
pub const CMD_MODIFY_CLOCK: ::core::ffi::c_int = 7; /* cmd_len = 3	stat_len = 0 */
pub const CMD_RESYNC_AUDIO_INPUTS: ::core::ffi::c_int = 8; /* cmd_len = 1	stat_len = 0 */
pub const CMD_GET_DSP_RESOURCES: ::core::ffi::c_int = 9; /* cmd_len = 1	stat_len = 4 */
pub const CMD_SET_TIMER_INTERRUPT: ::core::ffi::c_int = 10; /* cmd_len = 1	stat_len = 0 */
pub const CMD_RES_PIPE: ::core::ffi::c_int = 11; /* cmd_len >=2	stat_len = 0 */
pub const CMD_FREE_PIPE: ::core::ffi::c_int = 12; /* cmd_len = 1	stat_len = 0 */
pub const CMD_CONF_PIPE: ::core::ffi::c_int = 13; /* cmd_len = 2	stat_len = 0 */
pub const CMD_STOP_PIPE: ::core::ffi::c_int = 14; /* cmd_len = 1	stat_len = 0 */
pub const CMD_PIPE_SAMPLE_COUNT: ::core::ffi::c_int = 15; /* cmd_len = 2	stat_len = 2 */
pub const CMD_CAN_START_PIPE: ::core::ffi::c_int = 16; /* cmd_len >= 1	stat_len = 1 */
pub const CMD_START_STREAM: ::core::ffi::c_int = 17; /* cmd_len = 2	stat_len = 0 */
pub const CMD_STREAM_OUT_LEVEL_ADJUST: ::core::ffi::c_int = 18; /* cmd_len >= 1	stat_len = 0 */
pub const CMD_STOP_STREAM: ::core::ffi::c_int = 19; /* cmd_len = 2	stat_len = 0 */
pub const CMD_UPDATE_R_BUFFERS: ::core::ffi::c_int = 20; /* cmd_len = 4	stat_len = 0 */
pub const CMD_FORMAT_STREAM_OUT: ::core::ffi::c_int = 21; /* cmd_len >= 2	stat_len = 0 */
pub const CMD_FORMAT_STREAM_IN: ::core::ffi::c_int = 22; /* cmd_len >= 4	stat_len = 0 */
pub const CMD_STREAM_SAMPLE_COUNT: ::core::ffi::c_int = 23; /* cmd_len = 2	stat_len = (2 * nb_stream) */
pub const CMD_AUDIO_LEVEL_ADJUST: ::core::ffi::c_int = 24; /* cmd_len = 3	stat_len = 0 */
pub const CMD_GET_TIME_CODE: ::core::ffi::c_int = 25; /* cmd_len = 1  stat_len = 5 */
pub const CMD_MANAGE_SIGNAL: ::core::ffi::c_int = 26; /* cmd_len = 1  stat_len = 0 */
pub const CMD_LAST_INDEX: ::core::ffi::c_int = 27;

pub const MASK_DSP_WORD: u32 = 0x00ffffff;
pub const MASK_ALL_STREAM: u32 = 0x00ffffff;
pub const MASK_DSP_WORD_LEVEL: u32 = 0x000001ff;
pub const MASK_FIRST_FIELD: u32 = 0x0000001f;
pub const FIELD_SIZE: u32 = 5;

/*
 init the rmh struct; by default cmd_len is set to 1
 */
unsafe extern "C" {
    pub fn pcxhr_init_rmh(rmh: *mut pcxhr_rmh, cmd: ::core::ffi::c_int);

    pub fn pcxhr_set_pipe_cmd_params(
        rmh: *mut pcxhr_rmh,
        capture: ::core::ffi::c_int,
        param1: ::core::ffi::c_uint,
        param2: ::core::ffi::c_uint,
        param3: ::core::ffi::c_uint,
    );
}

/* #define DSP_EXT_CMD_SET(x) (x->dsp_version > 0x012800)
 * Requires the external definition of struct pcxhr_mgr containing dsp_version.
 */

/*
 send the rmh
 */
unsafe extern "C" {
    pub fn pcxhr_send_msg(mgr: *mut pcxhr_mgr, rmh: *mut pcxhr_rmh) -> ::core::ffi::c_int;
}

/* values used for CMD_ACCESS_IO_WRITE and CMD_ACCESS_IO_READ */
pub const IO_NUM_REG_CONT: u32 = 0;
pub const IO_NUM_REG_GENCLK: u32 = 1;
pub const IO_NUM_REG_MUTE_OUT: u32 = 2;
pub const IO_NUM_SPEED_RATIO: u32 = 4;
pub const IO_NUM_REG_STATUS: u32 = 5;
pub const IO_NUM_REG_CUER: u32 = 10;
pub const IO_NUM_UER_CHIP_REG: u32 = 11;
pub const IO_NUM_REG_CONFIG_SRC: u32 = 12;
pub const IO_NUM_REG_OUT_ANA_LEVEL: u32 = 20;
pub const IO_NUM_REG_IN_ANA_LEVEL: u32 = 21;

pub const REG_CONT_VALSMPTE: u32 = 0x000800;
pub const REG_CONT_UNMUTE_INPUTS: u32 = 0x020000;

/* parameters used with register IO_NUM_REG_STATUS */
pub const REG_STATUS_OPTIONS: u32 = 0;
pub const REG_STATUS_AES_SYNC: u32 = 8;
pub const REG_STATUS_AES_1: u32 = 9;
pub const REG_STATUS_AES_2: u32 = 10;
pub const REG_STATUS_AES_3: u32 = 11;
pub const REG_STATUS_AES_4: u32 = 12;
pub const REG_STATUS_WORD_CLOCK: u32 = 13;
pub const REG_STATUS_INTER_SYNC: u32 = 14;
pub const REG_STATUS_CURRENT: u32 = 0x80;
/* results */
pub const REG_STATUS_OPT_NO_VIDEO_SIGNAL: u32 = 0x01;
pub const REG_STATUS_OPT_DAUGHTER_MASK: u32 = 0x1c;
pub const REG_STATUS_OPT_ANALOG_BOARD: u32 = 0x00;
pub const REG_STATUS_OPT_NO_DAUGHTER: u32 = 0x1c;
pub const REG_STATUS_OPT_COMPANION_MASK: u32 = 0xe0;
pub const REG_STATUS_OPT_NO_COMPANION: u32 = 0xe0;
pub const REG_STATUS_SYNC_32000: u32 = 0x00;
pub const REG_STATUS_SYNC_44100: u32 = 0x01;
pub const REG_STATUS_SYNC_48000: u32 = 0x02;
pub const REG_STATUS_SYNC_64000: u32 = 0x03;
pub const REG_STATUS_SYNC_88200: u32 = 0x04;
pub const REG_STATUS_SYNC_96000: u32 = 0x05;
pub const REG_STATUS_SYNC_128000: u32 = 0x06;
pub const REG_STATUS_SYNC_176400: u32 = 0x07;
pub const REG_STATUS_SYNC_192000: u32 = 0x08;

unsafe extern "C" {
    pub fn pcxhr_set_pipe_state(
        mgr: *mut pcxhr_mgr,
        playback_mask: ::core::ffi::c_int,
        capture_mask: ::core::ffi::c_int,
        start: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;

    pub fn pcxhr_write_io_num_reg_cont(
        mgr: *mut pcxhr_mgr,
        mask: ::core::ffi::c_uint,
        value: ::core::ffi::c_uint,
        changed: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

/* codec parameters */
pub const CS8416_RUN: u32 = 0x200401;
pub const CS8416_FORMAT_DETECT: u32 = 0x200b00;
pub const CS8416_CSB0: u32 = 0x201900;
pub const CS8416_CSB1: u32 = 0x201a00;
pub const CS8416_CSB2: u32 = 0x201b00;
pub const CS8416_CSB3: u32 = 0x201c00;
pub const CS8416_CSB4: u32 = 0x201d00;
pub const CS8416_VERSION: u32 = 0x207f00;

pub const CS8420_DATA_FLOW_CTL: u32 = 0x200301;
pub const CS8420_CLOCK_SRC_CTL: u32 = 0x200401;
pub const CS8420_RECEIVER_ERRORS: u32 = 0x201000;
pub const CS8420_SRC_RATIO: u32 = 0x201e00;
pub const CS8420_CSB0: u32 = 0x202000;
pub const CS8420_CSB1: u32 = 0x202100;
pub const CS8420_CSB2: u32 = 0x202200;
pub const CS8420_CSB3: u32 = 0x202300;
pub const CS8420_CSB4: u32 = 0x202400;
pub const CS8420_VERSION: u32 = 0x207f00;

pub const CS4271_MODE_CTL_1: u32 = 0x200101;
pub const CS4271_DAC_CTL: u32 = 0x200201;
pub const CS4271_VOLMIX: u32 = 0x200301;
pub const CS4271_VOLMUTE_LEFT: u32 = 0x200401;
pub const CS4271_VOLMUTE_RIGHT: u32 = 0x200501;
pub const CS4271_ADC_CTL: u32 = 0x200601;
pub const CS4271_MODE_CTL_2: u32 = 0x200701;

pub const CHIP_SIG_AND_MAP_SPI: u32 = 0xff7f00;

/* codec selection */
pub const CS4271_01_CS: u32 = 0x160018;
pub const CS4271_23_CS: u32 = 0x160019;
pub const CS4271_45_CS: u32 = 0x16001a;
pub const CS4271_67_CS: u32 = 0x16001b;
pub const CS4271_89_CS: u32 = 0x16001c;
pub const CS4271_AB_CS: u32 = 0x16001d;
pub const CS8420_01_CS: u32 = 0x080090;
pub const CS8420_23_CS: u32 = 0x080092;
pub const CS8420_45_CS: u32 = 0x080094;
pub const CS8420_67_CS: u32 = 0x080096;
pub const CS8416_01_CS: u32 = 0x080098;

/* interrupt handling */
unsafe extern "C" {
    pub fn pcxhr_interrupt(irq: ::core::ffi::c_int, dev_id: *mut ::core::ffi::c_void) -> irqreturn_t;
    pub fn pcxhr_threaded_irq(irq: ::core::ffi::c_int, dev_id: *mut ::core::ffi::c_void) -> irqreturn_t;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
