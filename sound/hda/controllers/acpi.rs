// SPDX-License-Identifier: GPL-2.0-only
/*
 * ALSA driver for ACPI-based HDA Controllers.
 */

// C dependencies translated as external Rust dependencies:
// linux/module.h, linux/platform_device.h, linux/acpi.h,
// sound/hda_codec.h, and "hda_controller.h".

#[repr(C)]
pub struct hda_acpi {
    pub azx: azx,
    pub card: *mut snd_card,
    pub pdev: *mut platform_device,
    pub regs: *mut core::ffi::c_void,
    pub probe_work: work_struct,
    pub data: *const hda_data,
}

/**
 * struct hda_data - Optional device-specific data
 * @short_name: Used for the ALSA card name; defaults to KBUILD_MODNAME
 * @long_name:  Used for longer description; defaults to short_name
 * @flags:      Passed to &azx->driver_caps
 *
 * A pointer to a record of this type may be stored in the
 * &acpi_device_id->driver_data field of an ACPI match table entry in order to
 * customize the naming and behavior of a particular device. All fields are
 * optional and sensible defaults will be selected in their absence.
 */
#[repr(C)]
pub struct hda_data {
    pub short_name: *const core::ffi::c_char,
    pub long_name: *const core::ffi::c_char,
    pub flags: core::ffi::c_ulong,
}

unsafe extern "C" fn hda_acpi_dev_disconnect(device: *mut snd_device) -> core::ffi::c_int {
    let chip: *mut azx = unsafe { (*device).device_data as *mut azx };

    unsafe {
        (*chip).bus.shutdown = 1;
    }
    0
}

unsafe extern "C" fn hda_acpi_dev_free(device: *mut snd_device) -> core::ffi::c_int {
    let azx: *mut azx = unsafe { (*device).device_data as *mut azx };
    let hda: *mut hda_acpi = unsafe { container_of!(azx, hda_acpi, azx) };

    unsafe {
        cancel_work_sync(&mut (*hda).probe_work);
        if (*azx_bus(azx)).chip_init != 0 {
            azx_stop_all_streams(azx);
            azx_stop_chip(azx);
        }

        azx_free_stream_pages(azx);
        azx_free_streams(azx);
        snd_hdac_bus_exit(azx_bus(azx));
    }

    0
}

unsafe extern "C" fn hda_acpi_init(hda: *mut hda_acpi) -> core::ffi::c_int {
    let bus: *mut hdac_bus = unsafe { azx_bus(&mut (*hda).azx) };
    let card: *mut snd_card = unsafe { (*hda).azx.card };
    let dev: *mut device = unsafe { &mut (*(*hda).pdev).dev };
    let azx: *mut azx = unsafe { &mut (*hda).azx };
    let mut res: *mut resource = core::ptr::null_mut();
    let mut gcap: core::ffi::c_ushort;
    let sname: *const core::ffi::c_char;
    let lname: *const core::ffi::c_char;
    let mut err: core::ffi::c_int;
    let irq: core::ffi::c_int;

    /*
     * The base address for the HDA registers and the interrupt are wrapped
     * in an ACPI _CRS object which can be parsed by platform_get_irq() and
     * devm_platform_get_and_ioremap_resource()
     */

    unsafe {
        irq = platform_get_irq((*hda).pdev, 0);
        if irq < 0 {
            return irq;
        }

        (*hda).regs = devm_platform_get_and_ioremap_resource((*hda).pdev, 0, &mut res);
        if IS_ERR((*hda).regs) {
            return PTR_ERR((*hda).regs) as core::ffi::c_int;
        }

        (*bus).remap_addr = (*hda).regs;
        (*bus).addr = (*res).start;

        err = devm_request_irq(
            dev,
            irq,
            Some(azx_interrupt),
            IRQF_SHARED,
            KBUILD_MODNAME,
            azx as *mut core::ffi::c_void,
        );
        if err != 0 {
            dev_err(
                dev,
                c"unable to request IRQ %d, disabling device\n".as_ptr(),
                irq,
            );
            return err;
        }
        (*bus).irq = irq;
        (*bus).dma_stop_delay = 100;
        (*card).sync_irq = (*bus).irq;

        gcap = azx_readw(azx, GCAP);
        dev_dbg(dev, c"chipset global capabilities = 0x%x\n".as_ptr(), gcap as core::ffi::c_int);

        (*azx).align_buffer_size = 1;

        (*azx).capture_streams = ((gcap >> 8) & 0x0f) as _;
        (*azx).playback_streams = ((gcap >> 12) & 0x0f) as _;

        (*azx).capture_index_offset = 0;
        (*azx).playback_index_offset = (*azx).capture_streams;
        (*azx).num_streams = (*azx).playback_streams + (*azx).capture_streams;

        err = azx_init_streams(azx);
        if err < 0 {
            dev_err(dev, c"failed to initialize streams: %d\n".as_ptr(), err);
            return err;
        }

        err = azx_alloc_stream_pages(azx);
        if err < 0 {
            dev_err(dev, c"failed to allocate stream pages: %d\n".as_ptr(), err);
            return err;
        }

        azx_init_chip(azx, 1);

        if (*bus).codec_mask == 0 {
            dev_err(dev, c"no codecs found!\n".as_ptr());
            return -ENODEV;
        }

        strscpy((*card).driver.as_mut_ptr(), c"hda-acpi".as_ptr());

        sname = if !(*(*hda).data).short_name.is_null() {
            (*(*hda).data).short_name
        } else {
            KBUILD_MODNAME
        };

        if strlen(sname) > core::mem::size_of_val(&(*card).shortname) {
            dev_info(dev, c"truncating shortname for card %s\n".as_ptr(), sname);
        }
        strscpy((*card).shortname.as_mut_ptr(), sname);

        lname = if !(*(*hda).data).long_name.is_null() {
            (*(*hda).data).long_name
        } else {
            sname
        };

        snprintf(
            (*card).longname.as_mut_ptr(),
            core::mem::size_of_val(&(*card).longname),
            c"%s at 0x%lx irq %i".as_ptr(),
            lname,
            (*bus).addr,
            (*bus).irq,
        );
    }

    0
}

