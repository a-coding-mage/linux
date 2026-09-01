// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  The driver for the Cirrus Logic's Sound Fusion CS46XX based soundcards
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 */

/*
 * 2002-07 Benny Sjostrand benny@hostmobility.com
 */

// C source was guarded by CONFIG_SND_CS46XX_NEW_DSP and __DSP_SPOS_H__.

pub const DSP_MAX_SYMBOLS: u32 = 1024;
pub const DSP_MAX_MODULES: u32 = 64;

pub const DSP_CODE_BYTE_SIZE: u32 = 0x00007000;
pub const DSP_PARAMETER_BYTE_SIZE: u32 = 0x00003000;
pub const DSP_SAMPLE_BYTE_SIZE: u32 = 0x00003800;
pub const DSP_PARAMETER_BYTE_OFFSET: u32 = 0x00000000;
pub const DSP_SAMPLE_BYTE_OFFSET: u32 = 0x00010000;
pub const DSP_CODE_BYTE_OFFSET: u32 = 0x00020000;

pub const WIDE_INSTR_MASK: u32 = 0x0040;
pub const WIDE_LADD_INSTR_MASK: u32 = 0x0380;

/* this instruction types
   needs to be reallocated when load
   code into DSP */
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum wide_opcode {
    WIDE_FOR_BEGIN_LOOP = 0x20,
    WIDE_FOR_BEGIN_LOOP2 = 0x21,

    WIDE_COND_GOTO_ADDR = 0x30,
    WIDE_COND_GOTO_CALL = 0x31,

    WIDE_TBEQ_COND_GOTO_ADDR = 0x70,
    WIDE_TBEQ_COND_CALL_ADDR = 0x71,
    WIDE_TBEQ_NCOND_GOTO_ADDR = 0x72,
    WIDE_TBEQ_NCOND_CALL_ADDR = 0x73,
    WIDE_TBEQ_COND_GOTO1_ADDR = 0x74,
    WIDE_TBEQ_COND_CALL1_ADDR = 0x75,
    WIDE_TBEQ_NCOND_GOTOI_ADDR = 0x76,
    WIDE_TBEQ_NCOND_CALL1_ADDR = 0x77,
}

/* SAMPLE segment */
pub const VARI_DECIMATE_BUF1: u32 = 0x0000;
pub const WRITE_BACK_BUF1: u32 = 0x0400;
pub const CODEC_INPUT_BUF1: u32 = 0x0500;
pub const PCM_READER_BUF1: u32 = 0x0600;
pub const SRC_DELAY_BUF1: u32 = 0x0680;
pub const VARI_DECIMATE_BUF0: u32 = 0x0780;
pub const SRC_OUTPUT_BUF1: u32 = 0x07A0;
pub const ASYNC_IP_OUTPUT_BUFFER1: u32 = 0x0A00;
pub const OUTPUT_SNOOP_BUFFER: u32 = 0x0B00;
pub const SPDIFI_IP_OUTPUT_BUFFER1: u32 = 0x0E00;
pub const SPDIFO_IP_OUTPUT_BUFFER1: u32 = 0x1000;
pub const MIX_SAMPLE_BUF1: u32 = 0x1400;
pub const MIX_SAMPLE_BUF2: u32 = 0x2E80;
pub const MIX_SAMPLE_BUF3: u32 = 0x2F00;
pub const MIX_SAMPLE_BUF4: u32 = 0x2F80;
pub const MIX_SAMPLE_BUF5: u32 = 0x3000;

/* Task stack address */
pub const HFG_STACK: u32 = 0x066A;
pub const FG_STACK: u32 = 0x066E;
pub const BG_STACK: u32 = 0x068E;

