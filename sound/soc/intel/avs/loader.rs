// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021-2022 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
//          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

const AVS_ROM_STS_MASK: u32 = 0xFF;
const AVS_ROM_INIT_DONE: u32 = 0x1;
const SKL_ROM_BASEFW_ENTERED: u32 = 0xF;
const APL_ROM_FW_ENTERED: u32 = 0x5;
const AVS_ROM_INIT_POLLING_US: u32 = 5;
const SKL_ROM_INIT_TIMEOUT_US: u32 = 1000000;
const APL_ROM_INIT_TIMEOUT_US: u32 = 300000;
const APL_ROM_INIT_RETRIES: c_int = 3;

const AVS_FW_INIT_POLLING_US: u32 = 500;
const AVS_FW_INIT_TIMEOUT_MS: u32 = 3000;
const AVS_FW_INIT_TIMEOUT_US: u32 = AVS_FW_INIT_TIMEOUT_MS * 1000;

const AVS_CLDMA_START_DELAY_MS: u32 = 100;

const AVS_ROOT_DIR: &[u8] = b"intel/avs\0";
const AVS_BASEFW_FILENAME: &[u8] = b"dsp_basefw.bin\0";
const AVS_EXT_MANIFEST_MAGIC: u32 = 0x31454124;
const SKL_MANIFEST_MAGIC: u32 = 0x00000006;
const SKL_ADSPFW_OFFSET: c_int = 0x284;
const APL_MANIFEST_MAGIC: u32 = 0x44504324;
const APL_ADSPFW_OFFSET: c_int = 0x2000;

/* Occasionally, engineering (release candidate) firmware is provided for testing. */
static mut debug_ignore_fw_version: bool = false;
/* module_param_named(ignore_fw_version, debug_ignore_fw_version, bool, 0444); */
/* MODULE_PARM_DESC(ignore_fw_version, "Ignore firmware version check 0=no (default), 1=yes"); */

const AVS_LIB_NAME_SIZE: usize = 8;

#[repr(C, packed)]
pub struct avs_fw_manifest {
    pub id: u32,
    pub len: u32,
    pub name: [c_char; AVS_LIB_NAME_SIZE],
    pub preload_page_count: u32,
    pub img_flags: u32,
    pub feature_mask: u32,
    pub version: avs_fw_version,
}

const _: [(); 36] = [(); size_of::<avs_fw_manifest>()];

#[repr(C, packed)]
pub struct avs_fw_ext_manifest {
    pub id: u32,
    pub len: u32,
    pub version_major: u16,
    pub version_minor: u16,
    pub entries: u32,
}

const _: [(); 16] = [(); size_of::<avs_fw_ext_manifest>()];

unsafe fn avs_fw_ext_manifest_strip(fw: *mut firmware) -> c_int {
    let man: *mut avs_fw_ext_manifest;

    if (*fw).size < size_of::<avs_fw_ext_manifest>() {
        return -EINVAL;
    }

    man = (*fw).data as *mut avs_fw_ext_manifest;
    if (*man).id == AVS_EXT_MANIFEST_MAGIC {
        (*fw).data = (*fw).data.add((*man).len as usize);
        (*fw).size -= (*man).len as usize;
    }

    0
}

unsafe fn avs_fw_manifest_offset(fw: *mut firmware) -> c_int {
    /* Header type found in first DWORD of fw binary. */
    let magic: u32 = *((*fw).data as *mut u32);

    match magic {
        SKL_MANIFEST_MAGIC => SKL_ADSPFW_OFFSET,
        APL_MANIFEST_MAGIC => APL_ADSPFW_OFFSET,
        _ => -EINVAL,
    }
}

