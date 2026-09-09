// SPDX-License-Identifier: GPL-2.0-only
/*
 * Coherency fabric (Aurora) support for Armada 370, 375, 38x and XP
 * platforms.
 *
 * Copyright (C) 2012 Marvell
 *
 * Yehuda Yitschak <yehuday@marvell.com>
 * Gregory Clement <gregory.clement@free-electrons.com>
 * Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
 *
 * The Armada 370, 375, 38x and XP SOCs have a coherency fabric which is
 * responsible for ensuring hardware coherency between all CPUs and between
 * CPUs and I/O masters. This file initializes the coherency fabric and
 * supplies basic routines for configuring and controlling hardware coherency
 */

// C dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct DeviceNode;
#[repr(C)]
pub struct Device;
#[repr(C)]
pub struct Property;
#[repr(C)]
pub struct Resource { pub start: usize }
#[repr(C)]
pub struct NotifierBlock { pub notifier_call: Option<unsafe extern "C" fn(*mut NotifierBlock, usize, *mut core::ffi::c_void) -> i32> }
#[repr(C)]
pub struct OfDeviceId { pub compatible: *const u8, pub data: *const core::ffi::c_void }

pub type PhysAddr = usize;
pub type U32 = u32;

pub static mut coherency_phys_base: usize = 0;
pub static mut coherency_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut coherency_cpu_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut cpu_config_base: *mut core::ffi::c_void = core::ptr::null_mut();

const IO_SYNC_BARRIER_CTL_OFFSET: usize = 0x0;

pub const COHERENCY_FABRIC_TYPE_NONE: i32 = 0;
pub const COHERENCY_FABRIC_TYPE_ARMADA_370_XP: i32 = 1;
pub const COHERENCY_FABRIC_TYPE_ARMADA_375: i32 = 2;
pub const COHERENCY_FABRIC_TYPE_ARMADA_380: i32 = 3;

static OF_COHERENCY_TABLE: [OfDeviceId; 4] = [
    OfDeviceId { compatible: b"marvell,coherency-fabric\0".as_ptr(), data: COHERENCY_FABRIC_TYPE_ARMADA_370_XP as usize as *const _ },
    OfDeviceId { compatible: b"marvell,armada-375-coherency-fabric\0".as_ptr(), data: COHERENCY_FABRIC_TYPE_ARMADA_375 as usize as *const _ },
    OfDeviceId { compatible: b"marvell,armada-380-coherency-fabric\0".as_ptr(), data: COHERENCY_FABRIC_TYPE_ARMADA_380 as usize as *const _ },
    OfDeviceId { compatible: core::ptr::null(), data: core::ptr::null() },
];

extern "C" {
    fn ll_enable_coherency() -> i32;
    fn ll_add_cpu_to_smp_group();
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn sync_cache_w(addr: *mut usize);
    fn of_iomap(np: *mut DeviceNode, index: i32) -> *mut core::ffi::c_void;
    fn of_address_to_resource(np: *mut DeviceNode, index: i32, res: *mut Resource) -> i32;
    fn of_find_compatible_node(from: *mut DeviceNode, typ: *const u8, compatible: *const u8) -> *mut DeviceNode;
    fn of_node_put(np: *mut DeviceNode);
    fn set_cpu_coherent();
    fn coherency_available() -> bool;
    fn is_smp() -> bool;
    fn of_find_matching_node_and_match(from: *mut DeviceNode, table: *const OfDeviceId, matched: *mut *const OfDeviceId) -> *mut DeviceNode;
    fn of_find_matching_node(from: *mut DeviceNode, table: *const OfDeviceId) -> *mut DeviceNode;
    fn dev_set_dma_coherent(dev: *mut Device);
    fn bus_register_notifier(bus: *mut core::ffi::c_void, nb: *mut NotifierBlock) -> i32;
    fn __arm_ioremap_caller(phys: PhysAddr, size: usize, mtype: u32, caller: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn pci_ioremap_set_mem_type(mtype: u32);
    fn cpuhp_setup_state_nocalls(state: i32, name: *const u8, startup: Option<unsafe extern "C" fn(u32) -> i32>, teardown: Option<unsafe extern "C" fn(u32) -> i32>) -> i32;
}

const CPU_CONFIG_SHARED_L2: u32 = 1 << 16;

unsafe fn armada_xp_clear_shared_l2() {
    if cpu_config_base.is_null() { return; }
    let mut reg = readl(cpu_config_base);
    reg &= !CPU_CONFIG_SHARED_L2;
    writel(reg, cpu_config_base);
}

unsafe extern "C" fn mvebu_hwcc_notifier(_nb: *mut NotifierBlock, event: usize, dev: *mut core::ffi::c_void) -> i32 {
    const BUS_NOTIFY_ADD_DEVICE: usize = 0x1;
    const NOTIFY_DONE: i32 = 0;
    const NOTIFY_OK: i32 = 1;
    if event != BUS_NOTIFY_ADD_DEVICE { return NOTIFY_DONE; }
    dev_set_dma_coherent(dev as *mut Device);
    NOTIFY_OK
}

static mut mvebu_hwcc_nb: NotifierBlock = NotifierBlock { notifier_call: Some(mvebu_hwcc_notifier) };
static mut mvebu_hwcc_pci_nb: NotifierBlock = NotifierBlock { notifier_call: Some(mvebu_hwcc_notifier) };

unsafe extern "C" fn armada_xp_clear_l2_starting(_cpu: u32) -> i32 {
    armada_xp_clear_shared_l2();
    0
}

unsafe fn armada_370_coherency_init(np: *mut DeviceNode) {
    let mut res = Resource { start: 0 };
    let cpu_config_np: *mut DeviceNode;
    of_address_to_resource(np, 0, &mut res);
    coherency_phys_base = res.start;
    sync_cache_w(&mut coherency_phys_base);
    coherency_base = of_iomap(np, 0);
    coherency_cpu_base = of_iomap(np, 1);
    cpu_config_np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"marvell,armada-xp-cpu-config\0".as_ptr());
    cpu_config_base = of_iomap(cpu_config_np, 0);
    of_node_put(cpu_config_np);
    if cpu_config_base.is_null() { set_cpu_coherent(); return; }
    cpuhp_setup_state_nocalls(0, b"arm/mvebu/coherency:starting\0".as_ptr(), Some(armada_xp_clear_l2_starting), None);
    set_cpu_coherent();
}

