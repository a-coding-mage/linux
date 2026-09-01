// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2018 Intel Corporation
//
// Authors: Keyon Jie <yang.jie@linux.intel.com>
//

// Translated from the C implementation source. C include dependencies:
// linux/module.h, sound/hdaudio_ext.h, sound/hda_register.h,
// sound/hda_codec.h, sound/hda_i915.h, sound/sof.h, ../ops.h, hda.h.

// The following items are supplied by external kernel/SOF/HDA bindings.
// This file intentionally declares dependencies but does not implement them.
extern "C" {
    static mut hda_codec_mask: i32;

    fn device_attach(dev: *mut device) -> i32;
    fn dev_set_drvdata(dev: *mut device, data: *mut core::ffi::c_void);
    fn devm_kzalloc(
        dev: *mut device,
        size: usize,
        flags: gfp_t,
    ) -> *mut core::ffi::c_void;
    fn hda_codec_dev(codec: *mut hda_codec) -> *mut device;
    fn hda_codec_jack_check(sdev: *mut snd_sof_dev);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn pm_request_resume(dev: *mut device) -> i32;
    fn put_device(dev: *mut device);
    fn request_module(name: *const core::ffi::c_char) -> i32;
    fn snd_hda_codec_device_init(
        bus: *mut hda_bus,
        addr: i32,
        fmt: *const core::ffi::c_char,
        ...
    ) -> *mut hda_codec;
    fn snd_hdac_bus_get_response(bus: *mut hdac_bus, addr: i32, resp: *mut u32) -> i32;
    fn snd_hdac_bus_init_cmd_io(bus: *mut hdac_bus);
    fn snd_hdac_bus_send_cmd(bus: *mut hdac_bus, cmd: u32) -> i32;
    fn snd_hdac_bus_stop_cmd_io(bus: *mut hdac_bus);
    fn snd_hdac_bus_update_rirb(bus: *mut hdac_bus);
    fn snd_hdac_chip_readb(bus: *mut hdac_bus, reg: u32) -> u8;
    fn snd_hdac_chip_readw(bus: *mut hdac_bus, reg: u32) -> u16;
    fn snd_hdac_chip_updatew(bus: *mut hdac_bus, reg: u32, mask: u32, val: u32);
    fn snd_hdac_chip_writeb(bus: *mut hdac_bus, reg: u32, val: u8);
    fn snd_hdac_chip_writew(bus: *mut hdac_bus, reg: u32, val: u16);
    fn snd_hdac_codec_modalias(codec: *mut hdac_device, buf: *mut core::ffi::c_char, size: usize);
    fn snd_hdac_device_register(codec: *mut hdac_device) -> i32;
    fn snd_hdac_device_unregister(codec: *mut hdac_device);
    fn snd_hdac_display_power(bus: *mut hdac_bus, idx: i32, enable: bool);
    fn snd_hdac_ext_bus_device_remove(bus: *mut hdac_bus);
    fn snd_hdac_i915_exit(bus: *mut hdac_bus) -> i32;
    fn snd_hdac_i915_init(bus: *mut hdac_bus) -> i32;
    fn snd_hdac_set_codec_wakeup(bus: *mut hdac_bus, status: bool);
    fn sof_debug_check_flag(flag: u32) -> bool;
    fn sof_to_bus(sdev: *mut snd_sof_dev) -> *mut hdac_bus;
    fn sof_to_hbus(sdev: *mut snd_sof_dev) -> *mut hda_bus;
    fn strcmp(a: *const core::ffi::c_char, b: *const core::ffi::c_char) -> i32;
    fn to_hda_bus(bus: *mut hdac_bus) -> *mut hda_bus;
}

type gfp_t = u32;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_sof_dev {
    pub dev: *mut device,
}

#[repr(C)]
pub struct hdac_device {
    pub dev: device,
    pub addr: u32,
    pub type_: i32,
}

#[repr(C)]
pub struct hda_jack_tbl {
    pub used: i32,
}

