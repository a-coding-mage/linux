// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2021-2022 Intel Corporation
//
// Authors: Cezary Rojewski <cezary.rojewski@intel.com>
//          Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
//
// Special thanks to:
//    Krzysztof Hejmowski <krzysztof.hejmowski@intel.com>
//    Michal Sienkiewicz <michal.sienkiewicz@intel.com>
//    Filip Proborszcz
//
// for sharing Intel AudioDSP expertise and helping shape the very
// foundation of this driver
//

static mut pgctl_mask: u32 = AZX_PGCTL_LSRMD_MASK;
// module_param(pgctl_mask, uint, 0444);
// MODULE_PARM_DESC(pgctl_mask, "PCI PGCTL policy override");

static mut cgctl_mask: u32 = AZX_CGCTL_MISCBDCGE_MASK;
// module_param(cgctl_mask, uint, 0444);
// MODULE_PARM_DESC(cgctl_mask, "PCI CGCTL policy override");

unsafe fn avs_hda_update_config_dword(bus: *mut hdac_bus, reg: u32, mask: u32, value: u32) {
    let pci: *mut pci_dev = to_pci_dev((*bus).dev);
    let mut data: u32 = 0;

    pci_read_config_dword(pci, reg, &mut data);
    data &= !mask;
    data |= value & mask;
    pci_write_config_dword(pci, reg, data);
}

pub unsafe fn avs_hda_power_gating_enable(adev: *mut avs_dev, enable: bool) {
    let value: u32 = if enable { 0 } else { pgctl_mask };

    if !avs_platattr_test(adev, ACE) {
        avs_hda_update_config_dword(
            &mut (*adev).base.core,
            AZX_PCIREG_PGCTL,
            pgctl_mask,
            value,
        );
    }
}

unsafe fn avs_hdac_clock_gating_enable(bus: *mut hdac_bus, enable: bool) {
    let adev: *mut avs_dev = hdac_to_avs(bus);
    let value: u32 = if enable { cgctl_mask } else { 0 };

    if !avs_platattr_test(adev, ACE) {
        avs_hda_update_config_dword(bus, AZX_PCIREG_CGCTL, cgctl_mask, value);
    }
}

pub unsafe fn avs_hda_clock_gating_enable(adev: *mut avs_dev, enable: bool) {
    avs_hdac_clock_gating_enable(&mut (*adev).base.core, enable);
}

pub unsafe fn avs_hda_l1sen_enable(adev: *mut avs_dev, enable: bool) {
    if avs_platattr_test(adev, ACE) {
        return;
    }
    if enable {
        if atomic_inc_and_test(&mut (*adev).l1sen_counter) {
            snd_hdac_chip_updatel(
                &mut (*adev).base.core,
                VS_EM2,
                AZX_VS_EM2_L1SEN,
                AZX_VS_EM2_L1SEN,
            );
        }
    } else if atomic_dec_return(&mut (*adev).l1sen_counter) == -1 {
        snd_hdac_chip_updatel(&mut (*adev).base.core, VS_EM2, AZX_VS_EM2_L1SEN, 0);
    }
}

unsafe fn avs_hdac_bus_init_streams(bus: *mut hdac_bus) -> i32 {
    let cp_streams: u32;
    let pb_streams: u32;
    let gcap: u32;

    gcap = snd_hdac_chip_readw(bus, GCAP) as u32;
    cp_streams = (gcap >> 8) & 0x0f;
    pb_streams = (gcap >> 12) & 0x0f;
    (*bus).num_streams = cp_streams + pb_streams;

    snd_hdac_ext_stream_init_all(bus, 0, cp_streams, SNDRV_PCM_STREAM_CAPTURE);
    snd_hdac_ext_stream_init_all(bus, cp_streams, pb_streams, SNDRV_PCM_STREAM_PLAYBACK);

    snd_hdac_bus_alloc_stream_pages(bus)
}

unsafe fn avs_hdac_bus_init_chip(bus: *mut hdac_bus, full_reset: bool) -> bool {
    let adev: *mut avs_dev = hdac_to_avs(bus);
    let mut hlink: *mut hdac_ext_link;
    let ret: bool;

    avs_hdac_clock_gating_enable(bus, false);
    ret = snd_hdac_bus_init_chip(bus, full_reset);

    /* Reset stream-to-link mapping */
    list_for_each_entry!(hlink, &mut (*bus).hlink_list, list, {
        writel(0, (*hlink).ml_addr.add(AZX_REG_ML_LOSIDV as usize));
    });

    avs_hdac_clock_gating_enable(bus, true);

    /* Set DUM bit to address incorrect position reporting for capture
     * streams. In order to do so, CTRL needs to be out of reset state
     */
    if !avs_platattr_test(adev, ACE) {
        snd_hdac_chip_updatel(bus, VS_EM2, AZX_VS_EM2_DUM, AZX_VS_EM2_DUM);
    }

    ret
}