unsafe fn avs_fw_manifest_strip_verify(
    adev: *mut avs_dev,
    fw: *mut firmware,
    min: *const avs_fw_version,
) -> c_int {
    let man: *mut avs_fw_manifest;
    let mut offset: c_int;
    let mut ret: c_int;

    ret = avs_fw_ext_manifest_strip(fw);
    if ret != 0 {
        return ret;
    }

    offset = avs_fw_manifest_offset(fw);
    if offset < 0 {
        return offset;
    }

    if (*fw).size < offset as usize + size_of::<avs_fw_manifest>() {
        return -EINVAL;
    }
    if min.is_null() {
        return 0;
    }

    man = (*fw).data.add(offset as usize) as *mut avs_fw_manifest;
    if (*man).version.major != (*min).major
        || (*man).version.minor != (*min).minor
        || (*man).version.hotfix != (*min).hotfix
        || (*man).version.build < (*min).build
    {
        dev_warn(
            (*adev).dev,
            b"bad FW version %d.%d.%d.%d, expected %d.%d.%d.%d or newer\n\0".as_ptr()
                as *const c_char,
            (*man).version.major,
            (*man).version.minor,
            (*man).version.hotfix,
            (*man).version.build,
            (*min).major,
            (*min).minor,
            (*min).hotfix,
            (*min).build,
        );

        if !debug_ignore_fw_version {
            return -EINVAL;
        }
    }

    0
}

pub unsafe extern "C" fn avs_cldma_load_basefw(
    adev: *mut avs_dev,
    fw: *mut firmware,
) -> c_int {
    let cl: *mut hda_cldma = &raw mut code_loader;
    let mut reg: c_uint = 0;
    let mut ret: c_int;

    ret = avs_dsp_op_power(adev, AVS_MAIN_CORE_MASK, true);
    if ret < 0 {
        return ret;
    }

    ret = avs_dsp_op_reset(adev, AVS_MAIN_CORE_MASK, false);
    if ret < 0 {
        return ret;
    }

    ret = hda_cldma_reset(cl);
    if ret < 0 {
        dev_err((*adev).dev, b"cldma reset failed: %d\n\0".as_ptr() as *const c_char, ret);
        return ret;
    }
    hda_cldma_setup(cl);

    ret = avs_dsp_op_stall(adev, AVS_MAIN_CORE_MASK, false);
    if ret < 0 {
        return ret;
    }

    reinit_completion(&raw mut (*adev).fw_ready);
    avs_dsp_op_int_control(adev, true);

    /* await ROM init */
    ret = snd_hdac_adsp_readl_poll(
        adev,
        AVS_FW_REG_STATUS(adev),
        &mut reg,
        (reg & AVS_ROM_INIT_DONE) == AVS_ROM_INIT_DONE,
        AVS_ROM_INIT_POLLING_US,
        SKL_ROM_INIT_TIMEOUT_US,
    );
    if ret < 0 {
        dev_err(
            (*adev).dev,
            b"rom init failed: %d, status: 0x%08x, lec: 0x%08x\n\0".as_ptr() as *const c_char,
            ret,
            reg,
            snd_hdac_adsp_readl(adev, AVS_FW_REG_ERROR(adev)),
        );
        avs_dsp_core_disable(adev, AVS_MAIN_CORE_MASK);
        return ret;
    }

    hda_cldma_set_data(cl, (*fw).data as *mut c_void, (*fw).size);
    /* transfer firmware */
    hda_cldma_transfer(cl, 0);
    ret = snd_hdac_adsp_readl_poll(
        adev,
        AVS_FW_REG_STATUS(adev),
        &mut reg,
        (reg & AVS_ROM_STS_MASK) == SKL_ROM_BASEFW_ENTERED,
        AVS_FW_INIT_POLLING_US,
        AVS_FW_INIT_TIMEOUT_US,
    );
    hda_cldma_stop(cl);
    if ret < 0 {
        dev_err(
            (*adev).dev,
            b"transfer fw failed: %d, status: 0x%08x, lec: 0x%08x\n\0".as_ptr() as *const c_char,
            ret,
            reg,
            snd_hdac_adsp_readl(adev, AVS_FW_REG_ERROR(adev)),
        );
        avs_dsp_core_disable(adev, AVS_MAIN_CORE_MASK);
        return ret;
    }

    0
}

