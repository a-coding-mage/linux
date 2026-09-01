// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Driver for PowerMac onboard soundchips
 * Copyright (c) 2001 by Takashi Iwai <tiwai@suse.de>
 *   based on dmasound.c.
 */

// C header dependencies:
// <sound/control.h>, <sound/pcm.h>, "awacs.h", <linux/adb.h>,
// optionally <linux/cuda.h> when CONFIG_ADB_CUDA is enabled,
// optionally <linux/pmu.h> when CONFIG_ADB_PMU is enabled,
// <linux/nvram.h>, <linux/tty.h>, <linux/vt_kern.h>, <asm/dbdma.h>,
// <asm/prom.h>, <asm/machdep.h>, <asm/pmac_feature.h>.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// External kernel/ALSA types supplied by included headers.
pub type dma_addr_t = usize;
pub type spinlock_t = c_void;
pub type resource = c_void;
pub type dbdma_cmd = c_void;
pub type dbdma_regs = c_void;
pub type snd_card = c_void;
pub type device_node = c_void;
pub type pci_dev = c_void;
pub type snd_pcm_substream = c_void;
pub type awacs_regs = c_void;
pub type snd_pcm = c_void;
pub type pmac_beep = c_void;
pub type snd_kcontrol = c_void;
pub type i2c_client = c_void;

/* maximum number of fragments */
pub const PMAC_MAX_FRAGS: c_int = 32;

// PMAC_SUPPORT_AUTOMUTE is defined by this header.
pub const PMAC_SUPPORT_AUTOMUTE: bool = true;

/*
 * DBDMA space
 */
#[repr(C)]
pub struct pmac_dbdma {
    pub dma_base: dma_addr_t,
    pub addr: dma_addr_t,
    pub cmds: *mut dbdma_cmd,
    pub space: *mut c_void,
    pub size: c_int,
}

/*
 * playback/capture stream
 */
#[repr(C)]
pub struct pmac_stream {
    pub running: c_int, /* boolean */

    pub stream: c_int, /* PLAYBACK/CAPTURE */

    pub dma_size: c_int,    /* in bytes */
    pub period_size: c_int, /* in bytes */
    pub buffer_size: c_int, /* in kbytes */
    pub nperiods: c_int,
    pub cur_period: c_int,

    pub cmd: pmac_dbdma,
    pub dma: *mut dbdma_regs,

    pub substream: *mut snd_pcm_substream,

    pub cur_freqs: c_uint,   /* currently available frequencies */
    pub cur_formats: c_uint, /* currently available formats */
}

/*
 */

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum snd_pmac_model {
    PMAC_AWACS,
    PMAC_SCREAMER,
    PMAC_BURGUNDY,
    PMAC_DACA,
    PMAC_TUMBLER,
    PMAC_SNAPPER,
}

#[repr(C)]
pub struct snd_pmac {
    pub card: *mut snd_card,

    /* h/w info */
    pub node: *mut device_node,
    pub pdev: *mut pci_dev,
    pub revision: c_uint,
    pub manufacturer: c_uint,
    pub subframe: c_uint,
    pub device_id: c_uint,
    pub model: snd_pmac_model,

    // C bitfields packed into an unsigned int storage unit:
    // has_iic:1, is_pbook_3400:1, is_pbook_G3:1, is_k2:1,
    // can_byte_swap:1, can_duplex:1, can_capture:1,
    // auto_mute:1, initialized:1, feature_is_set:1.
    pub flags: c_uint,

    pub requested: c_uint,
    pub rsrc: [resource; 3],

    pub num_freqs: c_int,
    pub freq_table: *const c_int,
    pub freqs_ok: c_uint,   /* bit flags */
    pub formats_ok: c_uint, /* pcm hwinfo */
    pub active: c_int,
    pub rate_index: c_int,
    pub format: c_int, /* current format */

    pub reg_lock: spinlock_t,
    pub awacs: *mut awacs_regs,
    pub awacs_reg: [c_int; 8], /* register cache */
    pub hp_stat_mask: c_uint,

    pub latch_base: *mut u8,
    pub macio_base: *mut u8,

