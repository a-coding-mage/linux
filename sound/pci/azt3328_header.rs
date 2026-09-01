/* SPDX-License-Identifier: GPL-2.0 */

/* "PU" == "power-up value", as tested on PCI168 PCI rev. 10
 * "WRITE_ONLY"  == register does not indicate actual bit values */

/*** main I/O area port indices ***/
/* (only 0x70 of 0x80 bytes saved/restored by Windows driver) */
pub const AZF_IO_SIZE_CTRL: u32 = 0x80;
pub const AZF_IO_SIZE_CTRL_PM: u32 = 0x70;

/* the driver initialisation suggests a layout of 4 areas
 * within the main card control I/O:
 * from 0x00 (playback codec), from 0x20 (recording codec)
 * and from 0x40 (most certainly I2S out codec).
 * And another area from 0x60 to 0x6f (DirectX timer, IRQ management,
 * power management etc.???). */

pub const AZF_IO_OFFS_CODEC_PLAYBACK: u32 = 0x00;
pub const AZF_IO_OFFS_CODEC_CAPTURE: u32 = 0x20;
pub const AZF_IO_OFFS_CODEC_I2S_OUT: u32 = 0x40;

pub const IDX_IO_CODEC_DMA_FLAGS: u32 = 0x00; /* PU:0x0000 */
     /* able to reactivate output after output muting due to 8/16bit
      * output change, just like 0x0002.
      * 0x0001 is the only bit that's able to start the DMA counter */
pub const DMA_RESUME: u32 = 0x0001; /* paused if cleared? */
     /* 0x0002 *temporarily* set during DMA stopping. hmm
      * both 0x0002 and 0x0004 set in playback setup. */
     /* able to reactivate output after output muting due to 8/16bit
      * output change, just like 0x0001. */
pub const DMA_RUN_SOMETHING1: u32 = 0x0002; /* \ alternated (toggled) */
     /* 0x0004: NOT able to reactivate output */
pub const DMA_RUN_SOMETHING2: u32 = 0x0004; /* / bits */
pub const SOMETHING_ALMOST_ALWAYS_SET: u32 = 0x0008; /* ???; can be modified */
pub const DMA_EPILOGUE_SOMETHING: u32 = 0x0010;
pub const DMA_SOMETHING_ELSE: u32 = 0x0020; /* ??? */
pub const SOMETHING_UNMODIFIABLE: u32 = 0xffc0; /* unused? not modifiable */
pub const IDX_IO_CODEC_IRQTYPE: u32 = 0x02; /* PU:0x0001 */
  /* write back to flags in case flags are set, in order to ACK IRQ in handler
   * (bit 1 of port 0x64 indicates interrupt for one of these three types)
   * sometimes in this case it just writes 0xffff to globally ACK all IRQs
   * settings written are not reflected when reading back, though.
   * seems to be IRQ, too (frequently used: port |= 0x07 !), but who knows? */
pub const IRQ_SOMETHING: u32 = 0x0001; /* something & ACK */
pub const IRQ_FINISHED_DMABUF_1: u32 = 0x0002; /* 1st dmabuf finished & ACK */
pub const IRQ_FINISHED_DMABUF_2: u32 = 0x0004; /* 2nd dmabuf finished & ACK */
pub const IRQMASK_SOME_STATUS_1: u32 = 0x0008; /* \ related bits */
pub const IRQMASK_SOME_STATUS_2: u32 = 0x0010; /* / (checked together in loop) */
pub const IRQMASK_UNMODIFIABLE: u32 = 0xffe0; /* unused? not modifiable */
/* start address of 1st DMA transfer area, PU:0x00000000 */
pub const IDX_IO_CODEC_DMA_START_1: u32 = 0x04;
/* start address of 2nd DMA transfer area, PU:0x00000000 */
pub const IDX_IO_CODEC_DMA_START_2: u32 = 0x08;
/* both lengths of DMA transfer areas, PU:0x00000000
   length1: offset 0x0c, length2: offset 0x0e */
pub const IDX_IO_CODEC_DMA_LENGTHS: u32 = 0x0c;
pub const IDX_IO_CODEC_DMA_CURRPOS: u32 = 0x10; /* current DMA position, PU:0x00000000 */
/* offset within current DMA transfer area, PU:0x0000 */
pub const IDX_IO_CODEC_DMA_CURROFS: u32 = 0x14;
pub const IDX_IO_CODEC_SOUNDFORMAT: u32 = 0x16; /* PU:0x0010 */
/* all unspecified bits can't be modified */
pub const SOUNDFORMAT_FREQUENCY_MASK: u32 = 0x000f;
pub const SOUNDFORMAT_XTAL1: u32 = 0x00;
pub const SOUNDFORMAT_XTAL2: u32 = 0x01;
/* all _SUSPECTED_ values are not used by Windows drivers, so we don't
 * have any hard facts, only rough measurements.
 * All we know is that the crystal used on the board has 24.576MHz,
 * like many soundcards (which results in the frequencies below when
 * using certain divider values selected by the values below) */
