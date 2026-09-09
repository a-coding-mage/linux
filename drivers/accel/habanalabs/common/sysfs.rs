// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright 2016-2022 HabanaLabs, Ltd.
 * All Rights Reserved.
 */

// Dependency intent: symbols from habanalabs.h, linux/pci.h, and linux/types.h.

unsafe fn clk_max_freq_mhz_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let hdev = dev_get_drvdata(dev);
    let mut value: c_long;
    if !hl_device_operational(hdev, core::ptr::null_mut()) { return -ENODEV; }
    value = hl_fw_get_frequency(hdev, (*hdev).asic_prop.clk_pll_index, false);
    if value < 0 { return value; }
    (*hdev).asic_prop.max_freq_value = value as u64;
    sprintf(buf, "%lu\n", value / 1000 / 1000)
}

unsafe fn clk_max_freq_mhz_store(dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, mut count: usize) -> ssize_t {
    let hdev = dev_get_drvdata(dev);
    let mut value: u64 = 0;
    if !hl_device_operational(hdev, core::ptr::null_mut()) { count = (-ENODEV) as usize; return count as ssize_t; }
    if kstrtoull(buf, 0, &mut value) != 0 { count = (-EINVAL) as usize; return count as ssize_t; }
    (*hdev).asic_prop.max_freq_value = value.wrapping_mul(1000).wrapping_mul(1000);
    hl_fw_set_frequency(hdev, (*hdev).asic_prop.clk_pll_index, (*hdev).asic_prop.max_freq_value);
    count as ssize_t
}

unsafe fn clk_cur_freq_mhz_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let hdev = dev_get_drvdata(dev);
    if !hl_device_operational(hdev, core::ptr::null_mut()) { return -ENODEV; }
    let value = hl_fw_get_frequency(hdev, (*hdev).asic_prop.clk_pll_index, true);
    if value < 0 { return value; }
    sprintf(buf, "%lu\n", value / 1000 / 1000)
}

static mut hl_dev_clk_attrs: [*mut attribute; 3] = [
    &mut dev_attr_clk_max_freq_mhz.attr, &mut dev_attr_clk_cur_freq_mhz.attr, core::ptr::null_mut(),
];

unsafe fn vrm_ver_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let hdev = dev_get_drvdata(dev);
    let cpucp_info = &(*hdev).asic_prop.cpucp_info;
    let version = le32_to_cpu(cpucp_info.infineon_second_stage_version);
    let mask: u32 = 0xff;
    let first = version & mask;
    let second = (version >> 8) & mask;
    let third = (version >> 16) & mask;
    if cpucp_info.infineon_version != 0 && cpucp_info.infineon_second_stage_version != 0 {
        return sprintf(buf, "%#04x %#04x:%#04x:%#04x\n", le32_to_cpu(cpucp_info.infineon_version), first, second, third);
    } else if cpucp_info.infineon_second_stage_version != 0 {
        return sprintf(buf, "%#04x:%#04x:%#04x\n", first, second, third);
    } else if cpucp_info.infineon_version != 0 {
        return sprintf(buf, "%#04x\n", le32_to_cpu(cpucp_info.infineon_version));
    }
    0
}

static mut hl_dev_vrm_attrs: [*mut attribute; 2] = [&mut dev_attr_vrm_ver.attr, core::ptr::null_mut()];

unsafe fn uboot_ver_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t { let hdev = dev_get_drvdata(dev); sprintf(buf, "%s\n", (*hdev).asic_prop.uboot_ver) }
unsafe fn armcp_kernel_ver_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t { let hdev = dev_get_drvdata(dev); sprintf(buf, "%s", (*hdev).asic_prop.cpucp_info.kernel_version) }
unsafe fn armcp_ver_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t { let hdev = dev_get_drvdata(dev); sprintf(buf, "%s\n", (*hdev).asic_prop.cpucp_info.cpucp_version) }
unsafe fn cpld_ver_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t { let hdev = dev_get_drvdata(dev); sprintf(buf, "0x%08x%08x\n", le32_to_cpu((*hdev).asic_prop.cpucp_info.cpld_timestamp), le32_to_cpu((*hdev).asic_prop.cpucp_info.cpld_version)) }
unsafe fn cpucp_kernel_ver_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t { armcp_kernel_ver_show(dev, _attr, buf) }
unsafe fn cpucp_ver_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t { armcp_ver_show(dev, _attr, buf) }
unsafe fn fuse_ver_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t { let hdev = dev_get_drvdata(dev); sprintf(buf, "%s\n", (*hdev).asic_prop.cpucp_info.fuse_version) }
unsafe fn thermal_ver_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t { let hdev = dev_get_drvdata(dev); sprintf(buf, "%s", (*hdev).asic_prop.cpucp_info.thermal_version) }
unsafe fn fw_os_ver_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t { let hdev = dev_get_drvdata(dev); sprintf(buf, "%s", (*hdev).asic_prop.cpucp_info.fw_os_version) }
unsafe fn preboot_btl_ver_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t { let hdev = dev_get_drvdata(dev); sprintf(buf, "%s\n", (*hdev).asic_prop.preboot_ver) }

unsafe fn soft_reset_store(dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, mut count: usize) -> ssize_t {
    let hdev = dev_get_drvdata(dev); let mut value: c_long = 0;
    if kstrtoul(buf, 0, &mut value) != 0 { count = (-EINVAL) as usize; return count as ssize_t; }
    if !(*hdev).asic_prop.allow_inference_soft_reset { dev_err((*hdev).dev, "Device does not support inference soft-reset\n"); return count as ssize_t; }
    dev_warn((*hdev).dev, "Inference Soft-Reset requested through sysfs\n"); hl_device_reset(hdev, 0); count as ssize_t
}
unsafe fn hard_reset_store(dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, mut count: usize) -> ssize_t {
    let hdev = dev_get_drvdata(dev); let mut value: c_long = 0;
    if kstrtoul(buf, 0, &mut value) != 0 { count = (-EINVAL) as usize; return count as ssize_t; }
    dev_warn((*hdev).dev, "Hard-Reset requested through sysfs\n"); hl_device_reset(hdev, HL_DRV_RESET_HARD); count as ssize_t
}

