// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Parts of this file are based on Ralink's 2.6.21 BSP
 *
 * Copyright (C) 2008 Imre Kaloz <kaloz@openwrt.org>
 * Copyright (C) 2008-2011 Gabor Juhos <juhosg@openwrt.org>
 * Copyright (C) 2013 John Crispin <john@phrozen.org>
 */

// C dependencies supplied by the surrounding kernel translation unit.

static mut soc_info_ptr: *mut ralink_soc_info = core::ptr::null_mut();

unsafe fn rt3883_get_soc_name0() -> u32
{
    __raw_readl(RT3883_SYSC_BASE + RT3883_SYSC_REG_CHIPID0_3)
}

unsafe fn rt3883_get_soc_name1() -> u32
{
    __raw_readl(RT3883_SYSC_BASE + RT3883_SYSC_REG_CHIPID4_7)
}

unsafe fn rt3883_soc_valid() -> bool
{
    if rt3883_get_soc_name0() == RT3883_CHIP_NAME0
        && rt3883_get_soc_name1() == RT3883_CHIP_NAME1
    {
        true
    } else {
        false
    }
}

unsafe fn rt3883_get_soc_name() -> *const core::ffi::c_char
{
    if rt3883_soc_valid() {
        b"RT3883\0".as_ptr() as *const core::ffi::c_char
    } else {
        b"invalid\0".as_ptr() as *const core::ffi::c_char
    }
}

unsafe fn rt3883_get_soc_id() -> u32
{
    __raw_readl(RT3883_SYSC_BASE + RT3883_SYSC_REG_REVID)
}

unsafe fn rt3883_get_soc_ver() -> u32
{
    (rt3883_get_soc_id() >> RT3883_REVID_VER_ID_SHIFT) & RT3883_REVID_VER_ID_MASK
}

unsafe fn rt3883_get_soc_rev() -> u32
{
    rt3883_get_soc_id() & RT3883_REVID_ECO_ID_MASK
}

unsafe fn rt3883_soc_dev_init() -> i32
{
    let mut soc_dev: *mut soc_device;
    let soc_dev_attr: *mut soc_device_attribute;

    soc_dev_attr = kzalloc_obj::<soc_device_attribute>();
    if soc_dev_attr.is_null() {
        return -ENOMEM;
    }

    (*soc_dev_attr).family = b"Ralink\0".as_ptr() as *const core::ffi::c_char;
    (*soc_dev_attr).soc_id = rt3883_get_soc_name();

    (*soc_dev_attr).data = soc_info_ptr as *const core::ffi::c_void;

    soc_dev = soc_device_register(soc_dev_attr);
    if IS_ERR(soc_dev) {
        kfree(soc_dev_attr as *mut core::ffi::c_void);
        return PTR_ERR(soc_dev);
    }

    0
}

device_initcall!(rt3883_soc_dev_init);

unsafe fn prom_soc_init(soc_info: *mut ralink_soc_info)
{
    if rt3883_soc_valid() {
        (*soc_info).compatible = b"ralink,rt3883-soc\0".as_ptr() as *const core::ffi::c_char;
    } else {
        panic!(
            "rt3883: unknown SoC, n0:{:08x} n1:{:08x}",
            rt3883_get_soc_name0(),
            rt3883_get_soc_name1()
        );
    }

    snprintf(
        (*soc_info).sys_type.as_mut_ptr(),
        RAMIPS_SYS_TYPE_LEN,
        b"Ralink %s ver:%u eco:%u\0".as_ptr() as *const core::ffi::c_char,
        rt3883_get_soc_name(),
        rt3883_get_soc_ver(),
        rt3883_get_soc_rev(),
    );

    (*soc_info).mem_base = RT3883_SDRAM_BASE;
    (*soc_info).mem_size_min = RT3883_MEM_SIZE_MIN;
    (*soc_info).mem_size_max = RT3883_MEM_SIZE_MAX;

    ralink_soc = RT3883_SOC;
    soc_info_ptr = soc_info;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
