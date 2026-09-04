// SPDX-License-Identifier: GPL-2.0-only
/*
 * Driver for AMD7930 sound chips found on Sparcs.
 * Copyright (C) 2002, 2008 David S. Miller <davem@davemloft.net>
 *
 * Based entirely upon drivers/sbus/audio/amd7930.c which is:
 * Copyright (C) 1996,1997 Thomas K. Dyas (tdyas@eden.rutgers.edu)
 *
 * --- Notes from Thomas's original driver ---
 * This is the lowlevel driver for the AMD7930 audio chip found on all
 * sun4c machines and some sun4m machines.
 *
 * The amd7930 is actually an ISDN chip which has a very simple
 * integrated audio encoder/decoder. When Sun decided on what chip to
 * use for audio, they had the brilliant idea of using the amd7930 and
 * only connecting the audio encoder/decoder pins.
 *
 * Thanks to the AMD engineer who was able to get us the AMD79C30
 * databook which has all the programming information and gain tables.
 *
 * Advanced Micro Devices' Am79C30A is an ISDN/audio chip used in the
 * SparcStation 1+.  The chip provides microphone and speaker interfaces
 * which provide mono-channel audio at 8K samples per second via either
 * 8-bit A-law or 8-bit mu-law encoding.  Also, the chip features an
 * ISDN BRI Line Interface Unit (LIU), I.430 S/T physical interface,
 * which performs basic D channel LAPD processing and provides raw
 * B channel data.  The digital audio channel, the two ISDN B channels,
 * and two 64 Kbps channels to the microprocessor are all interconnected
 * via a multiplexer.
 * --- End of notes from Thoamas's original driver ---
 */

// Linux kernel headers that would be included:
// #include <linux/module.h>
// #include <linux/kernel.h>
// #include <linux/slab.h>
// #include <linux/init.h>
// #include <linux/interrupt.h>
// #include <linux/moduleparam.h>
// #include <linux/of.h>
// #include <linux/platform_device.h>
// #include <linux/io.h>
// #include <linux/string.h>
// #include <sound/core.h>
// #include <sound/pcm.h>
// #include <sound/info.h>
// #include <sound/control.h>
// #include <sound/initval.h>
// #include <asm/irq.h>

use core::ptr;

// Module parameters (would be exposed via kernel module parameter system)
// static int index[SNDRV_CARDS] = SNDRV_DEFAULT_IDX;
// static char *id[SNDRV_CARDS] = SNDRV_DEFAULT_STR;
// static bool enable[SNDRV_CARDS] = SNDRV_DEFAULT_ENABLE_PNP;
// MODULE_AUTHOR("Thomas K. Dyas and David S. Miller");
// MODULE_DESCRIPTION("Sun AMD7930");
// MODULE_LICENSE("GPL");

// Device register layout.

// Register interface presented to the CPU by the amd7930.
const AMD7930_CR: usize = 0x00;     // Command Register (W)
const AMD7930_IR: usize = 0x00;     // Interrupt Register (R) - same as CR
const AMD7930_DR: usize = 0x01;     // Data Register (R/W)
const AMD7930_DSR1: usize = 0x02;   // D-channel Status Register 1 (R)
const AMD7930_DER: usize = 0x03;    // D-channel Error Register (R)
const AMD7930_DCTB: usize = 0x04;   // D-channel Transmit Buffer (W)
const AMD7930_DCRB: usize = 0x04;   // D-channel Receive Buffer (R) - same as DCTB
const AMD7930_BBTB: usize = 0x05;   // Bb-channel Transmit Buffer (W)
const AMD7930_BBRB: usize = 0x05;   // Bb-channel Receive Buffer (R) - same as BBTB
const AMD7930_BCTB: usize = 0x06;   // Bc-channel Transmit Buffer (W)
const AMD7930_BCRB: usize = 0x06;   // Bc-channel Receive Buffer (R) - same as BCTB
const AMD7930_DSR2: usize = 0x07;   // D-channel Status Register 2 (R)

