/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  audio@tridentmicro.com
 *  Fri Feb 19 15:55:28 MST 1999
 *  Definitions for Trident 4DWave DX/NX chips
 */

/* C header dependencies: sound/pcm.h, sound/mpu401.h, sound/ac97_codec.h,
 * sound/util_mem.h.
 */

pub const TRIDENT_DEVICE_ID_DX: u32 =
    ((PCI_VENDOR_ID_TRIDENT as u32) << 16) | (PCI_DEVICE_ID_TRIDENT_4DWAVE_DX as u32);
pub const TRIDENT_DEVICE_ID_NX: u32 =
    ((PCI_VENDOR_ID_TRIDENT as u32) << 16) | (PCI_DEVICE_ID_TRIDENT_4DWAVE_NX as u32);
pub const TRIDENT_DEVICE_ID_SI7018: u32 =
    ((PCI_VENDOR_ID_SI as u32) << 16) | (PCI_DEVICE_ID_SI_7018 as u32);

pub const SNDRV_TRIDENT_VOICE_TYPE_PCM: u32 = 0;
pub const SNDRV_TRIDENT_VOICE_TYPE_SYNTH: u32 = 1;
pub const SNDRV_TRIDENT_VOICE_TYPE_MIDI: u32 = 2;

pub const SNDRV_TRIDENT_VFLG_RUNNING: u32 = 1 << 0;

/* TLB code constants */
pub const SNDRV_TRIDENT_PAGE_SIZE: u32 = 4096;
pub const SNDRV_TRIDENT_PAGE_SHIFT: u32 = 12;
pub const SNDRV_TRIDENT_PAGE_MASK: u32 = (1 << SNDRV_TRIDENT_PAGE_SHIFT) - 1;
pub const SNDRV_TRIDENT_MAX_PAGES: u32 = 4096;

/*
 * Direct registers
 */

pub unsafe fn TRID_REG(trident: *const snd_trident, x: c_ulong) -> c_ulong {
    unsafe { (*trident).port.wrapping_add(x) }
}

pub const ID_4DWAVE_DX: u32 = 0x2000;
pub const ID_4DWAVE_NX: u32 = 0x2001;

/* Bank definitions */

pub const T4D_BANK_A: u32 = 0;
pub const T4D_BANK_B: u32 = 1;
pub const T4D_NUM_BANKS: u32 = 2;

/* Register definitions */

/* Global registers */

pub const CHANNEL_IDX: u32 = 0x0000003f;
pub const OVERRUN_IE: u32 = 0x00000400; /* interrupt enable: capture overrun */
pub const UNDERRUN_IE: u32 = 0x00000800; /* interrupt enable: playback underrun */
pub const ENDLP_IE: u32 = 0x00001000; /* interrupt enable: end of buffer */
pub const MIDLP_IE: u32 = 0x00002000; /* interrupt enable: middle buffer */
pub const ETOG_IE: u32 = 0x00004000; /* interrupt enable: envelope toggling */
pub const EDROP_IE: u32 = 0x00008000; /* interrupt enable: envelope drop */
pub const BANK_B_EN: u32 = 0x00010000; /* SiS: enable bank B (64 channels) */
pub const PCMIN_B_MIX: u32 = 0x00020000; /* SiS: PCM IN B mixing enable */
pub const I2S_OUT_ASSIGN: u32 = 0x00040000; /* SiS: I2S Out contains surround PCM */
pub const SPDIF_OUT_ASSIGN: u32 = 0x00080000; /* SiS: 0=S/PDIF L/R | 1=PCM Out FIFO */
pub const MAIN_OUT_ASSIGN: u32 = 0x00100000; /* SiS: 0=PCM Out FIFO | 1=MMC Out buffer */

pub const PB_UNDERRUN_IRQ: u32 = 0x00000001;
pub const REC_OVERRUN_IRQ: u32 = 0x00000002;
pub const SB_IRQ: u32 = 0x00000004;
pub const MPU401_IRQ: u32 = 0x00000008;
pub const OPL3_IRQ: u32 = 0x00000010;
pub const ADDRESS_IRQ: u32 = 0x00000020;
pub const ENVELOPE_IRQ: u32 = 0x00000040;
pub const PB_UNDERRUN: u32 = 0x00000100;
pub const REC_OVERRUN: u32 = 0x00000200;
pub const MIXER_UNDERFLOW: u32 = 0x00000400;
pub const MIXER_OVERFLOW: u32 = 0x00000800;
pub const NX_SB_IRQ_DISABLE: u32 = 0x00001000;
pub const ST_TARGET_REACHED: u32 = 0x00008000;
pub const PB_24K_MODE: u32 = 0x00010000;
pub const ST_IRQ_EN: u32 = 0x00800000;
pub const ACGPIO_IRQ: u32 = 0x01000000;

