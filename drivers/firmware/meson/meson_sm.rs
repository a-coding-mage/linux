// SPDX-License-Identifier: GPL-2.0-only
/*
 * Amlogic Secure Monitor driver
 *
 * Copyright (C) 2016 Endless Mobile, Inc.
 * Author: Carlo Caione <carlo@endlessm.com>
 */

// Dependencies supplied by the surrounding kernel/Rust environment are intentionally
// left as external names, matching the original include dependencies.

#[repr(C)]
pub struct meson_sm_cmd {
    pub index: u32,
    pub smc_id: u32,
}

#[repr(C)]
pub struct meson_sm_chip {
    pub shmem_size: usize,
    pub cmd_shmem_in_base: u32,
    pub cmd_shmem_out_base: u32,
    pub cmd: *const meson_sm_cmd,
}

static GXBB_CMDS: [meson_sm_cmd; 8] = [
    meson_sm_cmd { index: SM_EFUSE_READ, smc_id: 0x82000030 },
    meson_sm_cmd { index: SM_EFUSE_WRITE, smc_id: 0x82000031 },
    meson_sm_cmd { index: SM_EFUSE_USER_MAX, smc_id: 0x82000033 },
    meson_sm_cmd { index: SM_GET_CHIP_ID, smc_id: 0x82000044 },
    meson_sm_cmd { index: SM_THERMAL_CALIB_READ, smc_id: 0x82000047 },
    meson_sm_cmd { index: SM_A1_PWRC_SET, smc_id: 0x82000093 },
    meson_sm_cmd { index: SM_A1_PWRC_GET, smc_id: 0x82000095 },
    meson_sm_cmd { index: 0, smc_id: 0 },
];

static GXBB_CHIP: meson_sm_chip = meson_sm_chip {
    shmem_size: SZ_4K,
    cmd_shmem_in_base: 0x82000020,
    cmd_shmem_out_base: 0x82000021,
    cmd: GXBB_CMDS.as_ptr(),
};

#[repr(C)]
pub struct meson_sm_firmware {
    pub chip: *const meson_sm_chip,
    pub sm_shmem_in_base: *mut core::ffi::c_void,
    pub sm_shmem_out_base: *mut core::ffi::c_void,
}

unsafe fn meson_sm_get_cmd(chip: *const meson_sm_chip, cmd_index: u32) -> u32 {
    let mut cmd = (*chip).cmd;
    while (*cmd).smc_id != 0 && (*cmd).index != cmd_index {
        cmd = cmd.add(1);
    }
    (*cmd).smc_id
}

unsafe fn __meson_sm_call(cmd: u32, arg0: u32, arg1: u32, arg2: u32,
                          arg3: u32, arg4: u32) -> i32 {
    let mut res = arm_smccc_res { a0: 0 };
    arm_smccc_smc(cmd, arg0, arg1, arg2, arg3, arg4, 0, 0, &mut res);
    res.a0 as i32
}

unsafe fn meson_sm_map_shmem(cmd_shmem: u32, size: usize) -> *mut core::ffi::c_void {
    let sm_phy_base = __meson_sm_call(cmd_shmem, 0, 0, 0, 0, 0) as u32;
    if sm_phy_base == 0 { return core::ptr::null_mut(); }
    ioremap_cache(sm_phy_base, size)
}

pub unsafe fn meson_sm_call(fw: *mut meson_sm_firmware, cmd_index: u32,
                             ret: *mut i32, arg0: u32, arg1: u32, arg2: u32,
                             arg3: u32, arg4: u32) -> i32 {
    if (*fw).chip.is_null() { return -ENOENT; }
    let cmd = meson_sm_get_cmd((*fw).chip, cmd_index);
    if cmd == 0 { return -EINVAL; }
    let lret = __meson_sm_call(cmd, arg0, arg1, arg2, arg3, arg4);
    if !ret.is_null() { *ret = lret; }
    0
}

pub unsafe fn meson_sm_call_read(fw: *mut meson_sm_firmware, buffer: *mut core::ffi::c_void,
                                 bsize: u32, cmd_index: u32, arg0: u32, arg1: u32,
                                 arg2: u32, arg3: u32, arg4: u32) -> i32 {
    if (*fw).chip.is_null() { return -ENOENT; }
    if (*(*fw).chip).cmd_shmem_out_base == 0 || bsize as usize > (*(*fw).chip).shmem_size { return -EINVAL; }
    let mut size = 0i32;
    if meson_sm_call(fw, cmd_index, &mut size, arg0, arg1, arg2, arg3, arg4) < 0 { return -EINVAL; }
    if size < 0 || size as u32 > bsize { return -EINVAL; }
    let ret = size;
    let copy_size = if size == 0 { bsize } else { size as u32 } as usize;
    if !buffer.is_null() { memcpy(buffer, (*fw).sm_shmem_out_base, copy_size); }
    ret
}