unsafe fn probe_codec(bus: *mut hdac_bus, addr: i32) -> i32 {
    let mut codec: *mut hda_codec;
    let cmd: u32 = ((addr as u32) << 28)
        | ((AC_NODE_ROOT as u32) << 20)
        | ((AC_VERB_PARAMETERS as u32) << 8)
        | AC_PAR_VENDOR_ID as u32;
    let mut res: u32 = !0;
    let ret: i32;

    mutex_lock(&mut (*bus).cmd_mutex);
    snd_hdac_bus_send_cmd(bus, cmd);
    snd_hdac_bus_get_response(bus, addr, &mut res);
    mutex_unlock(&mut (*bus).cmd_mutex);
    if res == !0 {
        return -EIO;
    }

    dev_dbg((*bus).dev, "codec #%d probed OK: 0x%x\n", addr, res);

    codec = snd_hda_codec_device_init(to_hda_bus(bus), addr, "hdaudioB%dD%d", (*bus).idx, addr);
    if IS_ERR(codec) {
        dev_err((*bus).dev, "init codec failed: %ld\n", PTR_ERR(codec));
        return PTR_ERR(codec) as i32;
    }
    /*
     * Allow avs_core suspend by forcing suspended state on all
     * of its codec child devices. Component interested in
     * dealing with hda codecs directly takes pm responsibilities
     */
    pm_runtime_set_suspended(hda_codec_dev(codec));

    /* configure effectively creates new ASoC component */
    ret = snd_hda_codec_configure(codec);
    if ret < 0 {
        dev_warn((*bus).dev, "failed to config codec #%d: %d\n", addr, ret);
        return ret;
    }

    0
}

unsafe fn avs_hdac_bus_probe_codecs(bus: *mut hdac_bus) {
    let mut ret: i32;
    let mut c: i32 = 0;

    /* First try to probe all given codec slots */
    while c < HDA_MAX_CODECS {
        if ((*bus).codec_mask & BIT(c)) == 0 {
            c += 1;
            continue;
        }

        ret = probe_codec(bus, c);
        /* Ignore codecs with no supporting driver. */
        if ret == 0 || ret == -ENODEV {
            c += 1;
            continue;
        }

        /*
         * Some BIOSen give you wrong codec addresses
         * that don't exist
         */
        dev_warn((*bus).dev, "Codec #%d probe error; disabling it...\n", c);
        (*bus).codec_mask &= !BIT(c);
        /*
         * More badly, accessing to a non-existing
         * codec often screws up the controller bus,
         * and disturbs the further communications.
         * Thus if an error occurs during probing,
         * better to reset the controller bus to get
         * back to the sanity state.
         */
        snd_hdac_bus_stop_chip(bus);
        avs_hdac_bus_init_chip(bus, true);
        c += 1;
    }
}

unsafe fn avs_hda_probe_work(work: *mut work_struct) {
    let adev: *mut avs_dev = container_of!(work, avs_dev, probe_work);
    let bus: *mut hdac_bus = &mut (*adev).base.core;
    let mut hlink: *mut hdac_ext_link;
    let ret: i32;

    pm_runtime_set_active((*bus).dev); /* clear runtime_error flag */

    snd_hdac_display_power(bus, HDA_CODEC_IDX_CONTROLLER, true);
    avs_hdac_bus_init_chip(bus, true);
    avs_hdac_bus_probe_codecs(bus);
    snd_hdac_display_power(bus, HDA_CODEC_IDX_CONTROLLER, false);

    /* with all codecs probed, links can be powered down */
    list_for_each_entry!(hlink, &mut (*bus).hlink_list, list, {
        snd_hdac_ext_bus_link_put(bus, hlink);
    });

    snd_hdac_ext_bus_ppcap_enable(bus, true);
    snd_hdac_ext_bus_ppcap_int_enable(bus, true);
    avs_debugfs_init(adev);

    ret = avs_dsp_first_boot_firmware(adev);
    if ret < 0 {
        return;
    }

    acpi_nhlt_get_gbl_table();

    avs_register_all_boards(adev);

    /* configure PM */
    pm_runtime_set_autosuspend_delay((*bus).dev, 2000);
    pm_runtime_use_autosuspend((*bus).dev);
    pm_runtime_put_autosuspend((*bus).dev);
    pm_runtime_allow((*bus).dev);
}

unsafe fn hdac_stream_update_pos(stream: *mut hdac_stream, buffer_size: u64) {
    let mut prev_pos: u64 = 0;
    let pos: u64;
    let num_bytes: u64;

    div64_u64_rem((*stream).curr_pos, buffer_size, &mut prev_pos);
    pos = snd_hdac_stream_get_pos_posbuf(stream);

    if pos < prev_pos {
        num_bytes = (buffer_size - prev_pos) + pos;
    } else {
        num_bytes = pos - prev_pos;
    }

    (*stream).curr_pos = (*stream).curr_pos.wrapping_add(num_bytes);
}

/* called from IRQ */
unsafe fn hdac_update_stream(_bus: *mut hdac_bus, stream: *mut hdac_stream) {
    if !(*stream).substream.is_null() {
        avs_period_elapsed((*stream).substream);
    } else if !(*stream).cstream.is_null() {
        let buffer_size: u64 = (*(*(*stream).cstream).runtime).buffer_size;

        hdac_stream_update_pos(stream, buffer_size);
        snd_compr_fragment_elapsed((*stream).cstream);
    }
}

