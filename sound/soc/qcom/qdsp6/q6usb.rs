// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2022-2025 Qualcomm Innovation Center, Inc. All rights reserved.
 */

/* Translated from Linux kernel C. External kernel, ASoC, USB offload, DT, and
 * Q6DSP symbols are declared here only as dependencies supplied by other files.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const Q6_USB_SID_MASK: c_uint = 0xF;

const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const USB_RX: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SND_JACK_USB: c_int = 0;

const SNDRV_PCM_RATE_8000: c_uint = 1 << 0;
const SNDRV_PCM_RATE_11025: c_uint = 1 << 1;
const SNDRV_PCM_RATE_16000: c_uint = 1 << 2;
const SNDRV_PCM_RATE_22050: c_uint = 1 << 3;
const SNDRV_PCM_RATE_32000: c_uint = 1 << 4;
const SNDRV_PCM_RATE_44100: c_uint = 1 << 5;
const SNDRV_PCM_RATE_48000: c_uint = 1 << 6;
const SNDRV_PCM_RATE_96000: c_uint = 1 << 7;
const SNDRV_PCM_RATE_192000: c_uint = 1 << 8;

const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 0;
const SNDRV_PCM_FMTBIT_S16_BE: u64 = 1 << 1;
const SNDRV_PCM_FMTBIT_U16_LE: u64 = 1 << 2;
const SNDRV_PCM_FMTBIT_U16_BE: u64 = 1 << 3;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << 4;
const SNDRV_PCM_FMTBIT_S24_BE: u64 = 1 << 5;
const SNDRV_PCM_FMTBIT_U24_LE: u64 = 1 << 6;
const SNDRV_PCM_FMTBIT_U24_BE: u64 = 1 << 7;

#[repr(C)]
pub struct device {
    pub parent: *mut device,
    pub release: Option<unsafe extern "C" fn(*mut device)>,
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct auxiliary_device {
    pub dev: device,
    pub name: *const c_char,
}

#[repr(C)]
pub struct q6afe_usb_cfg {
    _private: [u8; 0],
}

#[repr(C)]
pub struct q6usb_offload {
    pub dev: *mut device,
    pub intr_num: u16,
    pub sid: c_uint,
    pub domain: *mut c_void,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct snd_soc_usb {
    pub component: *mut snd_soc_component,
    pub connection_status_cb: Option<
        unsafe extern "C" fn(*mut snd_soc_usb, *mut snd_soc_usb_device, bool) -> c_int,
    >,
    pub update_offload_route_info: Option<
        unsafe extern "C" fn(
            *mut snd_soc_component,
            c_int,
            c_int,
            c_int,
            snd_soc_usb_kctl,
            *mut c_long,
        ) -> c_int,
    >,
}

#[repr(C)]
pub struct snd_soc_jack {
    pub jack: *mut snd_jack,
}

#[repr(C)]
pub struct snd_jack {
    _private: [u8; 0],
}

#[repr(C)]
pub struct q6usb_port_data {
    pub uauxdev: auxiliary_device,
    pub usb_cfg: q6afe_usb_cfg,
    pub usb: *mut snd_soc_usb,
    pub hs_jack: *mut snd_soc_jack,
    pub priv_: q6usb_offload,

    /* Protects against operations between SOC USB and ASoC */
    pub mutex: mutex,
    pub devices: list_head,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub name: *const c_char,
    pub dapm: *mut snd_soc_dapm_context,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub private_data: *mut c_void,
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
    pub stream: [snd_soc_dai_stream; 2],
}

#[repr(C)]
pub struct snd_soc_dai_stream {
    pub widget: *mut snd_soc_dapm_widget,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub id: c_int,
}

