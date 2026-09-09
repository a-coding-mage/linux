// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * arch/powerpc/platforms/embedded6xx/flipper-pic.c
 *
 * Nintendo GameCube/Wii "Flipper" interrupt controller support.
 * Copyright (C) 2004-2009 The GameCube Linux Team
 * Copyright (C) 2007,2008,2009 Albert Herranz
 */

// Dependency intent: Linux kernel and flipper-pic header declarations are supplied externally.

const FLIPPER_NR_IRQS: u32 = 32;
const FLIPPER_ICR: usize = 0x00;
const FLIPPER_ICR_RSS: u32 = 1 << 16; // reset switch state
const FLIPPER_IMR: usize = 0x04;
const FLIPPER_RESET: usize = 0x24;

#[repr(C)]
pub struct irq_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct irq_domain {
    pub host_data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    pub start: usize,
    _private: [u8; 0],
}

#[repr(C)]
pub struct irq_chip {
    pub name: *const core::ffi::c_char,
    pub irq_ack: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_mask_ack: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_mask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_unmask: Option<unsafe extern "C" fn(*mut irq_data)>,
}

#[repr(C)]
pub struct irq_domain_ops {
    pub map: Option<unsafe extern "C" fn(*mut irq_domain, u32, usize) -> i32>,
}

extern "C" {
    fn irqd_to_hwirq(d: *mut irq_data) -> usize;
    fn irq_data_get_irq_chip_data(d: *mut irq_data) -> *mut core::ffi::c_void;
    fn clrbits32(addr: *mut u8, mask: u32);
    fn setbits32(addr: *mut u8, mask: u32);
    fn out_be32(addr: *mut u8, value: u32);
    fn out_8(addr: *mut u8, value: u8);
    fn in_be32(addr: *mut u8) -> u32;
    fn irq_set_chip_data(virq: u32, data: *mut core::ffi::c_void);
    fn irq_set_status_flags(virq: u32, flags: u32);
    fn irq_set_chip_and_handler(virq: u32, chip: *mut irq_chip, handler: unsafe extern "C" fn());
    fn handle_level_irq();
    fn of_get_parent(np: *mut device_node) -> *mut device_node;
    fn of_device_is_compatible(np: *mut device_node, compatible: *const core::ffi::c_char) -> bool;
    fn of_address_to_resource(np: *mut device_node, index: u32, res: *mut resource) -> i32;
    fn resource_size(res: *const resource) -> usize;
    fn ioremap(start: usize, size: usize) -> *mut u8;
    fn of_fwnode_handle(np: *mut device_node) -> *mut core::ffi::c_void;
    fn irq_domain_create_linear(fwnode: *mut core::ffi::c_void, size: u32,
        ops: *const irq_domain_ops, host_data: *mut core::ffi::c_void) -> *mut irq_domain;
    fn __ffs(value: u32) -> i32;
    fn irq_find_mapping(domain: *mut irq_domain, hwirq: u32) -> u32;
    fn irq_set_default_domain(domain: *mut irq_domain);
    fn of_find_compatible_node(from: *mut device_node, type_: *const core::ffi::c_char,
        compatible: *const core::ffi::c_char) -> *mut device_node;
    fn of_node_put(np: *mut device_node);
    fn BUG_ON(condition: bool);
    fn pr_err(fmt: *const core::ffi::c_char, ...);
    fn pr_info(fmt: *const core::ffi::c_char, ...);
}

const IRQ_LEVEL: u32 = 1;

unsafe extern "C" fn flipper_pic_mask_and_ack(d: *mut irq_data) {
    let irq = irqd_to_hwirq(d);
    let io_base = irq_data_get_irq_chip_data(d) as *mut u8;
    let mask = 1u32 << irq;
    clrbits32(io_base.add(FLIPPER_IMR), mask);
    // this is at least needed for RSW
    out_be32(io_base.add(FLIPPER_ICR), mask);
}

unsafe extern "C" fn flipper_pic_ack(d: *mut irq_data) {
    let irq = irqd_to_hwirq(d);
    let io_base = irq_data_get_irq_chip_data(d) as *mut u8;
    // this is at least needed for RSW
    out_be32(io_base.add(FLIPPER_ICR), 1u32 << irq);
}

unsafe extern "C" fn flipper_pic_mask(d: *mut irq_data) {
    let irq = irqd_to_hwirq(d);
    let io_base = irq_data_get_irq_chip_data(d) as *mut u8;
    clrbits32(io_base.add(FLIPPER_IMR), 1u32 << irq);
}