unsafe fn avs_hda_interrupt(bus: *mut hdac_bus) -> irqreturn_t {
    let mut ret: irqreturn_t = IRQ_NONE;
    let mut status: u32;

    status = snd_hdac_chip_readl(bus, INTSTS);
    if snd_hdac_bus_handle_stream_irq(bus, status, hdac_update_stream) {
        ret = IRQ_HANDLED;
    }

    spin_lock_irq(&mut (*bus).reg_lock);
    /* Clear RIRB interrupt. */
    status = snd_hdac_chip_readb(bus, RIRBSTS) as u32;
    if (status & RIRB_INT_MASK) != 0 {
        if (status & RIRB_INT_RESPONSE) != 0 {
            snd_hdac_bus_update_rirb(bus);
        }
        snd_hdac_chip_writeb(bus, RIRBSTS, RIRB_INT_MASK);
        ret = IRQ_HANDLED;
    }
    spin_unlock_irq(&mut (*bus).reg_lock);

    ret
}

unsafe fn avs_hda_irq_handler(_irq: i32, dev_id: *mut c_void) -> irqreturn_t {
    let bus: *mut hdac_bus = dev_id as *mut hdac_bus;
    let intsts: u32;

    intsts = snd_hdac_chip_readl(bus, INTSTS);
    if intsts == UINT_MAX || (intsts & AZX_INT_GLOBAL_EN) == 0 {
        return IRQ_NONE;
    }

    /* Mask GIE, unmasked in irq_thread(). */
    snd_hdac_chip_updatel(bus, INTCTL, AZX_INT_GLOBAL_EN, 0);

    IRQ_WAKE_THREAD
}

unsafe fn avs_hda_irq_thread(_irq: i32, dev_id: *mut c_void) -> irqreturn_t {
    let bus: *mut hdac_bus = dev_id as *mut hdac_bus;
    let status: u32;

    status = snd_hdac_chip_readl(bus, INTSTS);
    if (status & !AZX_INT_GLOBAL_EN) != 0 {
        avs_hda_interrupt(bus);
    }

    /* Unmask GIE, masked in irq_handler(). */
    snd_hdac_chip_updatel(bus, INTCTL, AZX_INT_GLOBAL_EN, AZX_INT_GLOBAL_EN);

    IRQ_HANDLED
}

unsafe fn avs_dsp_irq_handler(irq: i32, dev_id: *mut c_void) -> irqreturn_t {
    let adev: *mut avs_dev = dev_id as *mut avs_dev;

    avs_hda_irq_handler(irq, &mut (*adev).base.core as *mut _ as *mut c_void)
}

unsafe fn avs_dsp_irq_thread(_irq: i32, dev_id: *mut c_void) -> irqreturn_t {
    let adev: *mut avs_dev = dev_id as *mut avs_dev;
    let bus: *mut hdac_bus = &mut (*adev).base.core;
    let status: u32;

    status = readl((*bus).ppcap.add(AZX_REG_PP_PPSTS as usize));
    if (status & AZX_PPCTL_PIE) != 0 {
        avs_dsp_op!(adev, dsp_interrupt);
    }

    /* Unmask GIE, masked in irq_handler(). */
    snd_hdac_chip_updatel(bus, INTCTL, AZX_INT_GLOBAL_EN, AZX_INT_GLOBAL_EN);

    IRQ_HANDLED
}

unsafe fn avs_hdac_acquire_irq(adev: *mut avs_dev) -> i32 {
    let bus: *mut hdac_bus = &mut (*adev).base.core;
    let pci: *mut pci_dev = to_pci_dev((*bus).dev);
    let mut ret: i32;

    /* request one and check that we only got one interrupt */
    ret = pci_alloc_irq_vectors(pci, 1, 1, PCI_IRQ_MSI | PCI_IRQ_INTX);
    if ret != 1 {
        dev_err((*adev).dev, "Failed to allocate IRQ vector: %d\n", ret);
        return ret;
    }

    ret = pci_request_irq(
        pci,
        0,
        avs_hda_irq_handler,
        avs_hda_irq_thread,
        bus as *mut c_void,
        KBUILD_MODNAME,
    );
    if ret < 0 {
        dev_err((*adev).dev, "Failed to request stream IRQ handler: %d\n", ret);
        pci_free_irq_vectors(pci);
        return ret;
    }

    ret = pci_request_irq(
        pci,
        0,
        avs_dsp_irq_handler,
        avs_dsp_irq_thread,
        adev as *mut c_void,
        KBUILD_MODNAME,
    );
    if ret < 0 {
        dev_err((*adev).dev, "Failed to request IPC IRQ handler: %d\n", ret);
        pci_free_irq(pci, 0, bus as *mut c_void);
        pci_free_irq_vectors(pci);
        return ret;
    }

    0
}

