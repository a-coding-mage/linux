// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//
// Authors: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//          Ranjani Sridharan <ranjani.sridharan@linux.intel.com>
//          Rander Wang <rander.wang@intel.com>
//          Keyon Jie <yang.jie@linux.intel.com>
//

/*
 * Hardware interface for generic Intel audio DSP HDA IP
 *
 * C include dependency intent:
 * sound/hdaudio_ext.h, sound/hda_register.h, linux/acpi.h, linux/debugfs.h,
 * linux/module.h, linux/soundwire/sdw.h, linux/soundwire/sdw_intel.h,
 * sound/intel-dsp-config.h, sound/intel-nhlt.h,
 * sound/soc-acpi-intel-ssp-common.h, sound/soc_sdw_utils.h, sound/sof.h,
 * sound/sof/xtensa.h, sound/hda-mlink.h, ../sof-audio.h, ../sof-pci-dev.h,
 * ../ops.h, ../ipc4-topology.h, ../../intel/common/sof-function-topology-lib.h,
 * hda.h, trace/events/sof_intel.h, sound/soc-acpi-intel-match.h, shim.h.
 */

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

/* IS_ENABLED(CONFIG_SND_SOC_SOF_INTEL_SOUNDWIRE) */

/*
 * The default for SoundWire clock stop quirks is to power gate the IP
 * and do a Bus Reset, this will need to be modified when the DSP
 * needs to remain in D0i3 so that the Master does not lose context
 * and enumeration is not required on clock restart
 */
static mut sdw_clock_stop_quirks: c_int = SDW_INTEL_CLK_STOP_BUS_RESET;
/* module_param(sdw_clock_stop_quirks, int, 0444); */
/* MODULE_PARM_DESC(sdw_clock_stop_quirks, "SOF SoundWire clock stop quirks"); */

unsafe fn sdw_params_stream(
    dev: *mut device,
    params_data: *mut sdw_intel_stream_params_data,
) -> c_int {
    let d = (*params_data).dai;
    let w = snd_soc_dai_get_widget(d, (*(*params_data).substream).stream);
    let mut data: snd_sof_dai_config_data = core::mem::zeroed();

    if w.is_null() {
        dev_err(
            dev,
            c"%s widget not found, check amp link num in the topology\n".as_ptr(),
            (*d).name,
        );
        return -EINVAL;
    }
    data.dai_index = ((*params_data).link_id << 8) | (*d).id;
    data.dai_data = (*params_data).alh_stream_id;
    data.dai_node_id = data.dai_data;

    hda_dai_config(w, SOF_DAI_CONFIG_FLAGS_HW_PARAMS, &mut data)
}

unsafe fn sdw_params_free(
    _dev: *mut device,
    free_data: *mut sdw_intel_stream_free_data,
) -> c_int {
    let d = (*free_data).dai;
    let w = snd_soc_dai_get_widget(d, (*(*free_data).substream).stream);
    let sdev = widget_to_sdev(w);

    if (*(*sdev).pdata).ipc_type == SOF_IPC_TYPE_4 {
        let swidget = (*w).dobj.private as *mut snd_sof_widget;
        let dai = (*swidget).private as *mut snd_sof_dai;
        let ipc4_copier = (*dai).private as *mut sof_ipc4_copier;

        (*ipc4_copier).dai_index = 0;
        let copier_data = &mut (*ipc4_copier).data;

        /* clear the node ID */
        copier_data.gtw_cfg.node_id &= !SOF_IPC4_NODE_INDEX_MASK;
    }

    0
}

static mut sdw_callback: sdw_intel_ops = sdw_intel_ops {
    params_stream: Some(sdw_params_stream),
    free_stream: Some(sdw_params_free),
    ..unsafe { core::mem::zeroed() }
};

unsafe fn sdw_ace2x_params_stream(
    _dev: *mut device,
    params_data: *mut sdw_intel_stream_params_data,
) -> c_int {
    sdw_hda_dai_hw_params(
        (*params_data).substream,
        (*params_data).hw_params,
        (*params_data).dai,
        (*params_data).link_id,
        (*params_data).alh_stream_id,
    )
}

unsafe fn sdw_ace2x_free_stream(
    _dev: *mut device,
    free_data: *mut sdw_intel_stream_free_data,
) -> c_int {
    sdw_hda_dai_hw_free(
        (*free_data).substream,
        (*free_data).dai,
        (*free_data).link_id,
    )
}

unsafe fn sdw_ace2x_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    sdw_hda_dai_trigger(substream, cmd, dai)
}

static mut sdw_ace2x_callback: sdw_intel_ops = sdw_intel_ops {
    params_stream: Some(sdw_ace2x_params_stream),
    free_stream: Some(sdw_ace2x_free_stream),
    trigger: Some(sdw_ace2x_trigger),
    ..unsafe { core::mem::zeroed() }
};

unsafe fn hda_sdw_acpi_scan(sdev: *mut snd_sof_dev) -> c_int {
    let interface_mask: u32 = hda_get_interface_mask(sdev);
    let hdev: *mut sof_intel_hda_dev;
    let handle: acpi_handle;
    let ret: c_int;

    if (interface_mask & BIT(SOF_DAI_INTEL_ALH)) == 0 {
        return -EINVAL;
    }

    handle = ACPI_HANDLE((*sdev).dev);

    /* save ACPI info for the probe step */
    hdev = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;

    ret = sdw_intel_acpi_scan(handle, &mut (*hdev).info);
    if ret < 0 {
        return -EINVAL;
    }

    0
}

unsafe fn hda_sdw_probe(sdev: *mut snd_sof_dev) -> c_int {
    let chip: *const sof_intel_dsp_desc;
    let hdev: *mut sof_intel_hda_dev;
    let mut res: sdw_intel_res = core::mem::zeroed();
    let sdw: *mut c_void;

    hdev = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;

    chip = get_chip_info((*sdev).pdata);
    if (*chip).hw_ip_version < SOF_INTEL_ACE_2_0 {
        res.mmio_base = (*sdev).bar[HDA_DSP_BAR as usize];
        res.hw_ops = &sdw_intel_cnl_hw_ops;
        res.shim_base = (*(*hdev).desc).sdw_shim_base;
        res.alh_base = (*(*hdev).desc).sdw_alh_base;
        res.ext = false;
        res.ops = &mut sdw_callback;
    } else {
        /*
         * retrieve eml_lock needed to protect shared registers
         * in the HDaudio multi-link areas
         */
        res.eml_lock = hdac_bus_eml_get_mutex(sof_to_bus(sdev), true, AZX_REG_ML_LEPTR_ID_SDW);
        if res.eml_lock.is_null() {
            return -ENODEV;
        }

        res.mmio_base = (*sdev).bar[HDA_DSP_HDA_BAR as usize];
        /*
         * the SHIM and SoundWire register offsets are link-specific
         * and will be determined when adding auxiliary devices
         */
        res.hw_ops = &sdw_intel_lnl_hw_ops;
        res.ext = true;
        res.ops = &mut sdw_ace2x_callback;

        /* ACE3+ supports microphone privacy */
        if (*chip).hw_ip_version >= SOF_INTEL_ACE_3_0 {
            res.mic_privacy = true;
        }
    }
    res.irq = (*sdev).ipc_irq;
    res.handle = (*hdev).info.handle;
    res.parent = (*sdev).dev;

    res.dev = (*sdev).dev;
    res.clock_stop_quirks = sdw_clock_stop_quirks;
    res.hbus = sof_to_bus(sdev);

    /*
     * ops and arg fields are not populated for now,
     * they will be needed when the DAI callbacks are
     * provided
     */

    /* we could filter links here if needed, e.g for quirks */
    res.count = (*hdev).info.count;
    res.link_mask = (*hdev).info.link_mask;

    sdw = sdw_intel_probe(&mut res);
    if sdw.is_null() {
        dev_err((*sdev).dev, c"error: SoundWire probe failed\n".as_ptr());
        return -EINVAL;
    }

    /* save context */
    (*hdev).sdw = sdw;

    0
}

pub unsafe fn hda_sdw_startup(sdev: *mut snd_sof_dev) -> c_int {
    let hdev: *mut sof_intel_hda_dev;
    let pdata: *mut snd_sof_pdata = (*sdev).pdata;
    let ret: c_int;

    hdev = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;

    if (*hdev).sdw.is_null() {
        return 0;
    }

    if !(*pdata).machine.is_null() && (*(*pdata).machine).mach_params.link_mask == 0 {
        return 0;
    }

    ret = hda_sdw_check_lcount(sdev);
    if ret < 0 {
        return ret;
    }

    sdw_intel_startup((*hdev).sdw)
}
/* EXPORT_SYMBOL_NS(hda_sdw_startup, "SND_SOC_SOF_INTEL_HDA_GENERIC"); */

unsafe fn hda_sdw_exit(sdev: *mut snd_sof_dev) -> c_int {
    let hdev: *mut sof_intel_hda_dev;

    hdev = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;

    if !(*hdev).sdw.is_null() {
        sdw_intel_exit((*hdev).sdw);
    }
    (*hdev).sdw = ptr::null_mut();

    hda_sdw_int_enable(sdev, false);

    0
}