#[repr(C)]
pub struct hda_codec {
    pub core: hdac_device,
    pub probe_id: u32,
    pub jacktbl: hda_jack_tbl,
}

#[repr(C)]
pub struct hdac_bus {
    pub dev: *mut device,
    pub idx: i32,
    pub codec_mask: core::ffi::c_ulong,
    pub audio_component: *mut core::ffi::c_void,
    pub cmd_dma_state: bool,
    pub cmd_mutex: mutex,
}

#[repr(C)]
pub struct hda_bus {
    pub core: hdac_bus,
    pub modelname: *const core::ffi::c_char,
}

#[repr(C)]
pub struct hdac_hda_priv {
    pub codec: *mut hda_codec,
    pub dev_index: i32,
    pub need_display_power: bool,
}

const MODULE_NAME_LEN: usize = 64;
const CODEC_PROBE_RETRIES: i32 = 3;
const IDISP_VID_INTEL: u32 = 0x80860000;

const AC_NODE_ROOT: u32 = 0;
const AC_VERB_PARAMETERS: u32 = 0xf00;
const AC_PAR_VENDOR_ID: u32 = 0;
const GFP_KERNEL: gfp_t = 0;
const HDA_CODEC_ID_GENERIC: u32 = 0x00000001;
const HDA_CODEC_IDX_CONTROLLER: i32 = 0;
const HDA_DEV_LEGACY: i32 = 0;
const HDA_MAX_CODECS: i32 = 8;
const RIRB_INT_MASK: u8 = 0x05;
const RIRB_INT_RESPONSE: u8 = 0x01;
const RIRBSTS: u32 = 0;
const SOF_DBG_FORCE_NOCODEC: u32 = 0;
const STATESTS: u32 = 0;
const STATESTS_INT_MASK: u32 = 0x7fff;

const EIO: i32 = 5;
const ENOENT: i32 = 2;
const ENOMEM: i32 = 12;

const fn BIT(n: u32) -> u32 {
    1u32 << n
}

unsafe fn IS_ERR<T>(ptr: *mut T) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

unsafe fn PTR_ERR_OR_ZERO<T>(ptr: *mut T) -> i32 {
    if IS_ERR(ptr) {
        ptr as isize as i32
    } else {
        0
    }
}

unsafe fn IS_ENABLED(_config: bool) -> bool {
    _config
}

const CONFIG_SND_SOC_SOF_HDA_AUDIO_CODEC: bool = true;
const CONFIG_SND_SOC_SOF_NOCODEC_DEBUG_SUPPORT: bool = false;
const CONFIG_SND_HDA_GENERIC: bool = false;
const CONFIG_SND_HDA_CODEC_HDMI: bool = true;
const MODULE: bool = false;
const CONFIG_SND_HDA_GENERIC_MODULE: bool = false;

unsafe fn HDA_IDISP_CODEC(mask: core::ffi::c_ulong) -> bool {
    (mask & BIT(HDA_CODEC_IDX_CONTROLLER as u32) as core::ffi::c_ulong) != 0
}

// module_param_named(codec_mask, hda_codec_mask, int, 0444);
// MODULE_PARM_DESC(codec_mask, "SOF HDA codec mask for probing");

// load the legacy HDA codec driver
unsafe fn request_codec_module(codec: *mut hda_codec) -> i32 {
    if IS_ENABLED(MODULE) {
        let mut alias: [core::ffi::c_char; MODULE_NAME_LEN] = [0; MODULE_NAME_LEN];
        let mut mod_: *const core::ffi::c_char = core::ptr::null();

        match (*codec).probe_id {
            HDA_CODEC_ID_GENERIC => {
                if IS_ENABLED(CONFIG_SND_HDA_GENERIC_MODULE) {
                    mod_ = c"snd-hda-codec-generic".as_ptr();
                }
            }
            _ => {
                snd_hdac_codec_modalias(&mut (*codec).core, alias.as_mut_ptr(), alias.len());
                mod_ = alias.as_ptr();
            }
        }

        if !mod_.is_null() {
            // dev_dbg(&codec->core.dev, "loading codec module: %s\n", mod);
            request_module(mod_);
        }
    }

    device_attach(hda_codec_dev(codec))
}