/* T2 legacy dma control registers. */
pub const LEGACY_DMAR0: u32 = 0x00; // ADR0
pub const LEGACY_DMAR4: u32 = 0x04; // CNT0
pub const LEGACY_DMAR6: u32 = 0x06; // CNT0 - High bits
pub const LEGACY_DMAR11: u32 = 0x0b; // MOD
pub const LEGACY_DMAR15: u32 = 0x0f; // MMR

pub const T4D_START_A: u32 = 0x80;
pub const T4D_STOP_A: u32 = 0x84;
pub const T4D_DLY_A: u32 = 0x88;
pub const T4D_SIGN_CSO_A: u32 = 0x8c;
pub const T4D_CSPF_A: u32 = 0x90;
pub const T4D_CSPF_B: u32 = 0xbc;
pub const T4D_CEBC_A: u32 = 0x94;
pub const T4D_AINT_A: u32 = 0x98;
pub const T4D_AINTEN_A: u32 = 0x9c;
pub const T4D_LFO_GC_CIR: u32 = 0xa0;
pub const T4D_MUSICVOL_WAVEVOL: u32 = 0xa8;
pub const T4D_SBDELTA_DELTA_R: u32 = 0xac;
pub const T4D_MISCINT: u32 = 0xb0;
pub const T4D_START_B: u32 = 0xb4;
pub const T4D_STOP_B: u32 = 0xb8;
pub const T4D_SBBL_SBCL: u32 = 0xc0;
pub const T4D_SBCTRL_SBE2R_SBDD: u32 = 0xc4;
pub const T4D_STIMER: u32 = 0xc8;
pub const T4D_AINT_B: u32 = 0xd8;
pub const T4D_AINTEN_B: u32 = 0xdc;
pub const T4D_RCI: u32 = 0x70;

/* MPU-401 UART */
pub const T4D_MPU401_BASE: u32 = 0x20;
pub const T4D_MPUR0: u32 = 0x20;
pub const T4D_MPUR1: u32 = 0x21;
pub const T4D_MPUR2: u32 = 0x22;
pub const T4D_MPUR3: u32 = 0x23;

/* S/PDIF Registers */
pub const NX_SPCTRL_SPCSO: u32 = 0x24;
pub const NX_SPLBA: u32 = 0x28;
pub const NX_SPESO: u32 = 0x2c;
pub const NX_SPCSTATUS: u32 = 0x64;

/* Joystick */
pub const GAMEPORT_GCR: u32 = 0x30;
pub const GAMEPORT_MODE_ADC: u32 = 0x80;
pub const GAMEPORT_LEGACY: u32 = 0x31;
pub const GAMEPORT_AXES: u32 = 0x34;

/* NX Specific Registers */
pub const NX_TLBC: u32 = 0x6c;

/* Channel Registers */

pub const CH_START: u32 = 0xe0;

pub const CH_DX_CSO_ALPHA_FMS: u32 = 0xe0;
pub const CH_DX_ESO_DELTA: u32 = 0xe8;
pub const CH_DX_FMC_RVOL_CVOL: u32 = 0xec;

pub const CH_NX_DELTA_CSO: u32 = 0xe0;
pub const CH_NX_DELTA_ESO: u32 = 0xe8;
pub const CH_NX_ALPHA_FMS_FMC_RVOL_CVOL: u32 = 0xec;

pub const CH_LBA: u32 = 0xe4;
pub const CH_GVSEL_PAN_VOL_CTRL_EC: u32 = 0xf0;
pub const CH_EBUF1: u32 = 0xf4;
pub const CH_EBUF2: u32 = 0xf8;

/* AC-97 Registers */

pub const DX_ACR0_AC97_W: u32 = 0x40;
pub const DX_ACR1_AC97_R: u32 = 0x44;
pub const DX_ACR2_AC97_COM_STAT: u32 = 0x48;

