/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>,
 *		     Creative Labs, Inc.
 *  Definitions for EMU10K1 (SB Live!) chips
 */
// #ifndef __SOUND_EMU10K1_H
// C declaration: #define __SOUND_EMU10K1_H


// #include <sound/pcm.h>
// #include <sound/rawmidi.h>
// #include <sound/hwdep.h>
// #include <sound/ac97_codec.h>
// #include <sound/util_mem.h>
// #include <sound/pcm-indirect.h>
// #include <sound/timer.h>
// #include <linux/interrupt.h>
// #include <linux/mutex.h>
// #include <linux/firmware.h>
// #include <linux/io.h>

// #include <uapi/sound/emu10k1.h>

/* ------------------- DEFINES -------------------- */

pub const EMUPAGESIZE: u64 = 4096;
pub const MAXPAGES0: u64 = 4096;
pub const MAXPAGES1: u64 = 8192;
pub const NUM_G: u64 = 64;
pub const NUM_EFX_PLAYBACK: u64 = 16;

/* FIXME? - according to the OSS driver the EMU10K1 needs a 29 bit DMA mask */
pub const EMU10K1_DMA_MASK: u64 = 0x7fffffffu32;
pub const AUDIGY_DMA_MASK: u64 = 0xffffffffu32;

pub const TMEMSIZE: u64 = 256*1024;

// C declaration: #define IP_TO_CP(ip) ((ip == 0) ? 0 : (((0x00001000uL | (ip & 0x00000FFFL)) << (((ip >> 12) & 0x000FL) + 4)) & 0xFFFF0000uL))

// This is used to define hardware bit-fields (sub-registers) by combining
// the bit shift and count with the actual register address. The passed
// mask must represent a single run of adjacent bits.
// The non-concatenating (_NC) variant should be used directly only for
// sub-registers that do not follow the <register>_<field> naming pattern.
// C declaration: #define SUB_REG_NC(reg, field, mask) \
// C declaration: 	enum { \
// C declaration: 		field ## _MASK = mask, \
// C declaration: 		field = reg | \
// C declaration: 			(__builtin_ctz(mask) << 16) | \
// C declaration: 			(__builtin_popcount(mask) << 24), \
// C declaration: 	};
// C declaration: #define SUB_REG(reg, field, mask) SUB_REG_NC(reg, reg ## _ ## field, mask)

// Macros for manipulating values of bit-fields declared using the above macros.
// Best used with constant register addresses, as otherwise quite some code is
// generated. The actual register read/write functions handle combined addresses
// automatically, so use of these macros conveys no advantage when accessing a
// single sub-register at a time.
// C declaration: #define REG_SHIFT(r) (((r) >> 16) & 0x1f)
// C declaration: #define REG_SIZE(r) (((r) >> 24) & 0x1f)
// C declaration: #define REG_MASK0(r) ((1U << REG_SIZE(r)) - 1U)
// C declaration: #define REG_MASK(r) (REG_MASK0(r) << REG_SHIFT(r))
// C declaration: #define REG_VAL_GET(r, v) ((v & REG_MASK(r)) >> REG_SHIFT(r))
// C declaration: #define REG_VAL_PUT(r, v) ((v) << REG_SHIFT(r))

// List terminator for snd_emu10k1_ptr_write_multiple()
pub const REGLIST_END: u64 = ~0;

// Audigy specify registers are prefixed with 'A_'

/************************************************************************************************/
/* PCI function 0 registers, address = <val> + PCIBASE0						*/
/************************************************************************************************/

pub const PTR: u64 = 0x00;
// C declaration: 						/* NOTE: The CHANNELNUM and ADDRESS words can	*/
// C declaration: 						/* be modified independently of each other.	*/
pub const PTR_CHANNELNUM_MASK: u64 = 0x0000003f;
// C declaration: 						/* channel number of the register to be		*/
// C declaration: 						/* accessed.  For non per-channel registers the	*/
// C declaration: 						/* value should be set to zero.			*/
pub const PTR_ADDRESS_MASK: u64 = 0x07ff0000;
pub const A_PTR_ADDRESS_MASK: u64 = 0x0fff0000;

pub const DATA: u64 = 0x04;

pub const IPR: u64 = 0x08;
// C declaration: 						/* Clear pending interrupts by writing a 1 to	*/
// C declaration: 						/* the relevant bits and zero to the other bits	*/
pub const IPR_P16V: u64 = 0x80000000;
// C declaration: 						   to interrupt */
pub const IPR_WATERMARK_REACHED: u64 = 0x40000000;
pub const IPR_A_GPIO: u64 = 0x20000000;

/* The next two interrupts are for the midi port on the Audigy Drive (A_MPU1)			*/
pub const IPR_A_MIDITRANSBUFEMPTY2: u64 = 0x10000000;
pub const IPR_A_MIDIRECVBUFEMPTY2: u64 = 0x08000000;

pub const IPR_SPDIFBUFFULL: u64 = 0x04000000;
pub const IPR_SPDIFBUFHALFFULL: u64 = 0x02000000;

pub const IPR_SAMPLERATETRACKER: u64 = 0x01000000;
pub const IPR_FXDSP: u64 = 0x00800000;
pub const IPR_FORCEINT: u64 = 0x00400000;
pub const IPR_PCIERROR: u64 = 0x00200000;
pub const IPR_VOLINCR: u64 = 0x00100000;
pub const IPR_VOLDECR: u64 = 0x00080000;
pub const IPR_MUTE: u64 = 0x00040000;
pub const IPR_MICBUFFULL: u64 = 0x00020000;
pub const IPR_MICBUFHALFFULL: u64 = 0x00010000;
pub const IPR_ADCBUFFULL: u64 = 0x00008000;
pub const IPR_ADCBUFHALFFULL: u64 = 0x00004000;
pub const IPR_EFXBUFFULL: u64 = 0x00002000;
pub const IPR_EFXBUFHALFFULL: u64 = 0x00001000;
pub const IPR_GPSPDIFSTATUSCHANGE: u64 = 0x00000800;
pub const IPR_CDROMSTATUSCHANGE: u64 = 0x00000400;
pub const IPR_INTERVALTIMER: u64 = 0x00000200;
pub const IPR_MIDITRANSBUFEMPTY: u64 = 0x00000100;
pub const IPR_MIDIRECVBUFEMPTY: u64 = 0x00000080;
pub const IPR_CHANNELLOOP: u64 = 0x00000040;
// C declaration: 						/* The interrupt is triggered shortly after	*/
// C declaration: 						/* CCR_READADDRESS has crossed the boundary;	*/
// C declaration: 						/* due to the cache, this runs ahead of the	*/
// C declaration: 						/* actual playback position.			*/
pub const IPR_CHANNELNUMBERMASK: u64 = 0x0000003f;
// C declaration: 						/* highest set channel in CLIPL, CLIPH, HLIPL,  */
// C declaration: 						/* or HLIPH.  When IPR is written with CL set,	*/
// C declaration: 						/* the bit in H/CLIPL or H/CLIPH corresponding	*/
// C declaration: 						/* to the CN value written will be cleared.	*/

pub const INTE: u64 = 0x0c;
pub const INTE_VIRTUALSB_MASK: u64 = 0xc0000000;
pub const INTE_VIRTUALSB_220: u64 = 0x00000000;
pub const INTE_VIRTUALSB_240: u64 = 0x40000000;
pub const INTE_VIRTUALSB_260: u64 = 0x80000000;
pub const INTE_VIRTUALSB_280: u64 = 0xc0000000;
pub const INTE_VIRTUALMPU_MASK: u64 = 0x30000000;
pub const INTE_VIRTUALMPU_300: u64 = 0x00000000;
pub const INTE_VIRTUALMPU_310: u64 = 0x10000000;
pub const INTE_VIRTUALMPU_320: u64 = 0x20000000;
pub const INTE_VIRTUALMPU_330: u64 = 0x30000000;
pub const INTE_MASTERDMAENABLE: u64 = 0x08000000;
pub const INTE_SLAVEDMAENABLE: u64 = 0x04000000;
pub const INTE_MASTERPICENABLE: u64 = 0x02000000;
pub const INTE_SLAVEPICENABLE: u64 = 0x01000000;
pub const INTE_VSBENABLE: u64 = 0x00800000;
pub const INTE_ADLIBENABLE: u64 = 0x00400000;
pub const INTE_MPUENABLE: u64 = 0x00200000;
pub const INTE_FORCEINT: u64 = 0x00100000;

pub const INTE_MRHANDENABLE: u64 = 0x00080000;
// C declaration: 						/* NOTE: There is no reason to use this under	*/
// C declaration: 						/* Linux, and it will cause odd hardware 	*/
// C declaration: 						/* behavior and possibly random segfaults and	*/
// C declaration: 						/* lockups if enabled.				*/

pub const INTE_A_GPIOENABLE: u64 = 0x00040000;

/* The next two interrupts are for the midi port on the Audigy Drive (A_MPU1)			*/
pub const INTE_A_MIDITXENABLE2: u64 = 0x00020000;
pub const INTE_A_MIDIRXENABLE2: u64 = 0x00010000;

pub const INTE_A_SPDIF_BUFFULL_ENABLE: u64 = 0x00008000;
pub const INTE_A_SPDIF_HALFBUFFULL_ENABLE: u64 = 0x00004000;

pub const INTE_SAMPLERATETRACKER: u64 = 0x00002000;
// C declaration: 						/* NOTE: This bit must always be enabled       	*/
pub const INTE_FXDSPENABLE: u64 = 0x00001000;
pub const INTE_PCIERRORENABLE: u64 = 0x00000800;
pub const INTE_VOLINCRENABLE: u64 = 0x00000400;
pub const INTE_VOLDECRENABLE: u64 = 0x00000200;
pub const INTE_MUTEENABLE: u64 = 0x00000100;
pub const INTE_MICBUFENABLE: u64 = 0x00000080;
pub const INTE_ADCBUFENABLE: u64 = 0x00000040;
pub const INTE_EFXBUFENABLE: u64 = 0x00000020;
pub const INTE_GPSPDIFENABLE: u64 = 0x00000010;
pub const INTE_CDSPDIFENABLE: u64 = 0x00000008;
pub const INTE_INTERVALTIMERENB: u64 = 0x00000004;
pub const INTE_MIDITXENABLE: u64 = 0x00000002;
pub const INTE_MIDIRXENABLE: u64 = 0x00000001;

pub const WC: u64 = 0x10;
pub const SAMPLECOUNTER_MASK: u32 = 0x03FFFFC0;
pub const SAMPLECOUNTER: u32 = WC | ((0x03FFFFC0.trailing_zeros() as u32) << 16) | ((0x03FFFFC0.count_ones() as u32) << 24);
pub const CURRENTCHANNEL_MASK: u32 = 0x0000003F;
pub const CURRENTCHANNEL: u32 = WC | ((0x0000003F.trailing_zeros() as u32) << 16) | ((0x0000003F.count_ones() as u32) << 24);
// C declaration: 						/* NOTE: Each channel takes 1/64th of a sample	*/
// C declaration: 						/* period to be serviced.			*/

pub const HCFG: u64 = 0x14;
// C declaration: 						/* NOTE: There is no reason to use the legacy	*/
// C declaration: 						/* SoundBlaster emulation stuff described below	*/
// C declaration: 						/* under Linux, and all kinds of weird hardware	*/
// C declaration: 						/* behavior can result if you try.  Don't.	*/
pub const HCFG_LEGACYFUNC_MASK: u64 = 0xe0000000;
pub const HCFG_LEGACYFUNC_MPU: u64 = 0x00000000;
pub const HCFG_LEGACYFUNC_SB: u64 = 0x40000000;
pub const HCFG_LEGACYFUNC_AD: u64 = 0x60000000;
pub const HCFG_LEGACYFUNC_MPIC: u64 = 0x80000000;
pub const HCFG_LEGACYFUNC_MDMA: u64 = 0xa0000000;
pub const HCFG_LEGACYFUNC_SPCI: u64 = 0xc0000000;
pub const HCFG_LEGACYFUNC_SDMA: u64 = 0xe0000000;
pub const HCFG_IOCAPTUREADDR: u64 = 0x1f000000;
pub const HCFG_LEGACYWRITE: u64 = 0x00800000;
pub const HCFG_LEGACYWORD: u64 = 0x00400000;
pub const HCFG_LEGACYINT: u64 = 0x00200000;
// C declaration: 						/* NOTE: The rest of the bits in this register	*/
// C declaration: 						/* _are_ relevant under Linux.			*/
pub const HCFG_PUSH_BUTTON_ENABLE: u64 = 0x00100000;
pub const HCFG_BAUD_RATE: u64 = 0x00080000;
pub const HCFG_EXPANDED_MEM: u64 = 0x00040000;
pub const HCFG_CODECFORMAT_MASK: u64 = 0x00030000;

/* Specific to Alice2, CA0102 */

pub const HCFG_CODECFORMAT_AC97_1: u64 = 0x00000000;
pub const HCFG_CODECFORMAT_AC97_2: u64 = 0x00010000;
pub const HCFG_AUTOMUTE_ASYNC: u64 = 0x00008000;
// C declaration: 						/* will automatically mute their output when	*/
// C declaration: 						/* they are not rate-locked to the external	*/
// C declaration: 						/* async audio source  				*/
pub const HCFG_AUTOMUTE_SPDIF: u64 = 0x00004000;
// C declaration: 						/* will automatically mute their output when	*/
// C declaration: 						/* the SPDIF V-bit indicates invalid audio	*/
pub const HCFG_EMU32_SLAVE: u64 = 0x00002000;
pub const HCFG_SLOW_RAMP: u64 = 0x00001000;
/* 0x00000800 not used on Alice2 */
pub const HCFG_PHASE_TRACK_MASK: u64 = 0x00000700;
// C declaration: 						/* phase track the previous input.		*/
// C declaration: 						/* I2S0 can phase track the last S/PDIF input	*/
pub const HCFG_I2S_ASRC_ENABLE: u64 = 0x00000070;
// C declaration: 						/* conversion for the corresponding		*/
// C declaration:  						/* I2S format input				*/
/* Rest of HCFG 0x0000000f same as below. LOCKSOUNDCACHE etc.  */

/* Older chips */

pub const HCFG_CODECFORMAT_AC97: u64 = 0x00000000;
pub const HCFG_CODECFORMAT_I2S: u64 = 0x00010000;
pub const HCFG_GPINPUT0: u64 = 0x00004000;
pub const HCFG_GPINPUT1: u64 = 0x00002000;
pub const HCFG_GPOUTPUT_MASK: u64 = 0x00001c00;
pub const HCFG_GPOUT0: u64 = 0x00001000;
pub const HCFG_GPOUT1: u64 = 0x00000800;
pub const HCFG_GPOUT2: u64 = 0x00000400;
pub const HCFG_JOYENABLE: u64 = 0x00000200;
pub const HCFG_PHASETRACKENABLE: u64 = 0x00000100;
// C declaration: 						/* 1 = Force all 3 async digital inputs to use	*/
// C declaration: 						/* the same async sample rate tracker (ZVIDEO)	*/
pub const HCFG_AC3ENABLE_MASK: u64 = 0x000000e0;
pub const HCFG_AC3ENABLE_ZVIDEO: u64 = 0x00000080;
pub const HCFG_AC3ENABLE_CDSPDIF: u64 = 0x00000040;
pub const HCFG_AC3ENABLE_GPSPDIF: u64 = 0x00000020;
pub const HCFG_AUTOMUTE: u64 = 0x00000010;
// C declaration: 						/* will automatically mute their output when	*/
// C declaration: 						/* they are not rate-locked to the external	*/
// C declaration: 						/* async audio source  				*/
pub const HCFG_LOCKSOUNDCACHE: u64 = 0x00000008;
// C declaration: 						/* NOTE: This should generally never be used.  	*/
pub const LOCKTANKCACHE_MASK: u32 = 0x00000004;
pub const LOCKTANKCACHE: u32 = HCFG | ((0x00000004.trailing_zeros() as u32) << 16) | ((0x00000004.count_ones() as u32) << 24);
// C declaration: 						/* NOTE: This should generally never be used.  	*/
pub const HCFG_MUTEBUTTONENABLE: u64 = 0x00000002;
// C declaration: 						/* NOTE: This is a 'cheap' way to implement a	*/
// C declaration: 						/* master mute function on the mute button, and	*/
// C declaration: 						/* in general should not be used unless a more	*/
// C declaration: 						/* sophisticated master mute function has not	*/
// C declaration: 						/* been written.       				*/
pub const HCFG_AUDIOENABLE: u64 = 0x00000001;
// C declaration: 						/* Should be set to 1 when the EMU10K1 is	*/
// C declaration: 						/* completely initialized.			*/