unsafe fn hda_codec_load_module(codec: *mut hda_codec) -> i32 {
    let mut ret: i32;

    ret = snd_hdac_device_register(&mut (*codec).core);
    if ret != 0 {
        // dev_err(&codec->core.dev, "failed to register hdac device\n");
        put_device(&mut (*codec).core.dev);
        return ret;
    }

    ret = request_codec_module(codec);
    if ret <= 0 {
        (*codec).probe_id = HDA_CODEC_ID_GENERIC;
        ret = request_codec_module(codec);
    }

    ret
}

// enable controller wake up event for all codecs with jack connectors
#[no_mangle]
pub unsafe extern "C" fn hda_codec_jack_wake_enable(sdev: *mut snd_sof_dev, enable: bool) {
    let hbus: *mut hda_bus = sof_to_hbus(sdev);
    let bus: *mut hdac_bus = sof_to_bus(sdev);
    let mut mask: u32 = 0;
    let mut val: u32 = 0;

    if IS_ENABLED(CONFIG_SND_SOC_SOF_NOCODEC_DEBUG_SUPPORT)
        && sof_debug_check_flag(SOF_DBG_FORCE_NOCODEC)
    {
        return;
    }

    if enable {
        // list_for_each_codec(codec, hbus)
        for addr in 0..HDA_MAX_CODECS {
            if ((*bus).codec_mask & BIT(addr as u32) as core::ffi::c_ulong) == 0 {
                continue;
            }

            let codec = snd_hda_codec_device_init(
                hbus,
                addr,
                c"ehdaudio%dD%d".as_ptr(),
                (*bus).idx,
                addr,
            );

            mask |= BIT((*codec).core.addr);
            if (*codec).jacktbl.used != 0 {
                val |= BIT((*codec).core.addr);
            }
        }
    } else {
        // list_for_each_codec(codec, hbus)
        for addr in 0..HDA_MAX_CODECS {
            if ((*bus).codec_mask & BIT(addr as u32) as core::ffi::c_ulong) == 0 {
                continue;
            }

            let codec = snd_hda_codec_device_init(
                hbus,
                addr,
                c"ehdaudio%dD%d".as_ptr(),
                (*bus).idx,
                addr,
            );

            mask |= BIT((*codec).core.addr);
        }
    }

    snd_hdac_chip_updatew(bus, WAKEEN, mask & STATESTS_INT_MASK, val);
}

const WAKEEN: u32 = 0;

// EXPORT_SYMBOL_NS_GPL(hda_codec_jack_wake_enable, "SND_SOC_SOF_HDA_AUDIO_CODEC");

// check jack status after resuming from suspend mode
#[no_mangle]
pub unsafe extern "C" fn hda_codec_jack_check_exported(sdev: *mut snd_sof_dev) {
    let hbus: *mut hda_bus = sof_to_hbus(sdev);
    let bus: *mut hdac_bus = sof_to_bus(sdev);

    if IS_ENABLED(CONFIG_SND_SOC_SOF_NOCODEC_DEBUG_SUPPORT)
        && sof_debug_check_flag(SOF_DBG_FORCE_NOCODEC)
    {
        return;
    }

    // list_for_each_codec(codec, hbus)
    for addr in 0..HDA_MAX_CODECS {
        if ((*bus).codec_mask & BIT(addr as u32) as core::ffi::c_ulong) == 0 {
            continue;
        }

        let codec = snd_hda_codec_device_init(
            hbus,
            addr,
            c"ehdaudio%dD%d".as_ptr(),
            (*bus).idx,
            addr,
        );

        /*
         * Wake up all jack-detecting codecs regardless whether an event
         * has been recorded in STATESTS
         */
        if (*codec).jacktbl.used != 0 {
            pm_request_resume(&mut (*codec).core.dev);
        }
    }
}

