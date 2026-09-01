// SPDX-License-Identifier: GPL-2.0+
//
// AMD Vangogh ACP PCI Driver
//
// Copyright (C) 2021, 2023 Advanced Micro Devices, Inc. All rights reserved.

// C dependencies: linux/pci.h, linux/module.h, linux/io.h, linux/delay.h,
// linux/platform_device.h, linux/interrupt.h, linux/pm_runtime.h, acp5x.h,
// and ../mach-config.h.

#[repr(C)]
struct acp5x_dev_data {
    acp5x_base: *mut core::ffi::c_void,
    acp5x_audio_mode: bool,
    res: *mut resource,
    pdev: [*mut platform_device; ACP5x_DEVS],
}

unsafe fn acp5x_power_on(acp5x_base: *mut core::ffi::c_void) -> core::ffi::c_int {
    let mut val: u32;
    let mut timeout: core::ffi::c_int;

    val = acp_readl(acp5x_base.byte_add(ACP_PGFSM_STATUS as usize));

    if val == 0 {
        return val as core::ffi::c_int;
    }

    if (val & ACP_PGFSM_STATUS_MASK) != ACP_POWER_ON_IN_PROGRESS {
        acp_writel(
            ACP_PGFSM_CNTL_POWER_ON_MASK,
            acp5x_base.byte_add(ACP_PGFSM_CONTROL as usize),
        );
    }
    timeout = 0;
    while {
        timeout += 1;
        timeout < 500
    } {
        val = acp_readl(acp5x_base.byte_add(ACP_PGFSM_STATUS as usize));
        if (val & ACP_PGFSM_STATUS_MASK) == ACP_POWERED_ON {
            return 0;
        }
        udelay(1);
    }
    -ETIMEDOUT
}

unsafe fn acp5x_reset(acp5x_base: *mut core::ffi::c_void) -> core::ffi::c_int {
    let mut val: u32;
    let mut timeout: core::ffi::c_int;

    acp_writel(1, acp5x_base.byte_add(ACP_SOFT_RESET as usize));
    timeout = 0;
    while {
        timeout += 1;
        timeout < 500
    } {
        val = acp_readl(acp5x_base.byte_add(ACP_SOFT_RESET as usize));
        if (val & ACP_SOFT_RESET_SOFTRESET_AUDDONE_MASK) != 0 {
            break;
        }
        cpu_relax();
    }
    acp_writel(0, acp5x_base.byte_add(ACP_SOFT_RESET as usize));
    timeout = 0;
    while {
        timeout += 1;
        timeout < 500
    } {
        val = acp_readl(acp5x_base.byte_add(ACP_SOFT_RESET as usize));
        if val == 0 {
            return 0;
        }
        cpu_relax();
    }
    -ETIMEDOUT
}

unsafe fn acp5x_enable_interrupts(acp5x_base: *mut core::ffi::c_void) {
    acp_writel(0x01, acp5x_base.byte_add(ACP_EXTERNAL_INTR_ENB as usize));
}

unsafe fn acp5x_disable_interrupts(acp5x_base: *mut core::ffi::c_void) {
    acp_writel(
        ACP_EXT_INTR_STAT_CLEAR_MASK,
        acp5x_base.byte_add(ACP_EXTERNAL_INTR_STAT as usize),
    );
    acp_writel(0x00, acp5x_base.byte_add(ACP_EXTERNAL_INTR_CNTL as usize));
    acp_writel(0x00, acp5x_base.byte_add(ACP_EXTERNAL_INTR_ENB as usize));
}

unsafe fn acp5x_init(acp5x_base: *mut core::ffi::c_void) -> core::ffi::c_int {
    let mut ret: core::ffi::c_int;

    /* power on */
    ret = acp5x_power_on(acp5x_base);
    if ret != 0 {
        pr_err(c"ACP5x power on failed\n".as_ptr());
        return ret;
    }
    acp_writel(0x01, acp5x_base.byte_add(ACP_CONTROL as usize));
    /* Reset */
    ret = acp5x_reset(acp5x_base);
    if ret != 0 {
        pr_err(c"ACP5x reset failed\n".as_ptr());
        return ret;
    }
    acp_writel(0x03, acp5x_base.byte_add(ACP_CLKMUX_SEL as usize));
    acp5x_enable_interrupts(acp5x_base);
    0
}

