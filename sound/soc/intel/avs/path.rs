// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
//          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//

use core::ffi::{c_char, c_int, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{addr_of_mut, null_mut};

// Dependencies originally provided by:
// linux/cleanup.h, linux/acpi.h, acpi/nhlt.h, sound/pcm_params.h,
// sound/soc.h, avs.h, control.h, path.h and topology.h.
use crate::*;

/* Must be called with adev->comp_list_mutex held. */
unsafe fn avs_path_find_tplg(adev: *mut avs_dev, name: *const c_char) -> *mut avs_tplg {
    let mut acomp: *mut avs_soc_component;

    list_for_each_entry!(acomp, &mut (*adev).comp_list, node, {
        if strcmp((*(*acomp).tplg).name, name) == 0 {
            return (*acomp).tplg;
        }
    });
    null_mut()
}

unsafe fn avs_path_find_module(
    ppl: *mut avs_path_pipeline,
    template_id: u32,
) -> *mut avs_path_module {
    let mut mod_: *mut avs_path_module;

    list_for_each_entry!(mod_, &mut (*ppl).mod_list, node, {
        if (*(*mod_).template).id == template_id {
            return mod_;
        }
    });
    null_mut()
}

unsafe fn avs_path_find_pipeline(path: *mut avs_path, template_id: u32) -> *mut avs_path_pipeline {
    let mut ppl: *mut avs_path_pipeline;

    list_for_each_entry!(ppl, &mut (*path).ppl_list, node, {
        if (*(*ppl).template).id == template_id {
            return ppl;
        }
    });
    null_mut()
}

unsafe fn avs_path_find_path(
    adev: *mut avs_dev,
    name: *const c_char,
    template_id: u32,
) -> *mut avs_path {
    let mut pos: *mut avs_tplg_path_template;
    let mut template: *mut avs_tplg_path_template = null_mut();
    let tplg: *mut avs_tplg;
    let mut path: *mut avs_path;

    tplg = avs_path_find_tplg(adev, name);
    if tplg.is_null() {
        return null_mut();
    }

    list_for_each_entry!(pos, &mut (*tplg).path_tmpl_list, node, {
        if (*pos).id == template_id {
            template = pos;
            break;
        }
    });
    if template.is_null() {
        return null_mut();
    }

    guard_spinlock!(&mut (*adev).path_list_lock);
    /* Only one variant of given path template may be instantiated at a time. */
    list_for_each_entry!(path, &mut (*adev).path_list, node, {
        if (*(*path).template).owner == template {
            return path;
        }
    });

    null_mut()
}

unsafe fn avs_test_hw_params(
    params: *mut snd_pcm_hw_params,
    fmt: *mut avs_audio_format,
) -> bool {
    params_rate(params) == (*fmt).sampling_freq
        && params_channels(params) == (*fmt).num_channels
        && params_physical_width(params) == (*fmt).bit_depth
        && snd_pcm_hw_params_bits(params) == (*fmt).valid_bit_depth
}

unsafe fn avs_path_find_variant(
    adev: *mut avs_dev,
    template: *mut avs_tplg_path_template,
    fe_params: *mut snd_pcm_hw_params,
    be_params: *mut snd_pcm_hw_params,
) -> *mut avs_tplg_path {
    let mut variant: *mut avs_tplg_path;

    list_for_each_entry!(variant, &mut (*template).path_list, node, {
        dev_dbg(
            (*adev).dev,
            c"check FE rate %d chn %d vbd %d bd %d\n".as_ptr(),
            (*(*variant).fe_fmt).sampling_freq,
            (*(*variant).fe_fmt).num_channels,
            (*(*variant).fe_fmt).valid_bit_depth,
            (*(*variant).fe_fmt).bit_depth,
        );
        dev_dbg(
            (*adev).dev,
            c"check BE rate %d chn %d vbd %d bd %d\n".as_ptr(),
            (*(*variant).be_fmt).sampling_freq,
            (*(*variant).be_fmt).num_channels,
            (*(*variant).be_fmt).valid_bit_depth,
            (*(*variant).be_fmt).bit_depth,
        );

        if !(*variant).fe_fmt.is_null()
            && avs_test_hw_params(fe_params, (*variant).fe_fmt)
            && !(*variant).be_fmt.is_null()
            && avs_test_hw_params(be_params, (*variant).be_fmt)
        {
            return variant;
        }
    });

    null_mut()
}

unsafe fn avs_condpath_find_variant(
    _adev: *mut avs_dev,
    template: *mut avs_tplg_path_template,
    source: *mut avs_path,
    sink: *mut avs_path,
) -> *mut avs_tplg_path {
    let mut variant: *mut avs_tplg_path;

    list_for_each_entry!(variant, &mut (*template).path_list, node, {
        if (*variant).source_path_id == (*(*source).template).id
            && (*variant).sink_path_id == (*(*sink).template).id
        {
            return variant;
        }
    });

    null_mut()
}

unsafe fn avs_tplg_path_template_id_equal(
    id: *mut avs_tplg_path_template_id,
    id2: *mut avs_tplg_path_template_id,
) -> bool {
    (*id).id == (*id2).id && sysfs_streq((*id).tplg_name, (*id2).tplg_name) == 0
}

unsafe fn avs_condpath_find_match(
    adev: *mut avs_dev,
    template: *mut avs_tplg_path_template,
    path: *mut avs_path,
    dir: c_int,
) -> *mut avs_path {
    let id: *mut avs_tplg_path_template_id;
    let id2: *mut avs_tplg_path_template_id;

    if dir != 0 {
        id = addr_of_mut!((*template).source);
        id2 = addr_of_mut!((*template).sink);
    } else {
        id = addr_of_mut!((*template).sink);
        id2 = addr_of_mut!((*template).source);
    }

    /* Check whether this path is either source or sink of condpath template. */
    if (*id).id != (*(*(*path).template).owner).id
        || strcmp((*id).tplg_name, (*(*(*(*path).template).owner).owner).name) != 0
    {
        return null_mut();
    }

    /* Unidirectional condpaths are allowed. */
    if avs_tplg_path_template_id_equal(id, id2) {
        return path;
    }

    /* Now find the counterpart. */
    avs_path_find_path(adev, (*id2).tplg_name, (*id2).id)
}

static mut DEFAULT_BLOB_CAPS: [u32; 2] = [4, 0];
static mut default_blob: *mut acpi_nhlt_config = unsafe { DEFAULT_BLOB_CAPS.as_mut_ptr() as *mut acpi_nhlt_config };

unsafe fn avs_nhlt_config_or_default(
    adev: *mut avs_dev,
    t: *mut avs_tplg_module,
) -> *mut acpi_nhlt_config {
    let mut fmtcfg: *mut acpi_nhlt_format_config;
    let te: *mut avs_tplg_modcfg_ext;
    let mut fmt: *mut avs_audio_format;
    let link_type: c_int;
    let dev_type: c_int;
    let bus_id: c_int;
    let dir: c_int;

    te = (*t).cfg_ext;

    match (*te).copier.dma_type {
        AVS_DMA_I2S_LINK_OUTPUT => {
            link_type = ACPI_NHLT_LINKTYPE_SSP;
            dev_type = ACPI_NHLT_DEVICETYPE_CODEC;
            bus_id = (*te).copier.vindex.i2s.instance;
            dir = SNDRV_PCM_STREAM_PLAYBACK;
            fmt = (*te).copier.out_fmt;
        }
        AVS_DMA_I2S_LINK_INPUT => {
            link_type = ACPI_NHLT_LINKTYPE_SSP;
            dev_type = ACPI_NHLT_DEVICETYPE_CODEC;
            bus_id = (*te).copier.vindex.i2s.instance;
            dir = SNDRV_PCM_STREAM_CAPTURE;
            fmt = (*t).in_fmt;
        }
        AVS_DMA_DMIC_LINK_INPUT => {
            link_type = ACPI_NHLT_LINKTYPE_PDM;
            dev_type = -1; /* ignored */
            bus_id = 0;
            dir = SNDRV_PCM_STREAM_CAPTURE;
            fmt = (*t).in_fmt;
        }
        _ => return default_blob,
    }

    /* Override format selection if necessary. */
    if !(*te).copier.blob_fmt.is_null() {
        fmt = (*te).copier.blob_fmt;
    }

    fmtcfg = acpi_nhlt_find_fmtcfg(
        link_type,
        dev_type,
        dir,
        bus_id,
        (*fmt).num_channels,
        (*fmt).sampling_freq,
        (*fmt).valid_bit_depth,
        (*fmt).bit_depth,
    );
    if fmtcfg.is_null() {
        dev_warn((*adev).dev, c"Endpoint format configuration not found.\n".as_ptr());
        return ERR_PTR(-ENOENT) as *mut acpi_nhlt_config;
    }

    if (*fmtcfg).config.capabilities_size < (*default_blob).capabilities_size {
        return ERR_PTR(-ETOOSMALL) as *mut acpi_nhlt_config;
    }
    /* The firmware expects the payload to be DWORD-aligned. */
    if (*fmtcfg).config.capabilities_size % size_of::<u32>() != 0 {
        return ERR_PTR(-EINVAL) as *mut acpi_nhlt_config;
    }

    addr_of_mut!((*fmtcfg).config)
}

unsafe fn avs_append_dma_cfg(
    adev: *mut avs_dev,
    gtw: *mut avs_copier_gtw_cfg,
    t: *mut avs_tplg_module,
    dma_id: u32,
    cfg_size: *mut usize,
) -> c_int {
    let dma_type: u32 = (*(*t).cfg_ext).copier.dma_type;
    let dma: *mut avs_dma_cfg;
    let tlv: *mut avs_tlv;
    let tlv_size: usize;

    if !avs_platattr_test(adev, ALTHDA) {
        return 0;
    }

    match dma_type {
        AVS_DMA_HDA_HOST_OUTPUT | AVS_DMA_HDA_HOST_INPUT | AVS_DMA_HDA_LINK_OUTPUT
        | AVS_DMA_HDA_LINK_INPUT => return 0,
        _ => {}
    }

    tlv_size = size_of::<avs_tlv>() + size_of::<avs_dma_cfg>();
    if *cfg_size + tlv_size > AVS_MAILBOX_SIZE {
        return -E2BIG;
    }

    /* DMA config is a TLV tailing the existing payload. */
    tlv = (*gtw).config.blob.as_mut_ptr().add((*gtw).config_length as usize) as *mut avs_tlv;
    (*tlv).type_ = AVS_GTW_DMA_CONFIG_ID;
    (*tlv).length = size_of::<avs_dma_cfg>() as _;

    dma = (*tlv).value.as_mut_ptr() as *mut avs_dma_cfg;
    memset(dma as *mut c_void, 0, size_of::<avs_dma_cfg>());
    (*dma).dma_method = AVS_DMA_METHOD_HDA;
    (*dma).pre_allocated = true;
    (*dma).dma_channel_id = dma_id;
    (*dma).stream_id = dma_id + 1;

    (*gtw).config_length += (tlv_size / size_of::<u32>()) as u32;
    *cfg_size += tlv_size;

    0
}

unsafe fn avs_fill_gtw_config(
    adev: *mut avs_dev,
    gtw: *mut avs_copier_gtw_cfg,
    t: *mut avs_tplg_module,
    dma_id: u32,
    cfg_size: *mut usize,
) -> c_int {
    let blob: *mut acpi_nhlt_config;
    let gtw_size: usize;

    if !(*t).nhlt_config.is_null() {
        blob = (*(*t).nhlt_config).blob;
    } else {
        blob = avs_nhlt_config_or_default(adev, t);
    }
    if IS_ERR(blob as *const c_void) {
        return PTR_ERR(blob as *const c_void) as c_int;
    }

    gtw_size = (*blob).capabilities_size as usize;
    if *cfg_size + gtw_size > AVS_MAILBOX_SIZE {
        return -E2BIG;
    }

    (*gtw).config_length = (gtw_size / size_of::<u32>()) as _;
    memcpy(
        (*gtw).config.blob.as_mut_ptr() as *mut c_void,
        (*blob).capabilities.as_ptr() as *const c_void,
        (*blob).capabilities_size as usize,
    );
    *cfg_size += gtw_size;

    avs_append_dma_cfg(adev, gtw, t, dma_id, cfg_size)
}

unsafe fn avs_init_node_id(node_id: *mut avs_connector_node_id, te: *mut avs_tplg_modcfg_ext, dma_id: u32) {
    (*node_id).val = 0;
    (*node_id).dma_type = (*te).copier.dma_type;

    match (*node_id).dma_type {
        AVS_DMA_DMIC_LINK_INPUT | AVS_DMA_I2S_LINK_OUTPUT | AVS_DMA_I2S_LINK_INPUT => {
            /* Gateway's virtual index is statically assigned in the topology. */
            (*node_id).vindex = (*te).copier.vindex.val;
        }
        AVS_DMA_HDA_HOST_OUTPUT | AVS_DMA_HDA_HOST_INPUT => {
            /* Gateway's virtual index is dynamically assigned with DMA ID */
            (*node_id).vindex = dma_id;
        }
        AVS_DMA_HDA_LINK_OUTPUT | AVS_DMA_HDA_LINK_INPUT => {
            (*node_id).vindex = (*te).copier.vindex.val | dma_id;
        }
        _ => {
            *node_id = INVALID_NODE_ID;
        }
    }
}

unsafe fn avs_copier_create(adev: *mut avs_dev, mod_: *mut avs_path_module) -> c_int {
    let t: *mut avs_tplg_module = (*mod_).template;
    let te: *mut avs_tplg_modcfg_ext = (*t).cfg_ext;
    let cfg: *mut avs_copier_cfg = (*adev).modcfg_buf as *mut avs_copier_cfg;
    let dma_id: u32 = (*(*(*mod_).owner).owner).dma_id;
    let mut cfg_size: usize = offset_of!(avs_copier_cfg, gtw_cfg.config);
    let ret: c_int;

    ret = avs_fill_gtw_config(adev, addr_of_mut!((*cfg).gtw_cfg), t, dma_id, &mut cfg_size);
    if ret != 0 {
        return ret;
    }

    (*cfg).base.cpc = (*(*t).cfg_base).cpc;
    (*cfg).base.ibs = (*(*t).cfg_base).ibs;
    (*cfg).base.obs = (*(*t).cfg_base).obs;
    (*cfg).base.is_pages = (*(*t).cfg_base).is_pages;
    (*cfg).base.audio_fmt = *(*t).in_fmt;
    (*cfg).out_fmt = *(*te).copier.out_fmt;
    (*cfg).feature_mask = (*te).copier.feature_mask;
    avs_init_node_id(addr_of_mut!((*cfg).gtw_cfg.node_id), te, dma_id);
    (*cfg).gtw_cfg.dma_buffer_size = (*te).copier.dma_buffer_size;
    (*mod_).gtw_attrs = (*cfg).gtw_cfg.config.attrs;

    avs_dsp_init_module(
        adev,
        (*mod_).module_id,
        (*(*mod_).owner).instance_id,
        (*t).core_id,
        (*t).domain,
        cfg as *mut c_void,
        cfg_size,
        addr_of_mut!((*mod_).instance_id),
    )
}

unsafe fn avs_whm_create(adev: *mut avs_dev, mod_: *mut avs_path_module) -> c_int {
    let t: *mut avs_tplg_module = (*mod_).template;
    let te: *mut avs_tplg_modcfg_ext = (*t).cfg_ext;
    let cfg: *mut avs_whm_cfg = (*adev).modcfg_buf as *mut avs_whm_cfg;
    let dma_id: u32 = (*(*(*mod_).owner).owner).dma_id;
    let mut cfg_size: usize = offset_of!(avs_whm_cfg, gtw_cfg.config);
    let ret: c_int;

    ret = avs_fill_gtw_config(adev, addr_of_mut!((*cfg).gtw_cfg), t, dma_id, &mut cfg_size);
    if ret != 0 {
        return ret;
    }

    (*cfg).base.cpc = (*(*t).cfg_base).cpc;
    (*cfg).base.ibs = (*(*t).cfg_base).ibs;
    (*cfg).base.obs = (*(*t).cfg_base).obs;
    (*cfg).base.is_pages = (*(*t).cfg_base).is_pages;
    (*cfg).base.audio_fmt = *(*t).in_fmt;
    (*cfg).ref_fmt = *(*te).whm.ref_fmt;
    (*cfg).out_fmt = *(*te).whm.out_fmt;
    (*cfg).wake_tick_period = (*te).whm.wake_tick_period;
    avs_init_node_id(addr_of_mut!((*cfg).gtw_cfg.node_id), te, dma_id);
    (*cfg).gtw_cfg.dma_buffer_size = (*te).whm.dma_buffer_size;
    (*mod_).gtw_attrs = (*cfg).gtw_cfg.config.attrs;

    avs_dsp_init_module(
        adev,
        (*mod_).module_id,
        (*(*mod_).owner).instance_id,
        (*t).core_id,
        (*t).domain,
        cfg as *mut c_void,
        cfg_size,
        addr_of_mut!((*mod_).instance_id),
    )
}

unsafe fn avs_get_module_control(
    mod_: *mut avs_path_module,
    name: *const c_char,
) -> *mut soc_mixer_control {
    let t: *mut avs_tplg_module = (*mod_).template;
    let path_tmpl: *mut avs_tplg_path_template = (*(*(*t).owner).owner).owner;
    let w: *mut snd_soc_dapm_widget = (*path_tmpl).w;

    for i in 0..(*w).num_kcontrols {
        let mc: *mut soc_mixer_control =
            (*(*(*w).kcontrols.add(i as usize))).private_value as *mut soc_mixer_control;
        let ctl_data: *mut avs_control_data = (*mc).dobj.private as *mut avs_control_data;
        if (*ctl_data).id == (*t).ctl_id
            && !strstr((*(*(*w).kcontrols.add(i as usize))).id.name, name).is_null()
        {
            return mc;
        }
    }

    null_mut()
}

pub unsafe fn avs_peakvol_set_volume(
    adev: *mut avs_dev,
    mod_: *mut avs_path_module,
    mc: *mut soc_mixer_control,
    mut input: *mut c_long,
) -> c_int {
    let mut vols: [avs_volume_cfg; SND_SOC_TPLG_MAX_CHAN] = zeroed();
    let ctl_data: *mut avs_control_data = (*mc).dobj.private as *mut avs_control_data;
    let t: *mut avs_tplg_module = (*mod_).template;
    let ret: c_int;

    if input.is_null() {
        input = (*ctl_data).values;
    }

    if (*mc).num_channels != 0 {
        for i in 0..(*mc).num_channels {
            vols[i as usize].channel_id = i;
            vols[i as usize].target_volume = *input.add(i as usize);
            vols[i as usize].curve_type = (*(*t).cfg_ext).peakvol.curve_type;
            vols[i as usize].curve_duration = (*(*t).cfg_ext).peakvol.curve_duration;
        }

        ret = avs_ipc_peakvol_set_volumes(
            adev,
            (*mod_).module_id,
            (*mod_).instance_id,
            vols.as_mut_ptr(),
            (*mc).num_channels,
        );
        return AVS_IPC_RET(ret);
    }

    /* Target all channels if no individual selected. */
    vols[0].channel_id = AVS_ALL_CHANNELS_MASK;
    vols[0].target_volume = *input;
    vols[0].curve_type = (*(*t).cfg_ext).peakvol.curve_type;
    vols[0].curve_duration = (*(*t).cfg_ext).peakvol.curve_duration;

    ret = avs_ipc_peakvol_set_volume(adev, (*mod_).module_id, (*mod_).instance_id, &mut vols[0]);
    AVS_IPC_RET(ret)
}

pub unsafe fn avs_peakvol_set_mute(
    adev: *mut avs_dev,
    mod_: *mut avs_path_module,
    mc: *mut soc_mixer_control,
    mut input: *mut c_long,
) -> c_int {
    let mut mutes: [avs_mute_cfg; SND_SOC_TPLG_MAX_CHAN] = zeroed();
    let ctl_data: *mut avs_control_data = (*mc).dobj.private as *mut avs_control_data;
    let t: *mut avs_tplg_module = (*mod_).template;
    let ret: c_int;

    if input.is_null() {
        input = (*ctl_data).values;
    }

    if (*mc).num_channels != 0 {
        for i in 0..(*mc).num_channels {
            mutes[i as usize].channel_id = i;
            mutes[i as usize].mute = *input.add(i as usize) == 0;
            mutes[i as usize].curve_type = (*(*t).cfg_ext).peakvol.curve_type;
            mutes[i as usize].curve_duration = (*(*t).cfg_ext).peakvol.curve_duration;
        }

        ret = avs_ipc_peakvol_set_mutes(
            adev,
            (*mod_).module_id,
            (*mod_).instance_id,
            mutes.as_mut_ptr(),
            (*mc).num_channels,
        );
        return AVS_IPC_RET(ret);
    }

    /* Target all channels if no individual selected. */
    mutes[0].channel_id = AVS_ALL_CHANNELS_MASK;
    mutes[0].mute = *input == 0;
    mutes[0].curve_type = (*(*t).cfg_ext).peakvol.curve_type;
    mutes[0].curve_duration = (*(*t).cfg_ext).peakvol.curve_duration;

    ret = avs_ipc_peakvol_set_mute(adev, (*mod_).module_id, (*mod_).instance_id, &mut mutes[0]);
    AVS_IPC_RET(ret)
}

unsafe fn avs_peakvol_create(adev: *mut avs_dev, mod_: *mut avs_path_module) -> c_int {
    let t: *mut avs_tplg_module = (*mod_).template;
    let mut mc: *mut soc_mixer_control;
    let cfg: *mut avs_peakvol_cfg;
    let cfg_size: usize = struct_size!(avs_peakvol_cfg, vols, 1);
    let mut ret: c_int;

    if cfg_size > AVS_MAILBOX_SIZE {
        return -EINVAL;
    }

    cfg = (*adev).modcfg_buf as *mut avs_peakvol_cfg;
    memset(cfg as *mut c_void, 0, cfg_size);
    (*cfg).base.cpc = (*(*t).cfg_base).cpc;
    (*cfg).base.ibs = (*(*t).cfg_base).ibs;
    (*cfg).base.obs = (*(*t).cfg_base).obs;
    (*cfg).base.is_pages = (*(*t).cfg_base).is_pages;
    (*cfg).base.audio_fmt = *(*t).in_fmt;
    (*cfg).vols[0].channel_id = AVS_ALL_CHANNELS_MASK;
    (*cfg).vols[0].target_volume = S32_MAX;
    (*cfg).vols[0].curve_type = (*(*t).cfg_ext).peakvol.curve_type;
    (*cfg).vols[0].curve_duration = (*(*t).cfg_ext).peakvol.curve_duration;

    ret = avs_dsp_init_module(adev, (*mod_).module_id, (*(*mod_).owner).instance_id, (*t).core_id,
                              (*t).domain, cfg as *mut c_void, cfg_size,
                              addr_of_mut!((*mod_).instance_id));
    if ret != 0 {
        return ret;
    }

    /* Now configure both VOLUME and MUTE parameters. */
    mc = avs_get_module_control(mod_, c"Volume".as_ptr());
    if !mc.is_null() {
        ret = avs_peakvol_set_volume(adev, mod_, mc, null_mut());
        if ret != 0 {
            return ret;
        }
    }

    mc = avs_get_module_control(mod_, c"Switch".as_ptr());
    if !mc.is_null() {
        return avs_peakvol_set_mute(adev, mod_, mc, null_mut());
    }
    0
}

macro_rules! fill_base_cfg {
    ($cfg:expr, $t:expr) => {{
        (*$cfg).base.cpc = (*(*$t).cfg_base).cpc;
        (*$cfg).base.ibs = (*(*$t).cfg_base).ibs;
        (*$cfg).base.obs = (*(*$t).cfg_base).obs;
        (*$cfg).base.is_pages = (*(*$t).cfg_base).is_pages;
        (*$cfg).base.audio_fmt = *(*$t).in_fmt;
    }};
}

unsafe fn avs_updown_mix_create(adev: *mut avs_dev, mod_: *mut avs_path_module) -> c_int {
    let t = (*mod_).template;
    let mut cfg: avs_updown_mixer_cfg = zeroed();

    fill_base_cfg!(&mut cfg, t);
    cfg.out_channel_config = (*(*t).cfg_ext).updown_mix.out_channel_config;
    cfg.coefficients_select = (*(*t).cfg_ext).updown_mix.coefficients_select;
    for i in 0..AVS_COEFF_CHANNELS_MAX {
        cfg.coefficients[i] = (*(*t).cfg_ext).updown_mix.coefficients[i];
    }
    cfg.channel_map = (*(*t).cfg_ext).updown_mix.channel_map;

    avs_dsp_init_module(adev, (*mod_).module_id, (*(*mod_).owner).instance_id,
                        (*t).core_id, (*t).domain, &mut cfg as *mut _ as *mut c_void,
                        size_of::<avs_updown_mixer_cfg>(), addr_of_mut!((*mod_).instance_id))
}

unsafe fn avs_src_create(adev: *mut avs_dev, mod_: *mut avs_path_module) -> c_int {
    let t = (*mod_).template;
    let mut cfg: avs_src_cfg = zeroed();
    fill_base_cfg!(&mut cfg, t);
    cfg.out_freq = (*(*t).cfg_ext).src.out_freq;
    avs_dsp_init_module(adev, (*mod_).module_id, (*(*mod_).owner).instance_id,
                        (*t).core_id, (*t).domain, &mut cfg as *mut _ as *mut c_void,
                        size_of::<avs_src_cfg>(), addr_of_mut!((*mod_).instance_id))
}

unsafe fn avs_asrc_create(adev: *mut avs_dev, mod_: *mut avs_path_module) -> c_int {
    let t = (*mod_).template;
    let mut cfg: avs_asrc_cfg = zeroed();
    fill_base_cfg!(&mut cfg, t);
    cfg.out_freq = (*(*t).cfg_ext).asrc.out_freq;
    cfg.mode = (*(*t).cfg_ext).asrc.mode;
    cfg.disable_jitter_buffer = (*(*t).cfg_ext).asrc.disable_jitter_buffer;
    avs_dsp_init_module(adev, (*mod_).module_id, (*(*mod_).owner).instance_id,
                        (*t).core_id, (*t).domain, &mut cfg as *mut _ as *mut c_void,
                        size_of::<avs_asrc_cfg>(), addr_of_mut!((*mod_).instance_id))
}

unsafe fn avs_aec_create(adev: *mut avs_dev, mod_: *mut avs_path_module) -> c_int {
    let t = (*mod_).template;
    let mut cfg: avs_aec_cfg = zeroed();
    fill_base_cfg!(&mut cfg, t);
    cfg.ref_fmt = *(*(*t).cfg_ext).aec.ref_fmt;
    cfg.out_fmt = *(*(*t).cfg_ext).aec.out_fmt;
    cfg.cpc_lp_mode = (*(*t).cfg_ext).aec.cpc_lp_mode;
    avs_dsp_init_module(adev, (*mod_).module_id, (*(*mod_).owner).instance_id,
                        (*t).core_id, (*t).domain, &mut cfg as *mut _ as *mut c_void,
                        size_of::<avs_aec_cfg>(), addr_of_mut!((*mod_).instance_id))
}

unsafe fn avs_mux_create(adev: *mut avs_dev, mod_: *mut avs_path_module) -> c_int {
    let t = (*mod_).template;
    let mut cfg: avs_mux_cfg = zeroed();
    fill_base_cfg!(&mut cfg, t);
    cfg.ref_fmt = *(*(*t).cfg_ext).mux.ref_fmt;
    cfg.out_fmt = *(*(*t).cfg_ext).mux.out_fmt;
    avs_dsp_init_module(adev, (*mod_).module_id, (*(*mod_).owner).instance_id,
                        (*t).core_id, (*t).domain, &mut cfg as *mut _ as *mut c_void,
                        size_of::<avs_mux_cfg>(), addr_of_mut!((*mod_).instance_id))
}

unsafe fn avs_wov_create(adev: *mut avs_dev, mod_: *mut avs_path_module) -> c_int {
    let t = (*mod_).template;
    let mut cfg: avs_wov_cfg = zeroed();
    fill_base_cfg!(&mut cfg, t);
    cfg.cpc_lp_mode = (*(*t).cfg_ext).wov.cpc_lp_mode;
    avs_dsp_init_module(adev, (*mod_).module_id, (*(*mod_).owner).instance_id,
                        (*t).core_id, (*t).domain, &mut cfg as *mut _ as *mut c_void,
                        size_of::<avs_wov_cfg>(), addr_of_mut!((*mod_).instance_id))
}

unsafe fn avs_micsel_create(adev: *mut avs_dev, mod_: *mut avs_path_module) -> c_int {
    let t = (*mod_).template;
    let mut cfg: avs_micsel_cfg = zeroed();
    fill_base_cfg!(&mut cfg, t);
    cfg.out_fmt = *(*(*t).cfg_ext).micsel.out_fmt;
    avs_dsp_init_module(adev, (*mod_).module_id, (*(*mod_).owner).instance_id,
                        (*t).core_id, (*t).domain, &mut cfg as *mut _ as *mut c_void,
                        size_of::<avs_micsel_cfg>(), addr_of_mut!((*mod_).instance_id))
}

unsafe fn avs_modbase_create(adev: *mut avs_dev, mod_: *mut avs_path_module) -> c_int {
    let t = (*mod_).template;
    let mut cfg: avs_modcfg_base = zeroed();
    cfg.cpc = (*(*t).cfg_base).cpc;
    cfg.ibs = (*(*t).cfg_base).ibs;
    cfg.obs = (*(*t).cfg_base).obs;
    cfg.is_pages = (*(*t).cfg_base).is_pages;
    cfg.audio_fmt = *(*t).in_fmt;
    avs_dsp_init_module(adev, (*mod_).module_id, (*(*mod_).owner).instance_id,
                        (*t).core_id, (*t).domain, &mut cfg as *mut _ as *mut c_void,
                        size_of::<avs_modcfg_base>(), addr_of_mut!((*mod_).instance_id))
}

unsafe fn avs_modext_create(adev: *mut avs_dev, mod_: *mut avs_path_module) -> c_int {
    let t = (*mod_).template;
    let tcfg = (*t).cfg_ext;
    let cfg: *mut avs_modcfg_ext;
    let num_pins: usize = ((*tcfg).generic.num_input_pins + (*tcfg).generic.num_output_pins) as usize;
    let cfg_size: usize = struct_size!(avs_modcfg_ext, pin_fmts, num_pins);
    let ret: c_int;

    if cfg_size > AVS_MAILBOX_SIZE {
        return -EINVAL;
    }

    cfg = (*adev).modcfg_buf as *mut avs_modcfg_ext;
    memset(cfg as *mut c_void, 0, cfg_size);
    fill_base_cfg!(cfg, t);
    (*cfg).num_input_pins = (*tcfg).generic.num_input_pins;
    (*cfg).num_output_pins = (*tcfg).generic.num_output_pins;

    /* configure pin formats */
    for i in 0..num_pins {
        let tpin: *mut avs_tplg_pin_format = &mut *(*tcfg).generic.pin_fmts.as_mut_ptr().add(i);
        let pin: *mut avs_pin_format = &mut *(*cfg).pin_fmts.as_mut_ptr().add(i);

        (*pin).pin_index = (*tpin).pin_index;
        (*pin).iobs = (*tpin).iobs;
        (*pin).audio_fmt = *(*tpin).fmt;
    }

    ret = avs_dsp_init_module(adev, (*mod_).module_id, (*(*mod_).owner).instance_id,
                              (*t).core_id, (*t).domain, cfg as *mut c_void, cfg_size,
                              addr_of_mut!((*mod_).instance_id));
    ret
}

unsafe fn avs_probe_create(adev: *mut avs_dev, _mod: *mut avs_path_module) -> c_int {
    dev_err((*adev).dev, c"Probe module can't be instantiated by topology".as_ptr());
    -EINVAL
}

#[repr(C)]
pub struct avs_module_create {
    pub guid: *mut guid_t,
    pub create: unsafe fn(*mut avs_dev, *mut avs_path_module) -> c_int,
}

static mut avs_module_create: [avs_module_create; 15] = [
    avs_module_create { guid: unsafe { &raw mut AVS_MIXIN_MOD_UUID }, create: avs_modbase_create },
    avs_module_create { guid: unsafe { &raw mut AVS_MIXOUT_MOD_UUID }, create: avs_modbase_create },
    avs_module_create { guid: unsafe { &raw mut AVS_KPBUFF_MOD_UUID }, create: avs_modbase_create },
    avs_module_create { guid: unsafe { &raw mut AVS_COPIER_MOD_UUID }, create: avs_copier_create },
    avs_module_create { guid: unsafe { &raw mut AVS_PEAKVOL_MOD_UUID }, create: avs_peakvol_create },
    avs_module_create { guid: unsafe { &raw mut AVS_GAIN_MOD_UUID }, create: avs_peakvol_create },
    avs_module_create { guid: unsafe { &raw mut AVS_MICSEL_MOD_UUID }, create: avs_micsel_create },
    avs_module_create { guid: unsafe { &raw mut AVS_MUX_MOD_UUID }, create: avs_mux_create },
    avs_module_create { guid: unsafe { &raw mut AVS_UPDWMIX_MOD_UUID }, create: avs_updown_mix_create },
    avs_module_create { guid: unsafe { &raw mut AVS_SRCINTC_MOD_UUID }, create: avs_src_create },
    avs_module_create { guid: unsafe { &raw mut AVS_AEC_MOD_UUID }, create: avs_aec_create },
    avs_module_create { guid: unsafe { &raw mut AVS_ASRC_MOD_UUID }, create: avs_asrc_create },
    avs_module_create { guid: unsafe { &raw mut AVS_INTELWOV_MOD_UUID }, create: avs_wov_create },
    avs_module_create { guid: unsafe { &raw mut AVS_PROBE_MOD_UUID }, create: avs_probe_create },
    avs_module_create { guid: unsafe { &raw mut AVS_WOVHOSTM_MOD_UUID }, create: avs_whm_create },
];

unsafe fn avs_path_module_type_create(adev: *mut avs_dev, mod_: *mut avs_path_module) -> c_int {
    let type_: *const guid_t = addr_of_mut!((*(*(*mod_).template).cfg_ext).type_) as *const guid_t;

    for i in 0..avs_module_create.len() {
        if guid_equal(type_, avs_module_create[i].guid) {
            return (avs_module_create[i].create)(adev, mod_);
        }
    }

    avs_modext_create(adev, mod_)
}

unsafe fn avs_path_module_send_init_configs(adev: *mut avs_dev, mod_: *mut avs_path_module) -> c_int {
    let acomp: *mut avs_soc_component =
        to_avs_soc_component((*(*(*(*(*(*mod_).template).owner).owner).owner).owner).comp);
    let num_ids: u32 = (*(*mod_).template).num_config_ids;
    let ids: *mut u32 = (*(*mod_).template).config_ids;

    for i in 0..num_ids {
        let config: *mut avs_tplg_init_config =
            &mut *(*(*acomp).tplg).init_configs.add(*ids.add(i as usize) as usize);
        let len: usize = (*config).length;
        let data: *mut c_void = (*config).data;
        let param: u32 = (*config).param;
        let ret: c_int = avs_ipc_set_large_config(adev, (*mod_).module_id, (*mod_).instance_id, param, data, len);
        if ret != 0 {
            dev_err((*adev).dev, c"send initial module config failed: %d\n".as_ptr(), ret);
            return AVS_IPC_RET(ret);
        }
    }

    0
}

unsafe fn avs_path_module_free(_adev: *mut avs_dev, mod_: *mut avs_path_module) {
    kfree(mod_ as *mut c_void);
}

unsafe fn avs_path_module_create(
    adev: *mut avs_dev,
    owner: *mut avs_path_pipeline,
    template: *mut avs_tplg_module,
) -> *mut avs_path_module {
    let mod_: *mut avs_path_module;
    let module_id: c_int = avs_get_module_id(adev, addr_of_mut!((*(*template).cfg_ext).type_) as *const guid_t);
    let mut ret: c_int;

    if module_id < 0 {
        return ERR_PTR(module_id) as *mut avs_path_module;
    }

    mod_ = kzalloc_obj!(avs_path_module);
    if mod_.is_null() {
        return ERR_PTR(-ENOMEM) as *mut avs_path_module;
    }

    (*mod_).template = template;
    (*mod_).module_id = module_id;
    (*mod_).owner = owner;
    INIT_LIST_HEAD(addr_of_mut!((*mod_).node));

    ret = avs_path_module_type_create(adev, mod_);
    if ret != 0 {
        dev_err((*adev).dev, c"module-type create failed: %d\n".as_ptr(), ret);
        kfree(mod_ as *mut c_void);
        return ERR_PTR(ret) as *mut avs_path_module;
    }

    ret = avs_path_module_send_init_configs(adev, mod_);
    if ret != 0 {
        kfree(mod_ as *mut c_void);
        return ERR_PTR(ret) as *mut avs_path_module;
    }

    mod_
}

unsafe fn avs_path_binding_arm(adev: *mut avs_dev, binding: *mut avs_path_binding) -> c_int {
    let this_mod: *mut avs_path_module;
    let target_mod: *mut avs_path_module;
    let target_ppl: *mut avs_path_pipeline;
    let target_path: *mut avs_path;
    let t: *mut avs_tplg_binding = (*binding).template;

    this_mod = avs_path_find_module((*binding).owner, (*t).mod_id);
    if this_mod.is_null() {
        dev_err((*adev).dev, c"path mod %d not found\n".as_ptr(), (*t).mod_id);
        return -EINVAL;
    }

    /* update with target_tplg_name too */
    target_path = avs_path_find_path(adev, (*t).target_tplg_name, (*t).target_path_tmpl_id);
    if target_path.is_null() {
        dev_err((*adev).dev, c"target path %s:%d not found\n".as_ptr(), (*t).target_tplg_name, (*t).target_path_tmpl_id);
        return -EINVAL;
    }

    target_ppl = avs_path_find_pipeline(target_path, (*t).target_ppl_id);
    if target_ppl.is_null() {
        dev_err((*adev).dev, c"target ppl %d not found\n".as_ptr(), (*t).target_ppl_id);
        return -EINVAL;
    }

    target_mod = avs_path_find_module(target_ppl, (*t).target_mod_id);
    if target_mod.is_null() {
        dev_err((*adev).dev, c"target mod %d not found\n".as_ptr(), (*t).target_mod_id);
        return -EINVAL;
    }

    if (*t).is_sink {
        (*binding).sink = this_mod;
        (*binding).sink_pin = (*t).mod_pin;
        (*binding).source = target_mod;
        (*binding).source_pin = (*t).target_mod_pin;
    } else {
        (*binding).sink = target_mod;
        (*binding).sink_pin = (*t).target_mod_pin;
        (*binding).source = this_mod;
        (*binding).source_pin = (*t).mod_pin;
    }

    0
}

unsafe fn avs_path_binding_free(_adev: *mut avs_dev, binding: *mut avs_path_binding) {
    kfree(binding as *mut c_void);
}

unsafe fn avs_path_binding_create(
    _adev: *mut avs_dev,
    owner: *mut avs_path_pipeline,
    t: *mut avs_tplg_binding,
) -> *mut avs_path_binding {
    let binding: *mut avs_path_binding = kzalloc_obj!(avs_path_binding);
    if binding.is_null() {
        return ERR_PTR(-ENOMEM) as *mut avs_path_binding;
    }

    (*binding).template = t;
    (*binding).owner = owner;
    INIT_LIST_HEAD(addr_of_mut!((*binding).node));

    binding
}

unsafe fn avs_path_pipeline_arm(adev: *mut avs_dev, ppl: *mut avs_path_pipeline) -> c_int {
    let mut mod_: *mut avs_path_module;

    list_for_each_entry!(mod_, &mut (*ppl).mod_list, node, {
        let source: *mut avs_path_module;
        let sink: *mut avs_path_module;
        let ret: c_int;

        /*
         * Only one module (so it's implicitly last) or it is the last
         * one, either way we don't have next module to bind it to.
         */
        if mod_ == list_last_entry!(&mut (*ppl).mod_list, avs_path_module, node) {
            break;
        }

        /* bind current module to next module on list */
        source = mod_;
        sink = list_next_entry!(mod_, node);

        ret = avs_ipc_bind(adev, (*source).module_id, (*source).instance_id,
                           (*sink).module_id, (*sink).instance_id, 0, 0);
        if ret != 0 {
            return AVS_IPC_RET(ret);
        }
    });

    0
}

unsafe fn avs_path_pipeline_free(adev: *mut avs_dev, ppl: *mut avs_path_pipeline) {
    let mut binding: *mut avs_path_binding;
    let mut bsave: *mut avs_path_binding;
    let mut mod_: *mut avs_path_module;
    let mut save: *mut avs_path_module;

    list_for_each_entry_safe!(binding, bsave, &mut (*ppl).binding_list, node, {
        list_del(addr_of_mut!((*binding).node));
        avs_path_binding_free(adev, binding);
    });

    avs_dsp_delete_pipeline(adev, (*ppl).instance_id);

    /* Unload resources occupied by owned modules */
    list_for_each_entry_safe!(mod_, save, &mut (*ppl).mod_list, node, {
        avs_dsp_delete_module(adev, (*mod_).module_id, (*mod_).instance_id,
                              (*(*mod_).owner).instance_id, (*(*mod_).template).core_id);
        avs_path_module_free(adev, mod_);
    });

    list_del(addr_of_mut!((*ppl).node));
    kfree(ppl as *mut c_void);
}

unsafe fn avs_path_pipeline_create(
    adev: *mut avs_dev,
    owner: *mut avs_path,
    template: *mut avs_tplg_pipeline,
) -> *mut avs_path_pipeline {
    let ppl: *mut avs_path_pipeline;
    let cfg: *mut avs_tplg_pplcfg = (*template).cfg;
    let mut tmod: *mut avs_tplg_module;
    let mut ret: c_int;

    ppl = kzalloc_obj!(avs_path_pipeline);
    if ppl.is_null() {
        return ERR_PTR(-ENOMEM) as *mut avs_path_pipeline;
    }

    (*ppl).template = template;
    (*ppl).owner = owner;
    INIT_LIST_HEAD(addr_of_mut!((*ppl).binding_list));
    INIT_LIST_HEAD(addr_of_mut!((*ppl).mod_list));
    INIT_LIST_HEAD(addr_of_mut!((*ppl).node));

    ret = avs_dsp_create_pipeline(adev, (*cfg).req_size, (*cfg).priority, (*cfg).lp,
                                  (*cfg).attributes, addr_of_mut!((*ppl).instance_id));
    if ret != 0 {
        dev_err((*adev).dev, c"error creating pipeline %d\n".as_ptr(), ret);
        kfree(ppl as *mut c_void);
        return ERR_PTR(ret) as *mut avs_path_pipeline;
    }

    list_for_each_entry!(tmod, &mut (*template).mod_list, node, {
        let mod_: *mut avs_path_module = avs_path_module_create(adev, ppl, tmod);
        if IS_ERR(mod_ as *const c_void) {
            ret = PTR_ERR(mod_ as *const c_void) as c_int;
            dev_err((*adev).dev, c"error creating module %d\n".as_ptr(), ret);
            avs_path_pipeline_free(adev, ppl);
            return ERR_PTR(ret) as *mut avs_path_pipeline;
        }
        list_add_tail(addr_of_mut!((*mod_).node), addr_of_mut!((*ppl).mod_list));
    });

    for i in 0..(*template).num_bindings {
        let binding: *mut avs_path_binding =
            avs_path_binding_create(adev, ppl, *(*template).bindings.add(i as usize));
        if IS_ERR(binding as *const c_void) {
            ret = PTR_ERR(binding as *const c_void) as c_int;
            dev_err((*adev).dev, c"error creating binding %d\n".as_ptr(), ret);
            avs_path_pipeline_free(adev, ppl);
            return ERR_PTR(ret) as *mut avs_path_pipeline;
        }

        list_add_tail(addr_of_mut!((*binding).node), addr_of_mut!((*ppl).binding_list));
    }

    ppl
}

unsafe fn avs_path_init(
    adev: *mut avs_dev,
    path: *mut avs_path,
    template: *mut avs_tplg_path,
    dma_id: u32,
) -> c_int {
    let mut tppl: *mut avs_tplg_pipeline;

    (*path).owner = adev;
    (*path).template = template;
    (*path).dma_id = dma_id;
    INIT_LIST_HEAD(addr_of_mut!((*path).ppl_list));
    INIT_LIST_HEAD(addr_of_mut!((*path).node));
    INIT_LIST_HEAD(addr_of_mut!((*path).source_list));
    INIT_LIST_HEAD(addr_of_mut!((*path).sink_list));
    INIT_LIST_HEAD(addr_of_mut!((*path).source_node));
    INIT_LIST_HEAD(addr_of_mut!((*path).sink_node));

    /* create all the pipelines */
    list_for_each_entry!(tppl, &mut (*template).ppl_list, node, {
        let ppl: *mut avs_path_pipeline = avs_path_pipeline_create(adev, path, tppl);
        if IS_ERR(ppl as *const c_void) {
            return PTR_ERR(ppl as *const c_void) as c_int;
        }

        list_add_tail(addr_of_mut!((*ppl).node), addr_of_mut!((*path).ppl_list));
    });

    spin_lock(addr_of_mut!((*adev).path_list_lock));
    list_add_tail(addr_of_mut!((*path).node), addr_of_mut!((*adev).path_list));
    spin_unlock(addr_of_mut!((*adev).path_list_lock));

    0
}

unsafe fn avs_path_arm(adev: *mut avs_dev, path: *mut avs_path) -> c_int {
    let mut ppl: *mut avs_path_pipeline;
    let mut binding: *mut avs_path_binding;
    let mut ret: c_int;

    list_for_each_entry!(ppl, &mut (*path).ppl_list, node, {
        /*
         * Arm all ppl bindings before binding internal modules
         * as it costs no IPCs which isn't true for the latter.
         */
        list_for_each_entry!(binding, &mut (*ppl).binding_list, node, {
            ret = avs_path_binding_arm(adev, binding);
            if ret < 0 {
                return ret;
            }
        });

        ret = avs_path_pipeline_arm(adev, ppl);
        if ret < 0 {
            return ret;
        }
    });

    0
}

unsafe fn avs_path_free_unlocked(path: *mut avs_path) {
    let mut ppl: *mut avs_path_pipeline;
    let mut save: *mut avs_path_pipeline;

    spin_lock(addr_of_mut!((*(*path).owner).path_list_lock));
    list_del(addr_of_mut!((*path).node));
    spin_unlock(addr_of_mut!((*(*path).owner).path_list_lock));

    list_for_each_entry_safe!(ppl, save, &mut (*path).ppl_list, node, {
        avs_path_pipeline_free((*path).owner, ppl);
    });

    kfree(path as *mut c_void);
}

unsafe fn avs_path_create_unlocked(
    adev: *mut avs_dev,
    dma_id: u32,
    template: *mut avs_tplg_path,
) -> *mut avs_path {
    let path: *mut avs_path = kzalloc_obj!(avs_path);
    let mut ret: c_int;

    if path.is_null() {
        return ERR_PTR(-ENOMEM) as *mut avs_path;
    }

    ret = avs_path_init(adev, path, template, dma_id);
    if ret < 0 {
        avs_path_free_unlocked(path);
        return ERR_PTR(ret) as *mut avs_path;
    }

    ret = avs_path_arm(adev, path);
    if ret < 0 {
        avs_path_free_unlocked(path);
        return ERR_PTR(ret) as *mut avs_path;
    }

    (*path).state = AVS_PPL_STATE_INVALID;
    path
}

unsafe fn avs_condpath_free(adev: *mut avs_dev, path: *mut avs_path) {
    let mut ret: c_int;

    list_del(addr_of_mut!((*path).source_node));
    list_del(addr_of_mut!((*path).sink_node));

    ret = avs_path_reset(path);
    if ret < 0 {
        dev_err((*adev).dev, c"reset condpath failed: %d\n".as_ptr(), ret);
    }

    ret = avs_path_unbind(path);
    if ret < 0 {
        dev_err((*adev).dev, c"unbind condpath failed: %d\n".as_ptr(), ret);
    }

    avs_path_free_unlocked(path);
}

unsafe fn avs_condpath_create(
    adev: *mut avs_dev,
    template: *mut avs_tplg_path,
    source: *mut avs_path,
    sink: *mut avs_path,
) -> *mut avs_path {
    let path: *mut avs_path;
    let mut ret: c_int;

    path = avs_path_create_unlocked(adev, 0, template);
    if IS_ERR(path as *const c_void) {
        return path;
    }

    ret = avs_path_bind(path);
    if ret != 0 {
        avs_path_free_unlocked(path);
        return ERR_PTR(ret) as *mut avs_path;
    }

    ret = avs_path_reset(path);
    if ret != 0 {
        avs_path_unbind(path);
        avs_path_free_unlocked(path);
        return ERR_PTR(ret) as *mut avs_path;
    }

    (*path).source = source;
    (*path).sink = sink;
    list_add_tail(addr_of_mut!((*path).source_node), addr_of_mut!((*source).source_list));
    list_add_tail(addr_of_mut!((*path).sink_node), addr_of_mut!((*sink).sink_list));

    path
}

unsafe fn avs_condpaths_walk(adev: *mut avs_dev, path: *mut avs_path, dir: c_int) -> c_int {
    let mut acomp: *mut avs_soc_component;
    let mut source: *mut avs_path = null_mut();
    let mut sink: *mut avs_path = null_mut();
    let other: *mut *mut avs_path;

    if dir != 0 {
        source = path;
        other = &mut sink;
    } else {
        sink = path;
        other = &mut source;
    }

    list_for_each_entry!(acomp, &mut (*adev).comp_list, node, {
        for i in 0..(*(*acomp).tplg).num_condpath_tmpls {
            let template: *mut avs_tplg_path_template =
                &mut *(*(*acomp).tplg).condpath_tmpls.add(i as usize);
            let variant: *mut avs_tplg_path;
            let cpath: *mut avs_path;

            /* Do not create unidirectional condpaths twice. */
            if avs_tplg_path_template_id_equal(addr_of_mut!((*template).source), addr_of_mut!((*template).sink))
                && dir != 0
            {
                continue;
            }

            *other = avs_condpath_find_match(adev, template, path, dir);
            if (*other).is_null() {
                continue;
            }

            variant = avs_condpath_find_variant(adev, template, source, sink);
            if variant.is_null() {
                continue;
            }

            cpath = avs_condpath_create(adev, variant, source, sink);
            if IS_ERR(cpath as *const c_void) {
                return PTR_ERR(cpath as *const c_void) as c_int;
            }
        }
    });

    0
}

/* Caller responsible for holding adev->path_mutex. */
unsafe fn avs_condpaths_walk_all(adev: *mut avs_dev, path: *mut avs_path) -> c_int {
    let ret: c_int = avs_condpaths_walk(adev, path, SNDRV_PCM_STREAM_CAPTURE);
    if ret != 0 {
        return ret;
    }

    avs_condpaths_walk(adev, path, SNDRV_PCM_STREAM_PLAYBACK)
}

pub unsafe fn avs_path_free(path: *mut avs_path) {
    let mut cpath: *mut avs_path;
    let mut csave: *mut avs_path;
    let adev: *mut avs_dev = (*path).owner;

    guard_mutex!(&mut (*adev).path_mutex);

    /* Free all condpaths this path spawned. */
    list_for_each_entry_safe!(cpath, csave, &mut (*path).source_list, source_node, {
        avs_condpath_free((*path).owner, cpath);
    });
    list_for_each_entry_safe!(cpath, csave, &mut (*path).sink_list, sink_node, {
        avs_condpath_free((*path).owner, cpath);
    });

    avs_path_free_unlocked(path);
}

pub unsafe fn avs_path_create(
    adev: *mut avs_dev,
    dma_id: u32,
    template: *mut avs_tplg_path_template,
    fe_params: *mut snd_pcm_hw_params,
    be_params: *mut snd_pcm_hw_params,
) -> *mut avs_path {
    let variant: *mut avs_tplg_path;
    let mut path: *mut avs_path;
    let ret: c_int;

    variant = avs_path_find_variant(adev, template, fe_params, be_params);
    if variant.is_null() {
        dev_err((*adev).dev, c"no matching variant found\n".as_ptr());
        return ERR_PTR(-ENOENT) as *mut avs_path;
    }

    /* Serialize path and its components creation. */
    guard_mutex!(&mut (*adev).path_mutex);
    /* Satisfy needs of avs_path_find_tplg(). */
    guard_mutex!(&mut (*adev).comp_list_mutex);

    path = avs_path_create_unlocked(adev, dma_id, variant);
    if IS_ERR(path as *const c_void) {
        return path;
    }

    ret = avs_condpaths_walk_all(adev, path);
    if ret != 0 {
        avs_path_free_unlocked(path);
        path = ERR_PTR(ret) as *mut avs_path;
    }

    path
}

unsafe fn avs_path_bind_prepare(adev: *mut avs_dev, binding: *mut avs_path_binding) -> c_int {
    let src_fmt: *const avs_audio_format;
    let sink_fmt: *const avs_audio_format;
    let tsource: *mut avs_tplg_module = (*(*binding).source).template;
    let source: *mut avs_path_module = (*binding).source;
    let ret: c_int;

    /*
     * only copier modules about to be bound
     * to output pin other than 0 need preparation
     */
    if (*binding).source_pin == 0 {
        return 0;
    }
    if !guid_equal(addr_of_mut!((*(*tsource).cfg_ext).type_) as *const guid_t, &raw const AVS_COPIER_MOD_UUID) {
        return 0;
    }

    src_fmt = (*tsource).in_fmt;
    sink_fmt = (*(*(*binding).sink).template).in_fmt;

    ret = avs_ipc_copier_set_sink_format(adev, (*source).module_id, (*source).instance_id,
                                         (*binding).source_pin, src_fmt, sink_fmt);
    if ret != 0 {
        dev_err((*adev).dev, c"config copier failed: %d\n".as_ptr(), ret);
        return AVS_IPC_RET(ret);
    }

    0
}

pub unsafe fn avs_path_bind(path: *mut avs_path) -> c_int {
    let mut ppl: *mut avs_path_pipeline;
    let adev: *mut avs_dev = (*path).owner;
    let mut ret: c_int;

    list_for_each_entry!(ppl, &mut (*path).ppl_list, node, {
        let mut binding: *mut avs_path_binding;

        list_for_each_entry!(binding, &mut (*ppl).binding_list, node, {
            let source: *mut avs_path_module = (*binding).source;
            let sink: *mut avs_path_module = (*binding).sink;

            ret = avs_path_bind_prepare(adev, binding);
            if ret < 0 {
                return ret;
            }

            ret = avs_ipc_bind(adev, (*source).module_id, (*source).instance_id,
                               (*sink).module_id, (*sink).instance_id,
                               (*binding).sink_pin, (*binding).source_pin);
            if ret != 0 {
                dev_err((*adev).dev, c"bind path failed: %d\n".as_ptr(), ret);
                return AVS_IPC_RET(ret);
            }
        });
    });

    0
}

pub unsafe fn avs_path_unbind(path: *mut avs_path) -> c_int {
    let mut ppl: *mut avs_path_pipeline;
    let adev: *mut avs_dev = (*path).owner;

    list_for_each_entry!(ppl, &mut (*path).ppl_list, node, {
        let mut binding: *mut avs_path_binding;

        list_for_each_entry!(binding, &mut (*ppl).binding_list, node, {
            let source: *mut avs_path_module = (*binding).source;
            let sink: *mut avs_path_module = (*binding).sink;
            let ret: c_int = avs_ipc_unbind(adev, (*source).module_id, (*source).instance_id,
                                            (*sink).module_id, (*sink).instance_id,
                                            (*binding).sink_pin, (*binding).source_pin);
            if ret != 0 {
                dev_err((*adev).dev, c"unbind path failed: %d\n".as_ptr(), ret);
                return AVS_IPC_RET(ret);
            }
        });
    });

    0
}

pub unsafe fn avs_path_reset(path: *mut avs_path) -> c_int {
    let mut ppl: *mut avs_path_pipeline;
    let adev: *mut avs_dev = (*path).owner;

    if (*path).state == AVS_PPL_STATE_RESET {
        return 0;
    }

    list_for_each_entry!(ppl, &mut (*path).ppl_list, node, {
        let ret: c_int = avs_ipc_set_pipeline_state(adev, (*ppl).instance_id, AVS_PPL_STATE_RESET);
        if ret != 0 {
            dev_err((*adev).dev, c"reset path failed: %d\n".as_ptr(), ret);
            (*path).state = AVS_PPL_STATE_INVALID;
            return AVS_IPC_RET(ret);
        }
    });

    (*path).state = AVS_PPL_STATE_RESET;
    0
}

unsafe fn avs_condpath_pause(adev: *mut avs_dev, cpath: *mut avs_path) -> c_int {
    let mut ppl: *mut avs_path_pipeline;

    if (*cpath).state == AVS_PPL_STATE_PAUSED {
        return 0;
    }

    list_for_each_entry_reverse!(ppl, &mut (*cpath).ppl_list, node, {
        let ret: c_int = avs_ipc_set_pipeline_state(adev, (*ppl).instance_id, AVS_PPL_STATE_PAUSED);
        if ret != 0 {
            dev_err((*adev).dev, c"pause cpath failed: %d\n".as_ptr(), ret);
            (*cpath).state = AVS_PPL_STATE_INVALID;
            return AVS_IPC_RET(ret);
        }
    });

    (*cpath).state = AVS_PPL_STATE_PAUSED;
    0
}

unsafe fn avs_condpaths_pause(adev: *mut avs_dev, path: *mut avs_path) {
    let mut cpath: *mut avs_path;

    guard_mutex!(&mut (*adev).path_mutex);

    /* If either source or sink stops, so do the attached conditional paths. */
    list_for_each_entry!(cpath, &mut (*path).source_list, source_node, {
        avs_condpath_pause(adev, cpath);
    });
    list_for_each_entry!(cpath, &mut (*path).sink_list, sink_node, {
        avs_condpath_pause(adev, cpath);
    });
}

pub unsafe fn avs_path_pause(path: *mut avs_path) -> c_int {
    let mut ppl: *mut avs_path_pipeline;
    let adev: *mut avs_dev = (*path).owner;

    if (*path).state == AVS_PPL_STATE_PAUSED {
        return 0;
    }

    avs_condpaths_pause(adev, path);

    list_for_each_entry_reverse!(ppl, &mut (*path).ppl_list, node, {
        let ret: c_int = avs_ipc_set_pipeline_state(adev, (*ppl).instance_id, AVS_PPL_STATE_PAUSED);
        if ret != 0 {
            dev_err((*adev).dev, c"pause path failed: %d\n".as_ptr(), ret);
            (*path).state = AVS_PPL_STATE_INVALID;
            return AVS_IPC_RET(ret);
        }
    });

    (*path).state = AVS_PPL_STATE_PAUSED;
    0
}

unsafe fn avs_condpath_run(adev: *mut avs_dev, cpath: *mut avs_path, trigger: c_int) -> c_int {
    let mut ppl: *mut avs_path_pipeline;

    if (*cpath).state == AVS_PPL_STATE_RUNNING {
        return 0;
    }

    list_for_each_entry!(ppl, &mut (*cpath).ppl_list, node, {
        if (*(*(*ppl).template).cfg).trigger != trigger {
            continue;
        }

        let ret: c_int = avs_ipc_set_pipeline_state(adev, (*ppl).instance_id, AVS_PPL_STATE_RUNNING);
        if ret != 0 {
            dev_err((*adev).dev, c"run cpath failed: %d\n".as_ptr(), ret);
            (*cpath).state = AVS_PPL_STATE_INVALID;
            return AVS_IPC_RET(ret);
        }
    });

    (*cpath).state = AVS_PPL_STATE_RUNNING;
    0
}

unsafe fn avs_condpaths_run(adev: *mut avs_dev, path: *mut avs_path, trigger: c_int) {
    let mut cpath: *mut avs_path;

    guard_mutex!(&mut (*adev).path_mutex);

    /* Run conditional paths only if source and sink are both running. */
    list_for_each_entry!(cpath, &mut (*path).source_list, source_node, {
        if (*(*cpath).source).state == AVS_PPL_STATE_RUNNING
            && (*(*cpath).sink).state == AVS_PPL_STATE_RUNNING
        {
            avs_condpath_run(adev, cpath, trigger);
        }
    });

    list_for_each_entry!(cpath, &mut (*path).sink_list, sink_node, {
        if (*(*cpath).source).state == AVS_PPL_STATE_RUNNING
            && (*(*cpath).sink).state == AVS_PPL_STATE_RUNNING
        {
            avs_condpath_run(adev, cpath, trigger);
        }
    });
}

pub unsafe fn avs_path_run(path: *mut avs_path, trigger: c_int) -> c_int {
    let mut ppl: *mut avs_path_pipeline;
    let adev: *mut avs_dev = (*path).owner;

    if (*path).state == AVS_PPL_STATE_RUNNING && trigger == AVS_TPLG_TRIGGER_AUTO {
        return 0;
    }

    list_for_each_entry!(ppl, &mut (*path).ppl_list, node, {
        if (*(*(*ppl).template).cfg).trigger != trigger {
            continue;
        }

        let ret: c_int = avs_ipc_set_pipeline_state(adev, (*ppl).instance_id, AVS_PPL_STATE_RUNNING);
        if ret != 0 {
            dev_err((*adev).dev, c"run path failed: %d\n".as_ptr(), ret);
            (*path).state = AVS_PPL_STATE_INVALID;
            return AVS_IPC_RET(ret);
        }
    });

    (*path).state = AVS_PPL_STATE_RUNNING;

    /* Granular pipeline triggering not intended for conditional paths. */
    if trigger == AVS_TPLG_TRIGGER_AUTO {
        avs_condpaths_run(adev, path, trigger);
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
