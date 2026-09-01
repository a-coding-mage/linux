/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  Common functionality for the alsa driver code base for HD Audio.
 */

/* Translated from include dependencies:
 * linux/timecounter.h, linux/interrupt.h, sound/core.h, sound/pcm.h,
 * sound/initval.h, sound/hda_codec.h, sound/hda_register.h
 */

pub const AZX_MAX_CODECS: u32 = HDA_MAX_CODECS;
pub const AZX_DEFAULT_CODECS: u32 = 4;

/* driver quirks (capabilities) */
/* bits 0-7 are used for indicating driver type */
pub const AZX_DCAPS_NO_TCSEL: u32 = 1 << 8; /* No Intel TCSEL bit */
pub const AZX_DCAPS_NO_MSI: u32 = 1 << 9; /* No MSI support */
pub const AZX_DCAPS_SNOOP_MASK: u32 = 3 << 10; /* snoop type mask */
pub const AZX_DCAPS_SNOOP_OFF: u32 = 1 << 12; /* snoop default off */

/* C conditional:
 * #ifdef CONFIG_SND_HDA_I915
 * #define AZX_DCAPS_I915_COMPONENT (1 << 13)
 * #else
 * #define AZX_DCAPS_I915_COMPONENT 0
 * #endif
 */
#[cfg(CONFIG_SND_HDA_I915)]
pub const AZX_DCAPS_I915_COMPONENT: u32 = 1 << 13; /* bind with i915 gfx */
#[cfg(not(CONFIG_SND_HDA_I915))]
pub const AZX_DCAPS_I915_COMPONENT: u32 = 0; /* NOP */

/* 14 unused */
pub const AZX_DCAPS_CTX_WORKAROUND: u32 = 1 << 15; /* X-Fi workaround */
pub const AZX_DCAPS_POSFIX_LPIB: u32 = 1 << 16; /* Use LPIB as default */
pub const AZX_DCAPS_AMD_WORKAROUND: u32 = 1 << 17; /* AMD-specific workaround */
pub const AZX_DCAPS_NO_64BIT: u32 = 1 << 18; /* No 64bit address */
/* 19 unused */
pub const AZX_DCAPS_OLD_SSYNC: u32 = 1 << 20; /* Old SSYNC reg for ICH */
pub const AZX_DCAPS_NO_ALIGN_BUFSIZE: u32 = 1 << 21; /* no buffer size alignment */
/* 22 unused */
pub const AZX_DCAPS_4K_BDLE_BOUNDARY: u32 = 1 << 23; /* BDLE in 4k boundary */
/* 24 unused */
pub const AZX_DCAPS_COUNT_LPIB_DELAY: u32 = 1 << 25; /* Take LPIB as delay */
pub const AZX_DCAPS_PM_RUNTIME: u32 = 1 << 26; /* runtime PM support */
pub const AZX_DCAPS_RETRY_PROBE: u32 = 1 << 27; /* retry probe if no codec is configured */
pub const AZX_DCAPS_CORBRP_SELF_CLEAR: u32 = 1 << 28; /* CORBRP clears itself after reset */
pub const AZX_DCAPS_NO_MSI64: u32 = 1 << 29; /* Stick to 32-bit MSIs */
pub const AZX_DCAPS_SEPARATE_STREAM_TAG: u32 = 1 << 30; /* capture and playback use separate stream tag */
pub const AZX_DCAPS_PIO_COMMANDS: u32 = 1 << 31; /* Use PIO instead of CORB for commands */

pub const AZX_SNOOP_TYPE_NONE: u32 = 0;
pub const AZX_SNOOP_TYPE_SCH: u32 = 1;
pub const AZX_SNOOP_TYPE_ATI: u32 = 2;
pub const AZX_SNOOP_TYPE_NVIDIA: u32 = 3;

#[repr(C)]
pub struct azx_dev {
    pub core: hdac_stream,

    /*
     * For VIA:
     *  A flag to ensure DMA position is 0
     *  when link position is not greater than FIFO size
     */
    pub insufficient: bool,
}

#[macro_export]
macro_rules! azx_stream {
    ($dev:expr) => {
        ::core::ptr::addr_of_mut!((*$dev).core)
    };
}

#[macro_export]
macro_rules! stream_to_azx_dev {
    ($s:expr) => {
        container_of!($s, azx_dev, core)
    };
}