unsafe fn acp5x_deinit(acp5x_base: *mut core::ffi::c_void) -> core::ffi::c_int {
    let mut ret: core::ffi::c_int;

    acp5x_disable_interrupts(acp5x_base);
    /* Reset */
    ret = acp5x_reset(acp5x_base);
    if ret != 0 {
        pr_err(c"ACP5x reset failed\n".as_ptr());
        return ret;
    }
    acp_writel(0x00, acp5x_base.byte_add(ACP_CLKMUX_SEL as usize));
    acp_writel(0x00, acp5x_base.byte_add(ACP_CONTROL as usize));
    0
}

unsafe fn snd_acp5x_probe(
    pci: *mut pci_dev,
    pci_id: *const pci_device_id,
) -> core::ffi::c_int {
    let mut adata: *mut acp5x_dev_data;
    let mut pdevinfo: [platform_device_info; ACP5x_DEVS] = core::mem::zeroed();
    let mut irqflags: core::ffi::c_uint;
    let mut flag: core::ffi::c_uint;
    let mut ret: core::ffi::c_int;
    let mut i: core::ffi::c_int;
    let mut addr: u32;
    let mut val: u32;

    /*
     * Return if ACP config flag is defined, except when board
     * supports SOF while it is not being enabled in kernel config.
     */
    flag = snd_amd_acp_find_config(pci);
    if flag != FLAG_AMD_LEGACY
        && (flag != FLAG_AMD_SOF || IS_ENABLED_CONFIG_SND_SOC_SOF_AMD_VANGOGH())
    {
        return -ENODEV;
    }

    irqflags = IRQF_SHARED;
    if (*pci).revision != 0x50 {
        return -ENODEV;
    }

    if pci_enable_device(pci) != 0 {
        dev_err(&mut (*pci).dev, c"pci_enable_device failed\n".as_ptr());
        return -ENODEV;
    }

    ret = pci_request_regions(pci, c"AMD ACP5x audio".as_ptr());
    if ret < 0 {
        dev_err(&mut (*pci).dev, c"pci_request_regions failed\n".as_ptr());
        pci_disable_device(pci);
        return ret;
    }

    adata = devm_kzalloc(
        &mut (*pci).dev,
        core::mem::size_of::<acp5x_dev_data>(),
        GFP_KERNEL,
    ) as *mut acp5x_dev_data;
    if adata.is_null() {
        ret = -ENOMEM;
        pci_release_regions(pci);
        pci_disable_device(pci);
        return ret;
    }
    addr = pci_resource_start(pci, 0) as u32;
    (*adata).acp5x_base = devm_ioremap(&mut (*pci).dev, addr, pci_resource_len(pci, 0));
    if (*adata).acp5x_base.is_null() {
        ret = -ENOMEM;
        pci_release_regions(pci);
        pci_disable_device(pci);
        return ret;
    }
    pci_set_master(pci);
    pci_set_drvdata(pci, adata as *mut core::ffi::c_void);
    ret = acp5x_init((*adata).acp5x_base);
    if ret != 0 {
        pci_release_regions(pci);
        pci_disable_device(pci);
        return ret;
    }

    val = acp_readl((*adata).acp5x_base.byte_add(ACP_PIN_CONFIG as usize));
    match val {
        I2S_MODE => {
            (*adata).res = devm_kzalloc(
                &mut (*pci).dev,
                core::mem::size_of::<resource>() * ACP5x_RES,
                GFP_KERNEL,
            ) as *mut resource;
            if (*adata).res.is_null() {
                ret = -ENOMEM;
                if acp5x_deinit((*adata).acp5x_base) != 0 {
                    dev_err(&mut (*pci).dev, c"ACP de-init failed\n".as_ptr());
                }
                pci_release_regions(pci);
                pci_disable_device(pci);
                return ret;
            }

            (*(*adata).res.add(0)).name = c"acp5x_i2s_iomem".as_ptr();
            (*(*adata).res.add(0)).flags = IORESOURCE_MEM;
            (*(*adata).res.add(0)).start = addr;
            (*(*adata).res.add(0)).end = addr + (ACP5x_REG_END - ACP5x_REG_START);

            (*(*adata).res.add(1)).name = c"acp5x_i2s_sp".as_ptr();
            (*(*adata).res.add(1)).flags = IORESOURCE_MEM;
            (*(*adata).res.add(1)).start = addr + ACP5x_I2STDM_REG_START;
            (*(*adata).res.add(1)).end = addr + ACP5x_I2STDM_REG_END;

            (*(*adata).res.add(2)).name = c"acp5x_i2s_hs".as_ptr();
            (*(*adata).res.add(2)).flags = IORESOURCE_MEM;
            (*(*adata).res.add(2)).start = addr + ACP5x_HS_TDM_REG_START;
            (*(*adata).res.add(2)).end = addr + ACP5x_HS_TDM_REG_END;

            (*(*adata).res.add(3)).name = c"acp5x_i2s_irq".as_ptr();
            (*(*adata).res.add(3)).flags = IORESOURCE_IRQ;
            (*(*adata).res.add(3)).start = (*pci).irq;
            (*(*adata).res.add(3)).end = (*(*adata).res.add(3)).start;

            (*adata).acp5x_audio_mode = ACP5x_I2S_MODE;

            pdevinfo = core::mem::zeroed();
            pdevinfo[0].name = c"acp5x_i2s_dma".as_ptr();
            pdevinfo[0].id = 0;
            pdevinfo[0].parent = &mut (*pci).dev;
            pdevinfo[0].num_res = 4;
            pdevinfo[0].res = (*adata).res.add(0);
            pdevinfo[0].data = &mut irqflags as *mut _ as *mut core::ffi::c_void;
            pdevinfo[0].size_data = core::mem::size_of_val(&irqflags);

            pdevinfo[1].name = c"acp5x_i2s_playcap".as_ptr();
            pdevinfo[1].id = 0;
            pdevinfo[1].parent = &mut (*pci).dev;
            pdevinfo[1].num_res = 1;
            pdevinfo[1].res = (*adata).res.add(1);

            pdevinfo[2].name = c"acp5x_i2s_playcap".as_ptr();
            pdevinfo[2].id = 1;
            pdevinfo[2].parent = &mut (*pci).dev;
            pdevinfo[2].num_res = 1;
            pdevinfo[2].res = (*adata).res.add(2);

            pdevinfo[3].name = c"acp5x_mach".as_ptr();
            pdevinfo[3].id = 0;
            pdevinfo[3].parent = &mut (*pci).dev;
            i = 0;
            while i < ACP5x_DEVS as core::ffi::c_int {
                (*adata).pdev[i as usize] = platform_device_register_full(&mut pdevinfo[i as usize]);
                if IS_ERR((*adata).pdev[i as usize] as *const core::ffi::c_void) {
                    dev_err(
                        &mut (*pci).dev,
                        c"cannot register %s device\n".as_ptr(),
                        pdevinfo[i as usize].name,
                    );
                    ret = PTR_ERR((*adata).pdev[i as usize] as *const core::ffi::c_void);
                    i -= 1;
                    while i >= 0 {
                        platform_device_unregister((*adata).pdev[i as usize]);
                        i -= 1;
                    }
                    if acp5x_deinit((*adata).acp5x_base) != 0 {
                        dev_err(&mut (*pci).dev, c"ACP de-init failed\n".as_ptr());
                    }
                    pci_release_regions(pci);
                    pci_disable_device(pci);
                    return ret;
                }
                i += 1;
            }
        }
        _ => {
            dev_info(&mut (*pci).dev, c"ACP audio mode : %d\n".as_ptr(), val);
        }
    }
    pm_runtime_set_autosuspend_delay(&mut (*pci).dev, 2000);
    pm_runtime_use_autosuspend(&mut (*pci).dev);
    pm_runtime_put_noidle(&mut (*pci).dev);
    pm_runtime_allow(&mut (*pci).dev);
    0
}