pub unsafe fn hda_common_check_sdw_irq(sdev: *mut snd_sof_dev) -> bool {
    let hdev: *mut sof_intel_hda_dev;
    let mut ret = false;
    let irq_status: u32;

    hdev = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;

    if (*hdev).sdw.is_null() {
        return ret;
    }

    /* store status */
    irq_status = snd_sof_dsp_read(sdev, HDA_DSP_BAR, HDA_DSP_REG_ADSPIS2);

    /* invalid message ? */
    if irq_status == 0xffffffff {
        return ret;
    }

    /* SDW message ? */
    if (irq_status & HDA_DSP_REG_ADSPIS2_SNDW) != 0 {
        ret = true;
    }

    ret
}
/* EXPORT_SYMBOL_NS(hda_common_check_sdw_irq, "SND_SOC_SOF_INTEL_HDA_GENERIC"); */

unsafe fn hda_dsp_check_sdw_irq(sdev: *mut snd_sof_dev) -> bool {
    let interface_mask: u32 = hda_get_interface_mask(sdev);
    let chip: *const sof_intel_dsp_desc;

    if (interface_mask & BIT(SOF_DAI_INTEL_ALH)) == 0 {
        return false;
    }

    chip = get_chip_info((*sdev).pdata);
    if !chip.is_null() && (*chip).check_sdw_irq.is_some() {
        return ((*chip).check_sdw_irq.unwrap())(sdev);
    }

    false
}

unsafe fn hda_dsp_sdw_thread(irq: c_int, context: *mut c_void) -> irqreturn_t {
    sdw_intel_thread(irq, context)
}

pub unsafe fn hda_sdw_check_wakeen_irq_common(sdev: *mut snd_sof_dev) -> bool {
    let hdev: *mut sof_intel_hda_dev;

    hdev = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;
    if !(*hdev).sdw.is_null()
        && snd_sof_dsp_read(
            sdev,
            HDA_DSP_BAR,
            (*(*hdev).desc).sdw_shim_base + SDW_SHIM_WAKESTS,
        ) != 0
    {
        return true;
    }

    false
}
/* EXPORT_SYMBOL_NS(hda_sdw_check_wakeen_irq_common, "SND_SOC_SOF_INTEL_HDA_GENERIC"); */

unsafe fn hda_sdw_check_wakeen_irq(sdev: *mut snd_sof_dev) -> bool {
    let interface_mask: u32 = hda_get_interface_mask(sdev);
    let chip: *const sof_intel_dsp_desc;

    if (interface_mask & BIT(SOF_DAI_INTEL_ALH)) == 0 {
        return false;
    }

    chip = get_chip_info((*sdev).pdata);
    if !chip.is_null() && (*chip).check_sdw_wakeen_irq.is_some() {
        return ((*chip).check_sdw_wakeen_irq.unwrap())(sdev);
    }

    false
}

pub unsafe fn hda_sdw_process_wakeen_common(sdev: *mut snd_sof_dev) {
    let interface_mask: u32 = hda_get_interface_mask(sdev);
    let hdev: *mut sof_intel_hda_dev;

    if (interface_mask & BIT(SOF_DAI_INTEL_ALH)) == 0 {
        return;
    }

    hdev = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;
    if (*hdev).sdw.is_null() {
        return;
    }

    sdw_intel_process_wakeen_event((*hdev).sdw);
}
/* EXPORT_SYMBOL_NS(hda_sdw_process_wakeen_common, "SND_SOC_SOF_INTEL_HDA_GENERIC"); */

unsafe fn hda_dsp_sdw_check_mic_privacy_irq(sdev: *mut snd_sof_dev) -> bool {
    let chip: *const sof_intel_dsp_desc;

    chip = get_chip_info((*sdev).pdata);
    if !chip.is_null() && (*chip).check_mic_privacy_irq.is_some() {
        return ((*chip).check_mic_privacy_irq.unwrap())(sdev, true, AZX_REG_ML_LEPTR_ID_SDW);
    }

    false
}

unsafe fn hda_dsp_sdw_process_mic_privacy(sdev: *mut snd_sof_dev) {
    let chip: *const sof_intel_dsp_desc;

    chip = get_chip_info((*sdev).pdata);
    if !chip.is_null() && (*chip).process_mic_privacy.is_some() {
        ((*chip).process_mic_privacy.unwrap())(sdev, true, AZX_REG_ML_LEPTR_ID_SDW);
    }
}

/* Else branch for !IS_ENABLED(CONFIG_SND_SOC_SOF_INTEL_SOUNDWIRE) in C returned no-op values. */

/* pre fw run operations */
pub unsafe fn hda_dsp_pre_fw_run(sdev: *mut snd_sof_dev) -> c_int {
    let hda = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;
    let chip = (*hda).desc;
    let ret: c_int;

    /* Power down DSP if left enabled to ensure a clean boot state. */
    if hda_dsp_core_is_enabled(sdev, (*chip).host_managed_cores_mask) {
        dev_dbg((*sdev).dev, c"DSP core enabled, power down DSP first\n".as_ptr());

        ret = ((*chip).power_down_dsp.unwrap())(sdev);
        if ret < 0 {
            dev_warn(
                (*sdev).dev,
                c"%s: failed to power down already-enabled DSP\n".as_ptr(),
                c"hda_dsp_pre_fw_run".as_ptr(),
            );
        }
    }

    /* disable clock gating and power gating */
    hda_dsp_ctrl_clock_power_gating(sdev, false)
}

/* post fw run operations */
pub unsafe fn hda_dsp_post_fw_run(sdev: *mut snd_sof_dev) -> c_int {
    let ret: c_int;

    if (*sdev).first_boot {
        let hdev = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;

        ret = hda_sdw_startup(sdev);
        if ret < 0 {
            dev_err((*sdev).dev, c"error: could not startup SoundWire links\n".as_ptr());
            return ret;
        }

        /* Check if IMR boot is usable */
        if !sof_debug_check_flag(SOF_DBG_IGNORE_D3_PERSISTENT)
            && (((*sdev).fw_ready.flags & SOF_IPC_INFO_D3_PERSISTENT) != 0
                || (*(*sdev).pdata).ipc_type == SOF_IPC_TYPE_4)
        {
            (*hdev).imrboot_supported = true;
            debugfs_create_bool(
                c"skip_imr_boot".as_ptr(),
                0o644,
                (*sdev).debugfs_root,
                &mut (*hdev).skip_imr_boot,
            );
        }
    }

    hda_sdw_int_enable(sdev, true);

    /* re-enable clock gating and power gating */
    hda_dsp_ctrl_clock_power_gating(sdev, true)
}
/* EXPORT_SYMBOL_NS(hda_dsp_post_fw_run, "SND_SOC_SOF_INTEL_HDA_GENERIC"); */

/*
 * Debug
 */

/* IS_ENABLED(CONFIG_SND_SOC_SOF_DEBUG) */
static mut hda_use_msi: bool = true;
/* module_param_named(use_msi, hda_use_msi, bool, 0444); */
/* MODULE_PARM_DESC(use_msi, "SOF HDA use PCI MSI mode"); */
/* Else branch in C defines hda_use_msi as constant 1. */

static mut hda_model: *mut c_char = ptr::null_mut();
/* module_param(hda_model, charp, 0444); */
/* MODULE_PARM_DESC(hda_model, "Use the given HDA board model."); */

static mut dmic_num_override: c_int = -1;
/* module_param_named(dmic_num, dmic_num_override, int, 0444); */
/* MODULE_PARM_DESC(dmic_num, "SOF HDA DMIC number"); */

static mut mclk_id_override: c_int = -1;
/* module_param_named(mclk_id, mclk_id_override, int, 0444); */
/* MODULE_PARM_DESC(mclk_id, "SOF SSP mclk_id"); */

static mut bt_link_mask_override: c_int = -1;
/* module_param_named(bt_link_mask, bt_link_mask_override, int, 0444); */
/* MODULE_PARM_DESC(bt_link_mask, "SOF BT offload link mask"); */

unsafe fn hda_init(sdev: *mut snd_sof_dev) -> c_int {
    let hbus: *mut hda_bus;
    let bus: *mut hdac_bus;
    let pci = to_pci_dev((*sdev).dev);
    let mut ret: c_int;

    hbus = sof_to_hbus(sdev);
    bus = sof_to_bus(sdev);

    /* HDA bus init */
    sof_hda_bus_init(sdev, &mut (*pci).dev);

    if sof_hda_position_quirk == SOF_HDA_POSITION_QUIRK_USE_DPIB_REGISTERS {
        (*bus).use_posbuf = 0;
    } else {
        (*bus).use_posbuf = 1;
    }
    (*bus).bdl_pos_adj = 0;
    (*bus).sync_write = 1;

    mutex_init(&mut (*hbus).prepare_mutex);
    (*hbus).pci = pci;
    (*hbus).mixer_assigned = -1;
    (*hbus).modelname = hda_model;

    /* initialise hdac bus */
    (*bus).addr = pci_resource_start(pci, 0);
    (*bus).remap_addr = pci_ioremap_bar(pci, 0);
    if (*bus).remap_addr.is_null() {
        dev_err((*bus).dev, c"error: ioremap error\n".as_ptr());
        return -ENXIO;
    }

    /* HDA base */
    (*sdev).bar[HDA_DSP_HDA_BAR as usize] = (*bus).remap_addr;

    /* init i915 and HDMI codecs */
    ret = hda_codec_i915_init(sdev);
    if ret < 0 && ret != -ENODEV {
        dev_err_probe((*sdev).dev, ret, c"init of i915 and HDMI codec failed\n".as_ptr());
        iounmap((*sof_to_bus(sdev)).remap_addr);
        return ret;
    }

    /* get controller capabilities */
    ret = hda_dsp_ctrl_get_caps(sdev);
    if ret < 0 {
        dev_err((*sdev).dev, c"error: get caps error\n".as_ptr());
        hda_codec_i915_exit(sdev);
        iounmap((*sof_to_bus(sdev)).remap_addr);
    }

    ret
}

