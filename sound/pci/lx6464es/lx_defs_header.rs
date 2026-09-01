/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * ALSA driver for the digigram lx6464es interface
 * adapted upstream headers
 *
 * Copyright (c) 2009 Tim Blechmann <tim@klingt.org>
 */

/* code adapted from ethersound.h */
pub const XES_FREQ_COUNT8_MASK: u32 = 0x00001FFF; /* compteur 25MHz entre 8 ech. */
pub const XES_FREQ_COUNT8_44_MIN: u32 = 0x00001288; /*
                                                     * 25M /
                                                     * [ 44k - ( 44.1k + 48k ) / 2 ]
                                                     * * 8
                                                     */
pub const XES_FREQ_COUNT8_44_MAX: u32 = 0x000010F0; /*
                                                     * 25M / [ ( 44.1k + 48k ) / 2 ]
                                                     * * 8
                                                     */
pub const XES_FREQ_COUNT8_48_MAX: u32 = 0x00000F08; /*
                                                     * 25M /
                                                     * [ 48k + ( 44.1k + 48k ) / 2 ]
                                                     * * 8
                                                     */

/* code adapted from LXES_registers.h */

pub const IOCR_OUTPUTS_OFFSET: u32 = 0; /* (rw) offset for the number of OUTs in the
                                         * ConfES register. */
pub const IOCR_INPUTS_OFFSET: u32 = 8; /* (rw) offset for the number of INs in the
                                        * ConfES register. */
pub const FREQ_RATIO_OFFSET: u32 = 19; /* (rw) offset for frequency ratio in the
                                        * ConfES register. */
pub const FREQ_RATIO_SINGLE_MODE: u32 = 0x01; /*
                                               * value for single mode frequency ratio:
                                               * sample rate = frequency rate.
                                               */

pub const CONFES_READ_PART_MASK: u32 = 0x00070000;
pub const CONFES_WRITE_PART_MASK: u32 = 0x00F80000;

/* code adapted from if_drv_mb.h */

pub const MASK_SYS_STATUS_ERROR: u32 = 1u32 << 31; /*
                                                    * events that lead to a PCI irq if
                                                    * not yet pending
                                                    */
pub const MASK_SYS_STATUS_URUN: u32 = 1u32 << 30;
pub const MASK_SYS_STATUS_ORUN: u32 = 1u32 << 29;
pub const MASK_SYS_STATUS_EOBO: u32 = 1u32 << 28;
pub const MASK_SYS_STATUS_EOBI: u32 = 1u32 << 27;
pub const MASK_SYS_STATUS_FREQ: u32 = 1u32 << 26;
pub const MASK_SYS_STATUS_ESA: u32 = 1u32 << 25; /*
                                                  * reserved, this is set by the
                                                  * XES
                                                  */
pub const MASK_SYS_STATUS_TIMER: u32 = 1u32 << 24;

pub const MASK_SYS_ASYNC_EVENTS: u32 = MASK_SYS_STATUS_ERROR
    | MASK_SYS_STATUS_URUN
    | MASK_SYS_STATUS_ORUN
    | MASK_SYS_STATUS_EOBO
    | MASK_SYS_STATUS_EOBI
    | MASK_SYS_STATUS_FREQ
    | MASK_SYS_STATUS_ESA;

pub const MASK_SYS_PCI_EVENTS: u32 = MASK_SYS_ASYNC_EVENTS | MASK_SYS_STATUS_TIMER;

pub const MASK_SYS_TIMER_COUNT: u32 = 0x0000FFFF;

pub const MASK_SYS_STATUS_EOT_PLX: u32 = 1u32 << 22; /*
                                                      * event that remains
                                                      * internal: reserved fo end
                                                      * of plx dma
                                                      */
pub const MASK_SYS_STATUS_XES: u32 = 1u32 << 21; /*
                                                  * event that remains
                                                  * internal: pending XES
                                                  * IRQ
                                                  */
pub const MASK_SYS_STATUS_CMD_DONE: u32 = 1u32 << 20; /*
                                                       * alternate command
                                                       * management: notify driver
                                                       * instead of polling
                                                       */

pub const MAX_STREAM_BUFFER: u32 = 5; /* max amount of stream buffers. */

pub const MICROBLAZE_IBL_MIN: u32 = 32;
pub const MICROBLAZE_IBL_DEFAULT: u32 = 128;
pub const MICROBLAZE_IBL_MAX: u32 = 512;
/* #define MASK_GRANULARITY		(2*MICROBLAZE_IBL_MAX-1) */