pub unsafe extern "C" fn avs_cldma_load_library(
    adev: *mut avs_dev,
    lib: *mut firmware,
    id: u32,
) -> c_int {
    let cl: *mut hda_cldma = &raw mut code_loader;
    let mut ret: c_int;

    hda_cldma_set_data(cl, (*lib).data as *mut c_void, (*lib).size);
    /* transfer modules manifest */
    hda_cldma_transfer(cl, msecs_to_jiffies(AVS_CLDMA_START_DELAY_MS));

    /* DMA id ignored as there is only ever one code-loader DMA */
    ret = avs_ipc_load_library(adev, 0, id);
    hda_cldma_stop(cl);

    if ret != 0 {
        ret = AVS_IPC_RET(ret);
        dev_err(
            (*adev).dev,
            b"transfer lib %d failed: %d\n\0".as_ptr() as *const c_char,
            id,
            ret,
        );
    }

    ret
}

unsafe fn avs_cldma_load_module(adev: *mut avs_dev, mentry: *mut avs_module_entry) -> c_int {
    let cl: *mut hda_cldma = &raw mut code_loader;
    let mut mod_fw: *const firmware = ptr::null();
    let mod_name: *mut c_char;
    let mut ret: c_int;

    mod_name = kasprintf(
        GFP_KERNEL,
        b"%s/%s/dsp_mod_%pUL.bin\0".as_ptr() as *const c_char,
        AVS_ROOT_DIR.as_ptr(),
        (*(*adev).spec).name,
        (*mentry).uuid.b.as_ptr(),
    );
    if mod_name.is_null() {
        return -ENOMEM;
    }

    ret = avs_request_firmware(adev, &mut mod_fw, mod_name);
    kfree(mod_name as *const c_void);
    if ret < 0 {
        return ret;
    }

    avs_hda_power_gating_enable(adev, false);
    avs_hda_clock_gating_enable(adev, false);
    avs_hda_l1sen_enable(adev, false);

    hda_cldma_set_data(cl, (*mod_fw).data as *mut c_void, (*mod_fw).size);
    hda_cldma_transfer(cl, msecs_to_jiffies(AVS_CLDMA_START_DELAY_MS));
    ret = avs_ipc_load_modules(adev, &raw const (*mentry).module_id, 1);
    hda_cldma_stop(cl);

    avs_hda_l1sen_enable(adev, true);
    avs_hda_clock_gating_enable(adev, true);
    avs_hda_power_gating_enable(adev, true);

    if ret != 0 {
        dev_err(
            (*adev).dev,
            b"load module %d failed: %d\n\0".as_ptr() as *const c_char,
            (*mentry).module_id,
            ret,
        );
        avs_release_last_firmware(adev);
        return AVS_IPC_RET(ret);
    }

    0
}

pub unsafe extern "C" fn avs_cldma_transfer_modules(
    adev: *mut avs_dev,
    load: bool,
    mods: *mut avs_module_entry,
    num_mods: u32,
) -> c_int {
    let mod_ids: *mut u16;
    let mut ret: c_int;
    let mut i: c_int;

    /* Either load to DSP or unload them to free space. */
    if load {
        i = 0;
        while i < num_mods as c_int {
            ret = avs_cldma_load_module(adev, mods.add(i as usize));
            if ret != 0 {
                return ret;
            }
            i += 1;
        }

        return 0;
    }

    mod_ids = kcalloc(num_mods as usize, size_of::<u16>(), GFP_KERNEL) as *mut u16;
    if mod_ids.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while i < num_mods as c_int {
        *mod_ids.add(i as usize) = (*mods.add(i as usize)).module_id;
        i += 1;
    }

    ret = avs_ipc_unload_modules(adev, mod_ids, num_mods);
    kfree(mod_ids as *const c_void);
    if ret != 0 {
        return AVS_IPC_RET(ret);
    }

    0
}