unsafe fn avs_bus_init(adev: *mut avs_dev, pci: *mut pci_dev, id: *const pci_device_id) -> i32 {
    let bus: *mut hda_bus = &mut (*adev).base;
    let mut ipc: *mut avs_ipc;
    let dev: *mut device = &mut (*pci).dev;
    let mut ret: i32;

    ret = snd_hdac_ext_bus_init(&mut (*bus).core, dev, core::ptr::null_mut(), &soc_hda_ext_bus_ops);
    if ret < 0 {
        return ret;
    }

    (*bus).core.use_posbuf = 1;
    (*bus).core.bdl_pos_adj = 0;
    (*bus).core.sync_write = 1;
    (*bus).pci = pci;
    (*bus).mixer_assigned = -1;
    mutex_init(&mut (*bus).prepare_mutex);

    ipc = devm_kzalloc(dev, core::mem::size_of::<avs_ipc>(), GFP_KERNEL) as *mut avs_ipc;
    if ipc.is_null() {
        return -ENOMEM;
    }
    ret = avs_ipc_init(ipc, dev);
    if ret < 0 {
        return ret;
    }

    (*adev).modcfg_buf = devm_kzalloc(dev, AVS_MAILBOX_SIZE, GFP_KERNEL);
    if (*adev).modcfg_buf.is_null() {
        return -ENOMEM;
    }

    (*adev).dev = dev;
    (*adev).spec = (*id).driver_data as *const avs_spec;
    (*adev).ipc = ipc;
    (*adev).hw_cfg.dsp_cores = hweight_long(AVS_MAIN_CORE_MASK);
    INIT_WORK(&mut (*adev).probe_work, avs_hda_probe_work);
    INIT_LIST_HEAD(&mut (*adev).comp_list);
    INIT_LIST_HEAD(&mut (*adev).path_list);
    INIT_LIST_HEAD(&mut (*adev).fw_list);
    init_completion(&mut (*adev).fw_ready);
    spin_lock_init(&mut (*adev).path_list_lock);
    mutex_init(&mut (*adev).modres_mutex);
    mutex_init(&mut (*adev).comp_list_mutex);
    mutex_init(&mut (*adev).path_mutex);

    0
}

unsafe fn avs_pci_probe(pci: *mut pci_dev, id: *const pci_device_id) -> i32 {
    let mut bus: *mut hdac_bus;
    let mut adev: *mut avs_dev;
    let dev: *mut device = &mut (*pci).dev;
    let mut ret: i32;

    ret = snd_intel_dsp_driver_probe(pci);
    match ret {
        SND_INTEL_DSP_DRIVER_ANY | SND_INTEL_DSP_DRIVER_SST | SND_INTEL_DSP_DRIVER_AVS => {}
        _ => return -ENODEV,
    }

    ret = pcim_enable_device(pci);
    if ret < 0 {
        return ret;
    }

    adev = devm_kzalloc(dev, core::mem::size_of::<avs_dev>(), GFP_KERNEL) as *mut avs_dev;
    if adev.is_null() {
        return -ENOMEM;
    }
    bus = &mut (*adev).base.core;

    ret = avs_bus_init(adev, pci, id);
    if ret < 0 {
        dev_err(dev, "failed to init avs bus: %d\n", ret);
        return ret;
    }

    ret = pcim_request_all_regions(pci, "AVS HDAudio");
    if ret < 0 {
        return ret;
    }

    (*bus).addr = pci_resource_start(pci, 0);
    (*bus).remap_addr = pci_ioremap_bar(pci, 0);
    if (*bus).remap_addr.is_null() {
        dev_err((*bus).dev, "ioremap error\n");
        return -ENXIO;
    }

    (*adev).dsp_ba = pci_ioremap_bar(pci, 4);
    if (*adev).dsp_ba.is_null() {
        dev_err((*bus).dev, "ioremap error\n");
        iounmap((*bus).remap_addr);
        return -ENXIO;
    }

    snd_hdac_bus_parse_capabilities(bus);
    if !(*bus).mlcap.is_null() {
        snd_hdac_ext_bus_get_ml_capabilities(bus);
    }

    if dma_set_mask_and_coherent(dev, DMA_BIT_MASK(64)) != 0 {
        dma_set_mask_and_coherent(dev, DMA_BIT_MASK(32));
    }
    dma_set_max_seg_size(dev, UINT_MAX);

    ret = avs_hdac_bus_init_streams(bus);
    if ret < 0 {
        dev_err(dev, "failed to init streams: %d\n", ret);
        iounmap((*adev).dsp_ba);
        iounmap((*bus).remap_addr);
        return ret;
    }

    ret = avs_hdac_acquire_irq(adev);
    if ret < 0 {
        dev_err((*bus).dev, "failed to acquire irq: %d\n", ret);
        snd_hdac_bus_free_stream_pages(bus);
        snd_hdac_ext_stream_free_all(bus);
        iounmap((*adev).dsp_ba);
        iounmap((*bus).remap_addr);
        return ret;
    }

    pci_set_master(pci);
    pci_set_drvdata(pci, bus as *mut c_void);
    device_disable_async_suspend(dev);

    ret = snd_hdac_i915_init(bus);
    if ret == -EPROBE_DEFER {
        pci_free_irq(pci, 0, adev as *mut c_void);
        pci_free_irq(pci, 0, bus as *mut c_void);
        pci_free_irq_vectors(pci);
        pci_clear_master(pci);
        pci_set_drvdata(pci, core::ptr::null_mut());
        snd_hdac_bus_free_stream_pages(bus);
        snd_hdac_ext_stream_free_all(bus);
        iounmap((*adev).dsp_ba);
        iounmap((*bus).remap_addr);
        return ret;
    } else if ret < 0 {
        dev_info((*bus).dev, "i915 init unsuccessful: %d\n", ret);
    }

    schedule_work(&mut (*adev).probe_work);

    0
}

