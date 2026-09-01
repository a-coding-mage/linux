/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 */

// Rust translation of pci/au88x0/au88x0.h.
// C header guards and include directives are omitted. The original header
// depends on Linux/ALSA headers plus au88x0_eq.h, au88x0_a3d.h, and
// au88x0_wt.h under the same CHIP_* conditions.

use core::ffi::{c_int, c_ulong, c_uint, c_ushort, c_void};

pub type u32 = u32;
pub type u16 = u16;
pub type u8 = u8;
pub type s8 = i8;

unsafe extern "C" {
    pub fn readl(addr: *const c_void) -> u32;
    pub fn writel(value: u32, addr: *mut c_void);
}

#[inline]
pub unsafe fn hwread(x: *const c_void, y: usize) -> u32 {
    unsafe { readl((x as *const u8).add(y) as *const c_void) }
}

#[inline]
pub unsafe fn hwwrite(x: *mut c_void, y: usize, z: u32) {
    unsafe { writel(z, (x as *mut u8).add(y) as *mut c_void) }
}

/* Vortex MPU401 defines. */
pub const MIDI_CLOCK_DIV: c_int = 0x61;
/* Standart MPU401 defines. */
pub const MPU401_RESET: c_int = 0xff;
pub const MPU401_ENTER_UART: c_int = 0x3f;
pub const MPU401_ACK: c_int = 0xfe;

// Get src register value to convert from x to y.
#[inline]
pub const fn SRC_RATIO(x: u32, y: u32) -> u32 {
    ((((x << 15) / y) + 1) / 2)
}

/* FIFO software state constants. */
pub const FIFO_STOP: c_int = 0;
pub const FIFO_START: c_int = 1;
pub const FIFO_PAUSE: c_int = 2;

/* IRQ flags */
pub const IRQ_ERR_MASK: c_int = 0x00ff;
pub const IRQ_FATAL: c_int = 0x0001;
pub const IRQ_PARITY: c_int = 0x0002;
pub const IRQ_REG: c_int = 0x0004;
pub const IRQ_FIFO: c_int = 0x0008;
pub const IRQ_DMA: c_int = 0x0010;
pub const IRQ_PCMOUT: c_int = 0x0020; /* PCM OUT page crossing */
pub const IRQ_TIMER: c_int = 0x1000;
pub const IRQ_MIDI: c_int = 0x2000;
pub const IRQ_MODEM: c_int = 0x4000;

/* ADB Resource */
pub const VORTEX_RESOURCE_DMA: c_int = 0x00000000;
pub const VORTEX_RESOURCE_SRC: c_int = 0x00000001;
pub const VORTEX_RESOURCE_MIXIN: c_int = 0x00000002;
pub const VORTEX_RESOURCE_MIXOUT: c_int = 0x00000003;
pub const VORTEX_RESOURCE_A3D: c_int = 0x00000004;
pub const VORTEX_RESOURCE_LAST: c_int = 0x00000005;

/* codec io: VORTEX_CODEC_IO bits */
pub const VORTEX_CODEC_ID_SHIFT: c_int = 24;
pub const VORTEX_CODEC_WRITE: c_int = 0x00800000;
pub const VORTEX_CODEC_ADDSHIFT: c_int = 16;
pub const VORTEX_CODEC_ADDMASK: c_int = 0x7f0000;
pub const VORTEX_CODEC_DATSHIFT: c_int = 0;
pub const VORTEX_CODEC_DATMASK: c_int = 0xffff;

/* Check for SDAC bit in "Extended audio ID" AC97 register */
// Original disabled macro:
// #define VORTEX_IS_QUAD(x) (((x)->codec == NULL) ?  0 : ((x)->codec->ext_id&0x80))
#[inline]
pub unsafe fn VORTEX_IS_QUAD(x: *const vortex_t) -> c_int {
    unsafe { (*x).isquad }
}

/* Check if chip has bug. */
#[inline]
pub unsafe fn IS_BAD_CHIP(x: *const vortex_t) -> bool {
    unsafe {
        ((*x).rev == 0xfe && (*x).device == PCI_DEVICE_ID_AUREAL_VORTEX_2)
            || ((*x).rev == 0xfe && (*x).device == PCI_DEVICE_ID_AUREAL_ADVANTAGE)
    }
}

/* PCM devices */
pub const VORTEX_PCM_ADB: usize = 0;
pub const VORTEX_PCM_SPDIF: usize = 1;
pub const VORTEX_PCM_A3D: usize = 2;
pub const VORTEX_PCM_WT: usize = 3;
pub const VORTEX_PCM_I2S: usize = 4;
pub const VORTEX_PCM_LAST: usize = 5;

#[inline]
pub unsafe fn MIX_CAPT(x: usize) -> s8 {
    unsafe { vortex.mixcapt[x] }
}

#[inline]
pub unsafe fn MIX_PLAYB(x: usize) -> s8 {
    unsafe { vortex.mixplayb[x] }
}