/* Functions to read/write to hda registers. */
#[repr(C)]
pub struct hda_controller_ops {
    /* Disable msi if supported, PCI only */
    pub disable_msi_reset_irq: Option<unsafe extern "C" fn(*mut azx) -> ::core::ffi::c_int>,
    /* Check if current position is acceptable */
    pub position_check: Option<unsafe extern "C" fn(*mut azx, *mut azx_dev) -> ::core::ffi::c_int>,
    /* enable/disable the link power */
    pub link_power: Option<unsafe extern "C" fn(*mut azx, bool) -> ::core::ffi::c_int>,
    /* additional hook for PCM */
    pub pcm_close: Option<unsafe extern "C" fn(*mut azx, *mut azx_dev)>,
}

#[repr(C)]
pub struct azx_pcm {
    pub chip: *mut azx,
    pub pcm: *mut snd_pcm,
    pub codec: *mut hda_codec,
    pub info: *mut hda_pcm,
    pub list: list_head,
}

pub type azx_get_pos_callback_t = Option<unsafe extern "C" fn(*mut azx, *mut azx_dev) -> ::core::ffi::c_uint>;
pub type azx_get_delay_callback_t =
    Option<unsafe extern "C" fn(*mut azx, *mut azx_dev, ::core::ffi::c_uint) -> ::core::ffi::c_int>;

#[repr(C)]
pub struct azx {
    pub bus: hda_bus,

    pub card: *mut snd_card,
    pub pci: *mut pci_dev,
    pub dev_index: ::core::ffi::c_int,

    /* chip type specific */
    pub driver_type: ::core::ffi::c_int,
    pub driver_caps: ::core::ffi::c_uint,
    pub playback_streams: ::core::ffi::c_int,
    pub playback_index_offset: ::core::ffi::c_int,
    pub capture_streams: ::core::ffi::c_int,
    pub capture_index_offset: ::core::ffi::c_int,
    pub num_streams: ::core::ffi::c_int,
    pub jackpoll_interval: ::core::ffi::c_int, /* jack poll interval in jiffies */

    /* Register interaction. */
    pub ops: *const hda_controller_ops,

    /* position adjustment callbacks */
    pub get_position: [azx_get_pos_callback_t; 2],
    pub get_delay: [azx_get_delay_callback_t; 2],

    /* locks */
    pub open_mutex: mutex, /* Prevents concurrent open/close operations */

    /* PCM */
    pub pcm_list: list_head, /* azx_pcm list */

    /* HD codec */
    pub codec_probe_mask: ::core::ffi::c_int, /* copied from probe_mask option */
    pub beep_mode: ::core::ffi::c_uint,
    pub ctl_dev_id: bool,

    /* flags */
    pub bdl_pos_adj: ::core::ffi::c_int,
    /* C bitfields packed in one unsigned int:
     * running:1, fallback_to_single_cmd:1, single_cmd:1, msi:1, probing:1,
     * snoop:1, uc_buffer:1, align_buffer_size:1, disabled:1, pm_prepared:1,
     * gts_present:1
     */
    pub flags: ::core::ffi::c_uint,

    #[cfg(CONFIG_SND_HDA_DSP_LOADER)]
    pub saved_azx_dev: azx_dev,
}

pub const AZX_FLAG_RUNNING: u32 = 1 << 0;
pub const AZX_FLAG_FALLBACK_TO_SINGLE_CMD: u32 = 1 << 1;
pub const AZX_FLAG_SINGLE_CMD: u32 = 1 << 2;
pub const AZX_FLAG_MSI: u32 = 1 << 3;
pub const AZX_FLAG_PROBING: u32 = 1 << 4; /* codec probing phase */
pub const AZX_FLAG_SNOOP: u32 = 1 << 5;
pub const AZX_FLAG_UC_BUFFER: u32 = 1 << 6; /* non-cached pages for stream buffers */
pub const AZX_FLAG_ALIGN_BUFFER_SIZE: u32 = 1 << 7;
pub const AZX_FLAG_DISABLED: u32 = 1 << 8; /* disabled by vga_switcheroo */
pub const AZX_FLAG_PM_PREPARED: u32 = 1 << 9;
pub const AZX_FLAG_GTS_PRESENT: u32 = 1 << 10;

#[macro_export]
macro_rules! azx_bus {
    ($chip:expr) => {
        ::core::ptr::addr_of_mut!((*$chip).bus.core)
    };
}