// Indirect registers in the Main Audio Processor.
#[repr(C)]
struct Amd7930Map {
    x: [u16; 8],
    r: [u16; 8],
    gx: u16,
    gr: u16,
    ger: u16,
    stgr: u16,
    ftgr: u16,
    atgr: u16,
    mmr1: u8,
    mmr2: u8,
}

// After an amd7930 interrupt, reading the Interrupt Register (ir)
// clears the interrupt and returns a bitmask indicating which
// interrupt source(s) require service.

const AMR_IR_DTTHRSH: u8 = 0x01;    // D-channel xmit threshold
const AMR_IR_DRTHRSH: u8 = 0x02;    // D-channel recv threshold
const AMR_IR_DSRI: u8 = 0x04;       // D-channel packet status
const AMR_IR_DERI: u8 = 0x08;       // D-channel error
const AMR_IR_BBUF: u8 = 0x10;       // B-channel data xfer
const AMR_IR_LSRI: u8 = 0x20;       // LIU status
const AMR_IR_DSR2I: u8 = 0x40;      // D-channel buffer status
const AMR_IR_MLTFRMI: u8 = 0x80;    // multiframe or PP

// Initialization
const AMR_INIT: u8 = 0x21;
const AM_INIT_ACTIVE: u8 = 0x01;
const AM_INIT_DATAONLY: u8 = 0x02;
const AM_INIT_POWERDOWN: u8 = 0x03;
const AM_INIT_DISABLE_INTS: u8 = 0x04;
const AMR_INIT2: u8 = 0x20;
const AM_INIT2_ENABLE_POWERDOWN: u8 = 0x20;
const AM_INIT2_ENABLE_MULTIFRAME: u8 = 0x10;

// Line Interface Unit
const AMR_LIU_LSR: u8 = 0xA1;
const AM_LIU_LSR_STATE: u8 = 0x07;
const AM_LIU_LSR_F3: u8 = 0x08;
const AM_LIU_LSR_F7: u8 = 0x10;
const AM_LIU_LSR_F8: u8 = 0x20;
const AM_LIU_LSR_HSW: u8 = 0x40;
const AM_LIU_LSR_HSW_CHG: u8 = 0x80;
const AMR_LIU_LPR: u8 = 0xA2;
const AMR_LIU_LMR1: u8 = 0xA3;
const AM_LIU_LMR1_B1_ENABL: u8 = 0x01;
const AM_LIU_LMR1_B2_ENABL: u8 = 0x02;
const AM_LIU_LMR1_F_DISABL: u8 = 0x04;
const AM_LIU_LMR1_FA_DISABL: u8 = 0x08;
const AM_LIU_LMR1_REQ_ACTIV: u8 = 0x10;
const AM_LIU_LMR1_F8_F3: u8 = 0x20;
const AM_LIU_LMR1_LIU_ENABL: u8 = 0x40;
const AMR_LIU_LMR2: u8 = 0xA4;
const AM_LIU_LMR2_DECHO: u8 = 0x01;
const AM_LIU_LMR2_DLOOP: u8 = 0x02;
const AM_LIU_LMR2_DBACKOFF: u8 = 0x04;
const AM_LIU_LMR2_EN_F3_INT: u8 = 0x08;
const AM_LIU_LMR2_EN_F8_INT: u8 = 0x10;
const AM_LIU_LMR2_EN_HSW_INT: u8 = 0x20;
const AM_LIU_LMR2_EN_F7_INT: u8 = 0x40;
const AMR_LIU_2_4: u8 = 0xA5;
const AMR_LIU_MF: u8 = 0xA6;
const AMR_LIU_MFSB: u8 = 0xA7;
const AMR_LIU_MFQB: u8 = 0xA8;

