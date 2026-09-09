/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from <sound/hdaudio.h>; referenced types and constants are
 * supplied by the corresponding dependency. */

extern "C" {
    pub fn snd_hdac_ext_bus_init(
        bus: *mut hdac_bus,
        dev: *mut device,
        ops: *const hdac_bus_ops,
        ext_ops: *const hdac_ext_bus_ops,
    ) -> ::core::ffi::c_int;

    pub fn snd_hdac_ext_bus_exit(bus: *mut hdac_bus);
    pub fn snd_hdac_ext_bus_device_remove(bus: *mut hdac_bus);

    pub fn snd_hdac_ext_bus_ppcap_enable(chip: *mut hdac_bus, enable: bool);
    pub fn snd_hdac_ext_bus_ppcap_int_enable(chip: *mut hdac_bus, enable: bool);

    pub fn snd_hdac_ext_bus_get_ml_capabilities(bus: *mut hdac_bus) -> ::core::ffi::c_int;
    pub fn snd_hdac_ext_bus_get_hlink_by_id(bus: *mut hdac_bus, id: u32) -> *mut hdac_ext_link;
    pub fn snd_hdac_ext_bus_get_hlink_by_addr(
        bus: *mut hdac_bus,
        addr: ::core::ffi::c_int,
    ) -> *mut hdac_ext_link;
    pub fn snd_hdac_ext_bus_get_hlink_by_name(
        bus: *mut hdac_bus,
        codec_name: *const ::core::ffi::c_char,
    ) -> *mut hdac_ext_link;

    pub fn snd_hdac_ext_stream_init_all(
        bus: *mut hdac_bus,
        start_idx: ::core::ffi::c_int,
        num_stream: ::core::ffi::c_int,
        dir: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn snd_hdac_ext_stream_free_all(bus: *mut hdac_bus);
    pub fn snd_hdac_ext_link_free_all(bus: *mut hdac_bus);
    pub fn snd_hdac_ext_stream_assign(
        bus: *mut hdac_bus,
        substream: *mut snd_pcm_substream,
        stream_type: ::core::ffi::c_int,
    ) -> *mut hdac_ext_stream;
    pub fn snd_hdac_ext_stream_release(stream: *mut hdac_ext_stream, stream_type: ::core::ffi::c_int);
    pub fn snd_hdac_ext_cstream_assign(
        bus: *mut hdac_bus,
        cstream: *mut snd_compr_stream,
    ) -> *mut hdac_ext_stream;
    pub fn snd_hdac_ext_stream_decouple_locked(
        bus: *mut hdac_bus,
        stream: *mut hdac_ext_stream,
        decouple: bool,
    );
    pub fn snd_hdac_ext_stream_decouple(
        bus: *mut hdac_bus,
        stream: *mut hdac_ext_stream,
        decouple: bool,
    );
    pub fn snd_hdac_ext_stream_start(stream: *mut hdac_ext_stream);
    pub fn snd_hdac_ext_stream_clear(stream: *mut hdac_ext_stream);
    pub fn snd_hdac_ext_stream_reset(stream: *mut hdac_ext_stream);
    pub fn snd_hdac_ext_stream_setup(
        stream: *mut hdac_ext_stream,
        fmt: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    pub fn snd_hdac_ext_host_stream_setup(
        stream: *mut hdac_ext_stream,
        code_loading: bool,
    ) -> ::core::ffi::c_int;

    pub fn snd_hdac_ext_bus_link_power_up(link: *mut hdac_ext_link) -> ::core::ffi::c_int;
    pub fn snd_hdac_ext_bus_link_power_down(link: *mut hdac_ext_link) -> ::core::ffi::c_int;
    pub fn snd_hdac_ext_bus_link_power_up_all(bus: *mut hdac_bus) -> ::core::ffi::c_int;
    pub fn snd_hdac_ext_bus_link_power_down_all(bus: *mut hdac_bus) -> ::core::ffi::c_int;
    pub fn snd_hdac_ext_bus_link_set_stream_id(link: *mut hdac_ext_link, stream: ::core::ffi::c_int);
    pub fn snd_hdac_ext_bus_link_clear_stream_id(link: *mut hdac_ext_link, stream: ::core::ffi::c_int);
    pub fn snd_hdac_ext_bus_link_get(bus: *mut hdac_bus, link: *mut hdac_ext_link) -> ::core::ffi::c_int;
    pub fn snd_hdac_ext_bus_link_put(bus: *mut hdac_bus, link: *mut hdac_ext_link) -> ::core::ffi::c_int;
    pub fn snd_hdac_ext_bus_link_power(codec: *mut hdac_device, enable: bool);

    pub fn snd_hda_ext_driver_register(drv: *mut hdac_driver) -> ::core::ffi::c_int;
    pub fn snd_hda_ext_driver_unregister(drv: *mut hdac_driver);
}

#[repr(C)]
pub enum hdac_ext_stream_type {
    HDAC_EXT_STREAM_TYPE_COUPLED = 0,
    HDAC_EXT_STREAM_TYPE_HOST,
    HDAC_EXT_STREAM_TYPE_LINK,
}

#[repr(C)]
pub struct hdac_ext_stream {
    pub hstream: hdac_stream,
    pub pphc_addr: *mut ::core::ffi::c_void,
    pub pplc_addr: *mut ::core::ffi::c_void,
    pub pphcllpl: u32,
    pub pphcllpu: u32,
    pub pphcldpl: u32,
    pub pphcldpu: u32,
    pub pplcllpl: u32,
    pub pplcllpu: u32,
    /* C bit-fields; represented as their storage byte. */
    pub decoupled: u8,
    pub link_locked: u8,
    pub link_prepared: bool,
    pub host_setup: Option<unsafe extern "C" fn(*mut hdac_stream, bool) -> ::core::ffi::c_int>,
    pub link_substream: *mut snd_pcm_substream,
}

#[repr(C)]
pub struct hdac_ext_link {
    pub bus: *mut hdac_bus,
    pub index: ::core::ffi::c_int,
    pub ml_addr: *mut ::core::ffi::c_void,
    pub lcaps: u32,
    pub lsdiid: u16,
    pub id: u32,
    pub slcount: u8,
    pub ref_count: ::core::ffi::c_int,
    pub list: list_head,
}

#[repr(C)]
pub struct hdac_ext_codec_ops {
    pub build_controls: Option<unsafe extern "C" fn(*mut hdac_ext_device) -> ::core::ffi::c_int>,
    pub init: Option<unsafe extern "C" fn(*mut hdac_ext_device) -> ::core::ffi::c_int>,
    pub free: Option<unsafe extern "C" fn(*mut hdac_ext_device)>,
}

#[repr(C)]
pub struct hda_dai_map {
    pub dai_name: *mut ::core::ffi::c_char,
    pub nid: hda_nid_t,
    pub maxbps: u32,
}

#[repr(C)]
pub struct hdac_ext_dma_params {
    pub format: u32,
    pub stream_tag: u8,
}

#[inline]
pub unsafe fn hdac_stream(s: *mut hdac_ext_stream) -> *mut hdac_stream {
    unsafe { &mut (*s).hstream }
}

/* container_of(s, struct hdac_ext_stream, hstream) */
#[inline]
pub unsafe fn stream_to_hdac_ext_stream(s: *mut hdac_stream) -> *mut hdac_ext_stream {
    (s as *mut u8).sub(::core::mem::offset_of!(hdac_ext_stream, hstream)) as *mut hdac_ext_stream
}

#[macro_export]
macro_rules! HDA_CODEC_REV_EXT_ENTRY {
    ($vid:expr, $rev:expr, $name:expr, $drv_data:expr) => {
        hdac_device_id {
            vendor_id: $vid,
            rev_id: $rev,
            name: $name,
            api_version: HDA_DEV_ASOC,
            driver_data: $drv_data as ::core::ffi::c_ulong,
        }
    };
}

#[macro_export]
macro_rules! HDA_CODEC_EXT_ENTRY {
    ($vid:expr, $revid:expr, $name:expr, $drv_data:expr) => {
        HDA_CODEC_REV_EXT_ENTRY!($vid, $revid, $name, $drv_data)
    };
}

#[inline]
pub unsafe fn hdac_ext_link_alt(link: *const hdac_ext_link) -> u32 {
    (*link).lcaps & AZX_ML_HDA_LCAP_ALT
}

#[inline]
pub unsafe fn hdac_ext_link_ofls(link: *const hdac_ext_link) -> u32 {
    (*link).lcaps & AZX_ML_HDA_LCAP_OFLS
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
