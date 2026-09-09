// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2017-2018 Bartosz Golaszewski <brgl@bgdev.pl>
 * Copyright (C) 2020 Bartosz Golaszewski <bgolaszewski@baylibre.com>
 */

// External Linux kernel declarations supplied by the surrounding build.
use core::ffi::c_void;

#[repr(C)]
pub struct irq_work {
    _private: [u8; 0],
}
#[repr(C)]
pub struct irq_domain {
    pub host_data: *mut c_void,
}
#[repr(C)]
pub struct irq_data {
    _private: [u8; 0],
}
#[repr(C)]
pub struct irq_desc {
    _private: [u8; 0],
}
#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct irq_chip {
    _private: [u8; 0],
}
#[repr(C)]
pub struct irq_domain_ops {
    _private: [u8; 0],
}
#[repr(C)]
pub struct irq_sim_ops {
    pub irq_sim_irq_requested: Option<unsafe extern "C" fn(*mut irq_domain, usize, *mut c_void) -> i32>,
    pub irq_sim_irq_released: Option<unsafe extern "C" fn(*mut irq_domain, usize, *mut c_void)>,
}

type irq_hw_number_t = usize;

const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;
const IRQ_TYPE_EDGE_BOTH: u32 = 3;
const IRQCHIP_STATE_PENDING: i32 = 0;

#[repr(C)]
struct irq_sim_work_ctx {
    work: irq_work,
    irq_count: u32,
    pending: *mut usize,
    domain: *mut irq_domain,
    ops: irq_sim_ops,
    user_data: *mut c_void,
}

#[repr(C)]
struct irq_sim_irq_ctx {
    enabled: bool,
    work_ctx: *mut irq_sim_work_ctx,
}

unsafe extern "C" {
    fn irq_data_get_irq_chip_data(data: *mut irq_data) -> *mut c_void;
    fn irqd_to_hwirq(data: *mut irq_data) -> irq_hw_number_t;
    fn irqd_set_trigger_type(data: *mut irq_data, ty: u32);
    fn test_bit(nr: irq_hw_number_t, addr: *const usize) -> bool;
    fn assign_bit(nr: irq_hw_number_t, addr: *mut usize, value: bool);
    fn irq_work_queue(work: *mut irq_work);
    fn bitmap_empty(bitmap: *const usize, bits: u32) -> bool;
    fn find_next_bit(bitmap: *const usize, bits: u32, offset: u32) -> u32;
    fn clear_bit(nr: u32, addr: *mut usize);
    fn irq_find_mapping(domain: *mut irq_domain, hwirq: u32) -> i32;
    fn irq_to_desc(irq: i32) -> *mut irq_desc;
    fn handle_simple_irq(desc: *mut irq_desc);
    fn irq_set_chip(virq: u32, chip: *const irq_chip);
    fn irq_set_chip_data(virq: u32, data: *mut irq_sim_irq_ctx);
    fn irq_set_handler(virq: u32, handler: Option<unsafe extern "C" fn(*mut irq_desc)>);
    fn irq_modify_status(virq: u32, clear: u32, set: u32);
    fn irq_domain_get_irq_data(domain: *mut irq_domain, virq: u32) -> *mut irq_data;
    fn irq_domain_reset_irq_data(data: *mut irq_data);
    fn irq_domain_create_linear(fwnode: *mut fwnode_handle, num_irqs: u32, ops: *const irq_domain_ops, host_data: *mut irq_sim_work_ctx) -> *mut irq_domain;
    fn irq_work_sync(work: *mut irq_work);
    fn bitmap_free(bitmap: *mut usize);
    fn irq_domain_remove(domain: *mut irq_domain);
    fn devm_add_action_or_reset(dev: *mut device, action: unsafe extern "C" fn(*mut c_void), data: *mut c_void) -> i32;
}

static mut IRQ_SIM_IRQCHIP: irq_chip = irq_chip { _private: [] };
static IRQ_SIM_DOMAIN_OPS: irq_domain_ops = irq_domain_ops { _private: [] };

unsafe extern "C" fn irq_sim_irqmask(data: *mut irq_data) {
    let irq_ctx = irq_data_get_irq_chip_data(data) as *mut irq_sim_irq_ctx;
    (*irq_ctx).enabled = false;
}

unsafe extern "C" fn irq_sim_irqunmask(data: *mut irq_data) {
    let irq_ctx = irq_data_get_irq_chip_data(data) as *mut irq_sim_irq_ctx;
    (*irq_ctx).enabled = true;
}

unsafe extern "C" fn irq_sim_set_type(data: *mut irq_data, ty: u32) -> i32 {
    if ty & !IRQ_TYPE_EDGE_BOTH != 0 { return -EINVAL; }
    irqd_set_trigger_type(data, ty);
    0
}

unsafe extern "C" fn irq_sim_get_irqchip_state(data: *mut irq_data, which: i32, state: *mut bool) -> i32 {
    let irq_ctx = irq_data_get_irq_chip_data(data) as *mut irq_sim_irq_ctx;
    let hwirq = irqd_to_hwirq(data);
    match which {
        IRQCHIP_STATE_PENDING => if (*irq_ctx).enabled { *state = test_bit(hwirq, (*(*irq_ctx).work_ctx).pending); },
        _ => return -EINVAL,
    }
    0
}