unsafe extern "C" fn flipper_pic_unmask(d: *mut irq_data) {
    let irq = irqd_to_hwirq(d);
    let io_base = irq_data_get_irq_chip_data(d) as *mut u8;
    setbits32(io_base.add(FLIPPER_IMR), 1u32 << irq);
}

static mut flipper_pic: irq_chip = irq_chip {
    name: b"flipper-pic\0".as_ptr() as *const core::ffi::c_char,
    irq_ack: Some(flipper_pic_ack), irq_mask_ack: Some(flipper_pic_mask_and_ack),
    irq_mask: Some(flipper_pic_mask), irq_unmask: Some(flipper_pic_unmask),
};

static mut flipper_irq_host: *mut irq_domain = core::ptr::null_mut();

unsafe extern "C" fn flipper_pic_map(h: *mut irq_domain, virq: u32, _hwirq: usize) -> i32 {
    irq_set_chip_data(virq, (*h).host_data);
    irq_set_status_flags(virq, IRQ_LEVEL);
    irq_set_chip_and_handler(virq, &mut flipper_pic, handle_level_irq);
    0
}

static flipper_irq_domain_ops: irq_domain_ops = irq_domain_ops { map: Some(flipper_pic_map) };

unsafe fn __flipper_quiesce(io_base: *mut u8) {
    // mask and ack all IRQs
    out_be32(io_base.add(FLIPPER_IMR), 0x00000000);
    out_be32(io_base.add(FLIPPER_ICR), 0xffffffff);
}

unsafe fn flipper_pic_init(np: *mut device_node) -> *mut irq_domain {
    let pi = of_get_parent(np);
    if pi.is_null() { pr_err(b"no parent found\n\0".as_ptr() as _); return core::ptr::null_mut(); }
    if !of_device_is_compatible(pi, b"nintendo,flipper-pi\0".as_ptr() as _) {
        pr_err(b"unexpected parent compatible\n\0".as_ptr() as _); return core::ptr::null_mut();
    }
    let mut res = core::mem::MaybeUninit::<resource>::uninit();
    if of_address_to_resource(pi, 0, res.as_mut_ptr()) != 0 {
        pr_err(b"no io memory range found\n\0".as_ptr() as _); return core::ptr::null_mut();
    }
    let res = res.assume_init();
    let io_base = ioremap(res.start, resource_size(&res));
    pr_info(b"controller at 0x%pa mapped to 0x%p\n\0".as_ptr() as _, &res.start, io_base);
    __flipper_quiesce(io_base);
    let irq_domain = irq_domain_create_linear(of_fwnode_handle(np), FLIPPER_NR_IRQS,
        &flipper_irq_domain_ops, io_base as _);
    if irq_domain.is_null() { pr_err(b"failed to allocate irq_domain\n\0".as_ptr() as _); }
    irq_domain
}

pub unsafe extern "C" fn flipper_pic_get_irq() -> u32 {
    let io_base = (*flipper_irq_host).host_data as *mut u8;
    let irq_status = in_be32(io_base.add(FLIPPER_ICR)) & in_be32(io_base.add(FLIPPER_IMR));
    if irq_status == 0 { return 0; } // no more IRQs pending
    irq_find_mapping(flipper_irq_host, __ffs(irq_status) as u32)
}

pub unsafe extern "C" fn flipper_pic_probe() {
    let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"nintendo,flipper-pic\0".as_ptr() as _);
    BUG_ON(np.is_null());
    flipper_irq_host = flipper_pic_init(np);
    BUG_ON(flipper_irq_host.is_null());
    irq_set_default_domain(flipper_irq_host);
    of_node_put(np);
}

/// flipper_quiesce() - quiesce flipper irq controller
///
/// Mask and ack all interrupt sources.
pub unsafe extern "C" fn flipper_quiesce() {
    __flipper_quiesce((*flipper_irq_host).host_data as *mut u8);
}

/* Resets the platform. */
pub unsafe extern "C" fn flipper_platform_reset() {
    if !flipper_irq_host.is_null() && !(*flipper_irq_host).host_data.is_null() {
        out_8(((*flipper_irq_host).host_data as *mut u8).add(FLIPPER_RESET), 0x00);
    }
}

/* Returns non-zero if the reset button is pressed. */
pub unsafe extern "C" fn flipper_is_reset_button_pressed() -> i32 {
    if !flipper_irq_host.is_null() && !(*flipper_irq_host).host_data.is_null() {
        let icr = in_be32(((*flipper_irq_host).host_data as *mut u8).add(FLIPPER_ICR));
        return if (icr & FLIPPER_ICR_RSS) == 0 { 1 } else { 0 };
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