pub const NX_ACR0_AC97_COM_STAT: u32 = 0x40;
pub const NX_ACR1_AC97_W: u32 = 0x44;
pub const NX_ACR2_AC97_R_PRIMARY: u32 = 0x48;
pub const NX_ACR3_AC97_R_SECONDARY: u32 = 0x4c;

pub const SI_AC97_WRITE: u32 = 0x40;
pub const SI_AC97_READ: u32 = 0x44;
pub const SI_SERIAL_INTF_CTRL: u32 = 0x48;
pub const SI_AC97_GPIO: u32 = 0x4c;
pub const SI_ASR0: u32 = 0x50;
pub const SI_SPDIF_CS: u32 = 0x70;
pub const SI_GPIO: u32 = 0x7c;

pub const NX_AC97_BUSY_WRITE: u32 = 0x0800; /* ACR1-3 */
pub const NX_AC97_BUSY_READ: u32 = 0x0800;
pub const NX_AC97_BUSY_DATA: u32 = 0x0400;
pub const NX_AC97_WRITE_SECONDARY: u32 = 0x0100;
pub const NX_AC97_SECONDARY_READY: u32 = 0x0040; /* ACR0 */
pub const NX_AC97_SECONDARY_RECORD: u32 = 0x0020;
pub const NX_AC97_SURROUND_OUTPUT: u32 = 0x0010;
pub const NX_AC97_PRIMARY_READY: u32 = 0x0008;
pub const NX_AC97_PRIMARY_RECORD: u32 = 0x0004;
pub const NX_AC97_PCM_OUTPUT: u32 = 0x0002;
pub const NX_AC97_WARM_RESET: u32 = 0x0001;

pub const DX_AC97_BUSY_WRITE: u32 = 0x8000;
pub const DX_AC97_BUSY_READ: u32 = 0x8000;
pub const DX_AC97_READY: u32 = 0x0010;
pub const DX_AC97_RECORD: u32 = 0x0008;
pub const DX_AC97_PLAYBACK: u32 = 0x0002;

pub const SI_AC97_BUSY_WRITE: u32 = 0x00008000;
pub const SI_AC97_AUDIO_BUSY: u32 = 0x00004000;
pub const SI_AC97_MODEM_BUSY: u32 = 0x00002000;
pub const SI_AC97_BUSY_READ: u32 = 0x00008000;
pub const SI_AC97_SECONDARY: u32 = 0x00000080;

pub const WARM_RESET: u32 = 0x00000001;
pub const COLD_RESET: u32 = 0x00000002;
pub const I2S_CLOCK: u32 = 0x00000004;
pub const PCM_SEC_AC97: u32 = 0x00000008;
pub const AC97_DBL_RATE: u32 = 0x00000010;
pub const SPDIF_EN: u32 = 0x00000020;
pub const I2S_OUTPUT_EN: u32 = 0x00000040;
pub const I2S_INPUT_EN: u32 = 0x00000080;
pub const PCMIN: u32 = 0x00000100;
pub const LINE1IN: u32 = 0x00000200;
pub const MICIN: u32 = 0x00000400;
pub const LINE2IN: u32 = 0x00000800;
pub const HEAD_SET_IN: u32 = 0x00001000;
pub const GPIOIN: u32 = 0x00002000;
/* 7018 spec says id = 01 but the demo board routed to 10
   SECONDARY_ID= 0x00004000, */
pub const SECONDARY_ID: u32 = 0x00004000;
pub const PCMOUT: u32 = 0x00010000;
pub const SURROUT: u32 = 0x00020000;
pub const CENTEROUT: u32 = 0x00040000;
pub const LFEOUT: u32 = 0x00080000;
pub const LINE1OUT: u32 = 0x00100000;
pub const LINE2OUT: u32 = 0x00200000;
pub const GPIOOUT: u32 = 0x00400000;
pub const SI_AC97_PRIMARY_READY: u32 = 0x01000000;
pub const SI_AC97_SECONDARY_READY: u32 = 0x02000000;
pub const SI_AC97_POWERDOWN: u32 = 0x04000000;

/* PCM defaults */