// On Audigy, the MPU port moved to the 0x70-0x74 ptr registers

pub const MUDATA: u64 = 0x18;

pub const MUCMD: u64 = 0x19;
pub const MUCMD_RESET: u64 = 0xff;
pub const MUCMD_ENTERUARTMODE: u64 = 0x3f;
// C declaration: 						/* NOTE: All other commands are ignored		*/

pub const MUSTAT: u64 = MUCMD;
pub const MUSTAT_IRDYN: u64 = 0x80;
pub const MUSTAT_ORDYN: u64 = 0x40;

pub const A_GPIO: u64 = 0x18;
pub const A_GPINPUT_MASK: u64 = 0xff00;
pub const A3_GPINPUT_MASK: u64 = 0x3f00;
pub const A_GPOUTPUT_MASK: u64 = 0x00ff;

// The GPIO port is used for I/O config on Sound Blasters;
// card-specific info can be found in the emu_chip_details table.
// On E-MU cards the port is used as the interface to the FPGA.

// Audigy output/GPIO stuff taken from the kX drivers
pub const A_IOCFG: u64 = A_GPIO;
pub const A_IOCFG_GPOUT0: u64 = 0x0044;
pub const A_IOCFG_DISABLE_ANALOG: u64 = 0x0040;
pub const A_IOCFG_ENABLE_DIGITAL: u64 = 0x0004;
pub const A_IOCFG_ENABLE_DIGITAL_AUDIGY4: u64 = 0x0080;
pub const A_IOCFG_UNKNOWN_20: u64 = 0x0020;
pub const A_IOCFG_DISABLE_AC97_FRONT: u64 = 0x0080;
pub const A_IOCFG_GPOUT1: u64 = 0x0002;
pub const A_IOCFG_GPOUT2: u64 = 0x0001;
pub const A_IOCFG_MULTIPURPOSE_JACK: u64 = 0x2000;
// C declaration:                                                 /* + digital for generic 10k2			*/
pub const A_IOCFG_DIGITAL_JACK: u64 = 0x1000;
pub const A_IOCFG_FRONT_JACK: u64 = 0x4000;
pub const A_IOCFG_REAR_JACK: u64 = 0x8000;
pub const A_IOCFG_PHONES_JACK: u64 = 0x0100;

pub const TIMER: u64 = 0x1a;
// C declaration: 						/* NOTE: After the rate is changed, a maximum	*/
// C declaration: 						/* of 1024 sample periods should be allowed	*/
// C declaration: 						/* before the new rate is guaranteed accurate.	*/
pub const TIMER_RATE_MASK: u64 = 0x03ff;
// C declaration: 						/* 0 == 1024 periods, [1..4] are not useful	*/

pub const AC97DATA: u64 = 0x1c;

pub const AC97ADDRESS: u64 = 0x1e;
pub const AC97ADDRESS_READY: u64 = 0x80;
pub const AC97ADDRESS_ADDRESS: u64 = 0x7f;

/* Available on the Audigy 2 and Audigy 4 only. This is the P16V chip. */
pub const PTR2: u64 = 0x20;
pub const DATA2: u64 = 0x24;
pub const IPR2: u64 = 0x28;
pub const IPR2_PLAYBACK_CH_0_LOOP: u64 = 0x00001000;
pub const IPR2_PLAYBACK_CH_0_HALF_LOOP: u64 = 0x00000100;
pub const IPR2_CAPTURE_CH_0_LOOP: u64 = 0x00100000;
pub const IPR2_CAPTURE_CH_0_HALF_LOOP: u64 = 0x00010000;
// C declaration: 						/* 0x00000100 Playback. Only in once per period.
						 * 0x00110000 Capture. Int on half buffer.
						 */
pub const INTE2: u64 = 0x2c;
pub const INTE2_PLAYBACK_CH_0_LOOP: u64 = 0x00001000;
pub const INTE2_PLAYBACK_CH_0_HALF_LOOP: u64 = 0x00000100;
pub const INTE2_PLAYBACK_CH_1_LOOP: u64 = 0x00002000;
pub const INTE2_PLAYBACK_CH_1_HALF_LOOP: u64 = 0x00000200;
pub const INTE2_PLAYBACK_CH_2_LOOP: u64 = 0x00004000;
pub const INTE2_PLAYBACK_CH_2_HALF_LOOP: u64 = 0x00000400;
pub const INTE2_PLAYBACK_CH_3_LOOP: u64 = 0x00008000;
pub const INTE2_PLAYBACK_CH_3_HALF_LOOP: u64 = 0x00000800;
pub const INTE2_CAPTURE_CH_0_LOOP: u64 = 0x00100000;
pub const INTE2_CAPTURE_CH_0_HALF_LOOP: u64 = 0x00010000;
pub const HCFG2: u64 = 0x34;
// C declaration: 						/* 0x00000000 2-channel output. */
// C declaration: 						/* 0x00000200 8-channel output. */
// C declaration: 						/* 0x00000004 pauses stream/irq fail. */
// C declaration: 						/* Rest of bits do nothing to sound output */
// C declaration: 						/* bit 0: Enable P16V audio.
						 * bit 1: Lock P16V record memory cache.
						 * bit 2: Lock P16V playback memory cache.
						 * bit 3: Dummy record insert zero samples.
						 * bit 8: Record 8-channel in phase.
						 * bit 9: Playback 8-channel in phase.
						 * bit 11-12: Playback mixer attenuation: 0=0dB, 1=-6dB, 2=-12dB, 3=Mute.
						 * bit 13: Playback mixer enable.
						 * bit 14: Route SRC48 mixer output to fx engine.
						 * bit 15: Enable IEEE 1394 chip.
						 */
pub const IPR3: u64 = 0x38;
pub const INTE3: u64 = 0x3c;

/************************************************************************************************/
/* PCI function 1 registers, address = <val> + PCIBASE1						*/
/************************************************************************************************/

pub const JOYSTICK1: u64 = 0x00;
pub const JOYSTICK2: u64 = 0x01;
pub const JOYSTICK3: u64 = 0x02;
pub const JOYSTICK4: u64 = 0x03;
pub const JOYSTICK5: u64 = 0x04;
pub const JOYSTICK6: u64 = 0x05;
pub const JOYSTICK7: u64 = 0x06;
pub const JOYSTICK8: u64 = 0x07;

/* When writing, any write causes JOYSTICK_COMPARATOR output enable to be pulsed on write.	*/
/* When reading, use these bitfields: */
pub const JOYSTICK_BUTTONS: u64 = 0x0f;
pub const JOYSTICK_COMPARATOR: u64 = 0xf0;

/********************************************************************************************************/
/* Emu10k1 pointer-offset register set, accessed through the PTR and DATA registers			*/
/********************************************************************************************************/

// No official documentation was released for EMU10K1, but some info
// about playback can be extrapolated from the EMU8K documents:
// "AWE32/EMU8000 Programmer’s Guide" (emu8kpgm.pdf) - registers
// "AWE32 Developer's Information Pack" (adip301.pdf) - high-level view

// The short version:
// - The engine has 64 playback channels, also called voices. The channels
//   operate independently, except when paired for stereo (see below).
// - PCM samples are fetched into the cache; see description of CD0 below.
// - Samples are consumed at the rate CPF_CURRENTPITCH.
// - 8-bit samples are transformed upon use: cooked = (raw ^ 0x80) << 8
// - 8 samples are read at CCR_READADDRESS:CPF_FRACADDRESS and interpolated
//   according to CCCA_INTERPROM_*. With CCCA_INTERPROM_0 selected and a zero
//   CPF_FRACADDRESS, this results in CCR_READADDRESS[3] being used verbatim.
// - The value is multiplied by CVCF_CURRENTVOL.
// - The value goes through a filter with cutoff CVCF_CURRENTFILTER;
//   delay stages Z1 and Z2.
// - The value is added by so-called `sends` to 4 (EMU10K1) / 8 (EMU10K2)
//   of the 16 (EMU10K1) / 64 (EMU10K2) FX bus accumulators via FXRT*,
//   multiplied by a per-send amount (*_FXSENDAMOUNT_*).
//   The scaling of the send amounts is exponential-ish.
// - The DSP has a go at FXBUS* and outputs the values to EXTOUT* or EMU32OUT*.
// - The pitch, volume, and filter cutoff can be modulated by two envelope
//   engines and two low frequency oscillators.
// - To avoid abrupt changes to the parameters (which may cause audible
//   distortion), the modulation engine sets the target registers, towards
//   which the current registers "swerve" gradually.

// For the odd channel in a stereo pair, these registers are meaningless:
//   CPF_STEREO, CPF_CURRENTPITCH, PTRX_PITCHTARGET, CCR_CACHEINVALIDSIZE,
//   PSST_LOOPSTARTADDR, DSL_LOOPENDADDR, CCCA_CURRADDR
// The somewhat non-obviously still meaningful ones are:
//   CPF_STOP, CPF_FRACADDRESS, CCR_READADDRESS (!),
//   CCCA_INTERPROM, CCCA_8BITSELECT (!)
// (The envelope engine is ignored here, as stereo matters only for verbatim playback.)

pub const CPF: u64 = 0x00;
pub const CURRENTPITCH_MASK: u32 = 0xffff0000;
pub const CURRENTPITCH: u32 = CPF | ((0xffff0000.trailing_zeros() as u32) << 16) | ((0xffff0000.count_ones() as u32) << 24);
pub const CPF_STEREO_MASK: u64 = 0x00008000;
pub const STOP_MASK: u32 = 0x00004000;
pub const STOP: u32 = CPF | ((0x00004000.trailing_zeros() as u32) << 16) | ((0x00004000.count_ones() as u32) << 24);
// C declaration: 						/* Can be set only while matching bit in SOLEx is 1	*/
pub const CPF_FRACADDRESS_MASK: u64 = 0x00003fff;

pub const PTRX: u64 = 0x01;
pub const PITCHTARGET_MASK: u32 = 0xffff0000;
pub const PITCHTARGET: u32 = PTRX | ((0xffff0000.trailing_zeros() as u32) << 16) | ((0xffff0000.count_ones() as u32) << 24);
pub const FXSENDAMOUNT_A_MASK: u32 = 0x0000ff00;
pub const FXSENDAMOUNT_A: u32 = PTRX | ((0x0000ff00.trailing_zeros() as u32) << 16) | ((0x0000ff00.count_ones() as u32) << 24);
pub const FXSENDAMOUNT_B_MASK: u32 = 0x000000ff;
pub const FXSENDAMOUNT_B: u32 = PTRX | ((0x000000ff.trailing_zeros() as u32) << 16) | ((0x000000ff.count_ones() as u32) << 24);

// Note: the volumes are raw multpliers, so real 100% is impossible.
pub const CVCF: u64 = 0x02;
pub const CURRENTVOL_MASK: u32 = 0xffff0000;
pub const CURRENTVOL: u32 = CVCF | ((0xffff0000.trailing_zeros() as u32) << 16) | ((0xffff0000.count_ones() as u32) << 24);
pub const CURRENTFILTER_MASK: u32 = 0x0000ffff;
pub const CURRENTFILTER: u32 = CVCF | ((0x0000ffff.trailing_zeros() as u32) << 16) | ((0x0000ffff.count_ones() as u32) << 24);

pub const VTFT: u64 = 0x03;
pub const VOLUMETARGET_MASK: u32 = 0xffff0000;
pub const VOLUMETARGET: u32 = VTFT | ((0xffff0000.trailing_zeros() as u32) << 16) | ((0xffff0000.count_ones() as u32) << 24);
pub const FILTERTARGET_MASK: u32 = 0x0000ffff;
pub const FILTERTARGET: u32 = VTFT | ((0x0000ffff.trailing_zeros() as u32) << 16) | ((0x0000ffff.count_ones() as u32) << 24);

pub const Z1: u64 = 0x05;

pub const Z2: u64 = 0x04;

pub const PSST: u64 = 0x06;
pub const FXSENDAMOUNT_C_MASK: u32 = 0xff000000;
pub const FXSENDAMOUNT_C: u32 = PSST | ((0xff000000.trailing_zeros() as u32) << 16) | ((0xff000000.count_ones() as u32) << 24);
pub const LOOPSTARTADDR_MASK: u32 = 0x00ffffff;
pub const LOOPSTARTADDR: u32 = PSST | ((0x00ffffff.trailing_zeros() as u32) << 16) | ((0x00ffffff.count_ones() as u32) << 24);

pub const DSL: u64 = 0x07;
pub const FXSENDAMOUNT_D_MASK: u32 = 0xff000000;
pub const FXSENDAMOUNT_D: u32 = DSL | ((0xff000000.trailing_zeros() as u32) << 16) | ((0xff000000.count_ones() as u32) << 24);
pub const LOOPENDADDR_MASK: u32 = 0x00ffffff;
pub const LOOPENDADDR: u32 = DSL | ((0x00ffffff.trailing_zeros() as u32) << 16) | ((0x00ffffff.count_ones() as u32) << 24);

pub const CCCA: u64 = 0x08;
pub const RESONANCE_MASK: u32 = 0xf0000000;
pub const RESONANCE: u32 = CCCA | ((0xf0000000.trailing_zeros() as u32) << 16) | ((0xf0000000.count_ones() as u32) << 24);
pub const CCCA_INTERPROM_MASK: u64 = 0x0e000000;
// C declaration: 						/* 1 == full band, 7 == lowpass				*/
// C declaration: 						/* ROM 0 is used when pitch shifting downward or less	*/
// C declaration: 						/* then 3 semitones upward.  Increasingly higher ROM	*/
// C declaration: 						/* numbers are used, typically in steps of 3 semitones,	*/
// C declaration: 						/* as upward pitch shifting is performed.		*/
pub const CCCA_INTERPROM_0: u64 = 0x00000000;
pub const CCCA_INTERPROM_1: u64 = 0x02000000;
pub const CCCA_INTERPROM_2: u64 = 0x04000000;
pub const CCCA_INTERPROM_3: u64 = 0x06000000;
pub const CCCA_INTERPROM_4: u64 = 0x08000000;
pub const CCCA_INTERPROM_5: u64 = 0x0a000000;
pub const CCCA_INTERPROM_6: u64 = 0x0c000000;
pub const CCCA_INTERPROM_7: u64 = 0x0e000000;
pub const CCCA_8BITSELECT: u64 = 0x01000000;
// C declaration: 						/* 8-bit samples are unsigned, 16-bit ones signed	*/
pub const CURRADDR_MASK: u32 = 0x00ffffff;
pub const CURRADDR: u32 = CCCA | ((0x00ffffff.trailing_zeros() as u32) << 16) | ((0x00ffffff.count_ones() as u32) << 24);