/* command opcodes, see reference for details */

/*
 the capture bit position in the object_id field in driver commands
 depends upon the number of managed channels. For now, 64 IN + 64 OUT are
 supported. HOwever, the communication protocol forsees 1024 channels, hence
 bit 10 indicates a capture (input) object).
*/
pub const ID_IS_CAPTURE: u32 = 1u32 << 10;
pub const ID_OFFSET: u32 = 13; /* object ID is at the 13th bit in the
                                * 1st command word.*/
pub const ID_CH_MASK: u32 = 0x3F;
pub const OPCODE_OFFSET: u32 = 24; /* offset of the command opcode in the first
                                    * command word.*/

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum cmd_mb_opcodes {
    CMD_00_INFO_DEBUG = 0x00,
    CMD_01_GET_SYS_CFG = 0x01,
    CMD_02_SET_GRANULARITY = 0x02,
    CMD_03_SET_TIMER_IRQ = 0x03,
    CMD_04_GET_EVENT = 0x04,
    CMD_05_GET_PIPES = 0x05,

    CMD_06_ALLOCATE_PIPE = 0x06,
    CMD_07_RELEASE_PIPE = 0x07,
    CMD_08_ASK_BUFFERS = 0x08,
    CMD_09_STOP_PIPE = 0x09,
    CMD_0A_GET_PIPE_SPL_COUNT = 0x0a,
    CMD_0B_TOGGLE_PIPE_STATE = 0x0b,

    CMD_0C_DEF_STREAM = 0x0c,
    CMD_0D_SET_MUTE = 0x0d,
    CMD_0E_GET_STREAM_SPL_COUNT = 0x0e,
    CMD_0F_UPDATE_BUFFER = 0x0f,
    CMD_10_GET_BUFFER = 0x10,
    CMD_11_CANCEL_BUFFER = 0x11,
    CMD_12_GET_PEAK = 0x12,
    CMD_13_SET_STREAM_STATE = 0x13,
    CMD_14_INVALID = 0x14,
}

/* pipe states */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum pipe_state_t {
    PSTATE_IDLE = 0, /* the pipe is not processed in the XES_IRQ
                      * (free or stopped, or paused). */
    PSTATE_RUN = 1, /* sustained play/record state. */
    PSTATE_PURGE = 2, /* the ES channels are now off, render pipes do
                       * not DMA, record pipe do a last DMA. */
    PSTATE_ACQUIRE = 3, /* the ES channels are now on, render pipes do
                         * not yet increase their sample count, record
                         * pipes do not DMA. */
    PSTATE_CLOSING = 4, /* the pipe is releasing, and may not yet
                         * receive an "alloc" command. */
}

/* stream states */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum stream_state_t {
    SSTATE_STOP = 0x00, /* setting to stop resets the stream spl
                         * count.*/
    SSTATE_RUN = 0x01 << 0, /* start DMA and spl count handling. */
    SSTATE_PAUSE = 0x01 << 1, /* pause DMA and spl count handling. */
}

/* buffer flags */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum buffer_flags {
    BF_VALID = 0x80, /* set if the buffer is valid, clear if free.*/
    BF_CURRENT = 0x40, /* set if this is the current buffer (there is
                        * always a current buffer).*/
    BF_NOTIFY_EOB = 0x20, /* set if this buffer must cause a PCI event
                           * when finished.*/
    BF_CIRCULAR = 0x10, /* set if buffer[1] must be copied to buffer[0]
                         * by the end of this buffer.*/
    BF_64BITS_ADR = 0x08, /* set if the hi part of the address is valid.*/
    BF_xx = 0x04, /* future extension.*/
    BF_EOB = 0x02, /* set if finished, but not yet free.*/
    BF_PAUSE = 0x01, /* pause stream at buffer end.*/
    BF_ZERO = 0x00, /* no flags (init).*/
}

/*
*	Stream Flags definitions
*/
#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum stream_flags {
    SF_ZERO = 0x00000000, /* no flags (stream invalid). */
    SF_VALID = 0x10000000, /* the stream has a valid DMA_conf
                            * info (setstreamformat). */
    SF_XRUN = 0x20000000, /* the stream is un x-run state. */
    SF_START = 0x40000000, /* the DMA is running.*/
    SF_ASIO = 0x80000000, /* ASIO.*/
}

