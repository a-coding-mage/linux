// SPDX-License-Identifier: GPL-2.0-only
// Copyright(c) 2024 Advanced Micro Devices, Inc.

/*
 *  acp-sdw-sof-mach - ASoC Machine driver for AMD SoundWire platforms
 */

// C include dependencies translated as external Rust dependencies:
// linux/bitmap.h, linux/device.h, linux/dmi.h, linux/module.h,
// linux/soundwire/sdw.h, linux/soundwire/sdw_type.h, sound/soc.h,
// sound/soc-acpi.h, soc_amd_sdw_common.h, ../../codecs/rt711.h

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

static mut sof_sdw_quirk: c_ulong = RT711_JD1 as c_ulong;
static mut quirk_override: c_int = -1;
// module_param_named(quirk, quirk_override, int, 0444);
// MODULE_PARM_DESC(quirk, "Board-specific quirk override");

unsafe fn log_quirks(dev: *mut device) {
    if SOC_JACK_JDSRC(sof_sdw_quirk) != 0 {
        dev_dbg(
            dev,
            c"quirk realtek,jack-detect-source %ld\n".as_ptr(),
            SOC_JACK_JDSRC(sof_sdw_quirk),
        );
    }
    if (sof_sdw_quirk & ASOC_SDW_ACP_DMIC as c_ulong) != 0 {
        dev_dbg(dev, c"quirk SOC_SDW_ACP_DMIC enabled\n".as_ptr());
    }
}

unsafe extern "C" fn sof_sdw_quirk_cb(id: *const dmi_system_id) -> c_int {
    sof_sdw_quirk = (*id).driver_data as c_ulong;
    1
}

static sof_sdw_quirk_table: [dmi_system_id; 2] = [
    dmi_system_id {
        callback: Some(sof_sdw_quirk_cb),
        matches: [
            DMI_MATCH(DMI_SYS_VENDOR, c"AMD".as_ptr()),
            DMI_MATCH(DMI_PRODUCT_NAME, c"Birman-PHX".as_ptr()),
            DMI_MATCH_EMPTY,
            DMI_MATCH_EMPTY,
        ],
        driver_data: RT711_JD2 as *mut c_void,
    },
    dmi_system_id::default(),
];

static mut platform_component: [snd_soc_dai_link_component; 1] = [
    snd_soc_dai_link_component {
        /* name might be overridden during probe */
        name: c"0000:04:00.5".as_ptr(),
        ..snd_soc_dai_link_component::zeroed()
    },
];

static sdw_ops: snd_soc_ops = snd_soc_ops {
    startup: Some(asoc_sdw_startup),
    prepare: Some(asoc_sdw_prepare),
    trigger: Some(asoc_sdw_trigger),
    hw_params: Some(asoc_sdw_hw_params),
    hw_free: Some(asoc_sdw_hw_free),
    shutdown: Some(asoc_sdw_shutdown),
    ..snd_soc_ops::zeroed()
};

static type_strings: [*const c_char; 3] = [
    c"SimpleJack".as_ptr(),
    c"SmartAmp".as_ptr(),
    c"SmartMic".as_ptr(),
];