#[repr(C)]
pub struct q6afe_port {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_usb_device {
    pub list: list_head,
    pub chip_idx: c_int,
    pub card_idx: c_int,
    pub ppcm_idx: *mut c_int,
    pub num_playback: c_int,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub rates: c_uint,
    pub formats: u64,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rate_max: c_uint,
    pub rate_min: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub playback: snd_soc_pcm_stream,
    pub id: c_int,
    pub name: *const c_char,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct of_phandle_args {
    pub args: [c_uint; 1],
}

#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
    pub card: *mut snd_soc_card,
}

#[repr(C)]
pub struct snd_soc_card {
    pub snd_card: *mut snd_card,
}

#[repr(C)]
pub struct snd_card {
    pub number: c_int,
}

#[repr(C)]
pub struct snd_soc_dapm_path {
    pub sink: *mut snd_soc_dapm_widget,
    pub connect: c_int,
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub enum snd_soc_usb_kctl {
    SND_SOC_USB_KCTL_CARD_ROUTE = 0,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub set_jack:
        Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_soc_jack, *mut c_void) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    pub name: *const c_char,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: usize,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: usize,
    pub of_xlate_dai_name: Option<
        unsafe extern "C" fn(
            *mut snd_soc_component,
            *const of_phandle_args,
            *mut *const c_char,
        ) -> c_int,
    >,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_inner,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

unsafe extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn list_empty(head: *const list_head) -> c_int;
    fn list_add_tail(new: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_soc_usb_find_supported_format(
        chip_idx: c_int,
        params: *mut snd_pcm_hw_params,
        direction: c_int,
    ) -> c_int;
    fn q6afe_port_get_from_id(dev: *mut device, id: c_int) -> *mut q6afe_port;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn afe_port_send_usb_dev_param(q6usb_afe: *mut q6afe_port, card_idx: c_int, pcm_idx: c_int)
        -> c_int;
    fn snd_soc_dapm_to_card(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_card;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn snd_jack_report(jack: *mut snd_jack, status: c_int);
    fn auxiliary_device_init(auxdev: *mut auxiliary_device) -> c_int;
    fn auxiliary_device_add(auxdev: *mut auxiliary_device) -> c_int;
    fn auxiliary_device_uninit(auxdev: *mut auxiliary_device);
    fn auxiliary_device_delete(auxdev: *mut auxiliary_device);
    fn snd_soc_usb_allocate_port(
        component: *mut snd_soc_component,
        priv_: *mut q6usb_offload,
    ) -> *mut snd_soc_usb;
    fn snd_soc_usb_add_port(usb: *mut snd_soc_usb);
    fn snd_soc_usb_remove_port(usb: *mut snd_soc_usb);
    fn snd_soc_usb_free_port(usb: *mut snd_soc_usb);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn of_property_read_u16(np: *mut device_node, propname: *const c_char, out_value: *mut u16)
        -> c_int;
    fn of_parse_phandle_with_fixed_args(
        np: *mut device_node,
        list_name: *const c_char,
        cells_count: c_int,
        index: c_int,
        out_args: *mut of_phandle_args,
    ) -> c_int;
    fn devm_mutex_init(dev: *mut device, lock: *mut mutex) -> c_int;
    fn iommu_get_domain_for_dev(dev: *mut device) -> *mut c_void;
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: usize,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

/* Iteration helpers supplied by ASoC/list macros in C headers. */
unsafe extern "C" {
    fn for_each_card_rtds_next(
        card: *mut snd_soc_card,
        current: *mut snd_soc_pcm_runtime,
    ) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_dapm_widget_for_each_sink_path_next(
        w: *mut snd_soc_dapm_widget,
        current: *mut snd_soc_dapm_path,
    ) -> *mut snd_soc_dapm_path;
    fn for_each_card_widgets_next(
        card: *mut snd_soc_card,
        current: *mut snd_soc_dapm_widget,
    ) -> *mut snd_soc_dapm_widget;
}

unsafe fn list_last_entry_snd_soc_usb_device(head: *mut list_head) -> *mut snd_soc_usb_device {
    let last = (*head).prev;
    (last as *mut u8).offset(-(core::mem::offset_of!(snd_soc_usb_device, list) as isize))
        as *mut snd_soc_usb_device
}

static q6usb_dai_widgets: [snd_soc_dapm_widget; 1] = [snd_soc_dapm_widget {
    name: c"USB_RX_BE".as_ptr(),
    dapm: ptr::null_mut(),
}];

static q6usb_dapm_routes: [snd_soc_dapm_route; 1] = [snd_soc_dapm_route {
    sink: c"USB Playback".as_ptr(),
    control: ptr::null(),
    source: c"USB_RX_BE".as_ptr(),
}];

unsafe extern "C" fn q6usb_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let data = dev_get_drvdata((*dai).dev) as *mut q6usb_port_data;
    let rtd = (*substream).private_data as *mut snd_soc_pcm_runtime;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let direction = (*substream).stream;
    let q6usb_afe: *mut q6afe_port;
    let sdev: *mut snd_soc_usb_device;
    let mut ret: c_int = -EINVAL;

    mutex_lock(&mut (*data).mutex);

    /* No active chip index */
    if list_empty(&(*data).devices) != 0 {
        goto_out(&mut (*data).mutex, ret)
    } else {
        sdev = list_last_entry_snd_soc_usb_device(&mut (*data).devices);

        ret = snd_soc_usb_find_supported_format((*sdev).chip_idx, params, direction);
        if ret < 0 {
            goto_out(&mut (*data).mutex, ret)
        } else {
            q6usb_afe = q6afe_port_get_from_id((*cpu_dai).dev, USB_RX);
            if IS_ERR(q6usb_afe as *const c_void) {
                ret = PTR_ERR(q6usb_afe as *const c_void);
                goto_out(&mut (*data).mutex, ret)
            } else {
                /* Notify audio DSP about the devices being offloaded */
                ret = afe_port_send_usb_dev_param(
                    q6usb_afe,
                    (*sdev).card_idx,
                    *(*sdev).ppcm_idx.offset(((*sdev).num_playback - 1) as isize),
                );
                goto_out(&mut (*data).mutex, ret)
            }
        }
    }
}

unsafe fn goto_out(lock: *mut mutex, ret: c_int) -> c_int {
    mutex_unlock(lock);
    ret
}

static q6usb_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(q6usb_hw_params),
};

static mut q6usb_be_dais: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    playback: snd_soc_pcm_stream {
        stream_name: c"USB BE RX".as_ptr(),
        rates: SNDRV_PCM_RATE_8000
            | SNDRV_PCM_RATE_11025
            | SNDRV_PCM_RATE_16000
            | SNDRV_PCM_RATE_22050
            | SNDRV_PCM_RATE_32000
            | SNDRV_PCM_RATE_44100
            | SNDRV_PCM_RATE_48000
            | SNDRV_PCM_RATE_96000
            | SNDRV_PCM_RATE_192000,
        formats: SNDRV_PCM_FMTBIT_S16_LE
            | SNDRV_PCM_FMTBIT_S16_BE
            | SNDRV_PCM_FMTBIT_U16_LE
            | SNDRV_PCM_FMTBIT_U16_BE
            | SNDRV_PCM_FMTBIT_S24_LE
            | SNDRV_PCM_FMTBIT_S24_BE
            | SNDRV_PCM_FMTBIT_U24_LE
            | SNDRV_PCM_FMTBIT_U24_BE,
        channels_min: 1,
        channels_max: 2,
        rate_max: 192000,
        rate_min: 8000,
    },
    id: USB_RX,
    name: c"USB_RX_BE".as_ptr(),
    ops: &q6usb_ops,
}];

