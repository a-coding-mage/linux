// SPDX-License-Identifier: GPL-2.0-only
/*
 * Broadcom Brahma-B15 CPU read-ahead cache management functions
 *
 * Copyright (C) 2015-2016 Broadcom
 */

// Linux and ARM dependencies supplied by the surrounding kernel translation.

extern "C" {
    fn v7_flush_kern_cache_all();
}

const RAC_CONFIG0_REG: usize = 0x78;
const RACENPREF_MASK: u32 = 0x3;
const RACPREFINST_SHIFT: u32 = 0;
const RACENINST_SHIFT: u32 = 2;
const RACPREFDATA_SHIFT: u32 = 4;
const RACENDATA_SHIFT: u32 = 6;
const RAC_CPU_SHIFT: u32 = 8;
const RACCFG_MASK: u32 = 0xff;
const RAC_CONFIG1_REG: usize = 0x7c;
const B15_RAC_FLUSH_REG: u32 = 0x80;
const B53_RAC_FLUSH_REG: u32 = 0x84;
const FLUSH_RAC: u32 = 1 << 0;
const RAC_DATA_INST_EN_MASK: u32 =
    (1 << RACPREFINST_SHIFT) |
    (RACENPREF_MASK << RACENINST_SHIFT) |
    (1 << RACPREFDATA_SHIFT) |
    (RACENPREF_MASK << RACENDATA_SHIFT);
const RAC_ENABLED: usize = 0;
const RAC_SUSPENDED: usize = 1;

static mut b15_rac_base: *mut u8 = core::ptr::null_mut();
// DEFINE_SPINLOCK(rac_lock), supplied by the kernel environment.
static mut rac_lock: usize = 0;
static mut rac_config0_reg: u32 = 0;
static mut rac_flush_offset: u32 = 0;
static mut b15_rac_flags: usize = 0;

extern "C" {
    fn __raw_readl(addr: *mut u8) -> u32;
    fn __raw_writel(value: u32, addr: *mut u8);
    fn dmb();
    fn dsb();
    fn spin_lock(lock: *mut usize);
    fn spin_unlock(lock: *mut usize);
    fn test_bit(bit: usize, addr: *const usize) -> bool;
    fn set_bit(bit: usize, addr: *mut usize);
    fn clear_bit(bit: usize, addr: *mut usize);
    fn __set_bit(bit: usize, addr: *mut usize);
    fn __clear_bit(bit: usize, addr: *mut usize);
}

unsafe fn __b15_rac_disable() -> u32 {
    let val = __raw_readl(b15_rac_base.add(RAC_CONFIG0_REG));
    __raw_writel(0, b15_rac_base.add(RAC_CONFIG0_REG));
    dmb();
    val
}

unsafe fn __b15_rac_flush() {
    let mut reg: u32;
    __raw_writel(FLUSH_RAC, b15_rac_base.add(rac_flush_offset as usize));
    loop {
        // Required to force outstanding writes to be cleaned and insert an idle cycle.
        dmb();
        reg = __raw_readl(b15_rac_base.add(rac_flush_offset as usize));
        if reg & FLUSH_RAC == 0 { break; }
    }
}

unsafe fn b15_rac_disable_and_flush() -> u32 {
    let reg = __b15_rac_disable();
    __b15_rac_flush();
    reg
}

unsafe fn __b15_rac_enable(val: u32) {
    __raw_writel(val, b15_rac_base.add(RAC_CONFIG0_REG));
    dsb();
}

pub unsafe fn b15_flush_kern_cache_all() {
    let mut do_flush: u32;
    let mut val: u32 = 0;
    if test_bit(RAC_SUSPENDED, &b15_rac_flags) {
        v7_flush_kern_cache_all();
        return;
    }
    spin_lock(&mut rac_lock);
    do_flush = test_bit(RAC_ENABLED, &b15_rac_flags) as u32;
    if do_flush != 0 { val = b15_rac_disable_and_flush(); }
    v7_flush_kern_cache_all();
    if do_flush == 0 { /* nobarrier */ } else { __b15_rac_enable(val); }
    spin_unlock(&mut rac_lock);
}

unsafe fn b15_rac_enable() {
    let mut enable: u32 = 0;
    // for_each_possible_cpu(cpu)
    for cpu in 0..num_possible_cpus() {
        enable |= RAC_DATA_INST_EN_MASK << (cpu * RAC_CPU_SHIFT);
    }
    b15_rac_disable_and_flush();
    __b15_rac_enable(enable);
}

unsafe fn b15_rac_reboot_notifier(_nb: *mut notifier_block, action: usize, _data: *mut core::ffi::c_void) -> i32 {
    if action == SYS_RESTART {
        spin_lock(&mut rac_lock);
        b15_rac_disable_and_flush();
        clear_bit(RAC_ENABLED, &mut b15_rac_flags);
        set_bit(RAC_SUSPENDED, &mut b15_rac_flags);
        spin_unlock(&mut rac_lock);
    }
    NOTIFY_DONE
}

#[repr(C)]
struct notifier_block { notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, usize, *mut core::ffi::c_void) -> i32> }
static mut b15_rac_reboot_nb: notifier_block = notifier_block { notifier_call: Some(b15_rac_reboot_notifier) };

unsafe fn b15_rac_dying_cpu(_cpu: u32) -> i32 {
    if test_bit(RAC_SUSPENDED, &b15_rac_flags) { return 0; }
    spin_lock(&mut rac_lock);
    __clear_bit(RAC_ENABLED, &mut b15_rac_flags);
    rac_config0_reg = b15_rac_disable_and_flush();
    spin_unlock(&mut rac_lock);
    0
}