// Multiplexor
const AMR_MUX_MCR1: u8 = 0x41;
const AMR_MUX_MCR2: u8 = 0x42;
const AMR_MUX_MCR3: u8 = 0x43;
const AM_MUX_CHANNEL_B1: u8 = 0x01;
const AM_MUX_CHANNEL_B2: u8 = 0x02;
const AM_MUX_CHANNEL_BA: u8 = 0x03;
const AM_MUX_CHANNEL_BB: u8 = 0x04;
const AM_MUX_CHANNEL_BC: u8 = 0x05;
const AM_MUX_CHANNEL_BD: u8 = 0x06;
const AM_MUX_CHANNEL_BE: u8 = 0x07;
const AM_MUX_CHANNEL_BF: u8 = 0x08;
const AMR_MUX_MCR4: u8 = 0x44;
const AM_MUX_MCR4_ENABLE_INTS: u8 = 0x08;
const AM_MUX_MCR4_REVERSE_BB: u8 = 0x10;
const AM_MUX_MCR4_REVERSE_BC: u8 = 0x20;
const AMR_MUX_1_4: u8 = 0x45;

// Main Audio Processor
const AMR_MAP_X: u8 = 0x61;
const AMR_MAP_R: u8 = 0x62;
const AMR_MAP_GX: u8 = 0x63;
const AMR_MAP_GR: u8 = 0x64;
const AMR_MAP_GER: u8 = 0x65;
const AMR_MAP_STGR: u8 = 0x66;
const AMR_MAP_FTGR_1_2: u8 = 0x67;
const AMR_MAP_ATGR_1_2: u8 = 0x68;
const AMR_MAP_MMR1: u8 = 0x69;
const AM_MAP_MMR1_ALAW: u8 = 0x01;
const AM_MAP_MMR1_GX: u8 = 0x02;
const AM_MAP_MMR1_GR: u8 = 0x04;
const AM_MAP_MMR1_GER: u8 = 0x08;
const AM_MAP_MMR1_X: u8 = 0x10;
const AM_MAP_MMR1_R: u8 = 0x20;
const AM_MAP_MMR1_STG: u8 = 0x40;
const AM_MAP_MMR1_LOOPBACK: u8 = 0x80;
const AMR_MAP_MMR2: u8 = 0x6A;
const AM_MAP_MMR2_AINB: u8 = 0x01;
const AM_MAP_MMR2_LS: u8 = 0x02;
const AM_MAP_MMR2_ENABLE_DTMF: u8 = 0x04;
const AM_MAP_MMR2_ENABLE_TONEGEN: u8 = 0x08;
const AM_MAP_MMR2_ENABLE_TONERING: u8 = 0x10;
const AM_MAP_MMR2_DISABLE_HIGHPASS: u8 = 0x20;
const AM_MAP_MMR2_DISABLE_AUTOZERO: u8 = 0x40;
const AMR_MAP_1_10: u8 = 0x6B;
const AMR_MAP_MMR3: u8 = 0x6C;
const AMR_MAP_STRA: u8 = 0x6D;
const AMR_MAP_STRF: u8 = 0x6E;
const AMR_MAP_PEAKX: u8 = 0x70;
const AMR_MAP_PEAKR: u8 = 0x71;
const AMR_MAP_15_16: u8 = 0x72;