unsafe fn snd_acp5x_suspend(dev: *mut device) -> core::ffi::c_int {
    let mut ret: core::ffi::c_int;
    let mut adata: *mut acp5x_dev_data;

    adata = dev_get_drvdata(dev) as *mut acp5x_dev_data;
    ret = acp5x_deinit((*adata).acp5x_base);
    if ret != 0 {
        dev_err(dev, c"ACP de-init failed\n".as_ptr());
    } else {
        dev_dbg(dev, c"ACP de-initialized\n".as_ptr());
    }

    ret
}

unsafe fn snd_acp5x_resume(dev: *mut device) -> core::ffi::c_int {
    let mut ret: core::ffi::c_int;
    let mut adata: *mut acp5x_dev_data;

    adata = dev_get_drvdata(dev) as *mut acp5x_dev_data;
    ret = acp5x_init((*adata).acp5x_base);
    if ret != 0 {
        dev_err(dev, c"ACP init failed\n".as_ptr());
        return ret;
    }
    0
}

static acp5x_pm: dev_pm_ops = dev_pm_ops {
    // RUNTIME_PM_OPS(snd_acp5x_suspend, snd_acp5x_resume, NULL)
    // SYSTEM_SLEEP_PM_OPS(snd_acp5x_suspend, snd_acp5x_resume)
    runtime_suspend: Some(snd_acp5x_suspend),
    runtime_resume: Some(snd_acp5x_resume),
    suspend: Some(snd_acp5x_suspend),
    resume: Some(snd_acp5x_resume),
};