unsafe extern "C" fn hda_acpi_probe_work(work: *mut work_struct) {
    let hda: *mut hda_acpi = unsafe { container_of!(work, hda_acpi, probe_work) };
    let chip: *mut azx = unsafe { &mut (*hda).azx };
    let mut err: core::ffi::c_int;

    unsafe {
        err = hda_acpi_init(hda);
        if err < 0 {
            return;
        }

        err = azx_probe_codecs(chip, 8);
        if err < 0 {
            return;
        }

        err = azx_codec_configure(chip);
        if err < 0 {
            return;
        }

        err = snd_card_register((*chip).card);
        if err < 0 {
            return;
        }

        (*chip).running = 1;
    }
}

unsafe extern "C" fn hda_acpi_create(hda: *mut hda_acpi) -> core::ffi::c_int {
    static OPS: snd_device_ops = snd_device_ops {
        dev_disconnect: Some(hda_acpi_dev_disconnect),
        dev_free: Some(hda_acpi_dev_free),
    };
    static NULL_OPS: hda_controller_ops = hda_controller_ops {};
    let azx: *mut azx = unsafe { &mut (*hda).azx };
    let mut err: core::ffi::c_int;

    unsafe {
        mutex_init(&mut (*azx).open_mutex);
        (*azx).card = (*hda).card;
        INIT_LIST_HEAD(&mut (*azx).pcm_list);

        (*azx).ops = &NULL_OPS;
        (*azx).driver_caps = (*(*hda).data).flags;
        (*azx).driver_type = ((*(*hda).data).flags & 0xff) as _;
        (*azx).codec_probe_mask = -1;

        err = azx_bus_init(azx, core::ptr::null_mut());
        if err < 0 {
            return err;
        }

        err = snd_device_new(
            (*hda).card,
            SNDRV_DEV_LOWLEVEL,
            &mut (*hda).azx as *mut azx as *mut core::ffi::c_void,
            &OPS,
        );
        if err < 0 {
            dev_err(&mut (*(*hda).pdev).dev, c"Error creating device\n".as_ptr());
            return err;
        }
    }

    0
}