unsafe fn avs_pci_shutdown(pci: *mut pci_dev) {
    let bus: *mut hdac_bus = pci_get_drvdata(pci) as *mut hdac_bus;
    let adev: *mut avs_dev = hdac_to_avs(bus);

    cancel_work_sync(&mut (*adev).probe_work);
    avs_ipc_block((*adev).ipc);

    snd_hdac_stop_streams(bus);
    avs_dsp_op!(adev, int_control, false);
    snd_hdac_ext_bus_ppcap_int_enable(bus, false);
    snd_hdac_ext_bus_link_power_down_all(bus);

    snd_hdac_bus_stop_chip(bus);
    snd_hdac_display_power(bus, HDA_CODEC_IDX_CONTROLLER, false);

    pci_free_irq(pci, 0, adev as *mut c_void);
    pci_free_irq(pci, 0, bus as *mut c_void);
    pci_free_irq_vectors(pci);
}

unsafe fn avs_pci_remove(pci: *mut pci_dev) {
    let mut hdev: *mut hdac_device;
    let mut save: *mut hdac_device;
    let bus: *mut hdac_bus = pci_get_drvdata(pci) as *mut hdac_bus;
    let adev: *mut avs_dev = hdac_to_avs(bus);

    cancel_work_sync(&mut (*adev).probe_work);
    avs_ipc_block((*adev).ipc);

    avs_unregister_all_boards(adev);

    acpi_nhlt_put_gbl_table();
    avs_debugfs_exit(adev);

    if avs_platattr_test(adev, CLDMA) {
        hda_cldma_free(&mut code_loader);
    }

    snd_hdac_stop_streams_and_chip(bus);
    avs_dsp_op!(adev, int_control, false);
    snd_hdac_ext_bus_ppcap_int_enable(bus, false);

    /* it is safe to remove all codecs from the system now */
    list_for_each_entry_safe!(hdev, save, &mut (*bus).codec_list, list, {
        snd_hda_codec_unregister(hdac_to_hda_codec(hdev));
    });

    snd_hdac_bus_free_stream_pages(bus);
    snd_hdac_ext_stream_free_all(bus);
    /* reverse ml_capabilities */
    snd_hdac_ext_link_free_all(bus);
    snd_hdac_ext_bus_exit(bus);

    avs_dsp_core_disable(adev, GENMASK((*adev).hw_cfg.dsp_cores - 1, 0));
    snd_hdac_ext_bus_ppcap_enable(bus, false);

    /* snd_hdac_stop_streams_and_chip does that already? */
    snd_hdac_bus_stop_chip(bus);
    snd_hdac_display_power(bus, HDA_CODEC_IDX_CONTROLLER, false);
    if !(*bus).audio_component.is_null() {
        snd_hdac_i915_exit(bus);
    }

    avs_module_info_free(adev);
    pci_free_irq(pci, 0, adev as *mut c_void);
    pci_free_irq(pci, 0, bus as *mut c_void);
    pci_free_irq_vectors(pci);
    iounmap((*bus).remap_addr);
    iounmap((*adev).dsp_ba);

    /* Firmware is not needed anymore */
    avs_release_firmwares(adev);

    /* pm_runtime_forbid() can rpm_resume() which we do not want */
    pm_runtime_disable(&mut (*pci).dev);
    pm_runtime_forbid(&mut (*pci).dev);
    pm_runtime_enable(&mut (*pci).dev);
    pm_runtime_get_noresume(&mut (*pci).dev);
}

unsafe fn avs_suspend_standby(adev: *mut avs_dev) -> i32 {
    let bus: *mut hdac_bus = &mut (*adev).base.core;
    let pci: *mut pci_dev = (*adev).base.pci;

    if (*bus).cmd_dma_state {
        snd_hdac_bus_stop_cmd_io(bus);
    }

    snd_hdac_ext_bus_link_power_down_all(bus);

    enable_irq_wake((*pci).irq);
    pci_save_state(pci);

    0
}

unsafe fn avs_suspend_common(adev: *mut avs_dev, low_power: bool) -> i32 {
    let bus: *mut hdac_bus = &mut (*adev).base.core;
    let mut ret: i32;

    flush_work(&mut (*adev).probe_work);
    if low_power && (*adev).num_lp_paths != 0 {
        return avs_suspend_standby(adev);
    }

    snd_hdac_ext_bus_link_power_down_all(bus);

    ret = avs_ipc_set_dx(adev, AVS_MAIN_CORE_MASK, false);
    /*
     * pm_runtime is blocked on DSP failure but system-wide suspend is not.
     * Do not block entire system from suspending if that's the case.
     */
    if ret != 0 && ret != -EPERM {
        dev_err((*adev).dev, "set dx failed: %d\n", ret);
        return AVS_IPC_RET(ret);
    }

    avs_ipc_block((*adev).ipc);
    avs_dsp_op!(adev, int_control, false);
    snd_hdac_ext_bus_ppcap_int_enable(bus, false);

    ret = avs_dsp_core_disable(adev, AVS_MAIN_CORE_MASK);
    if ret < 0 {
        dev_err(
            (*adev).dev,
            "core_mask %ld disable failed: %d\n",
            AVS_MAIN_CORE_MASK,
            ret,
        );
        return ret;
    }

    snd_hdac_ext_bus_ppcap_enable(bus, false);
    /* disable LP SRAM retention */
    avs_hda_power_gating_enable(adev, false);
    snd_hdac_bus_stop_chip(bus);
    /* disable CG when putting controller to reset */
    avs_hdac_clock_gating_enable(bus, false);
    snd_hdac_bus_enter_link_reset(bus);
    avs_hdac_clock_gating_enable(bus, true);

    snd_hdac_display_power(bus, HDA_CODEC_IDX_CONTROLLER, false);

    0
}

