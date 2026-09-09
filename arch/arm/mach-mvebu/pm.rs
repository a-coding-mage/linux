// SPDX-License-Identifier: GPL-2.0-only
/*
 * Suspend/resume support. Currently supporting Armada XP only.
 *
 * Copyright (C) 2014 Marvell
 *
 * Thomas Petazzoni <thomas.petazzoni@free-electrons.com>
 */

// Linux and platform dependencies supplied by other translation units.

const SDRAM_CONFIG_OFFS: usize = 0x0;
const SDRAM_CONFIG_SR_MODE_BIT: u32 = 1 << 24;
const SDRAM_OPERATION_OFFS: usize = 0x18;
const SDRAM_OPERATION_SELF_REFRESH: u32 = 0x7;
const SDRAM_DLB_EVICTION_OFFS: usize = 0x30c;
const SDRAM_DLB_EVICTION_THRESHOLD_MASK: u32 = 0xff;

type U32 = u32;
type PhysAddr = usize;
type SuspendState = u32;
type DeviceNode = core::ffi::c_void;
type Resource = core::ffi::c_void;

extern "C" {
    static mut mvebu_board_pm_enter:
        Option<unsafe extern "C" fn(sdram_reg: *mut core::ffi::c_void, srcmd: U32)>;
    static mut sdram_ctrl: *mut core::ffi::c_void;

    fn flush_cache_all();
    fn outer_flush_all();
    fn dsb();
    fn readl(addr: *mut core::ffi::c_void) -> U32;
    fn writel(value: U32, addr: *mut core::ffi::c_void);
    fn udelay(usecs: U32);
    fn of_find_node_by_name(from: *mut DeviceNode, name: *const i8) -> *mut DeviceNode;
    fn BUG_ON(condition: bool);
    fn cpu_to_be32(value: U32) -> U32;
    fn of_translate_address(np: *mut DeviceNode, address: *const U32) -> PhysAddr;
    fn __pa_symbol(symbol: unsafe extern "C" fn());
    fn armada_370_xp_cpu_resume();
    fn phys_to_virt(address: PhysAddr) -> *mut U32;
    fn mvebu_mbus_save_cpu_target(store_addr: *mut U32) -> usize;
    fn of_machine_is_compatible(compatible: *const i8) -> bool;
    fn cpu_pm_enter();
    fn cpu_suspend(arg: U32, fnptr: unsafe extern "C" fn(usize) -> i32) -> i32;
    fn outer_resume();
    fn mvebu_v7_pmsu_idle_exit();
    fn set_cpu_coherent();
    fn cpu_pm_exit();
    fn cpu_do_idle();
    fn pr_warn(format: *const i8);
    fn suspend_set_ops(ops: *const PlatformSuspendOps);
    fn of_find_compatible_node(from: *mut DeviceNode, ty: *const i8, compatible: *const i8)
        -> *mut DeviceNode;
    fn of_address_to_resource(np: *mut DeviceNode, index: U32, resource: *mut Resource) -> i32;
    fn of_node_put(np: *mut DeviceNode);
    fn resource_size(resource: *const Resource) -> usize;
    fn request_mem_region(start: PhysAddr, size: usize, name: *const i8) -> bool;
    fn ioremap(start: PhysAddr, size: usize) -> *mut core::ffi::c_void;
    fn release_mem_region(start: PhysAddr, size: usize);
}

const BOOT_INFO_ADDR: PhysAddr = 0x3000;
const BOOT_MAGIC_WORD: U32 = 0xdeadb002;
const BOOT_MAGIC_LIST_END: U32 = 0xffffffff;

const MBUS_WINDOW_12_CTRL: U32 = 0xd00200b0;
const MBUS_INTERNAL_REG_ADDRESS: U32 = 0xd0020080;

#[inline]
const fn SDRAM_WIN_BASE_REG(x: U32) -> U32 { 0x20180 + (0x8 * x) }
#[inline]
const fn SDRAM_WIN_CTRL_REG(x: U32) -> U32 { 0x20184 + (0x8 * x) }

#[no_mangle]
unsafe extern "C" fn mvebu_pm_powerdown(_data: usize) -> i32 {
    let mut reg: U32;
    let mut srcmd: U32;

    flush_cache_all();
    outer_flush_all();
    dsb();

    reg = readl(sdram_ctrl.add(SDRAM_DLB_EVICTION_OFFS));
    reg &= !SDRAM_DLB_EVICTION_THRESHOLD_MASK;
    writel(reg, sdram_ctrl.add(SDRAM_DLB_EVICTION_OFFS));
    udelay(7);

    reg = readl(sdram_ctrl.add(SDRAM_CONFIG_OFFS));
    reg &= !SDRAM_CONFIG_SR_MODE_BIT;
    writel(reg, sdram_ctrl.add(SDRAM_CONFIG_OFFS));

    srcmd = readl(sdram_ctrl.add(SDRAM_OPERATION_OFFS));
    srcmd &= !0x1f;
    srcmd |= SDRAM_OPERATION_SELF_REFRESH;

    if let Some(enter) = mvebu_board_pm_enter {
        enter(sdram_ctrl.add(SDRAM_OPERATION_OFFS), srcmd);
    }
    0
}