unsafe extern "C" fn q6usb_audio_ports_of_xlate_dai_name(
    _component: *mut snd_soc_component,
    args: *const of_phandle_args,
    dai_name: *mut *const c_char,
) -> c_int {
    let id = (*args).args[0] as c_int;
    let mut ret: c_int = -EINVAL;
    let mut i: usize = 0;

    while i < q6usb_be_dais.len() {
        if q6usb_be_dais[i].id == id {
            *dai_name = q6usb_be_dais[i].name;
            ret = 0;
            break;
        }
        i += 1;
    }

    ret
}

unsafe fn q6usb_get_pcm_id_from_widget(w: *mut snd_soc_dapm_widget) -> c_int {
    let card = snd_soc_dapm_to_card((*w).dapm);
    let mut rtd: *mut snd_soc_pcm_runtime = ptr::null_mut();
    let mut dai: *mut snd_soc_dai;

    loop {
        rtd = for_each_card_rtds_next(card, rtd);
        if rtd.is_null() {
            break;
        }
        dai = snd_soc_rtd_to_cpu(rtd, 0);
        /*
         * Only look for playback widget. RTD number carries the assigned
         * PCM index.
         */
        if (*dai).stream[0].widget == w {
            return (*rtd).id;
        }
    }

    -1
}

unsafe fn q6usb_usb_mixer_enabled(w: *mut snd_soc_dapm_widget) -> c_int {
    let mut p: *mut snd_soc_dapm_path = ptr::null_mut();

    /* Checks to ensure USB path is enabled/connected */
    loop {
        p = snd_soc_dapm_widget_for_each_sink_path_next(w, p);
        if p.is_null() {
            break;
        }
        if strcmp((*(*p).sink).name, c"USB Mixer".as_ptr()) == 0 && (*p).connect != 0 {
            return 1;
        }
    }

    0
}