unsafe fn create_sdw_dailink(
    card: *mut snd_soc_card,
    sof_dai: *mut asoc_sdw_dailink,
    dai_links: *mut *mut snd_soc_dai_link,
    be_id: *mut c_int,
    codec_conf: *mut *mut snd_soc_codec_conf,
) -> c_int {
    let dev = (*card).dev;
    let ctx = snd_soc_card_get_drvdata(card) as *mut asoc_sdw_mc_private;
    let amd_ctx = (*ctx).private as *mut amd_mc_ctx;
    let mut sof_end: *mut asoc_sdw_endpoint;
    let mut cpu_pin_id: c_int = 0;
    let mut stream: c_int;
    let mut ret: c_int;

    sof_end = list_first_entry(&mut (*sof_dai).endpoints, asoc_sdw_endpoint_list_offset());
    while !list_entry_is_head(sof_end, &mut (*sof_dai).endpoints, asoc_sdw_endpoint_list_offset()) {
        if !(*sof_end).name_prefix.is_null() {
            (**codec_conf).dlc.name = (*sof_end).codec_name;
            (**codec_conf).name_prefix = (*sof_end).name_prefix;
            *codec_conf = (*codec_conf).add(1);
        }

        if (*sof_end).include_sidecar {
            ret = ((*(*sof_end).codec_info).add_sidecar.unwrap())(card, dai_links, codec_conf);
            if ret != 0 {
                return ret;
            }
        }
        sof_end = list_next_entry(sof_end, asoc_sdw_endpoint_list_offset());
    }

    stream = 0;
    while stream < SNDRV_PCM_STREAM_LAST + 1 {
        static sdw_stream_name: [*const c_char; 4] = [
            c"SDW%d-PIN%d-PLAYBACK".as_ptr(),
            c"SDW%d-PIN%d-CAPTURE".as_ptr(),
            c"SDW%d-PIN%d-PLAYBACK-%s".as_ptr(),
            c"SDW%d-PIN%d-CAPTURE-%s".as_ptr(),
        ];
        let mut codec_maps: *mut snd_soc_dai_link_ch_map;
        let mut codecs: *mut snd_soc_dai_link_component;
        let mut cpus: *mut snd_soc_dai_link_component;
        let num_cpus = hweight32((*sof_dai).link_mask[stream as usize]) as c_int;
        let num_codecs = (*sof_dai).num_devs[stream as usize];
        let playback: c_int;
        let capture: c_int;
        let mut j: c_int = 0;
        let name: *mut c_char;

        if (*sof_dai).num_devs[stream as usize] == 0 {
            stream += 1;
            continue;
        }

        sof_end = list_first_entry(&mut (*sof_dai).endpoints, asoc_sdw_endpoint_list_offset());

        *be_id = (*(*sof_end).dai_info).dailink[stream as usize];
        if *be_id < 0 {
            dev_err(dev, c"Invalid dailink id %d\n".as_ptr(), *be_id);
            return -EINVAL;
        }

        match (*amd_ctx).acp_rev {
            ACP63_PCI_REV => {
                ret = get_acp63_cpu_pin_id(
                    ffs((*sof_end).link_mask - 1),
                    *be_id,
                    &mut cpu_pin_id,
                    dev,
                );
                if ret != 0 {
                    return ret;
                }
            }
            ACP70_PCI_REV | ACP71_PCI_REV | ACP72_PCI_REV => {
                ret = get_acp70_cpu_pin_id(
                    ffs((*sof_end).link_mask - 1),
                    *be_id,
                    &mut cpu_pin_id,
                    dev,
                );
                if ret != 0 {
                    return ret;
                }
            }
            _ => return -EINVAL,
        }

        /* create stream name according to first link id */
        if (*ctx).append_dai_type {
            name = devm_kasprintf(
                dev,
                GFP_KERNEL,
                sdw_stream_name[(stream + 2) as usize],
                ffs((*sof_end).link_mask) - 1,
                cpu_pin_id,
                type_strings[(*(*sof_end).dai_info).dai_type as usize],
            );
        } else {
            name = devm_kasprintf(
                dev,
                GFP_KERNEL,
                sdw_stream_name[stream as usize],
                ffs((*sof_end).link_mask) - 1,
                cpu_pin_id,
            );
        }
        if name.is_null() {
            return -ENOMEM;
        }

        cpus = devm_kcalloc(dev, num_cpus as usize, size_of::<snd_soc_dai_link_component>(), GFP_KERNEL)
            as *mut snd_soc_dai_link_component;
        if cpus.is_null() {
            return -ENOMEM;
        }

        codecs = devm_kcalloc(
            dev,
            num_codecs as usize,
            size_of::<snd_soc_dai_link_component>(),
            GFP_KERNEL,
        ) as *mut snd_soc_dai_link_component;
        if codecs.is_null() {
            return -ENOMEM;
        }

        codec_maps = devm_kcalloc(
            dev,
            num_codecs as usize,
            size_of::<snd_soc_dai_link_ch_map>(),
            GFP_KERNEL,
        ) as *mut snd_soc_dai_link_ch_map;
        if codec_maps.is_null() {
            return -ENOMEM;
        }

        sof_end = list_first_entry(&mut (*sof_dai).endpoints, asoc_sdw_endpoint_list_offset());
        while !list_entry_is_head(sof_end, &mut (*sof_dai).endpoints, asoc_sdw_endpoint_list_offset()) {
            if (*(*sof_end).dai_info).direction[stream as usize] == 0 {
                sof_end = list_next_entry(sof_end, asoc_sdw_endpoint_list_offset());
                continue;
            }

            let link_num = ffs((*sof_end).link_mask) - 1;

            (*cpus).dai_name = devm_kasprintf(
                dev,
                GFP_KERNEL,
                c"SDW%d Pin%d".as_ptr(),
                link_num,
                cpu_pin_id,
            );
            if (*cpus).dai_name.is_null() {
                return -ENOMEM;
            }
            dev_dbg(dev, c"cpu->dai_name:%s\n".as_ptr(), (*cpus).dai_name);

            (*codec_maps.add(j as usize)).cpu = 0;
            (*codec_maps.add(j as usize)).codec = j;

            (*codecs.add(j as usize)).name = (*sof_end).codec_name;
            (*codecs.add(j as usize)).dai_name = (*(*sof_end).dai_info).dai_name;
            j += 1;
            sof_end = list_next_entry(sof_end, asoc_sdw_endpoint_list_offset());
        }

        WARN_ON((j != num_codecs) as c_int);

        playback = (stream == SNDRV_PCM_STREAM_PLAYBACK) as c_int;
        capture = (stream == SNDRV_PCM_STREAM_CAPTURE) as c_int;

        asoc_sdw_init_dai_link(
            dev,
            *dai_links,
            be_id,
            name,
            playback,
            capture,
            cpus,
            num_cpus,
            platform_component.as_mut_ptr(),
            platform_component.len(),
            codecs,
            num_codecs,
            1,
            Some(asoc_sdw_rtd_init),
            &sdw_ops,
        );

        /*
         * SoundWire DAILINKs use 'stream' functions and Bank Switch operations
         * based on wait_for_completion(), tag them as 'nonatomic'.
         */
        (**dai_links).nonatomic = true;
        (**dai_links).ch_maps = codec_maps;

        sof_end = list_first_entry(&mut (*sof_dai).endpoints, asoc_sdw_endpoint_list_offset());
        while !list_entry_is_head(sof_end, &mut (*sof_dai).endpoints, asoc_sdw_endpoint_list_offset()) {
            if let Some(init) = (*(*sof_end).dai_info).init {
                init(card, *dai_links, (*sof_end).codec_info, playback);
            }
            sof_end = list_next_entry(sof_end, asoc_sdw_endpoint_list_offset());
        }

        *dai_links = (*dai_links).add(1);
        stream += 1;
    }

    0
}