    pub playback: pmac_stream,
    pub capture: pmac_stream,

    pub extra_dma: pmac_dbdma,

    pub irq: c_int,
    pub tx_irq: c_int,
    pub rx_irq: c_int,

    pub pcm: *mut snd_pcm,

    pub beep: *mut pmac_beep,

    pub control_mask: c_uint, /* control mask */

    /* mixer stuffs */
    pub mixer_data: *mut c_void,
    pub mixer_free: Option<unsafe extern "C" fn(chip: *mut snd_pmac)>,
    pub master_sw_ctl: *mut snd_kcontrol,
    pub speaker_sw_ctl: *mut snd_kcontrol,
    pub drc_sw_ctl: *mut snd_kcontrol, /* only used for tumbler -ReneR */
    pub hp_detect_ctl: *mut snd_kcontrol,
    pub lineout_sw_ctl: *mut snd_kcontrol,

    /* lowlevel callbacks */
    pub set_format: Option<unsafe extern "C" fn(chip: *mut snd_pmac)>,
    pub update_automute: Option<unsafe extern "C" fn(chip: *mut snd_pmac, do_notify: c_int)>,
    pub detect_headphone: Option<unsafe extern "C" fn(chip: *mut snd_pmac) -> c_int>,

    // Present when CONFIG_PM is enabled in the original C build.
    pub suspend: Option<unsafe extern "C" fn(chip: *mut snd_pmac)>,
    pub resume: Option<unsafe extern "C" fn(chip: *mut snd_pmac)>,
}

/* exported functions */
unsafe extern "C" {
    pub fn snd_pmac_new(card: *mut snd_card, chip_return: *mut *mut snd_pmac) -> c_int;
    pub fn snd_pmac_pcm_new(chip: *mut snd_pmac) -> c_int;
    pub fn snd_pmac_attach_beep(chip: *mut snd_pmac) -> c_int;
    pub fn snd_pmac_detach_beep(chip: *mut snd_pmac);
    pub fn snd_pmac_beep_stop(chip: *mut snd_pmac);
    pub fn snd_pmac_rate_index(
        chip: *mut snd_pmac,
        rec: *mut pmac_stream,
        rate: c_uint,
    ) -> c_uint;

    pub fn snd_pmac_beep_dma_start(
        chip: *mut snd_pmac,
        bytes: c_int,
        addr: c_ulong,
        speed: c_int,
    );
    pub fn snd_pmac_beep_dma_stop(chip: *mut snd_pmac);

    // Present when CONFIG_PM is enabled in the original C build.
    pub fn snd_pmac_suspend(chip: *mut snd_pmac);
    pub fn snd_pmac_resume(chip: *mut snd_pmac);

    /* initialize mixer */
    pub fn snd_pmac_awacs_init(chip: *mut snd_pmac) -> c_int;
    pub fn snd_pmac_burgundy_init(chip: *mut snd_pmac) -> c_int;
    pub fn snd_pmac_daca_init(chip: *mut snd_pmac) -> c_int;
    pub fn snd_pmac_tumbler_init(chip: *mut snd_pmac) -> c_int;
    pub fn snd_pmac_tumbler_post_init() -> c_int;
}

/* i2c functions */
#[repr(C)]
pub struct pmac_keywest {
    pub addr: c_int,
    pub client: *mut i2c_client,
    pub id: c_int,
    pub init_client: Option<unsafe extern "C" fn(i2c: *mut pmac_keywest) -> c_int>,
    pub name: *mut c_char,
}

unsafe extern "C" {
    pub fn snd_pmac_keywest_init(i2c: *mut pmac_keywest) -> c_int;
    pub fn snd_pmac_keywest_cleanup(i2c: *mut pmac_keywest);

    /* misc */
    pub fn snd_ctl_boolean_stereo_info();
    pub fn snd_ctl_boolean_mono_info();

    pub fn snd_pmac_add_automute(chip: *mut snd_pmac) -> c_int;
}

pub use snd_ctl_boolean_mono_info as snd_pmac_boolean_mono_info;
pub use snd_ctl_boolean_stereo_info as snd_pmac_boolean_stereo_info;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