pub const SOUNDFORMAT_FREQ_SUSPECTED_4000: u32 = 0x0c | SOUNDFORMAT_XTAL1;
pub const SOUNDFORMAT_FREQ_SUSPECTED_4800: u32 = 0x0a | SOUNDFORMAT_XTAL1;
pub const SOUNDFORMAT_FREQ_5510: u32 = 0x0c | SOUNDFORMAT_XTAL2;
pub const SOUNDFORMAT_FREQ_6620: u32 = 0x0a | SOUNDFORMAT_XTAL2;
pub const SOUNDFORMAT_FREQ_8000: u32 = 0x00 | SOUNDFORMAT_XTAL1; /* also 0x0e | SOUNDFORMAT_XTAL1? */
pub const SOUNDFORMAT_FREQ_9600: u32 = 0x08 | SOUNDFORMAT_XTAL1;
pub const SOUNDFORMAT_FREQ_11025: u32 = 0x00 | SOUNDFORMAT_XTAL2; /* also 0x0e | SOUNDFORMAT_XTAL2? */
pub const SOUNDFORMAT_FREQ_SUSPECTED_13240: u32 = 0x08 | SOUNDFORMAT_XTAL2; /* seems to be 6620 *2 */
pub const SOUNDFORMAT_FREQ_16000: u32 = 0x02 | SOUNDFORMAT_XTAL1;
pub const SOUNDFORMAT_FREQ_22050: u32 = 0x02 | SOUNDFORMAT_XTAL2;
pub const SOUNDFORMAT_FREQ_32000: u32 = 0x04 | SOUNDFORMAT_XTAL1;
pub const SOUNDFORMAT_FREQ_44100: u32 = 0x04 | SOUNDFORMAT_XTAL2;
pub const SOUNDFORMAT_FREQ_48000: u32 = 0x06 | SOUNDFORMAT_XTAL1;
pub const SOUNDFORMAT_FREQ_SUSPECTED_66200: u32 = 0x06 | SOUNDFORMAT_XTAL2; /* 66200 (13240 * 5); 64000 may have been nicer :-\ */
pub const SOUNDFORMAT_FLAG_16BIT: u32 = 0x0010;
pub const SOUNDFORMAT_FLAG_2CHANNELS: u32 = 0x0020;

/* define frequency helpers, for maximum value safety */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum azf_freq_t {
    AZF_FREQ_4000 = 4000,
    AZF_FREQ_4800 = 4800,
    AZF_FREQ_5512 = 5512,
    AZF_FREQ_6620 = 6620,
    AZF_FREQ_8000 = 8000,
    AZF_FREQ_9600 = 9600,
    AZF_FREQ_11025 = 11025,
    AZF_FREQ_13240 = 13240,
    AZF_FREQ_16000 = 16000,
    AZF_FREQ_22050 = 22050,
    AZF_FREQ_32000 = 32000,
    AZF_FREQ_44100 = 44100,
    AZF_FREQ_48000 = 48000,
    AZF_FREQ_66200 = 66200,
}

/** DirectX timer, main interrupt area (FIXME: and something else?) **/
pub const IDX_IO_TIMER_VALUE: u32 = 0x60; /* found this timer area by pure luck :-) */
/* timer countdown value; triggers IRQ when timer is finished */
pub const TIMER_VALUE_MASK: u32 = 0x000fffff;
/* activate timer countdown */
pub const TIMER_COUNTDOWN_ENABLE: u32 = 0x01000000;
/* trigger timer IRQ on zero transition */
pub const TIMER_IRQ_ENABLE: u32 = 0x02000000;
/* being set in IRQ handler in case port 0x00 (hmm, not port 0x64!?!?)
 * had 0x0020 set upon IRQ handler */
pub const TIMER_IRQ_ACK: u32 = 0x04000000;
pub const IDX_IO_IRQSTATUS: u32 = 0x64;
/* some IRQ bit in here might also be used to signal a power-management timer
 * timeout, to request shutdown of the chip (e.g. AD1815JS has such a thing).
 * OPL3 hardware contains several timers which confusingly in most cases
 * are NOT routed to an IRQ, but some designs (e.g. LM4560) DO support that,
 * so I wouldn't be surprised at all to discover that AZF3328
 * supports that thing as well... */