// Data Link Controller
const AMR_DLC_FRAR_1_2_3: u8 = 0x81;
const AMR_DLC_SRAR_1_2_3: u8 = 0x82;
const AMR_DLC_TAR: u8 = 0x83;
const AMR_DLC_DRLR: u8 = 0x84;
const AMR_DLC_DTCR: u8 = 0x85;
const AMR_DLC_DMR1: u8 = 0x86;
const AMR_DLC_DMR1_DTTHRSH_INT: u8 = 0x01;
const AMR_DLC_DMR1_DRTHRSH_INT: u8 = 0x02;
const AMR_DLC_DMR1_TAR_ENABL: u8 = 0x04;
const AMR_DLC_DMR1_EORP_INT: u8 = 0x08;
const AMR_DLC_DMR1_EN_ADDR1: u8 = 0x10;
const AMR_DLC_DMR1_EN_ADDR2: u8 = 0x20;
const AMR_DLC_DMR1_EN_ADDR3: u8 = 0x40;
const AMR_DLC_DMR1_EN_ADDR4: u8 = 0x80;
const AMR_DLC_DMR1_EN_ADDRS: u8 = 0xf0;
const AMR_DLC_DMR2: u8 = 0x87;
const AMR_DLC_DMR2_RABRT_INT: u8 = 0x01;
const AMR_DLC_DMR2_RESID_INT: u8 = 0x02;
const AMR_DLC_DMR2_COLL_INT: u8 = 0x04;
const AMR_DLC_DMR2_FCS_INT: u8 = 0x08;
const AMR_DLC_DMR2_OVFL_INT: u8 = 0x10;
const AMR_DLC_DMR2_UNFL_INT: u8 = 0x20;
const AMR_DLC_DMR2_OVRN_INT: u8 = 0x40;
const AMR_DLC_DMR2_UNRN_INT: u8 = 0x80;
const AMR_DLC_1_7: u8 = 0x88;
const AMR_DLC_DRCR: u8 = 0x89;
const AMR_DLC_RNGR1: u8 = 0x8A;
const AMR_DLC_RNGR2: u8 = 0x8B;
const AMR_DLC_FRAR4: u8 = 0x8C;
const AMR_DLC_SRAR4: u8 = 0x8D;
const AMR_DLC_DMR3: u8 = 0x8E;
const AMR_DLC_DMR3_VA_INT: u8 = 0x01;
const AMR_DLC_DMR3_EOTP_INT: u8 = 0x02;
const AMR_DLC_DMR3_LBRP_INT: u8 = 0x04;
const AMR_DLC_DMR3_RBA_INT: u8 = 0x08;
const AMR_DLC_DMR3_LBT_INT: u8 = 0x10;
const AMR_DLC_DMR3_TBE_INT: u8 = 0x20;
const AMR_DLC_DMR3_RPLOST_INT: u8 = 0x40;
const AMR_DLC_DMR3_KEEP_FCS: u8 = 0x80;
const AMR_DLC_DMR4: u8 = 0x8F;
const AMR_DLC_DMR4_RCV_1: u8 = 0x00;
const AMR_DLC_DMR4_RCV_2: u8 = 0x01;
const AMR_DLC_DMR4_RCV_4: u8 = 0x02;
const AMR_DLC_DMR4_RCV_8: u8 = 0x03;
const AMR_DLC_DMR4_RCV_16: u8 = 0x01;
const AMR_DLC_DMR4_RCV_24: u8 = 0x02;
const AMR_DLC_DMR4_RCV_30: u8 = 0x03;
const AMR_DLC_DMR4_XMT_1: u8 = 0x00;
const AMR_DLC_DMR4_XMT_2: u8 = 0x04;
const AMR_DLC_DMR4_XMT_4: u8 = 0x08;
const AMR_DLC_DMR4_XMT_8: u8 = 0x0c;
const AMR_DLC_DMR4_XMT_10: u8 = 0x08;
const AMR_DLC_DMR4_XMT_14: u8 = 0x0c;
const AMR_DLC_DMR4_IDLE_MARK: u8 = 0x00;
const AMR_DLC_DMR4_IDLE_FLAG: u8 = 0x10;
const AMR_DLC_DMR4_ADDR_BOTH: u8 = 0x00;
const AMR_DLC_DMR4_ADDR_1ST: u8 = 0x20;
const AMR_DLC_DMR4_ADDR_2ND: u8 = 0xa0;
const AMR_DLC_DMR4_CR_ENABLE: u8 = 0x40;
const AMR_DLC_12_15: u8 = 0x90;
const AMR_DLC_ASR: u8 = 0x91;
const AMR_DLC_EFCR: u8 = 0x92;
const AMR_DLC_EFCR_EXTEND_FIFO: u8 = 0x01;
const AMR_DLC_EFCR_SEC_PKT_INT: u8 = 0x02;