pub unsafe fn meson_sm_call_write(fw: *mut meson_sm_firmware, buffer: *mut core::ffi::c_void,
                                  size: u32, cmd_index: u32, arg0: u32, arg1: u32,
                                  arg2: u32, arg3: u32, arg4: u32) -> i32 {
    if (*fw).chip.is_null() { return -ENOENT; }
    if size as usize > (*(*fw).chip).shmem_size || (*(*fw).chip).cmd_shmem_in_base == 0 { return -EINVAL; }
    memcpy((*fw).sm_shmem_in_base, buffer, size as usize);
    let mut written = 0i32;
    if meson_sm_call(fw, cmd_index, &mut written, arg0, arg1, arg2, arg3, arg4) < 0 { return -EINVAL; }
    if written <= 0 || written as u32 > size { return -EINVAL; }
    written
}

pub unsafe fn meson_sm_get(sm_node: *mut device_node) -> *mut meson_sm_firmware {
    let pdev = of_find_device_by_node(sm_node);
    if pdev.is_null() { return core::ptr::null_mut(); }
    let fw = platform_get_drvdata(pdev);
    put_device(pdev);
    fw
}

pub unsafe fn meson_sm_get_thermal_calib(fw: *mut meson_sm_firmware, trim_info: *mut u32,
                                         tsensor_id: u32) -> i32 {
    meson_sm_call(fw, SM_THERMAL_CALIB_READ, trim_info as *mut i32, tsensor_id, 0, 0, 0, 0)
}

const SM_CHIP_ID_LENGTH: usize = 119;
const SM_CHIP_ID_OFFSET: usize = 4;
const SM_CHIP_ID_SIZE: usize = 12;

unsafe fn serial_show(dev: *mut device) -> isize {
    let pdev = to_platform_device(dev);
    let fw = platform_get_drvdata(pdev);
    let id_buf = kmalloc(SM_CHIP_ID_LENGTH);
    if id_buf.is_null() { return -ENOMEM as isize; }
    let ret = meson_sm_call_read(fw, id_buf as *mut _, SM_CHIP_ID_LENGTH as u32,
                                 SM_GET_CHIP_ID, 0, 0, 0, 0, 0);
    if ret < 0 { kfree(id_buf); return ret as isize; }
    let shown = sprintf_serial(dev, id_buf.add(SM_CHIP_ID_OFFSET), SM_CHIP_ID_SIZE);
    kfree(id_buf);
    shown
}

unsafe fn meson_sm_probe(pdev: *mut platform_device) -> i32 {
    let dev = platform_device_dev(pdev);
    let fw = devm_kzalloc(dev, core::mem::size_of::<meson_sm_firmware>());
    if fw.is_null() { return -ENOMEM; }
    let chip = device_get_match_data(dev);
    if chip.is_null() { return -EINVAL; }
    if (*chip).cmd_shmem_in_base != 0 {
        (*fw).sm_shmem_in_base = meson_sm_map_shmem((*chip).cmd_shmem_in_base, (*chip).shmem_size);
        if (*fw).sm_shmem_in_base.is_null() { return -EINVAL; }
    }
    if (*chip).cmd_shmem_out_base != 0 {
        (*fw).sm_shmem_out_base = meson_sm_map_shmem((*chip).cmd_shmem_out_base, (*chip).shmem_size);
        if (*fw).sm_shmem_out_base.is_null() {
            iounmap((*fw).sm_shmem_in_base);
            return -EINVAL;
        }
    }
    (*fw).chip = chip;
    platform_set_drvdata(pdev, fw);
    if devm_of_platform_populate(dev) != 0 {
        iounmap((*fw).sm_shmem_out_base);
        iounmap((*fw).sm_shmem_in_base);
        return -EINVAL;
    }
    pr_info_secure_monitor_enabled();
    0
}

// The remaining sysfs and platform-driver registration declarations retain the
// original interfaces and are supplied by the surrounding kernel bindings.
extern "C" {
    fn arm_smccc_smc(cmd: u32, a0: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32, res: *mut arm_smccc_res);
    fn ioremap_cache(addr: u32, size: usize) -> *mut core::ffi::c_void;
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize);
    fn of_find_device_by_node(node: *mut device_node) -> *mut platform_device;
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut meson_sm_firmware;
    fn put_device(pdev: *mut platform_device);
    fn to_platform_device(dev: *mut device) -> *mut platform_device;
    fn kmalloc(size: usize) -> *mut u8;
    fn kfree(ptr: *mut u8);
    fn sprintf_serial(dev: *mut device, id: *mut u8, size: usize) -> isize;
    fn platform_device_dev(pdev: *mut platform_device) -> *mut device;
    fn devm_kzalloc(dev: *mut device, size: usize) -> *mut meson_sm_firmware;
    fn device_get_match_data(dev: *mut device) -> *const meson_sm_chip;
    fn platform_set_drvdata(pdev: *mut platform_device, fw: *mut meson_sm_firmware);
    fn devm_of_platform_populate(dev: *mut device) -> i32;
    fn iounmap(addr: *mut core::ffi::c_void);
    fn pr_info_secure_monitor_enabled();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