pub const MASK_SPL_COUNT_HI: u32 = 0x00FFFFFF; /* 4 MSBits are status bits */
pub const PSTATE_OFFSET: u32 = 28; /* 4 MSBits are status bits */

pub const MASK_STREAM_HAS_MAPPING: u32 = 1u32 << 12;
pub const MASK_STREAM_IS_ASIO: u32 = 1u32 << 9;
pub const STREAM_FMT_OFFSET: u32 = 10; /* the stream fmt bits start at the 10th
                                        * bit in the command word. */

pub const STREAM_FMT_16b: u32 = 0x02;
pub const STREAM_FMT_intel: u32 = 0x01;

pub const FREQ_FIELD_OFFSET: u32 = 15; /* offset of the freq field in the response
                                        * word */

pub const BUFF_FLAGS_OFFSET: u32 = 24; /*  offset of the buffer flags in the
                                        *  response word. */
pub const MASK_DATA_SIZE: u32 = 0x00FFFFFF; /* this must match the field size of
                                             * datasize in the buffer_t structure. */

pub const MASK_BUFFER_ID: u32 = 0xFF; /* the cancel command awaits a buffer ID,
                                       * may be 0xFF for "current". */

/* code adapted from PcxErr_e.h */

/* Bits masks */

pub const ERROR_MASK: u32 = 0x8000;

pub const SOURCE_MASK: u32 = 0x7800;

pub const E_SOURCE_BOARD: u32 = 0x4000; /* 8 >> 1 */
pub const E_SOURCE_DRV: u32 = 0x2000; /* 4 >> 1 */
pub const E_SOURCE_API: u32 = 0x1000; /* 2 >> 1 */
/* Error tools */
pub const E_SOURCE_TOOLS: u32 = 0x0800; /* 1 >> 1 */
/* Error pcxaudio */
pub const E_SOURCE_AUDIO: u32 = 0x1800; /* 3 >> 1 */
/* Error virtual pcx */
pub const E_SOURCE_VPCX: u32 = 0x2800; /* 5 >> 1 */
/* Error dispatcher */
pub const E_SOURCE_DISPATCHER: u32 = 0x3000; /* 6 >> 1 */
/* Error from CobraNet firmware */
pub const E_SOURCE_COBRANET: u32 = 0x3800; /* 7 >> 1 */

pub const E_SOURCE_USER: u32 = 0x7800;

pub const CLASS_MASK: u32 = 0x0700;

pub const CODE_MASK: u32 = 0x00FF;

/* Bits values */

/* Values for the error/warning bit */
pub const ERROR_VALUE: u32 = 0x8000;
pub const WARNING_VALUE: u32 = 0x0000;

/* Class values */
pub const E_CLASS_GENERAL: u32 = 0x0000;
pub const E_CLASS_INVALID_CMD: u32 = 0x0100;
pub const E_CLASS_INVALID_STD_OBJECT: u32 = 0x0200;
pub const E_CLASS_RSRC_IMPOSSIBLE: u32 = 0x0300;
pub const E_CLASS_WRONG_CONTEXT: u32 = 0x0400;
pub const E_CLASS_BAD_SPECIFIC_PARAMETER: u32 = 0x0500;
pub const E_CLASS_REAL_TIME_ERROR: u32 = 0x0600;
pub const E_CLASS_DIRECTSHOW: u32 = 0x0700;
pub const E_CLASS_FREE: u32 = 0x0700;