const AMR_DSR1_VADDR: u8 = 0x01;
const AMR_DSR1_EORP: u8 = 0x02;
const AMR_DSR1_PKT_IP: u8 = 0x04;
const AMR_DSR1_DECHO_ON: u8 = 0x08;
const AMR_DSR1_DLOOP_ON: u8 = 0x10;
const AMR_DSR1_DBACK_OFF: u8 = 0x20;
const AMR_DSR1_EOTP: u8 = 0x40;
const AMR_DSR1_CXMT_ABRT: u8 = 0x80;

const AMR_DSR2_LBRP: u8 = 0x01;
const AMR_DSR2_RBA: u8 = 0x02;
const AMR_DSR2_RPLOST: u8 = 0x04;
const AMR_DSR2_LAST_BYTE: u8 = 0x08;
const AMR_DSR2_TBE: u8 = 0x10;
const AMR_DSR2_MARK_IDLE: u8 = 0x20;
const AMR_DSR2_FLAG_IDLE: u8 = 0x40;
const AMR_DSR2_SECOND_PKT: u8 = 0x80;

const AMR_DER_RABRT: u8 = 0x01;
const AMR_DER_RFRAME: u8 = 0x02;
const AMR_DER_COLLISION: u8 = 0x04;
const AMR_DER_FCS: u8 = 0x08;
const AMR_DER_OVFL: u8 = 0x10;
const AMR_DER_UNFL: u8 = 0x20;
const AMR_DER_OVRN: u8 = 0x40;
const AMR_DER_UNRN: u8 = 0x80;

// Peripheral Port
const AMR_PP_PPCR1: u8 = 0xC0;
const AMR_PP_PPSR: u8 = 0xC1;
const AMR_PP_PPIER: u8 = 0xC2;
const AMR_PP_MTDR: u8 = 0xC3;
const AMR_PP_MRDR: u8 = 0xC3;
const AMR_PP_CITDR0: u8 = 0xC4;
const AMR_PP_CIRDR0: u8 = 0xC4;
const AMR_PP_CITDR1: u8 = 0xC5;
const AMR_PP_CIRDR1: u8 = 0xC5;
const AMR_PP_PPCR2: u8 = 0xC8;
const AMR_PP_PPCR3: u8 = 0xC9;

const AMD7930_FLAG_PLAYBACK: u32 = 0x00000001;
const AMD7930_FLAG_CAPTURE: u32 = 0x00000002;

struct SndAmd7930 {
    // TODO: spinlock_t lock - would be replaced with Rust Mutex or SpinLock
    // void __iomem *regs - raw pointer to memory-mapped registers
    regs: *mut u8,
    flags: u32,
    map: Amd7930Map,
    // TODO: struct snd_card *card
    // TODO: struct snd_pcm *pcm
    // TODO: struct snd_pcm_substream *playback_substream
    // TODO: struct snd_pcm_substream *capture_substream

    p_orig: *mut u8,
    p_cur: *mut u8,
    p_left: i32,
    c_orig: *mut u8,
    c_cur: *mut u8,
    c_left: i32,

    rgain: i32,
    pgain: i32,
    mgain: i32,

    // TODO: struct platform_device *op
    irq: u32,
    // TODO: struct snd_amd7930 *next
}

static mut AMD7930_LIST: *mut SndAmd7930 = ptr::null_mut();

unsafe fn amd7930_idle(amd: *mut SndAmd7930) {
    // guard(spinlock_irqsave)(&amd->lock);
    // sbus_writeb(AMR_INIT, amd->regs + AMD7930_CR);
    // sbus_writeb(0, amd->regs + AMD7930_DR);
    let amd_ref = &mut *amd;

    let reg_ptr = amd_ref.regs.add(AMD7930_CR);
    ptr::write_volatile(reg_ptr, AMR_INIT);

    let reg_ptr = amd_ref.regs.add(AMD7930_DR);
    ptr::write_volatile(reg_ptr, 0);
}

