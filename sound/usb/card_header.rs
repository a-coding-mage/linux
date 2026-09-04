// SPDX-License-Identifier: GPL-2.0

pub const MAX_NR_RATES: usize = 1024;
pub const MAX_PACKS: usize = 6;  // per URB
pub const MAX_PACKS_HS: usize = MAX_PACKS * 8;  // in high speed mode
pub const MAX_URBS: usize = 12;
pub const SYNC_URBS: usize = 4;  // always four urbs for sync
pub const MAX_QUEUE: usize = 18;  // try not to exceed this queue length, in ms

// Forward declarations for external types
#[repr(C)]
pub struct list_head;

#[repr(C)]
pub struct urb;

#[repr(C)]
pub struct snd_usb_substream;

#[repr(C)]
pub struct snd_usb_iface_ref;

#[repr(C)]
pub struct snd_usb_clock_ref;

#[repr(C)]
pub struct snd_usb_endpoint;

#[repr(C)]
pub struct snd_usb_power_domain;

#[repr(C)]
pub struct snd_usb_audio;

#[repr(C)]
pub struct usb_device;

#[repr(C)]
pub struct snd_pcm_substream;

#[repr(C)]
pub struct snd_pcm_chmap_elem;

#[repr(C)]
pub struct spinlock_t;

#[repr(C)]
pub struct snd_pcm;

#[repr(C)]
pub struct media_ctl;

#[repr(C)]
pub struct usb_interface;

#[repr(C)]
pub struct snd_pcm_hw_params;

// Type aliases for kernel types
pub type atomic_t = i32;
pub type dma_addr_t = usize;
pub type snd_pcm_format_t = u32;
pub type pm_message_t = i32;

#[repr(C)]
pub struct audioformat {
    pub list: list_head,
    pub formats: u64,
    pub channels: u32,
    pub fmt_type: u32,
    pub fmt_bits: u32,
    pub fmt_sz: u32,
    pub frame_size: u32,
    pub iface: u8,
    pub altsetting: u8,
    pub ep_idx: u8,
    pub altset_idx: u8,
    pub attributes: u8,
    pub endpoint: u8,
    pub ep_attr: u8,
    pub implicit_fb: bool,
    pub sync_ep: u8,
    pub sync_iface: u8,
    pub sync_altsetting: u8,
    pub sync_ep_idx: u8,
    pub datainterval: u8,
    pub protocol: u8,
    pub maxpacksize: u32,
    pub rates: u32,
    pub rate_min: u32,
    pub rate_max: u32,
    pub nr_rates: u32,
    pub rate_table: *mut u32,
    pub clock: u8,
    pub chmap: *mut snd_pcm_chmap_elem,
    pub dsd_dop: bool,
    pub dsd_bitrev: bool,
    pub dsd_raw: bool,
}

#[repr(C)]
pub struct snd_urb_ctx {
    pub urb: *mut urb,
    pub buffer_size: u32,
    pub subs: *mut snd_usb_substream,
    pub ep: *mut snd_usb_endpoint,
    pub index: i32,
    pub packets: i32,
    pub queued: i32,
    pub packet_size: [i32; MAX_PACKS_HS],
    pub ready_list: list_head,
}

#[repr(C)]
pub struct snd_usb_packet_info {
    pub packet_size: [i32; MAX_PACKS_HS],
    pub packets: i32,
}

