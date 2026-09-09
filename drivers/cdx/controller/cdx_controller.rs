// SPDX-License-Identifier: GPL-2.0
/*
 * CDX host controller driver for AMD versal-net platform.
 *
 * Copyright (C) 2022-2023, Advanced Micro Devices, Inc.
 */

// Linux kernel dependencies supplied by the surrounding repository.

unsafe fn cdx_mcdi_rpc_timeout(cdx: *mut cdx_mcdi, cmd: u32) -> u32 {
    MCDI_RPC_TIMEOUT
}

unsafe fn cdx_mcdi_request(
    cdx: *mut cdx_mcdi,
    hdr: *const cdx_dword,
    hdr_len: usize,
    sdu: *const cdx_dword,
    sdu_len: usize,
) {
    if cdx_rpmsg_send(cdx, hdr, hdr_len, sdu, sdu_len) != 0 {
        dev_err((*cdx).rpdev.dev, "Failed to send rpmsg data\n");
    }
}

static mut MCDI_OPS: cdx_mcdi_ops = cdx_mcdi_ops {
    mcdi_rpc_timeout: Some(cdx_mcdi_rpc_timeout),
    mcdi_request: Some(cdx_mcdi_request),
};

unsafe fn cdx_bus_enable(cdx: *mut cdx_controller, bus_num: u8) -> i32 {
    cdx_mcdi_bus_enable((*cdx).priv_, bus_num)
}

unsafe fn cdx_bus_disable(cdx: *mut cdx_controller, bus_num: u8) -> i32 {
    cdx_mcdi_bus_disable((*cdx).priv_, bus_num)
}

pub unsafe fn cdx_rpmsg_post_probe(cdx: *mut cdx_controller) {
    /* Register CDX controller with CDX bus driver */
    if cdx_register_controller(cdx) != 0 {
        dev_err((*cdx).dev, "Failed to register CDX controller\n");
    }
}

pub unsafe fn cdx_rpmsg_pre_remove(cdx: *mut cdx_controller) {
    cdx_unregister_controller(cdx);
    cdx_mcdi_wait_for_quiescence((*cdx).priv_, MCDI_RPC_TIMEOUT);
}

unsafe fn cdx_configure_device(
    cdx: *mut cdx_controller,
    bus_num: u8,
    dev_num: u8,
    dev_config: *mut cdx_device_config,
) -> i32 {
    let mut ret: i32 = 0;
    let (mut msi_index, mut data, mut addr): (u16, u32, u64);

    match (*dev_config).type_ {
        CDX_DEV_MSI_CONF => {
            msi_index = (*dev_config).msi.msi_index;
            data = (*dev_config).msi.data;
            addr = (*dev_config).msi.addr;
            ret = cdx_mcdi_write_msi((*cdx).priv_, bus_num, dev_num, msi_index, addr, data);
        }
        CDX_DEV_RESET_CONF => {
            ret = cdx_mcdi_reset_device((*cdx).priv_, bus_num, dev_num);
        }
        CDX_DEV_BUS_MASTER_CONF => {
            ret = cdx_mcdi_bus_master_enable(
                (*cdx).priv_, bus_num, dev_num, (*dev_config).bus_master_enable,
            );
        }
        CDX_DEV_MSI_ENABLE => {
            ret = cdx_mcdi_msi_enable((*cdx).priv_, bus_num, dev_num, (*dev_config).msi_enable);
        }
        _ => ret = -EINVAL,
    }
    ret
}