unsafe extern "C" fn armada_wa_ioremap_caller(phys_addr: PhysAddr, size: usize, _mtype: u32, caller: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    const MT_UNCACHED: u32 = 0;
    __arm_ioremap_caller(phys_addr, size, MT_UNCACHED, caller)
}

unsafe fn armada_375_380_coherency_init(np: *mut DeviceNode) {
    coherency_cpu_base = of_iomap(np, 0);
    pci_ioremap_set_mem_type(0);
    if !coherency_available() { return; }
    // for_each_compatible_node(cache_dn, NULL, "arm,pl310-cache") and the
    // property allocation/addition are supplied by the device-tree layer.
}

unsafe fn coherency_type() -> i32 {
    if !is_smp() { return COHERENCY_FABRIC_TYPE_NONE; }
    let mut matched: *const OfDeviceId = core::ptr::null();
    let np = of_find_matching_node_and_match(core::ptr::null_mut(), OF_COHERENCY_TABLE.as_ptr(), &mut matched);
    if np.is_null() { return COHERENCY_FABRIC_TYPE_NONE; }
    let ty = (*matched).data as usize as i32;
    of_node_put(np);
    ty
}

pub unsafe extern "C" fn set_cpu_coherent() -> i32 {
    let ty = coherency_type();
    if ty == COHERENCY_FABRIC_TYPE_ARMADA_370_XP {
        if coherency_base.is_null() { return 1; }
        armada_xp_clear_shared_l2();
        ll_add_cpu_to_smp_group();
        return ll_enable_coherency();
    }
    0
}

pub unsafe extern "C" fn coherency_available() -> bool { coherency_type() != COHERENCY_FABRIC_TYPE_NONE }

pub unsafe extern "C" fn coherency_init() -> i32 {
    let ty = coherency_type();
    let np = of_find_matching_node(core::ptr::null_mut(), OF_COHERENCY_TABLE.as_ptr());
    if ty == COHERENCY_FABRIC_TYPE_ARMADA_370_XP { armada_370_coherency_init(np); }
    else if ty == COHERENCY_FABRIC_TYPE_ARMADA_375 || ty == COHERENCY_FABRIC_TYPE_ARMADA_380 { armada_375_380_coherency_init(np); }
    of_node_put(np);
    0
}

unsafe extern "C" fn coherency_late_init() -> i32 {
    if coherency_available() { bus_register_notifier(core::ptr::null_mut(), &mut mvebu_hwcc_nb); }
    0
}

// postcore_initcall(coherency_late_init);

#[cfg(feature = "CONFIG_PCI")]
unsafe extern "C" fn coherency_pci_init() -> i32 {
    if coherency_available() { bus_register_notifier(core::ptr::null_mut(), &mut mvebu_hwcc_pci_nb); }
    0
}

// arch_initcall(coherency_pci_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
