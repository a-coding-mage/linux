// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021-2022 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
//          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

const GFP_KERNEL: c_int = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_INVALID_DEVICE: c_int = -1;
const FEDAI_NAME_PREFIX: *const c_char = b"HDMI\0".as_ptr() as *const c_char;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct hda_bus_core {
    pub idx: c_int,
}

#[repr(C)]
pub struct hda_bus {
    pub core: hda_bus_core,
}

#[repr(C)]
pub struct hda_codec_core {
    pub dev: device,
    pub addr: c_int,
}

#[repr(C)]
pub struct hda_codec {
    pub core: hda_codec_core,
    pub bus: *mut hda_bus,
    pub pcm_list_head: list_head,
}

#[repr(C)]
pub struct hda_pcm {
    pub list: list_head,
    pub name: *const c_char,
    pub pcm: *mut snd_pcm,
    pub device: c_int,
}

#[repr(C)]
pub struct avs_mach_pdata {
    pub codec: *mut hda_codec,
    pub obsolete_card_names: bool,
}

#[repr(C)]
pub struct snd_soc_acpi_mach {
    pub pdata: *mut avs_mach_pdata,
}

#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub name: *const c_char,
    pub dai_name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai_link {
    pub name: *const c_char,
    pub id: c_int,
    pub nonatomic: c_int,
    pub no_pcm: c_int,
    pub platforms: *mut snd_soc_dai_link_component,
    pub num_platforms: c_int,
    pub ignore_pmdown_time: c_int,
    pub codecs: *mut snd_soc_dai_link_component,
    pub num_codecs: c_int,
    pub cpus: *mut snd_soc_dai_link_component,
    pub num_cpus: c_int,
    pub init: Option<unsafe extern "C" fn(*mut snd_soc_pcm_runtime) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub pcm: *mut snd_pcm,
    pub list: list_head,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub pcm: *mut snd_pcm,
}

#[repr(C)]
pub struct snd_pcm {
    pub streams: [snd_pcm_substream; 2],
    pub id: [c_char; 64],
    pub device: c_int,
}

#[repr(C)]
pub struct snd_soc_card {
    pub dev: *mut device,
    pub owner: *mut c_void,
    pub name: *const c_char,
    pub driver_name: *const c_char,
    pub long_name: *const c_char,
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: c_int,
    pub fully_routed: bool,
    pub late_probe: Option<unsafe extern "C" fn(*mut snd_soc_card) -> c_int>,
    pub rtd_list: list_head,
}

#[repr(C)]
pub struct platform_device_id {
    pub name: [c_char; 32],
}

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
    pub pm: *const c_void,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: platform_driver_inner,
    pub id_table: *const platform_device_id,
}

unsafe extern "C" {
    static mut THIS_MODULE: *mut c_void;
    static mut snd_soc_dummy_dlc: snd_soc_dai_link_component;
    static snd_soc_pm_ops: c_void;

    fn dev_name(dev: *const device) -> *const c_char;
    fn dev_get_platdata(dev: *const device) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_int) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_int) -> *mut c_void;
    fn devm_kmemdup(dev: *mut device, src: *const c_void, len: usize, flags: c_int) -> *mut c_void;
    fn devm_kasprintf(dev: *mut device, flags: c_int, fmt: *const c_char, ...) -> *mut c_char;
    fn devm_kstrdup_const(dev: *mut device, s: *const c_char, flags: c_int) -> *const c_char;
    fn strstr(s1: *const c_char, s2: *const c_char) -> *mut c_char;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn hda_codec_probe_complete(codec: *mut hda_codec) -> c_int;
    fn snd_soc_add_pcm_runtimes(
        card: *mut snd_soc_card,
        links: *mut snd_soc_dai_link,
        num_links: c_int,
    ) -> c_int;
    fn device_is_registered(dev: *const device) -> bool;
    fn hda_codec_is_display(codec: *mut hda_codec) -> bool;
    fn devm_snd_soc_register_deferrable_card(dev: *mut device, card: *mut snd_soc_card) -> c_int;
    fn __platform_driver_register(driver: *mut platform_driver, owner: *mut c_void) -> c_int;
    fn platform_driver_unregister(driver: *mut platform_driver);
}

unsafe fn list_entry<T>(ptr: *mut list_head, member_offset: usize) -> *mut T {
    (ptr as *mut u8).sub(member_offset) as *mut T
}

unsafe fn hda_pcm_list_offset() -> usize {
    let uninit = core::mem::MaybeUninit::<hda_pcm>::uninit();
    let base = uninit.as_ptr();
    unsafe { (&raw const (*base).list as usize) - (base as usize) }
}