unsafe fn q6usb_get_pcm_id(component: *mut snd_soc_component) -> c_int {
    let mut w: *mut snd_soc_dapm_widget = ptr::null_mut();
    let mut p: *mut snd_soc_dapm_path;
    let pidx: c_int;

    /*
     * Traverse widgets to find corresponding FE widget.  The DAI links are
     * built like the following:
     *    MultiMedia* <-> MM_DL* <-> USB Mixer*
     */
    loop {
        w = for_each_card_widgets_next((*component).card, w);
        if w.is_null() {
            break;
        }
        if strncmp((*w).name, c"MultiMedia".as_ptr(), 10) == 0 {
            /*
             * Look up all paths associated with the FE widget to see if
             * the USB BE is enabled.  The sink widget is responsible to
             * link with the USB mixers.
             */
            p = ptr::null_mut();
            loop {
                p = snd_soc_dapm_widget_for_each_sink_path_next(w, p);
                if p.is_null() {
                    break;
                }
                if q6usb_usb_mixer_enabled((*p).sink) != 0 {
                    pidx = q6usb_get_pcm_id_from_widget(w);
                    return pidx;
                }
            }
        }
    }

    -1
}

unsafe extern "C" fn q6usb_update_offload_route(
    component: *mut snd_soc_component,
    card: c_int,
    pcm: c_int,
    direction: c_int,
    path: snd_soc_usb_kctl,
    route: *mut c_long,
) -> c_int {
    let data = dev_get_drvdata((*component).dev) as *mut q6usb_port_data;
    let sdev: *mut snd_soc_usb_device;
    let mut ret: c_int = 0;
    let mut idx: c_int = -1;

    mutex_lock(&mut (*data).mutex);

    if list_empty(&(*data).devices) != 0 || direction == SNDRV_PCM_STREAM_CAPTURE {
        ret = -ENODEV;
    } else {
        sdev = list_last_entry_snd_soc_usb_device(&mut (*data).devices);

        /*
         * Will always look for last PCM device discovered/probed as the
         * active offload index.
         */
        if card == (*sdev).card_idx
            && pcm == *(*sdev).ppcm_idx.offset(((*sdev).num_playback - 1) as isize)
        {
            idx = if path as c_int == snd_soc_usb_kctl::SND_SOC_USB_KCTL_CARD_ROUTE as c_int {
                (*(*(*component).card).snd_card).number
            } else {
                q6usb_get_pcm_id(component)
            };
        }
    }

    *route.offset(0) = idx as c_long;
    mutex_unlock(&mut (*data).mutex);

    ret
}

unsafe extern "C" fn q6usb_alsa_connection_cb(
    usb: *mut snd_soc_usb,
    sdev: *mut snd_soc_usb_device,
    connected: bool,
) -> c_int {
    let data: *mut q6usb_port_data;

    if (*usb).component.is_null() {
        return -ENODEV;
    }

    data = dev_get_drvdata((*(*usb).component).dev) as *mut q6usb_port_data;

    mutex_lock(&mut (*data).mutex);
    if connected {
        if !(*data).hs_jack.is_null() {
            snd_jack_report((*(*data).hs_jack).jack, SND_JACK_USB);
        }

        /* Selects the latest USB headset plugged in for offloading */
        list_add_tail(&mut (*sdev).list, &mut (*data).devices);
    } else {
        list_del(&mut (*sdev).list);

        if !(*data).hs_jack.is_null() {
            snd_jack_report((*(*data).hs_jack).jack, 0);
        }
    }
    mutex_unlock(&mut (*data).mutex);

    0
}

unsafe fn q6usb_component_disable_jack(data: *mut q6usb_port_data) {
    /* Offload jack has already been disabled */
    if (*data).hs_jack.is_null() {
        return;
    }

    snd_jack_report((*(*data).hs_jack).jack, 0);
    (*data).hs_jack = ptr::null_mut();
}

unsafe fn q6usb_component_enable_jack(data: *mut q6usb_port_data, jack: *mut snd_soc_jack) {
    snd_jack_report(
        (*jack).jack,
        if list_empty(&(*data).devices) == 0 {
            SND_JACK_USB
        } else {
            0
        },
    );
    (*data).hs_jack = jack;
}

unsafe extern "C" fn q6usb_component_set_jack(
    component: *mut snd_soc_component,
    jack: *mut snd_soc_jack,
    _priv: *mut c_void,
) -> c_int {
    let data = dev_get_drvdata((*component).dev) as *mut q6usb_port_data;

    mutex_lock(&mut (*data).mutex);
    if !jack.is_null() {
        q6usb_component_enable_jack(data, jack);
    } else {
        q6usb_component_disable_jack(data);
    }
    mutex_unlock(&mut (*data).mutex);

    0
}

unsafe extern "C" fn q6usb_dai_aux_release(_dev: *mut device) {}