// EXPORT_SYMBOL_NS_GPL(hda_codec_jack_check, "SND_SOC_SOF_HDA_AUDIO_CODEC");

unsafe fn is_generic_config(bus: *mut hda_bus) -> bool {
    if IS_ENABLED(CONFIG_SND_HDA_GENERIC) {
        !(*bus).modelname.is_null() && strcmp((*bus).modelname, c"generic".as_ptr()) == 0
    } else {
        false
    }
}

unsafe fn hda_codec_device_init(bus: *mut hdac_bus, addr: i32, type_: i32) -> *mut hda_codec {
    let codec: *mut hda_codec;

    codec = snd_hda_codec_device_init(to_hda_bus(bus), addr, c"ehdaudio%dD%d".as_ptr(), (*bus).idx, addr);
    if IS_ERR(codec) {
        // dev_err(bus->dev, "device init failed for hdac device\n");
        return codec;
    }

    (*codec).core.type_ = type_;

    codec
}

// probe individual codec
unsafe fn hda_codec_probe(sdev: *mut snd_sof_dev, address: i32) -> i32 {
    let mut hda_priv: *mut hdac_hda_priv;
    let hbus: *mut hda_bus = sof_to_hbus(sdev);
    let codec: *mut hda_codec;
    let hda_cmd: u32 = ((address as u32) << 28)
        | (AC_NODE_ROOT << 20)
        | (AC_VERB_PARAMETERS << 8)
        | AC_PAR_VENDOR_ID;
    let mut resp: u32 = u32::MAX;
    let mut ret: i32;
    let mut retry: i32 = 0;

    loop {
        mutex_lock(&mut (*hbus).core.cmd_mutex);
        snd_hdac_bus_send_cmd(&mut (*hbus).core, hda_cmd);
        snd_hdac_bus_get_response(&mut (*hbus).core, address, &mut resp);
        mutex_unlock(&mut (*hbus).core.cmd_mutex);

        let old_retry = retry;
        retry += 1;
        if !(resp == u32::MAX && old_retry < CODEC_PROBE_RETRIES) {
            break;
        }
    }

    if resp == u32::MAX {
        return -EIO;
    }
    // dev_dbg(sdev->dev, "HDA codec #%d probed OK: response: %x\n", address, resp);

    hda_priv = devm_kzalloc(
        (*sdev).dev,
        core::mem::size_of::<hdac_hda_priv>(),
        GFP_KERNEL,
    ) as *mut hdac_hda_priv;
    if hda_priv.is_null() {
        return -ENOMEM;
    }

    codec = hda_codec_device_init(&mut (*hbus).core, address, HDA_DEV_LEGACY);
    ret = PTR_ERR_OR_ZERO(codec);
    if ret < 0 {
        return ret;
    }

    (*hda_priv).codec = codec;
    (*hda_priv).dev_index = address;
    dev_set_drvdata(&mut (*codec).core.dev, hda_priv as *mut core::ffi::c_void);

    if (resp & 0xFFFF0000) == IDISP_VID_INTEL {
        if (*hbus).core.audio_component.is_null() {
            // dev_dbg(sdev->dev, "iDisp hw present but no driver\n");
            ret = -ENOENT;
            snd_hdac_device_unregister(&mut (*codec).core);
            put_device(&mut (*codec).core.dev);
            return ret;
        }
        (*hda_priv).need_display_power = true;
    }

    if is_generic_config(hbus) {
        (*codec).probe_id = HDA_CODEC_ID_GENERIC;
    } else {
        (*codec).probe_id = 0;
    }

    ret = hda_codec_load_module(codec);
    /*
     * handle ret==0 (no driver bound) as an error, but pass
     * other return codes without modification
     */
    if ret == 0 {
        ret = -ENOENT;
    }

    if ret < 0 {
        snd_hdac_device_unregister(&mut (*codec).core);
        put_device(&mut (*codec).core.dev);
    }

    ret
}

