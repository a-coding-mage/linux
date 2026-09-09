/*
 * Copyright 2019 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

// Linux and amdgpu dependencies are supplied by the surrounding translation unit.

const FRU_EEPROM_MADDR_6: u32 = 0x60000;
const FRU_EEPROM_MADDR_8: u32 = 0x80000;
const FRU_EEPROM_MADDR_INV: u32 = 0xFFFFF;

unsafe fn is_fru_eeprom_supported(adev: *mut amdgpu_device, fru_addr: *mut u32) -> bool {
    /* Only server cards have the FRU EEPROM. */
    let atom_ctx = (*(*adev).mode_info).atom_context;

    if amdgpu_sriov_vf(adev) || ((*adev).flags & AMD_IS_APU) != 0 {
        return false;
    }

    if !fru_addr.is_null() {
        *fru_addr = FRU_EEPROM_MADDR_8;
    }

    match amdgpu_ip_version(adev, MP1_HWIP, 0) {
        IP_VERSION(11, 0, 2) => match (*adev).asic_type {
            CHIP_VEGA20 => {
                if !atom_ctx.is_null()
                    && (strnstr((*atom_ctx).vbios_pn, b"D161\0".as_ptr(),
                                core::mem::size_of_val(&(*atom_ctx).vbios_pn)) != core::ptr::null_mut()
                        || strnstr((*atom_ctx).vbios_pn, b"D163\0".as_ptr(),
                                   core::mem::size_of_val(&(*atom_ctx).vbios_pn)) != core::ptr::null_mut())
                {
                    if !fru_addr.is_null() { *fru_addr = FRU_EEPROM_MADDR_6; }
                    true
                } else { false }
            }
            CHIP_ARCTURUS => false,
            _ => false,
        },
        IP_VERSION(11, 0, 7) => {
            if !atom_ctx.is_null() && strnstr((*atom_ctx).vbios_pn, b"D603\0".as_ptr(),
                    core::mem::size_of_val(&(*atom_ctx).vbios_pn)) != core::ptr::null_mut() {
                if strnstr((*atom_ctx).vbios_pn, b"D603GLXE\0".as_ptr(),
                        core::mem::size_of_val(&(*atom_ctx).vbios_pn)) != core::ptr::null_mut() { return false; }
                if !fru_addr.is_null() { *fru_addr = FRU_EEPROM_MADDR_6; }
                true
            } else { false }
        }
        IP_VERSION(13, 0, 2) => {
            if !atom_ctx.is_null() && strnstr((*atom_ctx).vbios_pn, b"D673\0".as_ptr(),
                    core::mem::size_of_val(&(*atom_ctx).vbios_pn)) == core::ptr::null_mut()
                && !fru_addr.is_null() { *fru_addr = FRU_EEPROM_MADDR_6; }
            true
        }
        IP_VERSION(13, 0, 6) | IP_VERSION(13, 0, 14) => {
            if !fru_addr.is_null() { *fru_addr = FRU_EEPROM_MADDR_8; } true
        }
        IP_VERSION(13, 0, 12) | IP_VERSION(15, 0, 8) => {
            if !fru_addr.is_null() { *fru_addr = FRU_EEPROM_MADDR_INV; } true
        }
        _ => false,
    }
}

unsafe fn fru_pia_advance(addr: *mut u32, pia: *const u8, len: i32) -> bool {
    if *addr >= len as u32 { return false; }
    *addr += 1 + ((*pia.add(*addr as usize) as u32) & 0x3f);
    true
}

unsafe fn fru_pia_copy_field(dst: *mut i8, dst_size: usize, pia: *const u8,
                             addr: u32, len: i32) -> bool {
    if addr + 1 >= len as u32 { return false; }
    let fl = core::cmp::min(core::cmp::min(((*pia.add(addr as usize) & 0x3f) as usize), dst_size - 1),
                            (len as u32 - addr - 1) as usize);
    core::ptr::copy_nonoverlapping(pia.add(addr as usize + 1), dst as *mut u8, fl);
    *dst.add(fl) = 0;
    true
}