pub const T4D_DEFAULT_PCM_VOL: u32 = 10; /* 0 - 255 */
pub const T4D_DEFAULT_PCM_PAN: u32 = 0; /* 0 - 127 */
pub const T4D_DEFAULT_PCM_RVOL: u32 = 127; /* 0 - 127 */
pub const T4D_DEFAULT_PCM_CVOL: u32 = 127; /* 0 - 127 */

#[repr(C)]
pub struct snd_trident_port {
    pub chset: *mut snd_midi_channel_set,
    pub trident: *mut snd_trident,
    pub mode: c_int,   /* operation mode */
    pub client: c_int, /* sequencer client number */
    pub port: c_int,   /* sequencer port number */
    pub midi_has_voices: c_uint,
}

#[repr(C)]
pub struct snd_trident_memblk_arg {
    pub first_page: c_short,
    pub last_page: c_short,
}

#[repr(C)]
pub struct snd_trident_tlb {
    pub entries: *mut __le32,             /* 16k-aligned TLB table */
    pub entries_dmaaddr: dma_addr_t,      /* 16k-aligned PCI address to TLB table */
    pub buffer: *mut snd_dma_buffer,
    pub memhdr: *mut snd_util_memhdr,     /* page allocation list */
    pub silent_page: *mut snd_dma_buffer,
}

#[repr(C)]
pub struct snd_trident_voice {
    pub number: c_uint,
    pub use_: c_uint,
    pub pcm: c_uint,
    pub synth: c_uint,
    pub midi: c_uint,
    pub flags: c_uint,
    pub client: c_uchar,
    pub port: c_uchar,
    pub index: c_uchar,

    pub sample_ops: *mut snd_trident_sample_ops,

    /* channel parameters */
    pub CSO: c_uint,       /* 24 bits (16 on DX) */
    pub ESO: c_uint,       /* 24 bits (16 on DX) */
    pub LBA: c_uint,       /* 30 bits */
    pub EC: c_ushort,      /* 12 bits */
    pub Alpha: c_ushort,   /* 12 bits */
    pub Delta: c_ushort,   /* 16 bits */
    pub Attribute: c_ushort, /* 16 bits - SiS 7018 */
    pub Vol: c_ushort,     /* 12 bits (6.6) */
    pub Pan: c_uchar,      /* 7 bits (1.4.2) */
    pub GVSel: c_uchar,    /* 1 bit */
    pub RVol: c_uchar,     /* 7 bits (5.2) */
    pub CVol: c_uchar,     /* 7 bits (5.2) */
    pub FMC: c_uchar,      /* 2 bits */
    pub CTRL: c_uchar,     /* 4 bits */
    pub FMS: c_uchar,      /* 4 bits */
    pub LFO: c_uchar,      /* 8 bits */

    pub negCSO: c_uint, /* nonzero - use negative CSO */

    pub memblk: *mut snd_util_memblk, /* memory block if TLB enabled */

    /* PCM data */

    pub trident: *mut snd_trident,
    pub substream: *mut snd_pcm_substream,
    pub extra: *mut snd_trident_voice, /* extra PCM voice (acts as interrupt generator) */
    pub running: c_uint,
    pub capture: c_uint,
    pub spdif: c_uint,
    pub foldback: c_uint,
    pub isync: c_uint,
    pub isync2: c_uint,
    pub isync3: c_uint,
    pub foldback_chan: c_int,          /* foldback subdevice number */
    pub stimer: c_uint,                /* global sample timer (to detect spurious interrupts) */
    pub spurious_threshold: c_uint,    /* spurious threshold */
    pub isync_mark: c_uint,
    pub isync_max: c_uint,
    pub isync_ESO: c_uint,

    /* --- */

    pub private_data: *mut c_void,
    pub private_free: Option<unsafe extern "C" fn(voice: *mut snd_trident_voice)>,
}

#[repr(C)]
pub struct snd_4dwave {
    pub seq_client: c_int,

    pub seq_ports: [snd_trident_port; 4],
    pub voices: [snd_trident_voice; 64],

    pub ChanSynthCount: c_int, /* number of allocated synth channels */
    pub max_size: c_int,       /* maximum synth memory size in bytes */
    pub current_size: c_int,   /* current allocated synth mem in bytes */
}