unsafe fn check_dmic_num(sdev: *mut snd_sof_dev) -> c_int {
    let hdev = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;
    let nhlt: *mut nhlt_acpi_table;
    let mut dmic_num: c_int = 0;

    nhlt = (*hdev).nhlt;
    if !nhlt.is_null() {
        dmic_num = intel_nhlt_get_dmic_geo((*sdev).dev, nhlt);
    }

    dev_info((*sdev).dev, c"DMICs detected in NHLT tables: %d\n".as_ptr(), dmic_num);

    /* allow for module parameter override */
    if dmic_num_override != -1 {
        dev_dbg(
            (*sdev).dev,
            c"overriding DMICs detected in NHLT tables %d by kernel param %d\n".as_ptr(),
            dmic_num,
            dmic_num_override,
        );
        dmic_num = dmic_num_override;
    }

    if dmic_num < 0 || dmic_num > 4 {
        dev_dbg((*sdev).dev, c"invalid dmic_number %d\n".as_ptr(), dmic_num);
        dmic_num = 0;
    }

    dmic_num
}

unsafe fn check_nhlt_ssp_mask(sdev: *mut snd_sof_dev, device_type: u8) -> c_int {
    let hdev = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;
    let nhlt: *mut nhlt_acpi_table;
    let mut ssp_mask: c_int = 0;

    nhlt = (*hdev).nhlt;
    if nhlt.is_null() {
        return ssp_mask;
    }

    if intel_nhlt_has_endpoint_type(nhlt, NHLT_LINK_SSP) {
        ssp_mask = intel_nhlt_ssp_endpoint_mask(nhlt, device_type);
        if ssp_mask != 0 {
            dev_info(
                (*sdev).dev,
                c"NHLT device %s(%d) detected, ssp_mask %#x\n".as_ptr(),
                if device_type == NHLT_DEVICE_BT { c"BT".as_ptr() } else { c"I2S".as_ptr() },
                device_type as c_int,
                ssp_mask,
            );
        }
    }

    ssp_mask
}

unsafe fn check_nhlt_ssp_mclk_mask(sdev: *mut snd_sof_dev, ssp_num: c_int) -> c_int {
    let hdev = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;
    let nhlt: *mut nhlt_acpi_table;

    nhlt = (*hdev).nhlt;
    if nhlt.is_null() {
        return 0;
    }

    intel_nhlt_ssp_mclk_mask(nhlt, ssp_num)
}

unsafe fn hda_init_caps(sdev: *mut snd_sof_dev) -> c_int {
    let interface_mask: u32 = hda_get_interface_mask(sdev);
    let bus = sof_to_bus(sdev);
    let pdata = (*sdev).pdata;
    let hdev = (*pdata).hw_pdata as *mut sof_intel_hda_dev;
    let link_mask: u32;
    let mut ret: c_int = 0;

    /* check if dsp is there */
    if !(*bus).ppcap.is_null() {
        dev_dbg((*sdev).dev, c"PP capability, will probe DSP later.\n".as_ptr());
    }

    /* Init HDA controller after i915 init */
    ret = hda_dsp_ctrl_init_chip(sdev, true);
    if ret < 0 {
        dev_err((*bus).dev, c"error: init chip failed with ret: %d\n".as_ptr(), ret);
        return ret;
    }

    /* Skip SoundWire if it is not supported */
    if (interface_mask & BIT(SOF_DAI_INTEL_ALH)) != 0 {
        /*
         * Skip SoundWire in nocodec mode when
         * IS_ENABLED(CONFIG_SND_SOC_SOF_NOCODEC_DEBUG_SUPPORT) &&
         * sof_debug_check_flag(SOF_DBG_FORCE_NOCODEC)
         */
        if !(IS_ENABLED_CONFIG_SND_SOC_SOF_NOCODEC_DEBUG_SUPPORT
            && sof_debug_check_flag(SOF_DBG_FORCE_NOCODEC))
        {
            /* scan SoundWire capabilities exposed by DSDT */
            ret = hda_sdw_acpi_scan(sdev);
            if ret < 0 {
                dev_dbg(
                    (*sdev).dev,
                    c"skipping SoundWire, not detected with ACPI scan\n".as_ptr(),
                );
            } else {
                link_mask = (*hdev).info.link_mask;
                if link_mask == 0 {
                    dev_dbg((*sdev).dev, c"skipping SoundWire, no links enabled\n".as_ptr());
                } else {
                    /*
                     * probe/allocate SoundWire resources.
                     * The hardware configuration takes place in hda_sdw_startup
                     * after power rails are enabled.
                     * It's entirely possible to have a mix of I2S/DMIC/SoundWire
                     * devices, so we allocate the resources in all cases.
                     */
                    ret = hda_sdw_probe(sdev);
                    if ret < 0 {
                        dev_err((*sdev).dev, c"error: SoundWire probe error\n".as_ptr());
                        return ret;
                    }
                }
            }
        }
    }

    /* create codec instances */
    hda_codec_probe_bus(sdev);

    if !HDA_IDISP_CODEC((*bus).codec_mask) {
        hda_codec_i915_display_power(sdev, false);
    }

    0
}

unsafe fn hda_dsp_interrupt_handler(irq: c_int, context: *mut c_void) -> irqreturn_t {
    let sdev = context as *mut snd_sof_dev;

    /*
     * Get global interrupt status. It includes all hardware interrupt
     * sources in the Intel HD Audio controller.
     */
    if (snd_sof_dsp_read(sdev, HDA_DSP_HDA_BAR, SOF_HDA_INTSTS) & SOF_HDA_INTSTS_GIS) != 0 {
        /* disable GIE interrupt */
        snd_sof_dsp_update_bits(
            sdev,
            HDA_DSP_HDA_BAR,
            SOF_HDA_INTCTL,
            SOF_HDA_INT_GLOBAL_EN,
            0,
        );

        return IRQ_WAKE_THREAD;
    }

    IRQ_NONE
}

unsafe fn hda_dsp_interrupt_thread(irq: c_int, context: *mut c_void) -> irqreturn_t {
    let sdev = context as *mut snd_sof_dev;
    let hdev = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;

    /* deal with streams and controller first */
    if hda_dsp_check_stream_irq(sdev) {
        trace_sof_intel_hda_irq(sdev, c"stream".as_ptr());
        hda_dsp_stream_threaded_handler(irq, sdev);
    }

    if hda_check_ipc_irq(sdev) {
        trace_sof_intel_hda_irq(sdev, c"ipc".as_ptr());
        ((*sof_ops(sdev)).irq_thread.unwrap())(irq, sdev);
    }

    if hda_dsp_check_sdw_irq(sdev) {
        trace_sof_intel_hda_irq(sdev, c"sdw".as_ptr());

        hda_dsp_sdw_thread(irq, (*hdev).sdw);

        if hda_dsp_sdw_check_mic_privacy_irq(sdev) {
            trace_sof_intel_hda_irq(sdev, c"mic privacy".as_ptr());
            hda_dsp_sdw_process_mic_privacy(sdev);
        }
    }

    if hda_sdw_check_wakeen_irq(sdev) {
        trace_sof_intel_hda_irq(sdev, c"wakeen".as_ptr());
        hda_sdw_process_wakeen(sdev);
    }

    hda_codec_check_for_state_change(sdev);

    /* enable GIE interrupt */
    snd_sof_dsp_update_bits(
        sdev,
        HDA_DSP_HDA_BAR,
        SOF_HDA_INTCTL,
        SOF_HDA_INT_GLOBAL_EN,
        SOF_HDA_INT_GLOBAL_EN,
    );

    IRQ_HANDLED
}