unsafe fn avs_hda_init_rom(adev: *mut avs_dev, dma_id: c_uint, purge: bool) -> c_int {
    let spec: *const avs_spec = (*adev).spec;
    let corex_mask: c_uint;
    let mut reg: c_uint = 0;
    let mut ret: c_int;

    corex_mask = (*spec).core_init_mask & !AVS_MAIN_CORE_MASK;

    ret = avs_dsp_op_power(adev, (*spec).core_init_mask, true);
    if ret < 0 {
        avs_dsp_core_disable(adev, (*spec).core_init_mask);
        return ret;
    }

    ret = avs_dsp_op_reset(adev, AVS_MAIN_CORE_MASK, false);
    if ret < 0 {
        avs_dsp_core_disable(adev, (*spec).core_init_mask);
        return ret;
    }

    reinit_completion(&raw mut (*adev).fw_ready);
    avs_dsp_op_int_control(adev, true);

    /* set boot config */
    ret = avs_ipc_set_boot_config(adev, dma_id, purge);
    if ret != 0 {
        ret = AVS_IPC_RET(ret);
        avs_dsp_core_disable(adev, (*spec).core_init_mask);
        return ret;
    }

    /* await ROM init */
    ret = snd_hdac_adsp_readl_poll(
        adev,
        (*(*spec).hipc).sts_offset,
        &mut reg,
        (reg & 0xF) == AVS_ROM_INIT_DONE || (reg & 0xF) == APL_ROM_FW_ENTERED,
        AVS_ROM_INIT_POLLING_US,
        APL_ROM_INIT_TIMEOUT_US,
    );
    if ret < 0 {
        dev_err(
            (*adev).dev,
            b"rom init failed: %d, status: 0x%08x, lec: 0x%08x\n\0".as_ptr() as *const c_char,
            ret,
            reg,
            snd_hdac_adsp_readl(adev, AVS_FW_REG_ERROR(adev)),
        );
        avs_dsp_core_disable(adev, (*spec).core_init_mask);
        return ret;
    }

    /* power down non-main cores */
    if corex_mask != 0 {
        ret = avs_dsp_op_power(adev, corex_mask, false);
        if ret < 0 {
            avs_dsp_core_disable(adev, (*spec).core_init_mask);
            return ret;
        }
    }

    0
}

unsafe fn avs_imr_load_basefw(adev: *mut avs_dev) -> c_int {
    let mut ret: c_int;

    /* DMA id ignored when flashing from IMR as no transfer occurs. */
    ret = avs_hda_init_rom(adev, 0, false);
    if ret < 0 {
        return ret;
    }

    ret = wait_for_completion_timeout(
        &raw mut (*adev).fw_ready,
        msecs_to_jiffies(AVS_FW_INIT_TIMEOUT_MS),
    ) as c_int;
    if ret == 0 {
        dev_err(
            (*adev).dev,
            b"firmware ready timeout, status: 0x%08x, lec: 0x%08x\n\0".as_ptr() as *const c_char,
            snd_hdac_adsp_readl(adev, AVS_FW_REG_STATUS(adev)),
            snd_hdac_adsp_readl(adev, AVS_FW_REG_ERROR(adev)),
        );
        avs_dsp_core_disable(adev, AVS_MAIN_CORE_MASK);
        return -ETIMEDOUT;
    }

    0
}