unsafe fn avs_resume_standby(adev: *mut avs_dev) -> i32 {
    let bus: *mut hdac_bus = &mut (*adev).base.core;
    let pci: *mut pci_dev = (*adev).base.pci;

    pci_restore_state(pci);
    disable_irq_wake((*pci).irq);

    snd_hdac_ext_bus_link_power_up_all(bus);

    if (*bus).cmd_dma_state {
        snd_hdac_bus_init_cmd_io(bus);
    }

    0
}

unsafe fn avs_resume_common(adev: *mut avs_dev, low_power: bool, purge: bool) -> i32 {
    let bus: *mut hdac_bus = &mut (*adev).base.core;
    let ret: i32;

    if low_power && (*adev).num_lp_paths != 0 {
        return avs_resume_standby(adev);
    }

    snd_hdac_display_power(bus, HDA_CODEC_IDX_CONTROLLER, true);
    avs_hdac_bus_init_chip(bus, true);

    snd_hdac_ext_bus_ppcap_enable(bus, true);
    snd_hdac_ext_bus_ppcap_int_enable(bus, true);

    ret = avs_dsp_boot_firmware(adev, purge);
    if ret < 0 {
        dev_err((*adev).dev, "firmware boot failed: %d\n", ret);
        return ret;
    }

    0
}

unsafe fn avs_suspend(dev: *mut device) -> i32 {
    avs_suspend_common(to_avs_dev(dev), true)
}

unsafe fn avs_resume(dev: *mut device) -> i32 {
    avs_resume_common(to_avs_dev(dev), true, true)
}

unsafe fn avs_runtime_suspend(dev: *mut device) -> i32 {
    avs_suspend_common(to_avs_dev(dev), true)
}

unsafe fn avs_runtime_resume(dev: *mut device) -> i32 {
    avs_resume_common(to_avs_dev(dev), true, false)
}

unsafe fn avs_freeze(dev: *mut device) -> i32 {
    avs_suspend_common(to_avs_dev(dev), false)
}

unsafe fn avs_thaw(dev: *mut device) -> i32 {
    avs_resume_common(to_avs_dev(dev), false, true)
}

unsafe fn avs_poweroff(dev: *mut device) -> i32 {
    avs_suspend_common(to_avs_dev(dev), false)
}

unsafe fn avs_restore(dev: *mut device) -> i32 {
    avs_resume_common(to_avs_dev(dev), false, true)
}

static avs_dev_pm: dev_pm_ops = dev_pm_ops {
    suspend: Some(avs_suspend),
    resume: Some(avs_resume),
    freeze: Some(avs_freeze),
    thaw: Some(avs_thaw),
    poweroff: Some(avs_poweroff),
    restore: Some(avs_restore),
    // RUNTIME_PM_OPS(avs_runtime_suspend, avs_runtime_resume, NULL)
    runtime_suspend: Some(avs_runtime_suspend),
    runtime_resume: Some(avs_runtime_resume),
    runtime_idle: None,
};

static skl_sram_spec: avs_sram_spec = avs_sram_spec {
    base_offset: SKL_ADSP_SRAM_BASE_OFFSET,
    window_size: SKL_ADSP_SRAM_WINDOW_SIZE,
};

static apl_sram_spec: avs_sram_spec = avs_sram_spec {
    base_offset: APL_ADSP_SRAM_BASE_OFFSET,
    window_size: APL_ADSP_SRAM_WINDOW_SIZE,
};

static mtl_sram_spec: avs_sram_spec = avs_sram_spec {
    base_offset: MTL_ADSP_SRAM_BASE_OFFSET,
    window_size: MTL_ADSP_SRAM_WINDOW_SIZE,
};

static skl_hipc_spec: avs_hipc_spec = avs_hipc_spec {
    req_offset: SKL_ADSP_REG_HIPCI,
    req_ext_offset: SKL_ADSP_REG_HIPCIE,
    req_busy_mask: SKL_ADSP_HIPCI_BUSY,
    ack_offset: SKL_ADSP_REG_HIPCIE,
    ack_done_mask: SKL_ADSP_HIPCIE_DONE,
    rsp_offset: SKL_ADSP_REG_HIPCT,
    rsp_busy_mask: SKL_ADSP_HIPCT_BUSY,
    ctl_offset: SKL_ADSP_REG_HIPCCTL,
    sts_offset: SKL_ADSP_SRAM_BASE_OFFSET,
};

static apl_hipc_spec: avs_hipc_spec = avs_hipc_spec {
    req_offset: SKL_ADSP_REG_HIPCI,
    req_ext_offset: SKL_ADSP_REG_HIPCIE,
    req_busy_mask: SKL_ADSP_HIPCI_BUSY,
    ack_offset: SKL_ADSP_REG_HIPCIE,
    ack_done_mask: SKL_ADSP_HIPCIE_DONE,
    rsp_offset: SKL_ADSP_REG_HIPCT,
    rsp_busy_mask: SKL_ADSP_HIPCT_BUSY,
    ctl_offset: SKL_ADSP_REG_HIPCCTL,
    sts_offset: APL_ADSP_SRAM_BASE_OFFSET,
};

