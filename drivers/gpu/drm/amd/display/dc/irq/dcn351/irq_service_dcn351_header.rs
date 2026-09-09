/* SPDX-License-Identifier: MIT */
/* Copyright 2021 Advanced Micro Devices, Inc. */

// Translated from irq_service_dcn351.h.
// Dependency: ../irq_service.h

/// Opaque C type declared by the irq_service dependency.
#[repr(C)]
pub struct irq_service {
    _private: [u8; 0],
}

/// Opaque C type declared by the irq_service dependency.
#[repr(C)]
pub struct irq_service_init_data {
    _private: [u8; 0],
}

extern "C" {
    pub fn dal_irq_service_dcn351_create(
        init_data: *mut irq_service_init_data,
    ) -> *mut irq_service;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