pub const IRQ_PLAYBACK: u32 = 0x0001;
pub const IRQ_RECORDING: u32 = 0x0002;
pub const IRQ_I2S_OUT: u32 = 0x0004; /* this IS I2S, right!? (untested) */
pub const IRQ_GAMEPORT: u32 = 0x0008; /* Interrupt of Digital(ly) Enhanced Game Port */
pub const IRQ_MPU401: u32 = 0x0010;
pub const IRQ_TIMER: u32 = 0x0020; /* DirectX timer */
pub const IRQ_UNKNOWN2: u32 = 0x0040; /* probably unused, or possibly OPL3 timer? */
pub const IRQ_UNKNOWN3: u32 = 0x0080; /* probably unused, or possibly OPL3 timer? */
pub const IDX_IO_66H: u32 = 0x66; /* writing 0xffff returns 0x0000 */
/* this is set to e.g. 0x3ff or 0x300, and writable;
 * maybe some buffer limit, but I couldn't find out more, PU:0x00ff: */
pub const IDX_IO_SOME_VALUE: u32 = 0x68;
pub const IO_68_RANDOM_TOGGLE1: u32 = 0x0100; /* toggles randomly */
pub const IO_68_RANDOM_TOGGLE2: u32 = 0x0200; /* toggles randomly */
/* umm, nope, behaviour of these bits changes depending on what we wrote
 * to 0x6b!!
 * And they change upon playback/stop, too:
 * Writing a value to 0x68 will display this exact value during playback,
 * too but when stopped it can fall back to a rather different
 * seemingly random value). Hmm, possibly this is a register which
 * has a remote shadow which needs proper device supply which only exists
 * in case playback is active? Or is this driver-induced?
 */

/* this WORD can be set to have bits 0x0028 activated (FIXME: correct??);
 * actually inhibits PCM playback!!! maybe power management??: */
pub const IDX_IO_6AH: u32 = 0x6A; /* WRITE_ONLY! */
/* bit 5: enabling this will activate permanent counting of bytes 2/3
 * at gameport I/O (0xb402/3) (equal values each) and cause
 * gameport legacy I/O at 0x0200 to be _DISABLED_!
 * Is this Digital Enhanced Game Port Enable??? Or maybe it's Testmode
 * for Enhanced Digital Gameport (see 4D Wave DX card): */
pub const IO_6A_SOMETHING1_GAMEPORT: u32 = 0x0020;
/* bit 8; sure, this _pauses_ playback (later resumes at same spot!),
 * but what the heck is this really about??: */
pub const IO_6A_PAUSE_PLAYBACK_BIT8: u32 = 0x0100;
/* bit 9; sure, this _pauses_ playback (later resumes at same spot!),
 * but what the heck is this really about??: */
pub const IO_6A_PAUSE_PLAYBACK_BIT9: u32 = 0x0200;
/* BIT8 and BIT9 are _NOT_ able to affect OPL3 MIDI playback,
 * thus it suggests influence on PCM only!!
 * However OTOH there seems to be no bit anywhere around here
 * which is able to disable OPL3... */
/* bit 10: enabling this actually changes values at legacy gameport
 * I/O address (0x200); is this enabling of the Digital Enhanced Game Port???
 * Or maybe this simply switches off the NE558 circuit, since enabling this
 * still lets us evaluate button states, but not axis states */
pub const IO_6A_SOMETHING2_GAMEPORT: u32 = 0x0400;
/* writing 0x0300: causes quite some crackling during
 * PC activity such as switching windows (PCI traffic??
 * --> FIFO/timing settings???) */
/* writing 0x0100 plus/or 0x0200 inhibits playback */
/* since the Windows .INF file has Flag_Enable_JoyStick and
 * Flag_Enable_SB_DOS_Emulation directly together, it stands to reason
 * that some other bit in this same register might be responsible
 * for SB DOS Emulation activation (note that the file did NOT define
 * a switch for OPL3!) */
pub const IDX_IO_6CH: u32 = 0x6C; /* unknown; fully read-writable */
pub const IDX_IO_6EH: u32 = 0x6E;
/* writing 0xffff returns 0x83fe (or 0x03fe only).
 * writing 0x83 (and only 0x83!!) to 0x6f will cause 0x6c to switch
 * from 0000 to ffff. */

/* further I/O indices not saved/restored and not readable after writing,
 * so probably not used */

/*** Gameport area port indices ***/
/* (only 0x06 of 0x08 bytes saved/restored by Windows driver) */
pub const AZF_IO_SIZE_GAME: u32 = 0x08;
pub const AZF_IO_SIZE_GAME_PM: u32 = 0x06;