unsafe fn create_sdw_dailinks(
    card: *mut snd_soc_card,
    dai_links: *mut *mut snd_soc_dai_link,
    be_id: *mut c_int,
    mut sof_dais: *mut asoc_sdw_dailink,
    num_dais: c_int,
    codec_conf: *mut *mut snd_soc_codec_conf,
) -> c_int {
    let mut i: c_int;
    let mut ret: c_int;

    /* generate DAI links by each sdw link */
    i = 0;
    while i < num_dais && (*sof_dais).initialised {
        let mut current_be_id: c_int = 0;

        ret = create_sdw_dailink(card, sof_dais, dai_links, &mut current_be_id, codec_conf);
        if ret != 0 {
            return ret;
        }

        /* Update the be_id to match the highest ID used for SDW link */
        if *be_id < current_be_id {
            *be_id = current_be_id;
        }

        sof_dais = sof_dais.add(1);
        i += 1;
    }

    0
}

unsafe fn create_dmic_dailinks(
    card: *mut snd_soc_card,
    dai_links: *mut *mut snd_soc_dai_link,
    be_id: *mut c_int,
    no_pcm: c_int,
) -> c_int {
    let dev = (*card).dev;
    let ret: c_int;

    ret = asoc_sdw_init_simple_dai_link(
        dev,
        *dai_links,
        be_id,
        c"acp-dmic-codec".as_ptr(),
        0,
        1, // DMIC only supports capture
        c"acp-sof-dmic".as_ptr(),
        (*platform_component.as_ptr()).name,
        c"dmic-codec".as_ptr(),
        c"dmic-hifi".as_ptr(),
        no_pcm,
        Some(asoc_sdw_dmic_init),
        ptr::null(),
    );
    if ret != 0 {
        return ret;
    }

    *dai_links = (*dai_links).add(1);

    0
}