#[macro_export]
macro_rules! bus_to_azx {
    ($_bus:expr) => {
        container_of!($_bus, azx, bus.core)
    };
}

#[inline]
pub unsafe fn azx_snoop(chip: *mut azx) -> bool {
    !IS_ENABLED(CONFIG_X86) || ((*chip).flags & AZX_FLAG_SNOOP) != 0
}

/*
 * macros for easy use
 */

#[macro_export]
macro_rules! azx_writel {
    ($chip:expr, $reg:expr, $value:expr) => {
        snd_hdac_chip_writel(azx_bus!($chip), $reg, $value)
    };
}

#[macro_export]
macro_rules! azx_readl {
    ($chip:expr, $reg:expr) => {
        snd_hdac_chip_readl(azx_bus!($chip), $reg)
    };
}

#[macro_export]
macro_rules! azx_writew {
    ($chip:expr, $reg:expr, $value:expr) => {
        snd_hdac_chip_writew(azx_bus!($chip), $reg, $value)
    };
}

#[macro_export]
macro_rules! azx_readw {
    ($chip:expr, $reg:expr) => {
        snd_hdac_chip_readw(azx_bus!($chip), $reg)
    };
}

#[macro_export]
macro_rules! azx_writeb {
    ($chip:expr, $reg:expr, $value:expr) => {
        snd_hdac_chip_writeb(azx_bus!($chip), $reg, $value)
    };
}

#[macro_export]
macro_rules! azx_readb {
    ($chip:expr, $reg:expr) => {
        snd_hdac_chip_readb(azx_bus!($chip), $reg)
    };
}

#[macro_export]
macro_rules! azx_has_pm_runtime {
    ($chip:expr) => {
        ((*$chip).driver_caps & AZX_DCAPS_PM_RUNTIME)
    };
}

/* PCM setup */
#[inline]
pub unsafe fn get_azx_dev(substream: *mut snd_pcm_substream) -> *mut azx_dev {
    (*(*substream).runtime).private_data as *mut azx_dev
}

unsafe extern "C" {
    pub fn azx_get_position(chip: *mut azx, azx_dev: *mut azx_dev) -> ::core::ffi::c_uint;
    pub fn azx_get_pos_lpib(chip: *mut azx, azx_dev: *mut azx_dev) -> ::core::ffi::c_uint;
    pub fn azx_get_pos_posbuf(chip: *mut azx, azx_dev: *mut azx_dev) -> ::core::ffi::c_uint;

    /* Stream control. */
    pub fn azx_stop_all_streams(chip: *mut azx);

    /* Low level azx interface */
    pub fn azx_init_chip(chip: *mut azx, full_reset: bool);
    pub fn azx_stop_chip(chip: *mut azx);
    pub fn azx_interrupt(irq: ::core::ffi::c_int, dev_id: *mut ::core::ffi::c_void) -> irqreturn_t;

    /* Codec interface */
    pub fn azx_bus_init(chip: *mut azx, model: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    pub fn azx_probe_codecs(chip: *mut azx, max_slots: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn azx_codec_configure(chip: *mut azx) -> ::core::ffi::c_int;
    pub fn azx_init_streams(chip: *mut azx) -> ::core::ffi::c_int;
    pub fn azx_add_stream(chip: *mut azx, s: *mut azx_dev, idx: ::core::ffi::c_int, tag: ::core::ffi::c_int);
    pub fn azx_free_streams(chip: *mut azx);
}

#[macro_export]
macro_rules! azx_alloc_stream_pages {
    ($chip:expr) => {
        snd_hdac_bus_alloc_stream_pages(azx_bus!($chip))
    };
}

#[macro_export]
macro_rules! azx_free_stream_pages {
    ($chip:expr) => {
        snd_hdac_bus_free_stream_pages(azx_bus!($chip))
    };
}

#[macro_export]
macro_rules! azx_enter_link_reset {
    ($chip:expr) => {
        snd_hdac_bus_enter_link_reset(azx_bus!($chip))
    };
}

#[inline]
pub unsafe fn azx_stream_direction(chip: *mut azx, index: ::core::ffi::c_uchar) -> ::core::ffi::c_int {
    if (index as ::core::ffi::c_int) >= (*chip).capture_index_offset
        && (index as ::core::ffi::c_int) < (*chip).capture_index_offset + (*chip).capture_streams
    {
        return SNDRV_PCM_STREAM_CAPTURE;
    }
    SNDRV_PCM_STREAM_PLAYBACK
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