static cnl_hipc_spec: avs_hipc_spec = avs_hipc_spec {
    req_offset: CNL_ADSP_REG_HIPCIDR,
    req_ext_offset: CNL_ADSP_REG_HIPCIDD,
    req_busy_mask: CNL_ADSP_HIPCIDR_BUSY,
    ack_offset: CNL_ADSP_REG_HIPCIDA,
    ack_done_mask: CNL_ADSP_HIPCIDA_DONE,
    rsp_offset: CNL_ADSP_REG_HIPCTDR,
    rsp_busy_mask: CNL_ADSP_HIPCTDR_BUSY,
    ctl_offset: CNL_ADSP_REG_HIPCCTL,
    sts_offset: APL_ADSP_SRAM_BASE_OFFSET,
};

static lnl_hipc_spec: avs_hipc_spec = avs_hipc_spec {
    req_offset: MTL_REG_HfIPCxIDR,
    req_ext_offset: MTL_REG_HfIPCxIDD,
    req_busy_mask: MTL_HfIPCxIDR_BUSY,
    ack_offset: MTL_REG_HfIPCxIDA,
    ack_done_mask: MTL_HfIPCxIDA_DONE,
    rsp_offset: MTL_REG_HfIPCxTDR,
    rsp_busy_mask: MTL_HfIPCxTDR_BUSY,
    ctl_offset: MTL_REG_HfIPCxCTL,
    sts_offset: LNL_REG_HfDFR(0),
};

static skl_desc: avs_spec = avs_spec {
    name: "skl",
    min_fw_version: [9, 21, 0, 4732],
    dsp_ops: &avs_skl_dsp_ops,
    core_init_mask: 1,
    attributes: AVS_PLATATTR_CLDMA,
    sram: &skl_sram_spec,
    hipc: &skl_hipc_spec,
};

static apl_desc: avs_spec = avs_spec {
    name: "apl",
    min_fw_version: [9, 22, 1, 4323],
    dsp_ops: &avs_apl_dsp_ops,
    core_init_mask: 3,
    attributes: AVS_PLATATTR_IMR,
    sram: &apl_sram_spec,
    hipc: &apl_hipc_spec,
};

static cnl_desc: avs_spec = avs_spec {
    name: "cnl",
    min_fw_version: [10, 23, 0, 5314],
    dsp_ops: &avs_cnl_dsp_ops,
    core_init_mask: 1,
    attributes: AVS_PLATATTR_IMR,
    sram: &apl_sram_spec,
    hipc: &cnl_hipc_spec,
};

static icl_desc: avs_spec = avs_spec {
    name: "icl",
    min_fw_version: [10, 23, 0, 5040],
    dsp_ops: &avs_icl_dsp_ops,
    core_init_mask: 1,
    attributes: AVS_PLATATTR_IMR,
    sram: &apl_sram_spec,
    hipc: &cnl_hipc_spec,
};

static jsl_desc: avs_spec = avs_spec {
    name: "jsl",
    min_fw_version: [10, 26, 0, 5872],
    dsp_ops: &avs_icl_dsp_ops,
    core_init_mask: 1,
    attributes: AVS_PLATATTR_IMR,
    sram: &apl_sram_spec,
    hipc: &cnl_hipc_spec,
};

// AVS_TGL_BASED_SPEC(sname, min)
static lkf_desc: avs_spec = avs_spec {
    name: "lkf",
    min_fw_version: [10, 28, 0, 5646],
    dsp_ops: &avs_tgl_dsp_ops,
    core_init_mask: 1,
    attributes: AVS_PLATATTR_IMR,
    sram: &apl_sram_spec,
    hipc: &cnl_hipc_spec,
};

static tgl_desc: avs_spec = avs_spec {
    name: "tgl",
    min_fw_version: [10, 29, 0, 5646],
    dsp_ops: &avs_tgl_dsp_ops,
    core_init_mask: 1,
    attributes: AVS_PLATATTR_IMR,
    sram: &apl_sram_spec,
    hipc: &cnl_hipc_spec,
};

static ehl_desc: avs_spec = avs_spec {
    name: "ehl",
    min_fw_version: [10, 30, 0, 5646],
    dsp_ops: &avs_tgl_dsp_ops,
    core_init_mask: 1,
    attributes: AVS_PLATATTR_IMR,
    sram: &apl_sram_spec,
    hipc: &cnl_hipc_spec,
};

static adl_desc: avs_spec = avs_spec {
    name: "adl",
    min_fw_version: [10, 35, 0, 5646],
    dsp_ops: &avs_tgl_dsp_ops,
    core_init_mask: 1,
    attributes: AVS_PLATATTR_IMR,
    sram: &apl_sram_spec,
    hipc: &cnl_hipc_spec,
};

static adl_n_desc: avs_spec = avs_spec {
    name: "adl_n",
    min_fw_version: [10, 35, 0, 5646],
    dsp_ops: &avs_tgl_dsp_ops,
    core_init_mask: 1,
    attributes: AVS_PLATATTR_IMR,
    sram: &apl_sram_spec,
    hipc: &cnl_hipc_spec,
};

static fcl_desc: avs_spec = avs_spec {
    name: "fcl",
    min_fw_version: [0, 0, 0, 0],
    dsp_ops: &avs_ptl_dsp_ops,
    core_init_mask: 1,
    attributes: AVS_PLATATTR_IMR | AVS_PLATATTR_ACE | AVS_PLATATTR_ALTHDA,
    sram: &mtl_sram_spec,
    hipc: &lnl_hipc_spec,
};