pub unsafe fn amdgpu_fru_get_product_info(adev: *mut amdgpu_device) -> i32 {
    let mut buf = [0u8; 8];
    let mut fru_addr = 0u32;
    if !is_fru_eeprom_supported(adev, &mut fru_addr) || fru_addr == FRU_EEPROM_MADDR_INV { return 0; }
    if (*adev).fru_info.is_null() {
        (*adev).fru_info = kzalloc_obj::<amdgpu_fru_info>();
        if (*adev).fru_info.is_null() { return -ENOMEM; }
    }
    let fru_info = (*adev).fru_info;
    sprintf((*fru_info).serial.as_mut_ptr(), b"%llx\0".as_ptr(), (*adev).unique_id);
    if (*adev).pm.fru_eeprom_i2c_bus.is_null() || (*(*adev).pm.fru_eeprom_i2c_bus).algo.is_null() {
        dev_warn((*adev).dev, b"Cannot access FRU, EEPROM accessor not initialized\0".as_ptr());
        return -ENODEV;
    }
    let mut len = amdgpu_eeprom_read((*adev).pm.fru_eeprom_i2c_bus, fru_addr, buf.as_mut_ptr(), 8);
    if len != 8 { return if len < 0 { len } else { -EIO }; }
    if buf[0] != 1 { return -EIO; }
    let mut csum = 0u8; while len > 0 { len -= 1; csum = csum.wrapping_add(buf[len as usize]); }
    if csum != 0 { return -EIO; }
    let mut addr = (buf[4] as u32) * 8; if addr == 0 { return 0; } addr += fru_addr;
    len = amdgpu_eeprom_read((*adev).pm.fru_eeprom_i2c_bus, addr, buf.as_mut_ptr(), 3);
    if len != 3 || buf[0] != 1 { return if len < 0 { len } else { -EIO }; }
    let size = (buf[1] as usize) * 8;
    let pia = kzalloc(size, GFP_KERNEL); if pia.is_null() { return -ENOMEM; }
    len = amdgpu_eeprom_read((*adev).pm.fru_eeprom_i2c_bus, addr, pia, size);
    if len != size as i32 { kfree(pia); return if len < 0 { len } else { -EIO }; }
    csum = 0; let mut n = size; while n > 0 { n -= 1; csum = csum.wrapping_add(*pia.add(n)); }
    if csum != 0 { kfree(pia); return -EIO; }
    addr = 3;
    if !fru_pia_copy_field((*fru_info).manufacturer_name.as_mut_ptr(), core::mem::size_of_val(&(*fru_info).manufacturer_name), pia, addr, len) { kfree(pia); return 0; }
    if !fru_pia_advance(&mut addr, pia, len) || !fru_pia_copy_field((*fru_info).product_name.as_mut_ptr(), core::mem::size_of_val(&(*fru_info).product_name), pia, addr, len) { kfree(pia); return 0; }
    if !fru_pia_advance(&mut addr, pia, len) || !fru_pia_copy_field((*fru_info).product_number.as_mut_ptr(), core::mem::size_of_val(&(*fru_info).product_number), pia, addr, len) { kfree(pia); return 0; }
    if !fru_pia_advance(&mut addr, pia, len) || !fru_pia_advance(&mut addr, pia, len) || !fru_pia_copy_field((*fru_info).serial.as_mut_ptr(), core::mem::size_of_val(&(*fru_info).serial), pia, addr, len) { kfree(pia); return 0; }
    if !fru_pia_advance(&mut addr, pia, len) || !fru_pia_advance(&mut addr, pia, len) || !fru_pia_copy_field((*fru_info).fru_id.as_mut_ptr(), core::mem::size_of_val(&(*fru_info).fru_id), pia, addr, len) { kfree(pia); return 0; }
    kfree(pia); 0
}

// The following sysfs callbacks and attribute declarations preserve the C sysfs interface.
unsafe fn amdgpu_fru_product_name_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> isize {
    let adev = drm_to_adev(dev_get_drvdata(dev)); sysfs_emit(buf, b"%s\n\0".as_ptr(), (*(*adev).fru_info).product_name.as_ptr())
}
unsafe fn amdgpu_fru_product_number_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> isize { let adev = drm_to_adev(dev_get_drvdata(dev)); sysfs_emit(buf, b"%s\n\0".as_ptr(), (*(*adev).fru_info).product_number.as_ptr()) }
unsafe fn amdgpu_fru_serial_number_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> isize { let adev = drm_to_adev(dev_get_drvdata(dev)); sysfs_emit(buf, b"%s\n\0".as_ptr(), (*(*adev).fru_info).serial.as_ptr()) }
unsafe fn amdgpu_fru_id_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> isize { let adev = drm_to_adev(dev_get_drvdata(dev)); sysfs_emit(buf, b"%s\n\0".as_ptr(), (*(*adev).fru_info).fru_id.as_ptr()) }
unsafe fn amdgpu_fru_manufacturer_name_show(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> isize { let adev = drm_to_adev(dev_get_drvdata(dev)); sysfs_emit(buf, b"%s\n\0".as_ptr(), (*(*adev).fru_info).manufacturer_name.as_ptr()) }

extern "C" {
    static dev_attr_product_name: device_attribute;
    static dev_attr_product_number: device_attribute;
    static dev_attr_serial_number: device_attribute;
    static dev_attr_fru_id: device_attribute;
    static dev_attr_manufacturer: device_attribute;
}

static amdgpu_fru_attributes: [*const attribute; 6] = [
    unsafe { &(*(&dev_attr_product_name as *const _)).attr }, unsafe { &(*(&dev_attr_product_number as *const _)).attr },
    unsafe { &(*(&dev_attr_serial_number as *const _)).attr }, unsafe { &(*(&dev_attr_fru_id as *const _)).attr },
    unsafe { &(*(&dev_attr_manufacturer as *const _)).attr }, core::ptr::null(),
];

pub unsafe fn amdgpu_fru_sysfs_init(adev: *mut amdgpu_device) -> i32 {
    if !is_fru_eeprom_supported(adev, core::ptr::null_mut()) || (*adev).fru_info.is_null() { return 0; }
    sysfs_create_files(&mut (*(*adev).dev).kobj, amdgpu_fru_attributes.as_ptr())
}

pub unsafe fn amdgpu_fru_sysfs_fini(adev: *mut amdgpu_device) {
    if (*adev).fru_info.is_null() { return; }
    sysfs_remove_files(&mut (*(*adev).dev).kobj, amdgpu_fru_attributes.as_ptr());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