pub const AZF_GAME_LEGACY_IO_PORT: u32 = 0x200;

pub const IDX_GAME_LEGACY_COMPATIBLE: u32 = 0x00;
/* in some operation mode, writing anything to this port
 * triggers an interrupt:
 * yup, that's in case IDX_GAME_01H has one of the
 * axis measurement bits enabled
 * (and of course one needs to have GAME_HWCFG_IRQ_ENABLE, too) */

pub const IDX_GAME_AXES_CONFIG: u32 = 0x01;
/* NOTE: layout of this register awfully similar (read: "identical??")
 * to AD1815JS.pdf (p.29) */

/* enables axis 1 (X axis) measurement: */
pub const GAME_AXES_ENABLE_1: u32 = 0x01;
/* enables axis 2 (Y axis) measurement: */
pub const GAME_AXES_ENABLE_2: u32 = 0x02;
/* enables axis 3 (X axis) measurement: */
pub const GAME_AXES_ENABLE_3: u32 = 0x04;
/* enables axis 4 (Y axis) measurement: */
pub const GAME_AXES_ENABLE_4: u32 = 0x08;
/* selects the current axis to read the measured value of
 * (at IDX_GAME_AXIS_VALUE):
 * 00 = axis 1, 01 = axis 2, 10 = axis 3, 11 = axis 4: */
pub const GAME_AXES_READ_MASK: u32 = 0x30;
/* enable to have the latch continuously accept ADC values
 * (and continuously cause interrupts in case interrupts are enabled);
 * AD1815JS.pdf says it's ~16ms interval there: */
pub const GAME_AXES_LATCH_ENABLE: u32 = 0x40;
/* joystick data (measured axes) ready for reading: */
pub const GAME_AXES_SAMPLING_READY: u32 = 0x80;

/* NOTE: other card specs (SiS960 and others!) state that the
 * game position latches should be frozen when reading and be freed
 * (== reset?) after reading!!!
 * Freezing most likely means disabling 0x40 (GAME_AXES_LATCH_ENABLE),
 *  but how to free the value? */
/* An internet search for "gameport latch ADC" should provide some insight
 * into how to program such a gameport system. */

/* writing 0xf0 to 01H once reset both counters to 0, in some special mode!?
 * yup, in case 6AH 0x20 is not enabled
 * (and 0x40 is sufficient, 0xf0 is not needed) */

pub const IDX_GAME_AXIS_VALUE: u32 = 0x02;
/* R: value of currently configured axis (word value!);
 * W: trigger axis measurement */

pub const IDX_GAME_HWCONFIG: u32 = 0x04;
/* note: bits 4 to 7 are never set (== 0) when reading!
 * --> reserved bits? */
/* enables IRQ notification upon axes measurement ready: */
pub const GAME_HWCFG_IRQ_ENABLE: u32 = 0x01;
/* these bits choose a different frequency for the
 *  internal ADC counter increment.
 * hmm, seems to be a combo of bits:
 * 00 --> standard frequency
 * 10 --> 1/2
 * 01 --> 1/20
 * 11 --> 1/200: */
pub const GAME_HWCFG_ADC_COUNTER_FREQ_MASK: u32 = 0x06;

/* FIXME: these values might be reversed... */
pub const GAME_HWCFG_ADC_COUNTER_FREQ_STD: u32 = 0;
pub const GAME_HWCFG_ADC_COUNTER_FREQ_1_2: u32 = 1;
pub const GAME_HWCFG_ADC_COUNTER_FREQ_1_20: u32 = 2;
pub const GAME_HWCFG_ADC_COUNTER_FREQ_1_200: u32 = 3;

/* enable gameport legacy I/O address (0x200)
 * I was unable to locate any configurability for a different address: */
pub const GAME_HWCFG_LEGACY_ADDRESS_ENABLE: u32 = 0x08;

/*** MPU401 ***/
pub const AZF_IO_SIZE_MPU: u32 = 0x04;
pub const AZF_IO_SIZE_MPU_PM: u32 = 0x04;

/*** OPL3 synth ***/
/* (only 0x06 of 0x08 bytes saved/restored by Windows driver) */
pub const AZF_IO_SIZE_OPL3: u32 = 0x08;
pub const AZF_IO_SIZE_OPL3_PM: u32 = 0x06;
/* hmm, given that a standard OPL3 has 4 registers only,
 * there might be some enhanced functionality lurking at the end
 * (especially since register 0x04 has a "non-empty" value 0xfe) */

