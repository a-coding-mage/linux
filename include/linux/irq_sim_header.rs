/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright (C) 2017-2018 Bartosz Golaszewski <brgl@bgdev.pl>
 * Copyright (C) 2020 Bartosz Golaszewski <bgolaszewski@baylibre.com>
 */

/*
 * Provides a framework for allocating simulated interrupts which can be
 * requested like normal irqs and enqueued from process context.
 */

// External types supplied by the corresponding Linux interfaces.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct irq_domain {
    _private: [u8; 0],
}

pub type irq_hw_number_t = u64;

#[repr(C)]
pub struct irq_sim_ops {
    pub irq_sim_irq_requested: Option<
        unsafe extern "C" fn(
            domain: *mut irq_domain,
            hwirq: irq_hw_number_t,
            data: *mut core::ffi::c_void,
        ) -> core::ffi::c_int,
    >,
    pub irq_sim_irq_released: Option<
        unsafe extern "C" fn(
            domain: *mut irq_domain,
            hwirq: irq_hw_number_t,
            data: *mut core::ffi::c_void,
        ),
    >,
}

unsafe extern "C" {
    pub fn irq_domain_create_sim(
        fwnode: *mut fwnode_handle,
        num_irqs: core::ffi::c_uint,
    ) -> *mut irq_domain;

    pub fn devm_irq_domain_create_sim(
        dev: *mut device,
        fwnode: *mut fwnode_handle,
        num_irqs: core::ffi::c_uint,
    ) -> *mut irq_domain;

    pub fn irq_domain_create_sim_full(
        fwnode: *mut fwnode_handle,
        num_irqs: core::ffi::c_uint,
        ops: *const irq_sim_ops,
        data: *mut core::ffi::c_void,
    ) -> *mut irq_domain;

    pub fn devm_irq_domain_create_sim_full(
        dev: *mut device,
        fwnode: *mut fwnode_handle,
        num_irqs: core::ffi::c_uint,
        ops: *const irq_sim_ops,
        data: *mut core::ffi::c_void,
    ) -> *mut irq_domain;

    pub fn irq_domain_remove_sim(domain: *mut irq_domain);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
