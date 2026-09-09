// SPDX-License-Identifier: GPL-2.0
/*
 * Driver for FPGA Management Engine (FME)
 *
 * Copyright (C) 2017-2018 Intel Corporation, Inc.
 *
 * Authors:
 *   Kang Luwei <luwei.kang@intel.com>
 *   Xiao Guangrong <guangrong.xiao@linux.intel.com>
 *   Joseph Grecco <joe.grecco@intel.com>
 *   Enno Luebbers <enno.luebbers@intel.com>
 *   Tim Whisonant <tim.whisonant@intel.com>
 *   Ananda Ravuri <ananda.ravuri@intel.com>
 *   Henry Mitchel <henry.mitchel@intel.com>
 */

// Dependencies supplied by the Linux kernel and sibling driver units remain external.

const FME_THERM_THRESHOLD: usize = 0x8;
const TEMP_THRESHOLD1: u64 = (1u64 << 7) - 1;
const TEMP_THRESHOLD1_EN: u64 = 1u64 << 7;
const TEMP_THRESHOLD2: u64 = ((1u64 << 7) - 1) << 8;
const TEMP_THRESHOLD2_EN: u64 = 1u64 << 15;
const TRIP_THRESHOLD: u64 = ((1u64 << 7) - 1) << 24;
const TEMP_THRESHOLD1_STATUS: u64 = 1u64 << 32;
const TEMP_THRESHOLD2_STATUS: u64 = 1u64 << 33;
const TEMP_THRESHOLD1_POLICY: u64 = 1u64 << 44;
const FME_THERM_RDSENSOR_FMT1: usize = 0x10;
const FPGA_TEMPERATURE: u64 = (1u64 << 7) - 1;
const FME_THERM_CAP: usize = 0x20;
const THERM_NO_THROTTLE: u64 = 1;

unsafe fn ports_num_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> ssize_t { let fdata = to_dfl_feature_dev_data(dev); let base = dfl_get_feature_ioaddr_by_id(fdata, FME_FEATURE_ID_HEADER); let v = readq(base.add(FME_HDR_CAP)); scnprintf(buf, PAGE_SIZE, "%u\n", field_get(FME_CAP_NUM_PORTS, v) as u32) }
unsafe fn bitstream_id_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> ssize_t { let fdata = to_dfl_feature_dev_data(dev); let base = dfl_get_feature_ioaddr_by_id(fdata, FME_FEATURE_ID_HEADER); let v = readq(base.add(FME_HDR_BITSTREAM_ID)); scnprintf(buf, PAGE_SIZE, "0x%llx\n", v) }
unsafe fn bitstream_metadata_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> ssize_t { let fdata = to_dfl_feature_dev_data(dev); let base = dfl_get_feature_ioaddr_by_id(fdata, FME_FEATURE_ID_HEADER); let v = readq(base.add(FME_HDR_BITSTREAM_MD)); scnprintf(buf, PAGE_SIZE, "0x%llx\n", v) }
unsafe fn cache_size_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> ssize_t { let fdata = to_dfl_feature_dev_data(dev); let base = dfl_get_feature_ioaddr_by_id(fdata, FME_FEATURE_ID_HEADER); let v = readq(base.add(FME_HDR_CAP)); sprintf(buf, "%u\n", field_get(FME_CAP_CACHE_SIZE, v) as u32) }
unsafe fn fabric_version_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> ssize_t { let fdata = to_dfl_feature_dev_data(dev); let base = dfl_get_feature_ioaddr_by_id(fdata, FME_FEATURE_ID_HEADER); let v = readq(base.add(FME_HDR_CAP)); sprintf(buf, "%u\n", field_get(FME_CAP_FABRIC_VERID, v) as u32) }
unsafe fn socket_id_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> ssize_t { let fdata = to_dfl_feature_dev_data(dev); let base = dfl_get_feature_ioaddr_by_id(fdata, FME_FEATURE_ID_HEADER); let v = readq(base.add(FME_HDR_CAP)); sprintf(buf, "%u\n", field_get(FME_CAP_SOCKET_ID, v) as u32) }