unsafe extern "C" fn mvebu_internal_reg_base() -> PhysAddr {
    let np = of_find_node_by_name(core::ptr::null_mut(), b"internal-regs\0".as_ptr() as *const i8);
    BUG_ON(np.is_null());
    let in_addr = [cpu_to_be32(0xf0010000), 0];
    of_translate_address(np, in_addr.as_ptr())
}

unsafe extern "C" fn mvebu_pm_store_armadaxp_bootinfo(mut store_addr: *mut U32) {
    let resume_pc = __pa_symbol(armada_370_xp_cpu_resume);
    writel(BOOT_MAGIC_WORD, store_addr); store_addr = store_addr.add(1);
    writel(resume_pc as U32, store_addr); store_addr = store_addr.add(1);
    writel(MBUS_WINDOW_12_CTRL, store_addr); store_addr = store_addr.add(1);
    writel(0, store_addr); store_addr = store_addr.add(1);
    writel(MBUS_INTERNAL_REG_ADDRESS, store_addr); store_addr = store_addr.add(1);
    writel(mvebu_internal_reg_base() as U32, store_addr); store_addr = store_addr.add(1);
    store_addr = store_addr.add(mvebu_mbus_save_cpu_target(store_addr));
    writel(BOOT_MAGIC_LIST_END, store_addr);
}

unsafe extern "C" fn mvebu_pm_store_bootinfo() -> i32 {
    let store_addr = phys_to_virt(BOOT_INFO_ADDR);
    if of_machine_is_compatible(b"marvell,armadaxp\0".as_ptr() as *const i8) {
        mvebu_pm_store_armadaxp_bootinfo(store_addr);
    } else { return -19; }
    0
}

unsafe extern "C" fn mvebu_enter_suspend() -> i32 {
    let ret = mvebu_pm_store_bootinfo();
    if ret != 0 { return ret; }
    cpu_pm_enter();
    cpu_suspend(0, mvebu_pm_powerdown);
    outer_resume();
    mvebu_v7_pmsu_idle_exit();
    set_cpu_coherent();
    cpu_pm_exit();
    0
}

unsafe extern "C" fn mvebu_pm_enter(state: SuspendState) -> i32 {
    match state {
        1 => cpu_do_idle(),
        3 => { pr_warn(b"Entering suspend to RAM. Only special wake-up sources will resume the system\n\0".as_ptr() as *const i8); return mvebu_enter_suspend(); }
        _ => return -22,
    }
    0
}

unsafe extern "C" fn mvebu_pm_valid(state: SuspendState) -> i32 {
    if state == 1 { return 1; }
    if state == 3 && mvebu_board_pm_enter.is_some() { return 1; }
    0
}

#[repr(C)]
struct PlatformSuspendOps {
    enter: Option<unsafe extern "C" fn(SuspendState) -> i32>,
    valid: Option<unsafe extern "C" fn(SuspendState) -> i32>,
}

static MVEVU_PM_OPS: PlatformSuspendOps = PlatformSuspendOps { enter: Some(mvebu_pm_enter), valid: Some(mvebu_pm_valid) };

unsafe extern "C" fn mvebu_pm_init() -> i32 {
    if !of_machine_is_compatible(b"marvell,armadaxp\0".as_ptr() as *const i8)
        && !of_machine_is_compatible(b"marvell,armada370\0".as_ptr() as *const i8)
        && !of_machine_is_compatible(b"marvell,armada380\0".as_ptr() as *const i8)
        && !of_machine_is_compatible(b"marvell,armada390\0".as_ptr() as *const i8) { return -19; }
    suspend_set_ops(&MVEVU_PM_OPS);
    0
}

// late_initcall(mvebu_pm_init);

#[no_mangle]
pub unsafe extern "C" fn mvebu_pm_suspend_init(
    board_pm_enter: Option<unsafe extern "C" fn(*mut core::ffi::c_void, U32)>,
) -> i32 {
    let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"marvell,armada-xp-sdram-controller\0".as_ptr() as *const i8);
    if np.is_null() { return -19; }
    let mut res: Resource = core::mem::zeroed();
    if of_address_to_resource(np, 0, &mut res) != 0 { of_node_put(np); return -19; }
    let size = resource_size(&res);
    if !request_mem_region(0, size, core::ptr::null()) { of_node_put(np); return -16; }
    sdram_ctrl = ioremap(0, size);
    if sdram_ctrl.is_null() { release_mem_region(0, size); of_node_put(np); return -12; }
    of_node_put(np);
    mvebu_board_pm_enter = board_pm_enter;
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