unsafe fn snd_soc_pcm_runtime_list_offset() -> usize {
    let uninit = core::mem::MaybeUninit::<snd_soc_pcm_runtime>::uninit();
    let base = uninit.as_ptr();
    unsafe { (&raw const (*base).list as usize) - (base as usize) }
}

unsafe fn list_first_entry_hda_pcm(head: *mut list_head) -> *mut hda_pcm {
    unsafe { list_entry((*head).next, hda_pcm_list_offset()) }
}

unsafe fn list_next_entry_hda_pcm(pos: *mut hda_pcm) -> *mut hda_pcm {
    unsafe { list_entry((*pos).list.next, hda_pcm_list_offset()) }
}

unsafe fn list_empty(head: *const list_head) -> bool {
    unsafe { (*head).next == head as *mut list_head }
}

unsafe extern "C" fn avs_create_dai_links(
    dev: *mut device,
    codec: *mut hda_codec,
    pcm_count: c_int,
    links: *mut *mut snd_soc_dai_link,
) -> c_int {
    let mut platform: *mut snd_soc_dai_link_component;
    let dl: *mut snd_soc_dai_link;
    let mut pcm: *mut hda_pcm;
    let cname = unsafe { dev_name(&raw const (*codec).core.dev) };
    let mut i: c_int;

    dl = unsafe {
        devm_kcalloc(
            dev,
            pcm_count as usize,
            size_of::<snd_soc_dai_link>(),
            GFP_KERNEL,
        ) as *mut snd_soc_dai_link
    };
    platform = unsafe {
        devm_kzalloc(dev, size_of::<snd_soc_dai_link_component>(), GFP_KERNEL)
            as *mut snd_soc_dai_link_component
    };
    if dl.is_null() || platform.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*platform).name = dev_name(dev);
        pcm = list_first_entry_hda_pcm(&raw mut (*codec).pcm_list_head);
    }

    i = 0;
    while i < pcm_count {
        unsafe {
            (*dl.add(i as usize)).name =
                devm_kasprintf(dev, GFP_KERNEL, b"%s link%d\0".as_ptr() as *const c_char, cname, i);
            if (*dl.add(i as usize)).name.is_null() {
                return -ENOMEM;
            }

            (*dl.add(i as usize)).id = i;
            (*dl.add(i as usize)).nonatomic = 1;
            (*dl.add(i as usize)).no_pcm = 1;
            (*dl.add(i as usize)).platforms = platform;
            (*dl.add(i as usize)).num_platforms = 1;
            (*dl.add(i as usize)).ignore_pmdown_time = 1;

            (*dl.add(i as usize)).codecs =
                devm_kzalloc(dev, size_of::<snd_soc_dai_link_component>(), GFP_KERNEL)
                    as *mut snd_soc_dai_link_component;
            (*dl.add(i as usize)).cpus =
                devm_kzalloc(dev, size_of::<snd_soc_dai_link_component>(), GFP_KERNEL)
                    as *mut snd_soc_dai_link_component;
            if (*dl.add(i as usize)).codecs.is_null() || (*dl.add(i as usize)).cpus.is_null() {
                return -ENOMEM;
            }

            (*(*dl.add(i as usize)).cpus).dai_name =
                devm_kasprintf(dev, GFP_KERNEL, b"%s-cpu%d\0".as_ptr() as *const c_char, cname, i);
            if (*(*dl.add(i as usize)).cpus).dai_name.is_null() {
                return -ENOMEM;
            }

            (*(*dl.add(i as usize)).codecs).name = devm_kstrdup_const(dev, cname, GFP_KERNEL);
            if (*(*dl.add(i as usize)).codecs).name.is_null() {
                return -ENOMEM;
            }

            (*(*dl.add(i as usize)).codecs).dai_name = (*pcm).name;
            (*dl.add(i as usize)).num_codecs = 1;
            (*dl.add(i as usize)).num_cpus = 1;

            i += 1;
            pcm = list_next_entry_hda_pcm(pcm);
        }
    }

    unsafe {
        *links = dl;
    }
    0
}

/* Should be aligned with SectionPCM's name from topology */

