// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * arch/powerpc/platforms/embedded6xx/hlwd-pic.c
 *
 * Nintendo Wii "Hollywood" interrupt controller support.
 * Copyright (C) 2009 The GameCube Linux Team
 * Copyright (C) 2009 Albert Herranz
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

const DRV_MODULE_NAME: &str = "hlwd-pic";
const HLWD_NR_IRQS: u32 = 32;
const HW_BROADWAY_ICR: usize = 0x00;
const HW_BROADWAY_IMR: usize = 0x04;
const HW_STARLET_ICR: usize = 0x08;
const HW_STARLET_IMR: usize = 0x0c;

#[repr(C)]
pub struct irq_data {
    _private: [u8; 0],
}
#[repr(C)]
pub struct irq_domain {
    pub host_data: *mut c_void,
}
#[repr(C)]
pub struct irq_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct irq_chip {
    pub name: *const u8,
    pub irq_ack: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_mask_ack: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_mask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_unmask: Option<unsafe extern "C" fn(*mut irq_data)>,
}
#[repr(C)]
pub struct irq_domain_ops {
    pub map: Option<unsafe extern "C" fn(*mut irq_domain, u32, usize) -> i32>,
}
#[repr(C)]
pub struct resource {
    pub start: usize,
    _private: [u8; 0],
}
pub struct device_node;
pub type irq_hw_number_t = usize;

extern "C" {
    fn irqd_to_hwirq(d: *mut irq_data) -> i32;
    fn irq_data_get_irq_chip_data(d: *mut irq_data) -> *mut c_void;
    fn clrbits32(addr: *mut c_void, mask: u32);
    fn setbits32(addr: *mut c_void, mask: u32);
    fn out_be32(addr: *mut c_void, value: u32);
    fn in_be32(addr: *mut c_void) -> u32;
    fn irq_set_chip_data(virq: u32, data: *mut c_void);
    fn irq_set_status_flags(virq: u32, flags: u32);
    fn irq_set_chip_and_handler(virq: u32, chip: *mut irq_chip, handler: *const c_void);
    fn __ffs(value: u32) -> u32;
    fn irq_desc_get_chip(desc: *mut irq_desc) -> *mut irq_chip;
    fn irq_desc_get_handler_data(desc: *mut irq_desc) -> *mut irq_domain;
    fn raw_spin_lock(lock: *mut c_void);
    fn raw_spin_unlock(lock: *mut c_void);
    fn generic_handle_domain_irq(domain: *mut irq_domain, hwirq: u32);
    fn irqd_irq_disabled(d: *mut irq_data) -> bool;
    fn irq_domain_create_linear(node: *mut c_void, size: u32, ops: *const irq_domain_ops, data: *mut c_void) -> *mut irq_domain;
    fn of_fwnode_handle(np: *mut device_node) -> *mut c_void;
    fn of_address_to_resource(np: *mut device_node, index: u32, res: *mut resource) -> i32;
    fn resource_size(res: *const resource) -> usize;
    fn ioremap(start: usize, size: usize) -> *mut c_void;
    fn iounmap(addr: *mut c_void);
    fn irq_find_mapping(domain: *mut irq_domain, hwirq: u32) -> u32;
    fn of_get_property(np: *mut device_node, name: *const u8, length: *mut usize) -> *const u32;
    fn irq_of_parse_and_map(np: *mut device_node, index: u32) -> i32;
    fn irq_set_handler_data(virq: i32, data: *mut irq_domain);
    fn irq_set_chained_handler(virq: i32, handler: unsafe extern "C" fn(*mut irq_desc));
    fn of_find_compatible_node(from: *mut device_node, type_: *const u8, compatible: *const u8) -> *mut device_node;
    fn of_node_put(node: *mut device_node);
    fn pr_err(fmt: *const u8, ...);
    fn pr_info(fmt: *const u8, ...);
}

const IRQ_LEVEL: u32 = 1;
extern "C" {
    static mut hlwd_irq_host: *mut irq_domain;
    static handle_level_irq: c_void;
}

unsafe extern "C" fn hlwd_pic_mask_and_ack(d: *mut irq_data) {
    let irq = irqd_to_hwirq(d);
    let io_base = irq_data_get_irq_chip_data(d);
    let mask = 1u32 << (irq as u32);
    clrbits32(io_base.add(HW_BROADWAY_IMR), mask);
    out_be32(io_base.add(HW_BROADWAY_ICR), mask);
}

unsafe extern "C" fn hlwd_pic_ack(d: *mut irq_data) {
    let irq = irqd_to_hwirq(d);
    let io_base = irq_data_get_irq_chip_data(d);
    out_be32(io_base.add(HW_BROADWAY_ICR), 1u32 << (irq as u32));
}

unsafe extern "C" fn hlwd_pic_mask(d: *mut irq_data) {
    let irq = irqd_to_hwirq(d);
    let io_base = irq_data_get_irq_chip_data(d);
    clrbits32(io_base.add(HW_BROADWAY_IMR), 1u32 << (irq as u32));
}