unsafe fn amd7930_enable_ints(amd: *mut SndAmd7930) {
    // guard(spinlock_irqsave)(&amd->lock);
    // sbus_writeb(AMR_INIT, amd->regs + AMD7930_CR);
    // sbus_writeb(AM_INIT_ACTIVE, amd->regs + AMD7930_DR);
    let amd_ref = &mut *amd;

    let reg_ptr = amd_ref.regs.add(AMD7930_CR);
    ptr::write_volatile(reg_ptr, AMR_INIT);

    let reg_ptr = amd_ref.regs.add(AMD7930_DR);
    ptr::write_volatile(reg_ptr, AM_INIT_ACTIVE);
}

unsafe fn amd7930_disable_ints(amd: *mut SndAmd7930) {
    // guard(spinlock_irqsave)(&amd->lock);
    // sbus_writeb(AMR_INIT, amd->regs + AMD7930_CR);
    // sbus_writeb(AM_INIT_ACTIVE | AM_INIT_DISABLE_INTS, amd->regs + AMD7930_DR);
    let amd_ref = &mut *amd;

    let reg_ptr = amd_ref.regs.add(AMD7930_CR);
    ptr::write_volatile(reg_ptr, AMR_INIT);

    let reg_ptr = amd_ref.regs.add(AMD7930_DR);
    ptr::write_volatile(reg_ptr, AM_INIT_ACTIVE | AM_INIT_DISABLE_INTS);
}

unsafe fn amd7930_write_map(amd: *mut SndAmd7930) {
    let amd_ref = &mut *amd;
    let map = &amd_ref.map;

    let reg_ptr = amd_ref.regs.add(AMD7930_CR);
    ptr::write_volatile(reg_ptr, AMR_MAP_GX);
    let reg_ptr = amd_ref.regs.add(AMD7930_DR);
    ptr::write_volatile(reg_ptr, ((map.gx >> 0) & 0xff) as u8);
    ptr::write_volatile(reg_ptr, ((map.gx >> 8) & 0xff) as u8);

    let reg_ptr = amd_ref.regs.add(AMD7930_CR);
    ptr::write_volatile(reg_ptr, AMR_MAP_GR);
    let reg_ptr = amd_ref.regs.add(AMD7930_DR);
    ptr::write_volatile(reg_ptr, ((map.gr >> 0) & 0xff) as u8);
    ptr::write_volatile(reg_ptr, ((map.gr >> 8) & 0xff) as u8);

    let reg_ptr = amd_ref.regs.add(AMD7930_CR);
    ptr::write_volatile(reg_ptr, AMR_MAP_STGR);
    let reg_ptr = amd_ref.regs.add(AMD7930_DR);
    ptr::write_volatile(reg_ptr, ((map.stgr >> 0) & 0xff) as u8);
    ptr::write_volatile(reg_ptr, ((map.stgr >> 8) & 0xff) as u8);

    let reg_ptr = amd_ref.regs.add(AMD7930_CR);
    ptr::write_volatile(reg_ptr, AMR_MAP_GER);
    let reg_ptr = amd_ref.regs.add(AMD7930_DR);
    ptr::write_volatile(reg_ptr, ((map.ger >> 0) & 0xff) as u8);
    ptr::write_volatile(reg_ptr, ((map.ger >> 8) & 0xff) as u8);

    let reg_ptr = amd_ref.regs.add(AMD7930_CR);
    ptr::write_volatile(reg_ptr, AMR_MAP_MMR1);
    let reg_ptr = amd_ref.regs.add(AMD7930_DR);
    ptr::write_volatile(reg_ptr, map.mmr1);

    let reg_ptr = amd_ref.regs.add(AMD7930_CR);
    ptr::write_volatile(reg_ptr, AMR_MAP_MMR2);
    let reg_ptr = amd_ref.regs.add(AMD7930_DR);
    ptr::write_volatile(reg_ptr, map.mmr2);
}