pub unsafe extern "C" fn avs_hda_load_basefw(adev: *mut avs_dev, fw: *mut firmware) -> c_int {
    let mut substream: snd_pcm_substream = zeroed();
    let mut dmab: snd_dma_buffer = zeroed();
    let estream: *mut hdac_ext_stream;
    let hstream: *mut hdac_stream;
    let bus: *mut hdac_bus = &raw mut (*adev).base.core;
    let mut sdfmt: c_uint;
    let mut reg: c_uint = 0;
    let mut ret: c_int;
    let mut i: c_int;

    /* configure hda dma */
    substream.stream = SNDRV_PCM_STREAM_PLAYBACK;
    estream = snd_hdac_ext_stream_assign(bus, &mut substream, HDAC_EXT_STREAM_TYPE_HOST);
    if estream.is_null() {
        return -ENODEV;
    }
    hstream = hdac_stream(estream);

    /* code loading performed with default format */
    sdfmt = snd_hdac_stream_format(1, 32, 48000);
    ret = snd_hdac_dsp_prepare(hstream, sdfmt, (*fw).size, &mut dmab);
    if ret < 0 {
        snd_hdac_ext_stream_release(estream, HDAC_EXT_STREAM_TYPE_HOST);
        return ret;
    }

    /* enable SPIB for hda stream */
    snd_hdac_stream_spbcap_enable(bus, true, (*hstream).index);
    ret = snd_hdac_stream_set_spib(bus, hstream, (*fw).size);
    if ret != 0 {
        snd_hdac_stream_spbcap_enable(bus, false, (*hstream).index);
        snd_hdac_stream_set_spib(bus, hstream, 0);
        snd_hdac_dsp_cleanup(hstream, &mut dmab);
        snd_hdac_ext_stream_release(estream, HDAC_EXT_STREAM_TYPE_HOST);
        return ret;
    }

    memcpy(dmab.area, (*fw).data as *const c_void, (*fw).size);

    i = 0;
    while i < APL_ROM_INIT_RETRIES {
        let dma_id: c_uint = (*hstream).stream_tag - 1;

        ret = avs_hda_init_rom(adev, dma_id, true);
        if ret == 0 {
            break;
        }
        dev_info(
            (*adev).dev,
            b"#%d rom init failed: %d\n\0".as_ptr() as *const c_char,
            i + 1,
            ret,
        );
        i += 1;
    }
    if ret < 0 {
        snd_hdac_stream_spbcap_enable(bus, false, (*hstream).index);
        snd_hdac_stream_set_spib(bus, hstream, 0);
        snd_hdac_dsp_cleanup(hstream, &mut dmab);
        snd_hdac_ext_stream_release(estream, HDAC_EXT_STREAM_TYPE_HOST);
        return ret;
    }

    /* transfer firmware */
    snd_hdac_dsp_trigger(hstream, true);
    ret = snd_hdac_adsp_readl_poll(
        adev,
        AVS_FW_REG_STATUS(adev),
        &mut reg,
        (reg & AVS_ROM_STS_MASK) == APL_ROM_FW_ENTERED,
        AVS_FW_INIT_POLLING_US,
        AVS_FW_INIT_TIMEOUT_US,
    );
    snd_hdac_dsp_trigger(hstream, false);
    if ret < 0 {
        dev_err(
            (*adev).dev,
            b"transfer fw failed: %d, status: 0x%08x, lec: 0x%08x\n\0".as_ptr() as *const c_char,
            ret,
            reg,
            snd_hdac_adsp_readl(adev, AVS_FW_REG_ERROR(adev)),
        );
        avs_dsp_core_disable(adev, AVS_MAIN_CORE_MASK);
    }

    /* disable SPIB for hda stream */
    snd_hdac_stream_spbcap_enable(bus, false, (*hstream).index);
    snd_hdac_stream_set_spib(bus, hstream, 0);

    snd_hdac_dsp_cleanup(hstream, &mut dmab);
    snd_hdac_ext_stream_release(estream, HDAC_EXT_STREAM_TYPE_HOST);

    ret
}

pub unsafe extern "C" fn avs_hda_load_library(
    adev: *mut avs_dev,
    lib: *mut firmware,
    id: u32,
) -> c_int {
    let mut substream: snd_pcm_substream = zeroed();
    let mut dmab: snd_dma_buffer = zeroed();
    let estream: *mut hdac_ext_stream;
    let stream: *mut hdac_stream;
    let bus: *mut hdac_bus = &raw mut (*adev).base.core;
    let sdfmt: c_uint;
    let mut ret: c_int;

    /* configure hda dma */
    substream.stream = SNDRV_PCM_STREAM_PLAYBACK;
    estream = snd_hdac_ext_stream_assign(bus, &mut substream, HDAC_EXT_STREAM_TYPE_HOST);
    if estream.is_null() {
        return -ENODEV;
    }
    stream = hdac_stream(estream);

    /* code loading performed with default format */
    sdfmt = snd_hdac_stream_format(1, 32, 48000);
    ret = snd_hdac_dsp_prepare(stream, sdfmt, (*lib).size, &mut dmab);
    if ret < 0 {
        snd_hdac_ext_stream_release(estream, HDAC_EXT_STREAM_TYPE_HOST);
        return ret;
    }

    /* enable SPIB for hda stream */
    snd_hdac_stream_spbcap_enable(bus, true, (*stream).index);
    snd_hdac_stream_set_spib(bus, stream, (*lib).size);

    memcpy(dmab.area, (*lib).data as *const c_void, (*lib).size);

    /* transfer firmware */
    snd_hdac_dsp_trigger(stream, true);
    ret = avs_ipc_load_library(adev, (*stream).stream_tag - 1, id);
    snd_hdac_dsp_trigger(stream, false);
    if ret != 0 {
        dev_err(
            (*adev).dev,
            b"transfer lib %d failed: %d\n\0".as_ptr() as *const c_char,
            id,
            ret,
        );
        ret = AVS_IPC_RET(ret);
    }

    /* disable SPIB for hda stream */
    snd_hdac_stream_spbcap_enable(bus, false, (*stream).index);
    snd_hdac_stream_set_spib(bus, stream, 0);

    snd_hdac_dsp_cleanup(stream, &mut dmab);
    snd_hdac_ext_stream_release(estream, HDAC_EXT_STREAM_TYPE_HOST);

    ret
}

