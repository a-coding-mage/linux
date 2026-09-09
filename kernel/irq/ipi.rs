// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2015 Imagination Technologies Ltd
 * Author: Qais Yousef <qais.yousef@imgtec.com>
 *
 * This file contains driver APIs to the IPI subsystem.
 */

use core::ffi::c_void;

pub type c_int = i32;
pub type c_uint = u32;
pub type irq_hw_number_t = u64;

pub const EINVAL: c_int = 22;
pub const ENOMEM: c_int = 12;
pub const EBUSY: c_int = 16;
pub const NUMA_NO_NODE: c_int = -1;
pub const INVALID_HWIRQ: irq_hw_number_t = !0;
pub const IRQ_NO_BALANCING: c_uint = 0x0004;

#[repr(C)]
pub struct cpumask {
    _private: [u8; 0],
}

#[repr(C)]
pub struct irq_domain {
    _private: [u8; 0],
}

#[repr(C)]
pub struct irq_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct irq_common_data {
    pub affinity: *mut cpumask,
    pub ipi_offset: c_uint,
}

#[repr(C)]
pub struct irq_data {
    pub irq: c_uint,
    pub common: *mut irq_common_data,
    pub domain: *mut irq_domain,
}

#[repr(C)]
pub struct irq_chip {
    pub ipi_send_single: Option<unsafe extern "C" fn(*mut irq_data, c_uint)>,
    pub ipi_send_mask: Option<unsafe extern "C" fn(*mut irq_data, *const cpumask)>,
}

extern "C" {
    static cpu_possible_mask: *const cpumask;
    static nr_cpu_ids: c_uint;

    fn irq_domain_is_ipi(domain: *mut irq_domain) -> bool;
    fn irq_domain_is_ipi_single(domain: *mut irq_domain) -> bool;
    fn irq_domain_is_ipi_per_cpu(domain: *mut irq_domain) -> bool;
    fn cpumask_subset(src: *const cpumask, dst: *const cpumask) -> bool;
    fn cpumask_weight(mask: *const cpumask) -> c_uint;
    fn cpumask_first(mask: *const cpumask) -> c_uint;
    fn cpumask_next_zero(n: c_uint, mask: *const cpumask) -> c_uint;
    fn cpumask_next(n: c_uint, mask: *const cpumask) -> c_uint;
    fn cpumask_test_cpu(cpu: c_uint, mask: *const cpumask) -> bool;
    fn cpumask_copy(dst: *mut cpumask, src: *const cpumask);
    fn cpumask_of(cpu: c_uint) -> *const cpumask;
    fn irq_domain_alloc_descs(irq: c_int, nr: c_uint, from: c_uint, node: c_int, arg: *mut c_void) -> c_int;
    fn __irq_domain_alloc_irqs(domain: *mut irq_domain, irq: c_int, nr: c_uint, node: c_int, arg: *mut c_void, nodeg: bool, ops: *mut c_void) -> c_int;
    fn irq_get_irq_data(irq: c_uint) -> *mut irq_data;
    fn irq_set_status_flags(irq: c_uint, flags: c_uint);
    fn irq_free_descs(irq: c_int, nr: c_uint);
    fn irq_domain_free_irqs(irq: c_uint, nr: c_uint);
    fn irq_data_get_affinity_mask(data: *mut irq_data) -> *const cpumask;
    fn irqd_to_hwirq(data: *mut irq_data) -> irq_hw_number_t;
    fn irq_desc_get_irq_data(desc: *mut irq_desc) -> *mut irq_data;
    fn irq_data_get_irq_chip(data: *mut irq_data) -> *mut irq_chip;
    fn irq_to_desc(irq: c_uint) -> *mut irq_desc;
    fn pr_warn(fmt: *const u8, ...);
    fn warn_on(condition: bool) -> bool;
    fn warn_on_once(condition: bool) -> bool;
}

pub unsafe extern "C" fn irq_reserve_ipi(domain: *mut irq_domain, dest: *const cpumask) -> c_int {
    let mut nr_irqs: c_uint;
    let mut offset: c_uint;
    let mut data: *mut irq_data;
    let mut virq: c_int;
    let mut i: c_int;

    if domain.is_null() || !irq_domain_is_ipi(domain) {
        pr_warn(b"Reservation on a non IPI domain\0".as_ptr());
        return -EINVAL;
    }
    if !cpumask_subset(dest, cpu_possible_mask) {
        pr_warn(b"Reservation is not in possible_cpu_mask\0".as_ptr());
        return -EINVAL;
    }
    nr_irqs = cpumask_weight(dest);
    if nr_irqs == 0 {
        pr_warn(b"Reservation for empty destination mask\0".as_ptr());
        return -EINVAL;
    }
    if irq_domain_is_ipi_single(domain) {
        nr_irqs = 1;
        offset = 0;
    } else {
        offset = cpumask_first(dest);
        let mut next = cpumask_next_zero(offset, dest);
        if next < nr_cpu_ids { next = cpumask_next(next, dest); }
        if next < nr_cpu_ids {
            pr_warn(b"Destination mask has holes\0".as_ptr());
            return -EINVAL;
        }
    }
    virq = irq_domain_alloc_descs(-1, nr_irqs, 0, NUMA_NO_NODE, core::ptr::null_mut());
    if virq <= 0 {
        pr_warn(b"Can't reserve IPI, failed to alloc descs\0".as_ptr());
        return -ENOMEM;
    }
    virq = __irq_domain_alloc_irqs(domain, virq, nr_irqs, NUMA_NO_NODE, dest as *mut c_void, true, core::ptr::null_mut());
    if virq <= 0 {
        pr_warn(b"Can't reserve IPI, failed to alloc hw irqs\0".as_ptr());
        irq_free_descs(virq, nr_irqs);
        return -EBUSY;
    }
    i = 0;
    while i < nr_irqs as c_int {
        data = irq_get_irq_data((virq + i) as c_uint);
        cpumask_copy((*data).common.as_mut().unwrap().affinity, dest);
        (*(*data).common).ipi_offset = offset;
        irq_set_status_flags((virq + i) as c_uint, IRQ_NO_BALANCING);
        i += 1;
    }
    virq
}