unsafe fn b15_rac_dead_cpu(_cpu: u32) -> i32 {
    if test_bit(RAC_SUSPENDED, &b15_rac_flags) { return 0; }
    spin_lock(&mut rac_lock);
    __b15_rac_enable(rac_config0_reg);
    __set_bit(RAC_ENABLED, &mut b15_rac_flags);
    spin_unlock(&mut rac_lock);
    0
}

unsafe fn b15_rac_suspend(_data: *mut core::ffi::c_void) -> i32 {
    rac_config0_reg = b15_rac_disable_and_flush();
    set_bit(RAC_SUSPENDED, &mut b15_rac_flags);
    0
}

unsafe fn b15_rac_resume(_data: *mut core::ffi::c_void) {
    __b15_rac_enable(rac_config0_reg);
    clear_bit(RAC_SUSPENDED, &mut b15_rac_flags);
}

#[repr(C)]
struct syscore_ops { suspend: Option<unsafe fn(*mut core::ffi::c_void) -> i32>, resume: Option<unsafe fn(*mut core::ffi::c_void)> }
#[repr(C)]
struct syscore { ops: *const syscore_ops }
static b15_rac_syscore_ops: syscore_ops = syscore_ops { suspend: Some(b15_rac_suspend), resume: Some(b15_rac_resume) };
static b15_rac_syscore: syscore = syscore { ops: &b15_rac_syscore_ops };

// Initialization and platform registration are provided by the kernel integration.
extern "C" {
    fn num_possible_cpus() -> u32;
    fn register_reboot_notifier(nb: *mut notifier_block) -> i32;
    fn unregister_reboot_notifier(nb: *mut notifier_block);
    fn register_syscore(syscore: *const syscore);
}

const SYS_RESTART: usize = 0x01234567;
const NOTIFY_DONE: i32 = 0;

// The following platform helpers correspond to the Linux device-tree, CPU-hotplug,
// and initcall interfaces used by the original implementation.
#[repr(C)] struct device_node;
extern "C" {
    fn of_find_compatible_node(from: *mut device_node, typ: *const u8, compat: *const u8) -> *mut device_node;
    fn of_iomap(node: *mut device_node, index: i32) -> *mut u8;
    fn of_get_cpu_node(cpu: u32, thread: *mut u32) -> *mut device_node;
    fn of_device_is_compatible(node: *mut device_node, compat: *const u8) -> bool;
    fn of_node_put(node: *mut device_node);
    fn iounmap(addr: *mut u8);
    fn cpuhp_setup_state_nocalls(state: i32, name: *const u8, startup: Option<unsafe fn(u32) -> i32>, teardown: Option<unsafe fn(u32) -> i32>) -> i32;
    fn cpuhp_remove_state_nocalls(state: i32);
}
const ENODEV: i32 = -19;
const ENOMEM: i32 = -12;
const EINVAL: i32 = -22;
const CPUHP_AP_ARM_CACHE_B15_RAC_DEAD: i32 = 0;
const CPUHP_AP_ARM_CACHE_B15_RAC_DYING: i32 = 1;

#[no_mangle]
pub unsafe fn b15_rac_init() -> i32 {
    let dn = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"brcm,brcmstb-cpu-biu-ctrl\0".as_ptr());
    if dn.is_null() { return ENODEV; }
    if num_possible_cpus() > 4 { of_node_put(dn); return 0; }
    b15_rac_base = of_iomap(dn, 0);
    if b15_rac_base.is_null() { of_node_put(dn); return ENOMEM; }
    let cpu_dn = of_get_cpu_node(0, core::ptr::null_mut());
    if cpu_dn.is_null() { of_node_put(dn); return ENODEV; }
    if of_device_is_compatible(cpu_dn, b"brcm,brahma-b15\0".as_ptr()) { rac_flush_offset = B15_RAC_FLUSH_REG; }
    else if of_device_is_compatible(cpu_dn, b"brcm,brahma-b53\0".as_ptr()) { rac_flush_offset = B53_RAC_FLUSH_REG; }
    else { of_node_put(cpu_dn); of_node_put(dn); return EINVAL; }
    of_node_put(cpu_dn);
    let mut ret = register_reboot_notifier(&mut b15_rac_reboot_nb);
    if ret != 0 { iounmap(b15_rac_base); of_node_put(dn); return ret; }
    ret = cpuhp_setup_state_nocalls(CPUHP_AP_ARM_CACHE_B15_RAC_DEAD, b"arm/cache-b15-rac:dead\0".as_ptr(), None, Some(b15_rac_dead_cpu));
    if ret != 0 { unregister_reboot_notifier(&mut b15_rac_reboot_nb); iounmap(b15_rac_base); of_node_put(dn); return ret; }
    ret = cpuhp_setup_state_nocalls(CPUHP_AP_ARM_CACHE_B15_RAC_DYING, b"arm/cache-b15-rac:dying\0".as_ptr(), None, Some(b15_rac_dying_cpu));
    if ret != 0 { cpuhp_remove_state_nocalls(CPUHP_AP_ARM_CACHE_B15_RAC_DEAD); unregister_reboot_notifier(&mut b15_rac_reboot_nb); iounmap(b15_rac_base); of_node_put(dn); return ret; }
    register_syscore(&b15_rac_syscore);
    spin_lock(&mut rac_lock);
    b15_rac_enable();
    set_bit(RAC_ENABLED, &mut b15_rac_flags);
    spin_unlock(&mut rac_lock);
    of_node_put(dn);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