unsafe extern "C" fn hda_acpi_probe(pdev: *mut platform_device) -> core::ffi::c_int {
    let hda: *mut hda_acpi;
    let mut err: core::ffi::c_int;

    unsafe {
        hda = devm_kzalloc(
            &mut (*pdev).dev,
            core::mem::size_of::<hda_acpi>(),
            GFP_KERNEL,
        ) as *mut hda_acpi;
        if hda.is_null() {
            return -ENOMEM;
        }

        (*hda).pdev = pdev;
        (*hda).data = acpi_device_get_match_data(&mut (*pdev).dev) as *const hda_data;

        /* Fall back to defaults if the table didn't have a *struct hda_data */
        if (*hda).data.is_null() {
            (*hda).data = devm_kzalloc(
                &mut (*pdev).dev,
                core::mem::size_of::<hda_data>(),
                GFP_KERNEL,
            ) as *const hda_data;
        }
        if (*hda).data.is_null() {
            return -ENOMEM;
        }

        err = snd_card_new(
            &mut (*pdev).dev,
            SNDRV_DEFAULT_IDX1,
            SNDRV_DEFAULT_STR1,
            THIS_MODULE,
            0,
            &mut (*hda).card,
        );
        if err < 0 {
            dev_err(&mut (*pdev).dev, c"Error creating card!\n".as_ptr());
            return err;
        }

        INIT_WORK(&mut (*hda).probe_work, Some(hda_acpi_probe_work));

        err = hda_acpi_create(hda);
        if err < 0 {
            snd_card_free((*hda).card);
            return err;
        }
        (*(*hda).card).private_data = &mut (*hda).azx as *mut azx as *mut core::ffi::c_void;

        dev_set_drvdata(&mut (*pdev).dev, (*hda).card as *mut core::ffi::c_void);

        schedule_work(&mut (*hda).probe_work);
    }

    0
}

unsafe extern "C" fn hda_acpi_remove(pdev: *mut platform_device) {
    unsafe {
        snd_card_free(dev_get_drvdata(&mut (*pdev).dev) as *mut snd_card);
    }
}

unsafe extern "C" fn hda_acpi_shutdown(pdev: *mut platform_device) {
    let card: *mut snd_card = unsafe { dev_get_drvdata(&mut (*pdev).dev) as *mut snd_card };
    let chip: *mut azx;

    unsafe {
        if card.is_null() {
            return;
        }
        chip = (*card).private_data as *mut azx;
        if !chip.is_null() && (*chip).running != 0 {
            azx_stop_chip(chip);
        }
    }
}

unsafe extern "C" fn hda_acpi_suspend(dev: *mut device) -> core::ffi::c_int {
    let card: *mut snd_card = unsafe { dev_get_drvdata(dev) as *mut snd_card };
    let rc: core::ffi::c_int;

    unsafe {
        rc = pm_runtime_force_suspend(dev);
        if rc < 0 {
            return rc;
        }
        snd_power_change_state(card, SNDRV_CTL_POWER_D3hot);
    }

    0
}

unsafe extern "C" fn hda_acpi_resume(dev: *mut device) -> core::ffi::c_int {
    let card: *mut snd_card = unsafe { dev_get_drvdata(dev) as *mut snd_card };
    let rc: core::ffi::c_int;

    unsafe {
        rc = pm_runtime_force_resume(dev);
        if rc < 0 {
            return rc;
        }
        snd_power_change_state(card, SNDRV_CTL_POWER_D0);
    }

    0
}

static HDA_ACPI_PM: dev_pm_ops = dev_pm_ops {
    // SYSTEM_SLEEP_PM_OPS(hda_acpi_suspend, hda_acpi_resume)
    suspend: Some(hda_acpi_suspend),
    resume: Some(hda_acpi_resume),
};

static NVIDIA_HDA_DATA: hda_data = hda_data {
    short_name: c"NVIDIA".as_ptr(),
    long_name: c"NVIDIA HDA Controller".as_ptr(),
    flags: AZX_DCAPS_CORBRP_SELF_CLEAR,
};

static HDA_ACPI_MATCH: [acpi_device_id; 3] = [
    acpi_device_id {
        id: *b"NVDA2014\0",
        driver_data: &NVIDIA_HDA_DATA as *const hda_data as uintptr_t,
    },
    acpi_device_id {
        id: *b"NVDA2015\0",
        driver_data: &NVIDIA_HDA_DATA as *const hda_data as uintptr_t,
    },
    acpi_device_id {
        id: [0; 9],
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(acpi, hda_acpi_match);

static mut HDA_ACPI_PLATFORM_DRIVER: platform_driver = platform_driver {
    driver: device_driver {
        name: KBUILD_MODNAME,
        pm: &HDA_ACPI_PM,
        acpi_match_table: HDA_ACPI_MATCH.as_ptr(),
    },
    probe: Some(hda_acpi_probe),
    remove: Some(hda_acpi_remove),
    shutdown: Some(hda_acpi_shutdown),
};
module_platform_driver!(HDA_ACPI_PLATFORM_DRIVER);

module_description!("Driver for ACPI-based HDA Controllers");
module_license!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
