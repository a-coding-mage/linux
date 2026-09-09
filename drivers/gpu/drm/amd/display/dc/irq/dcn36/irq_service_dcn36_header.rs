/* SPDX-License-Identifier: MIT */
/* Copyright 2025 Advanced Micro Devices, Inc. */

// Dependency supplied by ../irq_service.h.

#[repr(C)]
pub struct irq_service {
    _private: [u8; 0],
}

#[repr(C)]
pub struct irq_service_init_data {
    _private: [u8; 0],
}

extern "C" {
    pub fn dal_irq_service_dcn36_create(
        init_data: *mut irq_service_init_data,
    ) -> *mut irq_service;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