pub const CCR: u64 = 0x09;
pub const CACHEINVALIDSIZE_MASK: u32 = 0xfe000000;
pub const CACHEINVALIDSIZE: u32 = CCR | ((0xfe000000.trailing_zeros() as u32) << 16) | ((0xfe000000.count_ones() as u32) << 24);
pub const CCR_CACHELOOPFLAG: u64 = 0x01000000;
pub const CCR_INTERLEAVEDSAMPLES: u64 = 0x00800000;
// C declaration: 						/* Auto-set from CPF_STEREO_MASK			*/
pub const CCR_WORDSIZEDSAMPLES: u64 = 0x00400000;
// C declaration: 						/* Auto-set from CCCA_8BITSELECT			*/
pub const READADDRESS_MASK: u32 = 0x003f0000;
pub const READADDRESS: u32 = CCR | ((0x003f0000.trailing_zeros() as u32) << 16) | ((0x003f0000.count_ones() as u32) << 24);
pub const LOOPINVALSIZE_MASK: u32 = 0x0000fe00;
pub const LOOPINVALSIZE: u32 = CCR | ((0x0000fe00.trailing_zeros() as u32) << 16) | ((0x0000fe00.count_ones() as u32) << 24);
// C declaration: 						/* NOTE: This is valid only if CACHELOOPFLAG is set	*/
pub const CCR_LOOPFLAG: u64 = 0x00000100;
pub const CACHELOOPADDRHI_MASK: u32 = 0x000000ff;
pub const CACHELOOPADDRHI: u32 = CCR | ((0x000000ff.trailing_zeros() as u32) << 16) | ((0x000000ff.count_ones() as u32) << 24);

pub const CLP: u64 = 0x0a;
// C declaration: 						/* NOTE: This register is normally not used		*/
pub const CACHELOOPADDR_MASK: u32 = 0x0000ffff;
pub const CACHELOOPADDR: u32 = CLP | ((0x0000ffff.trailing_zeros() as u32) << 16) | ((0x0000ffff.count_ones() as u32) << 24);

pub const FXRT: u64 = 0x0b;
// C declaration: 						/* NOTE: It is illegal to assign the same routing to	*/
// C declaration: 						/* two effects sends.					*/
pub const FXRT_CHANNELA: u64 = 0x000f0000;
pub const FXRT_CHANNELB: u64 = 0x00f00000;
pub const FXRT_CHANNELC: u64 = 0x0f000000;
pub const FXRT_CHANNELD: u64 = 0xf0000000;

pub const MAPA: u64 = 0x0c;
pub const MAPB: u64 = 0x0d;

pub const MAP_PTE_MASK0: u64 = 0xfffff000;
pub const MAP_PTI_MASK0: u64 = 0x00000fff;

pub const MAP_PTE_MASK1: u64 = 0xffffe000;
pub const MAP_PTI_MASK1: u64 = 0x00001fff;

/* 0x0e, 0x0f: Internal state, at least on Audigy */

pub const ENVVOL: u64 = 0x10;
pub const ENVVOL_MASK: u64 = 0x0000ffff;
// C declaration: 						/* 0x8000-n == 666*n usec delay	       			*/

pub const ATKHLDV: u64 = 0x11;
pub const ATKHLDV_PHASE0_MASK: u64 = 0x00008000;
pub const ATKHLDV_HOLDTIME_MASK: u64 = 0x00007f00;
pub const ATKHLDV_ATTACKTIME_MASK: u64 = 0x0000007f;
// C declaration: 						/* 0 = infinite, 1 = 10.9msec, ... 0x7f = 5.5msec	*/

pub const DCYSUSV: u64 = 0x12;
pub const DCYSUSV_PHASE1_MASK: u64 = 0x00008000;
pub const DCYSUSV_SUSTAINLEVEL_MASK: u64 = 0x00007f00;
pub const DCYSUSV_CHANNELENABLE_MASK: u64 = 0x00000080;
// C declaration: 						/* this channel and from writing to pitch, filter and	*/
// C declaration: 						/* volume targets.					*/
pub const DCYSUSV_DECAYTIME_MASK: u64 = 0x0000007f;
// C declaration: 						/* 0 = 43.7msec, 1 = 21.8msec, 0x7f = 22msec		*/

pub const LFOVAL1: u64 = 0x13;
pub const LFOVAL_MASK: u64 = 0x0000ffff;
// C declaration: 						/* 0x8000-n == 666*n usec delay				*/

pub const ENVVAL: u64 = 0x14;
pub const ENVVAL_MASK: u64 = 0x0000ffff;
// C declaration: 						/* 0x8000-n == 666*n usec delay				*/

pub const ATKHLDM: u64 = 0x15;
pub const ATKHLDM_PHASE0_MASK: u64 = 0x00008000;
pub const ATKHLDM_HOLDTIME: u64 = 0x00007f00;
pub const ATKHLDM_ATTACKTIME: u64 = 0x0000007f;
// C declaration: 						/* 0 = infinite, 1 = 11msec, ... 0x7f = 5.5msec		*/

pub const DCYSUSM: u64 = 0x16;
pub const DCYSUSM_PHASE1_MASK: u64 = 0x00008000;
pub const DCYSUSM_SUSTAINLEVEL_MASK: u64 = 0x00007f00;
pub const DCYSUSM_DECAYTIME_MASK: u64 = 0x0000007f;
// C declaration: 						/* 0 = 43.7msec, 1 = 21.8msec, 0x7f = 22msec		*/

pub const LFOVAL2: u64 = 0x17;
pub const LFOVAL2_MASK: u64 = 0x0000ffff;
// C declaration: 						/* 0x8000-n == 666*n usec delay				*/

pub const IP: u64 = 0x18;
pub const IP_MASK: u64 = 0x0000ffff;
// C declaration: 						/* 4 bits of octave, 12 bits of fractional octave	*/
pub const IP_UNITY: u64 = 0x0000e000;

pub const IFATN: u64 = 0x19;
pub const FILTERCUTOFF_MASK: u32 = 0x0000ff00;
pub const FILTERCUTOFF: u32 = IFATN | ((0x0000ff00.trailing_zeros() as u32) << 16) | ((0x0000ff00.count_ones() as u32) << 24);
// C declaration: 						/* 6 most significant bits are semitones		*/
// C declaration: 						/* 2 least significant bits are fractions		*/
pub const ATTENUATION_MASK: u32 = 0x000000ff;
pub const ATTENUATION: u32 = IFATN | ((0x000000ff.trailing_zeros() as u32) << 16) | ((0x000000ff.count_ones() as u32) << 24);

pub const PEFE: u64 = 0x1a;
pub const PITCHAMOUNT_MASK: u32 = 0x0000ff00;
pub const PITCHAMOUNT: u32 = PEFE | ((0x0000ff00.trailing_zeros() as u32) << 16) | ((0x0000ff00.count_ones() as u32) << 24);
// C declaration: 						/* Signed 2's complement, +/- one octave peak extremes	*/
pub const FILTERAMOUNT_MASK: u32 = 0x000000ff;
pub const FILTERAMOUNT: u32 = PEFE | ((0x000000ff.trailing_zeros() as u32) << 16) | ((0x000000ff.count_ones() as u32) << 24);
// C declaration: 						/* Signed 2's complement, +/- six octaves peak extremes */


pub const FMMOD: u64 = 0x1b;
pub const FMMOD_MODVIBRATO: u64 = 0x0000ff00;
// C declaration: 						/* Signed 2's complement, +/- one octave extremes	*/
pub const FMMOD_MOFILTER: u64 = 0x000000ff;
// C declaration: 						/* Signed 2's complement, +/- three octave extremes	*/

pub const TREMFRQ: u64 = 0x1c;
pub const TREMFRQ_DEPTH: u64 = 0x0000ff00;
// C declaration: 						/* Signed 2's complement, with +/- 12dB extremes	*/
pub const TREMFRQ_FREQUENCY: u64 = 0x000000ff;
// C declaration: 						/* ??Hz steps, maximum of ?? Hz.			*/

pub const FM2FRQ2: u64 = 0x1d;
pub const FM2FRQ2_DEPTH: u64 = 0x0000ff00;
// C declaration: 						/* Signed 2's complement, +/- one octave extremes	*/
pub const FM2FRQ2_FREQUENCY: u64 = 0x000000ff;
// C declaration: 						/* 0.039Hz steps, maximum of 9.85 Hz.			*/

pub const TEMPENV: u64 = 0x1e;
pub const TEMPENV_MASK: u64 = 0x0000ffff;
// C declaration: 						/* NOTE: All channels contain internal variables; do	*/
// C declaration: 						/* not write to these locations.			*/

/* 0x1f: not used */

// 32 cache registers (== 128 bytes) per channel follow.
// In stereo mode, the two channels' caches are concatenated into one,
// and hold the interleaved frames.
// The cache holds 64 frames, so the upper half is not used in 8-bit mode.
// All registers mentioned below count in frames. Shortcuts:
//   CA = CCCA_CURRADDR, CRA = CCR_READADDRESS,
//   CLA = CCR_CACHELOOPADDRHI:CLP_CACHELOOPADDR,
//   CIS = CCR_CACHEINVALIDSIZE, LIS = CCR_LOOPINVALSIZE,
//   CLF = CCR_CACHELOOPFLAG, LF = CCR_LOOPFLAG
// The cache is a ring buffer; CRA operates modulo 64.
// The cache is filled from (CA - CIS) into (CRA - CIS).
// The engine has a fetch threshold of 32 bytes, so it tries to keep
// CIS below 8 (16-bit stereo), 16 (16-bit mono, 8-bit stereo), or
// 32 (8-bit mono). The actual transfers are pretty unpredictable,
// especially if several voices are running.
// Frames are consumed at CRA, which is incremented afterwards,
// along with CA and CIS. This implies that the actual playback
// position always lags CA by exactly 64 frames.
// When CA reaches DSL_LOOPENDADDR, LF is set for one frame's time.
// LF's rising edge causes the current values of CA and CIS to be
// copied into CLA and LIS, resp., and CLF to be set.
// If CLF is set, the first LIS of the CIS frames are instead
// filled from (CLA - LIS), and CLF is subsequently reset.
pub const CD0: u64 = 0x20;

pub const PTB: u64 = 0x40;
pub const PTB_MASK: u64 = 0xfffff000;

pub const TCB: u64 = 0x41;
pub const TCB_MASK: u64 = 0xfffff000;

pub const ADCCR: u64 = 0x42;
pub const ADCCR_RCHANENABLE: u64 = 0x00000010;
pub const ADCCR_LCHANENABLE: u64 = 0x00000008;
// C declaration: 						/* NOTE: To guarantee phase coherency, both channels	*/
// C declaration: 						/* must be disabled prior to enabling both channels.	*/
pub const A_ADCCR_RCHANENABLE: u64 = 0x00000020;
pub const A_ADCCR_LCHANENABLE: u64 = 0x00000010;

pub const A_ADCCR_SAMPLERATE_MASK: u64 = 0x0000000F;
pub const ADCCR_SAMPLERATE_MASK: u64 = 0x00000007;
pub const ADCCR_SAMPLERATE_48: u64 = 0x00000000;
pub const ADCCR_SAMPLERATE_44: u64 = 0x00000001;
pub const ADCCR_SAMPLERATE_32: u64 = 0x00000002;
pub const ADCCR_SAMPLERATE_24: u64 = 0x00000003;
pub const ADCCR_SAMPLERATE_22: u64 = 0x00000004;
pub const ADCCR_SAMPLERATE_16: u64 = 0x00000005;
pub const ADCCR_SAMPLERATE_11: u64 = 0x00000006;
pub const ADCCR_SAMPLERATE_8: u64 = 0x00000007;
pub const A_ADCCR_SAMPLERATE_12: u64 = 0x00000006;
pub const A_ADCCR_SAMPLERATE_11: u64 = 0x00000007;
pub const A_ADCCR_SAMPLERATE_8: u64 = 0x00000008;

pub const FXWC: u64 = 0x43;
// C declaration: 						/* When set, each bit enables the writing of the	*/
// C declaration: 						/* corresponding FX output channel (internal registers  */
// C declaration: 						/* 0x20-0x3f) to host memory.  This mode of recording   */
// C declaration: 						/* is 16bit, 48KHz only. All 32 channels can be enabled */
// C declaration: 						/* simultaneously.					*/

pub const A_TBLSZ: u64 = 0x43;

pub const TCBS: u64 = 0x44;
pub const TCBS_MASK: u64 = 0x00000007;
pub const TCBS_BUFFSIZE_16K: u64 = 0x00000000;
pub const TCBS_BUFFSIZE_32K: u64 = 0x00000001;
pub const TCBS_BUFFSIZE_64K: u64 = 0x00000002;
pub const TCBS_BUFFSIZE_128K: u64 = 0x00000003;
pub const TCBS_BUFFSIZE_256K: u64 = 0x00000004;
pub const TCBS_BUFFSIZE_512K: u64 = 0x00000005;
pub const TCBS_BUFFSIZE_1024K: u64 = 0x00000006;
pub const TCBS_BUFFSIZE_2048K: u64 = 0x00000007;

pub const MICBA: u64 = 0x45;
pub const MICBA_MASK: u64 = 0xfffff000;

pub const ADCBA: u64 = 0x46;
pub const ADCBA_MASK: u64 = 0xfffff000;

pub const FXBA: u64 = 0x47;
pub const FXBA_MASK: u64 = 0xfffff000;

pub const A_HWM: u64 = 0x48;

pub const MICBS: u64 = 0x49;

pub const ADCBS: u64 = 0x4a;

pub const FXBS: u64 = 0x4b;

/* The following mask values define the size of the ADC, MIC and FX buffers in bytes */
pub const ADCBS_BUFSIZE_NONE: u64 = 0x00000000;
pub const ADCBS_BUFSIZE_384: u64 = 0x00000001;
pub const ADCBS_BUFSIZE_448: u64 = 0x00000002;
pub const ADCBS_BUFSIZE_512: u64 = 0x00000003;
pub const ADCBS_BUFSIZE_640: u64 = 0x00000004;
pub const ADCBS_BUFSIZE_768: u64 = 0x00000005;
pub const ADCBS_BUFSIZE_896: u64 = 0x00000006;
pub const ADCBS_BUFSIZE_1024: u64 = 0x00000007;
pub const ADCBS_BUFSIZE_1280: u64 = 0x00000008;
pub const ADCBS_BUFSIZE_1536: u64 = 0x00000009;
pub const ADCBS_BUFSIZE_1792: u64 = 0x0000000a;
pub const ADCBS_BUFSIZE_2048: u64 = 0x0000000b;
pub const ADCBS_BUFSIZE_2560: u64 = 0x0000000c;
pub const ADCBS_BUFSIZE_3072: u64 = 0x0000000d;
pub const ADCBS_BUFSIZE_3584: u64 = 0x0000000e;
pub const ADCBS_BUFSIZE_4096: u64 = 0x0000000f;
pub const ADCBS_BUFSIZE_5120: u64 = 0x00000010;
pub const ADCBS_BUFSIZE_6144: u64 = 0x00000011;
pub const ADCBS_BUFSIZE_7168: u64 = 0x00000012;
pub const ADCBS_BUFSIZE_8192: u64 = 0x00000013;
pub const ADCBS_BUFSIZE_10240: u64 = 0x00000014;
pub const ADCBS_BUFSIZE_12288: u64 = 0x00000015;
pub const ADCBS_BUFSIZE_14366: u64 = 0x00000016;
pub const ADCBS_BUFSIZE_16384: u64 = 0x00000017;
pub const ADCBS_BUFSIZE_20480: u64 = 0x00000018;
pub const ADCBS_BUFSIZE_24576: u64 = 0x00000019;
pub const ADCBS_BUFSIZE_28672: u64 = 0x0000001a;
pub const ADCBS_BUFSIZE_32768: u64 = 0x0000001b;
pub const ADCBS_BUFSIZE_40960: u64 = 0x0000001c;
pub const ADCBS_BUFSIZE_49152: u64 = 0x0000001d;
pub const ADCBS_BUFSIZE_57344: u64 = 0x0000001e;
pub const ADCBS_BUFSIZE_65536: u64 = 0x0000001f;