/*** mixer I/O area port indices ***/
/* (only 0x22 of 0x40 bytes saved/restored by Windows driver)
 * UNFORTUNATELY azf3328 is NOT truly AC97 compliant: see main file intro */
pub const AZF_IO_SIZE_MIXER: u32 = 0x40;
pub const AZF_IO_SIZE_MIXER_PM: u32 = 0x22;

pub const MIXER_VOLUME_RIGHT_MASK: u32 = 0x001f;
pub const MIXER_VOLUME_LEFT_MASK: u32 = 0x1f00;
pub const MIXER_MUTE_MASK: u32 = 0x8000;
pub const IDX_MIXER_RESET: u32 = 0x00; /* does NOT seem to have AC97 ID bits */
pub const IDX_MIXER_PLAY_MASTER: u32 = 0x02;
pub const IDX_MIXER_MODEMOUT: u32 = 0x04;
pub const IDX_MIXER_BASSTREBLE: u32 = 0x06;
pub const MIXER_BASSTREBLE_TREBLE_VOLUME_MASK: u32 = 0x000e;
pub const MIXER_BASSTREBLE_BASS_VOLUME_MASK: u32 = 0x0e00;
pub const IDX_MIXER_PCBEEP: u32 = 0x08;
pub const IDX_MIXER_MODEMIN: u32 = 0x0a;
pub const IDX_MIXER_MIC: u32 = 0x0c;
pub const MIXER_MIC_MICGAIN_20DB_ENHANCEMENT_MASK: u32 = 0x0040;
pub const IDX_MIXER_LINEIN: u32 = 0x0e;
pub const IDX_MIXER_CDAUDIO: u32 = 0x10;
pub const IDX_MIXER_VIDEO: u32 = 0x12;
pub const IDX_MIXER_AUX: u32 = 0x14;
pub const IDX_MIXER_WAVEOUT: u32 = 0x16;
pub const IDX_MIXER_FMSYNTH: u32 = 0x18;
pub const IDX_MIXER_REC_SELECT: u32 = 0x1a;
pub const MIXER_REC_SELECT_MIC: u32 = 0x00;
pub const MIXER_REC_SELECT_CD: u32 = 0x01;
pub const MIXER_REC_SELECT_VIDEO: u32 = 0x02;
pub const MIXER_REC_SELECT_AUX: u32 = 0x03;
pub const MIXER_REC_SELECT_LINEIN: u32 = 0x04;
pub const MIXER_REC_SELECT_MIXSTEREO: u32 = 0x05;
pub const MIXER_REC_SELECT_MIXMONO: u32 = 0x06;
pub const MIXER_REC_SELECT_MONOIN: u32 = 0x07;
pub const IDX_MIXER_REC_VOLUME: u32 = 0x1c;
pub const IDX_MIXER_ADVCTL1: u32 = 0x1e;
/* unlisted bits are unmodifiable */
pub const MIXER_ADVCTL1_3DWIDTH_MASK: u32 = 0x000e;
pub const MIXER_ADVCTL1_HIFI3D_MASK: u32 = 0x0300; /* yup, this is missing the high bit that official AC97 contains, plus it doesn't have linear bit value range behaviour but instead acts weirdly (possibly we're dealing with two *different* 3D settings here??) */
pub const IDX_MIXER_ADVCTL2: u32 = 0x20; /* subset of AC97_GENERAL_PURPOSE reg! */
/* unlisted bits are unmodifiable */
pub const MIXER_ADVCTL2_LPBK: u32 = 0x0080; /* Loopback mode -- Win driver: "WaveOut3DBypass"? mutes WaveOut at LineOut */
pub const MIXER_ADVCTL2_MS: u32 = 0x0100; /* Mic Select 0=Mic1, 1=Mic2 -- Win driver: "ModemOutSelect"?? */
pub const MIXER_ADVCTL2_MIX: u32 = 0x0200; /* Mono output select 0=Mix, 1=Mic; Win driver: "MonoSelectSource"?? */
pub const MIXER_ADVCTL2_3D: u32 = 0x2000; /* 3D Enhancement 1=on */
pub const MIXER_ADVCTL2_POP: u32 = 0x8000; /* Pcm Out Path, 0=pre 3D, 1=post 3D */

pub const IDX_MIXER_SOMETHING30H: u32 = 0x30; /* used, but unknown??? */

/* driver internal flags */
pub const SET_CHAN_LEFT: u32 = 1;
pub const SET_CHAN_RIGHT: u32 = 2;

/* helper macro to align I/O port ranges to 32bit I/O width */
pub const fn AZF_ALIGN(x: u32) -> u32 {
    (x + 3) & !3
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