// gx, gr & stg gains. this table must contain 256 elements with
// the 0th being "infinity" (the magic value 9008). The remaining
// elements match sun's gain curve (but with higher resolution):
// -18 to 0dB in .16dB steps then 0 to 12dB in .08dB steps.
const GX_COEFF: [u16; 256] = [
    0x9008, 0x8b7c, 0x8b51, 0x8b45, 0x8b42, 0x8b3b, 0x8b36, 0x8b33,
    0x8b32, 0x8b2a, 0x8b2b, 0x8b2c, 0x8b25, 0x8b23, 0x8b22, 0x8b22,
    0x9122, 0x8b1a, 0x8aa3, 0x8aa3, 0x8b1c, 0x8aa6, 0x912d, 0x912b,
    0x8aab, 0x8b12, 0x8aaa, 0x8ab2, 0x9132, 0x8ab4, 0x913c, 0x8abb,
    0x9142, 0x9144, 0x9151, 0x8ad5, 0x8aeb, 0x8a79, 0x8a5a, 0x8a4a,
    0x8b03, 0x91c2, 0x91bb, 0x8a3f, 0x8a33, 0x91b2, 0x9212, 0x9213,
    0x8a2c, 0x921d, 0x8a23, 0x921a, 0x9222, 0x9223, 0x922d, 0x9231,
    0x9234, 0x9242, 0x925b, 0x92dd, 0x92c1, 0x92b3, 0x92ab, 0x92a4,
    0x92a2, 0x932b, 0x9341, 0x93d3, 0x93b2, 0x93a2, 0x943c, 0x94b2,
    0x953a, 0x9653, 0x9782, 0x9e21, 0x9d23, 0x9cd2, 0x9c23, 0x9baa,
    0x9bde, 0x9b33, 0x9b22, 0x9b1d, 0x9ab2, 0xa142, 0xa1e5, 0x9a3b,
    0xa213, 0xa1a2, 0xa231, 0xa2eb, 0xa313, 0xa334, 0xa421, 0xa54b,
    0xada4, 0xac23, 0xab3b, 0xaaab, 0xaa5c, 0xb1a3, 0xb2ca, 0xb3bd,
    0xbe24, 0xbb2b, 0xba33, 0xc32b, 0xcb5a, 0xd2a2, 0xe31d, 0x0808,
    0x72ba, 0x62c2, 0x5c32, 0x52db, 0x513e, 0x4cce, 0x43b2, 0x4243,
    0x41b4, 0x3b12, 0x3bc3, 0x3df2, 0x34bd, 0x3334, 0x32c2, 0x3224,
    0x31aa, 0x2a7b, 0x2aaa, 0x2b23, 0x2bba, 0x2c42, 0x2e23, 0x25bb,
    0x242b, 0x240f, 0x231a, 0x22bb, 0x2241, 0x2223, 0x221f, 0x1a33,
    0x1a4a, 0x1acd, 0x2132, 0x1b1b, 0x1b2c, 0x1b62, 0x1c12, 0x1c32,
    0x1d1b, 0x1e71, 0x16b1, 0x1522, 0x1434, 0x1412, 0x1352, 0x1323,
    0x1315, 0x12bc, 0x127a, 0x1235, 0x1226, 0x11a2, 0x1216, 0x0a2a,
    0x11bc, 0x11d1, 0x1163, 0x0ac2, 0x0ab2, 0x0aab, 0x0b1b, 0x0b23,
    0x0b33, 0x0c0f, 0x0bb3, 0x0c1b, 0x0c3e, 0x0cb1, 0x0d4c, 0x0ec1,
    0x079a, 0x0614, 0x0521, 0x047c, 0x0422, 0x03b1, 0x03e3, 0x0333,
    0x0322, 0x031c, 0x02aa, 0x02ba, 0x02f2, 0x0242, 0x0232, 0x0227,
    0x0222, 0x021b, 0x01ad, 0x0212, 0x01b2, 0x01bb, 0x01cb, 0x01f6,
    0x0152, 0x013a, 0x0133, 0x0131, 0x012c, 0x0123, 0x0122, 0x00a2,
    0x011b, 0x011e, 0x0114, 0x00b1, 0x00aa, 0x00b3, 0x00bd, 0x00ba,
    0x00c5, 0x00d3, 0x00f3, 0x0062, 0x0051, 0x0042, 0x003b, 0x0033,
    0x0032, 0x002a, 0x002c, 0x0025, 0x0023, 0x0022, 0x001a, 0x0021,
    0x001b, 0x001b, 0x001d, 0x0015, 0x0013, 0x0013, 0x0012, 0x0012,
    0x000a, 0x000a, 0x0011, 0x0011, 0x000b, 0x000b, 0x000c, 0x000e,
];