// On Audigy, the FX send amounts are not applied instantly, but determine
// targets towards which the following registers swerve gradually.
pub const A_CSBA: u64 = 0x4c;
pub const A_CSDC: u64 = 0x4d;
pub const A_CSFE: u64 = 0x4e;
pub const A_CSHG: u64 = 0x4f;

// NOTE: 0x50,51,52: 64-bit (split over voices 0 & 1)
pub const CDCS: u64 = 0x50;

pub const GPSCS: u64 = 0x51;

// Corresponding EMU10K1_DBG_* constants are in the public header
pub const DBG: u64 = 0x52;

pub const A_SPSC: u64 = 0x52;

pub const REG53: u64 = 0x53;

// Corresponding A_DBG_* constants are in the public header
pub const A_DBG: u64 = 0x53;

// NOTE: 0x54,55,56: 64-bit (split over voices 0 & 1)
pub const SPCS0: u64 = 0x54;

pub const SPCS1: u64 = 0x55;

pub const SPCS2: u64 = 0x56;

pub const SPCS_CLKACCYMASK: u64 = 0x30000000;
pub const SPCS_CLKACCY_1000PPM: u64 = 0x00000000;
pub const SPCS_CLKACCY_50PPM: u64 = 0x10000000;
pub const SPCS_CLKACCY_VARIABLE: u64 = 0x20000000;
pub const SPCS_SAMPLERATEMASK: u64 = 0x0f000000;
pub const SPCS_SAMPLERATE_44: u64 = 0x00000000;
pub const SPCS_SAMPLERATE_48: u64 = 0x02000000;
pub const SPCS_SAMPLERATE_32: u64 = 0x03000000;
pub const SPCS_CHANNELNUMMASK: u64 = 0x00f00000;
pub const SPCS_CHANNELNUM_UNSPEC: u64 = 0x00000000;
pub const SPCS_CHANNELNUM_LEFT: u64 = 0x00100000;
pub const SPCS_CHANNELNUM_RIGHT: u64 = 0x00200000;
pub const SPCS_SOURCENUMMASK: u64 = 0x000f0000;
pub const SPCS_SOURCENUM_UNSPEC: u64 = 0x00000000;
pub const SPCS_GENERATIONSTATUS: u64 = 0x00008000;
pub const SPCS_CATEGORYCODEMASK: u64 = 0x00007f00;
pub const SPCS_MODEMASK: u64 = 0x000000c0;
pub const SPCS_EMPHASISMASK: u64 = 0x00000038;
pub const SPCS_EMPHASIS_NONE: u64 = 0x00000000;
pub const SPCS_EMPHASIS_50_15: u64 = 0x00000008;
pub const SPCS_COPYRIGHT: u64 = 0x00000004;
pub const SPCS_NOTAUDIODATA: u64 = 0x00000002;
pub const SPCS_PROFESSIONAL: u64 = 0x00000001;

/* 0x57: Not used */

/* The 32-bit CLIx and SOLEx registers all have one bit per channel control/status      	*/
pub const CLIEL: u64 = 0x58;
pub const CLIEH: u64 = 0x59;

pub const CLIPL: u64 = 0x5a;
pub const CLIPH: u64 = 0x5b;

// These cause CPF_STOP_MASK to be set shortly after CCCA_CURRADDR passes DSL_LOOPENDADDR.
// Subsequent changes to the address registers don't resume; clearing the bit here or in CPF does.
// The registers are NOT synchronized; the next serviced channel picks up immediately.
pub const SOLEL: u64 = 0x5c;
pub const SOLEH: u64 = 0x5d;

pub const SPBYPASS: u64 = 0x5e;
pub const SPBYPASS_SPDIF0_MASK: u64 = 0x00000003;
pub const SPBYPASS_SPDIF1_MASK: u64 = 0x0000000c;
/* bypass mode: 0 - DSP; 1 - SPDIF A, 2 - SPDIF B, 3 - SPDIF C					*/
pub const SPBYPASS_FORMAT: u64 = 0x00000f00;

pub const AC97SLOT: u64 = 0x5f;
pub const AC97SLOT_REAR_RIGHT: u64 = 0x01;
pub const AC97SLOT_REAR_LEFT: u64 = 0x02;
pub const AC97SLOT_CNTR: u64 = 0x10;
pub const AC97SLOT_LFE: u64 = 0x20;

pub const A_PCB: u64 = 0x5f;

// NOTE: 0x60,61,62: 64-bit
pub const CDSRCS: u64 = 0x60;

pub const GPSRCS: u64 = 0x61;

pub const ZVSRCS: u64 = 0x62;
// C declaration: 						/* NOTE: This one has no SPDIFLOCKED field	*/
// C declaration: 						/* Assumes sample lock				*/

/* These three bitfields apply to CDSRCS, GPSRCS, and (except as noted) ZVSRCS.			*/
pub const SRCS_SPDIFVALID: u64 = 0x04000000;
pub const SRCS_SPDIFLOCKED: u64 = 0x02000000;
pub const SRCS_RATELOCKED: u64 = 0x01000000;
pub const SRCS_ESTSAMPLERATE: u64 = 0x0007ffff;

/* Note that these values can vary +/- by a small amount                                        */
pub const SRCS_SPDIFRATE_44: u64 = 0x0003acd9;
pub const SRCS_SPDIFRATE_48: u64 = 0x00040000;
pub const SRCS_SPDIFRATE_96: u64 = 0x00080000;

pub const MICIDX: u64 = 0x63;
pub const IDX_MASK: u32 = 0x0000ffff;
pub const IDX: u32 = MICIDX | ((0x0000ffff.trailing_zeros() as u32) << 16) | ((0x0000ffff.count_ones() as u32) << 24);

pub const ADCIDX: u64 = 0x64;
pub const IDX_MASK: u32 = 0x0000ffff;
pub const IDX: u32 = ADCIDX | ((0x0000ffff.trailing_zeros() as u32) << 16) | ((0x0000ffff.count_ones() as u32) << 24);

pub const A_ADCIDX: u64 = 0x63;
pub const IDX_MASK: u32 = 0x0000ffff;
pub const IDX: u32 = A_ADCIDX | ((0x0000ffff.trailing_zeros() as u32) << 16) | ((0x0000ffff.count_ones() as u32) << 24);

pub const A_MICIDX: u64 = 0x64;
pub const IDX_MASK: u32 = 0x0000ffff;
pub const IDX: u32 = A_MICIDX | ((0x0000ffff.trailing_zeros() as u32) << 16) | ((0x0000ffff.count_ones() as u32) << 24);

pub const FXIDX: u64 = 0x65;
pub const IDX_MASK: u32 = 0x0000ffff;
pub const IDX: u32 = FXIDX | ((0x0000ffff.trailing_zeros() as u32) << 16) | ((0x0000ffff.count_ones() as u32) << 24);

/* The 32-bit HLIEx and HLIPx registers all have one bit per channel control/status      		*/
pub const HLIEL: u64 = 0x66;
pub const HLIEH: u64 = 0x67;

pub const HLIPL: u64 = 0x68;
pub const HLIPH: u64 = 0x69;

pub const A_SPRI: u64 = 0x6a;
pub const A_SPRA: u64 = 0x6b;
pub const A_SPRC: u64 = 0x6c;

pub const A_DICE: u64 = 0x6d;

pub const A_TTB: u64 = 0x6e;
pub const A_TDOF: u64 = 0x6f;

/* This is the MPU port on the card (via the game port)						*/
pub const A_MUDATA1: u64 = 0x70;
pub const A_MUCMD1: u64 = 0x71;
pub const A_MUSTAT1: u64 = A_MUCMD1;

/* This is the MPU port on the Audigy Drive 							*/
pub const A_MUDATA2: u64 = 0x72;
pub const A_MUCMD2: u64 = 0x73;
pub const A_MUSTAT2: u64 = A_MUCMD2	;

/* The next two are the Audigy equivalent of FXWC						*/
/* the Audigy can record any output (16bit, 48kHz, up to 64 channels simultaneously) 		*/
/* Each bit selects a channel for recording */
pub const A_FXWC1: u64 = 0x74;
pub const A_FXWC2: u64 = 0x75;

pub const A_EHC: u64 = 0x76;

pub const A_SPDIF_SAMPLERATE: u64 = A_EHC;
pub const A_SPDIF_RATE_MASK: u64 = 0x000000e0;
pub const A_SPDIF_48000: u64 = 0x00000000;
pub const A_SPDIF_192000: u64 = 0x00000020;
pub const A_SPDIF_96000: u64 = 0x00000040;
pub const A_SPDIF_44100: u64 = 0x00000080;
pub const A_SPDIF_MUTED: u64 = 0x000000c0;

pub const A_I2S_CAPTURE_RATE_MASK: u32 = 0x00000e00;
pub const A_I2S_CAPTURE_RATE: u32 = A_EHC | ((0x00000e00.trailing_zeros() as u32) << 16) | ((0x00000e00.count_ones() as u32) << 24);
// C declaration: 						   /* unclear if this sets the ADC rate as well. */
pub const A_I2S_CAPTURE_48000: u64 = 0x0;
pub const A_I2S_CAPTURE_192000: u64 = 0x1;
pub const A_I2S_CAPTURE_96000: u64 = 0x2;
pub const A_I2S_CAPTURE_44100: u64 = 0x4;

pub const A_EHC_SRC48_MASK: u64 = 0x0000e000;
pub const A_EHC_SRC48_BYPASS: u64 = 0x00000000;
pub const A_EHC_SRC48_192: u64 = 0x00002000;
pub const A_EHC_SRC48_96: u64 = 0x00004000;
pub const A_EHC_SRC48_44: u64 = 0x00008000;
pub const A_EHC_SRC48_MUTED: u64 = 0x0000c000;

pub const A_EHC_P17V_TVM: u64 = 0x00000001;
pub const A_EHC_P17V_SEL0_MASK: u64 = 0x00030000;
pub const A_EHC_P17V_SEL1_MASK: u64 = 0x000c0000;
pub const A_EHC_P17V_SEL2_MASK: u64 = 0x00300000;
pub const A_EHC_P17V_SEL3_MASK: u64 = 0x00c00000;

pub const A_EHC_ASYNC_BYPASS: u64 = 0x80000000;

pub const A_SRT3: u64 = 0x77;
pub const A_SRT4: u64 = 0x78;
pub const A_SRT5: u64 = 0x79;
/* - default to 0x01080000 on my audigy 2 ZS --rlrevell	*/

pub const A_SRT_ESTSAMPLERATE: u64 = 0x001fffff;
pub const A_SRT_RATELOCKED: u64 = 0x01000000;

pub const A_TTDA: u64 = 0x7a;
pub const A_TTDD: u64 = 0x7b;

// In A_FXRT1 & A_FXRT2, the 0x80 bit of each byte completely disables the
// filter (CVCF_CURRENTFILTER) for the corresponding channel. There is no
// effect on the volume (CVCF_CURRENTVOLUME) or the interpolator's filter
// (CCCA_INTERPROM_MASK).

pub const A_FXRT2: u64 = 0x7c;
pub const A_FXRT_CHANNELE: u64 = 0x0000003f;
pub const A_FXRT_CHANNELF: u64 = 0x00003f00;
pub const A_FXRT_CHANNELG: u64 = 0x003f0000;
pub const A_FXRT_CHANNELH: u64 = 0x3f000000;

pub const A_SENDAMOUNTS: u64 = 0x7d;
pub const A_FXSENDAMOUNT_E_MASK: u64 = 0xFF000000;
pub const A_FXSENDAMOUNT_F_MASK: u64 = 0x00FF0000;
pub const A_FXSENDAMOUNT_G_MASK: u64 = 0x0000FF00;
pub const A_FXSENDAMOUNT_H_MASK: u64 = 0x000000FF;

/* The send amounts for this one are the same as used with the emu10k1 */
pub const A_FXRT1: u64 = 0x7e;
pub const A_FXRT_CHANNELA: u64 = 0x0000003f;
pub const A_FXRT_CHANNELB: u64 = 0x00003f00;
pub const A_FXRT_CHANNELC: u64 = 0x003f0000;
pub const A_FXRT_CHANNELD: u64 = 0x3f000000;

/* 0x7f: Not used */

/* The public header defines the GPR and TRAM base addresses that
 * are valid for _both_ CPU and DSP addressing. */

/* Each DSP microcode instruction is mapped into 2 doublewords 					*/
/* NOTE: When writing, always write the LO doubleword first.  Reads can be in either order.	*/
pub const MICROCODEBASE: u64 = 0x400;
pub const A_MICROCODEBASE: u64 = 0x600;


/************************************************************************************************/
/* E-MU Digital Audio System overview								*/
/************************************************************************************************/

// - These cards use a regular PCI-attached Audigy chip (Alice2/Tina/Tina2);
//   the PCIe variants simply put the Audigy chip behind a PCI bridge.
// - All physical PCM I/O is routed through an additional FPGA; the regular
//   EXTIN/EXTOUT ports are unconnected.
// - The FPGA has a signal routing matrix, to connect each destination (output
//   socket or capture channel) to a source (input socket or playback channel).
// - The FPGA is controlled via Audigy's GPIO port, while sample data is
//   transmitted via proprietary EMU32 serial links. On first-generation
//   E-MU 1010 cards, Audigy's I2S inputs are also used for sample data.
// - The Audio/Micro Dock is attached to Hana via EDI, a "network" link.
// - The Audigy chip operates in slave mode; the clock is supplied by the FPGA.
//   Gen1 E-MU 1010 cards have two crystals (for 44.1 kHz and 48 kHz multiples),
//   while the later cards use a single crystal and a PLL chip.
// - The whole card is switched to 2x/4x mode to achieve 88.2/96/176.4/192 kHz
//   sample rates. Alice2/Tina keeps running at 44.1/48 kHz, but multiple channels
//   are bundled.
// - The number of available EMU32/EDI channels is hit in 2x/4x mode, so the total
//   number of usable inputs/outputs is limited, esp. with ADAT in use.
// - S/PDIF is unavailable in 4x mode (only over TOSLINK on newer 1010 cards) due
//   to being unspecified at 176.4/192 kHz. Therefore, the Dock's S/PDIF channels
//   can overlap with the Dock's ADC/DAC's high channels.
// - The code names are mentioned below and in the emu_chip_details table.

/************************************************************************************************/
/* EMU1010 FPGA registers									*/
/************************************************************************************************/

pub const EMU_HANA_DESTHI: u64 = 0x00;
pub const EMU_HANA_DESTLO: u64 = 0x01;

pub const EMU_HANA_SRCHI: u64 = 0x02;
pub const EMU_HANA_SRCLO: u64 = 0x03;

pub const EMU_HANA_DOCK_PWR: u64 = 0x04;
pub const EMU_HANA_DOCK_PWR_ON: u64 = 0x01;