pub unsafe fn hda_dsp_probe_early(sdev: *mut snd_sof_dev) -> c_int {
    let pci = to_pci_dev((*sdev).dev);
    let hdev: *mut sof_intel_hda_dev;
    let chip: *const sof_intel_dsp_desc;
    let mut ret: c_int = 0;

    if !(*sdev).dspless_mode_selected {
        /*
         * detect DSP by checking class/subclass/prog-id information
         * class=04 subclass 03 prog-if 00: no DSP, legacy driver is required
         * class=04 subclass 01 prog-if 00: DSP is present
         *   (and may be required e.g. for DMIC or SSP support)
         * class=04 subclass 03 prog-if 80: either of DSP or legacy mode works
         */
        if (*pci).class == 0x040300 {
            dev_err(
                (*sdev).dev,
                c"the DSP is not enabled on this platform, aborting probe\n".as_ptr(),
            );
            return -ENODEV;
        } else if (*pci).class != 0x040100 && (*pci).class != 0x040380 {
            dev_err(
                (*sdev).dev,
                c"unknown PCI class/subclass/prog-if 0x%06x found, aborting probe\n".as_ptr(),
                (*pci).class,
            );
            return -ENODEV;
        }
        dev_info_once(
            (*sdev).dev,
            c"DSP detected with PCI class/subclass/prog-if 0x%06x\n".as_ptr(),
            (*pci).class,
        );
    }

    chip = get_chip_info((*sdev).pdata);
    if chip.is_null() {
        dev_err((*sdev).dev, c"error: no such device supported, chip id:%x\n".as_ptr(), (*pci).device);
        ret = -EIO;
        return ret;
    }

    (*sdev).num_cores = (*chip).cores_num;

    hdev = devm_kzalloc((*sdev).dev, core::mem::size_of::<sof_intel_hda_dev>(), GFP_KERNEL)
        as *mut sof_intel_hda_dev;
    if hdev.is_null() {
        return -ENOMEM;
    }
    (*(*sdev).pdata).hw_pdata = hdev as *mut c_void;
    (*hdev).desc = chip;
    ret = hda_init(sdev);

    ret
}
/* EXPORT_SYMBOL_NS(hda_dsp_probe_early, "SND_SOC_SOF_INTEL_HDA_GENERIC"); */

pub unsafe fn hda_dsp_probe(sdev: *mut snd_sof_dev) -> c_int {
    let pci = to_pci_dev((*sdev).dev);
    let hdev = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;
    let chip: *const sof_intel_dsp_desc;
    let mut ret: c_int = 0;

    (*hdev).dmic_dev = platform_device_register_data(
        (*sdev).dev,
        c"dmic-codec".as_ptr(),
        PLATFORM_DEVID_NONE,
        ptr::null(),
        0,
    );
    if IS_ERR((*hdev).dmic_dev) {
        dev_err((*sdev).dev, c"error: failed to create DMIC device\n".as_ptr());
        return PTR_ERR((*hdev).dmic_dev);
    }

    /*
     * use position update IPC if either it is forced
     * or we don't have other choice
     */
    /* IS_ENABLED(CONFIG_SND_SOC_SOF_DEBUG_FORCE_IPC_POSITION) sets no_ipc_position to 0. */
    if IS_ENABLED_CONFIG_SND_SOC_SOF_DEBUG_FORCE_IPC_POSITION {
        (*hdev).no_ipc_position = 0;
    } else {
        (*hdev).no_ipc_position = if (*sof_ops(sdev)).pcm_pointer.is_some() { 1 } else { 0 };
    }

    if (*sdev).dspless_mode_selected {
        (*hdev).no_ipc_position = 1;
    }

    if !(*sdev).dspless_mode_selected {
        /* DSP base */
        (*sdev).bar[HDA_DSP_BAR as usize] = pci_ioremap_bar(pci, HDA_DSP_BAR);
        if (*sdev).bar[HDA_DSP_BAR as usize].is_null() {
            dev_err((*sdev).dev, c"error: ioremap error\n".as_ptr());
            ret = -ENXIO;
            platform_device_unregister((*hdev).dmic_dev);
            return ret;
        }

        (*sdev).mmio_bar = HDA_DSP_BAR;
        (*sdev).mailbox_bar = HDA_DSP_BAR;
    }

    /* allow 64bit DMA address if supported by H/W */
    if dma_set_mask_and_coherent(&mut (*pci).dev, DMA_BIT_MASK(64)) != 0 {
        dev_dbg((*sdev).dev, c"DMA mask is 32 bit\n".as_ptr());
        dma_set_mask_and_coherent(&mut (*pci).dev, DMA_BIT_MASK(32));
    }
    dma_set_max_seg_size(&mut (*pci).dev, UINT_MAX);

    /* init streams */
    ret = hda_dsp_stream_init(sdev);
    if ret < 0 {
        dev_err((*sdev).dev, c"error: failed to init streams\n".as_ptr());
        hda_dsp_stream_free(sdev);
        if !(*sdev).dspless_mode_selected {
            iounmap((*sdev).bar[HDA_DSP_BAR as usize]);
        }
        platform_device_unregister((*hdev).dmic_dev);
        return ret;
    }

    /*
     * register our IRQ
     * let's try to enable msi firstly
     * if it fails, use legacy interrupt mode
     * TODO: support msi multiple vectors
     */
    if hda_use_msi && pci_alloc_irq_vectors(pci, 1, 1, PCI_IRQ_MSI) > 0 {
        dev_info((*sdev).dev, c"use msi interrupt mode\n".as_ptr());
        (*sdev).ipc_irq = pci_irq_vector(pci, 0);
        /* initialised to "false" by kzalloc() */
        (*sdev).msi_enabled = true;
    }

    if !(*sdev).msi_enabled {
        dev_info((*sdev).dev, c"use legacy interrupt mode\n".as_ptr());
        /*
         * in IO-APIC mode, hda->irq and ipc_irq are using the same
         * irq number of pci->irq
         */
        (*sdev).ipc_irq = (*pci).irq;
    }

    dev_dbg((*sdev).dev, c"using IPC IRQ %d\n".as_ptr(), (*sdev).ipc_irq);
    ret = request_threaded_irq(
        (*sdev).ipc_irq,
        Some(hda_dsp_interrupt_handler),
        Some(hda_dsp_interrupt_thread),
        IRQF_SHARED,
        c"AudioDSP".as_ptr(),
        sdev as *mut c_void,
    );
    if ret < 0 {
        dev_err((*sdev).dev, c"error: failed to register IPC IRQ %d\n".as_ptr(), (*sdev).ipc_irq);
        if (*sdev).msi_enabled {
            pci_free_irq_vectors(pci);
        }
        hda_dsp_stream_free(sdev);
        if !(*sdev).dspless_mode_selected {
            iounmap((*sdev).bar[HDA_DSP_BAR as usize]);
        }
        platform_device_unregister((*hdev).dmic_dev);
        return ret;
    }

    pci_set_master(pci);
    synchronize_irq((*pci).irq);

    /*
     * clear TCSEL to clear playback on some HD Audio
     * codecs. PCI TCSEL is defined in the Intel manuals.
     */
    snd_sof_pci_update_bits(sdev, PCI_TCSEL, 0x07, 0);

    /* init HDA capabilities */
    ret = hda_init_caps(sdev);
    if ret < 0 {
        free_irq((*sdev).ipc_irq, sdev as *mut c_void);
        if (*sdev).msi_enabled {
            pci_free_irq_vectors(pci);
        }
        hda_dsp_stream_free(sdev);
        if !(*sdev).dspless_mode_selected {
            iounmap((*sdev).bar[HDA_DSP_BAR as usize]);
        }
        platform_device_unregister((*hdev).dmic_dev);
        return ret;
    }

    if !(*sdev).dspless_mode_selected {
        /* enable ppcap interrupt */
        hda_dsp_ctrl_ppcap_enable(sdev, true);
        hda_dsp_ctrl_ppcap_int_enable(sdev, true);

        /* set default mailbox offset for FW ready message */
        (*sdev).dsp_box.offset = HDA_DSP_MBOX_UPLINK_OFFSET;

        INIT_DELAYED_WORK(&mut (*hdev).d0i3_work, Some(hda_dsp_d0i3_work));
    }

    chip = get_chip_info((*sdev).pdata);
    if !chip.is_null() && (*chip).hw_ip_version >= SOF_INTEL_ACE_2_0 {
        ret = hda_sdw_startup(sdev);
        if ret < 0 {
            dev_err((*sdev).dev, c"could not startup SoundWire links\n".as_ptr());
            if !(*sdev).dspless_mode_selected {
                hda_dsp_ctrl_ppcap_int_enable(sdev, false);
                hda_dsp_ctrl_ppcap_enable(sdev, false);
            }
            free_irq((*sdev).ipc_irq, sdev as *mut c_void);
            if (*sdev).msi_enabled {
                pci_free_irq_vectors(pci);
            }
            hda_dsp_stream_free(sdev);
            if !(*sdev).dspless_mode_selected {
                iounmap((*sdev).bar[HDA_DSP_BAR as usize]);
            }
            platform_device_unregister((*hdev).dmic_dev);
            return ret;
        }
    }

    init_waitqueue_head(&mut (*hdev).waitq);

    (*hdev).nhlt = intel_nhlt_init((*sdev).dev);

    0
}
/* EXPORT_SYMBOL_NS(hda_dsp_probe, "SND_SOC_SOF_INTEL_HDA_GENERIC"); */

