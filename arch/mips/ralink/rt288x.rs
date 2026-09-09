// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Parts of this file are based on Ralink's 2.6.21 BSP
 *
 * Copyright (C) 2008-2011 Gabor Juhos <juhosg@openwrt.org>
 * Copyright (C) 2008 Imre Kaloz <kaloz@openwrt.org>
 * Copyright (C) 2013 John Crispin <john@phrozen.org>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

// Kernel and Ralink declarations supplied by the surrounding translation unit.
extern "C" {
    static mut ralink_soc: c_uint;
    fn __raw_readl(addr: usize) -> c_uint;
    fn soc_device_register(attr: *mut soc_device_attribute) -> *mut soc_device;
    fn kfree(ptr: *mut c_void);
    fn PTR_ERR(ptr: *mut soc_device) -> c_int;
    fn panic(fmt: *const c_char, ...) -> !;
    fn snprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ... ) -> c_int;
}

#[repr(C)]
pub struct ralink_soc_info {
    pub compatible: *const c_char,
    pub sys_type: [c_char; RAMIPS_SYS_TYPE_LEN],
    pub mem_base: usize,
    pub mem_size_min: usize,
    pub mem_size_max: usize,
}

#[repr(C)]
pub struct soc_device;

#[repr(C)]
pub struct soc_device_attribute {
    pub family: *const c_char,
    pub soc_id: *const c_char,
    pub data: *mut ralink_soc_info,
}

extern "C" {
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
}

static mut soc_info_ptr: *mut ralink_soc_info = core::ptr::null_mut();

unsafe fn rt2880_get_soc_name0() -> c_uint {
    __raw_readl(RT2880_SYSC_BASE as usize + SYSC_REG_CHIP_NAME0)
}

unsafe fn rt2880_get_soc_name1() -> c_uint {
    __raw_readl(RT2880_SYSC_BASE as usize + SYSC_REG_CHIP_NAME1)
}

unsafe fn rt2880_soc_valid() -> bool {
    if rt2880_get_soc_name0() == RT2880_CHIP_NAME0 &&
       rt2880_get_soc_name1() == RT2880_CHIP_NAME1 {
        true
    } else {
        false
    }
}

unsafe fn rt2880_get_soc_name() -> *const c_char {
    if rt2880_soc_valid() {
        b"RT2880\0".as_ptr() as *const c_char
    } else {
        b"invalid\0".as_ptr() as *const c_char
    }
}

unsafe fn rt2880_get_soc_id() -> c_uint {
    __raw_readl(RT2880_SYSC_BASE as usize + SYSC_REG_CHIP_ID)
}

unsafe fn rt2880_get_soc_ver() -> c_uint {
    (rt2880_get_soc_id() >> CHIP_ID_ID_SHIFT) & CHIP_ID_ID_MASK
}

unsafe fn rt2880_get_soc_rev() -> c_uint {
    rt2880_get_soc_id() & CHIP_ID_REV_MASK
}

unsafe fn rt2880_soc_dev_init() -> c_int {
    let soc_dev_attr = kzalloc(core::mem::size_of::<soc_device_attribute>(), 0)
        as *mut soc_device_attribute;
    if soc_dev_attr.is_null() {
        return -12; // -ENOMEM
    }

    (*soc_dev_attr).family = b"Ralink\0".as_ptr() as *const c_char;
    (*soc_dev_attr).soc_id = rt2880_get_soc_name();
    (*soc_dev_attr).data = soc_info_ptr;

    let soc_dev = soc_device_register(soc_dev_attr);
    if (soc_dev as isize) < 0 && (soc_dev as isize) >= -4095 {
        kfree(soc_dev_attr as *mut c_void);
        return PTR_ERR(soc_dev);
    }
    0
}

// device_initcall(rt2880_soc_dev_init);

pub unsafe fn prom_soc_init(soc_info: *mut ralink_soc_info) {
    if rt2880_soc_valid() {
        (*soc_info).compatible = b"ralink,r2880-soc\0".as_ptr() as *const c_char;
    } else {
        panic(
            b"rt288x: unknown SoC, n0:%08x n1:%08x\0".as_ptr() as *const c_char,
            rt2880_get_soc_name0(), rt2880_get_soc_name1(),
        );
    }

    snprintf(
        (*soc_info).sys_type.as_mut_ptr(), RAMIPS_SYS_TYPE_LEN,
        b"Ralink %s id:%u rev:%u\0".as_ptr() as *const c_char,
        rt2880_get_soc_name(), rt2880_get_soc_ver(), rt2880_get_soc_rev(),
    );

    (*soc_info).mem_base = RT2880_SDRAM_BASE as usize;
    (*soc_info).mem_size_min = RT2880_MEM_SIZE_MIN as usize;
    (*soc_info).mem_size_max = RT2880_MEM_SIZE_MAX as usize;

    ralink_soc = RT2880_SOC;
    soc_info_ptr = soc_info;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