unsafe fn q6usb_dai_add_aux_device(
    data: *mut q6usb_port_data,
    auxdev: *mut auxiliary_device,
) -> c_int {
    let mut ret: c_int;

    (*auxdev).dev.parent = (*data).priv_.dev;
    (*auxdev).dev.release = Some(q6usb_dai_aux_release);
    (*auxdev).name = c"qc-usb-audio-offload".as_ptr();

    ret = auxiliary_device_init(auxdev);
    if ret != 0 {
        return ret;
    }

    ret = auxiliary_device_add(auxdev);
    if ret != 0 {
        auxiliary_device_uninit(auxdev);
    }

    ret
}

unsafe extern "C" fn q6usb_component_probe(component: *mut snd_soc_component) -> c_int {
    let data = dev_get_drvdata((*component).dev) as *mut q6usb_port_data;
    let usb: *mut snd_soc_usb;
    let ret: c_int;

    /* Add the QC USB SND aux device */
    ret = q6usb_dai_add_aux_device(data, &mut (*data).uauxdev);
    if ret < 0 {
        return ret;
    }

    usb = snd_soc_usb_allocate_port(component, &mut (*data).priv_);
    if IS_ERR(usb as *const c_void) {
        return -ENOMEM;
    }

    (*usb).connection_status_cb = Some(q6usb_alsa_connection_cb);
    (*usb).update_offload_route_info = Some(q6usb_update_offload_route);

    snd_soc_usb_add_port(usb);
    (*data).usb = usb;

    0
}

unsafe extern "C" fn q6usb_component_remove(component: *mut snd_soc_component) {
    let data = dev_get_drvdata((*component).dev) as *mut q6usb_port_data;

    snd_soc_usb_remove_port((*data).usb);
    auxiliary_device_delete(&mut (*data).uauxdev);
    auxiliary_device_uninit(&mut (*data).uauxdev);
    snd_soc_usb_free_port((*data).usb);
}

static q6usb_dai_component: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(q6usb_component_probe),
    set_jack: Some(q6usb_component_set_jack),
    remove: Some(q6usb_component_remove),
    name: c"q6usb-dai-component".as_ptr(),
    dapm_widgets: q6usb_dai_widgets.as_ptr(),
    num_dapm_widgets: q6usb_dai_widgets.len(),
    dapm_routes: q6usb_dapm_routes.as_ptr(),
    num_dapm_routes: q6usb_dapm_routes.len(),
    of_xlate_dai_name: Some(q6usb_audio_ports_of_xlate_dai_name),
};

unsafe extern "C" fn q6usb_dai_dev_probe(pdev: *mut platform_device) -> c_int {
    let node = (*pdev).dev.of_node;
    let data: *mut q6usb_port_data;
    let dev = &mut (*pdev).dev as *mut device;
    let mut args = of_phandle_args { args: [0; 1] };
    let mut ret: c_int;

    data = devm_kzalloc(dev, size_of::<q6usb_port_data>(), GFP_KERNEL) as *mut q6usb_port_data;
    if data.is_null() {
        return -ENOMEM;
    }

    ret = of_property_read_u16(
        node,
        c"qcom,usb-audio-intr-idx".as_ptr(),
        &mut (*data).priv_.intr_num,
    );
    if ret != 0 {
        dev_err(
            &mut (*pdev).dev,
            c"failed to read intr idx.\n".as_ptr(),
        );
        return ret;
    }

    ret = of_parse_phandle_with_fixed_args(node, c"iommus".as_ptr(), 1, 0, &mut args);
    if ret == 0 {
        (*data).priv_.sid = args.args[0] & Q6_USB_SID_MASK;
    }

    ret = devm_mutex_init(dev, &mut (*data).mutex);
    if ret < 0 {
        return ret;
    }

    (*data).priv_.domain = iommu_get_domain_for_dev(&mut (*pdev).dev);

    (*data).priv_.dev = dev;
    INIT_LIST_HEAD(&mut (*data).devices);
    dev_set_drvdata(dev, data as *mut c_void);

    devm_snd_soc_register_component(
        dev,
        &q6usb_dai_component,
        q6usb_be_dais.as_mut_ptr(),
        q6usb_be_dais.len(),
    )
}

static q6usb_dai_device_id: [of_device_id; 2] = [
    of_device_id {
        compatible: c"qcom,q6usb".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, q6usb_dai_device_id); */

static mut q6usb_dai_platform_driver: platform_driver = platform_driver {
    driver: platform_driver_inner {
        name: c"q6usb-dai".as_ptr(),
        of_match_table: q6usb_dai_device_id.as_ptr(),
    },
    probe: Some(q6usb_dai_dev_probe),
    /*
     * Remove not required as resources are cleaned up as part of
     * component removal.  Others are device managed resources.
     */
};
/* module_platform_driver(q6usb_dai_platform_driver); */

/* MODULE_DESCRIPTION("Q6 USB backend dai driver"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