pub const EMU_HANA_WCLOCK: u64 = 0x05;
// C declaration: 					/* Must be written after power on to reset DLL */
// C declaration: 					/* One is unable to detect the Audio dock without this */
pub const EMU_HANA_WCLOCK_SRC_MASK: u64 = 0x07;
pub const EMU_HANA_WCLOCK_INT_48K: u64 = 0x00;
pub const EMU_HANA_WCLOCK_INT_44_1K: u64 = 0x01;
pub const EMU_HANA_WCLOCK_HANA_SPDIF_IN: u64 = 0x02;
pub const EMU_HANA_WCLOCK_HANA_ADAT_IN: u64 = 0x03;
pub const EMU_HANA_WCLOCK_SYNC_BNC: u64 = 0x04;
pub const EMU_HANA_WCLOCK_2ND_HANA: u64 = 0x05;
pub const EMU_HANA_WCLOCK_SRC_RESERVED: u64 = 0x06;
pub const EMU_HANA_WCLOCK_OFF: u64 = 0x07;
pub const EMU_HANA_WCLOCK_MULT_MASK: u64 = 0x18;
pub const EMU_HANA_WCLOCK_1X: u64 = 0x00;
pub const EMU_HANA_WCLOCK_2X: u64 = 0x08;
pub const EMU_HANA_WCLOCK_4X: u64 = 0x10;
pub const EMU_HANA_WCLOCK_MULT_RESERVED: u64 = 0x18;

// If the selected external clock source is/becomes invalid or incompatible
// with the clock multiplier, the clock source is reset to this value, and
// a WCLK_CHANGED interrupt is raised.
pub const EMU_HANA_DEFCLOCK: u64 = 0x06;
pub const EMU_HANA_DEFCLOCK_48K: u64 = 0x00;
pub const EMU_HANA_DEFCLOCK_44_1K: u64 = 0x01;

pub const EMU_HANA_UNMUTE: u64 = 0x07;
pub const EMU_MUTE: u64 = 0x00;
pub const EMU_UNMUTE: u64 = 0x01;

pub const EMU_HANA_FPGA_CONFIG: u64 = 0x08;
pub const EMU_HANA_FPGA_CONFIG_AUDIODOCK: u64 = 0x01;
pub const EMU_HANA_FPGA_CONFIG_HANA: u64 = 0x02;

pub const EMU_HANA_IRQ_ENABLE: u64 = 0x09;
pub const EMU_HANA_IRQ_WCLK_CHANGED: u64 = 0x01;
pub const EMU_HANA_IRQ_ADAT: u64 = 0x02;
pub const EMU_HANA_IRQ_DOCK: u64 = 0x04;
pub const EMU_HANA_IRQ_DOCK_LOST: u64 = 0x08;

pub const EMU_HANA_SPDIF_MODE: u64 = 0x0a;
pub const EMU_HANA_SPDIF_MODE_TX_CONSUMER: u64 = 0x00;
pub const EMU_HANA_SPDIF_MODE_TX_PRO: u64 = 0x01;
pub const EMU_HANA_SPDIF_MODE_TX_NOCOPY: u64 = 0x02;
pub const EMU_HANA_SPDIF_MODE_RX_CONSUMER: u64 = 0x00;
pub const EMU_HANA_SPDIF_MODE_RX_PRO: u64 = 0x04;
pub const EMU_HANA_SPDIF_MODE_RX_NOCOPY: u64 = 0x08;
pub const EMU_HANA_SPDIF_MODE_RX_INVALID: u64 = 0x10;

pub const EMU_HANA_OPTICAL_TYPE: u64 = 0x0b;
pub const EMU_HANA_OPTICAL_IN_SPDIF: u64 = 0x00;
pub const EMU_HANA_OPTICAL_IN_ADAT: u64 = 0x01;
pub const EMU_HANA_OPTICAL_OUT_SPDIF: u64 = 0x00;
pub const EMU_HANA_OPTICAL_OUT_ADAT: u64 = 0x02;

pub const EMU_HANA_MIDI_IN: u64 = 0x0c;
pub const EMU_HANA_MIDI_INA_FROM_HAMOA: u64 = 0x01;
pub const EMU_HANA_MIDI_INA_FROM_DOCK1: u64 = 0x02;
pub const EMU_HANA_MIDI_INA_FROM_DOCK2: u64 = 0x03;
pub const EMU_HANA_MIDI_INB_FROM_HAMOA: u64 = 0x08;
pub const EMU_HANA_MIDI_INB_FROM_DOCK1: u64 = 0x10;
pub const EMU_HANA_MIDI_INB_FROM_DOCK2: u64 = 0x18;

pub const EMU_HANA_DOCK_LEDS_1: u64 = 0x0d;
pub const EMU_HANA_DOCK_LEDS_1_MIDI1: u64 = 0x01;
pub const EMU_HANA_DOCK_LEDS_1_MIDI2: u64 = 0x02;
pub const EMU_HANA_DOCK_LEDS_1_SMPTE_IN: u64 = 0x04;
pub const EMU_HANA_DOCK_LEDS_1_SMPTE_OUT: u64 = 0x08;

pub const EMU_HANA_DOCK_LEDS_2: u64 = 0x0e;
pub const EMU_HANA_DOCK_LEDS_2_44K: u64 = 0x01;
pub const EMU_HANA_DOCK_LEDS_2_48K: u64 = 0x02;
pub const EMU_HANA_DOCK_LEDS_2_96K: u64 = 0x04;
pub const EMU_HANA_DOCK_LEDS_2_192K: u64 = 0x08;
pub const EMU_HANA_DOCK_LEDS_2_LOCK: u64 = 0x10;
pub const EMU_HANA_DOCK_LEDS_2_EXT: u64 = 0x20;

pub const EMU_HANA_DOCK_LEDS_3: u64 = 0x0f;
pub const EMU_HANA_DOCK_LEDS_3_CLIP_A: u64 = 0x01;
pub const EMU_HANA_DOCK_LEDS_3_CLIP_B: u64 = 0x02;
pub const EMU_HANA_DOCK_LEDS_3_SIGNAL_A: u64 = 0x04;
pub const EMU_HANA_DOCK_LEDS_3_SIGNAL_B: u64 = 0x08;
pub const EMU_HANA_DOCK_LEDS_3_MANUAL_CLIP: u64 = 0x10;
pub const EMU_HANA_DOCK_LEDS_3_MANUAL_SIGNAL: u64 = 0x20;

pub const EMU_HANA_ADC_PADS: u64 = 0x10;
pub const EMU_HANA_DOCK_ADC_PAD1: u64 = 0x01;
pub const EMU_HANA_DOCK_ADC_PAD2: u64 = 0x02;
pub const EMU_HANA_DOCK_ADC_PAD3: u64 = 0x04;
pub const EMU_HANA_0202_ADC_PAD1: u64 = 0x08;

pub const EMU_HANA_DOCK_MISC: u64 = 0x11;
pub const EMU_HANA_DOCK_DAC1_MUTE: u64 = 0x01;
pub const EMU_HANA_DOCK_DAC2_MUTE: u64 = 0x02;
pub const EMU_HANA_DOCK_DAC3_MUTE: u64 = 0x04;
pub const EMU_HANA_DOCK_DAC4_MUTE: u64 = 0x08;
pub const EMU_HANA_DOCK_PHONES_192_DAC1: u64 = 0x00;
pub const EMU_HANA_DOCK_PHONES_192_DAC2: u64 = 0x10;
pub const EMU_HANA_DOCK_PHONES_192_DAC3: u64 = 0x20;
pub const EMU_HANA_DOCK_PHONES_192_DAC4: u64 = 0x30;

pub const EMU_HANA_MIDI_OUT: u64 = 0x12;
pub const EMU_HANA_MIDI_OUT_0202: u64 = 0x01;
pub const EMU_HANA_MIDI_OUT_DOCK1: u64 = 0x02;
pub const EMU_HANA_MIDI_OUT_DOCK2: u64 = 0x04;
pub const EMU_HANA_MIDI_OUT_SYNC2: u64 = 0x08;
pub const EMU_HANA_MIDI_OUT_LOOP: u64 = 0x10;

pub const EMU_HANA_DAC_PADS: u64 = 0x13;
pub const EMU_HANA_DOCK_DAC_PAD1: u64 = 0x01;
pub const EMU_HANA_DOCK_DAC_PAD2: u64 = 0x02;
pub const EMU_HANA_DOCK_DAC_PAD3: u64 = 0x04;
pub const EMU_HANA_DOCK_DAC_PAD4: u64 = 0x08;
pub const EMU_HANA_0202_DAC_PAD1: u64 = 0x10;

/* 0x14 - 0x1f Unused R/W registers */

pub const EMU_HANA_IRQ_STATUS: u64 = 0x20;
// C declaration: 					/* Same bits as for EMU_HANA_IRQ_ENABLE */
// C declaration: 					/* Reading the register resets it. */

pub const EMU_HANA_OPTION_CARDS: u64 = 0x21;
pub const EMU_HANA_OPTION_HAMOA: u64 = 0x01;
pub const EMU_HANA_OPTION_SYNC: u64 = 0x02;
pub const EMU_HANA_OPTION_DOCK_ONLINE: u64 = 0x04;
pub const EMU_HANA_OPTION_DOCK_OFFLINE: u64 = 0x08;

pub const EMU_HANA_ID: u64 = 0x22;
// C declaration: 					/* 0010101  5 bits ID byte & 0x1f = 0x15 with Tina/2 */

pub const EMU_HANA_MAJOR_REV: u64 = 0x23;
pub const EMU_HANA_MINOR_REV: u64 = 0x24;

pub const EMU_DOCK_MAJOR_REV: u64 = 0x25;
pub const EMU_DOCK_MINOR_REV: u64 = 0x26;

pub const EMU_DOCK_BOARD_ID: u64 = 0x27;
pub const EMU_DOCK_BOARD_ID0: u64 = 0x00;
pub const EMU_DOCK_BOARD_ID1: u64 = 0x03;

// The actual code disagrees about the bit width of the registers -
// the formula used is freq = 0x1770000 / (((X_HI << 5) | X_LO) + 1)

pub const EMU_HANA_WC_SPDIF_HI: u64 = 0x28;
pub const EMU_HANA_WC_SPDIF_LO: u64 = 0x29;

pub const EMU_HANA_WC_ADAT_HI: u64 = 0x2a;
pub const EMU_HANA_WC_ADAT_LO: u64 = 0x2b;

pub const EMU_HANA_WC_BNC_LO: u64 = 0x2c;
pub const EMU_HANA_WC_BNC_HI: u64 = 0x2d;

pub const EMU_HANA2_WC_SPDIF_HI: u64 = 0x2e;
pub const EMU_HANA2_WC_SPDIF_LO: u64 = 0x2f;

/* 0x30 - 0x3f Unused Read only registers */

// The meaning of this is not clear; kX-project just calls it "lock" in some info-only code.
pub const EMU_HANA_LOCK_STS_LO: u64 = 0x38;
pub const EMU_HANA_LOCK_STS_HI: u64 = 0x39;

/************************************************************************************************/
/* EMU1010 Audio Destinations									*/
/************************************************************************************************/
/* Hana, original 1010,1212m,1820[m] using Alice2
 * 0x00, 0x00-0x0f: 16 EMU32 channels to Alice2
 * 0x01, 0x00-0x1f: 32 EDI channels to Audio Dock
 *       0x00: Dock DAC 1 Left
 *       0x04: Dock DAC 1 Right
 *       0x08: Dock DAC 2 Left
 *       0x0c: Dock DAC 2 Right
 *       0x10: Dock DAC 3 Left
 *       0x12: PHONES Left (n/a in 2x/4x mode; output mirrors DAC4 Left)
 *       0x14: Dock DAC 3 Right
 *       0x16: PHONES Right (n/a in 2x/4x mode; output mirrors DAC4 Right)
 *       0x18: Dock DAC 4 Left
 *       0x1a: S/PDIF Left
 *       0x1c: Dock DAC 4 Right
 *       0x1e: S/PDIF Right
 * 0x02, 0x00: Hana S/PDIF Left
 * 0x02, 0x01: Hana S/PDIF Right
 * 0x03, 0x00: Hamoa DAC Left
 * 0x03, 0x01: Hamoa DAC Right
 * 0x04, 0x00-0x07: Hana ADAT
 * 0x05, 0x00: I2S0 Left to Alice2
 * 0x05, 0x01: I2S0 Right to Alice2
 * 0x06, 0x00: I2S0 Left to Alice2
 * 0x06, 0x01: I2S0 Right to Alice2
 * 0x07, 0x00: I2S0 Left to Alice2
 * 0x07, 0x01: I2S0 Right to Alice2
 *
 * Hana2 never released, but used Tina
 * Not needed.
 *
 * Hana3, rev2 1010,1212m,1616[m] using Tina
 * 0x00, 0x00-0x0f: 16 EMU32A channels to Tina
 * 0x01, 0x00-0x1f: 32 EDI channels to Micro Dock
 *       0x00: Dock DAC 1 Left
 *       0x04: Dock DAC 1 Right
 *       0x08: Dock DAC 2 Left
 *       0x0c: Dock DAC 2 Right
 *       0x10: Dock DAC 3 Left
 *       0x12: Dock S/PDIF Left
 *       0x14: Dock DAC 3 Right
 *       0x16: Dock S/PDIF Right
 *       0x18-0x1f: Dock ADAT 0-7
 * 0x02, 0x00: Hana3 S/PDIF Left
 * 0x02, 0x01: Hana3 S/PDIF Right
 * 0x03, 0x00: Hamoa DAC Left
 * 0x03, 0x01: Hamoa DAC Right
 * 0x04, 0x00-0x07: Hana3 ADAT 0-7
 * 0x05, 0x00-0x0f: 16 EMU32B channels to Tina
 * 0x06-0x07: Not used
 *
 * HanaLite, rev1 0404 using Alice2
 * HanaLiteLite, rev2 0404 using Tina
 * 0x00, 0x00-0x0f: 16 EMU32 channels to Alice2/Tina
 * 0x01: Not used
 * 0x02, 0x00: S/PDIF Left
 * 0x02, 0x01: S/PDIF Right
 * 0x03, 0x00: DAC Left
 * 0x03, 0x01: DAC Right
 * 0x04-0x07: Not used
 *
 * Mana, Cardbus 1616 using Tina2
 * 0x00, 0x00-0x0f: 16 EMU32A channels to Tina2
 * 0x01, 0x00-0x1f: 32 EDI channels to Micro Dock
 *       (same as rev2 1010)
 * 0x02: Not used
 * 0x03, 0x00: Mana DAC Left
 * 0x03, 0x01: Mana DAC Right
 * 0x04, 0x00-0x0f: 16 EMU32B channels to Tina2
 * 0x05-0x07: Not used
 */

/* 32-bit destinations of signal in the Hana FPGA. Destinations are either
 * physical outputs of Hana, or outputs going to Alice2/Tina for capture -
 * 16 x EMU_DST_ALICE2_EMU32_X (2x on rev2 boards). Which data is fed into
 * a channel depends on the mixer control setting for each destination - see
 * the register arrays in emumixer.c.
 */