unsafe fn cdx_scan_devices(cdx: *mut cdx_controller) -> i32 {
    let cdx_mcdi = (*cdx).priv_;
    let ret = cdx_mcdi_get_num_buses(cdx_mcdi);
    if ret < 0 {
        dev_err((*cdx).dev, "Get number of CDX buses failed: %d\n", ret);
        return ret;
    }
    let num_cdx_bus = ret as u8;

    for bus_num in 0..num_cdx_bus {
        /* Add the bus on cdx subsystem */
        let bus_dev = cdx_bus_add(cdx, bus_num);
        if bus_dev.is_null() {
            continue;
        }

        /* MCDI FW Read: Fetch the number of devices present */
        let ret = cdx_mcdi_get_num_devs(cdx_mcdi, bus_num);
        if ret < 0 {
            dev_err((*cdx).dev, "Get devices on CDX bus %d failed: %d\n", bus_num, ret);
            continue;
        }

        for dev_num in 0..(ret as u8) {
            let mut dev_params: cdx_dev_params = core::mem::zeroed();
            /* MCDI FW: Get the device config */
            let ret = cdx_mcdi_get_dev_config(cdx_mcdi, bus_num, dev_num, &mut dev_params);
            if ret != 0 {
                dev_err((*cdx).dev, "CDX device config get failed for %d(bus):%d(dev), %d\n", bus_num, dev_num, ret);
                continue;
            }
            dev_params.cdx = cdx;
            dev_params.parent = bus_dev;

            /* Add the device to the cdx bus */
            let ret = cdx_device_add(&mut dev_params);
            if ret != 0 {
                dev_err((*cdx).dev, "registering cdx dev: %d failed: %d\n", dev_num, ret);
                continue;
            }
            dev_dbg((*cdx).dev, "CDX dev: %d on cdx bus: %d created\n", dev_num, bus_num);
        }
    }
    0
}

static mut CDX_OPS: cdx_ops = cdx_ops {
    bus_enable: Some(cdx_bus_enable),
    bus_disable: Some(cdx_bus_disable),
    scan: Some(cdx_scan_devices),
    dev_configure: Some(cdx_configure_device),
};

// The remaining probe/remove and module-registration glue is a direct Rust
// representation of the platform-driver lifecycle and depends on kernel APIs.
unsafe fn xlnx_cdx_probe(pdev: *mut platform_device) -> i32 {
    let cdx_mcdi = kzalloc_obj::<cdx_mcdi>();
    if cdx_mcdi.is_null() { return -ENOMEM; }
    (*cdx_mcdi).mcdi_ops = &raw mut MCDI_OPS;
    let mut ret = cdx_mcdi_init(cdx_mcdi);
    if ret != 0 { dev_err_probe((*pdev).dev, ret, "MCDI Initialization failed\n"); cdx_mcdi_finish(cdx_mcdi); kfree(cdx_mcdi); return ret; }

    let cdx = kzalloc_obj::<cdx_controller>();
    if cdx.is_null() { cdx_mcdi_finish(cdx_mcdi); kfree(cdx_mcdi); return -ENOMEM; }
    platform_set_drvdata(pdev, cdx);
    (*cdx).dev = (*pdev).dev;
    (*cdx).priv_ = cdx_mcdi;
    (*cdx).ops = &raw mut CDX_OPS;
    if IS_ENABLED(CONFIG_GENERIC_MSI_IRQ) { (*cdx).msi_domain = cdx_msi_domain_init((*pdev).dev); }
    if (*cdx).msi_domain.is_null() { ret = dev_err_probe((*pdev).dev, -ENODEV, "cdx_msi_domain_init() failed"); kfree(cdx); cdx_mcdi_finish(cdx_mcdi); kfree(cdx_mcdi); return ret; }
    ret = cdx_setup_rpmsg(pdev);
    if ret != 0 { dev_err_probe((*pdev).dev, ret, "Failed to register CDX RPMsg transport\n"); irq_domain_remove((*cdx).msi_domain); kfree(cdx); cdx_mcdi_finish(cdx_mcdi); kfree(cdx_mcdi); return ret; }
    0
}

unsafe fn xlnx_cdx_remove(pdev: *mut platform_device) {
    let cdx = platform_get_drvdata(pdev);
    let cdx_mcdi = (*cdx).priv_;
    cdx_destroy_rpmsg(pdev);
    irq_domain_remove((*cdx).msi_domain);
    kfree(cdx);
    cdx_mcdi_finish(cdx_mcdi);
    kfree(cdx_mcdi);
}

// MODULE_DEVICE_TABLE(of, cdx_match_table);
// module_platform_driver(cdx_pdriver);
// MODULE_AUTHOR("AMD Inc."); MODULE_DESCRIPTION("CDX controller for AMD devices");
// MODULE_LICENSE("GPL"); MODULE_IMPORT_NS("CDX_BUS_CONTROLLER");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