unsafe extern "C" fn avs_card_hdmi_pcm_at(
    card: *mut snd_soc_card,
    hdmi_idx: c_int,
) -> *mut snd_pcm {
    let mut rtd: *mut snd_soc_pcm_runtime;
    let dir = SNDRV_PCM_STREAM_PLAYBACK;
    let head = unsafe { &raw mut (*card).rtd_list };

    unsafe {
        rtd = list_entry((*head).next, snd_soc_pcm_runtime_list_offset());
        while &raw mut (*rtd).list != head {
            let spcm: *mut snd_pcm;
            let mut n: c_int = 0;
            let ret: c_int;

            spcm = if !(*rtd).pcm.is_null() {
                (*(*rtd).pcm).streams[dir as usize].pcm
            } else {
                ptr::null_mut()
            };
            if spcm.is_null() || strstr((*spcm).id.as_ptr(), FEDAI_NAME_PREFIX).is_null() {
                rtd = list_entry((*rtd).list.next, snd_soc_pcm_runtime_list_offset());
                continue;
            }

            ret = sscanf(
                (*spcm).id.as_ptr(),
                b"HDMI%d\0".as_ptr() as *const c_char,
                &mut n as *mut c_int,
            );
            if ret != 1 {
                rtd = list_entry((*rtd).list.next, snd_soc_pcm_runtime_list_offset());
                continue;
            }
            if n == hdmi_idx {
                return (*rtd).pcm;
            }

            rtd = list_entry((*rtd).list.next, snd_soc_pcm_runtime_list_offset());
        }
    }

    ptr::null_mut()
}

unsafe extern "C" fn avs_card_late_probe(card: *mut snd_soc_card) -> c_int {
    let mach = unsafe { dev_get_platdata((*card).dev) as *mut snd_soc_acpi_mach };
    let pdata = unsafe { (*mach).pdata };
    let codec = unsafe { (*pdata).codec };
    let mut hpcm: *mut hda_pcm;
    /* Topology pcm indexing is 1-based */
    let mut i: c_int = 1;
    let head = unsafe { &raw mut (*codec).pcm_list_head };

    unsafe {
        hpcm = list_entry((*head).next, hda_pcm_list_offset());
        while &raw mut (*hpcm).list != head {
            let spcm: *mut snd_pcm;

            spcm = avs_card_hdmi_pcm_at(card, i);
            if !spcm.is_null() {
                (*hpcm).pcm = spcm;
                (*hpcm).device = (*spcm).device;
                dev_info(
                    (*card).dev,
                    b"%s: mapping HDMI converter %d to PCM %d (%p)\n\0".as_ptr() as *const c_char,
                    b"avs_card_late_probe\0".as_ptr() as *const c_char,
                    i,
                    (*hpcm).device,
                    spcm,
                );
            } else {
                (*hpcm).pcm = ptr::null_mut();
                (*hpcm).device = SNDRV_PCM_INVALID_DEVICE;
                dev_warn(
                    (*card).dev,
                    b"%s: no PCM in topology for HDMI converter %d\n\0".as_ptr() as *const c_char,
                    b"avs_card_late_probe\0".as_ptr() as *const c_char,
                    i,
                );
            }
            i += 1;
            hpcm = list_entry((*hpcm).list.next, hda_pcm_list_offset());
        }

        hda_codec_probe_complete(codec)
    }
}