static avs_ids: [pci_device_id; 36] = [
    PCI_DEVICE_DATA(INTEL, HDA_SKL_LP, &skl_desc),
    PCI_DEVICE_DATA(INTEL, HDA_SKL, &skl_desc),
    PCI_DEVICE_DATA(INTEL, HDA_KBL_LP, &skl_desc),
    PCI_DEVICE_DATA(INTEL, HDA_KBL, &skl_desc),
    PCI_DEVICE_DATA(INTEL, HDA_KBL_H, &skl_desc),
    PCI_DEVICE_DATA(INTEL, HDA_CML_S, &skl_desc),
    PCI_DEVICE_DATA(INTEL, HDA_APL, &apl_desc),
    PCI_DEVICE_DATA(INTEL, HDA_GLK, &apl_desc),
    PCI_DEVICE_DATA(INTEL, HDA_CNL_LP, &cnl_desc),
    PCI_DEVICE_DATA(INTEL, HDA_CNL_H, &cnl_desc),
    PCI_DEVICE_DATA(INTEL, HDA_CML_LP, &cnl_desc),
    PCI_DEVICE_DATA(INTEL, HDA_CML_H, &cnl_desc),
    PCI_DEVICE_DATA(INTEL, HDA_RKL_S, &cnl_desc),
    PCI_DEVICE_DATA(INTEL, HDA_ICL_LP, &icl_desc),
    PCI_DEVICE_DATA(INTEL, HDA_ICL_N, &icl_desc),
    PCI_DEVICE_DATA(INTEL, HDA_ICL_H, &icl_desc),
    PCI_DEVICE_DATA(INTEL, HDA_JSL_N, &jsl_desc),
    PCI_DEVICE_DATA(INTEL, HDA_LKF, &lkf_desc),
    PCI_DEVICE_DATA(INTEL, HDA_TGL_LP, &tgl_desc),
    PCI_DEVICE_DATA(INTEL, HDA_TGL_H, &tgl_desc),
    PCI_DEVICE_DATA(INTEL, HDA_CML_R, &tgl_desc),
    PCI_DEVICE_DATA(INTEL, HDA_EHL_0, &ehl_desc),
    PCI_DEVICE_DATA(INTEL, HDA_EHL_3, &ehl_desc),
    PCI_DEVICE_DATA(INTEL, HDA_ADL_S, &adl_desc),
    PCI_DEVICE_DATA(INTEL, HDA_ADL_P, &adl_desc),
    PCI_DEVICE_DATA(INTEL, HDA_ADL_PS, &adl_desc),
    PCI_DEVICE_DATA(INTEL, HDA_ADL_M, &adl_desc),
    PCI_DEVICE_DATA(INTEL, HDA_ADL_PX, &adl_desc),
    PCI_DEVICE_DATA(INTEL, HDA_ADL_N, &adl_n_desc),
    PCI_DEVICE_DATA(INTEL, HDA_RPL_S, &adl_desc),
    PCI_DEVICE_DATA(INTEL, HDA_RPL_P_0, &adl_desc),
    PCI_DEVICE_DATA(INTEL, HDA_RPL_P_1, &adl_desc),
    PCI_DEVICE_DATA(INTEL, HDA_RPL_M, &adl_desc),
    PCI_DEVICE_DATA(INTEL, HDA_RPL_PX, &adl_desc),
    PCI_DEVICE_DATA(INTEL, HDA_FCL, &fcl_desc),
    pci_device_id { driver_data: 0 },
];
// MODULE_DEVICE_TABLE(pci, avs_ids);

static mut avs_pci_driver: pci_driver = pci_driver {
    name: KBUILD_MODNAME,
    id_table: avs_ids.as_ptr(),
    probe: Some(avs_pci_probe),
    remove: Some(avs_pci_remove),
    shutdown: Some(avs_pci_shutdown),
    dev_groups: avs_attr_groups,
    driver: device_driver {
        pm: pm_ptr(&avs_dev_pm),
    },
};
// module_pci_driver(avs_pci_driver);

// MODULE_AUTHOR("Cezary Rojewski <cezary.rojewski@intel.com>");
// MODULE_AUTHOR("Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>");
// MODULE_DESCRIPTION("Intel cAVS sound driver");
// MODULE_LICENSE("GPL");
// MODULE_FIRMWARE("intel/avs/skl/dsp_basefw.bin");
// MODULE_FIRMWARE("intel/avs/apl/dsp_basefw.bin");
// MODULE_FIRMWARE("intel/avs/cnl/dsp_basefw.bin");
// MODULE_FIRMWARE("intel/avs/icl/dsp_basefw.bin");
// MODULE_FIRMWARE("intel/avs/jsl/dsp_basefw.bin");
// MODULE_FIRMWARE("intel/avs/lkf/dsp_basefw.bin");
// MODULE_FIRMWARE("intel/avs/tgl/dsp_basefw.bin");
// MODULE_FIRMWARE("intel/avs/ehl/dsp_basefw.bin");
// MODULE_FIRMWARE("intel/avs/adl/dsp_basefw.bin");
// MODULE_FIRMWARE("intel/avs/adl_n/dsp_basefw.bin");
// MODULE_FIRMWARE("intel/fcl/dsp_basefw.bin");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