#[repr(C)]
pub struct snd_usb_endpoint {
    pub chip: *mut snd_usb_audio,
    pub iface_ref: *mut snd_usb_iface_ref,
    pub clock_ref: *mut snd_usb_clock_ref,
    pub opened: i32,
    pub running: atomic_t,
    pub ep_num: i32,
    pub type_: i32,
    pub iface: u8,
    pub altsetting: u8,
    pub ep_idx: u8,
    pub state: atomic_t,
    pub prepare_data_urb: Option<unsafe extern "C" fn(*mut snd_usb_substream, *mut urb, bool) -> i32>,
    pub retire_data_urb: Option<unsafe extern "C" fn(*mut snd_usb_substream, *mut urb)>,
    pub data_subs: *mut snd_usb_substream,
    pub sync_source: *mut snd_usb_endpoint,
    pub sync_sink: *mut snd_usb_endpoint,
    pub urb: [snd_urb_ctx; MAX_URBS],
    pub next_packet: [snd_usb_packet_info; MAX_URBS],
    pub next_packet_head: u32,
    pub next_packet_queued: u32,
    pub ready_playback_urbs: list_head,
    pub nurbs: u32,
    pub active_mask: usize,
    pub unlink_mask: usize,
    pub submitted_urbs: atomic_t,
    pub syncbuf: *mut u8,
    pub sync_dma: dma_addr_t,
    pub pipe: u32,
    pub packsize: [u32; 2],
    pub sample_rem: u32,
    pub sample_accum: u32,
    pub pps: u32,
    pub freqn: u32,
    pub freqm: u32,
    pub freqshift: i32,
    pub freqmax: u32,
    pub phase: u32,
    pub maxpacksize: u32,
    pub maxframesize: u32,
    pub max_urb_frames: u32,
    pub curpacksize: u32,
    pub curframesize: u32,
    pub syncmaxsize: u32,
    pub fill_max: bool,
    pub tenor_fb_quirk: bool,
    pub datainterval: u32,
    pub syncinterval: u32,
    pub silence_value: u8,
    pub stride: u32,
    pub skip_packets: i32,
    pub implicit_fb_sync: bool,
    pub lowlatency_playback: bool,
    pub need_setup: bool,
    pub need_prepare: bool,
    pub fixed_rate: bool,
    pub cur_audiofmt: *const audioformat,
    pub cur_rate: u32,
    pub cur_format: snd_pcm_format_t,
    pub cur_channels: u32,
    pub cur_frame_bytes: u32,
    pub cur_period_frames: u32,
    pub cur_period_bytes: u32,
    pub cur_buffer_periods: u32,
    pub lock: spinlock_t,
    pub list: list_head,
}

#[repr(C)]
pub struct snd_usb_dsd_dop {
    pub marker: i32,
    pub channel: i32,
    pub byte_idx: i32,
}

#[repr(C)]
pub struct snd_usb_substream {
    pub stream: *mut snd_usb_stream,
    pub dev: *mut usb_device,
    pub pcm_substream: *mut snd_pcm_substream,
    pub direction: i32,
    pub endpoint: i32,
    pub cur_audiofmt: *const audioformat,
    pub str_pd: *mut snd_usb_power_domain,
    pub channels_max: u32,
    pub txfr_quirk: bool,
    pub tx_length_quirk: bool,
    pub fmt_type: u32,
    pub pkt_offset_adj: u32,
    pub stream_offset_adj: u32,
    pub opened: bool,
    pub running: bool,
    pub period_elapsed_pending: u32,
    pub buffer_bytes: u32,
    pub inflight_bytes: u32,
    pub hwptr_done: u32,
    pub transfer_done: u32,
    pub frame_limit: u32,
    pub ep_num: u32,
    pub data_endpoint: *mut snd_usb_endpoint,
    pub sync_endpoint: *mut snd_usb_endpoint,
    pub flags: usize,
    pub speed: u32,
    pub formats: u64,
    pub num_formats: u32,
    pub fmt_list: list_head,
    pub lock: spinlock_t,
    pub last_frame_number: u32,
    pub dsd_dop: snd_usb_dsd_dop,
    pub trigger_tstamp_pending_update: bool,
    pub lowlatency_playback: bool,
    pub media_ctl: *mut media_ctl,
}

#[repr(C)]
pub struct snd_usb_stream {
    pub chip: *mut snd_usb_audio,
    pub pcm: *mut snd_pcm,
    pub pcm_index: i32,
    pub fmt_type: u32,
    pub substream: [snd_usb_substream; 2],
    pub list: list_head,
}

#[repr(C)]
pub struct snd_usb_platform_ops {
    pub connect_cb: Option<unsafe extern "C" fn(*mut snd_usb_audio)>,
    pub disconnect_cb: Option<unsafe extern "C" fn(*mut snd_usb_audio)>,
    pub suspend_cb: Option<unsafe extern "C" fn(*mut usb_interface, pm_message_t)>,
    pub resume_cb: Option<unsafe extern "C" fn(*mut usb_interface)>,
}

extern "C" {
    pub fn snd_usb_find_suppported_substream(
        card_idx: i32,
        params: *mut snd_pcm_hw_params,
        direction: i32,
    ) -> *mut snd_usb_stream;

    pub fn snd_usb_register_platform_ops(ops: *mut snd_usb_platform_ops) -> i32;

    pub fn snd_usb_unregister_platform_ops() -> i32;

    pub fn snd_usb_rediscover_devices();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
