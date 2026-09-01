/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  Copyright (c) by Jaroslav Kysela <perex@perex.cz>
 *  Definitions for Yahama YMF724/740/744/754 chips
 */

use core::ffi::{c_int, c_ulong, c_uchar, c_uint, c_void};

pub type __le32 = u32;
pub type dma_addr_t = usize;
pub type u16 = u16;
pub type u32 = u32;

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_dma_buffer {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_ac97_bus {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_ac97 {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_rawmidi {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_timer {
    _private: [u8; 0],
}
#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_card {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}
#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct atomic_t {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_info_entry {
    _private: [u8; 0],
}
#[repr(C)]
pub struct firmware {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

/* Direct registers */

/* C macro intent: YMFREG(chip, reg) expands to (chip->port + YDSXGR_##reg). */

pub const YDSXGR_INTFLAG: c_int = 0x0004;
pub const YDSXGR_ACTIVITY: c_int = 0x0006;
pub const YDSXGR_GLOBALCTRL: c_int = 0x0008;
pub const YDSXGR_ZVCTRL: c_int = 0x000A;
pub const YDSXGR_TIMERCTRL: c_int = 0x0010;
pub const YDSXGR_TIMERCOUNT: c_int = 0x0012;
pub const YDSXGR_SPDIFOUTCTRL: c_int = 0x0018;
pub const YDSXGR_SPDIFOUTSTATUS: c_int = 0x001C;
pub const YDSXGR_EEPROMCTRL: c_int = 0x0020;
pub const YDSXGR_SPDIFINCTRL: c_int = 0x0034;
pub const YDSXGR_SPDIFINSTATUS: c_int = 0x0038;
pub const YDSXGR_DSPPROGRAMDL: c_int = 0x0048;
pub const YDSXGR_DLCNTRL: c_int = 0x004C;
pub const YDSXGR_GPIOININTFLAG: c_int = 0x0050;
pub const YDSXGR_GPIOININTENABLE: c_int = 0x0052;
pub const YDSXGR_GPIOINSTATUS: c_int = 0x0054;
pub const YDSXGR_GPIOOUTCTRL: c_int = 0x0056;
pub const YDSXGR_GPIOFUNCENABLE: c_int = 0x0058;
pub const YDSXGR_GPIOTYPECONFIG: c_int = 0x005A;
pub const YDSXGR_AC97CMDDATA: c_int = 0x0060;
pub const YDSXGR_AC97CMDADR: c_int = 0x0062;
pub const YDSXGR_PRISTATUSDATA: c_int = 0x0064;
pub const YDSXGR_PRISTATUSADR: c_int = 0x0066;
pub const YDSXGR_SECSTATUSDATA: c_int = 0x0068;
pub const YDSXGR_SECSTATUSADR: c_int = 0x006A;
pub const YDSXGR_SECCONFIG: c_int = 0x0070;
pub const YDSXGR_LEGACYOUTVOL: c_int = 0x0080;
pub const YDSXGR_LEGACYOUTVOLL: c_int = 0x0080;
pub const YDSXGR_LEGACYOUTVOLR: c_int = 0x0082;
pub const YDSXGR_NATIVEDACOUTVOL: c_int = 0x0084;
pub const YDSXGR_NATIVEDACOUTVOLL: c_int = 0x0084;
pub const YDSXGR_NATIVEDACOUTVOLR: c_int = 0x0086;
pub const YDSXGR_ZVOUTVOL: c_int = 0x0088;
pub const YDSXGR_ZVOUTVOLL: c_int = 0x0088;
pub const YDSXGR_ZVOUTVOLR: c_int = 0x008A;
pub const YDSXGR_SECADCOUTVOL: c_int = 0x008C;
pub const YDSXGR_SECADCOUTVOLL: c_int = 0x008C;
pub const YDSXGR_SECADCOUTVOLR: c_int = 0x008E;
pub const YDSXGR_PRIADCOUTVOL: c_int = 0x0090;
pub const YDSXGR_PRIADCOUTVOLL: c_int = 0x0090;
pub const YDSXGR_PRIADCOUTVOLR: c_int = 0x0092;
pub const YDSXGR_LEGACYLOOPVOL: c_int = 0x0094;
pub const YDSXGR_LEGACYLOOPVOLL: c_int = 0x0094;
pub const YDSXGR_LEGACYLOOPVOLR: c_int = 0x0096;
pub const YDSXGR_NATIVEDACLOOPVOL: c_int = 0x0098;
pub const YDSXGR_NATIVEDACLOOPVOLL: c_int = 0x0098;
pub const YDSXGR_NATIVEDACLOOPVOLR: c_int = 0x009A;
pub const YDSXGR_ZVLOOPVOL: c_int = 0x009C;
pub const YDSXGR_ZVLOOPVOLL: c_int = 0x009E;
pub const YDSXGR_ZVLOOPVOLR: c_int = 0x009E;
pub const YDSXGR_SECADCLOOPVOL: c_int = 0x00A0;
pub const YDSXGR_SECADCLOOPVOLL: c_int = 0x00A0;
pub const YDSXGR_SECADCLOOPVOLR: c_int = 0x00A2;
pub const YDSXGR_PRIADCLOOPVOL: c_int = 0x00A4;
pub const YDSXGR_PRIADCLOOPVOLL: c_int = 0x00A4;
pub const YDSXGR_PRIADCLOOPVOLR: c_int = 0x00A6;
pub const YDSXGR_NATIVEADCINVOL: c_int = 0x00A8;
pub const YDSXGR_NATIVEADCINVOLL: c_int = 0x00A8;
pub const YDSXGR_NATIVEADCINVOLR: c_int = 0x00AA;
pub const YDSXGR_NATIVEDACINVOL: c_int = 0x00AC;
pub const YDSXGR_NATIVEDACINVOLL: c_int = 0x00AC;
pub const YDSXGR_NATIVEDACINVOLR: c_int = 0x00AE;
pub const YDSXGR_BUF441OUTVOL: c_int = 0x00B0;
pub const YDSXGR_BUF441OUTVOLL: c_int = 0x00B0;
pub const YDSXGR_BUF441OUTVOLR: c_int = 0x00B2;
pub const YDSXGR_BUF441LOOPVOL: c_int = 0x00B4;
pub const YDSXGR_BUF441LOOPVOLL: c_int = 0x00B4;
pub const YDSXGR_BUF441LOOPVOLR: c_int = 0x00B6;
pub const YDSXGR_SPDIFOUTVOL: c_int = 0x00B8;
pub const YDSXGR_SPDIFOUTVOLL: c_int = 0x00B8;
pub const YDSXGR_SPDIFOUTVOLR: c_int = 0x00BA;
pub const YDSXGR_SPDIFLOOPVOL: c_int = 0x00BC;
pub const YDSXGR_SPDIFLOOPVOLL: c_int = 0x00BC;
pub const YDSXGR_SPDIFLOOPVOLR: c_int = 0x00BE;
pub const YDSXGR_ADCSLOTSR: c_int = 0x00C0;
pub const YDSXGR_RECSLOTSR: c_int = 0x00C4;
pub const YDSXGR_ADCFORMAT: c_int = 0x00C8;
pub const YDSXGR_RECFORMAT: c_int = 0x00CC;
pub const YDSXGR_P44SLOTSR: c_int = 0x00D0;
pub const YDSXGR_STATUS: c_int = 0x0100;
pub const YDSXGR_CTRLSELECT: c_int = 0x0104;
pub const YDSXGR_MODE: c_int = 0x0108;
pub const YDSXGR_SAMPLECOUNT: c_int = 0x010C;
pub const YDSXGR_NUMOFSAMPLES: c_int = 0x0110;
pub const YDSXGR_CONFIG: c_int = 0x0114;
pub const YDSXGR_PLAYCTRLSIZE: c_int = 0x0140;
pub const YDSXGR_RECCTRLSIZE: c_int = 0x0144;
pub const YDSXGR_EFFCTRLSIZE: c_int = 0x0148;
pub const YDSXGR_WORKSIZE: c_int = 0x014C;
pub const YDSXGR_MAPOFREC: c_int = 0x0150;
pub const YDSXGR_MAPOFEFFECT: c_int = 0x0154;
pub const YDSXGR_PLAYCTRLBASE: c_int = 0x0158;
pub const YDSXGR_RECCTRLBASE: c_int = 0x015C;
pub const YDSXGR_EFFCTRLBASE: c_int = 0x0160;
pub const YDSXGR_WORKBASE: c_int = 0x0164;
pub const YDSXGR_DSPINSTRAM: c_int = 0x1000;
pub const YDSXGR_CTRLINSTRAM: c_int = 0x4000;

pub const YDSXG_AC97READCMD: c_int = 0x8000;
pub const YDSXG_AC97WRITECMD: c_int = 0x0000;

pub const PCIR_DSXG_LEGACY: c_int = 0x40;
pub const PCIR_DSXG_ELEGACY: c_int = 0x42;
pub const PCIR_DSXG_CTRL: c_int = 0x48;
pub const PCIR_DSXG_PWRCTRL1: c_int = 0x4a;
pub const PCIR_DSXG_PWRCTRL2: c_int = 0x4e;
pub const PCIR_DSXG_FMBASE: c_int = 0x60;
pub const PCIR_DSXG_SBBASE: c_int = 0x62;
pub const PCIR_DSXG_MPU401BASE: c_int = 0x64;
pub const PCIR_DSXG_JOYBASE: c_int = 0x66;

pub const YDSXG_DSPLENGTH: c_int = 0x0080;
pub const YDSXG_CTRLLENGTH: c_int = 0x3000;

pub const YDSXG_DEFAULT_WORK_SIZE: c_int = 0x0400;

pub const YDSXG_PLAYBACK_VOICES: usize = 64;
pub const YDSXG_CAPTURE_VOICES: usize = 2;
pub const YDSXG_EFFECT_VOICES: usize = 5;

pub const YMFPCI_LEGACY_SBEN: c_int = 1 << 0; /* soundblaster enable */
pub const YMFPCI_LEGACY_FMEN: c_int = 1 << 1; /* OPL3 enable */
pub const YMFPCI_LEGACY_JPEN: c_int = 1 << 2; /* joystick enable */
pub const YMFPCI_LEGACY_MEN: c_int = 1 << 3; /* MPU401 enable */
pub const YMFPCI_LEGACY_MIEN: c_int = 1 << 4; /* MPU RX irq enable */
pub const YMFPCI_LEGACY_IOBITS: c_int = 1 << 5; /* i/o bits range, 0 = 16bit, 1 =10bit */
pub const YMFPCI_LEGACY_SDMA: c_int = 3 << 6; /* SB DMA select */
pub const YMFPCI_LEGACY_SBIRQ: c_int = 7 << 8; /* SB IRQ select */
pub const YMFPCI_LEGACY_MPUIRQ: c_int = 7 << 11; /* MPU IRQ select */
pub const YMFPCI_LEGACY_SIEN: c_int = 1 << 14; /* serialized IRQ */
pub const YMFPCI_LEGACY_LAD: c_int = 1 << 15; /* legacy audio disable */

pub const YMFPCI_LEGACY2_FMIO: c_int = 3 << 0; /* OPL3 i/o address (724/740) */
pub const YMFPCI_LEGACY2_SBIO: c_int = 3 << 2; /* SB i/o address (724/740) */
pub const YMFPCI_LEGACY2_MPUIO: c_int = 3 << 4; /* MPU401 i/o address (724/740) */
pub const YMFPCI_LEGACY2_JSIO: c_int = 3 << 6; /* joystick i/o address (724/740) */
pub const YMFPCI_LEGACY2_MAIM: c_int = 1 << 8; /* MPU401 ack intr mask */
pub const YMFPCI_LEGACY2_SMOD: c_int = 3 << 11; /* SB DMA mode */
pub const YMFPCI_LEGACY2_SBVER: c_int = 3 << 13; /* SB version select */
pub const YMFPCI_LEGACY2_IMOD: c_int = 1 << 15; /* legacy IRQ mode */
/* SIEN:IMOD 0:0 = legacy irq, 0:1 = INTA, 1:0 = serialized IRQ */

/* C conditional intent: SUPPORT_JOYSTICK is defined when IS_REACHABLE(CONFIG_GAMEPORT). */

#[repr(C)]
pub struct snd_ymfpci_playback_bank {
    pub format: __le32,
    pub loop_default: __le32,
    pub base: __le32,       /* 32-bit address */
    pub loop_start: __le32, /* 32-bit offset */
    pub loop_end: __le32,   /* 32-bit offset */
    pub loop_frac: __le32,  /* 8-bit fraction - loop_start */
    pub delta_end: __le32,  /* pitch delta end */
    pub lpfK_end: __le32,
    pub eg_gain_end: __le32,
    pub left_gain_end: __le32,
    pub right_gain_end: __le32,
    pub eff1_gain_end: __le32,
    pub eff2_gain_end: __le32,
    pub eff3_gain_end: __le32,
    pub lpfQ: __le32,
    pub status: __le32,
    pub num_of_frames: __le32,
    pub loop_count: __le32,
    pub start: __le32,
    pub start_frac: __le32,
    pub delta: __le32,
    pub lpfK: __le32,
    pub eg_gain: __le32,
    pub left_gain: __le32,
    pub right_gain: __le32,
    pub eff1_gain: __le32,
    pub eff2_gain: __le32,
    pub eff3_gain: __le32,
    pub lpfD1: __le32,
    pub lpfD2: __le32,
}

#[repr(C)]
pub struct snd_ymfpci_capture_bank {
    pub base: __le32,        /* 32-bit address */
    pub loop_end: __le32,    /* 32-bit offset */
    pub start: __le32,       /* 32-bit offset */
    pub num_of_loops: __le32, /* counter */
}

#[repr(C)]
pub struct snd_ymfpci_effect_bank {
    pub base: __le32,     /* 32-bit address */
    pub loop_end: __le32, /* 32-bit offset */
    pub start: __le32,    /* 32-bit offset */
    pub temp: __le32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum snd_ymfpci_voice_type {
    YMFPCI_PCM,
    YMFPCI_SYNTH,
    YMFPCI_MIDI,
}

#[repr(C)]
pub struct snd_ymfpci_voice {
    pub chip: *mut snd_ymfpci,
    pub number: c_int,
    pub flags: c_uint, /* use:1, pcm:1, synth:1, midi:1 */
    pub bank: *mut snd_ymfpci_playback_bank,
    pub bank_addr: dma_addr_t,
    pub interrupt: Option<unsafe extern "C" fn(chip: *mut snd_ymfpci, voice: *mut snd_ymfpci_voice)>,
    pub ypcm: *mut snd_ymfpci_pcm,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum snd_ymfpci_pcm_type {
    PLAYBACK_VOICE,
    CAPTURE_REC,
    CAPTURE_AC97,
    EFFECT_DRY_LEFT,
    EFFECT_DRY_RIGHT,
    EFFECT_EFF1,
    EFFECT_EFF2,
    EFFECT_EFF3,
}

#[repr(C)]
pub struct snd_ymfpci_pcm {
    pub chip: *mut snd_ymfpci,
    pub type_: snd_ymfpci_pcm_type,
    pub substream: *mut snd_pcm_substream,
    pub voices: [*mut snd_ymfpci_voice; 2], /* playback only */
    pub flags: c_uint, /* running:1, use_441_slot:1, output_front:1, output_rear:1, swap_rear:1 */
    pub update_pcm_vol: c_uint,
    pub period_size: u32, /* cached from runtime->period_size */
    pub buffer_size: u32, /* cached from runtime->buffer_size */
    pub period_pos: u32,
    pub last_pos: u32,
    pub capture_bank_number: u32,
    pub shift: u32,
}

pub const saved_regs_index: [c_int; 21] = [
    /* spdif */
    YDSXGR_SPDIFOUTCTRL,
    YDSXGR_SPDIFOUTSTATUS,
    YDSXGR_SPDIFINCTRL,
    /* volumes */
    YDSXGR_PRIADCLOOPVOL,
    YDSXGR_NATIVEDACINVOL,
    YDSXGR_NATIVEDACOUTVOL,
    YDSXGR_BUF441OUTVOL,
    YDSXGR_NATIVEADCINVOL,
    YDSXGR_SPDIFLOOPVOL,
    YDSXGR_SPDIFOUTVOL,
    YDSXGR_ZVOUTVOL,
    YDSXGR_LEGACYOUTVOL,
    /* address bases */
    YDSXGR_PLAYCTRLBASE,
    YDSXGR_RECCTRLBASE,
    YDSXGR_EFFCTRLBASE,
    YDSXGR_WORKBASE,
    /* capture set up */
    YDSXGR_MAPOFREC,
    YDSXGR_RECFORMAT,
    YDSXGR_RECSLOTSR,
    YDSXGR_ADCFORMAT,
    YDSXGR_ADCSLOTSR,
];
pub const YDSXGR_NUM_SAVED_REGS: usize = saved_regs_index.len();

pub const pci_saved_regs_index: [c_int; 6] = [
    /* All Chips */
    PCIR_DSXG_LEGACY,
    PCIR_DSXG_ELEGACY,
    /* YMF 744/754 */
    PCIR_DSXG_FMBASE,
    PCIR_DSXG_SBBASE,
    PCIR_DSXG_MPU401BASE,
    PCIR_DSXG_JOYBASE,
];
pub const DSXG_PCI_NUM_SAVED_REGS: usize = pci_saved_regs_index.len();
pub const DSXG_PCI_NUM_SAVED_LEGACY_REGS: usize = 2;
const _: () = assert!(DSXG_PCI_NUM_SAVED_LEGACY_REGS <= DSXG_PCI_NUM_SAVED_REGS);

#[repr(C)]
pub struct snd_ymfpci_pcm_mixer {
    pub left: u16,
    pub right: u16,
    pub ctl: *mut snd_kcontrol,
}

#[repr(C)]
pub struct snd_ymfpci {
    pub irq: c_int,

    pub device_id: c_uint, /* PCI device ID */
    pub rev: c_uchar,      /* PCI revision */
    pub reg_area_phys: c_ulong,
    pub reg_area_virt: *mut c_void,

    pub old_legacy_ctrl: u16,
    /* C conditional field: struct gameport *gameport when SUPPORT_JOYSTICK is defined. */

    pub work_ptr: *mut snd_dma_buffer,

    pub bank_size_playback: c_uint,
    pub bank_size_capture: c_uint,
    pub bank_size_effect: c_uint,
    pub work_size: c_uint,

    pub bank_base_playback: *mut c_void,
    pub bank_base_capture: *mut c_void,
    pub bank_base_effect: *mut c_void,
    pub work_base: *mut c_void,
    pub bank_base_playback_addr: dma_addr_t,
    pub bank_base_capture_addr: dma_addr_t,
    pub bank_base_effect_addr: dma_addr_t,
    pub work_base_addr: dma_addr_t,
    pub ac3_tmp_base: snd_dma_buffer,

    pub ctrl_playback: *mut __le32,
    pub bank_playback: [[*mut snd_ymfpci_playback_bank; 2]; YDSXG_PLAYBACK_VOICES],
    pub bank_capture: [[*mut snd_ymfpci_capture_bank; 2]; YDSXG_CAPTURE_VOICES],
    pub bank_effect: [[*mut snd_ymfpci_effect_bank; 2]; YDSXG_EFFECT_VOICES],

    pub start_count: c_int,

    pub active_bank: u32,
    pub voices: [snd_ymfpci_voice; 64],
    pub src441_used: c_int,

    pub ac97_bus: *mut snd_ac97_bus,
    pub ac97: *mut snd_ac97,
    pub rawmidi: *mut snd_rawmidi,
    pub timer: *mut snd_timer,
    pub timer_ticks: c_uint,

    pub pci: *mut pci_dev,
    pub card: *mut snd_card,
    pub pcm: *mut snd_pcm,
    pub pcm2: *mut snd_pcm,
    pub pcm_spdif: *mut snd_pcm,
    pub pcm_4ch: *mut snd_pcm,
    pub capture_substream: [*mut snd_pcm_substream; YDSXG_CAPTURE_VOICES],
    pub effect_substream: [*mut snd_pcm_substream; YDSXG_EFFECT_VOICES],
    pub ctl_vol_recsrc: *mut snd_kcontrol,
    pub ctl_vol_adcrec: *mut snd_kcontrol,
    pub ctl_vol_spdifrec: *mut snd_kcontrol,
    pub spdif_bits: u16,
    pub spdif_pcm_bits: u16,
    pub spdif_pcm_ctl: *mut snd_kcontrol,
    pub mode_dup4ch: c_int,
    pub rear_opened: c_int,
    pub spdif_opened: c_int,
    pub pcm_mixer: [snd_ymfpci_pcm_mixer; 32],

    pub reg_lock: spinlock_t,
    pub voice_lock: spinlock_t,
    pub interrupt_sleep: wait_queue_head_t,
    pub interrupt_sleep_count: atomic_t,
    pub proc_entry: *mut snd_info_entry,
    pub dsp_microcode: *const firmware,
    pub controller_microcode: *const firmware,

    pub saved_regs: [u32; YDSXGR_NUM_SAVED_REGS],
    pub saved_ydsxgr_mode: u32,
    pub saved_dsxg_pci_regs: [u16; DSXG_PCI_NUM_SAVED_REGS],
}

unsafe extern "C" {
    pub fn snd_ymfpci_create(card: *mut snd_card, pci: *mut pci_dev, old_legacy_ctrl: u16) -> c_int;
    pub fn snd_ymfpci_free_gameport(chip: *mut snd_ymfpci);

    pub static snd_ymfpci_pm: dev_pm_ops;

    pub fn snd_ymfpci_pcm(chip: *mut snd_ymfpci, device: c_int) -> c_int;
    pub fn snd_ymfpci_pcm2(chip: *mut snd_ymfpci, device: c_int) -> c_int;
    pub fn snd_ymfpci_pcm_spdif(chip: *mut snd_ymfpci, device: c_int) -> c_int;
    pub fn snd_ymfpci_pcm_4ch(chip: *mut snd_ymfpci, device: c_int) -> c_int;
    pub fn snd_ymfpci_mixer(chip: *mut snd_ymfpci, rear_switch: c_int) -> c_int;
    pub fn snd_ymfpci_timer(chip: *mut snd_ymfpci, device: c_int) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
