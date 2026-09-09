// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Xilinx SLCR driver
 *
 * Copyright (c) 2011-2013 Xilinx Inc.
 */

// Linux dependencies are supplied by the surrounding translation unit.

const SLCR_UNLOCK_OFFSET: u32 = 0x8;
const SLCR_PS_RST_CTRL_OFFSET: u32 = 0x200;
const SLCR_A9_CPU_RST_CTRL_OFFSET: u32 = 0x244;
const SLCR_REBOOT_STATUS_OFFSET: u32 = 0x258;
const SLCR_PSS_IDCODE: u32 = 0x530;
const SLCR_L2C_RAM: u32 = 0xA1C;

const SLCR_UNLOCK_MAGIC: u32 = 0xDF0D;
const SLCR_A9_CPU_CLKSTOP: u32 = 0x10;
const SLCR_A9_CPU_RST: u32 = 0x1;
const SLCR_PSS_IDCODE_DEVICE_SHIFT: u32 = 12;
const SLCR_PSS_IDCODE_DEVICE_MASK: u32 = 0x1F;

static mut zynq_slcr_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut zynq_slcr_regmap: *mut regmap = core::ptr::null_mut();

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct notifier_block {
    pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, c_ulong, *mut c_void) -> c_int>,
    pub next: *mut notifier_block,
    pub priority: c_int,
}

type c_int = i32;
type c_ulong = usize;
type c_void = core::ffi::c_void;

extern "C" {
    fn regmap_write(map: *mut regmap, offset: u32, val: u32) -> c_int;
    fn regmap_read(map: *mut regmap, offset: u32, val: *mut u32) -> c_int;
    fn readl(addr: *mut c_void) -> u32;
    fn writel(val: u32, addr: *mut c_void);
    fn of_find_compatible_node(from: *mut c_void, typ: *mut c_void, compatible: *const u8) -> *mut device_node;
    fn of_iomap(node: *mut device_node, index: c_int) -> *mut c_void;
    fn syscon_regmap_lookup_by_compatible(compatible: *const u8) -> *mut regmap;
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> c_int;
    fn register_restart_handler(nb: *mut notifier_block) -> c_int;
    fn of_node_put(node: *mut device_node);
}

#[repr(C)]
pub struct device_node {
    pub data: *mut c_void,
}

unsafe fn zynq_slcr_write(val: u32, offset: u32) -> c_int {
    regmap_write(zynq_slcr_regmap, offset, val)
}

unsafe fn zynq_slcr_read(val: *mut u32, offset: u32) -> c_int {
    regmap_read(zynq_slcr_regmap, offset, val)
}

#[inline]
unsafe fn zynq_slcr_unlock() -> c_int {
    zynq_slcr_write(SLCR_UNLOCK_MAGIC, SLCR_UNLOCK_OFFSET);
    0
}

pub unsafe extern "C" fn zynq_slcr_get_device_id() -> u32 {
    let mut val: u32 = 0;
    zynq_slcr_read(&mut val, SLCR_PSS_IDCODE);
    val >>= SLCR_PSS_IDCODE_DEVICE_SHIFT;
    val &= SLCR_PSS_IDCODE_DEVICE_MASK;
    val
}

unsafe extern "C" fn zynq_slcr_system_restart(
    _nb: *mut notifier_block,
    _action: c_ulong,
    _data: *mut c_void,
) -> c_int {
    let mut reboot: u32 = 0;
    zynq_slcr_read(&mut reboot, SLCR_REBOOT_STATUS_OFFSET);
    zynq_slcr_write(reboot & 0xF0FFFFFF, SLCR_REBOOT_STATUS_OFFSET);
    zynq_slcr_write(1, SLCR_PS_RST_CTRL_OFFSET);
    0
}

static mut zynq_slcr_restart_nb: notifier_block = notifier_block {
    notifier_call: Some(zynq_slcr_system_restart),
    next: core::ptr::null_mut(),
    priority: 192,
};

pub unsafe extern "C" fn zynq_slcr_cpu_start(cpu: c_int) {
    let mut reg: u32 = 0;
    zynq_slcr_read(&mut reg, SLCR_A9_CPU_RST_CTRL_OFFSET);
    reg &= !(SLCR_A9_CPU_RST << cpu);
    zynq_slcr_write(reg, SLCR_A9_CPU_RST_CTRL_OFFSET);
    reg &= !(SLCR_A9_CPU_CLKSTOP << cpu);
    zynq_slcr_write(reg, SLCR_A9_CPU_RST_CTRL_OFFSET);
    zynq_slcr_cpu_state_write(cpu, false);
}

pub unsafe extern "C" fn zynq_slcr_cpu_stop(cpu: c_int) {
    let mut reg: u32 = 0;
    zynq_slcr_read(&mut reg, SLCR_A9_CPU_RST_CTRL_OFFSET);
    reg |= (SLCR_A9_CPU_CLKSTOP | SLCR_A9_CPU_RST) << cpu;
    zynq_slcr_write(reg, SLCR_A9_CPU_RST_CTRL_OFFSET);
}

pub unsafe extern "C" fn zynq_slcr_cpu_state_read(cpu: c_int) -> bool {
    let mut state = readl((zynq_slcr_base as *mut u8).add(SLCR_REBOOT_STATUS_OFFSET as usize) as *mut c_void);
    state &= 1 << (31 - cpu);
    state == 0
}

pub unsafe extern "C" fn zynq_slcr_cpu_state_write(cpu: c_int, die: bool) {
    let addr = (zynq_slcr_base as *mut u8).add(SLCR_REBOOT_STATUS_OFFSET as usize) as *mut c_void;
    let mut state = readl(addr);
    let mask = 1 << (31 - cpu);
    if die { state |= mask; } else { state &= !mask; }
    writel(state, addr);
}

pub unsafe extern "C" fn zynq_early_slcr_init() -> c_int {
    let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), b"xlnx,zynq-slcr\0".as_ptr());
    if np.is_null() { return -19; }
    zynq_slcr_base = of_iomap(np, 0);
    if zynq_slcr_base.is_null() { return -19; }
    (*np).data = zynq_slcr_base;
    zynq_slcr_regmap = syscon_regmap_lookup_by_compatible(b"xlnx,zynq-slcr\0".as_ptr());
    zynq_slcr_unlock();
    regmap_update_bits(zynq_slcr_regmap, SLCR_L2C_RAM, 0x70707, 0x20202);
    register_restart_handler(&mut zynq_slcr_restart_nb);
    of_node_put(np);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