pub unsafe extern "C" fn irq_destroy_ipi(mut irq: c_uint, dest: *const cpumask) -> c_int {
    let data = irq_get_irq_data(irq);
    if irq == 0 || data.is_null() { return -EINVAL; }
    let domain = (*data).domain;
    if warn_on(domain.is_null()) || !irq_domain_is_ipi(domain) { return -EINVAL; }
    let ipimask = irq_data_get_affinity_mask(data);
    if ipimask.is_null() || warn_on(!cpumask_subset(dest, ipimask)) { return -EINVAL; }
    let nr_irqs;
    if irq_domain_is_ipi_per_cpu(domain) {
        irq = irq + cpumask_first(dest) - (*(*data).common).ipi_offset;
        nr_irqs = cpumask_weight(dest);
    } else { nr_irqs = 1; }
    irq_domain_free_irqs(irq, nr_irqs);
    0
}

pub unsafe extern "C" fn ipi_get_hwirq(irq: c_uint, cpu: c_uint) -> irq_hw_number_t {
    let mut data = irq_get_irq_data(irq);
    if data.is_null() || cpu >= nr_cpu_ids { return INVALID_HWIRQ; }
    let ipimask = irq_data_get_affinity_mask(data);
    if ipimask.is_null() || !cpumask_test_cpu(cpu, ipimask) { return INVALID_HWIRQ; }
    if irq_domain_is_ipi_per_cpu((*data).domain) {
        data = irq_get_irq_data(irq + cpu - (*(*data).common).ipi_offset);
    }
    if data.is_null() { INVALID_HWIRQ } else { irqd_to_hwirq(data) }
}

unsafe fn ipi_send_verify(chip: *mut irq_chip, data: *mut irq_data, dest: *const cpumask, cpu: c_uint) -> c_int {
    if chip.is_null() || data.is_null() { return -EINVAL; }
    if (*chip).ipi_send_single.is_none() && (*chip).ipi_send_mask.is_none() { return -EINVAL; }
    if cpu >= nr_cpu_ids { return -EINVAL; }
    let ipimask = irq_data_get_affinity_mask(data);
    if ipimask.is_null() { return -EINVAL; }
    if !dest.is_null() { if !cpumask_subset(dest, ipimask) { return -EINVAL; } }
    else if !cpumask_test_cpu(cpu, ipimask) { return -EINVAL; }
    0
}

pub unsafe extern "C" fn __ipi_send_single(desc: *mut irq_desc, cpu: c_uint) -> c_int {
    let mut data = irq_desc_get_irq_data(desc);
    let chip = irq_data_get_irq_chip(data);
    if (*chip).ipi_send_single.is_none() {
        ((*chip).ipi_send_mask.unwrap())(data, cpumask_of(cpu)); return 0;
    }
    if irq_domain_is_ipi_per_cpu((*data).domain) && cpu != (*(*data).common).ipi_offset {
        data = irq_get_irq_data((*data).irq + cpu - (*(*data).common).ipi_offset);
    }
    ((*chip).ipi_send_single.unwrap())(data, cpu); 0
}

pub unsafe extern "C" fn __ipi_send_mask(desc: *mut irq_desc, dest: *const cpumask) -> c_int {
    let mut data = irq_desc_get_irq_data(desc);
    let chip = irq_data_get_irq_chip(data);
    if let Some(send_mask) = (*chip).ipi_send_mask { send_mask(data, dest); return 0; }
    let mut cpu = cpumask_first(dest);
    if irq_domain_is_ipi_per_cpu((*data).domain) {
        let base = (*data).irq;
        while cpu < nr_cpu_ids { let irq = base + cpu - (*(*data).common).ipi_offset; data = irq_get_irq_data(irq); ((*chip).ipi_send_single.unwrap())(data, cpu); cpu = cpumask_next(cpu, dest); }
    } else { while cpu < nr_cpu_ids { ((*chip).ipi_send_single.unwrap())(data, cpu); cpu = cpumask_next(cpu, dest); } }
    0
}

pub unsafe extern "C" fn ipi_send_single(virq: c_uint, cpu: c_uint) -> c_int {
    let desc = irq_to_desc(virq); let data = if desc.is_null() { core::ptr::null_mut() } else { irq_desc_get_irq_data(desc) }; let chip = if data.is_null() { core::ptr::null_mut() } else { irq_data_get_irq_chip(data) };
    if warn_on_once(ipi_send_verify(chip, data, core::ptr::null(), cpu) != 0) { return -EINVAL; } __ipi_send_single(desc, cpu)
}

pub unsafe extern "C" fn ipi_send_mask(virq: c_uint, dest: *const cpumask) -> c_int {
    let desc = irq_to_desc(virq); let data = if desc.is_null() { core::ptr::null_mut() } else { irq_desc_get_irq_data(desc) }; let chip = if data.is_null() { core::ptr::null_mut() } else { irq_data_get_irq_chip(data) };
    if warn_on_once(ipi_send_verify(chip, data, dest, 0) != 0) { return -EINVAL; } __ipi_send_mask(desc, dest)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
