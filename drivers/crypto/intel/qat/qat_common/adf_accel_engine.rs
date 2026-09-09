// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2014 - 2020 Intel Corporation */
// Dependencies are supplied by the surrounding kernel/QAT codebase.

unsafe fn adf_ae_fw_load_images(
    accel_dev: *mut adf_accel_dev,
    fw_addr: *mut core::ffi::c_void,
    fw_size: u32,
) -> i32 {
    let loader_data = (*accel_dev).fw_loader;
    let hw_device = (*accel_dev).hw_device;
    let loader = (*loader_data).fw_loader;
    let num_objs = ((*hw_device).uof_get_num_objs)(accel_dev);

    for i in 0..num_objs {
        let obj_name = ((*hw_device).uof_get_name)(accel_dev, i);
        let ae_mask = ((*hw_device).uof_get_ae_mask)(accel_dev, i);
        if obj_name.is_null() || ae_mask == 0 {
            dev_err(get_dev(accel_dev), "Invalid UOF image\n");
            adf_ae_fw_release(accel_dev);
            return -EFAULT;
        }

        if qat_uclo_set_cfg_ae_mask(loader, ae_mask) != 0 {
            dev_err(get_dev(accel_dev), "Invalid mask for UOF image\n");
            adf_ae_fw_release(accel_dev);
            return -EFAULT;
        }
        if qat_uclo_map_obj(loader, fw_addr, fw_size, obj_name) != 0 {
            dev_err(get_dev(accel_dev), "Failed to map UOF firmware\n");
            adf_ae_fw_release(accel_dev);
            return -EFAULT;
        }
        if qat_uclo_wr_all_uimage(loader) != 0 {
            dev_err(get_dev(accel_dev), "Failed to load UOF firmware\n");
            adf_ae_fw_release(accel_dev);
            return -EFAULT;
        }
        qat_uclo_del_obj(loader);
    }

    0
}

pub unsafe fn adf_ae_fw_load(accel_dev: *mut adf_accel_dev) -> i32 {
    let loader_data = (*accel_dev).fw_loader;
    let hw_device = (*accel_dev).hw_device;
    let mut fw_addr: *mut core::ffi::c_void;
    let mut mmp_addr: *mut core::ffi::c_void;
    let mut fw_size: u32;
    let mut mmp_size: u32;

    if (*hw_device).fw_name.is_null() { return 0; }

    if request_firmware(&mut (*loader_data).mmp_fw, (*hw_device).fw_mmp_name,
                        (*accel_dev).accel_pci_dev.pci_dev.dev()) != 0 {
        dev_err(get_dev(accel_dev), "Failed to load MMP firmware %s\n", (*hw_device).fw_mmp_name);
        return -EFAULT;
    }
    if request_firmware(&mut (*loader_data).uof_fw, (*hw_device).fw_name,
                        (*accel_dev).accel_pci_dev.pci_dev.dev()) != 0 {
        dev_err(get_dev(accel_dev), "Failed to load UOF firmware %s\n", (*hw_device).fw_name);
        adf_ae_fw_release(accel_dev);
        return -EFAULT;
    }

    fw_size = (*loader_data).uof_fw.size;
    fw_addr = (*loader_data).uof_fw.data as *mut core::ffi::c_void;
    mmp_size = (*loader_data).mmp_fw.size;
    mmp_addr = (*loader_data).mmp_fw.data as *mut core::ffi::c_void;

    if qat_uclo_wr_mimage((*loader_data).fw_loader, mmp_addr, mmp_size) != 0 {
        dev_err(get_dev(accel_dev), "Failed to load MMP\n");
        adf_ae_fw_release(accel_dev);
        return -EFAULT;
    }

    if let Some(get_num_objs) = (*hw_device).uof_get_num_objs {
        return adf_ae_fw_load_images(accel_dev, fw_addr, fw_size);
    }

    if qat_uclo_map_obj((*loader_data).fw_loader, fw_addr, fw_size, core::ptr::null()) != 0 {
        dev_err(get_dev(accel_dev), "Failed to map FW\n");
        adf_ae_fw_release(accel_dev);
        return -EFAULT;
    }
    if qat_uclo_wr_all_uimage((*loader_data).fw_loader) != 0 {
        dev_err(get_dev(accel_dev), "Failed to load UOF\n");
        adf_ae_fw_release(accel_dev);
        return -EFAULT;
    }
    0
}