// Codec initialization
#[no_mangle]
pub unsafe extern "C" fn hda_codec_probe_bus(sdev: *mut snd_sof_dev) {
    let bus: *mut hdac_bus = sof_to_bus(sdev);
    let mut i: i32;
    let mut ret: i32;

    if IS_ENABLED(CONFIG_SND_SOC_SOF_NOCODEC_DEBUG_SUPPORT)
        && sof_debug_check_flag(SOF_DBG_FORCE_NOCODEC)
    {
        return;
    }

    // probe codecs in avail slots
    i = 0;
    while i < HDA_MAX_CODECS {
        if ((*bus).codec_mask & (1_u64 << i) as core::ffi::c_ulong) == 0 {
            i += 1;
            continue;
        }

        ret = hda_codec_probe(sdev, i);
        if ret < 0 {
            // dev_warn(bus->dev, "codec #%d probe error, ret: %d\n", i, ret);
            (*bus).codec_mask &= !(BIT(i as u32) as core::ffi::c_ulong);
        }

        i += 1;
    }
}

// EXPORT_SYMBOL_NS_GPL(hda_codec_probe_bus, "SND_SOC_SOF_HDA_AUDIO_CODEC");

#[no_mangle]
pub unsafe extern "C" fn hda_codec_check_for_state_change(sdev: *mut snd_sof_dev) {
    let bus: *mut hdac_bus = sof_to_bus(sdev);
    let mut codec_mask: u32;

    codec_mask = snd_hdac_chip_readw(bus, STATESTS) as u32;
    if codec_mask != 0 {
        hda_codec_jack_check(sdev);
        snd_hdac_chip_writew(bus, STATESTS, codec_mask as u16);
    }
}

// EXPORT_SYMBOL_NS_GPL(hda_codec_check_for_state_change, "SND_SOC_SOF_HDA_AUDIO_CODEC");

#[no_mangle]
pub unsafe extern "C" fn hda_codec_detect_mask(sdev: *mut snd_sof_dev) {
    let bus: *mut hdac_bus = sof_to_bus(sdev);

    if IS_ENABLED(CONFIG_SND_SOC_SOF_NOCODEC_DEBUG_SUPPORT)
        && sof_debug_check_flag(SOF_DBG_FORCE_NOCODEC)
    {
        return;
    }

    // detect codecs
    if (*bus).codec_mask == 0 {
        (*bus).codec_mask = snd_hdac_chip_readw(bus, STATESTS) as core::ffi::c_ulong;
        // dev_dbg(bus->dev, "codec_mask = 0x%lx\n", bus->codec_mask);
    }

    if hda_codec_mask != -1 {
        (*bus).codec_mask &= hda_codec_mask as core::ffi::c_ulong;
        // dev_dbg(bus->dev, "filtered codec_mask = 0x%lx\n", bus->codec_mask);
    }
}

// EXPORT_SYMBOL_NS_GPL(hda_codec_detect_mask, "SND_SOC_SOF_HDA_AUDIO_CODEC");

#[no_mangle]
pub unsafe extern "C" fn hda_codec_init_cmd_io(sdev: *mut snd_sof_dev) {
    let bus: *mut hdac_bus = sof_to_bus(sdev);

    if IS_ENABLED(CONFIG_SND_SOC_SOF_NOCODEC_DEBUG_SUPPORT)
        && sof_debug_check_flag(SOF_DBG_FORCE_NOCODEC)
    {
        return;
    }

    // initialize the codec command I/O
    snd_hdac_bus_init_cmd_io(bus);
}

// EXPORT_SYMBOL_NS_GPL(hda_codec_init_cmd_io, "SND_SOC_SOF_HDA_AUDIO_CODEC");

#[no_mangle]
pub unsafe extern "C" fn hda_codec_resume_cmd_io(sdev: *mut snd_sof_dev) {
    let bus: *mut hdac_bus = sof_to_bus(sdev);

    if IS_ENABLED(CONFIG_SND_SOC_SOF_NOCODEC_DEBUG_SUPPORT)
        && sof_debug_check_flag(SOF_DBG_FORCE_NOCODEC)
    {
        return;
    }

    // set up CORB/RIRB buffers if was on before suspend
    if (*bus).cmd_dma_state {
        snd_hdac_bus_init_cmd_io(bus);
    }
}