pub const EMU_DST_ALICE2_EMU32_0: u64 = 0x000f;
// C declaration: 					/* This channel is delayed by one sample. */
pub const EMU_DST_ALICE2_EMU32_1: u64 = 0x0000;
pub const EMU_DST_ALICE2_EMU32_2: u64 = 0x0001;
pub const EMU_DST_ALICE2_EMU32_3: u64 = 0x0002;
pub const EMU_DST_ALICE2_EMU32_4: u64 = 0x0003;
pub const EMU_DST_ALICE2_EMU32_5: u64 = 0x0004;
pub const EMU_DST_ALICE2_EMU32_6: u64 = 0x0005;
pub const EMU_DST_ALICE2_EMU32_7: u64 = 0x0006;
pub const EMU_DST_ALICE2_EMU32_8: u64 = 0x0007;
pub const EMU_DST_ALICE2_EMU32_9: u64 = 0x0008;
pub const EMU_DST_ALICE2_EMU32_A: u64 = 0x0009;
pub const EMU_DST_ALICE2_EMU32_B: u64 = 0x000a;
pub const EMU_DST_ALICE2_EMU32_C: u64 = 0x000b;
pub const EMU_DST_ALICE2_EMU32_D: u64 = 0x000c;
pub const EMU_DST_ALICE2_EMU32_E: u64 = 0x000d;
pub const EMU_DST_ALICE2_EMU32_F: u64 = 0x000e;
pub const EMU_DST_DOCK_DAC1_LEFT1: u64 = 0x0100;
pub const EMU_DST_DOCK_DAC1_LEFT2: u64 = 0x0101;
pub const EMU_DST_DOCK_DAC1_LEFT3: u64 = 0x0102;
pub const EMU_DST_DOCK_DAC1_LEFT4: u64 = 0x0103;
pub const EMU_DST_DOCK_DAC1_RIGHT1: u64 = 0x0104;
pub const EMU_DST_DOCK_DAC1_RIGHT2: u64 = 0x0105;
pub const EMU_DST_DOCK_DAC1_RIGHT3: u64 = 0x0106;
pub const EMU_DST_DOCK_DAC1_RIGHT4: u64 = 0x0107;
pub const EMU_DST_DOCK_DAC2_LEFT1: u64 = 0x0108;
pub const EMU_DST_DOCK_DAC2_LEFT2: u64 = 0x0109;
pub const EMU_DST_DOCK_DAC2_LEFT3: u64 = 0x010a;
pub const EMU_DST_DOCK_DAC2_LEFT4: u64 = 0x010b;
pub const EMU_DST_DOCK_DAC2_RIGHT1: u64 = 0x010c;
pub const EMU_DST_DOCK_DAC2_RIGHT2: u64 = 0x010d;
pub const EMU_DST_DOCK_DAC2_RIGHT3: u64 = 0x010e;
pub const EMU_DST_DOCK_DAC2_RIGHT4: u64 = 0x010f;
pub const EMU_DST_DOCK_DAC3_LEFT1: u64 = 0x0110;
pub const EMU_DST_DOCK_DAC3_LEFT2: u64 = 0x0111;
pub const EMU_DST_DOCK_DAC3_LEFT3: u64 = 0x0112;
pub const EMU_DST_DOCK_DAC3_LEFT4: u64 = 0x0113;
pub const EMU_DST_DOCK_PHONES_LEFT1: u64 = 0x0112;
pub const EMU_DST_DOCK_PHONES_LEFT2: u64 = 0x0113;
pub const EMU_DST_DOCK_DAC3_RIGHT1: u64 = 0x0114;
pub const EMU_DST_DOCK_DAC3_RIGHT2: u64 = 0x0115;
pub const EMU_DST_DOCK_DAC3_RIGHT3: u64 = 0x0116;
pub const EMU_DST_DOCK_DAC3_RIGHT4: u64 = 0x0117;
pub const EMU_DST_DOCK_PHONES_RIGHT1: u64 = 0x0116;
pub const EMU_DST_DOCK_PHONES_RIGHT2: u64 = 0x0117;
pub const EMU_DST_DOCK_DAC4_LEFT1: u64 = 0x0118;
pub const EMU_DST_DOCK_DAC4_LEFT2: u64 = 0x0119;
pub const EMU_DST_DOCK_DAC4_LEFT3: u64 = 0x011a;
pub const EMU_DST_DOCK_DAC4_LEFT4: u64 = 0x011b;
pub const EMU_DST_DOCK_SPDIF_LEFT1: u64 = 0x011a;
pub const EMU_DST_DOCK_SPDIF_LEFT2: u64 = 0x011b;
pub const EMU_DST_DOCK_DAC4_RIGHT1: u64 = 0x011c;
pub const EMU_DST_DOCK_DAC4_RIGHT2: u64 = 0x011d;
pub const EMU_DST_DOCK_DAC4_RIGHT3: u64 = 0x011e;
pub const EMU_DST_DOCK_DAC4_RIGHT4: u64 = 0x011f;
pub const EMU_DST_DOCK_SPDIF_RIGHT1: u64 = 0x011e;
pub const EMU_DST_DOCK_SPDIF_RIGHT2: u64 = 0x011f;
pub const EMU_DST_HANA_SPDIF_LEFT1: u64 = 0x0200;
pub const EMU_DST_HANA_SPDIF_LEFT2: u64 = 0x0202;
pub const EMU_DST_HANA_SPDIF_LEFT3: u64 = 0x0204;
pub const EMU_DST_HANA_SPDIF_LEFT4: u64 = 0x0206;
pub const EMU_DST_HANA_SPDIF_RIGHT1: u64 = 0x0201;
pub const EMU_DST_HANA_SPDIF_RIGHT2: u64 = 0x0203;
pub const EMU_DST_HANA_SPDIF_RIGHT3: u64 = 0x0205;
pub const EMU_DST_HANA_SPDIF_RIGHT4: u64 = 0x0207;
pub const EMU_DST_HAMOA_DAC_LEFT1: u64 = 0x0300;
pub const EMU_DST_HAMOA_DAC_LEFT2: u64 = 0x0302;
pub const EMU_DST_HAMOA_DAC_LEFT3: u64 = 0x0304;
pub const EMU_DST_HAMOA_DAC_LEFT4: u64 = 0x0306;
pub const EMU_DST_HAMOA_DAC_RIGHT1: u64 = 0x0301;
pub const EMU_DST_HAMOA_DAC_RIGHT2: u64 = 0x0303;
pub const EMU_DST_HAMOA_DAC_RIGHT3: u64 = 0x0305;
pub const EMU_DST_HAMOA_DAC_RIGHT4: u64 = 0x0307;
// In S/MUX mode, the samples of one channel are adjacent.
pub const EMU_DST_HANA_ADAT: u64 = 0x0400;
pub const EMU_DST_ALICE_I2S0_LEFT: u64 = 0x0500;
pub const EMU_DST_ALICE_I2S0_RIGHT: u64 = 0x0501;
pub const EMU_DST_ALICE_I2S1_LEFT: u64 = 0x0600;
pub const EMU_DST_ALICE_I2S1_RIGHT: u64 = 0x0601;
pub const EMU_DST_ALICE_I2S2_LEFT: u64 = 0x0700;
pub const EMU_DST_ALICE_I2S2_RIGHT: u64 = 0x0701;

/* Additional destinations for 1616(M)/Microdock */

pub const EMU_DST_MDOCK_SPDIF_LEFT1: u64 = 0x0112;
pub const EMU_DST_MDOCK_SPDIF_LEFT2: u64 = 0x0113;
pub const EMU_DST_MDOCK_SPDIF_RIGHT1: u64 = 0x0116;
pub const EMU_DST_MDOCK_SPDIF_RIGHT2: u64 = 0x0117;
pub const EMU_DST_MDOCK_ADAT: u64 = 0x0118;

pub const EMU_DST_MANA_DAC_LEFT: u64 = 0x0300;
pub const EMU_DST_MANA_DAC_RIGHT: u64 = 0x0301;

/************************************************************************************************/
/* EMU1010 Audio Sources									*/
/************************************************************************************************/
/* Hana, original 1010,1212m,1820[m] using Alice2
 * 0x00, 0x00-0x1f: Silence
 * 0x01, 0x00-0x1f: 32 EDI channels from Audio Dock
 *       0x00: Dock Mic A
 *       0x04: Dock Mic B
 *       0x08: Dock ADC 1 Left
 *       0x0c: Dock ADC 1 Right
 *       0x10: Dock ADC 2 Left
 *       0x14: Dock ADC 2 Right
 *       0x18: Dock ADC 3 Left
 *       0x1c: Dock ADC 3 Right
 * 0x02, 0x00: Hamoa ADC Left
 * 0x02, 0x01: Hamoa ADC Right
 * 0x03, 0x00-0x0f: 16 inputs from Alice2 Emu32A output
 * 0x03, 0x10-0x1f: 16 inputs from Alice2 Emu32B output
 * 0x04, 0x00-0x07: Hana ADAT
 * 0x05, 0x00: Hana S/PDIF Left
 * 0x05, 0x01: Hana S/PDIF Right
 * 0x06-0x07: Not used
 *
 * Hana2 never released, but used Tina
 * Not needed.
 *
 * Hana3, rev2 1010,1212m,1616[m] using Tina
 * 0x00, 0x00-0x1f: Silence
 * 0x01, 0x00-0x1f: 32 EDI channels from Micro Dock
 *       0x00: Dock Mic A
 *       0x04: Dock Mic B
 *       0x08: Dock ADC 1 Left
 *       0x0c: Dock ADC 1 Right
 *       0x10: Dock ADC 2 Left
 *       0x12: Dock S/PDIF Left
 *       0x14: Dock ADC 2 Right
 *       0x16: Dock S/PDIF Right
 *       0x18-0x1f: Dock ADAT 0-7
 * 0x02, 0x00: Hamoa ADC Left
 * 0x02, 0x01: Hamoa ADC Right
 * 0x03, 0x00-0x0f: 16 inputs from Tina Emu32A output
 * 0x03, 0x10-0x1f: 16 inputs from Tina Emu32B output
 * 0x04, 0x00-0x07: Hana3 ADAT
 * 0x05, 0x00: Hana3 S/PDIF Left
 * 0x05, 0x01: Hana3 S/PDIF Right
 * 0x06-0x07: Not used
 *
 * HanaLite, rev1 0404 using Alice2
 * HanaLiteLite, rev2 0404 using Tina
 * 0x00, 0x00-0x1f: Silence
 * 0x01: Not used
 * 0x02, 0x00: ADC Left
 * 0x02, 0x01: ADC Right
 * 0x03, 0x00-0x0f: 16 inputs from Alice2/Tina Emu32A output
 * 0x03, 0x10-0x1f: 16 inputs from Alice2/Tina Emu32B output
 * 0x04: Not used
 * 0x05, 0x00: S/PDIF Left
 * 0x05, 0x01: S/PDIF Right
 * 0x06-0x07: Not used
 *
 * Mana, Cardbus 1616 using Tina2
 * 0x00, 0x00-0x1f: Silence
 * 0x01, 0x00-0x1f: 32 EDI channels from Micro Dock
 *       (same as rev2 1010)
 * 0x02: Not used
 * 0x03, 0x00-0x0f: 16 inputs from Tina2 Emu32A output
 * 0x03, 0x10-0x1f: 16 inputs from Tina2 Emu32B output
 * 0x04-0x07: Not used
 */

/* 32-bit sources of signal in the Hana FPGA. The sources are routed to
 * destinations using a mixer control for each destination - see emumixer.c.
 * Sources are either physical inputs of Hana, or inputs from Alice2/Tina -
 * 16 x EMU_SRC_ALICE_EMU32A + 16 x EMU_SRC_ALICE_EMU32B.
 */
pub const EMU_SRC_SILENCE: u64 = 0x0000;
pub const EMU_SRC_DOCK_MIC_A1: u64 = 0x0100;
pub const EMU_SRC_DOCK_MIC_A2: u64 = 0x0101;
pub const EMU_SRC_DOCK_MIC_A3: u64 = 0x0102;
pub const EMU_SRC_DOCK_MIC_A4: u64 = 0x0103;
pub const EMU_SRC_DOCK_MIC_B1: u64 = 0x0104;
pub const EMU_SRC_DOCK_MIC_B2: u64 = 0x0105;
pub const EMU_SRC_DOCK_MIC_B3: u64 = 0x0106;
pub const EMU_SRC_DOCK_MIC_B4: u64 = 0x0107;
pub const EMU_SRC_DOCK_ADC1_LEFT1: u64 = 0x0108;
pub const EMU_SRC_DOCK_ADC1_LEFT2: u64 = 0x0109;
pub const EMU_SRC_DOCK_ADC1_LEFT3: u64 = 0x010a;
pub const EMU_SRC_DOCK_ADC1_LEFT4: u64 = 0x010b;
pub const EMU_SRC_DOCK_ADC1_RIGHT1: u64 = 0x010c;
pub const EMU_SRC_DOCK_ADC1_RIGHT2: u64 = 0x010d;
pub const EMU_SRC_DOCK_ADC1_RIGHT3: u64 = 0x010e;
pub const EMU_SRC_DOCK_ADC1_RIGHT4: u64 = 0x010f;
pub const EMU_SRC_DOCK_ADC2_LEFT1: u64 = 0x0110;
pub const EMU_SRC_DOCK_ADC2_LEFT2: u64 = 0x0111;
pub const EMU_SRC_DOCK_ADC2_LEFT3: u64 = 0x0112;
pub const EMU_SRC_DOCK_ADC2_LEFT4: u64 = 0x0113;
pub const EMU_SRC_DOCK_ADC2_RIGHT1: u64 = 0x0114;
pub const EMU_SRC_DOCK_ADC2_RIGHT2: u64 = 0x0115;
pub const EMU_SRC_DOCK_ADC2_RIGHT3: u64 = 0x0116;
pub const EMU_SRC_DOCK_ADC2_RIGHT4: u64 = 0x0117;
pub const EMU_SRC_DOCK_ADC3_LEFT1: u64 = 0x0118;
pub const EMU_SRC_DOCK_ADC3_LEFT2: u64 = 0x0119;
pub const EMU_SRC_DOCK_ADC3_LEFT3: u64 = 0x011a;
pub const EMU_SRC_DOCK_ADC3_LEFT4: u64 = 0x011b;
pub const EMU_SRC_DOCK_ADC3_RIGHT1: u64 = 0x011c;
pub const EMU_SRC_DOCK_ADC3_RIGHT2: u64 = 0x011d;
pub const EMU_SRC_DOCK_ADC3_RIGHT3: u64 = 0x011e;
pub const EMU_SRC_DOCK_ADC3_RIGHT4: u64 = 0x011f;
pub const EMU_SRC_HAMOA_ADC_LEFT1: u64 = 0x0200;
pub const EMU_SRC_HAMOA_ADC_LEFT2: u64 = 0x0202;
pub const EMU_SRC_HAMOA_ADC_LEFT3: u64 = 0x0204;
pub const EMU_SRC_HAMOA_ADC_LEFT4: u64 = 0x0206;
pub const EMU_SRC_HAMOA_ADC_RIGHT1: u64 = 0x0201;
pub const EMU_SRC_HAMOA_ADC_RIGHT2: u64 = 0x0203;
pub const EMU_SRC_HAMOA_ADC_RIGHT3: u64 = 0x0205;
pub const EMU_SRC_HAMOA_ADC_RIGHT4: u64 = 0x0207;
pub const EMU_SRC_ALICE_EMU32A: u64 = 0x0300;
pub const EMU_SRC_ALICE_EMU32B: u64 = 0x0310;
// In S/MUX mode, the samples of one channel are adjacent.
pub const EMU_SRC_HANA_ADAT: u64 = 0x0400;
pub const EMU_SRC_HANA_SPDIF_LEFT1: u64 = 0x0500;
pub const EMU_SRC_HANA_SPDIF_LEFT2: u64 = 0x0502;
pub const EMU_SRC_HANA_SPDIF_LEFT3: u64 = 0x0504;
pub const EMU_SRC_HANA_SPDIF_LEFT4: u64 = 0x0506;
pub const EMU_SRC_HANA_SPDIF_RIGHT1: u64 = 0x0501;
pub const EMU_SRC_HANA_SPDIF_RIGHT2: u64 = 0x0503;
pub const EMU_SRC_HANA_SPDIF_RIGHT3: u64 = 0x0505;
pub const EMU_SRC_HANA_SPDIF_RIGHT4: u64 = 0x0507;