pub unsafe fn adf_ae_fw_release(accel_dev: *mut adf_accel_dev) {
    let loader_data = (*accel_dev).fw_loader;
    let hw_device = (*accel_dev).hw_device;
    if (*hw_device).fw_name.is_null() { return; }

    qat_uclo_del_obj((*loader_data).fw_loader);
    qat_hal_deinit((*loader_data).fw_loader);
    release_firmware((*loader_data).uof_fw);
    release_firmware((*loader_data).mmp_fw);
    (*loader_data).uof_fw = core::ptr::null_mut();
    (*loader_data).mmp_fw = core::ptr::null_mut();
    (*loader_data).fw_loader = core::ptr::null_mut();
}

pub unsafe fn adf_ae_start(accel_dev: *mut adf_accel_dev) -> i32 {
    let loader_data = (*accel_dev).fw_loader;
    let hw_data = (*accel_dev).hw_device;
    if (*hw_data).fw_name.is_null() { return 0; }
    let ae_ctr = qat_hal_start((*loader_data).fw_loader);
    dev_info(get_dev(accel_dev), "qat_dev%d started %d acceleration engines\n", (*accel_dev).accel_id, ae_ctr);
    0
}

pub unsafe fn adf_ae_stop(accel_dev: *mut adf_accel_dev) -> i32 {
    let loader_data = (*accel_dev).fw_loader;
    let hw_data = (*accel_dev).hw_device;
    let max_aes = get_max_accelengines(accel_dev);
    if (*hw_data).fw_name.is_null() { return 0; }
    let mut ae_ctr = 0;
    for ae in 0..max_aes {
        if (*hw_data).ae_mask & (1u32 << ae) != 0 {
            qat_hal_stop((*loader_data).fw_loader, ae, 0xFF);
            ae_ctr += 1;
        }
    }
    dev_info(get_dev(accel_dev), "qat_dev%d stopped %d acceleration engines\n", (*accel_dev).accel_id, ae_ctr);
    0
}

unsafe fn adf_ae_reset(accel_dev: *mut adf_accel_dev, _ae: i32) -> i32 {
    let loader_data = (*accel_dev).fw_loader;
    qat_hal_reset((*loader_data).fw_loader);
    let reset_delay = (*(*loader_data).fw_loader).chip_info.reset_delay_us;
    if reset_delay != 0 { fsleep(reset_delay); }
    if qat_hal_clr_reset((*loader_data).fw_loader) != 0 { return -EFAULT; }
    0
}

pub unsafe fn adf_ae_init(accel_dev: *mut adf_accel_dev) -> i32 {
    let hw_device = (*accel_dev).hw_device;
    if (*hw_device).fw_name.is_null() { return 0; }
    let loader_data = kzalloc_obj::<adf_fw_loader_data>();
    if loader_data.is_null() { return -ENOMEM; }
    (*accel_dev).fw_loader = loader_data;
    if qat_hal_init(accel_dev) != 0 {
        dev_err(get_dev(accel_dev), "Failed to init the AEs\n");
        kfree(loader_data);
        return -EFAULT;
    }
    if adf_ae_reset(accel_dev, 0) != 0 {
        dev_err(get_dev(accel_dev), "Failed to reset the AEs\n");
        qat_hal_deinit((*loader_data).fw_loader);
        kfree(loader_data);
        return -EFAULT;
    }
    0
}

pub unsafe fn adf_ae_shutdown(accel_dev: *mut adf_accel_dev) -> i32 {
    let loader_data = (*accel_dev).fw_loader;
    let hw_device = (*accel_dev).hw_device;
    if (*hw_device).fw_name.is_null() { return 0; }
    qat_hal_deinit((*loader_data).fw_loader);
    kfree((*accel_dev).fw_loader);
    (*accel_dev).fw_loader = core::ptr::null_mut();
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