/* Complete DRV error code for the general class */
pub const ED_GN: u32 = ERROR_VALUE | E_SOURCE_DRV | E_CLASS_GENERAL;
pub const ED_CONCURRENCY: u32 = ED_GN | 0x01;
pub const ED_DSP_CRASHED: u32 = ED_GN | 0x02;
pub const ED_UNKNOWN_BOARD: u32 = ED_GN | 0x03;
pub const ED_NOT_INSTALLED: u32 = ED_GN | 0x04;
pub const ED_CANNOT_OPEN_SVC_MANAGER: u32 = ED_GN | 0x05;
pub const ED_CANNOT_READ_REGISTRY: u32 = ED_GN | 0x06;
pub const ED_DSP_VERSION_MISMATCH: u32 = ED_GN | 0x07;
pub const ED_UNAVAILABLE_FEATURE: u32 = ED_GN | 0x08;
pub const ED_CANCELLED: u32 = ED_GN | 0x09;
pub const ED_NO_RESPONSE_AT_IRQA: u32 = ED_GN | 0x10;
pub const ED_INVALID_ADDRESS: u32 = ED_GN | 0x11;
pub const ED_DSP_CORRUPTED: u32 = ED_GN | 0x12;
pub const ED_PENDING_OPERATION: u32 = ED_GN | 0x13;
pub const ED_NET_ALLOCATE_MEMORY_IMPOSSIBLE: u32 = ED_GN | 0x14;
pub const ED_NET_REGISTER_ERROR: u32 = ED_GN | 0x15;
pub const ED_NET_THREAD_ERROR: u32 = ED_GN | 0x16;
pub const ED_NET_OPEN_ERROR: u32 = ED_GN | 0x17;
pub const ED_NET_CLOSE_ERROR: u32 = ED_GN | 0x18;
pub const ED_NET_NO_MORE_PACKET: u32 = ED_GN | 0x19;
pub const ED_NET_NO_MORE_BUFFER: u32 = ED_GN | 0x1A;
pub const ED_NET_SEND_ERROR: u32 = ED_GN | 0x1B;
pub const ED_NET_RECEIVE_ERROR: u32 = ED_GN | 0x1C;
pub const ED_NET_WRONG_MSG_SIZE: u32 = ED_GN | 0x1D;
pub const ED_NET_WAIT_ERROR: u32 = ED_GN | 0x1E;
pub const ED_NET_EEPROM_ERROR: u32 = ED_GN | 0x1F;
pub const ED_INVALID_RS232_COM_NUMBER: u32 = ED_GN | 0x20;
pub const ED_INVALID_RS232_INIT: u32 = ED_GN | 0x21;
pub const ED_FILE_ERROR: u32 = ED_GN | 0x22;
pub const ED_INVALID_GPIO_CMD: u32 = ED_GN | 0x23;
pub const ED_RS232_ALREADY_OPENED: u32 = ED_GN | 0x24;
pub const ED_RS232_NOT_OPENED: u32 = ED_GN | 0x25;
pub const ED_GPIO_ALREADY_OPENED: u32 = ED_GN | 0x26;
pub const ED_GPIO_NOT_OPENED: u32 = ED_GN | 0x27;
pub const ED_REGISTRY_ERROR: u32 = ED_GN | 0x28; /* <- NCX */
pub const ED_INVALID_SERVICE: u32 = ED_GN | 0x29; /* <- NCX */

pub const ED_READ_FILE_ALREADY_OPENED: u32 = ED_GN | 0x2a; /*
                                                           * <- Decalage
                                                           * pour RCX
                                                           * (old 0x28)
                                                           */
pub const ED_READ_FILE_INVALID_COMMAND: u32 = ED_GN | 0x2b; /* ~ */
pub const ED_READ_FILE_INVALID_PARAMETER: u32 = ED_GN | 0x2c; /* ~ */
pub const ED_READ_FILE_ALREADY_CLOSED: u32 = ED_GN | 0x2d; /* ~ */
pub const ED_READ_FILE_NO_INFORMATION: u32 = ED_GN | 0x2e; /* ~ */
pub const ED_READ_FILE_INVALID_HANDLE: u32 = ED_GN | 0x2f; /* ~ */
pub const ED_READ_FILE_END_OF_FILE: u32 = ED_GN | 0x30; /* ~ */
pub const ED_READ_FILE_ERROR: u32 = ED_GN | 0x31; /* ~ */

pub const ED_DSP_CRASHED_EXC_DSPSTACK_OVERFLOW: u32 = ED_GN | 0x32; /*
                                                                     * <- Decalage pour
                                                                     * PCX (old 0x14)
                                                                     */
pub const ED_DSP_CRASHED_EXC_SYSSTACK_OVERFLOW: u32 = ED_GN | 0x33; /* ~ */
pub const ED_DSP_CRASHED_EXC_ILLEGAL: u32 = ED_GN | 0x34; /* ~ */
pub const ED_DSP_CRASHED_EXC_TIMER_REENTRY: u32 = ED_GN | 0x35; /* ~ */
pub const ED_DSP_CRASHED_EXC_FATAL_ERROR: u32 = ED_GN | 0x36; /* ~ */

pub const ED_FLASH_PCCARD_NOT_PRESENT: u32 = ED_GN | 0x37;

pub const ED_NO_CURRENT_CLOCK: u32 = ED_GN | 0x38;