pub unsafe extern "C" fn avs_hda_transfer_modules(
    _adev: *mut avs_dev,
    _load: bool,
    _mods: *mut avs_module_entry,
    _num_mods: u32,
) -> c_int {
    /*
     * All platforms without CLDMA are equipped with IMR,
     * and thus the module transferring is offloaded to DSP.
     */
    0
}

pub unsafe extern "C" fn avs_dsp_load_libraries(
    adev: *mut avs_dev,
    libs: *mut avs_tplg_library,
    num_libs: u32,
) -> c_int {
    let mut start: c_int;
    let mut id: c_int;
    let mut i: c_int = 0;
    let mut ret: c_int;

    /* Calculate the id to assign for the next lib. */
    id = 0;
    while id < (*adev).fw_cfg.max_libs_count as c_int {
        if *(*(*adev).lib_names.add(id as usize)).add(0) == 0 {
            break;
        }
        id += 1;
    }
    if id + num_libs as c_int >= (*adev).fw_cfg.max_libs_count as c_int {
        return -EINVAL;
    }

    start = id;
    'next_lib: while i < num_libs as c_int {
        let man: *mut avs_fw_manifest;
        let mut fw: *const firmware = ptr::null();
        let mut stripped_fw: firmware;
        let filename: *mut c_char;
        let mut j: c_int;

        filename = kasprintf(
            GFP_KERNEL,
            b"%s/%s/%s\0".as_ptr() as *const c_char,
            AVS_ROOT_DIR.as_ptr(),
            (*(*adev).spec).name,
            (*libs.add(i as usize)).name,
        );
        if filename.is_null() {
            return -ENOMEM;
        }

        /*
         * If any call after this one fails, requested firmware is not released with
         * avs_release_last_firmware() as failing to load code results in need for reload
         * of entire driver module. And then avs_release_firmwares() is in place already.
         */
        ret = avs_request_firmware(adev, &mut fw, filename);
        kfree(filename as *const c_void);
        if ret < 0 {
            return ret;
        }

        stripped_fw = *fw;
        ret = avs_fw_manifest_strip_verify(adev, &mut stripped_fw, ptr::null());
        if ret != 0 {
            dev_err(
                (*adev).dev,
                b"invalid library data: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }

        ret = avs_fw_manifest_offset(&mut stripped_fw);
        if ret < 0 {
            return ret;
        }
        man = stripped_fw.data.add(ret as usize) as *mut avs_fw_manifest;

        /* Don't load anything that's already in DSP memory. */
        j = 0;
        while j < id {
            if strncmp(
                *(*adev).lib_names.add(j as usize),
                (*man).name.as_ptr(),
                AVS_LIB_NAME_SIZE,
            ) == 0
            {
                i += 1;
                continue 'next_lib;
            }
            j += 1;
        }

        ret = avs_dsp_op_load_lib(adev, &mut stripped_fw, id);
        if ret != 0 {
            return ret;
        }

        strscpy(*(*adev).lib_names.add(id as usize), (*man).name.as_ptr(), AVS_LIB_NAME_SIZE);
        id += 1;
        i += 1;
    }

    if start == id { 1 } else { 0 }
}

unsafe fn avs_dsp_load_basefw(adev: *mut avs_dev) -> c_int {
    let min_req: *const avs_fw_version;
    let spec: *const avs_spec = (*adev).spec;
    let mut fw: *const firmware = ptr::null();
    let mut stripped_fw: firmware;
    let filename: *mut c_char;
    let mut ret: c_int;

    filename = kasprintf(
        GFP_KERNEL,
        b"%s/%s/%s\0".as_ptr() as *const c_char,
        AVS_ROOT_DIR.as_ptr(),
        (*spec).name,
        AVS_BASEFW_FILENAME.as_ptr(),
    );
    if filename.is_null() {
        return -ENOMEM;
    }

    ret = avs_request_firmware(adev, &mut fw, filename);
    kfree(filename as *const c_void);
    if ret < 0 {
        dev_err(
            (*adev).dev,
            b"request firmware failed: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    stripped_fw = *fw;
    min_req = &raw const (*(*adev).spec).min_fw_version;

    ret = avs_fw_manifest_strip_verify(adev, &mut stripped_fw, min_req);
    if ret < 0 {
        dev_err(
            (*adev).dev,
            b"invalid firmware data: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        avs_release_last_firmware(adev);
        return ret;
    }

    ret = avs_dsp_op_load_basefw(adev, &mut stripped_fw);
    if ret < 0 {
        dev_err(
            (*adev).dev,
            b"basefw load failed: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        avs_release_last_firmware(adev);
        return ret;
    }

    ret = wait_for_completion_timeout(
        &raw mut (*adev).fw_ready,
        msecs_to_jiffies(AVS_FW_INIT_TIMEOUT_MS),
    ) as c_int;
    if ret == 0 {
        dev_err(
            (*adev).dev,
            b"firmware ready timeout, status: 0x%08x, lec: 0x%08x\n\0".as_ptr() as *const c_char,
            snd_hdac_adsp_readl(adev, AVS_FW_REG_STATUS(adev)),
            snd_hdac_adsp_readl(adev, AVS_FW_REG_ERROR(adev)),
        );
        avs_dsp_core_disable(adev, AVS_MAIN_CORE_MASK);
        ret = -ETIMEDOUT;
        avs_release_last_firmware(adev);
        return ret;
    }

    0
}

unsafe fn avs_load_firmware(adev: *mut avs_dev, purge: bool) -> c_int {
    let mut acomp: *mut avs_soc_component;
    let mut ret: c_int = 0;
    let mut i: c_int;

    /* Forgo full boot if flash from IMR succeeds. */
    if !purge && avs_platattr_test(adev, IMR) {
        ret = avs_imr_load_basefw(adev);
        if ret == 0 {
            return 0;
        }

        dev_dbg(
            (*adev).dev,
            b"firmware flash from imr failed: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
    }

    /* Full boot, clear cached data except for basefw (slot 0). */
    i = 1;
    while i < (*adev).fw_cfg.max_libs_count as c_int {
        memset(*(*adev).lib_names.add(i as usize) as *mut c_void, 0, AVS_LIB_NAME_SIZE);
        i += 1;
    }

    avs_hda_power_gating_enable(adev, false);
    avs_hda_clock_gating_enable(adev, false);
    avs_hda_l1sen_enable(adev, false);

    ret = avs_dsp_load_basefw(adev);
    if ret == 0 {
        mutex_lock(&raw mut (*adev).comp_list_mutex);
        acomp = list_first_entry(&raw mut (*adev).comp_list);
        while !list_entry_is_head(acomp, &raw mut (*adev).comp_list, avs_soc_component_node()) {
            let tplg: *mut avs_tplg = (*acomp).tplg;

            ret = avs_dsp_load_libraries(adev, (*tplg).libs, (*tplg).num_libs);
            if ret < 0 {
                break;
            }
            acomp = list_next_entry(acomp, avs_soc_component_node());
        }
        mutex_unlock(&raw mut (*adev).comp_list_mutex);
    }

    avs_hda_l1sen_enable(adev, true);
    avs_hda_clock_gating_enable(adev, true);
    avs_hda_power_gating_enable(adev, true);

    if ret < 0 {
        return ret;
    }

    /* With all code loaded, refresh module information. */
    ret = avs_module_info_init(adev, true);
    if ret != 0 {
        dev_err(
            (*adev).dev,
            b"init module info failed: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    0
}

unsafe fn avs_config_basefw(adev: *mut avs_dev) -> c_int {
    let ret: c_int;

    if !(*(*(*adev).spec).dsp_ops).config_basefw.is_null() {
        ret = avs_dsp_op_config_basefw(adev);
        if ret != 0 {
            return ret;
        }
    }

    0
}

pub unsafe extern "C" fn avs_dsp_boot_firmware(adev: *mut avs_dev, purge: bool) -> c_int {
    let ret: c_int;

    ret = avs_load_firmware(adev, purge);
    if ret != 0 {
        return ret;
    }

    avs_config_basefw(adev)
}

unsafe fn avs_dsp_alloc_resources(adev: *mut avs_dev) -> c_int {
    let mut link: *mut hdac_ext_link;
    let mut ret: c_int;
    let mut i: c_int;

    ret = avs_ipc_get_hw_config(adev, &mut (*adev).hw_cfg);
    if ret != 0 {
        return AVS_IPC_RET(ret);
    }

    ret = avs_ipc_get_fw_config(adev, &mut (*adev).fw_cfg);
    if ret != 0 {
        return AVS_IPC_RET(ret);
    }

    /* If hw allows, read capabilities directly from it. */
    if avs_platattr_test(adev, ALTHDA) {
        link = snd_hdac_ext_bus_get_hlink_by_id(
            &raw mut (*adev).base.core,
            AZX_REG_ML_LEPTR_ID_INTEL_SSP,
        );
        if !link.is_null() {
            (*adev).hw_cfg.i2s_caps.ctrl_count = (*link).slcount;
        }
    }

    (*adev).core_refs = devm_kcalloc(
        (*adev).dev,
        (*adev).hw_cfg.dsp_cores as usize,
        size_of_val_raw((*adev).core_refs),
        GFP_KERNEL,
    ) as *mut _;
    (*adev).lib_names = devm_kcalloc(
        (*adev).dev,
        (*adev).fw_cfg.max_libs_count as usize,
        size_of_val_raw((*adev).lib_names),
        GFP_KERNEL,
    ) as *mut _;
    if (*adev).core_refs.is_null() || (*adev).lib_names.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while i < (*adev).fw_cfg.max_libs_count as c_int {
        *(*adev).lib_names.add(i as usize) =
            devm_kzalloc((*adev).dev, AVS_LIB_NAME_SIZE, GFP_KERNEL) as *mut c_char;
        if (*(*adev).lib_names.add(i as usize)).is_null() {
            return -ENOMEM;
        }
        i += 1;
    }

    /* basefw always occupies slot 0 */
    strscpy(*(*adev).lib_names.add(0), b"BASEFW\0".as_ptr() as *const c_char, AVS_LIB_NAME_SIZE);

    ida_init(&raw mut (*adev).ppl_ida);
    0
}

pub unsafe extern "C" fn avs_dsp_first_boot_firmware(adev: *mut avs_dev) -> c_int {
    let mut ret: c_int;

    if avs_platattr_test(adev, CLDMA) {
        ret = hda_cldma_init(
            &raw mut code_loader,
            &raw mut (*adev).base.core,
            (*adev).dsp_ba,
            AVS_CL_DEFAULT_BUFFER_SIZE,
        );
        if ret < 0 {
            dev_err(
                (*adev).dev,
                b"cldma init failed: %d\n\0".as_ptr() as *const c_char,
                ret,
            );
            return ret;
        }
    }

    ret = avs_dsp_core_disable(adev, AVS_MAIN_CORE_MASK);
    if ret < 0 {
        return ret;
    }

    ret = avs_dsp_boot_firmware(adev, true);
    if ret < 0 {
        dev_err(
            (*adev).dev,
            b"firmware boot failed: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
        return ret;
    }

    avs_dsp_alloc_resources(adev)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