#[repr(C)]
pub struct snd_trident_pcm_mixer {
    pub voice: *mut snd_trident_voice, /* active voice */
    pub vol: c_ushort,                 /* front volume */
    pub pan: c_uchar,                  /* pan control */
    pub rvol: c_uchar,                 /* rear volume */
    pub cvol: c_uchar,                 /* center volume */
    pub pad: c_uchar,
}

#[repr(C)]
pub struct snd_trident {
    pub irq: c_int,

    pub device: c_uint, /* device ID */

    pub bDMAStart: c_uchar,

    pub port: c_ulong,
    pub midi_port: c_ulong,

    pub spurious_irq_count: c_uint,
    pub spurious_irq_max_delta: c_uint,

    pub tlb: snd_trident_tlb, /* TLB entries for NX cards */

    pub spdif_ctrl: c_uchar,
    pub spdif_pcm_ctrl: c_uchar,
    pub spdif_bits: c_uint,
    pub spdif_pcm_bits: c_uint,
    pub spdif_pcm_ctl: *mut snd_kcontrol, /* S/PDIF settings */
    pub ac97_ctrl: c_uint,

    pub ChanMap: [c_uint; 2], /* allocation map for hardware channels */

    pub ChanPCM: c_int,    /* max number of PCM channels */
    pub ChanPCMcnt: c_int, /* actual number of PCM channels */

    pub ac97_detect: c_uint, /* 1 = AC97 in detection phase */
    pub in_suspend: c_uint,  /* 1 during suspend/resume */

    pub synth: snd_4dwave, /* synth specific variables */

    pub event_lock: spinlock_t,
    pub voice_alloc: spinlock_t,

    pub dma_dev: snd_dma_device,

    pub pci: *mut pci_dev,
    pub card: *mut snd_card,
    pub pcm: *mut snd_pcm,      /* ADC/DAC PCM */
    pub foldback: *mut snd_pcm, /* Foldback PCM */
    pub spdif: *mut snd_pcm,    /* SPDIF PCM */
    pub rmidi: *mut snd_rawmidi,

    pub ac97_bus: *mut snd_ac97_bus,
    pub ac97: *mut snd_ac97,
    pub ac97_sec: *mut snd_ac97,

    pub musicvol_wavevol: c_uint,
    pub pcm_mixer: [snd_trident_pcm_mixer; 32],
    pub ctl_vol: *mut snd_kcontrol,  /* front volume */
    pub ctl_pan: *mut snd_kcontrol,  /* pan */
    pub ctl_rvol: *mut snd_kcontrol, /* rear volume */
    pub ctl_cvol: *mut snd_kcontrol, /* center volume */

    pub reg_lock: spinlock_t,

    pub gameport: *mut gameport,
}

unsafe extern "C" {
    pub fn snd_trident_create(
        card: *mut snd_card,
        pci: *mut pci_dev,
        pcm_streams: c_int,
        pcm_spdif_device: c_int,
        max_wavetable_size: c_int,
    ) -> c_int;
    pub fn snd_trident_create_gameport(trident: *mut snd_trident) -> c_int;

    pub fn snd_trident_pcm(trident: *mut snd_trident, device: c_int) -> c_int;
    pub fn snd_trident_foldback_pcm(trident: *mut snd_trident, device: c_int) -> c_int;
    pub fn snd_trident_spdif_pcm(trident: *mut snd_trident, device: c_int) -> c_int;
    pub fn snd_trident_alloc_voice(
        trident: *mut snd_trident,
        type_: c_int,
        client: c_int,
        port: c_int,
    ) -> *mut snd_trident_voice;
    pub fn snd_trident_free_voice(trident: *mut snd_trident, voice: *mut snd_trident_voice);
    pub fn snd_trident_start_voice(trident: *mut snd_trident, voice: c_uint);
    pub fn snd_trident_stop_voice(trident: *mut snd_trident, voice: c_uint);
    pub fn snd_trident_write_voice_regs(
        trident: *mut snd_trident,
        voice: *mut snd_trident_voice,
    );
    pub static snd_trident_pm: dev_pm_ops;

    /* TLB memory allocation */
    pub fn snd_trident_alloc_pages(
        trident: *mut snd_trident,
        substream: *mut snd_pcm_substream,
    ) -> *mut snd_util_memblk;
    pub fn snd_trident_free_pages(trident: *mut snd_trident, blk: *mut snd_util_memblk) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
