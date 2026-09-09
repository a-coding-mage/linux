// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Multiplex several virtual IPIs over a single HW IPI.
 *
 * Copyright The Asahi Linux Contributors
 * Copyright (c) 2022 Ventana Micro Systems Inc.
 */

// Linux kernel dependencies and build-time definitions are supplied externally.

#[repr(C)]
struct ipi_mux_cpu {
    enable: atomic_t,
    bits: atomic_t,
}

// External kernel types, functions, constants, and macros are supplied by other files.
type atomic_t = ::core::ffi::c_int;
type irq_hw_number_t = ::core::ffi::c_ulong;
#[repr(C)] struct irq_data { _private: [u8; 0] }
#[repr(C)] struct cpumask { _private: [u8; 0] }
#[repr(C)] struct irq_domain { flags: ::core::ffi::c_ulong }
#[repr(C)] struct fwnode_handle { _private: [u8; 0] }
#[repr(C)] struct irq_chip { _private: [u8; 0] }
#[repr(C)] struct irq_domain_ops { _private: [u8; 0] }

static mut ipi_mux_pcpu: *mut ipi_mux_cpu = core::ptr::null_mut();
static mut ipi_mux_domain: *mut irq_domain = core::ptr::null_mut();
static mut ipi_mux_send: Option<unsafe extern "C" fn(cpu: ::core::ffi::c_uint)> = None;

unsafe fn ipi_mux_mask(d: *mut irq_data) {
    let icpu = this_cpu_ptr(ipi_mux_pcpu);
    atomic_andnot(BIT(irqd_to_hwirq(d)), &mut (*icpu).enable);
}

unsafe fn ipi_mux_unmask(d: *mut irq_data) {
    let icpu = this_cpu_ptr(ipi_mux_pcpu);
    let ibit: u32 = BIT(irqd_to_hwirq(d));

    atomic_or(ibit, &mut (*icpu).enable);

    // The atomic_or() above must complete before the atomic_read() below to avoid racing ipi_mux_send_mask().
    smp_mb__after_atomic();

    // If a pending IPI was unmasked, raise a parent IPI immediately.
    if atomic_read(&(*icpu).bits) & ibit != 0 {
        if let Some(send) = ipi_mux_send { send(smp_processor_id()); }
    }
}

unsafe fn ipi_mux_send_mask(d: *mut irq_data, mask: *const cpumask) {
    let mut icpu = this_cpu_ptr(ipi_mux_pcpu);
    let ibit: u32 = BIT(irqd_to_hwirq(d));
    let mut pending: ::core::ffi::c_ulong;
    let mut cpu: ::core::ffi::c_int;

    for_each_cpu!(cpu, mask, {
        icpu = per_cpu_ptr(ipi_mux_pcpu, cpu);
        // This sequence is the mirror of the one in ipi_mux_unmask();
        // see the comment there. Additionally, release semantics ensure that the vIPI flag set is ordered after any shared
        // memory accesses that precede it. This therefore also pairs with the atomic_fetch_andnot in ipi_mux_process().
        pending = atomic_fetch_or_release(ibit, &mut (*icpu).bits);
        // The atomic_fetch_or_release() above must complete before the atomic_read() below to avoid racing with ipi_mux_unmask().
        smp_mb__after_atomic();
        // The flag writes must complete before the physical IPI is issued to another CPU.
        if pending & ibit as ::core::ffi::c_ulong == 0 && atomic_read(&(*icpu).enable) as u32 & ibit != 0 {
            if let Some(send) = ipi_mux_send { send(cpu as u32); }
        }
    });
}

static ipi_mux_chip: irq_chip = irq_chip { _private: [] };
static ipi_mux_domain_ops: irq_domain_ops = irq_domain_ops { _private: [] };

unsafe fn ipi_mux_domain_alloc(d: *mut irq_domain, virq: u32, nr_irqs: u32, _arg: *mut ::core::ffi::c_void) -> i32 {
    for i in 0..nr_irqs {
        irq_set_percpu_devid(virq + i);
        irq_domain_set_info(d, virq + i, i, &ipi_mux_chip, core::ptr::null_mut(), handle_percpu_devid_irq, core::ptr::null_mut(), core::ptr::null_mut());
    }
    0
}

/// Process multiplexed virtual IPIs
pub unsafe fn ipi_mux_process() {
    let icpu = this_cpu_ptr(ipi_mux_pcpu);
    let mut hwirq: irq_hw_number_t = 0;
    let mut ipis: ::core::ffi::c_ulong;
    let en: u32 = atomic_read(&(*icpu).enable) as u32;
    ipis = atomic_fetch_andnot(en, &mut (*icpu).bits) as ::core::ffi::c_ulong & en as ::core::ffi::c_ulong;
    for_each_set_bit!(hwirq, &ipis, BITS_PER_TYPE_INT, {
        generic_handle_domain_irq(ipi_mux_domain, hwirq);
    });
}

/// Create virtual IPIs multiplexed on top of a single parent IPI.
pub unsafe fn ipi_mux_create(nr_ipi: u32, mux_send: Option<unsafe extern "C" fn(u32)>) -> i32 {
    let mut fwnode: *mut fwnode_handle;
    let mut domain: *mut irq_domain;
    let mut rc: i32;
    if !ipi_mux_domain.is_null() { return -EEXIST; }
    if BITS_PER_TYPE_INT < nr_ipi || mux_send.is_none() { return -EINVAL; }
    ipi_mux_pcpu = alloc_percpu();
    if ipi_mux_pcpu.is_null() { return -ENOMEM; }
    fwnode = irq_domain_alloc_named_fwnode(c"IPI-Mux".as_ptr());
    if fwnode.is_null() { pr_err!("unable to create IPI Mux fwnode\n"); rc = -ENOMEM; goto fail_free_cpu; }
    domain = irq_domain_create_linear(fwnode, nr_ipi, &ipi_mux_domain_ops, core::ptr::null_mut());
    if domain.is_null() { pr_err!("unable to add IPI Mux domain\n"); rc = -ENOMEM; goto fail_free_fwnode; }
    (*domain).flags |= IRQ_DOMAIN_FLAG_IPI_SINGLE;
    irq_domain_update_bus_token(domain, DOMAIN_BUS_IPI);
    rc = irq_domain_alloc_irqs(domain, nr_ipi, NUMA_NO_NODE, core::ptr::null_mut());
    if rc <= 0 { pr_err!("unable to alloc IRQs from IPI Mux domain\n"); goto fail_free_domain; }
    ipi_mux_domain = domain;
    ipi_mux_send = mux_send;
    return rc;
fail_free_domain: irq_domain_remove(domain);
fail_free_fwnode: irq_domain_free_fwnode(fwnode);
fail_free_cpu: free_percpu(ipi_mux_pcpu); rc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