const FME_PWR_STATUS: usize = 0x8;
const FME_LATENCY_TOLERANCE: u64 = 1u64 << 18;
const PWR_CONSUMED: u64 = (1u64 << 18) - 1;
const FME_PWR_THRESHOLD: usize = 0x10;
const PWR_THRESHOLD1: u64 = (1u64 << 7) - 1;
const PWR_THRESHOLD2: u64 = ((1u64 << 7) - 1) << 8;
const PWR_THRESHOLD_MAX: i64 = 0x7f;
const PWR_THRESHOLD1_STATUS: u64 = 1u64 << 16;
const PWR_THRESHOLD2_STATUS: u64 = 1u64 << 17;
const FME_PWR_XEON_LIMIT: usize = 0x18;
const XEON_PWR_LIMIT: u64 = (1u64 << 15) - 1;
const XEON_PWR_EN: u64 = 1u64 << 15;
const FME_PWR_FPGA_LIMIT: usize = 0x20;
const FPGA_PWR_LIMIT: u64 = (1u64 << 15) - 1;
const FPGA_PWR_EN: u64 = 1u64 << 15;

// MD_PRE_DEG

unsafe fn fme_thermal_throttle_support(base: *mut u8) -> bool {
    let v = readq(base.add(FME_THERM_CAP));
    field_get(THERM_NO_THROTTLE, v) == 0
}

unsafe fn thermal_hwmon_attrs_visible(drvdata: *const dfl_feature, _type: hwmon_sensor_types, attr: u32, _channel: i32) -> umode_t {
    if attr == hwmon_temp_input { return 0o444; }
    if fme_thermal_throttle_support((*drvdata).ioaddr) { 0o444 } else { 0 }
}

unsafe fn thermal_hwmon_read(dev: *mut device, _type: hwmon_sensor_types, attr: u32, _channel: i32, val: *mut i64) -> i32 {
    let feature = dev_get_drvdata(dev) as *mut dfl_feature;
    let mut v: u64;
    match attr {
        hwmon_temp_input => { v = readq((*feature).ioaddr.add(FME_THERM_RDSENSOR_FMT1)); *val = (field_get(FPGA_TEMPERATURE, v) * MILLI) as i64; }
        hwmon_temp_max => { v = readq((*feature).ioaddr.add(FME_THERM_THRESHOLD)); *val = (field_get(TEMP_THRESHOLD1, v) * MILLI) as i64; }
        hwmon_temp_crit => { v = readq((*feature).ioaddr.add(FME_THERM_THRESHOLD)); *val = (field_get(TEMP_THRESHOLD2, v) * MILLI) as i64; }
        hwmon_temp_emergency => { v = readq((*feature).ioaddr.add(FME_THERM_THRESHOLD)); *val = (field_get(TRIP_THRESHOLD, v) * MILLI) as i64; }
        hwmon_temp_max_alarm => { v = readq((*feature).ioaddr.add(FME_THERM_THRESHOLD)); *val = field_get(TEMP_THRESHOLD1_STATUS, v) as i64; }
        hwmon_temp_crit_alarm => { v = readq((*feature).ioaddr.add(FME_THERM_THRESHOLD)); *val = field_get(TEMP_THRESHOLD2_STATUS, v) as i64; }
        _ => return -EOPNOTSUPP,
    }
    0
}

unsafe fn temp1_max_policy_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> ssize_t {
    let feature = dev_get_drvdata(dev) as *mut dfl_feature;
    let v = readq((*feature).ioaddr.add(FME_THERM_THRESHOLD));
    sprintf(buf, "%u\n", field_get(TEMP_THRESHOLD1_POLICY, v) as u32)
}

