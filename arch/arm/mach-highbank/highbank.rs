// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2010-2011 Calxeda, Inc.
 */

use core::ffi::c_void;

// Linux kernel dependencies supplied by other translation units.
extern "C" {
    static mut platform_bus_type: bus_type;
    static mut amba_bustype: bus_type;
    static mut psci_ops: psci_operations;

    fn ioremap(base: usize, size: usize) -> *mut c_void;
    fn irqchip_init();
    fn of_find_compatible_node(
        from: *mut device_node,
        type_: *const u8,
        compatible: *const u8,
    ) -> *mut device_node;
    fn of_device_is_compatible(np: *mut device_node, compatible: *const u8) -> bool;
    fn platform_get_resource(dev: *mut platform_device, type_: u32, num: u32) -> *mut resource;
    fn to_platform_device(dev: *mut device) -> *mut platform_device;
    fn of_property_read_bool(np: *mut device_node, propname: *const u8) -> bool;
    fn readl(addr: *mut u8) -> u32;
    fn writel(value: u32, addr: *mut u8);
    fn dev_set_dma_coherent(dev: *mut device);
    fn warn_once(condition: bool, format: *const u8, ...);
    fn highbank_smc1(op: u32, value: usize);
    fn highbank_set_pwr_shutdown();
    fn cpu_do_idle();
    fn orderly_poweroff(force: bool);
    fn ctrl_alt_del();
    fn register_platform_power_off(handler: unsafe extern "C" fn());
    fn highbank_pm_init();
    fn bus_register_notifier(bus: *mut bus_type, nb: *mut notifier_block) -> i32;
    fn pl320_ipc_register_notifier(nb: *mut notifier_block) -> i32;
    fn platform_device_register(dev: *mut platform_device) -> i32;
    fn of_iomap(np: *mut device_node, index: i32) -> *mut u8;
    fn highbank_restart();
}

#[repr(C)] pub struct bus_type { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct device { pub of_node: *mut device_node }
#[repr(C)] pub struct platform_device { pub dev: device, pub name: *const u8 }
#[repr(C)] pub struct resource { pub start: usize }
#[repr(C)] pub struct psci_operations { pub cpu_suspend: Option<unsafe extern "C" fn()> }

#[repr(C)]
pub struct notifier_block {
    pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, usize, *mut c_void) -> i32>,
}

pub static mut sregs_base: *mut u8 = core::ptr::null_mut();
pub static mut scu_base_addr: *mut c_void = core::ptr::null_mut();

const SZ_4K: usize = 4096;
const SZ_1G: u64 = 1u64 << 30;
const L2X0_CTRL: u32 = 1;
const BUS_NOTIFY_ADD_DEVICE: usize = 0x00000005;
const NOTIFY_DONE: i32 = 0;
const NOTIFY_OK: i32 = 1;
const KEY_POWER: u32 = 116;

unsafe extern "C" fn highbank_scu_map_io() {
    let base: usize;
    core::arch::asm!("mrc p15, 4, {0}, c15, c0, 0", out(reg) base);
    scu_base_addr = ioremap(base, SZ_4K);
}

unsafe extern "C" fn highbank_l2c310_write_sec(val: usize, reg: u32) {
    if reg == L2X0_CTRL {
        highbank_smc1(0x102, val);
    } else {
        warn_once(true, b"Highbank L2C310: ignoring write to reg 0x%x\n\0", reg);
    }
}

unsafe extern "C" fn highbank_init_irq() {
    irqchip_init();
    if !of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"arm,cortex-a9\0".as_ptr()).is_null() {
        highbank_scu_map_io();
    }
}

unsafe extern "C" fn highbank_power_off() {
    highbank_set_pwr_shutdown();
    loop { cpu_do_idle(); }
}

unsafe extern "C" fn highbank_platform_notifier(
    _nb: *mut notifier_block, event: usize, dev_ptr: *mut c_void,
) -> i32 {
    let mut reg: i32 = -1;
    let dev = dev_ptr as *mut device;
    if event != BUS_NOTIFY_ADD_DEVICE { return NOTIFY_DONE; }
    if of_device_is_compatible((*dev).of_node, b"calxeda,hb-ahci\0".as_ptr()) { reg = 0xc; }
    else if of_device_is_compatible((*dev).of_node, b"calxeda,hb-sdhci\0".as_ptr()) { reg = 0x18; }
    else if of_device_is_compatible((*dev).of_node, b"arm,pl330\0".as_ptr()) { reg = 0x20; }
    else if of_device_is_compatible((*dev).of_node, b"calxeda,hb-xgmac\0".as_ptr()) {
        let res = platform_get_resource(to_platform_device(dev), 0x00000200, 0);
        if !res.is_null() {
            if (*res).start == 0xfff50000 { reg = 0; }
            else if (*res).start == 0xfff51000 { reg = 4; }
        }
    }
    if reg < 0 { return NOTIFY_DONE; }
    if of_property_read_bool((*dev).of_node, b"dma-coherent\0".as_ptr()) {
        let addr = sregs_base.add(reg as usize);
        let val = readl(addr);
        writel(val | 0xff01, addr);
        dev_set_dma_coherent(dev);
    }
    NOTIFY_OK
}

static mut highbank_amba_nb: notifier_block = notifier_block { notifier_call: Some(highbank_platform_notifier) };
static mut highbank_platform_nb: notifier_block = notifier_block { notifier_call: Some(highbank_platform_notifier) };
static mut highbank_cpuidle_device: platform_device = platform_device { dev: device { of_node: core::ptr::null_mut() }, name: b"cpuidle-calxeda\0".as_ptr() };

unsafe extern "C" fn hb_keys_notifier(_nb: *mut notifier_block, event: usize, data: *mut c_void) -> i32 {
    let key = *(data as *mut u32);
    if event != 0x1000 { return 0; }
    if key == KEY_POWER { orderly_poweroff(false); }
    else if key == 0xffff { ctrl_alt_del(); }
    0
}
static mut hb_keys_nb: notifier_block = notifier_block { notifier_call: Some(hb_keys_notifier) };

unsafe extern "C" fn highbank_init() {
    let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"calxeda,hb-sregs\0".as_ptr());
    sregs_base = of_iomap(np, 0);
    warn_once(sregs_base.is_null(), b"sregs_base is null\0");
    register_platform_power_off(highbank_power_off);
    highbank_pm_init();
    bus_register_notifier(&mut platform_bus_type, &mut highbank_platform_nb);
    bus_register_notifier(&mut amba_bustype, &mut highbank_amba_nb);
    pl320_ipc_register_notifier(&mut hb_keys_nb);
    if (*core::ptr::addr_of!(psci_ops)).cpu_suspend.is_some() {
        platform_device_register(&mut highbank_cpuidle_device);
    }
}

pub static highbank_match: [*const u8; 3] = [b"calxeda,highbank\0".as_ptr(), b"calxeda,ecx-2000\0".as_ptr(), core::ptr::null()];

// DT_MACHINE_START(HIGHBANK, "Highbank")
// CONFIG_ZONE_DMA && CONFIG_ARM_LPAE: dma_zone_size = 4ULL * SZ_1G
// l2c_aux_val = 0; l2c_aux_mask = ~0; l2c_write_sec = highbank_l2c310_write_sec;
// init_irq = highbank_init_irq; init_machine = highbank_init; dt_compat = highbank_match;
// restart = highbank_restart; MACHINE_END

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