pub unsafe fn hda_dsp_remove(sdev: *mut snd_sof_dev) {
    let hda = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;
    let chip = (*hda).desc;
    let pci = to_pci_dev((*sdev).dev);
    let nhlt = (*hda).nhlt;

    if !nhlt.is_null() {
        intel_nhlt_free(nhlt);
    }

    if !(*sdev).dspless_mode_selected {
        /* cancel any attempt for DSP D0I3 */
        cancel_delayed_work_sync(&mut (*hda).d0i3_work);
    }

    hda_codec_device_remove(sdev);

    hda_sdw_exit(sdev);

    if !IS_ERR_OR_NULL((*hda).dmic_dev) {
        platform_device_unregister((*hda).dmic_dev);
    }

    if !(*sdev).dspless_mode_selected {
        /* disable DSP IRQ */
        hda_dsp_ctrl_ppcap_int_enable(sdev, false);
    }

    /* disable CIE and GIE interrupts */
    snd_sof_dsp_update_bits(
        sdev,
        HDA_DSP_HDA_BAR,
        SOF_HDA_INTCTL,
        SOF_HDA_INT_CTRL_EN | SOF_HDA_INT_GLOBAL_EN,
        0,
    );

    if !(*sdev).dspless_mode_selected {
        /* Cancel the microphone privacy work if mic privacy is active */
        if (*hda).mic_privacy.active {
            cancel_work_sync(&mut (*hda).mic_privacy.work);
        }

        /* no need to check for error as the DSP will be disabled anyway */
        if !chip.is_null() && (*chip).power_down_dsp.is_some() {
            ((*chip).power_down_dsp.unwrap())(sdev);
        }

        /* disable DSP */
        hda_dsp_ctrl_ppcap_enable(sdev, false);

        /* Free the persistent DMA buffers used for base firmware download */
        if !(*hda).cl_dmab.area.is_null() {
            snd_dma_free_pages(&mut (*hda).cl_dmab);
        }
        if !(*hda).iccmax_dmab.area.is_null() {
            snd_dma_free_pages(&mut (*hda).iccmax_dmab);
        }
    }

    free_irq((*sdev).ipc_irq, sdev as *mut c_void);
    if (*sdev).msi_enabled {
        pci_free_irq_vectors(pci);
    }

    hda_dsp_stream_free(sdev);

    hda_bus_ml_free(sof_to_bus(sdev));

    if !(*sdev).dspless_mode_selected {
        iounmap((*sdev).bar[HDA_DSP_BAR as usize]);
    }
}
/* EXPORT_SYMBOL_NS(hda_dsp_remove, "SND_SOC_SOF_INTEL_HDA_GENERIC"); */

pub unsafe fn hda_dsp_remove_late(sdev: *mut snd_sof_dev) {
    iounmap((*sof_to_bus(sdev)).remap_addr);
    sof_hda_bus_exit(sdev);
    hda_codec_i915_exit(sdev);
}

pub unsafe fn hda_power_down_dsp(sdev: *mut snd_sof_dev) -> c_int {
    let hda = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;
    let chip = (*hda).desc;

    hda_dsp_core_reset_power_down(sdev, (*chip).host_managed_cores_mask)
}
/* EXPORT_SYMBOL_NS(hda_power_down_dsp, "SND_SOC_SOF_INTEL_HDA_GENERIC"); */

/* IS_ENABLED(CONFIG_SND_SOC_SOF_HDA_AUDIO_CODEC) */
unsafe fn hda_generic_machine_select(
    sdev: *mut snd_sof_dev,
    mach: *mut *mut snd_soc_acpi_mach,
) {
    let bus = sof_to_bus(sdev);
    let mach_params: *mut snd_soc_acpi_mach_params;
    let hda_mach: *mut snd_soc_acpi_mach;
    let pdata = (*sdev).pdata;
    let tplg_filename: *const c_char;
    let mut codec_num: c_int = 0;
    let mut i: c_int;

    /* codec detection */
    if (*bus).codec_mask == 0 {
        dev_info((*bus).dev, c"no hda codecs found!\n".as_ptr());
    } else {
        dev_info((*bus).dev, c"hda codecs found, mask %lx\n".as_ptr(), (*bus).codec_mask);

        i = 0;
        while i < HDA_MAX_CODECS {
            if ((*bus).codec_mask & (1 << i)) != 0 {
                codec_num += 1;
            }
            i += 1;
        }

        /*
         * If no machine driver is found, then:
         *
         * generic hda machine driver can handle:
         *  - one HDMI codec, and/or
         *  - one external HDAudio codec
         */
        if (*mach).is_null() && codec_num <= 2 {
            let mut tplg_fixup = false;

            /*
             * make a local copy of the match array since we might
             * be modifying it
             */
            hda_mach = devm_kmemdup_array(
                (*sdev).dev,
                snd_soc_acpi_intel_hda_machines.as_ptr() as *const c_void,
                2,
                core::mem::size_of::<snd_soc_acpi_mach>(),
                GFP_KERNEL,
            ) as *mut snd_soc_acpi_mach;
            if hda_mach.is_null() {
                dev_err(
                    (*bus).dev,
                    c"%s: failed to duplicate the HDA match table\n".as_ptr(),
                    c"hda_generic_machine_select".as_ptr(),
                );
                return;
            }

            dev_info((*bus).dev, c"using HDA machine driver %s now\n".as_ptr(), (*hda_mach).drv_name);

            /*
             * topology: use the info from hda_machines since tplg file name
             * is not overwritten
             */
            if (*pdata).tplg_filename.is_null() {
                tplg_fixup = true;
            }

            if tplg_fixup && codec_num == 1 && HDA_IDISP_CODEC((*bus).codec_mask) {
                tplg_filename = devm_kasprintf(
                    (*sdev).dev,
                    GFP_KERNEL,
                    c"%s-idisp".as_ptr(),
                    (*hda_mach).sof_tplg_filename,
                );
                if tplg_filename.is_null() {
                    return;
                }

                (*hda_mach).sof_tplg_filename = tplg_filename;
            }

            if codec_num == 2 || (codec_num == 1 && !HDA_IDISP_CODEC((*bus).codec_mask)) {
                /*
                 * Prevent SoundWire links from starting when an external
                 * HDaudio codec is used
                 */
                (*hda_mach).mach_params.link_mask = 0;
            } else {
                /*
                 * Allow SoundWire links to start when no external HDaudio codec
                 * was detected. This will not create a SoundWire card but
                 * will help detect if any SoundWire codec reports as ATTACHED.
                 */
                let hdev = (*(*sdev).pdata).hw_pdata as *mut sof_intel_hda_dev;

                (*hda_mach).mach_params.link_mask = (*hdev).info.link_mask;
            }

            *mach = hda_mach;
        }
    }

    /* used by hda machine driver to create dai links */
    if !(*mach).is_null() {
        mach_params = &mut (**mach).mach_params;
        (*mach_params).codec_mask = (*bus).codec_mask;
    }
}

/* IS_ENABLED(CONFIG_SND_SOC_SOF_INTEL_SOUNDWIRE) */

unsafe fn is_endpoint_present(sdw_device: *mut sdw_slave, dai_type: c_int) -> bool {
    let mut i: c_int;

    /* If SDCA is not present, assume the endpoint is present */
    if (*sdw_device).sdca_data.interface_revision == 0 {
        dev_warn(&mut (*sdw_device).dev, c"SDCA properties not found in BIOS\n".as_ptr());
        return true;
    }

    i = 0;
    while i < (*sdw_device).sdca_data.num_functions {
        if dai_type
            == asoc_sdw_get_dai_type((*sdw_device).sdca_data.function[i as usize].type_)
        {
            return true;
        }
        i += 1;
    }
    dev_dbg(&mut (*sdw_device).dev, c"Endpoint DAI type %d not found\n".as_ptr(), dai_type);
    false
}