unsafe fn sof_card_dai_links_create(card: *mut snd_soc_card) -> c_int {
    let dev = (*card).dev;
    let mach = dev_get_platdata((*card).dev) as *mut snd_soc_acpi_mach;
    let mut sdw_be_num: c_int = 0;
    let mut dmic_num: c_int = 0;
    let ctx = snd_soc_card_get_drvdata(card) as *mut asoc_sdw_mc_private;
    let mach_params = &mut (*mach).mach_params as *mut snd_soc_acpi_mach_params;
    let mut sof_aux: *mut snd_soc_aux_dev;
    let mut codec_conf: *mut snd_soc_codec_conf;
    let mut dai_links: *mut snd_soc_dai_link;
    let mut num_devs: c_int = 0;
    let mut num_ends: c_int = 0;
    let mut num_aux: c_int = 0;
    let num_links: c_int;
    let mut be_id: c_int = 0;
    let mut ret: c_int;

    ret = asoc_sdw_count_sdw_endpoints(card, &mut num_devs, &mut num_ends, &mut num_aux);
    if ret < 0 {
        dev_err(dev, c"failed to count devices/endpoints: %d\n".as_ptr(), ret);
        return ret;
    }

    /* One per DAI link, worst case is a DAI link for every endpoint */
    let sof_dais = kzalloc_objs::<asoc_sdw_dailink>(num_ends);
    if sof_dais.is_null() {
        return -ENOMEM;
    }

    /* One per endpoint, ie. each DAI on each codec/amp */
    let sof_ends = kzalloc_objs::<asoc_sdw_endpoint>(num_ends);
    if sof_ends.is_null() {
        kfree(sof_dais as *mut c_void);
        return -ENOMEM;
    }

    sof_aux = devm_kcalloc(dev, num_aux as usize, size_of::<snd_soc_aux_dev>(), GFP_KERNEL)
        as *mut snd_soc_aux_dev;
    if sof_aux.is_null() {
        kfree(sof_ends as *mut c_void);
        kfree(sof_dais as *mut c_void);
        return -ENOMEM;
    }

    ret = asoc_sdw_parse_sdw_endpoints(dev, ctx, sof_aux, sof_dais, sof_ends, &mut num_devs);
    if ret < 0 {
        kfree(sof_ends as *mut c_void);
        kfree(sof_dais as *mut c_void);
        return ret;
    }

    sdw_be_num = ret;

    /* enable dmic */
    if (sof_sdw_quirk & ASOC_SDW_ACP_DMIC as c_ulong) != 0 || (*mach_params).dmic_num != 0 {
        dmic_num = 1;
    }

    dev_dbg(dev, c"sdw %d, dmic %d".as_ptr(), sdw_be_num, dmic_num);

    codec_conf = devm_kcalloc(
        dev,
        num_devs as usize,
        size_of::<snd_soc_codec_conf>(),
        GFP_KERNEL,
    ) as *mut snd_soc_codec_conf;
    if codec_conf.is_null() {
        kfree(sof_ends as *mut c_void);
        kfree(sof_dais as *mut c_void);
        return -ENOMEM;
    }

    /* allocate BE dailinks */
    num_links = sdw_be_num + dmic_num;
    dai_links = devm_kcalloc(
        dev,
        num_links as usize,
        size_of::<snd_soc_dai_link>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link;
    if dai_links.is_null() {
        kfree(sof_ends as *mut c_void);
        kfree(sof_dais as *mut c_void);
        return -ENOMEM;
    }

    (*card).codec_conf = codec_conf;
    (*card).num_configs = num_devs;
    (*card).dai_link = dai_links;
    (*card).num_links = num_links;
    (*card).aux_dev = sof_aux;
    (*card).num_aux_devs = num_aux;

    /* SDW */
    if sdw_be_num != 0 {
        ret = create_sdw_dailinks(card, &mut dai_links, &mut be_id, sof_dais, num_ends, &mut codec_conf);
        if ret != 0 {
            kfree(sof_ends as *mut c_void);
            kfree(sof_dais as *mut c_void);
            return ret;
        }
    }

    /* dmic */
    if dmic_num > 0 {
        if (*ctx).ignore_internal_dmic {
            dev_warn(dev, c"Ignoring ACP DMIC\n".as_ptr());
        } else {
            ret = create_dmic_dailinks(card, &mut dai_links, &mut be_id, 1);
            if ret != 0 {
                kfree(sof_ends as *mut c_void);
                kfree(sof_dais as *mut c_void);
                return ret;
            }
        }
    }

    WARN_ON((codec_conf != (*card).codec_conf.add((*card).num_configs as usize)) as c_int);
    WARN_ON((dai_links != (*card).dai_link.add((*card).num_links as usize)) as c_int);

    kfree(sof_ends as *mut c_void);
    kfree(sof_dais as *mut c_void);
    ret
}

unsafe extern "C" fn mc_probe(pdev: *mut platform_device) -> c_int {
    let mach = dev_get_platdata(&mut (*pdev).dev) as *mut snd_soc_acpi_mach;
    let card: *mut snd_soc_card;
    let amd_ctx: *mut amd_mc_ctx;
    let ctx: *mut asoc_sdw_mc_private;
    let mut amp_num: c_int = 0;
    let mut i: c_int;
    let mut ret: c_int;

    amd_ctx = devm_kzalloc(&mut (*pdev).dev, size_of::<amd_mc_ctx>(), GFP_KERNEL) as *mut amd_mc_ctx;
    if amd_ctx.is_null() {
        return -ENOMEM;
    }

    (*amd_ctx).acp_rev = (*mach).mach_params.subsystem_rev;
    (*amd_ctx).max_sdw_links = ACP63_SDW_MAX_LINKS;
    ctx = devm_kzalloc(&mut (*pdev).dev, size_of::<asoc_sdw_mc_private>(), GFP_KERNEL)
        as *mut asoc_sdw_mc_private;
    if ctx.is_null() {
        return -ENOMEM;
    }
    (*ctx).codec_info_list_count = asoc_sdw_get_codec_info_list_count();
    (*ctx).private = amd_ctx as *mut c_void;
    card = &mut (*ctx).card;
    (*card).dev = &mut (*pdev).dev;
    (*card).name = c"amd-soundwire".as_ptr();
    (*card).owner = THIS_MODULE;
    (*card).late_probe = Some(asoc_sdw_card_late_probe);

    snd_soc_card_set_drvdata(card, ctx as *mut c_void);

    dmi_check_system(sof_sdw_quirk_table.as_ptr());

    if quirk_override != -1 {
        dev_info(
            &mut (*pdev).dev,
            c"Overriding quirk 0x%lx => 0x%x\n".as_ptr(),
            sof_sdw_quirk,
            quirk_override,
        );
        sof_sdw_quirk = quirk_override as c_ulong;
    }

    log_quirks(&mut (*pdev).dev);

    (*ctx).mc_quirk = sof_sdw_quirk;
    /* reset amp_num to ensure amp_num++ starts from 0 in each probe */
    i = 0;
    while i < (*ctx).codec_info_list_count {
        codec_info_list[i as usize].amp_num = 0;
        i += 1;
    }

    ret = sof_card_dai_links_create(card);
    if ret < 0 {
        return ret;
    }

    /*
     * the default amp_num is zero for each codec and
     * amp_num will only be increased for active amp
     * codecs on used platform
     */
    i = 0;
    while i < (*ctx).codec_info_list_count {
        amp_num += codec_info_list[i as usize].amp_num;
        i += 1;
    }

    (*card).components = devm_kasprintf(&mut (*pdev).dev, GFP_KERNEL, c" cfg-amp:%d".as_ptr(), amp_num);
    if (*card).components.is_null() {
        return -ENOMEM;
    }

    /* Register the card */
    ret = devm_snd_soc_register_card(&mut (*pdev).dev, card);
    if ret != 0 {
        dev_err_probe(
            &mut (*pdev).dev,
            ret,
            c"snd_soc_register_card failed %d\n".as_ptr(),
            ret,
        );
        asoc_sdw_mc_dailink_exit_loop(card);
        return ret;
    }

    platform_set_drvdata(pdev, card as *mut c_void);

    ret
}

unsafe extern "C" fn mc_remove(pdev: *mut platform_device) {
    let card = platform_get_drvdata(pdev) as *mut snd_soc_card;

    asoc_sdw_mc_dailink_exit_loop(card);
}

static mc_id_table: [platform_device_id; 2] = [
    platform_device_id {
        name: *b"amd_sof_sdw\0",
        ..platform_device_id::zeroed()
    },
    platform_device_id::zeroed(),
];
// MODULE_DEVICE_TABLE(platform, mc_id_table);

static mut sof_sdw_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"amd_sof_sdw".as_ptr(),
        pm: &snd_soc_pm_ops,
        ..device_driver::zeroed()
    },
    probe: Some(mc_probe),
    remove: Some(mc_remove),
    id_table: mc_id_table.as_ptr(),
    ..platform_driver::zeroed()
};

// module_platform_driver(sof_sdw_driver);

// MODULE_DESCRIPTION("ASoC AMD SoundWire Generic Machine driver");
// MODULE_AUTHOR("Vijendar Mukunda <Vijendar.Mukunda@amd.com");
// MODULE_LICENSE("GPL");
// MODULE_IMPORT_NS("SND_SOC_SDW_UTILS");
// MODULE_IMPORT_NS("SND_SOC_AMD_SDW_MACH");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