unsafe extern "C" fn hlwd_pic_unmask(d: *mut irq_data) {
    let irq = irqd_to_hwirq(d);
    let io_base = irq_data_get_irq_chip_data(d);
    setbits32(io_base.add(HW_BROADWAY_IMR), 1u32 << (irq as u32));
    // Make sure the ARM (aka. Starlet) doesn't handle this interrupt.
    clrbits32(io_base.add(HW_STARLET_IMR), 1u32 << (irq as u32));
}

static mut hlwd_pic: irq_chip = irq_chip {
    name: b"hlwd-pic\0".as_ptr(),
    irq_ack: Some(hlwd_pic_ack),
    irq_mask_ack: Some(hlwd_pic_mask_and_ack),
    irq_mask: Some(hlwd_pic_mask),
    irq_unmask: Some(hlwd_pic_unmask),
};

unsafe extern "C" fn hlwd_pic_map(h: *mut irq_domain, virq: u32, _hwirq: irq_hw_number_t) -> i32 {
    irq_set_chip_data(virq, (*h).host_data);
    irq_set_status_flags(virq, IRQ_LEVEL);
    irq_set_chip_and_handler(virq, &mut hlwd_pic, &handle_level_irq);
    0
}

static hlwd_irq_domain_ops: irq_domain_ops = irq_domain_ops { map: Some(hlwd_pic_map) };

unsafe fn __hlwd_pic_get_irq(h: *mut irq_domain) -> u32 {
    let io_base = (*h).host_data;
    let irq_status = in_be32(io_base.add(HW_BROADWAY_ICR)) & in_be32(io_base.add(HW_BROADWAY_IMR));
    if irq_status == 0 { return 0; }
    __ffs(irq_status)
}

unsafe extern "C" fn hlwd_pic_irq_cascade(desc: *mut irq_desc) {
    let chip = irq_desc_get_chip(desc);
    let domain = irq_desc_get_handler_data(desc);
    // The descriptor lock and irq_data are supplied by the kernel layout.
    raw_spin_lock(desc as *mut c_void);
    ((*chip).irq_mask.unwrap())((*chip as *mut irq_chip) as *mut irq_data);
    raw_spin_unlock(desc as *mut c_void);
    let hwirq = __hlwd_pic_get_irq(domain);
    if hwirq != 0 { generic_handle_domain_irq(domain, hwirq); } else { pr_err(b"spurious interrupt!\n\0".as_ptr()); }
    raw_spin_lock(desc as *mut c_void);
    ((*chip).irq_ack.unwrap())((*chip as *mut irq_chip) as *mut irq_data);
    if !irqd_irq_disabled(desc as *mut irq_data) { if let Some(unmask) = (*chip).irq_unmask { unmask(desc as *mut irq_data); } }
    raw_spin_unlock(desc as *mut c_void);
}

unsafe fn __hlwd_quiesce(io_base: *mut c_void) {
    out_be32(io_base.add(HW_BROADWAY_IMR), 0);
    out_be32(io_base.add(HW_BROADWAY_ICR), 0xffff_ffff);
}

unsafe fn hlwd_pic_init(np: *mut device_node) -> *mut irq_domain {
    let mut res = core::mem::MaybeUninit::<resource>::uninit();
    if of_address_to_resource(np, 0, res.as_mut_ptr()) != 0 { pr_err(b"no io memory range found\n\0".as_ptr()); return core::ptr::null_mut(); }
    let res = res.assume_init();
    let io_base = ioremap(res.start, resource_size(&res));
    if io_base.is_null() { pr_err(b"ioremap failed\n\0".as_ptr()); return core::ptr::null_mut(); }
    pr_info(b"controller mapped\n\0".as_ptr());
    __hlwd_quiesce(io_base);
    let domain = irq_domain_create_linear(of_fwnode_handle(np), HLWD_NR_IRQS, &hlwd_irq_domain_ops, io_base);
    if domain.is_null() { pr_err(b"failed to allocate irq_domain\n\0".as_ptr()); iounmap(io_base); }
    domain
}

pub unsafe extern "C" fn hlwd_pic_get_irq() -> u32 {
    let hwirq = __hlwd_pic_get_irq(hlwd_irq_host);
    if hwirq != 0 { irq_find_mapping(hlwd_irq_host, hwirq) } else { 0 }
}

pub unsafe extern "C" fn hlwd_pic_probe() {
    let mut np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), b"nintendo,hollywood-pic\0".as_ptr());
    while !np.is_null() {
        let interrupts = of_get_property(np, b"interrupts\0".as_ptr(), core::ptr::null_mut());
        if !interrupts.is_null() {
            let host = hlwd_pic_init(np);
            if host.is_null() { core::intrinsics::abort(); }
            let cascade_virq = irq_of_parse_and_map(np, 0);
            irq_set_handler_data(cascade_virq, host);
            irq_set_chained_handler(cascade_virq, hlwd_pic_irq_cascade);
            hlwd_irq_host = host;
            of_node_put(np);
            break;
        }
        let old_np = np;
        np = of_find_compatible_node(np, core::ptr::null(), b"nintendo,hollywood-pic\0".as_ptr());
        of_node_put(old_np);
    }
}

pub unsafe extern "C" fn hlwd_quiesce() {
    __hlwd_quiesce((*hlwd_irq_host).host_data);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