/* SCB's addresses */
pub const SPOSCB_ADDR: u32 = 0x070;
pub const BG_TREE_SCB_ADDR: u32 = 0x635;
pub const NULL_SCB_ADDR: u32 = 0x000;
pub const TIMINGMASTER_SCB_ADDR: u32 = 0x010;
pub const CODECOUT_SCB_ADDR: u32 = 0x020;
pub const PCMREADER_SCB_ADDR: u32 = 0x030;
pub const WRITEBACK_SCB_ADDR: u32 = 0x040;
pub const CODECIN_SCB_ADDR: u32 = 0x080;
pub const MASTERMIX_SCB_ADDR: u32 = 0x090;
pub const SRCTASK_SCB_ADDR: u32 = 0x0A0;
pub const VARIDECIMATE_SCB_ADDR: u32 = 0x0B0;
pub const PCMSERIALIN_SCB_ADDR: u32 = 0x0C0;
pub const FG_TASK_HEADER_ADDR: u32 = 0x600;
pub const ASYNCTX_SCB_ADDR: u32 = 0x0E0;
pub const ASYNCRX_SCB_ADDR: u32 = 0x0F0;
pub const SRCTASKII_SCB_ADDR: u32 = 0x100;
pub const OUTPUTSNOOP_SCB_ADDR: u32 = 0x110;
pub const PCMSERIALINII_SCB_ADDR: u32 = 0x120;
pub const SPIOWRITE_SCB_ADDR: u32 = 0x130;
pub const REAR_CODECOUT_SCB_ADDR: u32 = 0x140;
pub const OUTPUTSNOOPII_SCB_ADDR: u32 = 0x150;
pub const PCMSERIALIN_PCM_SCB_ADDR: u32 = 0x160;
pub const RECORD_MIXER_SCB_ADDR: u32 = 0x170;
pub const REAR_MIXER_SCB_ADDR: u32 = 0x180;
pub const CLFE_MIXER_SCB_ADDR: u32 = 0x190;
pub const CLFE_CODEC_SCB_ADDR: u32 = 0x1A0;

/* hyperforground SCB's*/
pub const HFG_TREE_SCB: u32 = 0xBA0;
pub const SPDIFI_SCB_INST: u32 = 0xBB0;
pub const SPDIFO_SCB_INST: u32 = 0xBC0;
pub const WRITE_BACK_SPB: u32 = 0x0D0;

/* offsets */
pub const AsyncCIOFIFOPointer: u32 = 0xd;
pub const SPDIFOFIFOPointer: u32 = 0xd;
pub const SPDIFIFIFOPointer: u32 = 0xd;
pub const TCBData: u32 = 0xb;
pub const HFGFlags: u32 = 0xa;
pub const TCBContextBlk: u32 = 0x10;
pub const AFGTxAccumPhi: u32 = 0x4;
pub const SCBsubListPtr: u32 = 0x9;
pub const SCBfuncEntryPtr: u32 = 0xA;
pub const SRCCorPerGof: u32 = 0x2;
pub const SRCPhiIncr6Int26Frac: u32 = 0xd;
pub const SCBVolumeCtrl: u32 = 0xe;

/* conf */
pub const UseASER1Input: u32 = 1;

/*
 * The following defines are for the flags in the rsConfig01/23 registers of
 * the SP.
 */

pub const RSCONFIG_MODULO_SIZE_MASK: u32 = 0x0000000F;
pub const RSCONFIG_MODULO_16: u32 = 0x00000001;
pub const RSCONFIG_MODULO_32: u32 = 0x00000002;
pub const RSCONFIG_MODULO_64: u32 = 0x00000003;
pub const RSCONFIG_MODULO_128: u32 = 0x00000004;
pub const RSCONFIG_MODULO_256: u32 = 0x00000005;
pub const RSCONFIG_MODULO_512: u32 = 0x00000006;
pub const RSCONFIG_MODULO_1024: u32 = 0x00000007;
pub const RSCONFIG_MODULO_4: u32 = 0x00000008;
pub const RSCONFIG_MODULO_8: u32 = 0x00000009;
pub const RSCONFIG_SAMPLE_SIZE_MASK: u32 = 0x000000C0;
pub const RSCONFIG_SAMPLE_8MONO: u32 = 0x00000000;
pub const RSCONFIG_SAMPLE_8STEREO: u32 = 0x00000040;
pub const RSCONFIG_SAMPLE_16MONO: u32 = 0x00000080;
pub const RSCONFIG_SAMPLE_16STEREO: u32 = 0x000000C0;
pub const RSCONFIG_UNDERRUN_ZERO: u32 = 0x00004000;
pub const RSCONFIG_DMA_TO_HOST: u32 = 0x00008000;
pub const RSCONFIG_STREAM_NUM_MASK: u32 = 0x00FF0000;
pub const RSCONFIG_MAX_DMA_SIZE_MASK: u32 = 0x1F000000;
pub const RSCONFIG_DMA_ENABLE: u32 = 0x20000000;
pub const RSCONFIG_PRIORITY_MASK: u32 = 0xC0000000;
pub const RSCONFIG_PRIORITY_HIGH: u32 = 0x00000000;
pub const RSCONFIG_PRIORITY_MEDIUM_HIGH: u32 = 0x40000000;
pub const RSCONFIG_PRIORITY_MEDIUM_LOW: u32 = 0x80000000;
pub const RSCONFIG_PRIORITY_LOW: u32 = 0xC0000000;
pub const RSCONFIG_STREAM_NUM_SHIFT: u32 = 16;
pub const RSCONFIG_MAX_DMA_SIZE_SHIFT: u32 = 24;