unsafe fn find_acpi_adr_device(
    dev: *mut device,
    sdw_device: *mut sdw_slave,
    link: *mut snd_soc_acpi_link_adr,
    amp_index: *mut c_int,
) -> *mut snd_soc_acpi_adr_device {
    let mut adr_dev: *mut snd_soc_acpi_adr_device;
    let mut name_prefix: *const c_char = c"".as_ptr();
    let index: c_int = (*link).num_adr;
    let mut ep_index: c_int = 0;
    let mut i: c_int;
    let mut j: c_int;

    (*link).mask = BIT((*(*sdw_device).bus).link_id);
    /* index is 0 based, we need allocate index + 1 for the array size */
    if index == 0 {
        adr_dev = devm_kzalloc(dev, core::mem::size_of::<snd_soc_acpi_adr_device>(), GFP_KERNEL)
            as *mut snd_soc_acpi_adr_device;
    } else {
        adr_dev = devm_krealloc(
            dev,
            (*link).adr_d as *mut c_void,
            ((index + 1) as usize) * core::mem::size_of::<snd_soc_acpi_adr_device>(),
            GFP_KERNEL,
        ) as *mut snd_soc_acpi_adr_device;
    }

    if adr_dev.is_null() {
        return ptr::null_mut();
    }

    i = 0;
    while i < asoc_sdw_get_codec_info_list_count() {
        let endpoints: *mut snd_soc_acpi_endpoint;
        let mut amp_group_id: c_int = 1;

        if (*sdw_device).id.mfg_id != codec_info_list[i as usize].vendor_id {
            i += 1;
            continue;
        }

        if (*sdw_device).id.part_id != codec_info_list[i as usize].part_id {
            i += 1;
            continue;
        }

        endpoints = devm_kcalloc(
            dev,
            codec_info_list[i as usize].dai_num as usize,
            core::mem::size_of::<snd_soc_acpi_endpoint>(),
            GFP_KERNEL,
        ) as *mut snd_soc_acpi_endpoint;
        if endpoints.is_null() {
            return ptr::null_mut();
        }

        name_prefix = codec_info_list[i as usize].name_prefix;
        /*
         * This should not happen, but add a paranoid check to avoid NULL pointer
         * dereference
         */
        if name_prefix.is_null() {
            dev_err(
                dev,
                c"codec_info_list name_prefix of part id %#x-%#x is missing\n".as_ptr(),
                codec_info_list[i as usize].vendor_id,
                codec_info_list[i as usize].part_id,
            );
            return ptr::null_mut();
        }
        j = 0;
        while j < codec_info_list[i as usize].dai_num {
            /* Check if the endpoint is present by the SDCA DisCo table */
            if !is_endpoint_present(sdw_device, codec_info_list[i as usize].dais[j as usize].dai_type)
            {
                j += 1;
                continue;
            }

            (*endpoints.add(ep_index as usize)).num = j;
            if codec_info_list[i as usize].dais[j as usize].dai_type == SOC_SDW_DAI_TYPE_AMP {
                /* Assume all amp are aggregated */
                (*endpoints.add(ep_index as usize)).aggregated = 1;
                (*endpoints.add(ep_index as usize)).group_id = amp_group_id;
                (*endpoints.add(ep_index as usize)).group_position = *amp_index;
                /* Set group id = 2 for feedback capture endpoint */
                amp_group_id += 1;
            } else {
                (*endpoints.add(ep_index as usize)).aggregated = 0;
                (*endpoints.add(ep_index as usize)).group_id = 0;
                (*endpoints.add(ep_index as usize)).group_position = 0;
            }
            ep_index += 1;
            j += 1;
        }
        (*adr_dev.add(index as usize)).endpoints = endpoints;
        (*adr_dev.add(index as usize)).num_endpoints = ep_index;
        break;
    }

    if i == asoc_sdw_get_codec_info_list_count() {
        dev_err(dev, c"part id %#x is not supported\n".as_ptr(), (*sdw_device).id.part_id);
        return ptr::null_mut();
    }

    (*adr_dev.add(index as usize)).adr =
        (((*sdw_device).id.class_id as u64) & 0xFF)
            | ((((*sdw_device).id.part_id as u64) & 0xFFFF) << 8)
            | ((((*sdw_device).id.mfg_id as u64) & 0xFFFF) << 24)
            | ((((*sdw_device).id.unique_id & 0xF) as u64) << 40)
            | ((((*sdw_device).id.sdw_version & 0xF) as u64) << 44)
            | ((((*(*sdw_device).bus).link_id & 0xF) as u64) << 48);

    if !codec_info_list[i as usize].is_amp {
        /* For non-amp codecs, get name_prefix from codec_info_list[] */
        (*adr_dev.add(index as usize)).name_prefix =
            devm_kasprintf(dev, GFP_KERNEL, c"%s".as_ptr(), name_prefix);
    } else {
        /*
         * The name_prefix comes from codec_info_list which has a name_prefix per codec.
         * And we need to give a unique name_prefix for each amp and should be backwards
         * compatible to the existing acpi match tables to not break existing UCMs.
         * For the common name_prefix, we append the amp index to it. However, for the
         * "Left" name_prefix, we convert the second amp name_prefix to "Right" and
         * for the third and further amps, we set the name_prefix to "AMP<amp_index>".
         */
        if strcmp(name_prefix, c"Left".as_ptr()) == 0 {
            match *amp_index {
                1 => {
                    (*adr_dev.add(index as usize)).name_prefix =
                        devm_kasprintf(dev, GFP_KERNEL, c"%s".as_ptr(), c"Left".as_ptr());
                }
                2 => {
                    (*adr_dev.add(index as usize)).name_prefix =
                        devm_kasprintf(dev, GFP_KERNEL, c"%s".as_ptr(), c"Right".as_ptr());
                }
                _ => {
                    /* Set the name_fix to AMP<amp_index> if there are more than 2 amps */
                    (*adr_dev.add(index as usize)).name_prefix =
                        devm_kasprintf(dev, GFP_KERNEL, c"%s%d".as_ptr(), c"AMP".as_ptr(), *amp_index);
                }
            }
        } else if strcmp(name_prefix, c"AMP".as_ptr()) == 0 {
            (*adr_dev.add(index as usize)).name_prefix =
                devm_kasprintf(dev, GFP_KERNEL, c"%s%d".as_ptr(), name_prefix, *amp_index);
        } else {
            /*
             * The name_prefix will be the amp name if it is not "Left" or "AMP", set it to
             * <name_prefix>-<amp_index> format. Like rt1320-1
             */
            (*adr_dev.add(index as usize)).name_prefix =
                devm_kasprintf(dev, GFP_KERNEL, c"%s-%d".as_ptr(), name_prefix, *amp_index);
        }
        *amp_index += 1;
    }

    if (*adr_dev.add(index as usize)).name_prefix.is_null() {
        dev_err(dev, c"failed to allocate memory for name_prefix\n".as_ptr());
        return ptr::null_mut();
    }

    dev_dbg(
        dev,
        c"adr[%d] 0x%llx link id %d name_prefix \"%s\" is found\n".as_ptr(),
        index,
        (*adr_dev.add(index as usize)).adr,
        (*(*sdw_device).bus).link_id,
        (*adr_dev.add(index as usize)).name_prefix,
    );

    (*link).num_adr += 1;

    adr_dev
}

unsafe fn hda_sdw_machine_select(sdev: *mut snd_sof_dev) -> *mut snd_soc_acpi_mach {
    let pdata = (*sdev).pdata;
    let mut link: *const snd_soc_acpi_link_adr;
    let chip: *const sof_intel_dsp_desc;
    let links: *mut snd_soc_acpi_link_adr;
    let peripherals: *mut sdw_peripherals;
    let mut mach: *mut snd_soc_acpi_mach;
    let mut link_index: c_int;
    let link_num: c_int;
    let mut amp_index: c_int = 1;
    let mut link_mask: u32 = 0;
    let mut i: c_int;

    let hdev = (*pdata).hw_pdata as *mut sof_intel_hda_dev;

    if (*hdev).info.link_mask == 0 {
        dev_info((*sdev).dev, c"SoundWire links not enabled\n".as_ptr());
        return ptr::null_mut();
    }

    if (*hdev).sdw.is_null() {
        dev_dbg((*sdev).dev, c"SoundWire context not allocated\n".as_ptr());
        return ptr::null_mut();
    }

    if (*(*hdev).sdw).peripherals.is_null() || (*(*(*hdev).sdw).peripherals).num_peripherals == 0 {
        dev_warn((*sdev).dev, c"No SoundWire peripheral detected in ACPI tables\n".as_ptr());
        return ptr::null_mut();
    }

    /*
     * Select SoundWire machine driver if needed using the
     * alternate tables. This case deals with SoundWire-only
     * machines, for mixed cases with I2C/I2S the detection relies
     * on the HID list.
     */
    mach = (*(*pdata).desc).alt_machines;
    while !mach.is_null() && (*mach).link_mask != 0 {
        /*
         * On some platforms such as Up Extreme all links
         * are enabled but only one link can be used by
         * external codec. Instead of exact match of two masks,
         * first check whether link_mask of mach is subset of
         * link_mask supported by hw and then go on searching
         * link_adr
         */
        if ((!(*hdev).info.link_mask) & (*mach).link_mask) != 0 {
            mach = mach.add(1);
            continue;
        }

        /* No need to match adr if there is no links defined */
        if (*mach).links.is_null() {
            break;
        }

        link = (*mach).links;
        i = 0;
        while i < (*hdev).info.count && (*link).num_adr != 0 {
            /*
             * Try next machine if any expected Slaves
             * are not found on this link.
             */
            if !snd_soc_acpi_sdw_link_slaves_found((*sdev).dev, link, (*(*hdev).sdw).peripherals) {
                break;
            }
            i += 1;
            link = link.add(1);
        }
        /* Found if all Slaves are checked */
        if i == (*hdev).info.count || (*link).num_adr == 0 {
            if (*mach).machine_check.is_none() || ((*mach).machine_check.unwrap())((*hdev).sdw) {
                break;
            }
        }
        mach = mach.add(1);
    }
    if !mach.is_null() && (*mach).link_mask != 0 {
        (*mach).mach_params.links = (*mach).links;
        (*mach).mach_params.link_mask = (*mach).link_mask;
        (*mach).mach_params.platform = dev_name((*sdev).dev);

        return mach;
    }

    dev_info((*sdev).dev, c"No SoundWire machine driver found for the ACPI-reported configuration:\n".as_ptr());
    peripherals = (*(*hdev).sdw).peripherals;
    i = 0;
    while i < (*peripherals).num_peripherals {
        dev_info(
            (*sdev).dev,
            c"link %d mfg_id 0x%04x part_id 0x%04x version %#x\n".as_ptr(),
            (*(*(*peripherals).array[i as usize]).bus).link_id,
            (*(*peripherals).array[i as usize]).id.mfg_id,
            (*(*peripherals).array[i as usize]).id.part_id,
            (*(*peripherals).array[i as usize]).id.sdw_version,
        );
        i += 1;
    }

    chip = get_chip_info((*sdev).pdata);

    /* SDCA was not well supported in the BIOS before ACE2.0 */
    if (*chip).hw_ip_version < SOF_INTEL_ACE_2_0 {
        return ptr::null_mut();
    }

    if (*peripherals).num_peripherals == 0 {
        return ptr::null_mut();
    }

    /* Create default SDW mach */
    mach = devm_kzalloc((*sdev).dev, core::mem::size_of::<snd_soc_acpi_mach>(), GFP_KERNEL)
        as *mut snd_soc_acpi_mach;
    if mach.is_null() {
        return ptr::null_mut();
    }

    /* Get link mask and link number */
    i = 0;
    while i < (*peripherals).num_peripherals {
        link_mask |= BIT((*(*(*peripherals).array[i as usize]).bus).link_id);
        i += 1;
    }

    link_num = hweight32(link_mask);
    /* An empty adr_link is needed to terminate the adr_link loop */
    links = devm_kcalloc(
        (*sdev).dev,
        (link_num + 1) as usize,
        core::mem::size_of::<snd_soc_acpi_link_adr>(),
        GFP_KERNEL,
    ) as *mut snd_soc_acpi_link_adr;
    if links.is_null() {
        return ptr::null_mut();
    }

    /* Generate snd_soc_acpi_link_adr struct for each peripheral reported by the ACPI table */
    i = 0;
    while i < (*peripherals).num_peripherals {
        /* link_index = the number of used links below the current link */
        link_index = hweight32(link_mask & (BIT((*(*(*peripherals).array[i as usize]).bus).link_id) - 1));
        (*links.add(link_index as usize)).adr_d = find_acpi_adr_device(
            (*sdev).dev,
            (*peripherals).array[i as usize],
            links.add(link_index as usize),
            &mut amp_index,
        );
        if (*links.add(link_index as usize)).adr_d.is_null() {
            return ptr::null_mut();
        }
        i += 1;
    }

    (*mach).drv_name = c"sof_sdw".as_ptr();
    (*mach).mach_params.links = links;
    (*mach).mach_params.link_mask = link_mask;
    (*mach).mach_params.platform = dev_name((*sdev).dev);
    (*mach).get_function_tplg_files = Some(sof_sdw_get_tplg_files);
    /*
     * Set mach->sof_tplg_filename as a dummy topology to avoid tplg file checking
     * and being used.
     */
    (*mach).sof_tplg_filename =
        devm_kasprintf((*sdev).dev, GFP_KERNEL, c"sof-%s-dummy.tplg".as_ptr(), (*chip).platform);

    dev_info((*sdev).dev, c"Use SoundWire default machine driver with function topologies\n".as_ptr());
    mach
}