unsafe fn snd_acp5x_remove(pci: *mut pci_dev) {
    let mut adata: *mut acp5x_dev_data;
    let mut i: core::ffi::c_int;
    let mut ret: core::ffi::c_int;

    adata = pci_get_drvdata(pci) as *mut acp5x_dev_data;
    if (*adata).acp5x_audio_mode == ACP5x_I2S_MODE {
        i = 0;
        while i < ACP5x_DEVS as core::ffi::c_int {
            platform_device_unregister((*adata).pdev[i as usize]);
            i += 1;
        }
    }
    ret = acp5x_deinit((*adata).acp5x_base);
    if ret != 0 {
        dev_err(&mut (*pci).dev, c"ACP de-init failed\n".as_ptr());
    }
    pm_runtime_forbid(&mut (*pci).dev);
    pm_runtime_get_noresume(&mut (*pci).dev);
    pci_release_regions(pci);
    pci_disable_device(pci);
}

static snd_acp5x_ids: [pci_device_id; 2] = [
    pci_device_id {
        // PCI_DEVICE(PCI_VENDOR_ID_AMD, ACP_DEVICE_ID)
        vendor: PCI_VENDOR_ID_AMD,
        device: ACP_DEVICE_ID,
        subvendor: PCI_ANY_ID,
        subdevice: PCI_ANY_ID,
        class: PCI_CLASS_MULTIMEDIA_OTHER << 8,
        class_mask: 0xffffff,
        driver_data: 0,
    },
    pci_device_id {
        vendor: 0,
        device: 0,
        subvendor: 0,
        subdevice: 0,
        class: 0,
        class_mask: 0,
        driver_data: 0,
    },
];
// MODULE_DEVICE_TABLE(pci, snd_acp5x_ids);

static mut acp5x_driver: pci_driver = pci_driver {
    name: KBUILD_MODNAME,
    id_table: snd_acp5x_ids.as_ptr(),
    probe: Some(snd_acp5x_probe),
    remove: Some(snd_acp5x_remove),
    driver: device_driver {
        pm: pm_ptr(&acp5x_pm),
    },
};

// module_pci_driver(acp5x_driver);
// MODULE_AUTHOR("Vijendar.Mukunda@amd.com");
// MODULE_DESCRIPTION("AMD Vangogh ACP PCI driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