/* Complete DRV error code for real time class */
pub const ED_RT: u32 = ERROR_VALUE | E_SOURCE_DRV | E_CLASS_REAL_TIME_ERROR;
pub const ED_DSP_TIMED_OUT: u32 = ED_RT | 0x01;
pub const ED_DSP_CHK_TIMED_OUT: u32 = ED_RT | 0x02;
pub const ED_STREAM_OVERRUN: u32 = ED_RT | 0x03;
pub const ED_DSP_BUSY: u32 = ED_RT | 0x04;
pub const ED_DSP_SEMAPHORE_TIME_OUT: u32 = ED_RT | 0x05;
pub const ED_BOARD_TIME_OUT: u32 = ED_RT | 0x06;
pub const ED_XILINX_ERROR: u32 = ED_RT | 0x07;
pub const ED_COBRANET_ITF_NOT_RESPONDING: u32 = ED_RT | 0x08;

/* Complete BOARD error code for the invaid standard object class */
pub const EB_ISO: u32 = ERROR_VALUE | E_SOURCE_BOARD | E_CLASS_INVALID_STD_OBJECT;
pub const EB_INVALID_EFFECT: u32 = EB_ISO | 0x00;
pub const EB_INVALID_PIPE: u32 = EB_ISO | 0x40;
pub const EB_INVALID_STREAM: u32 = EB_ISO | 0x80;
pub const EB_INVALID_AUDIO: u32 = EB_ISO | 0xC0;

/* Complete BOARD error code for impossible resource allocation class */
pub const EB_RI: u32 = ERROR_VALUE | E_SOURCE_BOARD | E_CLASS_RSRC_IMPOSSIBLE;
pub const EB_ALLOCATE_ALL_STREAM_TRANSFERT_BUFFERS_IMPOSSIBLE: u32 = EB_RI | 0x01;
pub const EB_ALLOCATE_PIPE_SAMPLE_BUFFER_IMPOSSIBLE: u32 = EB_RI | 0x02;

pub const EB_ALLOCATE_MEM_STREAM_IMPOSSIBLE: u32 =
    EB_ALLOCATE_ALL_STREAM_TRANSFERT_BUFFERS_IMPOSSIBLE;
pub const EB_ALLOCATE_MEM_PIPE_IMPOSSIBLE: u32 = EB_ALLOCATE_PIPE_SAMPLE_BUFFER_IMPOSSIBLE;

pub const EB_ALLOCATE_DIFFERED_CMD_IMPOSSIBLE: u32 = EB_RI | 0x03;
pub const EB_TOO_MANY_DIFFERED_CMD: u32 = EB_RI | 0x04;
pub const EB_RBUFFERS_TABLE_OVERFLOW: u32 = EB_RI | 0x05;
pub const EB_ALLOCATE_EFFECTS_IMPOSSIBLE: u32 = EB_RI | 0x08;
pub const EB_ALLOCATE_EFFECT_POS_IMPOSSIBLE: u32 = EB_RI | 0x09;
pub const EB_RBUFFER_NOT_AVAILABLE: u32 = EB_RI | 0x0A;
pub const EB_ALLOCATE_CONTEXT_LIII_IMPOSSIBLE: u32 = EB_RI | 0x0B;
pub const EB_STATUS_DIALOG_IMPOSSIBLE: u32 = EB_RI | 0x1D;
pub const EB_CONTROL_CMD_IMPOSSIBLE: u32 = EB_RI | 0x1E;
pub const EB_STATUS_SEND_IMPOSSIBLE: u32 = EB_RI | 0x1F;
pub const EB_ALLOCATE_PIPE_IMPOSSIBLE: u32 = EB_RI | 0x40;
pub const EB_ALLOCATE_STREAM_IMPOSSIBLE: u32 = EB_RI | 0x80;
pub const EB_ALLOCATE_AUDIO_IMPOSSIBLE: u32 = EB_RI | 0xC0;

/* Complete BOARD error code for wrong call context class */
pub const EB_WCC: u32 = ERROR_VALUE | E_SOURCE_BOARD | E_CLASS_WRONG_CONTEXT;
pub const EB_CMD_REFUSED: u32 = EB_WCC | 0x00;
pub const EB_START_STREAM_REFUSED: u32 = EB_WCC | 0xFC;
pub const EB_SPC_REFUSED: u32 = EB_WCC | 0xFD;
pub const EB_CSN_REFUSED: u32 = EB_WCC | 0xFE;
pub const EB_CSE_REFUSED: u32 = EB_WCC | 0xFF;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