#[inline]
pub unsafe fn MIX_SPDIF(x: usize) -> s8 {
    unsafe { vortex.mixspdif[x] }
}

pub const NR_WTPB: usize = 0x20; /* WT channels per each bank. */
pub const NR_PCM: usize = 0x10;

#[repr(C)]
pub struct pcm_vol {
    pub kctl: *mut snd_kcontrol,
    pub active: c_int,
    pub dma: c_int,
    pub mixin: [c_int; 4],
    pub vol: [c_int; 4],
}

/* Structs */
#[repr(C)]
pub struct stream_t {
    // pub this_08: c_int,          /* Still unknown */
    pub fifo_enabled: c_int, /* this_24 */
    pub fifo_status: c_int,  /* this_1c */
    pub dma_ctrl: u32,       /* this_78 (ADB), this_7c (WT) */
    pub dma_unknown: c_int,  /* this_74 (ADB), this_78 (WT). WDM: +8 */
    pub cfg0: c_int,
    pub cfg1: c_int,

    pub nr_ch: c_int, /* Nr of PCM channels in use */
    pub type_: c_int, /* Output type (ac97, a3d, spdif, i2s, dsp) */
    pub dma: c_int,   /* Hardware DMA index. */
    pub dir: c_int,   /* Stream Direction. */
    pub resources: [u32; 5],

    /* Virtual page extender stuff */
    pub nr_periods: c_int,
    pub period_bytes: c_int,
    pub period_real: c_int,
    pub period_virt: c_int,

    pub substream: *mut snd_pcm_substream,
}

pub type vortex_t = snd_vortex;

#[repr(C)]
pub struct snd_vortex {
    /* ALSA structs. */
    pub card: *mut snd_card,
    pub pcm: [*mut snd_pcm; VORTEX_PCM_LAST],

    pub rmidi: *mut snd_rawmidi, /* Legacy Midi interface. */
    pub codec: *mut snd_ac97,

    /* Stream structs. */
    pub dma_adb: [stream_t; NR_ADB],
    pub spdif_sr: c_int,

    // Present when CHIP_AU8810 is not defined.
    pub dma_wt: [stream_t; NR_WT],
    pub wt_voice: [wt_voice_t; NR_WT], /* WT register cache. */
    pub mixwt: [s8; (NR_WT / NR_WTPB) * 6], /* WT mixin objects */

    /* Global resources */
    pub mixcapt: [s8; 2],
    pub mixplayb: [s8; 4],

    // Present when CHIP_AU8820 is not defined.
    pub mixspdif: [s8; 2],
    pub mixa3d: [s8; 2],  /* mixers which collect all a3d streams. */
    pub mixxtlk: [s8; 2], /* crosstalk canceler mixer inputs. */

    pub fixed_res: [u32; 5],

    // Present when CHIP_AU8820 is not defined.
    /* Hardware equalizer structs */
    pub eq: eqlzr_t,
    /* A3D structs */
    pub a3d: [a3dsrc_t; NR_A3D],
    /* Xtalk canceler */
    pub xt_mode: c_int, /* 1: speakers, 0:headphones. */

    pub pcm_vol: [pcm_vol; NR_PCM],

    pub isquad: c_int, /* cache of extended ID codec flag. */

    /* Gameport stuff. */
    pub gameport: *mut gameport,

    /* PCI hardware resources */
    pub io: c_ulong,
    pub mmio: *mut c_void, /* __iomem */
    pub irq: c_uint,
    pub lock: spinlock_t,

    /* PCI device */
    pub pci_dev: *mut pci_dev,
    pub vendor: u16,
    pub device: u16,
    pub rev: u8,
}