unsafe fn power_hwmon_read(dev: *mut device, _type: hwmon_sensor_types, attr: u32, _channel: i32, val: *mut i64) -> i32 {
    let feature = dev_get_drvdata(dev) as *mut dfl_feature;
    let v;
    match attr {
        hwmon_power_input => { v = readq((*feature).ioaddr.add(FME_PWR_STATUS)); *val = (field_get(PWR_CONSUMED, v) * MICRO) as i64; }
        hwmon_power_max => { v = readq((*feature).ioaddr.add(FME_PWR_THRESHOLD)); *val = (field_get(PWR_THRESHOLD1, v) * MICRO) as i64; }
        hwmon_power_crit => { v = readq((*feature).ioaddr.add(FME_PWR_THRESHOLD)); *val = (field_get(PWR_THRESHOLD2, v) * MICRO) as i64; }
        hwmon_power_max_alarm => { v = readq((*feature).ioaddr.add(FME_PWR_THRESHOLD)); *val = field_get(PWR_THRESHOLD1_STATUS, v) as i64; }
        hwmon_power_crit_alarm => { v = readq((*feature).ioaddr.add(FME_PWR_THRESHOLD)); *val = field_get(PWR_THRESHOLD2_STATUS, v) as i64; }
        _ => return -EOPNOTSUPP,
    }
    0
}

unsafe fn power_hwmon_write(dev: *mut device, _type: hwmon_sensor_types, attr: u32, _channel: i32, mut val: i64) -> i32 {
    let fdata = to_dfl_feature_dev_data((*dev).parent);
    let feature = dev_get_drvdata(dev) as *mut dfl_feature;
    let mut ret = 0;
    val = clamp_val(val / MICRO, 0, PWR_THRESHOLD_MAX);
    mutex_lock(&mut (*fdata).lock);
    match attr {
        hwmon_power_max => { let mut v = readq((*feature).ioaddr.add(FME_PWR_THRESHOLD)); v &= !PWR_THRESHOLD1; v |= field_prep(PWR_THRESHOLD1, val as u64); writeq(v, (*feature).ioaddr.add(FME_PWR_THRESHOLD)); }
        hwmon_power_crit => { let mut v = readq((*feature).ioaddr.add(FME_PWR_THRESHOLD)); v &= !PWR_THRESHOLD2; v |= field_prep(PWR_THRESHOLD2, val as u64); writeq(v, (*feature).ioaddr.add(FME_PWR_THRESHOLD)); }
        _ => ret = -EOPNOTSUPP,
    }
    mutex_unlock(&mut (*fdata).lock);
    ret
}

unsafe fn power_hwmon_attrs_visible(_drvdata: *const core::ffi::c_void, _type: hwmon_sensor_types, attr: u32, _channel: i32) -> umode_t {
    match attr { hwmon_power_input | hwmon_power_max_alarm | hwmon_power_crit_alarm => 0o444, hwmon_power_max | hwmon_power_crit => 0o644, _ => 0 }
}

unsafe fn power1_xeon_limit_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> ssize_t { let feature = dev_get_drvdata(dev) as *mut dfl_feature; let v = readq((*feature).ioaddr.add(FME_PWR_XEON_LIMIT)); let limit = if field_get(XEON_PWR_EN, v) != 0 { field_get(XEON_PWR_LIMIT, v) } else { 0 }; sprintf(buf, "%u\n", limit * 100000) }
unsafe fn power1_fpga_limit_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> ssize_t { let feature = dev_get_drvdata(dev) as *mut dfl_feature; let v = readq((*feature).ioaddr.add(FME_PWR_FPGA_LIMIT)); let limit = if field_get(FPGA_PWR_EN, v) != 0 { field_get(FPGA_PWR_LIMIT, v) } else { 0 }; sprintf(buf, "%u\n", limit * 100000) }
unsafe fn power1_ltr_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> ssize_t { let feature = dev_get_drvdata(dev) as *mut dfl_feature; let v = readq((*feature).ioaddr.add(FME_PWR_STATUS)); sprintf(buf, "%u\n", field_get(FME_LATENCY_TOLERANCE, v) as u32) }