pub unsafe fn hda_set_mach_params(mach: *mut snd_soc_acpi_mach, sdev: *mut snd_sof_dev) {
    let pdata = (*sdev).pdata;
    let desc = (*pdata).desc;
    let mach_params: *mut snd_soc_acpi_mach_params;

    mach_params = &mut (*mach).mach_params;
    (*mach_params).platform = dev_name((*sdev).dev);
    if IS_ENABLED_CONFIG_SND_SOC_SOF_NOCODEC_DEBUG_SUPPORT
        && sof_debug_check_flag(SOF_DBG_FORCE_NOCODEC)
    {
        (*mach_params).num_dai_drivers = SOF_SKL_NUM_DAIS_NOCODEC;
    } else {
        (*mach_params).num_dai_drivers = (*(*desc).ops).num_drv;
    }
    (*mach_params).dai_drivers = (*(*desc).ops).drv;
}

unsafe fn check_tplg_quirk_mask(mach: *mut snd_soc_acpi_mach) -> c_int {
    let dmic_ssp_quirk: u32;
    let codec_amp_name_quirk: u32;

    /*
     * In current implementation dmic and ssp quirks are designed for es8336
     * machine driver and could not be mixed with codec name and amp name
     * quirks.
     */
    dmic_ssp_quirk = (*mach).tplg_quirk_mask
        & (SND_SOC_ACPI_TPLG_INTEL_DMIC_NUMBER | SND_SOC_ACPI_TPLG_INTEL_SSP_NUMBER);
    codec_amp_name_quirk = (*mach).tplg_quirk_mask
        & (SND_SOC_ACPI_TPLG_INTEL_AMP_NAME | SND_SOC_ACPI_TPLG_INTEL_CODEC_NAME);

    if dmic_ssp_quirk != 0 && codec_amp_name_quirk != 0 {
        return -EINVAL;
    }

    0
}

unsafe fn remove_file_ext(dev: *mut device, tplg_filename: *const c_char) -> *mut c_char {
    let filename: *mut c_char;
    let mut tmp: *mut c_char;

    filename = devm_kstrdup(dev, tplg_filename, GFP_KERNEL);
    if filename.is_null() {
        return ptr::null_mut();
    }

    /* remove file extension if exist */
    tmp = filename;
    strsep(&mut tmp, c".".as_ptr())
}