// EXPORT_SYMBOL_NS_GPL(hda_codec_resume_cmd_io, "SND_SOC_SOF_HDA_AUDIO_CODEC");

#[no_mangle]
pub unsafe extern "C" fn hda_codec_stop_cmd_io(sdev: *mut snd_sof_dev) {
    let bus: *mut hdac_bus = sof_to_bus(sdev);

    if IS_ENABLED(CONFIG_SND_SOC_SOF_NOCODEC_DEBUG_SUPPORT)
        && sof_debug_check_flag(SOF_DBG_FORCE_NOCODEC)
    {
        return;
    }

    // initialize the codec command I/O
    snd_hdac_bus_stop_cmd_io(bus);
}

// EXPORT_SYMBOL_NS_GPL(hda_codec_stop_cmd_io, "SND_SOC_SOF_HDA_AUDIO_CODEC");

#[no_mangle]
pub unsafe extern "C" fn hda_codec_suspend_cmd_io(sdev: *mut snd_sof_dev) {
    let bus: *mut hdac_bus = sof_to_bus(sdev);

    if IS_ENABLED(CONFIG_SND_SOC_SOF_NOCODEC_DEBUG_SUPPORT)
        && sof_debug_check_flag(SOF_DBG_FORCE_NOCODEC)
    {
        return;
    }

    // stop the CORB/RIRB DMA if it is On
    if (*bus).cmd_dma_state {
        snd_hdac_bus_stop_cmd_io(bus);
    }
}

// EXPORT_SYMBOL_NS_GPL(hda_codec_suspend_cmd_io, "SND_SOC_SOF_HDA_AUDIO_CODEC");

#[no_mangle]
pub unsafe extern "C" fn hda_codec_rirb_status_clear(sdev: *mut snd_sof_dev) {
    let bus: *mut hdac_bus = sof_to_bus(sdev);

    if IS_ENABLED(CONFIG_SND_SOC_SOF_NOCODEC_DEBUG_SUPPORT)
        && sof_debug_check_flag(SOF_DBG_FORCE_NOCODEC)
    {
        return;
    }

    // clear rirb status
    snd_hdac_chip_writeb(bus, RIRBSTS, RIRB_INT_MASK);
}

// EXPORT_SYMBOL_NS_GPL(hda_codec_rirb_status_clear, "SND_SOC_SOF_HDA_AUDIO_CODEC");

#[no_mangle]
pub unsafe extern "C" fn hda_codec_set_codec_wakeup(sdev: *mut snd_sof_dev, status: bool) {
    let bus: *mut hdac_bus = sof_to_bus(sdev);

    if sof_debug_check_flag(SOF_DBG_FORCE_NOCODEC) {
        return;
    }

    snd_hdac_set_codec_wakeup(bus, status);
}

// EXPORT_SYMBOL_NS_GPL(hda_codec_set_codec_wakeup, "SND_SOC_SOF_HDA_AUDIO_CODEC");

#[no_mangle]
pub unsafe extern "C" fn hda_codec_check_rirb_status(sdev: *mut snd_sof_dev) -> bool {
    let bus: *mut hdac_bus = sof_to_bus(sdev);
    let mut active: bool = false;
    let rirb_status: u32;

    if IS_ENABLED(CONFIG_SND_SOC_SOF_NOCODEC_DEBUG_SUPPORT)
        && sof_debug_check_flag(SOF_DBG_FORCE_NOCODEC)
    {
        return false;
    }

    rirb_status = snd_hdac_chip_readb(bus, RIRBSTS) as u32;
    if (rirb_status & RIRB_INT_MASK as u32) != 0 {
        /*
         * Clearing the interrupt status here ensures
         * that no interrupt gets masked after the RIRB
         * wp is read in snd_hdac_bus_update_rirb.
         */
        snd_hdac_chip_writeb(bus, RIRBSTS, RIRB_INT_MASK);
        active = true;
        if (rirb_status & RIRB_INT_RESPONSE as u32) != 0 {
            snd_hdac_bus_update_rirb(bus);
        }
    }
    active
}

