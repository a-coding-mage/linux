// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2011-2012, Meador Inge, Mentor Graphics Corporation.
 *
 * Some ideas based on un-pushed work done by Vivek Mahajan, Jason Jin, and
 * Mingkai Hu from Freescale Semiconductor, Inc.
 */

// Dependencies supplied by the surrounding kernel translation.

const MPIC_MSGR_REGISTERS_PER_BLOCK: u32 = 4;
const MPIC_MSGR_STRIDE: usize = 0x10;
const MPIC_MSGR_MER_OFFSET: usize = 0x100 / core::mem::size_of::<u32>();
const MSGR_INUSE: u32 = 0;
const MSGR_FREE: u32 = 1;

static mut mpic_msgrs: *mut *mut mpic_msgr = core::ptr::null_mut();
static mut mpic_msgr_count: u32 = 0;
static mut msgrs_lock: raw_spinlock_t = raw_spinlock_t;

#[repr(C)]
pub struct mpic_msgr {
    pub base: *mut u8,
    pub mer: *mut u8,
    pub in_use: u32,
    pub num: u32,
    pub irq: u32,
    pub lock: raw_spinlock_t,
}

pub struct raw_spinlock_t;
pub struct device_node;
pub struct device { pub of_node: *mut device_node }
pub struct platform_device { pub dev: device }
pub struct resource {
    pub start: usize,
}
pub struct property {
    pub value: *const core::ffi::c_char,
}
pub struct of_device_id {
    pub compatible: *const core::ffi::c_char,
    pub data: *const core::ffi::c_void,
}
pub struct platform_driver {
    pub driver: driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
}
pub struct driver {
    pub name: *const core::ffi::c_char,
    pub of_match_table: *const of_device_id,
}

extern "C" {
    fn out_be32(addr: *mut u8, value: u32);
    fn in_be32(addr: *mut u8) -> u32;
    fn raw_spin_lock_irqsave(lock: *mut raw_spinlock_t, flags: *mut usize);
    fn raw_spin_unlock_irqrestore(lock: *mut raw_spinlock_t, flags: usize);
    fn raw_spin_lock_init(lock: *mut raw_spinlock_t);
    fn of_find_node_by_name(from: *mut device_node, name: *const core::ffi::c_char) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn of_property_present(node: *mut device_node, name: *const core::ffi::c_char) -> bool;
    fn snprintf(buf: *mut core::ffi::c_char, size: usize, fmt: *const core::ffi::c_char, ...);
    fn of_find_property(node: *mut device_node, name: *const core::ffi::c_char, len: *mut usize) -> *mut property;
    fn of_find_node_by_path(path: *const core::ffi::c_char) -> *mut device_node;
    fn of_address_to_resource(node: *mut device_node, index: u32, resource: *mut resource) -> i32;
    fn devm_ioremap(dev: *mut core::ffi::c_void, start: usize, size: usize) -> *mut u8;
    fn resource_size(resource: *const resource) -> usize;
    fn of_get_property(node: *mut device_node, name: *const core::ffi::c_char, len: *mut usize) -> *const u32;
    fn irq_of_parse_and_map(node: *mut device_node, index: u32) -> u32;
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn kzalloc(size: usize) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const core::ffi::c_char, ...);
}

unsafe fn _mpic_msgr_mer_write(msgr: *mut mpic_msgr, value: u32) {
    out_be32((*msgr).mer, value);
}

unsafe fn _mpic_msgr_mer_read(msgr: *mut mpic_msgr) -> u32 {
    in_be32((*msgr).mer)
}

unsafe fn _mpic_msgr_disable(msgr: *mut mpic_msgr) {
    let mer = _mpic_msgr_mer_read(msgr);
    _mpic_msgr_mer_write(msgr, mer & !(1u32 << (*msgr).num));
}

pub unsafe extern "C" fn mpic_msgr_get(reg_num: u32) -> *mut mpic_msgr {
    let mut flags = 0usize;
    let mut msgr = (-16isize) as *mut mpic_msgr;
    if reg_num >= mpic_msgr_count { return (-19isize) as *mut mpic_msgr; }
    raw_spin_lock_irqsave(&mut msgrs_lock, &mut flags);
    msgr = *mpic_msgrs.add(reg_num as usize);
    if (*msgr).in_use == MSGR_FREE { (*msgr).in_use = MSGR_INUSE; }
    raw_spin_unlock_irqrestore(&mut msgrs_lock, flags);
    msgr
}

pub unsafe extern "C" fn mpic_msgr_put(msgr: *mut mpic_msgr) {
    let mut flags = 0usize;
    raw_spin_lock_irqsave(&mut (*msgr).lock, &mut flags);
    (*msgr).in_use = MSGR_FREE; _mpic_msgr_disable(msgr);
    raw_spin_unlock_irqrestore(&mut (*msgr).lock, flags);
}

pub unsafe extern "C" fn mpic_msgr_enable(msgr: *mut mpic_msgr) {
    let mut flags = 0usize;
    raw_spin_lock_irqsave(&mut (*msgr).lock, &mut flags);
    let mer = _mpic_msgr_mer_read(msgr);
    _mpic_msgr_mer_write(msgr, mer | (1u32 << (*msgr).num));
    raw_spin_unlock_irqrestore(&mut (*msgr).lock, flags);
}