/* Functions. */
unsafe extern "C" {
    /* SRC */
    pub fn vortex_adb_setsrc(vortex: *mut vortex_t, adbdma: c_int, cvrt: c_uint, dir: c_int);

    /* DMA Engines. */
    pub fn vortex_adbdma_setbuffers(vortex: *mut vortex_t, adbdma: c_int, size: c_int, count: c_int);
    pub fn vortex_adbdma_setmode(
        vortex: *mut vortex_t,
        adbdma: c_int,
        ie: c_int,
        dir: c_int,
        fmt: c_int,
        d: c_int,
        offset: u32,
    );
    pub fn vortex_adbdma_setstartbuffer(vortex: *mut vortex_t, adbdma: c_int, sb: c_int);

    // Present when CHIP_AU8810 is not defined.
    pub fn vortex_wtdma_setbuffers(vortex: *mut vortex_t, wtdma: c_int, size: c_int, count: c_int);
    pub fn vortex_wtdma_setmode(
        vortex: *mut vortex_t,
        wtdma: c_int,
        ie: c_int,
        fmt: c_int,
        d: c_int,
        /* int e, */
        offset: u32,
    );
    pub fn vortex_wtdma_setstartbuffer(vortex: *mut vortex_t, wtdma: c_int, sb: c_int);

    pub fn vortex_adbdma_startfifo(vortex: *mut vortex_t, adbdma: c_int);
    // pub fn vortex_adbdma_stopfifo(vortex: *mut vortex_t, adbdma: c_int);
    pub fn vortex_adbdma_pausefifo(vortex: *mut vortex_t, adbdma: c_int);
    pub fn vortex_adbdma_resumefifo(vortex: *mut vortex_t, adbdma: c_int);
    pub fn vortex_adbdma_getlinearpos(vortex: *mut vortex_t, adbdma: c_int) -> c_int;
    pub fn vortex_adbdma_resetup(vortex: *mut vortex_t, adbdma: c_int);

    // Present when CHIP_AU8810 is not defined.
    pub fn vortex_wtdma_startfifo(vortex: *mut vortex_t, wtdma: c_int);
    pub fn vortex_wtdma_stopfifo(vortex: *mut vortex_t, wtdma: c_int);
    pub fn vortex_wtdma_pausefifo(vortex: *mut vortex_t, wtdma: c_int);
    pub fn vortex_wtdma_resumefifo(vortex: *mut vortex_t, wtdma: c_int);
    pub fn vortex_wtdma_getlinearpos(vortex: *mut vortex_t, wtdma: c_int) -> c_int;

    /* global stuff. */
    pub fn vortex_codec_init(vortex: *mut vortex_t);
    pub fn vortex_codec_write(codec: *mut snd_ac97, addr: c_ushort, data: c_ushort);
    pub fn vortex_codec_read(codec: *mut snd_ac97, addr: c_ushort) -> c_ushort;
    pub fn vortex_spdif_init(vortex: *mut vortex_t, spdif_sr: c_int, spdif_mode: c_int);

    pub fn vortex_core_init(card: *mut vortex_t) -> c_int;
    pub fn vortex_core_shutdown(card: *mut vortex_t) -> c_int;
    pub fn vortex_enable_int(card: *mut vortex_t);
    pub fn vortex_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
    pub fn vortex_alsafmt_aspfmt(alsafmt: snd_pcm_format_t, v: *mut vortex_t) -> c_int;

    /* Connection  stuff. */
    pub fn vortex_connect_default(vortex: *mut vortex_t, en: c_int);
    pub fn vortex_adb_allocroute(
        vortex: *mut vortex_t,
        dma: c_int,
        nr_ch: c_int,
        dir: c_int,
        type_: c_int,
        subdev: c_int,
    ) -> c_int;
    pub fn vortex_adb_checkinout(
        vortex: *mut vortex_t,
        resmap: *mut c_int,
        out: c_int,
        restype: c_int,
    ) -> c_int;

    // Present when CHIP_AU8810 is not defined.
    pub fn vortex_wt_allocroute(vortex: *mut vortex_t, dma: c_int, nr_ch: c_int) -> c_int;
    pub fn vortex_wt_connect(vortex: *mut vortex_t, en: c_int);
    pub fn vortex_wt_init(vortex: *mut vortex_t);

    pub fn vortex_route(
        vortex: *mut vortex_t,
        en: c_int,
        channel: u8,
        source: u8,
        dest: u8,
    );

    // Original #if 0 declaration preserved as a comment:
    // pub fn vortex_routes(
    //     vortex: *mut vortex_t,
    //     en: c_int,
    //     channel: u8,
    //     source: u8,
    //     dest0: u8,
    //     dest1: u8,
    // );

    pub fn vortex_connection_mixin_mix(
        vortex: *mut vortex_t,
        en: c_int,
        mixin: u8,
        mix: u8,
        a: c_int,
    );
    pub fn vortex_mix_setinputvolumebyte(vortex: *mut vortex_t, mix: u8, mixin: c_int, vol: u8);
    pub fn vortex_mix_setvolumebyte(vortex: *mut vortex_t, mix: u8, vol: u8);

    /* A3D functions. */
    // Present when CHIP_AU8820 is not defined.
    pub fn vortex_Vort3D_enable(v: *mut vortex_t);
    pub fn vortex_Vort3D_disable(v: *mut vortex_t);
    pub fn vortex_Vort3D_connect(vortex: *mut vortex_t, en: c_int);
    pub fn vortex_Vort3D_InitializeSource(a: *mut a3dsrc_t, en: c_int, v: *mut vortex_t);

    /* Driver stuff. */
    pub fn vortex_gameport_register(card: *mut vortex_t) -> c_int;
    pub fn vortex_gameport_unregister(card: *mut vortex_t);

    // Present when CHIP_AU8820 is not defined.
    pub fn vortex_eq_init(vortex: *mut vortex_t) -> c_int;
    pub fn vortex_eq_free(vortex: *mut vortex_t) -> c_int;

    /* ALSA stuff. */
    pub fn snd_vortex_new_pcm(vortex: *mut vortex_t, idx: c_int, nr: c_int) -> c_int;
    pub fn snd_vortex_mixer(vortex: *mut vortex_t) -> c_int;
    pub fn snd_vortex_midi(vortex: *mut vortex_t) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