// EXPORT_SYMBOL_NS_GPL(hda_codec_check_rirb_status, "SND_SOC_SOF_HDA_AUDIO_CODEC");

#[no_mangle]
pub unsafe extern "C" fn hda_codec_device_remove(sdev: *mut snd_sof_dev) {
    let bus: *mut hdac_bus = sof_to_bus(sdev);

    if IS_ENABLED(CONFIG_SND_SOC_SOF_NOCODEC_DEBUG_SUPPORT)
        && sof_debug_check_flag(SOF_DBG_FORCE_NOCODEC)
    {
        return;
    }

    // codec removal, invoke bus_device_remove
    snd_hdac_ext_bus_device_remove(bus);
}

// EXPORT_SYMBOL_NS_GPL(hda_codec_device_remove, "SND_SOC_SOF_HDA_AUDIO_CODEC");

// #if IS_ENABLED(CONFIG_SND_SOC_SOF_HDA_AUDIO_CODEC) && IS_ENABLED(CONFIG_SND_HDA_CODEC_HDMI)

#[no_mangle]
pub unsafe extern "C" fn hda_codec_i915_display_power(sdev: *mut snd_sof_dev, enable: bool) {
    let bus: *mut hdac_bus = sof_to_bus(sdev);

    if IS_ENABLED(CONFIG_SND_SOC_SOF_NOCODEC_DEBUG_SUPPORT)
        && sof_debug_check_flag(SOF_DBG_FORCE_NOCODEC)
    {
        return;
    }

    if HDA_IDISP_CODEC((*bus).codec_mask) {
        // dev_dbg(bus->dev, "Turning i915 HDAC power %d\n", enable);
        snd_hdac_display_power(bus, HDA_CODEC_IDX_CONTROLLER, enable);
    }
}

// EXPORT_SYMBOL_NS_GPL(hda_codec_i915_display_power, "SND_SOC_SOF_HDA_AUDIO_CODEC_I915");

#[no_mangle]
pub unsafe extern "C" fn hda_codec_i915_init(sdev: *mut snd_sof_dev) -> i32 {
    let bus: *mut hdac_bus = sof_to_bus(sdev);
    let ret: i32;

    if IS_ENABLED(CONFIG_SND_SOC_SOF_NOCODEC_DEBUG_SUPPORT)
        && sof_debug_check_flag(SOF_DBG_FORCE_NOCODEC)
    {
        return 0;
    }

    // i915 exposes a HDA codec for HDMI audio
    ret = snd_hdac_i915_init(bus);
    if ret < 0 {
        return ret;
    }

    // codec_mask not yet known, power up for probe
    snd_hdac_display_power(bus, HDA_CODEC_IDX_CONTROLLER, true);

    0
}

// EXPORT_SYMBOL_NS_GPL(hda_codec_i915_init, "SND_SOC_SOF_HDA_AUDIO_CODEC_I915");

#[no_mangle]
pub unsafe extern "C" fn hda_codec_i915_exit(sdev: *mut snd_sof_dev) -> i32 {
    let bus: *mut hdac_bus = sof_to_bus(sdev);

    if IS_ENABLED(CONFIG_SND_SOC_SOF_NOCODEC_DEBUG_SUPPORT)
        && sof_debug_check_flag(SOF_DBG_FORCE_NOCODEC)
    {
        return 0;
    }

    if (*bus).audio_component.is_null() {
        return 0;
    }

    // power down unconditionally
    snd_hdac_display_power(bus, HDA_CODEC_IDX_CONTROLLER, false);

    snd_hdac_i915_exit(bus)
}

// EXPORT_SYMBOL_NS_GPL(hda_codec_i915_exit, "SND_SOC_SOF_HDA_AUDIO_CODEC_I915");

// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_DESCRIPTION("SOF support for HDaudio codecs");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