unsafe fn device_type_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t {
    let hdev = dev_get_drvdata(dev);
    let s = match (*hdev).asic_type { ASIC_GOYA => "GOYA", ASIC_GAUDI => "GAUDI", ASIC_GAUDI_SEC => "GAUDI SEC", ASIC_GAUDI2 => "GAUDI2", ASIC_GAUDI2B => "GAUDI2B", ASIC_GAUDI2C => "GAUDI2C", ASIC_GAUDI2D => "GAUDI2D", _ => { dev_err((*hdev).dev, "Unrecognized ASIC type %d\n", (*hdev).asic_type); return -EINVAL; } };
    sprintf(buf, "%s\n", s)
}
unsafe fn pci_addr_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t { let hdev = dev_get_drvdata(dev); sprintf(buf, "%04x:%02x:%02x.%x\n", pci_domain_nr((*hdev).pdev.bus), (*hdev).pdev.bus.number, PCI_SLOT((*hdev).pdev.devfn), PCI_FUNC((*hdev).pdev.devfn)) }
unsafe fn status_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t { let hdev = dev_get_drvdata(dev); let mut str_ = [0i8; HL_STR_MAX]; strscpy(str_.as_mut_ptr(), (*hdev).status[hl_device_status(hdev)], HL_STR_MAX); str_[0] = b'A' as i8 + (str_[0] - b'a' as i8); sprintf(buf, "%s\n", str_.as_ptr()) }
unsafe fn soft_reset_cnt_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t { let hdev = dev_get_drvdata(dev); sprintf(buf, "%d\n", (*hdev).reset_info.compute_reset_cnt) }
unsafe fn hard_reset_cnt_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t { let hdev = dev_get_drvdata(dev); sprintf(buf, "%d\n", (*hdev).reset_info.hard_reset_cnt) }
unsafe fn max_power_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut c_char) -> ssize_t { let hdev = dev_get_drvdata(dev); if !hl_device_operational(hdev, core::ptr::null_mut()) { return -ENODEV; } let val = hl_fw_get_max_power(hdev); if val < 0 { return val; } sprintf(buf, "%lu\n", val) }
unsafe fn max_power_store(dev: *mut device, _attr: *mut device_attribute, buf: *const c_char, mut count: usize) -> ssize_t { let hdev = dev_get_drvdata(dev); let mut value: c_ulong = 0; if !hl_device_operational(hdev, core::ptr::null_mut()) { count=(-ENODEV) as usize; return count as ssize_t; } if kstrtoul(buf, 0, &mut value)!=0 { count=(-EINVAL) as usize; return count as ssize_t; } (*hdev).max_power=value; hl_fw_set_max_power(hdev); count as ssize_t }

unsafe fn eeprom_read_handler(_filp: *mut file, kobj: *mut kobject, _attr: *const bin_attribute, buf: *mut c_char, _offset: loff_t, max_size: usize) -> ssize_t {
    let hdev = dev_get_drvdata(kobj_to_dev(kobj)); if !hl_device_operational(hdev, core::ptr::null_mut()) { return -ENODEV; } if max_size == 0 { return -EINVAL; }
    let data = kzalloc(max_size, GFP_KERNEL); if data.is_null() { return -ENOMEM; }
    let rc = ((*(*hdev).asic_funcs).get_eeprom_data)(hdev, data, max_size); if rc == 0 { memcpy(buf, data, max_size); } kfree(data); max_size as ssize_t
}
unsafe fn security_enabled_show(dev:*mut device,_attr:*mut device_attribute,buf:*mut c_char)->ssize_t { let h=dev_get_drvdata(dev); sprintf(buf,"%d\n",(*h).asic_prop.fw_security_enabled) }
unsafe fn module_id_show(dev:*mut device,_attr:*mut device_attribute,buf:*mut c_char)->ssize_t { let h=dev_get_drvdata(dev); sprintf(buf,"%u\n",le32_to_cpu((*h).asic_prop.cpucp_info.card_location)) }
unsafe fn parent_device_show(dev:*mut device,_attr:*mut device_attribute,buf:*mut c_char)->ssize_t { let h=dev_get_drvdata(dev); sprintf(buf,"%s\n",HL_DEV_NAME(h)) }

// DEVICE_ATTR_* declarations and the attribute arrays are generated by the kernel macros.
static mut hl_dev_attrs: [*mut attribute; 20] = [
    &mut dev_attr_armcp_kernel_ver.attr, &mut dev_attr_armcp_ver.attr, &mut dev_attr_cpld_ver.attr,
    &mut dev_attr_cpucp_kernel_ver.attr, &mut dev_attr_cpucp_ver.attr, &mut dev_attr_device_type.attr,
    &mut dev_attr_fuse_ver.attr, &mut dev_attr_hard_reset.attr, &mut dev_attr_hard_reset_cnt.attr,
    &mut dev_attr_max_power.attr, &mut dev_attr_pci_addr.attr, &mut dev_attr_preboot_btl_ver.attr,
    &mut dev_attr_status.attr, &mut dev_attr_thermal_ver.attr, &mut dev_attr_uboot_ver.attr,
    &mut dev_attr_fw_os_ver.attr, &mut dev_attr_security_enabled.attr, &mut dev_attr_module_id.attr,
    &mut dev_attr_parent_device.attr, core::ptr::null_mut(),
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