unsafe extern "C" fn irq_sim_set_irqchip_state(data: *mut irq_data, which: i32, state: bool) -> i32 {
    let irq_ctx = irq_data_get_irq_chip_data(data) as *mut irq_sim_irq_ctx;
    let hwirq = irqd_to_hwirq(data);
    match which {
        IRQCHIP_STATE_PENDING => if (*irq_ctx).enabled { assign_bit(hwirq, (*(*irq_ctx).work_ctx).pending, state); if state { irq_work_queue(&mut (*(*irq_ctx).work_ctx).work); } },
        _ => return -EINVAL,
    }
    0
}

unsafe extern "C" fn irq_sim_request_resources(data: *mut irq_data) -> i32 {
    let irq_ctx = irq_data_get_irq_chip_data(data) as *mut irq_sim_irq_ctx;
    let work_ctx = (*irq_ctx).work_ctx;
    let hwirq = irqd_to_hwirq(data);
    if let Some(f) = (*work_ctx).ops.irq_sim_irq_requested { return f((*work_ctx).domain, hwirq, (*work_ctx).user_data); }
    0
}

unsafe extern "C" fn irq_sim_release_resources(data: *mut irq_data) {
    let irq_ctx = irq_data_get_irq_chip_data(data) as *mut irq_sim_irq_ctx;
    let work_ctx = (*irq_ctx).work_ctx;
    let hwirq = irqd_to_hwirq(data);
    if let Some(f) = (*work_ctx).ops.irq_sim_irq_released { f((*work_ctx).domain, hwirq, (*work_ctx).user_data); }
}

unsafe extern "C" fn irq_sim_handle_irq(work: *mut irq_work) {
    let work_ctx = (work as *mut u8).sub(core::mem::offset_of!(irq_sim_work_ctx, work)) as *mut irq_sim_work_ctx;
    let mut offset = 0u32;
    while !bitmap_empty((*work_ctx).pending, (*work_ctx).irq_count) {
        offset = find_next_bit((*work_ctx).pending, (*work_ctx).irq_count, offset);
        clear_bit(offset, (*work_ctx).pending);
        let irqnum = irq_find_mapping((*work_ctx).domain, offset);
        handle_simple_irq(irq_to_desc(irqnum));
    }
}

unsafe extern "C" fn devm_irq_domain_remove_sim(data: *mut c_void) {
    irq_domain_remove_sim(data as *mut irq_domain);
}

pub unsafe extern "C" fn irq_domain_create_sim(fwnode: *mut fwnode_handle, num_irqs: u32) -> *mut irq_domain {
    irq_domain_create_sim_full(fwnode, num_irqs, core::ptr::null(), core::ptr::null_mut())
}

pub unsafe extern "C" fn irq_domain_create_sim_full(fwnode: *mut fwnode_handle, num_irqs: u32, ops: *const irq_sim_ops, data: *mut c_void) -> *mut irq_domain {
    let work_ctx = Box::into_raw(Box::new(core::mem::zeroed::<irq_sim_work_ctx>()));
    if work_ctx.is_null() { return (-ENOMEM) as isize as *mut irq_domain; }
    let domain = irq_domain_create_linear(fwnode, num_irqs, &IRQ_SIM_DOMAIN_OPS, work_ctx);
    if domain.is_null() { return (-ENOMEM) as isize as *mut irq_domain; }
    (*work_ctx).domain = domain;
    (*work_ctx).irq_count = num_irqs;
    (*work_ctx).pending = core::ptr::null_mut();
    (*work_ctx).user_data = data;
    if !ops.is_null() { (*work_ctx).ops = *ops; }
    domain
}

pub unsafe extern "C" fn irq_domain_remove_sim(domain: *mut irq_domain) {
    let work_ctx = (*domain).host_data as *mut irq_sim_work_ctx;
    irq_work_sync(&mut (*work_ctx).work);
    bitmap_free((*work_ctx).pending);
    drop(Box::from_raw(work_ctx));
    irq_domain_remove(domain);
}

pub unsafe extern "C" fn devm_irq_domain_create_sim(dev: *mut device, fwnode: *mut fwnode_handle, num_irqs: u32) -> *mut irq_domain {
    devm_irq_domain_create_sim_full(dev, fwnode, num_irqs, core::ptr::null(), core::ptr::null_mut())
}

pub unsafe extern "C" fn devm_irq_domain_create_sim_full(dev: *mut device, fwnode: *mut fwnode_handle, num_irqs: u32, ops: *const irq_sim_ops, data: *mut c_void) -> *mut irq_domain {
    let domain = irq_domain_create_sim_full(fwnode, num_irqs, ops, data);
    if domain.is_null() { return domain; }
    let ret = devm_add_action_or_reset(dev, devm_irq_domain_remove_sim, domain as *mut c_void);
    if ret != 0 { return (ret as isize) as *mut irq_domain; }
    domain
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