const GER_COEFF: [u16; 21] = [
    0x431f, // 5. dB
    0x331f, // 5.5 dB
    0x40dd, // 6. dB
    0x11dd, // 6.5 dB
    0x440f, // 7. dB
    0x411f, // 7.5 dB
    0x311f, // 8. dB
    0x5520, // 8.5 dB
    0x10dd, // 9. dB
    0x4211, // 9.5 dB
    0x410f, // 10. dB
    0x111f, // 10.5 dB
    0x600b, // 11. dB
    0x00dd, // 11.5 dB
    0x4210, // 12. dB
    0x110f, // 13. dB
    0x7200, // 14. dB
    0x2110, // 15. dB
    0x2200, // 15.9 dB
    0x000b, // 16.9 dB
    0x000f  // 18. dB
];

unsafe fn amd7930_update_map(amd: *mut SndAmd7930) {
    let amd_ref = &mut *amd;
    let map = &mut amd_ref.map;

    map.gx = GX_COEFF[amd_ref.rgain as usize];
    map.stgr = GX_COEFF[amd_ref.mgain as usize];
    let level = ((amd_ref.pgain as u32) * (256 + GER_COEFF.len() as u32)) >> 8;
    if level >= 256 {
        map.ger = GER_COEFF[(level - 256) as usize];
        map.gr = GX_COEFF[255];
    } else {
        map.ger = GER_COEFF[0];
        map.gr = GX_COEFF[level as usize];
    }
    amd7930_write_map(amd);
}

// TODO: snd_amd7930_interrupt - IRQ handler would need appropriate Rust callback binding

// TODO: snd_amd7930_trigger

// TODO: snd_amd7930_playback_trigger

// TODO: snd_amd7930_capture_trigger

// TODO: snd_amd7930_playback_prepare

// TODO: snd_amd7930_capture_prepare

// TODO: snd_amd7930_playback_pointer

// TODO: snd_amd7930_capture_pointer

// TODO: snd_amd7930_pcm_hw - PCM hardware descriptor would be a struct with configuration

// TODO: snd_amd7930_playback_open

// TODO: snd_amd7930_capture_open

// TODO: snd_amd7930_playback_close

// TODO: snd_amd7930_capture_close

// TODO: snd_amd7930_playback_ops and snd_amd7930_capture_ops - operation tables

// TODO: snd_amd7930_pcm

// Volume control constants
const VOLUME_MONITOR: i32 = 0;
const VOLUME_CAPTURE: i32 = 1;
const VOLUME_PLAYBACK: i32 = 2;

// TODO: snd_amd7930_info_volume

// TODO: snd_amd7930_get_volume

// TODO: snd_amd7930_put_volume

// TODO: amd7930_controls - array of control descriptors

// TODO: snd_amd7930_mixer

// TODO: snd_amd7930_free

// TODO: snd_amd7930_dev_free

// TODO: snd_amd7930_dev_ops - device operations

// TODO: snd_amd7930_create

// TODO: amd7930_sbus_probe - platform device probe function

// TODO: amd7930_match - device match table

// TODO: amd7930_sbus_driver - platform driver structure

// TODO: amd7930_init - module init function

// TODO: amd7930_exit - module exit function

// Module entry points (would be connected via module_init/module_exit)
// fn amd7930_init()
// fn amd7930_exit()

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