unsafe fn fme_thermal_mgmt_init(pdev: *mut platform_device, feature: *mut dfl_feature) -> i32 {
    let hwmon = devm_hwmon_device_register_with_info(&mut (*pdev).dev, "dfl_fme_thermal", feature, &thermal_hwmon_chip_info, thermal_extra_groups.as_ptr());
    if is_err(hwmon) { dev_err(&mut (*pdev).dev, "Fail to register thermal hwmon\n"); return ptr_err(hwmon); } 0
}
unsafe fn fme_power_mgmt_init(pdev: *mut platform_device, feature: *mut dfl_feature) -> i32 {
    let hwmon = devm_hwmon_device_register_with_info(&mut (*pdev).dev, "dfl_fme_power", feature, &power_hwmon_chip_info, power_extra_groups.as_ptr());
    if is_err(hwmon) { dev_err(&mut (*pdev).dev, "Fail to register power hwmon\n"); return ptr_err(hwmon); } 0
}

// Equivalent to DEVICE_ATTR_RO/ATTRIBUTE_GROUPS and HWMON_CHANNEL_INFO declarations.
extern "C" {
    static mut fme_hdr_id_table: [dfl_feature_id; 0];
    static mut fme_hdr_ops: dfl_feature_ops;
    static mut fme_thermal_mgmt_id_table: [dfl_feature_id; 0];
    static mut fme_thermal_mgmt_ops: dfl_feature_ops;
    static mut fme_power_mgmt_id_table: [dfl_feature_id; 0];
    static mut fme_power_mgmt_ops: dfl_feature_ops;
    static mut thermal_hwmon_chip_info: hwmon_chip_info;
    static mut power_hwmon_chip_info: hwmon_chip_info;
    static mut thermal_extra_groups: [*const attribute_group; 0];
    static mut power_extra_groups: [*const attribute_group; 0];
    static mut fme_fops: file_operations;
}

// The remaining feature tables, hwmon descriptors, platform-driver registration,
// and module metadata are direct declarations over symbols/types supplied by the
// Linux DFL driver framework and sibling translation units.
extern "C" {
    static mut fme_pr_mgmt_id_table: [dfl_feature_id; 0];
    static mut fme_pr_mgmt_ops: dfl_feature_ops;
    static mut fme_global_err_id_table: [dfl_feature_id; 0];
    static mut fme_global_err_ops: dfl_feature_ops;
    static mut fme_perf_id_table: [dfl_feature_id; 0];
    static mut fme_perf_ops: dfl_feature_ops;
}

unsafe fn fme_hdr_ioctl_release_port(fdata: *mut dfl_feature_dev_data, arg: usize) -> i64 {
    let port_id = *(arg as *const i32);
    dfl_fpga_cdev_release_port((*fdata).dfl_cdev, port_id)
}
unsafe fn fme_hdr_ioctl_assign_port(fdata: *mut dfl_feature_dev_data, arg: usize) -> i64 {
    let port_id = *(arg as *const i32);
    dfl_fpga_cdev_assign_port((*fdata).dfl_cdev, port_id)
}
unsafe fn fme_hdr_ioctl(pdev: *mut platform_device, _feature: *mut dfl_feature, cmd: u32, arg: usize) -> i64 {
    let fdata = to_dfl_feature_dev_data(&mut (*pdev).dev);
    match cmd { DFL_FPGA_FME_PORT_RELEASE => fme_hdr_ioctl_release_port(fdata, arg), DFL_FPGA_FME_PORT_ASSIGN => fme_hdr_ioctl_assign_port(fdata, arg), _ => -ENODEV }
}

