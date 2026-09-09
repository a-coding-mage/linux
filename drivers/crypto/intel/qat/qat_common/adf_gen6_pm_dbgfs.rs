// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2025 Intel Corporation */

// Linux headers and project headers supplying the referenced types, constants,
// macros, and functions are dependencies of this translation.

#[allow(non_upper_case_globals)]
static mut pm_fuse_rows: [pm_status_row; 3] = [
    PM_INFO_REGSET_ENTRY!(fusectl0, ENABLE_PM),
    PM_INFO_REGSET_ENTRY!(fusectl0, ENABLE_PM_IDLE),
    PM_INFO_REGSET_ENTRY!(fusectl0, ENABLE_DEEP_PM_IDLE),
];

#[allow(non_upper_case_globals)]
static mut pm_info_rows: [pm_status_row; 3] = [
    PM_INFO_REGSET_ENTRY!(pm.status, CPM_PM_STATE),
    PM_INFO_REGSET_ENTRY!(pm.fw_init, IDLE_ENABLE),
    PM_INFO_REGSET_ENTRY!(pm.fw_init, IDLE_FILTER),
];

#[allow(non_upper_case_globals)]
static mut pm_ssm_rows: [pm_status_row; 2] = [
    PM_INFO_REGSET_ENTRY!(ssm.pm_enable, SSM_PM_ENABLE),
    PM_INFO_REGSET_ENTRY!(ssm.pm_domain_status, DOMAIN_POWERED_UP),
];

#[allow(non_upper_case_globals)]
static mut pm_csrs_rows: [pm_status_row; 2] = [
    PM_INFO_REGSET_ENTRY32!(pm.fw_init, CPM_PM_FW_INIT),
    PM_INFO_REGSET_ENTRY32!(pm.status, CPM_PM_STATUS),
];

// static_assert(sizeof(struct icp_qat_fw_init_admin_pm_info) < PAGE_SIZE);

unsafe fn adf_gen6_print_pm_status(
    accel_dev: *mut adf_accel_dev,
    buf: *mut core::ffi::c_char,
    count: usize,
    pos: *mut loff_t,
) -> ssize_t {
    let pmisc: *mut core::ffi::c_void = adf_get_pmisc_base(accel_dev);
    let mut pm_info: *mut icp_qat_fw_init_admin_pm_info =
        kzalloc(PAGE_SIZE, GFP_KERNEL) as *mut icp_qat_fw_init_admin_pm_info;
    let p_state_addr: dma_addr_t;
    let mut pm_info_regs: *mut u32;
    let mut len: usize = 0;
    let mut pm_kv: *mut core::ffi::c_char;
    let mut val: u32;
    let mut ret: i32;

    if pm_info.is_null() {
        return -ENOMEM as ssize_t;
    }

    pm_kv = kzalloc(PAGE_SIZE, GFP_KERNEL) as *mut core::ffi::c_char;
    if pm_kv.is_null() {
        kfree(pm_info as *mut core::ffi::c_void);
        return -ENOMEM as ssize_t;
    }

    p_state_addr = dma_map_single(&mut GET_DEV!(accel_dev), pm_info as *mut core::ffi::c_void,
                                  PAGE_SIZE, DMA_FROM_DEVICE);
    ret = dma_mapping_error(&mut GET_DEV!(accel_dev), p_state_addr);
    if ret != 0 {
        kfree(pm_info as *mut core::ffi::c_void);
        kfree(pm_kv as *mut core::ffi::c_void);
        return ret as ssize_t;
    }

    /* Query power management information from QAT FW */
    ret = adf_get_pm_info(accel_dev, p_state_addr, PAGE_SIZE);
    dma_unmap_single(&mut GET_DEV!(accel_dev), p_state_addr, PAGE_SIZE, DMA_FROM_DEVICE);
    if ret != 0 {
        kfree(pm_info as *mut core::ffi::c_void);
        kfree(pm_kv as *mut core::ffi::c_void);
        return ret as ssize_t;
    }

    pm_info_regs = pm_info as *mut u32;

    /* Fuse control register */
    len += scnprintf(pm_kv.add(len), PAGE_SIZE - len,
                     "----------- PM Fuse info ---------\n");
    len += adf_pm_scnprint_table_lower_keys(pm_kv.add(len), pm_fuse_rows.as_ptr(),
                                             pm_info_regs, PAGE_SIZE - len, 3);

    /* Power management */
    len += scnprintf(pm_kv.add(len), PAGE_SIZE - len,
                     "----------- PM Info --------------\n");
    len += adf_pm_scnprint_table_lower_keys(pm_kv.add(len), pm_info_rows.as_ptr(),
                                             pm_info_regs, PAGE_SIZE - len, 3);
    len += scnprintf(pm_kv.add(len), PAGE_SIZE - len, "pm_mode: ACTIVE\n");

    /* Shared Slice Module */
    len += scnprintf(pm_kv.add(len), PAGE_SIZE - len,
                     "----------- SSM_PM Info ----------\n");
    len += adf_pm_scnprint_table_lower_keys(pm_kv.add(len), pm_ssm_rows.as_ptr(),
                                             pm_info_regs, PAGE_SIZE - len, 2);

    /* Control status register content */
    len += scnprintf(pm_kv.add(len), PAGE_SIZE - len,
                     "----------- HW PM CSRs -----------\n");
    len += adf_pm_scnprint_table_upper_keys(pm_kv.add(len), pm_csrs_rows.as_ptr(),
                                             pm_info_regs, PAGE_SIZE - len, 2);

    val = ADF_CSR_RD!(pmisc, ADF_GEN6_PM_INTERRUPT);
    len += scnprintf(pm_kv.add(len), PAGE_SIZE - len, "CPM_PM_INTERRUPT: %#x\n", val);
    ret = simple_read_from_buffer(buf, count, pos, pm_kv, len);

    kfree(pm_info as *mut core::ffi::c_void);
    kfree(pm_kv as *mut core::ffi::c_void);
    ret as ssize_t
}

unsafe fn adf_gen6_init_dev_pm_data(accel_dev: *mut adf_accel_dev) {
    (*accel_dev).power_management.print_pm_status = Some(adf_gen6_print_pm_status);
    (*accel_dev).power_management.present = true;
}

// EXPORT_SYMBOL_GPL(adf_gen6_init_dev_pm_data);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