/* Additional inputs for 1616(M)/Microdock */

pub const EMU_SRC_MDOCK_SPDIF_LEFT1: u64 = 0x0112;
pub const EMU_SRC_MDOCK_SPDIF_LEFT2: u64 = 0x0113;
pub const EMU_SRC_MDOCK_SPDIF_RIGHT1: u64 = 0x0116;
pub const EMU_SRC_MDOCK_SPDIF_RIGHT2: u64 = 0x0117;
pub const EMU_SRC_MDOCK_ADAT: u64 = 0x0118;

/* 0x600 and 0x700 no used */


/* ------------------- CONSTANTS -------------------- */

extern "C" { pub static mut snd_emu10k1_fxbus: [*const core::ffi::c_char; 32]; }
extern "C" { pub static mut snd_emu10k1_sblive_ins: [*const core::ffi::c_char; 16]; }
extern "C" { pub static mut snd_emu10k1_audigy_ins: [*const core::ffi::c_char; 16]; }
extern "C" { pub static mut snd_emu10k1_sblive_outs: [*const core::ffi::c_char; 32]; }
extern "C" { pub static mut snd_emu10k1_audigy_outs: [*const core::ffi::c_char; 32]; }
// C declaration: extern const s8 snd_emu10k1_sblive51_fxbus2_map[16];

/* ------------------- STRUCTURES -------------------- */

// C declaration: enum {
// C declaration: 	EMU10K1_UNUSED,  // This must be zero
// C declaration: 	EMU10K1_EFX,
// C declaration: 	EMU10K1_EFX_IRQ,
// C declaration: 	EMU10K1_PCM,
// C declaration: 	EMU10K1_PCM_IRQ,
// C declaration: 	EMU10K1_SYNTH,
// C declaration: 	EMU10K1_NUM_TYPES
// C declaration: };

// C declaration: struct snd_emu10k1;

// C declaration: struct snd_emu10k1_voice {
// C declaration: 	unsigned char number;
// C declaration: 	unsigned char use;
// C declaration: 	unsigned char dirty;
// C declaration: 	unsigned char last;
// C declaration: 	void (*interrupt)(struct snd_emu10k1 *emu, struct snd_emu10k1_voice *pvoice);

// C declaration: 	struct snd_emu10k1_pcm *epcm;
// C declaration: };

// C declaration: enum {
// C declaration: 	PLAYBACK_EMUVOICE,
// C declaration: 	PLAYBACK_EFX,
// C declaration: 	CAPTURE_AC97ADC,
// C declaration: 	CAPTURE_AC97MIC,
// C declaration: 	CAPTURE_EFX
// C declaration: };

// C declaration: struct snd_emu10k1_pcm {
// C declaration: 	struct snd_emu10k1 *emu;
// C declaration: 	int type;
// C declaration: 	struct snd_pcm_substream *substream;
// C declaration: 	struct snd_emu10k1_voice *voices[NUM_EFX_PLAYBACK];
// C declaration: 	struct snd_emu10k1_voice *extra;
// C declaration: 	unsigned short running;
// C declaration: 	unsigned short first_ptr;
// C declaration: 	snd_pcm_uframes_t resume_pos;
// C declaration: 	struct snd_util_memblk *memblk;
// C declaration: 	unsigned int pitch_target;
// C declaration: 	unsigned int start_addr;
// C declaration: 	unsigned int ccca_start_addr;
// C declaration: 	unsigned int capture_ipr;	/* interrupt acknowledge mask */
// C declaration: 	unsigned int capture_inte;	/* interrupt enable mask */
// C declaration: 	unsigned int capture_ba_reg;	/* buffer address register */
// C declaration: 	unsigned int capture_bs_reg;	/* buffer size register */
// C declaration: 	unsigned int capture_idx_reg;	/* buffer index register */
// C declaration: 	unsigned int capture_cr_val;	/* control value */
// C declaration: 	unsigned int capture_cr_val2;	/* control value2 (for audigy) */
// C declaration: 	unsigned int capture_bs_val;	/* buffer size value */
// C declaration: 	unsigned int capture_bufsize;	/* buffer size in bytes */
// C declaration: };

// C declaration: struct snd_emu10k1_pcm_mixer {
// C declaration: 	/* mono, left, right x 8 sends (4 on emu10k1) */
// C declaration: 	unsigned char send_routing[3][8];
// C declaration: 	unsigned char send_volume[3][8];
// C declaration: 	// 0x8000 is neutral. The mixer code rescales it to 0xffff to maintain
// C declaration: 	// backwards compatibility with user space.
// C declaration: 	unsigned short attn[3];
// C declaration: 	struct snd_emu10k1_pcm *epcm;
// C declaration: };

// C declaration: #define snd_emu10k1_compose_send_routing(route) \
// C declaration: ((route[0] | (route[1] << 4) | (route[2] << 8) | (route[3] << 12)) << 16)

// C declaration: #define snd_emu10k1_compose_audigy_fxrt1(route) \
// C declaration: ((unsigned int)route[0] | ((unsigned int)route[1] << 8) | ((unsigned int)route[2] << 16) | ((unsigned int)route[3] << 24) | 0x80808080)

// C declaration: #define snd_emu10k1_compose_audigy_fxrt2(route) \
// C declaration: ((unsigned int)route[4] | ((unsigned int)route[5] << 8) | ((unsigned int)route[6] << 16) | ((unsigned int)route[7] << 24) | 0x80808080)

// C declaration: #define snd_emu10k1_compose_audigy_sendamounts(vol) \
// C declaration: (((unsigned int)vol[4] << 24) | ((unsigned int)vol[5] << 16) | ((unsigned int)vol[6] << 8) | (unsigned int)vol[7])

// C declaration: struct snd_emu10k1_memblk {
// C declaration: 	struct snd_util_memblk mem;
// C declaration: 	/* private part */
// C declaration: 	int first_page, last_page, pages, mapped_page;
// C declaration: 	unsigned int map_locked;
// C declaration: 	struct list_head mapped_link;
// C declaration: 	struct list_head mapped_order_link;
// C declaration: };

// C declaration: #define snd_emu10k1_memblk_offset(blk)	(((blk)->mapped_page << PAGE_SHIFT) | ((blk)->mem.offset & (PAGE_SIZE - 1)))

pub const EMU10K1_MAX_TRAM_BLOCKS_PER_CODE: u64 = 16;

// C declaration: struct snd_emu10k1_fx8010_ctl {
// C declaration: 	struct list_head list;		/* list link container */
// C declaration: 	unsigned int vcount;
// C declaration: 	unsigned int count;		/* count of GPR (1..16) */
// C declaration: 	unsigned short gpr[32];		/* GPR number(s) */
// C declaration: 	int value[32];
// C declaration: 	int min;			/* minimum range */
// C declaration: 	int max;			/* maximum range */
// C declaration: 	unsigned int translation;	/* translation type (EMU10K1_GPR_TRANSLATION*) */
// C declaration: 	struct snd_kcontrol *kcontrol;
// C declaration: };

// C declaration: typedef void (snd_fx8010_irq_handler_t)(struct snd_emu10k1 *emu, void *private_data);

// C declaration: struct snd_emu10k1_fx8010_irq {
// C declaration: 	struct snd_emu10k1_fx8010_irq *next;
// C declaration: 	snd_fx8010_irq_handler_t *handler;
// C declaration: 	unsigned short gpr_running;
// C declaration: 	void *private_data;
// C declaration: };

// C declaration: struct snd_emu10k1_fx8010_pcm {
// C declaration: 	unsigned int valid: 1,
// C declaration: 		     opened: 1,
// C declaration: 		     active: 1;
// C declaration: 	unsigned int channels;		/* 16-bit channels count */
// C declaration: 	unsigned int tram_start;	/* initial ring buffer position in TRAM (in samples) */
// C declaration: 	unsigned int buffer_size;	/* count of buffered samples */
// C declaration: 	unsigned short gpr_size;		/* GPR containing size of ring buffer in samples (host) */
// C declaration: 	unsigned short gpr_ptr;		/* GPR containing current pointer in the ring buffer (host = reset, FX8010) */
// C declaration: 	unsigned short gpr_count;	/* GPR containing count of samples between two interrupts (host) */
// C declaration: 	unsigned short gpr_tmpcount;	/* GPR containing current count of samples to interrupt (host = set, FX8010) */
// C declaration: 	unsigned short gpr_trigger;	/* GPR containing trigger (activate) information (host) */
// C declaration: 	unsigned short gpr_running;	/* GPR containing info if PCM is running (FX8010) */
// C declaration: 	unsigned char etram[32];	/* external TRAM address & data */
// C declaration: 	struct snd_pcm_indirect pcm_rec;
// C declaration: 	unsigned int tram_pos;
// C declaration: 	unsigned int tram_shift;
// C declaration: 	struct snd_emu10k1_fx8010_irq irq;
// C declaration: };

// C declaration: struct snd_emu10k1_fx8010 {
// C declaration: 	unsigned short extin_mask;	/* used external inputs (bitmask); not used for Audigy */
// C declaration: 	unsigned short extout_mask;	/* used external outputs (bitmask); not used for Audigy */
// C declaration: 	unsigned int itram_size;	/* internal TRAM size in samples */
// C declaration: 	struct snd_dma_buffer etram_pages; /* external TRAM pages and size */
// C declaration: 	unsigned int dbg;		/* FX debugger register */
// C declaration: 	unsigned char name[128];
// C declaration: 	int gpr_size;			/* size of allocated GPR controls */
// C declaration: 	int gpr_count;			/* count of used kcontrols */
// C declaration: 	struct list_head gpr_ctl;	/* GPR controls */
// C declaration: 	struct mutex lock;
// C declaration: 	struct snd_emu10k1_fx8010_pcm pcm[8];
// C declaration: 	spinlock_t irq_lock;
// C declaration: 	struct snd_emu10k1_fx8010_irq *irq_handlers;
// C declaration: };

// C declaration: struct snd_emu10k1_midi {
// C declaration: 	struct snd_emu10k1 *emu;
// C declaration: 	struct snd_rawmidi *rmidi;
// C declaration: 	struct snd_rawmidi_substream *substream_input;
// C declaration: 	struct snd_rawmidi_substream *substream_output;
// C declaration: 	unsigned int midi_mode;
// C declaration: 	spinlock_t input_lock;
// C declaration: 	spinlock_t output_lock;
// C declaration: 	spinlock_t open_lock;
// C declaration: 	int tx_enable, rx_enable;
// C declaration: 	int port;
// C declaration: 	int ipr_tx, ipr_rx;
// C declaration: 	void (*interrupt)(struct snd_emu10k1 *emu, unsigned int status);
// C declaration: };

// C declaration: enum {
// C declaration: 	EMU_MODEL_SB,
// C declaration: 	EMU_MODEL_EMU1010,
// C declaration: 	EMU_MODEL_EMU1010B,
// C declaration: 	EMU_MODEL_EMU1616,
// C declaration: 	EMU_MODEL_EMU0404,
// C declaration: };

// Chip-o-logy:
// - All SB Live! cards use EMU10K1 chips
// - All SB Audigy cards use CA* chips, termed "emu10k2" by the driver
// - Original Audigy uses CA0100 "Alice"
// - Audigy 2 uses CA0102/CA10200 "Alice2"
//   - Has an interface for CA0151 (P16V) "Alice3"
// - Audigy 2 Value uses CA0108/CA10300 "Tina"
//   - Approximately a CA0102 with an on-chip CA0151 (P17V)
// - Audigy 2 ZS NB uses CA0109 "Tina2"
//   - Cardbus version of CA0108
// C declaration: struct snd_emu_chip_details {
// C declaration: 	u32 vendor;
// C declaration: 	u32 device;
// C declaration: 	u32 subsystem;
// C declaration: 	unsigned char revision;
// C declaration: 	unsigned char emu_model;	/* EMU model type */
// C declaration: 	unsigned int emu10k1_chip:1;	/* Original SB Live. Not SB Live 24bit. */
// C declaration: 					/* Redundant with emu10k2_chip being unset. */
// C declaration: 	unsigned int emu10k2_chip:1;	/* Audigy 1 or Audigy 2. */
// C declaration: 	unsigned int ca0102_chip:1;	/* Audigy 1 or Audigy 2. Not SB Audigy 2 Value. */
// C declaration: 					/* Redundant with ca0108_chip being unset. */
// C declaration: 	unsigned int ca0108_chip:1;	/* Audigy 2 Value */
// C declaration: 	unsigned int ca_cardbus_chip:1;	/* Audigy 2 ZS Notebook */
// C declaration: 	unsigned int ca0151_chip:1;	/* P16V */
// C declaration: 	unsigned int spk20:1;		/* Stereo only */
// C declaration: 	unsigned int spk71:1;		/* Has 7.1 speakers */
// C declaration: 	unsigned int no_adat:1;		/* Has no ADAT, only SPDIF */
// C declaration: 	unsigned int sblive51:1;	/* SBLive! 5.1 - extout 0x11 -> center, 0x12 -> lfe */
// C declaration: 	unsigned int spdif_bug:1;	/* Has Spdif phasing bug */
// C declaration: 	unsigned int ac97_chip:2;	/* Has an AC97 chip: 1 = mandatory, 2 = optional */
// C declaration: 	unsigned int ecard:1;		/* APS EEPROM */
// C declaration: 	unsigned int spi_dac:1;		/* SPI interface for DAC; requires ca0108_chip */
// C declaration: 	unsigned int i2c_adc:1;		/* I2C interface for ADC; requires ca0108_chip */
// C declaration: 	unsigned int adc_1361t:1;	/* Use Philips 1361T ADC */
// C declaration: 	unsigned int invert_shared_spdif:1;  /* analog/digital switch inverted */
// C declaration: 	const char *driver;
// C declaration: 	const char *name;
// C declaration: 	const char *id;		/* for backward compatibility - can be NULL if not needed */
// C declaration: };

pub const NUM_OUTPUT_DESTS: u64 = 28;
pub const NUM_INPUT_DESTS: u64 = 22;

// C declaration: struct snd_emu1010 {
// C declaration: 	unsigned char output_source[NUM_OUTPUT_DESTS];
// C declaration: 	unsigned char input_source[NUM_INPUT_DESTS];
// C declaration: 	unsigned int adc_pads; /* bit mask */
// C declaration: 	unsigned int dac_pads; /* bit mask */
// C declaration: 	unsigned int wclock;  /* Cached register value */
// C declaration: 	unsigned int word_clock;  /* Cached effective value */
// C declaration: 	unsigned int clock_source;
// C declaration: 	unsigned int clock_fallback;
// C declaration: 	unsigned int optical_in; /* 0:SPDIF, 1:ADAT */
// C declaration: 	unsigned int optical_out; /* 0:SPDIF, 1:ADAT */
// C declaration: 	struct work_struct work;
// C declaration: 	struct mutex lock;
// C declaration: };

// C declaration: struct snd_emu10k1 {
// C declaration: 	int irq;

