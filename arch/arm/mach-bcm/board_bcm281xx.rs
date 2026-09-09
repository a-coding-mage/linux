// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2012-2014 Broadcom Corporation

// Dependencies supplied by the surrounding kernel translation are referenced
// here but are not implemented in this file.

const SECWDOG_OFFSET: usize = 0x00000000;
const SECWDOG_RESERVED_MASK: u32 = 0xe2000000;
const SECWDOG_WD_LOAD_FLAG_MASK: u32 = 0x10000000;
const SECWDOG_EN_MASK: u32 = 0x08000000;
const SECWDOG_SRSTEN_MASK: u32 = 0x04000000;
const SECWDOG_CLKS_SHIFT: u32 = 20;
const SECWDOG_COUNT_SHIFT: u32 = 0;

extern "C" {
    fn of_find_compatible_node(
        from: *mut device_node,
        ty: *mut core::ffi::c_void,
        compatible: *const core::ffi::c_char,
    ) -> *mut device_node;
    fn of_iomap(np: *mut device_node, index: i32) -> *mut u8;
    fn of_node_put(np: *mut device_node);
    fn readl(addr: *const u8) -> u32;
    fn writel(value: u32, addr: *mut u8);
    fn kona_l2_cache_init();
    fn pr_emerg(fmt: *const core::ffi::c_char, ...);
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum reboot_mode {
    _Invalid = -1,
}

unsafe fn bcm281xx_restart(_mode: reboot_mode, _cmd: *const core::ffi::c_char) {
    let mut val: u32;
    let mut base: *mut u8;
    let np_wdog: *mut device_node;

    np_wdog = of_find_compatible_node(
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        b"brcm,kona-wdt\0".as_ptr() as *const core::ffi::c_char,
    );
    if np_wdog.is_null() {
        pr_emerg(b"Couldn't find brcm,kona-wdt\n\0".as_ptr() as *const core::ffi::c_char);
        return;
    }
    base = of_iomap(np_wdog, 0);
    of_node_put(np_wdog);
    if base.is_null() {
        pr_emerg(b"Couldn't map brcm,kona-wdt\n\0".as_ptr() as *const core::ffi::c_char);
        return;
    }

    /* Enable watchdog with short timeout (244us). */
    val = readl(base.add(SECWDOG_OFFSET));
    val &= SECWDOG_RESERVED_MASK | SECWDOG_WD_LOAD_FLAG_MASK;
    val |= SECWDOG_EN_MASK
        | SECWDOG_SRSTEN_MASK
        | (0x15u32 << SECWDOG_CLKS_SHIFT)
        | (0x8u32 << SECWDOG_COUNT_SHIFT);
    writel(val, base.add(SECWDOG_OFFSET));

    /* Wait for reset */
    loop {}
}

unsafe fn bcm281xx_init() {
    kona_l2_cache_init();
}

static BCM281XX_DT_COMPAT: [Option<&'static core::ffi::CStr>; 2] = [
    Some(unsafe { core::ffi::CStr::from_bytes_with_nul_unchecked(b"brcm,bcm11351\0") }),
    None,
];

// Equivalent of DT_MACHINE_START(BCM281XX_DT, "BCM281xx Broadcom Application Processor")
// .init_machine = bcm281xx_init, .restart = bcm281xx_restart,
// .dt_compat = bcm281xx_dt_compat, MACHINE_END
#[repr(C)]
pub struct MachineDesc {
    pub name: &'static str,
    pub init_machine: unsafe fn(),
    pub restart: unsafe fn(reboot_mode, *const core::ffi::c_char),
    pub dt_compat: &'static [Option<&'static core::ffi::CStr>; 2],
}

#[no_mangle]
pub static BCM281XX_DT: MachineDesc = MachineDesc {
    name: "BCM281xx Broadcom Application Processor",
    init_machine: bcm281xx_init,
    restart: bcm281xx_restart,
    dt_compat: &BCM281XX_DT_COMPAT,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