pub unsafe fn hda_machine_select(sdev: *mut snd_sof_dev) -> *mut snd_soc_acpi_mach {
    let interface_mask: u32 = hda_get_interface_mask(sdev);
    let sof_pdata = (*sdev).pdata;
    let desc = (*sof_pdata).desc;
    let bus = sof_to_bus(sdev);
    let mut mach: *mut snd_soc_acpi_mach = ptr::null_mut();
    let mut codec_type: snd_soc_acpi_intel_codec;
    let mut amp_type: snd_soc_acpi_intel_codec;
    let mut tplg_filename: *const c_char;
    let tplg_suffix: *const c_char;
    let amp_name_valid: bool;
    let mut i2s_mach_found = false;
    let mut sdw_mach_found = false;

    /* Try I2S or DMIC if it is supported */
    if (interface_mask & (BIT(SOF_DAI_INTEL_SSP) | BIT(SOF_DAI_INTEL_DMIC))) != 0 {
        mach = snd_soc_acpi_find_machine((*desc).machines);
        if !mach.is_null() {
            i2s_mach_found = true;
        }
    }

    /*
     * If I2S fails and no external HDaudio codec is detected,
     * try SoundWire if it is supported
     */
    if mach.is_null()
        && !HDA_EXT_CODEC((*bus).codec_mask)
        && (interface_mask & BIT(SOF_DAI_INTEL_ALH)) != 0
    {
        mach = hda_sdw_machine_select(sdev);
        if !mach.is_null() {
            sdw_mach_found = true;
        }
    }

    /*
     * Choose HDA generic machine driver if mach is NULL.
     * Otherwise, set certain mach params.
     */
    hda_generic_machine_select(sdev, &mut mach);
    if mach.is_null() {
        dev_warn((*sdev).dev, c"warning: No matching ASoC machine driver found\n".as_ptr());
        return ptr::null_mut();
    }

    /* report BT offload link mask to machine driver */
    (*mach).mach_params.bt_link_mask = check_nhlt_ssp_mask(sdev, NHLT_DEVICE_BT);

    dev_info(
        (*sdev).dev,
        c"BT link detected in NHLT tables: %#x\n".as_ptr(),
        (*mach).mach_params.bt_link_mask,
    );

    /* allow for module parameter override */
    if bt_link_mask_override != -1 {
        dev_dbg(
            (*sdev).dev,
            c"overriding BT link detected in NHLT tables %#x by kernel param %#x\n".as_ptr(),
            (*mach).mach_params.bt_link_mask,
            bt_link_mask_override,
        );
        (*mach).mach_params.bt_link_mask = bt_link_mask_override;
    }

    if hweight_long((*mach).mach_params.bt_link_mask) > 1 {
        dev_warn(
            (*sdev).dev,
            c"invalid BT link mask %#x found, reset the mask\n".as_ptr(),
            (*mach).mach_params.bt_link_mask,
        );
        (*mach).mach_params.bt_link_mask = 0;
    }

    /*
     * Fixup tplg file name by appending dmic num, ssp num, codec/amplifier
     * name string if quirk flag is set.
     */
    if !mach.is_null() {
        let chip = get_chip_info((*sdev).pdata);
        let mut tplg_fixup = false;
        let mut dmic_fixup = false;

        /*
         * If tplg file name is overridden, use it instead of
         * the one set in mach table
         */
        if (*sof_pdata).tplg_filename.is_null() {
            /* remove file extension if it exists */
            tplg_filename = remove_file_ext((*sdev).dev, (*mach).sof_tplg_filename);
            if tplg_filename.is_null() {
                return ptr::null_mut();
            }

            (*sof_pdata).tplg_filename = tplg_filename;
            tplg_fixup = true;
        }

        /*
         * Checking quirk mask integrity; some quirk flags could not be
         * set concurrently.
         */
        if tplg_fixup && check_tplg_quirk_mask(mach) != 0 {
            dev_err((*sdev).dev, c"Invalid tplg quirk mask 0x%x\n".as_ptr(), (*mach).tplg_quirk_mask);
            return ptr::null_mut();
        }

        /* report to machine driver if any DMICs are found */
        (*mach).mach_params.dmic_num = check_dmic_num(sdev);

        if sdw_mach_found || ((*mach).tplg_quirk_mask & SND_SOC_ACPI_TPLG_INTEL_DMIC_NUMBER) != 0
        {
            dmic_fixup = true;
        }

        if tplg_fixup && dmic_fixup && (*mach).mach_params.dmic_num != 0 {
            tplg_filename = devm_kasprintf(
                (*sdev).dev,
                GFP_KERNEL,
                c"%s%s%d%s".as_ptr(),
                (*sof_pdata).tplg_filename,
                if i2s_mach_found { c"-dmic".as_ptr() } else { c"-".as_ptr() },
                (*mach).mach_params.dmic_num,
                c"ch".as_ptr(),
            );
            if tplg_filename.is_null() {
                return ptr::null_mut();
            }

            (*sof_pdata).tplg_filename = tplg_filename;
        }

        if tplg_fixup
            && (*mach).mach_params.bt_link_mask != 0
            && (*chip).hw_ip_version >= SOF_INTEL_ACE_4_0
        {
            let bt_port = fls((*mach).mach_params.bt_link_mask) - 1;

            tplg_filename = devm_kasprintf(
                (*sdev).dev,
                GFP_KERNEL,
                c"%s-ssp%d-bt".as_ptr(),
                (*sof_pdata).tplg_filename,
                bt_port,
            );
            if tplg_filename.is_null() {
                return ptr::null_mut();
            }

            (*sof_pdata).tplg_filename = tplg_filename;
        }

        if (*mach).link_mask != 0 {
            (*mach).mach_params.links = (*mach).links;
            (*mach).mach_params.link_mask = (*mach).link_mask;
        }

        /* report SSP link mask to machine driver */
        (*mach).mach_params.i2s_link_mask = check_nhlt_ssp_mask(sdev, NHLT_DEVICE_I2S);

        if tplg_fixup
            && ((*mach).tplg_quirk_mask & SND_SOC_ACPI_TPLG_INTEL_SSP_NUMBER) != 0
            && (*mach).mach_params.i2s_link_mask != 0
        {
            let ssp_num: c_int;

            if hweight_long((*mach).mach_params.i2s_link_mask) > 1
                && ((*mach).tplg_quirk_mask & SND_SOC_ACPI_TPLG_INTEL_SSP_MSB) == 0
            {
                dev_warn((*sdev).dev, c"More than one SSP exposed by NHLT, choosing MSB\n".as_ptr());
            }

            /* fls returns 1-based results, SSPs indices are 0-based */
            ssp_num = fls((*mach).mach_params.i2s_link_mask) - 1;

            if ssp_num >= (*chip).ssp_count {
                dev_err(
                    (*sdev).dev,
                    c"Invalid SSP %d, max on this platform is %d\n".as_ptr(),
                    ssp_num,
                    (*chip).ssp_count,
                );
                return ptr::null_mut();
            }

            tplg_filename = devm_kasprintf(
                (*sdev).dev,
                GFP_KERNEL,
                c"%s%s%d".as_ptr(),
                (*sof_pdata).tplg_filename,
                c"-ssp".as_ptr(),
                ssp_num,
            );
            if tplg_filename.is_null() {
                return ptr::null_mut();
            }

            (*sof_pdata).tplg_filename = tplg_filename;

            if (*sof_pdata).ipc_type == SOF_IPC_TYPE_3 {
                let mclk_mask = check_nhlt_ssp_mclk_mask(sdev, ssp_num);

                if mclk_mask < 0 {
                    dev_err((*sdev).dev, c"Invalid MCLK configuration for SSP%d\n".as_ptr(), ssp_num);
                    return ptr::null_mut();
                }

                if mclk_mask != 0 {
                    (*sdev).mclk_id_override = true;
                    (*sdev).mclk_id_quirk = if (mclk_mask & BIT(0)) != 0 { 0 } else { 1 };
                    dev_info(
                        (*sdev).dev,
                        c"SSP%d to use MCLK id %d (mask: %#x)\n".as_ptr(),
                        ssp_num,
                        (*sdev).mclk_id_quirk,
                        mclk_mask,
                    );
                } else {
                    dev_dbg((*sdev).dev, c"MCLK mask is empty for SSP%d in NHLT\n".as_ptr(), ssp_num);
                }
            }
        }

        amp_type = snd_soc_acpi_intel_detect_amp_type((*sdev).dev);
        codec_type = snd_soc_acpi_intel_detect_codec_type((*sdev).dev);
        amp_name_valid = amp_type != CODEC_NONE && amp_type != codec_type;

        if tplg_fixup
            && amp_name_valid
            && ((*mach).tplg_quirk_mask & SND_SOC_ACPI_TPLG_INTEL_AMP_NAME) != 0
        {
            tplg_suffix = snd_soc_acpi_intel_get_amp_tplg_suffix(amp_type);
            if tplg_suffix.is_null() {
                dev_err((*sdev).dev, c"no tplg suffix found, amp %d\n".as_ptr(), amp_type);
                return ptr::null_mut();
            }

            tplg_filename = devm_kasprintf(
                (*sdev).dev,
                GFP_KERNEL,
                c"%s-%s".as_ptr(),
                (*sof_pdata).tplg_filename,
                tplg_suffix,
            );
            if tplg_filename.is_null() {
                return ptr::null_mut();
            }

            (*sof_pdata).tplg_filename = tplg_filename;
        }

        if tplg_fixup
            && ((*mach).tplg_quirk_mask & SND_SOC_ACPI_TPLG_INTEL_CODEC_NAME) != 0
            && codec_type != CODEC_NONE
        {
            tplg_suffix = snd_soc_acpi_intel_get_codec_tplg_suffix(codec_type);
            if tplg_suffix.is_null() {
                dev_err((*sdev).dev, c"no tplg suffix found, codec %d\n".as_ptr(), codec_type);
                return ptr::null_mut();
            }

            tplg_filename = devm_kasprintf(
                (*sdev).dev,
                GFP_KERNEL,
                c"%s-%s".as_ptr(),
                (*sof_pdata).tplg_filename,
                tplg_suffix,
            );
            if tplg_filename.is_null() {
                return ptr::null_mut();
            }

            (*sof_pdata).tplg_filename = tplg_filename;
        }

        if tplg_fixup {
            tplg_filename = devm_kasprintf(
                (*sdev).dev,
                GFP_KERNEL,
                c"%s%s".as_ptr(),
                (*sof_pdata).tplg_filename,
                c".tplg".as_ptr(),
            );
            if tplg_filename.is_null() {
                return ptr::null_mut();
            }

            (*sof_pdata).tplg_filename = tplg_filename;
        }

        /* check if mclk_id should be modified from topology defaults */
        if mclk_id_override >= 0 {
            dev_info(
                (*sdev).dev,
                c"Overriding topology with MCLK %d from kernel_parameter\n".as_ptr(),
                mclk_id_override,
            );
            (*sdev).mclk_id_override = true;
            (*sdev).mclk_id_quirk = mclk_id_override;
        }
    }

    mach
}

pub unsafe fn hda_pci_intel_probe(
    pci: *mut pci_dev,
    pci_id: *const pci_device_id,
) -> c_int {
    let ret: c_int;

    ret = snd_intel_dsp_driver_probe(pci);
    if ret != SND_INTEL_DSP_DRIVER_ANY && ret != SND_INTEL_DSP_DRIVER_SOF {
        dev_dbg(&mut (*pci).dev, c"SOF PCI driver not selected, aborting probe\n".as_ptr());
        return -ENODEV;
    }

    sof_pci_probe(pci, pci_id)
}
/* EXPORT_SYMBOL_NS(hda_pci_intel_probe, "SND_SOC_SOF_INTEL_HDA_GENERIC"); */

pub unsafe fn hda_register_clients(sdev: *mut snd_sof_dev) -> c_int {
    hda_probes_register(sdev)
}

pub unsafe fn hda_unregister_clients(sdev: *mut snd_sof_dev) {
    hda_probes_unregister(sdev);
}

/* MODULE_LICENSE("Dual BSD/GPL"); */
/* MODULE_DESCRIPTION("SOF support for HDaudio platforms"); */
/* MODULE_IMPORT_NS("SND_SOC_SOF_PCI_DEV"); */
/* MODULE_IMPORT_NS("SND_SOC_SOF_HDA_AUDIO_CODEC"); */
/* MODULE_IMPORT_NS("SND_SOC_SOF_HDA_AUDIO_CODEC_I915"); */
/* MODULE_IMPORT_NS("SND_SOC_SOF_XTENSA"); */
/* MODULE_IMPORT_NS("SND_INTEL_SOUNDWIRE_ACPI"); */
/* MODULE_IMPORT_NS("SOUNDWIRE_INTEL_INIT"); */
/* MODULE_IMPORT_NS("SOUNDWIRE_INTEL"); */
/* MODULE_IMPORT_NS("SND_SOC_SDW_UTILS"); */
/* MODULE_IMPORT_NS("SND_SOC_SOF_HDA_MLINK"); */
/* MODULE_IMPORT_NS("SND_SOC_SOF_INTEL_HDA_COMMON"); */
/* MODULE_IMPORT_NS("SND_SOC_ACPI_INTEL_MATCH"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