pub unsafe extern "C" fn mpic_msgr_disable(msgr: *mut mpic_msgr) {
    let mut flags = 0usize;
    raw_spin_lock_irqsave(&mut (*msgr).lock, &mut flags); _mpic_msgr_disable(msgr);
    raw_spin_unlock_irqrestore(&mut (*msgr).lock, flags);
}

// The following three functions compute the order and number of message register blocks.
unsafe fn mpic_msgr_number_of_blocks() -> u32 {
    let mut count = 0u32;
    let aliases = of_find_node_by_name(core::ptr::null_mut(), b"aliases\0".as_ptr() as _);
    if !aliases.is_null() {
        let mut buf = [0i8; 32];
        loop {
            snprintf(buf.as_mut_ptr(), buf.len(), b"mpic-msgr-block%d\0".as_ptr() as _, count);
            if !of_property_present(aliases, buf.as_ptr()) { break; }
            count += 1;
        }
        of_node_put(aliases);
    }
    count
}

unsafe fn mpic_msgr_number_of_registers() -> u32 { mpic_msgr_number_of_blocks() * MPIC_MSGR_REGISTERS_PER_BLOCK }

unsafe fn mpic_msgr_block_number(node: *mut device_node) -> i32 {
    let number_of_blocks = mpic_msgr_number_of_blocks();
    let aliases = of_find_node_by_name(core::ptr::null_mut(), b"aliases\0".as_ptr() as _);
    if aliases.is_null() { return -1; }
    let mut index = 0;
    let mut buf = [0i8; 64];
    while index < number_of_blocks {
        snprintf(buf.as_mut_ptr(), buf.len(), b"mpic-msgr-block%d\0".as_ptr() as _, index);
        let prop = of_find_property(aliases, buf.as_ptr(), core::ptr::null_mut());
        let tn = of_find_node_by_path((*prop).value);
        if node == tn { of_node_put(tn); break; }
        of_node_put(tn); index += 1;
    }
    of_node_put(aliases);
    if index == number_of_blocks { -1 } else { index as i32 }
}

// The probe function for a single message register block.
unsafe extern "C" fn mpic_msgr_probe(dev: *mut platform_device) -> i32 {
    let np = (*dev).dev.of_node;
    if np.is_null() { dev_err(&mut (*dev).dev, b"Device OF-Node is NULL\0".as_ptr() as _); return -14; }
    if mpic_msgrs.is_null() {
        mpic_msgr_count = mpic_msgr_number_of_registers();
        mpic_msgrs = kzalloc((mpic_msgr_count as usize) * core::mem::size_of::<*mut mpic_msgr>()) as *mut *mut mpic_msgr;
        if mpic_msgrs.is_null() { dev_err(&mut (*dev).dev, b"No memory for message register blocks\0".as_ptr() as _); return -12; }
    }
    let mut rsrc = resource { start: 0 };
    of_address_to_resource(np, 0, &mut rsrc);
    let addr = devm_ioremap(&mut (*dev).dev as *mut _ as _, rsrc.start, resource_size(&rsrc));
    if addr.is_null() { return -14; }
    let block = mpic_msgr_block_number(np);
    if block < 0 { return -19; }
    let prop = of_get_property(np, b"mpic-msgr-receive-mask\0".as_ptr() as _, core::ptr::null_mut());
    let receive_mask = if prop.is_null() { 0xFu32 } else { *prop };
    let mut irq_index = 0u32;
    for i in 0..MPIC_MSGR_REGISTERS_PER_BLOCK {
        let msgr = kzalloc(core::mem::size_of::<mpic_msgr>()) as *mut mpic_msgr;
        if msgr.is_null() { return -12; }
        (*msgr).base = addr.add((i as usize) * MPIC_MSGR_STRIDE);
        (*msgr).mer = (*msgr).base.add(MPIC_MSGR_MER_OFFSET);
        (*msgr).in_use = MSGR_FREE; (*msgr).num = i; raw_spin_lock_init(&mut (*msgr).lock);
        if receive_mask & (1 << i) != 0 {
            (*msgr).irq = irq_of_parse_and_map(np, irq_index);
            if (*msgr).irq == 0 { kfree(msgr as _); return -14; }
            irq_index += 1;
        }
        let reg = (block as u32) * MPIC_MSGR_REGISTERS_PER_BLOCK + i;
        *mpic_msgrs.add(reg as usize) = msgr;
        mpic_msgr_disable(msgr);
    }
    0
}

static mpic_msgr_ids: [of_device_id; 2] = [
    of_device_id { compatible: b"fsl,mpic-v3.1-msgr\0".as_ptr() as _, data: core::ptr::null() },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

static mut mpic_msgr_driver: platform_driver = platform_driver {
    driver: driver { name: b"mpic-msgr\0".as_ptr() as _, of_match_table: mpic_msgr_ids.as_ptr() },
    probe: Some(mpic_msgr_probe),
};

unsafe extern "C" fn mpic_msgr_init() -> i32 { platform_driver_register(&mut mpic_msgr_driver) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