// C declaration: 	unsigned long port;			/* I/O port number */
// C declaration: 	unsigned int tos_link: 1,		/* tos link detected */
// C declaration: 		rear_ac97: 1,			/* rear channels are on AC'97 */
// C declaration: 		enable_ir: 1;
// C declaration: 	unsigned int support_tlv :1;
// C declaration: 	/* Contains profile of card capabilities */
// C declaration: 	const struct snd_emu_chip_details *card_capabilities;
// C declaration: 	unsigned int audigy;			/* is Audigy? */
// C declaration: 	unsigned int revision;			/* chip revision */
// C declaration: 	unsigned int serial;			/* serial number */
// C declaration: 	unsigned short model;			/* subsystem id */
// C declaration: 	unsigned int ecard_ctrl;		/* ecard control bits */
// C declaration: 	unsigned int address_mode;		/* address mode */
// C declaration: 	unsigned long dma_mask;			/* PCI DMA mask */
// C declaration: 	bool iommu_workaround;			/* IOMMU workaround needed */
// C declaration: 	int max_cache_pages;			/* max memory size / PAGE_SIZE */
// C declaration: 	struct snd_dma_buffer silent_page;	/* silent page */
// C declaration: 	struct snd_dma_buffer ptb_pages;	/* page table pages */
// C declaration: 	struct snd_dma_device p16v_dma_dev;
// C declaration: 	struct snd_dma_buffer *p16v_buffer;

// C declaration: 	struct snd_util_memhdr *memhdr;		/* page allocation list */

// C declaration: 	struct list_head mapped_link_head;
// C declaration: 	struct list_head mapped_order_link_head;
// C declaration: 	void **page_ptr_table;
// C declaration: 	unsigned long *page_addr_table;
// C declaration: 	spinlock_t memblk_lock;

// C declaration: 	unsigned int spdif_bits[3];		/* s/pdif out setup */
// C declaration: 	unsigned int i2c_capture_source;
// C declaration: 	u8 i2c_capture_volume[4][2];

// C declaration: 	struct snd_emu10k1_fx8010 fx8010;		/* FX8010 info */
// C declaration: 	int gpr_base;
	
// C declaration: 	struct snd_ac97 *ac97;

// C declaration: 	struct pci_dev *pci;
// C declaration: 	struct snd_card *card;
// C declaration: 	struct snd_pcm *pcm;
// C declaration: 	struct snd_pcm *pcm_mic;
// C declaration: 	struct snd_pcm *pcm_efx;
// C declaration: 	struct snd_pcm *pcm_multi;
// C declaration: 	struct snd_pcm *pcm_p16v;

// C declaration: 	spinlock_t synth_lock;
// C declaration: 	void *synth;
// C declaration: 	int (*get_synth_voice)(struct snd_emu10k1 *emu);

// C declaration: 	spinlock_t reg_lock;  // high-level driver lock
// C declaration: 	spinlock_t emu_lock;  // low-level i/o lock
// C declaration: 	spinlock_t voice_lock;  // voice allocator lock
// C declaration: 	spinlock_t spi_lock; /* serialises access to spi port */
// C declaration: 	spinlock_t i2c_lock; /* serialises access to i2c port */

// C declaration: 	struct snd_emu10k1_voice voices[NUM_G];
// C declaration: 	int p16v_device_offset;
// C declaration: 	u32 p16v_capture_source;
// C declaration: 	u32 p16v_capture_channel;
// C declaration:         struct snd_emu1010 emu1010;
// C declaration: 	struct snd_emu10k1_pcm_mixer pcm_mixer[32];
// C declaration: 	struct snd_emu10k1_pcm_mixer efx_pcm_mixer[NUM_EFX_PLAYBACK];
// C declaration: 	struct snd_kcontrol *ctl_send_routing;
// C declaration: 	struct snd_kcontrol *ctl_send_volume;
// C declaration: 	struct snd_kcontrol *ctl_attn;
// C declaration: 	struct snd_kcontrol *ctl_efx_send_routing;
// C declaration: 	struct snd_kcontrol *ctl_efx_send_volume;
// C declaration: 	struct snd_kcontrol *ctl_efx_attn;
// C declaration: 	struct snd_kcontrol *ctl_clock_source;

// C declaration: 	void (*hwvol_interrupt)(struct snd_emu10k1 *emu, unsigned int status);
// C declaration: 	void (*capture_interrupt)(struct snd_emu10k1 *emu, unsigned int status);
// C declaration: 	void (*capture_mic_interrupt)(struct snd_emu10k1 *emu, unsigned int status);
// C declaration: 	void (*capture_efx_interrupt)(struct snd_emu10k1 *emu, unsigned int status);
// C declaration: 	void (*spdif_interrupt)(struct snd_emu10k1 *emu, unsigned int status);
// C declaration: 	void (*dsp_interrupt)(struct snd_emu10k1 *emu);
// C declaration: 	void (*gpio_interrupt)(struct snd_emu10k1 *emu);
// C declaration: 	void (*p16v_interrupt)(struct snd_emu10k1 *emu);

// C declaration: 	struct snd_pcm_substream *pcm_capture_substream;
// C declaration: 	struct snd_pcm_substream *pcm_capture_mic_substream;
// C declaration: 	struct snd_pcm_substream *pcm_capture_efx_substream;

// C declaration: 	struct snd_timer *timer;

// C declaration: 	struct snd_emu10k1_midi midi;
// C declaration: 	struct snd_emu10k1_midi midi2; /* for audigy */

// C declaration: 	unsigned int efx_voices_mask[2];
// C declaration: 	unsigned int next_free_voice;

// C declaration: 	const struct firmware *firmware;
// C declaration: 	const struct firmware *dock_fw;

// C declaration: #ifdef CONFIG_PM_SLEEP
// C declaration: 	unsigned int *saved_ptr;
// C declaration: 	unsigned int *saved_gpr;
// C declaration: 	unsigned int *tram_val_saved;
// C declaration: 	unsigned int *tram_addr_saved;
// C declaration: 	unsigned int *saved_icode;
// C declaration: 	unsigned int *p16v_saved;
// C declaration: 	unsigned int saved_a_iocfg, saved_hcfg;
// C declaration: 	bool suspend;
// #endif

// C declaration: };

// C declaration: int snd_emu10k1_create(struct snd_card *card,
// C declaration: 		       struct pci_dev *pci,
// C declaration: 		       unsigned short extin_mask,
// C declaration: 		       unsigned short extout_mask,
// C declaration: 		       long max_cache_bytes,
// C declaration: 		       int enable_ir,
// C declaration: 		       uint subsystem);

// C declaration: int snd_emu10k1_pcm(struct snd_emu10k1 *emu, int device);
// C declaration: int snd_emu10k1_pcm_mic(struct snd_emu10k1 *emu, int device);
// C declaration: int snd_emu10k1_pcm_efx(struct snd_emu10k1 *emu, int device);
// C declaration: int snd_p16v_pcm(struct snd_emu10k1 *emu, int device);
// C declaration: int snd_p16v_mixer(struct snd_emu10k1 * emu);
// C declaration: int snd_emu10k1_pcm_multi(struct snd_emu10k1 *emu, int device);
// C declaration: int snd_emu10k1_fx8010_pcm(struct snd_emu10k1 *emu, int device);
// C declaration: int snd_emu10k1_mixer(struct snd_emu10k1 * emu, int pcm_device, int multi_device);
// C declaration: int snd_emu10k1_timer(struct snd_emu10k1 * emu, int device);
// C declaration: int snd_emu10k1_fx8010_new(struct snd_emu10k1 *emu, int device);

// C declaration: irqreturn_t snd_emu10k1_interrupt(int irq, void *dev_id);

// C declaration: void snd_emu10k1_voice_init(struct snd_emu10k1 * emu, int voice);
// C declaration: int snd_emu10k1_init_efx(struct snd_emu10k1 *emu);
// C declaration: void snd_emu10k1_free_efx(struct snd_emu10k1 *emu);
// C declaration: int snd_emu10k1_fx8010_tram_setup(struct snd_emu10k1 *emu, u32 size);
// C declaration: int snd_emu10k1_done(struct snd_emu10k1 * emu);

/* I/O functions */
// C declaration: unsigned int snd_emu10k1_ptr_read(struct snd_emu10k1 * emu, unsigned int reg, unsigned int chn);
// C declaration: void snd_emu10k1_ptr_write(struct snd_emu10k1 *emu, unsigned int reg, unsigned int chn, unsigned int data);
// C declaration: void snd_emu10k1_ptr_write_multiple(struct snd_emu10k1 *emu, unsigned int chn, ...);
// C declaration: unsigned int snd_emu10k1_ptr20_read(struct snd_emu10k1 * emu, unsigned int reg, unsigned int chn);
// C declaration: void snd_emu10k1_ptr20_write(struct snd_emu10k1 *emu, unsigned int reg, unsigned int chn, unsigned int data);
// C declaration: int snd_emu10k1_spi_write(struct snd_emu10k1 * emu, unsigned int data);
// C declaration: int snd_emu10k1_i2c_write(struct snd_emu10k1 *emu, u32 reg, u32 value);
// C declaration: DEFINE_GUARD(snd_emu1010_fpga_lock, struct snd_emu10k1 *, mutex_lock(&(_T)->emu1010.lock), mutex_unlock(&(_T)->emu1010.lock))
// C declaration: void snd_emu1010_fpga_write_lock(struct snd_emu10k1 *emu, u32 reg, u32 value);
// C declaration: void snd_emu1010_fpga_write(struct snd_emu10k1 *emu, u32 reg, u32 value);
// C declaration: void snd_emu1010_fpga_read(struct snd_emu10k1 *emu, u32 reg, u32 *value);
// C declaration: void snd_emu1010_fpga_link_dst_src_write(struct snd_emu10k1 *emu, u32 dst, u32 src);
// C declaration: u32 snd_emu1010_fpga_link_dst_src_read(struct snd_emu10k1 *emu, u32 dst);
// C declaration: int snd_emu1010_get_raw_rate(struct snd_emu10k1 *emu, u8 src);
// C declaration: void snd_emu1010_update_clock(struct snd_emu10k1 *emu);
// C declaration: void snd_emu1010_load_firmware_entry(struct snd_emu10k1 *emu, int dock, const struct firmware *fw_entry);
// C declaration: unsigned int snd_emu10k1_efx_read(struct snd_emu10k1 *emu, unsigned int pc);
// C declaration: void snd_emu10k1_intr_enable(struct snd_emu10k1 *emu, unsigned int intrenb);
// C declaration: void snd_emu10k1_intr_disable(struct snd_emu10k1 *emu, unsigned int intrenb);
// C declaration: void snd_emu10k1_voice_intr_enable(struct snd_emu10k1 *emu, unsigned int voicenum);
// C declaration: void snd_emu10k1_voice_intr_disable(struct snd_emu10k1 *emu, unsigned int voicenum);
// C declaration: void snd_emu10k1_voice_intr_ack(struct snd_emu10k1 *emu, unsigned int voicenum);
// C declaration: void snd_emu10k1_voice_half_loop_intr_enable(struct snd_emu10k1 *emu, unsigned int voicenum);
// C declaration: void snd_emu10k1_voice_half_loop_intr_disable(struct snd_emu10k1 *emu, unsigned int voicenum);
// C declaration: void snd_emu10k1_voice_half_loop_intr_ack(struct snd_emu10k1 *emu, unsigned int voicenum);
// C declaration: #if 0
// C declaration: void snd_emu10k1_voice_set_loop_stop(struct snd_emu10k1 *emu, unsigned int voicenum);
// C declaration: void snd_emu10k1_voice_clear_loop_stop(struct snd_emu10k1 *emu, unsigned int voicenum);
// #endif
// C declaration: void snd_emu10k1_voice_set_loop_stop_multiple(struct snd_emu10k1 *emu, u64 voices);
// C declaration: void snd_emu10k1_voice_clear_loop_stop_multiple(struct snd_emu10k1 *emu, u64 voices);
// C declaration: int snd_emu10k1_voice_clear_loop_stop_multiple_atomic(struct snd_emu10k1 *emu, u64 voices);
// C declaration: void snd_emu10k1_wait(struct snd_emu10k1 *emu, unsigned int wait);
// C declaration: static inline unsigned int snd_emu10k1_wc(struct snd_emu10k1 *emu) { return (inl(emu->port + WC) >> 6) & 0xfffff; }
// C declaration: unsigned short snd_emu10k1_ac97_read(struct snd_ac97 *ac97, unsigned short reg);
// C declaration: void snd_emu10k1_ac97_write(struct snd_ac97 *ac97, unsigned short reg, unsigned short data);

// C declaration: #ifdef CONFIG_PM_SLEEP
// C declaration: void snd_emu10k1_suspend_regs(struct snd_emu10k1 *emu);
// C declaration: void snd_emu10k1_resume_init(struct snd_emu10k1 *emu);
// C declaration: void snd_emu10k1_resume_regs(struct snd_emu10k1 *emu);
// C declaration: int snd_emu10k1_efx_alloc_pm_buffer(struct snd_emu10k1 *emu);
// C declaration: void snd_emu10k1_efx_free_pm_buffer(struct snd_emu10k1 *emu);
// C declaration: void snd_emu10k1_efx_suspend(struct snd_emu10k1 *emu);
// C declaration: void snd_emu10k1_efx_resume(struct snd_emu10k1 *emu);
// C declaration: int snd_p16v_alloc_pm_buffer(struct snd_emu10k1 *emu);
// C declaration: void snd_p16v_free_pm_buffer(struct snd_emu10k1 *emu);
// C declaration: void snd_p16v_suspend(struct snd_emu10k1 *emu);
// C declaration: void snd_p16v_resume(struct snd_emu10k1 *emu);
// #endif

/* memory allocation */
// C declaration: struct snd_util_memblk *snd_emu10k1_alloc_pages(struct snd_emu10k1 *emu, struct snd_pcm_substream *substream);
// C declaration: int snd_emu10k1_free_pages(struct snd_emu10k1 *emu, struct snd_util_memblk *blk);
// C declaration: int snd_emu10k1_alloc_pages_maybe_wider(struct snd_emu10k1 *emu, size_t size,
// C declaration: 					struct snd_dma_buffer *dmab);
// C declaration: struct snd_util_memblk *snd_emu10k1_synth_alloc(struct snd_emu10k1 *emu, unsigned int size);
// C declaration: int snd_emu10k1_synth_free(struct snd_emu10k1 *emu, struct snd_util_memblk *blk);
// C declaration: int snd_emu10k1_synth_memset(struct snd_emu10k1 *emu, struct snd_util_memblk *blk, int offset, int size, u8 value);
// C declaration: int snd_emu10k1_synth_copy_from_user(struct snd_emu10k1 *emu, struct snd_util_memblk *blk, int offset, const char __user *data, int size, u32 xor);
// C declaration: int snd_emu10k1_memblk_map(struct snd_emu10k1 *emu, struct snd_emu10k1_memblk *blk);

/* voice allocation */
// C declaration: int snd_emu10k1_voice_alloc(struct snd_emu10k1 *emu, int type, int count, int channels,
// C declaration: 			    struct snd_emu10k1_pcm *epcm, struct snd_emu10k1_voice **rvoice);
// C declaration: int snd_emu10k1_voice_free(struct snd_emu10k1 *emu, struct snd_emu10k1_voice *pvoice);

/* MIDI uart */
// C declaration: int snd_emu10k1_midi(struct snd_emu10k1 * emu);
// C declaration: int snd_emu10k1_audigy_midi(struct snd_emu10k1 * emu);

/* proc interface */
// C declaration: int snd_emu10k1_proc_init(struct snd_emu10k1 * emu);

/* fx8010 irq handler */
// C declaration: int snd_emu10k1_fx8010_register_irq_handler(struct snd_emu10k1 *emu,
// C declaration: 					    snd_fx8010_irq_handler_t *handler,
// C declaration: 					    unsigned char gpr_running,
// C declaration: 					    void *private_data,
// C declaration: 					    struct snd_emu10k1_fx8010_irq *irq);
// C declaration: int snd_emu10k1_fx8010_unregister_irq_handler(struct snd_emu10k1 *emu,
// C declaration: 					      struct snd_emu10k1_fx8010_irq *irq);

// #endif	/* __SOUND_EMU10K1_H */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