/* SP constants */
pub const FG_INTERVAL_TIMER_PERIOD: u32 = 0x0051;
pub const BG_INTERVAL_TIMER_PERIOD: u32 = 0x0100;

/* Only SP accessible registers */
pub const SP_ASER_COUNTDOWN: u32 = 0x8040;
pub const SP_SPDOUT_FIFO: u32 = 0x0108;
pub const SP_SPDIN_MI_FIFO: u32 = 0x01E0;
pub const SP_SPDIN_D_FIFO: u32 = 0x01F0;
pub const SP_SPDIN_STATUS: u32 = 0x8048;
pub const SP_SPDIN_CONTROL: u32 = 0x8049;
pub const SP_SPDIN_FIFOPTR: u32 = 0x804A;
pub const SP_SPDOUT_STATUS: u32 = 0x804C;
pub const SP_SPDOUT_CONTROL: u32 = 0x804D;
pub const SP_SPDOUT_CSUV: u32 = 0x808E;

// External dependency declarations supplied by other translated files.
#[repr(C)]
pub struct snd_cs46xx {
    _private: [u8; 0],
}

// TODO: The complete C layout of struct dsp_scb_descriptor is an external
// dependency. This header's inline functions require these fields.
#[repr(C)]
pub struct dsp_scb_descriptor {
    pub address: u32,
    pub sub_list_ptr: *mut dsp_scb_descriptor,
    pub next_scb_ptr: *mut dsp_scb_descriptor,
    pub updated: i32,
    pub volume_set: i32,
    pub volume: [u16; 2],
}

extern "C" {
    pub fn snd_cs46xx_poke(chip: *mut snd_cs46xx, reg: u32, val: u32);
}

#[inline]
pub fn _wrap_all_bits(val: u8) -> u8 {
    let wrapped: u8;

    /* wrap all 8 bits */
    wrapped = (((val & 0x1) as u8) << 7)
        | (((val & 0x2) as u8) << 5)
        | (((val & 0x4) as u8) << 3)
        | (((val & 0x8) as u8) << 1)
        | (((val & 0x10) as u8) >> 1)
        | (((val & 0x20) as u8) >> 3)
        | (((val & 0x40) as u8) >> 5)
        | (((val & 0x80) as u8) >> 7);

    wrapped
}

#[inline]
pub unsafe fn cs46xx_dsp_spos_update_scb(
    chip: *mut snd_cs46xx,
    scb: *mut dsp_scb_descriptor,
) {
    /* update nextSCB and subListPtr in SCB */
    snd_cs46xx_poke(
        chip,
        ((*scb).address + SCBsubListPtr) << 2,
        ((*(*scb).sub_list_ptr).address << 0x10) | (*(*scb).next_scb_ptr).address,
    );
    (*scb).updated = 1;
}

#[inline]
pub unsafe fn cs46xx_dsp_scb_set_volume(
    chip: *mut snd_cs46xx,
    scb: *mut dsp_scb_descriptor,
    left: u16,
    right: u16,
) {
    let val: u32 = ((0xffffu32 - left as u32) << 16) | (0xffffu32 - right as u32);

    snd_cs46xx_poke(chip, ((*scb).address + SCBVolumeCtrl) << 2, val);
    snd_cs46xx_poke(chip, ((*scb).address + SCBVolumeCtrl + 1) << 2, val);
    (*scb).volume_set = 1;
    (*scb).volume[0] = left;
    (*scb).volume[1] = right;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