unsafe extern "C" fn avs_probing_link_init(rtm: *mut snd_soc_pcm_runtime) -> c_int {
    let mach: *mut snd_soc_acpi_mach;
    let pdata: *mut avs_mach_pdata;
    let mut links: *mut snd_soc_dai_link = ptr::null_mut();
    let card = unsafe { (*rtm).card };
    let codec: *mut hda_codec;
    let mut pcm: *mut hda_pcm;
    let mut ret: c_int;
    let mut pcm_count: c_int = 0;

    unsafe {
        mach = dev_get_platdata((*card).dev) as *mut snd_soc_acpi_mach;
        pdata = (*mach).pdata;
        codec = (*pdata).codec;

        if list_empty(&raw const (*codec).pcm_list_head) {
            return -EINVAL;
        }
        let head = &raw mut (*codec).pcm_list_head;
        pcm = list_entry((*head).next, hda_pcm_list_offset());
        while &raw mut (*pcm).list != head {
            pcm_count += 1;
            pcm = list_entry((*pcm).list.next, hda_pcm_list_offset());
        }

        ret = avs_create_dai_links((*card).dev, codec, pcm_count, &mut links);
        if ret < 0 {
            dev_err(
                (*card).dev,
                b"create links failed: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }

        ret = snd_soc_add_pcm_runtimes(card, links, pcm_count);
        if ret < 0 {
            dev_err(
                (*card).dev,
                b"add links failed: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }
    }

    0
}

static mut probing_link: snd_soc_dai_link = snd_soc_dai_link {
    name: b"probing-LINK\0".as_ptr() as *const c_char,
    id: -1,
    nonatomic: 1,
    no_pcm: 1,
    cpus: unsafe { &raw mut snd_soc_dummy_dlc },
    num_cpus: 1,
    init: Some(avs_probing_link_init),
    platforms: ptr::null_mut(),
    num_platforms: 0,
    ignore_pmdown_time: 0,
    codecs: ptr::null_mut(),
    num_codecs: 0,
};

unsafe extern "C" fn avs_hdaudio_probe(pdev: *mut platform_device) -> c_int {
    let binder: *mut snd_soc_dai_link;
    let mach: *mut snd_soc_acpi_mach;
    let pdata: *mut avs_mach_pdata;
    let card: *mut snd_soc_card;
    let dev = unsafe { &raw mut (*pdev).dev };
    let codec: *mut hda_codec;

    unsafe {
        mach = dev_get_platdata(dev) as *mut snd_soc_acpi_mach;
        pdata = (*mach).pdata;
        codec = (*pdata).codec;

        /* codec may be unloaded before card's probe() fires */
        if !device_is_registered(&raw const (*codec).core.dev) {
            return -ENODEV;
        }

        binder = devm_kmemdup(
            dev,
            &raw const probing_link as *const c_void,
            size_of::<snd_soc_dai_link>(),
            GFP_KERNEL,
        ) as *mut snd_soc_dai_link;
        if binder.is_null() {
            return -ENOMEM;
        }

        (*binder).platforms =
            devm_kzalloc(dev, size_of::<snd_soc_dai_link_component>(), GFP_KERNEL)
                as *mut snd_soc_dai_link_component;
        (*binder).codecs =
            devm_kzalloc(dev, size_of::<snd_soc_dai_link_component>(), GFP_KERNEL)
                as *mut snd_soc_dai_link_component;
        if (*binder).platforms.is_null() || (*binder).codecs.is_null() {
            return -ENOMEM;
        }

        (*(*binder).codecs).name =
            devm_kstrdup_const(dev, dev_name(&raw const (*codec).core.dev), GFP_KERNEL);
        if (*(*binder).codecs).name.is_null() {
            return -ENOMEM;
        }

        (*(*binder).platforms).name = dev_name(dev);
        (*binder).num_platforms = 1;
        (*(*binder).codecs).dai_name = b"codec-probing-DAI\0".as_ptr() as *const c_char;
        (*binder).num_codecs = 1;

        card = devm_kzalloc(dev, size_of::<snd_soc_card>(), GFP_KERNEL) as *mut snd_soc_card;
        if card.is_null() {
            return -ENOMEM;
        }

        if (*pdata).obsolete_card_names {
            (*card).name = devm_kasprintf(
                dev,
                GFP_KERNEL,
                b"hdaudioB%dD%d\0".as_ptr() as *const c_char,
                (*(*codec).bus).core.idx,
                (*codec).core.addr,
            );
            if (*card).name.is_null() {
                return -ENOMEM;
            }
        } else {
            (*card).driver_name = b"avs_hdaudio\0".as_ptr() as *const c_char;
            if hda_codec_is_display(codec) {
                (*card).name = b"AVS HDMI\0".as_ptr() as *const c_char;
                (*card).long_name = (*card).name;
            } else {
                (*card).name = b"AVS HD-Audio\0".as_ptr() as *const c_char;
                (*card).long_name = (*card).name;
            }
        }

        (*card).dev = dev;
        (*card).owner = THIS_MODULE;
        (*card).dai_link = binder;
        (*card).num_links = 1;
        (*card).fully_routed = true;
        if hda_codec_is_display(codec) {
            (*card).late_probe = Some(avs_card_late_probe);
        }

        devm_snd_soc_register_deferrable_card(dev, card)
    }
}

static avs_hdaudio_driver_ids: [platform_device_id; 2] = [
    platform_device_id {
        name: [
            b'a' as c_char,
            b'v' as c_char,
            b's' as c_char,
            b'_' as c_char,
            b'h' as c_char,
            b'd' as c_char,
            b'a' as c_char,
            b'u' as c_char,
            b'd' as c_char,
            b'i' as c_char,
            b'o' as c_char,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
    },
    platform_device_id { name: [0; 32] },
];

// MODULE_DEVICE_TABLE(platform, avs_hdaudio_driver_ids);

static mut avs_hdaudio_driver: platform_driver = platform_driver {
    probe: Some(avs_hdaudio_probe),
    driver: platform_driver_inner {
        name: b"avs_hdaudio\0".as_ptr() as *const c_char,
        pm: unsafe { &raw const snd_soc_pm_ops },
    },
    id_table: avs_hdaudio_driver_ids.as_ptr(),
};

// module_platform_driver(avs_hdaudio_driver)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn avs_hdaudio_driver_init() -> c_int {
    unsafe { __platform_driver_register(&raw mut avs_hdaudio_driver, THIS_MODULE) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn avs_hdaudio_driver_exit() {
    unsafe {
        platform_driver_unregister(&raw mut avs_hdaudio_driver);
    }
}

// MODULE_DESCRIPTION("Intel HD-Audio machine driver");
// MODULE_AUTHOR("Cezary Rojewski <cezary.rojewski@intel.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
