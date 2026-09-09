// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2023 Intel Corporation */

// Linux kernel headers and project headers are supplied by the surrounding
// translation unit.

#[allow(non_camel_case_types)]
type ssize_t = isize;

extern "C" {
    fn adf_get_pmisc_base(accel_dev: *mut adf_accel_dev) -> *mut core::ffi::c_void;
    fn kmalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn dma_map_single(dev: *mut core::ffi::c_void, ptr: *mut core::ffi::c_void,
                      size: usize, direction: u32) -> dma_addr_t;
    fn dma_mapping_error(dev: *mut core::ffi::c_void, addr: dma_addr_t) -> i32;
    fn dma_unmap_single(dev: *mut core::ffi::c_void, addr: dma_addr_t,
                        size: usize, direction: u32);
    fn adf_get_pm_info(accel_dev: *mut adf_accel_dev, addr: dma_addr_t, size: usize) -> i32;
    fn adf_pm_scnprint_table_lower_keys(dst: *mut i8, rows: *const pm_status_row,
                                        regs: *const u32, size: usize, count: usize) -> i32;
    fn adf_pm_scnprint_table_upper_keys(dst: *mut i8, rows: *const pm_status_row,
                                        regs: *const u32, size: usize, count: usize) -> i32;
    fn scnprintf(dst: *mut i8, size: usize, fmt: *const i8, ...) -> i32;
    fn simple_read_from_buffer(buf: *mut i8, count: usize, pos: *mut i64,
                               src: *const i8, len: usize) -> ssize_t;
    fn ADF_CSR_RD(base: *mut core::ffi::c_void, offset: u32) -> u32;
}

type dma_addr_t = u64;

#[repr(C)]
struct pm_status_row { _private: [u8; 0] }

#[repr(C)]
struct adf_accel_dev {
    power_management: adf_pm,
}

#[repr(C)]
struct adf_pm {
    idle_irq_counters: u32,
    fw_irq_counters: u32,
    throttle_irq_counters: u32,
    host_ack_counter: u32,
    host_nack_counter: u32,
    print_pm_status: Option<unsafe extern "C" fn(*mut adf_accel_dev, *mut i8, usize, *mut i64) -> ssize_t>,
    present: bool,
}

#[repr(C)]
struct icp_qat_fw_init_admin_pm_info {
    max_pwrreq: u32,
    min_pwrreq: u32,
    pwr_state: u32,
}

extern "C" {
    static pm_fuse_rows: [pm_status_row; 3];
    static pm_info_rows: [pm_status_row; 8];
    static pm_ssm_rows: [pm_status_row; 20];
    static pm_log_rows: [pm_status_row; 5];
    static pm_event_rows: [pm_status_row; 8];
    static pm_csrs_rows: [pm_status_row; 4];
}

unsafe extern "C" fn adf_gen4_print_pm_status(
    accel_dev: *mut adf_accel_dev,
    buf: *mut i8,
    count: usize,
    pos: *mut i64,
) -> ssize_t {
    let pmisc = adf_get_pmisc_base(accel_dev);
    let pm = &mut (*accel_dev).power_management;
    let pm_info = kmalloc(4096, 0) as *mut icp_qat_fw_init_admin_pm_info;
    if pm_info.is_null() { return -12; }
    let pm_kv = kmalloc(4096, 0) as *mut i8;
    if pm_kv.is_null() { kfree(pm_info as *mut _); return -12; }

    let p_state_addr = dma_map_single(core::ptr::null_mut(), pm_info as *mut _, 4096, 2);
    let mut ret = dma_mapping_error(core::ptr::null_mut(), p_state_addr);
    if ret != 0 { kfree(pm_info as *mut _); kfree(pm_kv as *mut _); return ret as ssize_t; }
    ret = adf_get_pm_info(accel_dev, p_state_addr, 4096);
    dma_unmap_single(core::ptr::null_mut(), p_state_addr, 4096, 2);
    if ret != 0 { kfree(pm_info as *mut _); kfree(pm_kv as *mut _); return ret as ssize_t; }

    let pm_info_regs = pm_info as *const u32;
    let mut len: i32 = 0;
    let mut append = |fmt: *const i8| { len += scnprintf(pm_kv.add(len as usize), 4096 - len as usize, fmt); };
    append(c"----------- PM Fuse info ---------\n".as_ptr());
    len += adf_pm_scnprint_table_lower_keys(pm_kv.add(len as usize), pm_fuse_rows.as_ptr(), pm_info_regs, 4096 - len as usize, 3);
    append(c"max_pwrreq: %#x\n".as_ptr());
    append(c"min_pwrreq: %#x\n".as_ptr());
    append(c"------------  PM Info ------------\n".as_ptr());
    append(c"power_level: %s\n".as_ptr());
    len += adf_pm_scnprint_table_lower_keys(pm_kv.add(len as usize), pm_info_rows.as_ptr(), pm_info_regs, 4096 - len as usize, 8);
    append(c"pm_mode: STATIC\n".as_ptr());
    append(c"----------- SSM_PM Info ----------\n".as_ptr());
    len += adf_pm_scnprint_table_lower_keys(pm_kv.add(len as usize), pm_ssm_rows.as_ptr(), pm_info_regs, 4096 - len as usize, 20);
    append(c"------------- PM Log -------------\n".as_ptr());
    len += adf_pm_scnprint_table_lower_keys(pm_kv.add(len as usize), pm_log_rows.as_ptr(), pm_info_regs, 4096 - len as usize, 5);
    len += adf_pm_scnprint_table_lower_keys(pm_kv.add(len as usize), pm_event_rows.as_ptr(), pm_info_regs, 4096 - len as usize, 8);
    append(c"idle_irq_count: %#x\n".as_ptr());
    append(c"fw_irq_count: %#x\n".as_ptr());
    append(c"throttle_irq_count: %#x\n".as_ptr());
    append(c"host_ack_count: %#x\n".as_ptr());
    append(c"host_nack_count: %#x\n".as_ptr());
    append(c"----------- HW PM CSRs -----------\n".as_ptr());
    len += adf_pm_scnprint_table_upper_keys(pm_kv.add(len as usize), pm_csrs_rows.as_ptr(), pm_info_regs, 4096 - len as usize, 4);
    let val = ADF_CSR_RD(pmisc, 0);
    append(c"CPM_PM_HOST_MSG: %#x\n".as_ptr());
    let _ = val;
    let val = ADF_CSR_RD(pmisc, 0);
    append(c"CPM_PM_INTERRUPT: %#x\n".as_ptr());
    let _ = val;
    ret = simple_read_from_buffer(buf, count, pos, pm_kv, len as usize) as i32;
    kfree(pm_info as *mut _);
    kfree(pm_kv as *mut _);
    ret as ssize_t
}

pub unsafe extern "C" fn adf_gen4_init_dev_pm_data(accel_dev: *mut adf_accel_dev) {
    (*accel_dev).power_management.print_pm_status = Some(adf_gen4_print_pm_status);
    (*accel_dev).power_management.present = true;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