unsafe fn fme_ioctl_check_extension(_fdata: *mut dfl_feature_dev_data, _arg: usize) -> i64 { 0 }
unsafe fn fme_open(inode: *mut inode, filp: *mut file) -> i32 {
    let fdata = dfl_fpga_inode_to_feature_dev_data(inode); let fdev = (*fdata).dev;
    mutex_lock(&mut (*fdata).lock);
    let ret = dfl_feature_dev_use_begin(fdata, (*filp).f_flags & O_EXCL != 0);
    if ret == 0 { dev_dbg(&mut (*fdev).dev, "Device File Opened %d Times\n", dfl_feature_dev_use_count(fdata)); (*filp).private_data = fdata; }
    mutex_unlock(&mut (*fdata).lock); ret
}
unsafe fn fme_release(_inode: *mut inode, filp: *mut file) -> i32 {
    let fdata = (*filp).private_data as *mut dfl_feature; let pdev = (*fdata).dev;
    mutex_lock(&mut (*fdata).lock); dfl_feature_dev_use_end(fdata);
    if dfl_feature_dev_use_count(fdata) == 0 { dfl_fpga_dev_for_each_feature(fdata, |feature| { dfl_fpga_set_irq_triggers(feature, 0, (*feature).nr_irqs, core::ptr::null_mut()); }); }
    mutex_unlock(&mut (*fdata).lock); 0
}
unsafe fn fme_ioctl(filp: *mut file, cmd: u32, arg: usize) -> i64 {
    let fdata = (*filp).private_data as *mut dfl_feature_dev_data; let pdev = (*fdata).dev;
    match cmd {
        DFL_FPGA_GET_API_VERSION => DFL_FPGA_API_VERSION as i64,
        DFL_FPGA_CHECK_EXTENSION => fme_ioctl_check_extension(fdata, arg),
        _ => { let mut result = -EINVAL; dfl_fpga_dev_for_each_feature(fdata, |f| { if !(*f).ops.is_null() && !(*(*f).ops).ioctl.is_null() { let r = ((*(*f).ops).ioctl)(pdev, f, cmd, arg); if r != -ENODEV { result = r; } } }); result }
    }
}
unsafe fn fme_dev_init(pdev: *mut platform_device) -> i32 {
    let fdata = to_dfl_feature_dev_data(&mut (*pdev).dev); let fme = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<dfl_fme>(), GFP_KERNEL);
    if fme.is_null() { return -ENOMEM; } mutex_lock(&mut (*fdata).lock); dfl_fpga_fdata_set_private(fdata, fme); mutex_unlock(&mut (*fdata).lock); 0
}
unsafe fn fme_dev_destroy(pdev: *mut platform_device) { let fdata = to_dfl_feature_dev_data(&mut (*pdev).dev); mutex_lock(&mut (*fdata).lock); dfl_fpga_fdata_set_private(fdata, core::ptr::null_mut()); mutex_unlock(&mut (*fdata).lock); }
unsafe fn fme_probe(pdev: *mut platform_device) -> i32 {
    let mut ret = fme_dev_init(pdev); if ret != 0 { return ret; }
    ret = dfl_fpga_dev_feature_init(pdev, fme_feature_drvs.as_mut_ptr()); if ret != 0 { fme_dev_destroy(pdev); return ret; }
    ret = dfl_fpga_dev_ops_register(pdev, &fme_fops, THIS_MODULE); if ret != 0 { dfl_fpga_dev_feature_uinit(pdev); fme_dev_destroy(pdev); } ret
}
unsafe fn fme_remove(pdev: *mut platform_device) { dfl_fpga_dev_ops_unregister(pdev); dfl_fpga_dev_feature_uinit(pdev); fme_dev_destroy(pdev); }

static mut fme_feature_drvs: [dfl_feature_driver; 7] = [
    dfl_feature_driver { id_table: fme_hdr_id_table.as_ptr(), ops: &fme_hdr_ops },
    dfl_feature_driver { id_table: fme_pr_mgmt_id_table.as_ptr(), ops: &fme_pr_mgmt_ops },
    dfl_feature_driver { id_table: fme_global_err_id_table.as_ptr(), ops: &fme_global_err_ops },
    dfl_feature_driver { id_table: fme_thermal_mgmt_id_table.as_ptr(), ops: &fme_thermal_mgmt_ops },
    dfl_feature_driver { id_table: fme_power_mgmt_id_table.as_ptr(), ops: &fme_power_mgmt_ops },
    dfl_feature_driver { id_table: fme_perf_id_table.as_ptr(), ops: &fme_perf_ops },
    dfl_feature_driver { id_table: core::ptr::null(), ops: core::ptr::null() },
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
