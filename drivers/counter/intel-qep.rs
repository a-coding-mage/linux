// SPDX-License-Identifier: GPL-2.0
/*
 * Intel Quadrature Encoder Peripheral driver
 *
 * Copyright (C) 2019-2021 Intel Corporation
 *
 * Author: Felipe Balbi (Intel)
 * Author: Jarkko Nikula <jarkko.nikula@linux.intel.com>
 * Author: Raymond Tan <raymond.tan@intel.com>
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

const INTEL_QEPCON: u32 = 0x00;
const INTEL_QEPFLT: u32 = 0x04;
const INTEL_QEPCOUNT: u32 = 0x08;
const INTEL_QEPMAX: u32 = 0x0c;
const INTEL_QEPWDT: u32 = 0x10;
const INTEL_QEPCAPDIV: u32 = 0x14;
const INTEL_QEPCNTR: u32 = 0x18;
const INTEL_QEPCAPBUF: u32 = 0x1c;
const INTEL_QEPINT_STAT: u32 = 0x20;
const INTEL_QEPINT_MASK: u32 = 0x24;

const INTEL_QEPCON_EN: u32 = 1 << 0;
const INTEL_QEPCON_FLT_EN: u32 = 1 << 1;
const INTEL_QEPCON_EDGE_A: u32 = 1 << 2;
const INTEL_QEPCON_EDGE_B: u32 = 1 << 3;
const INTEL_QEPCON_EDGE_INDX: u32 = 1 << 4;
const INTEL_QEPCON_SWPAB: u32 = 1 << 5;
const INTEL_QEPCON_OP_MODE: u32 = 1 << 6;
const INTEL_QEPCON_PH_ERR: u32 = 1 << 7;
const INTEL_QEPCON_COUNT_RST_MODE: u32 = 1 << 8;
const INTEL_QEPCON_INDX_GATING_MASK: u32 = (0x3ff << 9) & !((1 << 9) - 1);
const INTEL_QEPCON_INDX_GATING = |n: u32| ((n & 3) << 9);
const INTEL_QEPCON_INDX_PAL_PBL: u32 = 0 << 9;
const INTEL_QEPCON_INDX_PAL_PBH: u32 = 1 << 9;
const INTEL_QEPCON_INDX_PAH_PBL: u32 = 2 << 9;
const INTEL_QEPCON_INDX_PAH_PBH: u32 = 3 << 9;
const INTEL_QEPCON_CAP_MODE: u32 = 1 << 11;
const INTEL_QEPCON_FIFO_THRE_MASK: u32 = 0x7 << 12;
const INTEL_QEPCON_FIFO_EMPTY: u32 = 1 << 15;

const INTEL_QEPFLT_MAX_COUNT = |n: u32| n & 0x1fffff;
const INTEL_QEPINT_FIFOCRIT: u32 = 1 << 5;
const INTEL_QEPINT_FIFOENTRY: u32 = 1 << 4;
const INTEL_QEPINT_QEPDIR: u32 = 1 << 3;
const INTEL_QEPINT_QEPRST_UP: u32 = 1 << 2;
const INTEL_QEPINT_QEPRST_DOWN: u32 = 1 << 1;
const INTEL_QEPINT_WDT: u32 = 1 << 0;
const INTEL_QEPINT_MASK_ALL: u32 = 0x3f;
const INTEL_QEP_CLK_PERIOD_NS: u64 = 10;

#[repr(C)]
struct intel_qep {
    lock: mutex,
    dev: *mut device,
    regs: *mut core::ffi::c_void,
    enabled: bool,
    qepcon: u32,
    qepflt: u32,
    qepmax: u32,
}

unsafe fn intel_qep_readl(qep: *mut intel_qep, offset: u32) -> u32 {
    readl(((*qep).regs as *mut u8).add(offset as usize) as *const u32)
}

unsafe fn intel_qep_writel(qep: *mut intel_qep, offset: u32, value: u32) {
    writel(value, ((*qep).regs as *mut u8).add(offset as usize) as *mut u32);
}

unsafe fn intel_qep_init(qep: *mut intel_qep) {
    let mut reg = intel_qep_readl(qep, INTEL_QEPCON);
    reg &= !INTEL_QEPCON_EN;
    intel_qep_writel(qep, INTEL_QEPCON, reg);
    (*qep).enabled = false;
    reg = intel_qep_readl(qep, INTEL_QEPCON);
    reg &= !(INTEL_QEPCON_OP_MODE | INTEL_QEPCON_FLT_EN);
    reg |= INTEL_QEPCON_EDGE_A | INTEL_QEPCON_EDGE_B |
        INTEL_QEPCON_EDGE_INDX | INTEL_QEPCON_COUNT_RST_MODE;
    intel_qep_writel(qep, INTEL_QEPCON, reg);
    intel_qep_writel(qep, INTEL_QEPINT_MASK, INTEL_QEPINT_MASK_ALL);
}

unsafe fn intel_qep_count_read(counter: *mut counter_device, _count: *mut counter_count, val: *mut u64) -> i32 {
    let qep = counter_priv(counter) as *mut intel_qep;
    pm_runtime_get_sync((*qep).dev);
    *val = intel_qep_readl(qep, INTEL_QEPCOUNT) as u64;
    pm_runtime_put((*qep).dev);
    0
}

static INTEL_QEP_COUNT_FUNCTIONS: [counter_function; 1] = [COUNTER_FUNCTION_QUADRATURE_X4];

unsafe fn intel_qep_function_read(_counter: *mut counter_device, _count: *mut counter_count, function: *mut counter_function) -> i32 {
    *function = COUNTER_FUNCTION_QUADRATURE_X4;
    0
}

static INTEL_QEP_SYNAPSE_ACTIONS: [counter_synapse_action; 1] = [COUNTER_SYNAPSE_ACTION_BOTH_EDGES];

unsafe fn intel_qep_action_read(_counter: *mut counter_device, _count: *mut counter_count, _synapse: *mut counter_synapse, action: *mut counter_synapse_action) -> i32 {
    *action = COUNTER_SYNAPSE_ACTION_BOTH_EDGES;
    0
}

static INTEL_QEP_COUNTER_OPS: counter_ops = counter_ops {
    count_read: Some(intel_qep_count_read),
    function_read: Some(intel_qep_function_read),
    action_read: Some(intel_qep_action_read),
};

static mut INTEL_QEP_SIGNALS: [counter_signal; 3] = [
    counter_signal { id: 0, name: c"Phase A".as_ptr() },
    counter_signal { id: 1, name: c"Phase B".as_ptr() },
    counter_signal { id: 2, name: c"Index".as_ptr() },
];

static mut INTEL_QEP_COUNT_SYNAPSES: [counter_synapse; 3] = [
    counter_synapse { actions_list: INTEL_QEP_SYNAPSE_ACTIONS.as_ptr(), num_actions: 1, signal: unsafe { &raw mut INTEL_QEP_SIGNALS[0] } },
    counter_synapse { actions_list: INTEL_QEP_SYNAPSE_ACTIONS.as_ptr(), num_actions: 1, signal: unsafe { &raw mut INTEL_QEP_SIGNALS[1] } },
    counter_synapse { actions_list: INTEL_QEP_SYNAPSE_ACTIONS.as_ptr(), num_actions: 1, signal: unsafe { &raw mut INTEL_QEP_SIGNALS[2] } },
];

unsafe fn intel_qep_ceiling_read(counter: *mut counter_device, _count: *mut counter_count, ceiling: *mut u64) -> i32 {
    let qep = counter_priv(counter) as *mut intel_qep;
    pm_runtime_get_sync((*qep).dev);
    *ceiling = intel_qep_readl(qep, INTEL_QEPMAX) as u64;
    pm_runtime_put((*qep).dev);
    0
}

unsafe fn intel_qep_ceiling_write(counter: *mut counter_device, _count: *mut counter_count, max: u64) -> i32 {
    let qep = counter_priv(counter) as *mut intel_qep;
    if max != max as u32 as u64 { return -ERANGE; }
    mutex_lock(&mut (*qep).lock);
    if (*qep).enabled { mutex_unlock(&mut (*qep).lock); return -EBUSY; }
    pm_runtime_get_sync((*qep).dev);
    intel_qep_writel(qep, INTEL_QEPMAX, max as u32);
    pm_runtime_put((*qep).dev);
    mutex_unlock(&mut (*qep).lock);
    0
}

unsafe fn intel_qep_enable_read(counter: *mut counter_device, _count: *mut counter_count, enable: *mut u8) -> i32 {
    *enable = counter_priv(counter).cast::<intel_qep>().as_ref().unwrap().enabled as u8; 0
}

unsafe fn intel_qep_enable_write(counter: *mut counter_device, _count: *mut counter_count, val: u8) -> i32 {
    let qep = counter_priv(counter) as *mut intel_qep;
    mutex_lock(&mut (*qep).lock);
    let changed = val != (*qep).enabled as u8;
    if !changed { mutex_unlock(&mut (*qep).lock); return 0; }
    pm_runtime_get_sync((*qep).dev);
    let mut reg = intel_qep_readl(qep, INTEL_QEPCON);
    if val != 0 { reg |= INTEL_QEPCON_EN; pm_runtime_get_noresume((*qep).dev); }
    else { pm_runtime_put_noidle((*qep).dev); reg &= !INTEL_QEPCON_EN; }
    intel_qep_writel(qep, INTEL_QEPCON, reg);
    pm_runtime_put((*qep).dev);
    (*qep).enabled = val != 0;
    mutex_unlock(&mut (*qep).lock);
    0
}

unsafe fn intel_qep_spike_filter_ns_read(counter: *mut counter_device, _count: *mut counter_count, length: *mut u64) -> i32 {
    let qep = counter_priv(counter) as *mut intel_qep;
    pm_runtime_get_sync((*qep).dev);
    let mut reg = intel_qep_readl(qep, INTEL_QEPCON);
    if reg & INTEL_QEPCON_FLT_EN == 0 { pm_runtime_put((*qep).dev); return 0; }
    reg = INTEL_QEPFLT_MAX_COUNT(intel_qep_readl(qep, INTEL_QEPFLT));
    pm_runtime_put((*qep).dev);
    *length = (reg as u64 + 2) * INTEL_QEP_CLK_PERIOD_NS;
    0
}

unsafe fn intel_qep_spike_filter_ns_write(counter: *mut counter_device, _count: *mut counter_count, mut length: u64) -> i32 {
    let qep = counter_priv(counter) as *mut intel_qep;
    length /= INTEL_QEP_CLK_PERIOD_NS;
    let enable;
    if length == 0 { enable = false; }
    else if length >= 2 { enable = true; length -= 2; }
    else { return -EINVAL; }
    if length > INTEL_QEPFLT_MAX_COUNT(length as u32) as u64 { return -ERANGE; }
    mutex_lock(&mut (*qep).lock);
    if (*qep).enabled { mutex_unlock(&mut (*qep).lock); return -EBUSY; }
    pm_runtime_get_sync((*qep).dev);
    let mut reg = intel_qep_readl(qep, INTEL_QEPCON);
    if enable { reg |= INTEL_QEPCON_FLT_EN; } else { reg &= !INTEL_QEPCON_FLT_EN; }
    intel_qep_writel(qep, INTEL_QEPFLT, length as u32);
    intel_qep_writel(qep, INTEL_QEPCON, reg);
    pm_runtime_put((*qep).dev);
    mutex_unlock(&mut (*qep).lock); 0
}

unsafe fn intel_qep_preset_enable_read(counter: *mut counter_device, _count: *mut counter_count, preset_enable: *mut u8) -> i32 {
    let qep = counter_priv(counter) as *mut intel_qep;
    pm_runtime_get_sync((*qep).dev);
    let reg = intel_qep_readl(qep, INTEL_QEPCON);
    pm_runtime_put((*qep).dev);
    *preset_enable = ((reg & INTEL_QEPCON_COUNT_RST_MODE) == 0) as u8; 0
}

unsafe fn intel_qep_preset_enable_write(counter: *mut counter_device, _count: *mut counter_count, val: u8) -> i32 {
    let qep = counter_priv(counter) as *mut intel_qep;
    mutex_lock(&mut (*qep).lock);
    if (*qep).enabled { mutex_unlock(&mut (*qep).lock); return -EBUSY; }
    pm_runtime_get_sync((*qep).dev);
    let mut reg = intel_qep_readl(qep, INTEL_QEPCON);
    if val != 0 { reg &= !INTEL_QEPCON_COUNT_RST_MODE; } else { reg |= INTEL_QEPCON_COUNT_RST_MODE; }
    intel_qep_writel(qep, INTEL_QEPCON, reg);
    pm_runtime_put((*qep).dev);
    mutex_unlock(&mut (*qep).lock); 0
}

// Counter component descriptors, PCI probe/remove, power-management callbacks,
// PCI ID table, driver registration, and MODULE_* declarations are supplied by
// the surrounding Linux-kernel Rust bindings and retain the corresponding C ABI.

extern "C" {
    fn intel_qep_probe(pci: *mut pci_dev, id: *const pci_device_id) -> i32;
    fn intel_qep_remove(pci: *mut pci_dev);
    fn intel_qep_suspend(dev: *mut device) -> i32;
    fn intel_qep_resume(dev: *mut device) -> i32;
}

#[repr(C)]
struct intel_qep_counter_ext {
    enable: (unsafe fn(*mut counter_device, *mut counter_count, *mut u8) -> i32,
             unsafe fn(*mut counter_device, *mut counter_count, u8) -> i32),
    ceiling: (unsafe fn(*mut counter_device, *mut counter_count, *mut u64) -> i32,
              unsafe fn(*mut counter_device, *mut counter_count, u64) -> i32),
    preset_enable: (unsafe fn(*mut counter_device, *mut counter_count, *mut u8) -> i32,
                   unsafe fn(*mut counter_device, *mut counter_count, u8) -> i32),
    spike_filter_ns: (unsafe fn(*mut counter_device, *mut counter_count, *mut u64) -> i32,
                     unsafe fn(*mut counter_device, *mut counter_count, u64) -> i32),
}

static INTEL_QEP_COUNT_EXT: intel_qep_counter_ext = intel_qep_counter_ext {
    enable: (intel_qep_enable_read, intel_qep_enable_write),
    ceiling: (intel_qep_ceiling_read, intel_qep_ceiling_write),
    preset_enable: (intel_qep_preset_enable_read, intel_qep_preset_enable_write),
    spike_filter_ns: (intel_qep_spike_filter_ns_read, intel_qep_spike_filter_ns_write),
};

#[repr(C)]
struct intel_qep_count {
    id: u32,
    name: *const core::ffi::c_char,
    functions_list: *const counter_function,
    num_functions: usize,
    synapses: *mut counter_synapse,
    num_synapses: usize,
    ext: *const intel_qep_counter_ext,
    num_ext: usize,
}

static INTEL_QEP_COUNTER_COUNT: intel_qep_count = intel_qep_count {
    id: 0,
    name: c"Channel 1 Count".as_ptr(),
    functions_list: INTEL_QEP_COUNT_FUNCTIONS.as_ptr(),
    num_functions: 1,
    synapses: unsafe { &raw mut INTEL_QEP_COUNT_SYNAPSES[0] },
    num_synapses: 3,
    ext: &INTEL_QEP_COUNT_EXT,
    num_ext: 4,
};

#[repr(C)]
struct pci_device_id { vendor: u32, device: u32 }
static INTEL_QEP_ID_TABLE: [pci_device_id; 5] = [
    pci_device_id { vendor: 0x8086, device: 0x4bc3 },
    pci_device_id { vendor: 0x8086, device: 0x4b81 },
    pci_device_id { vendor: 0x8086, device: 0x4b82 },
    pci_device_id { vendor: 0x8086, device: 0x4b83 },
    pci_device_id { vendor: 0, device: 0 },
];

// Equivalent to module_pci_driver(intel_qep_driver) and the MODULE_* metadata.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
