// SPDX-License-Identifier: MIT
//
// Copyright 2026 Advanced Micro Devices, Inc.

// Dependency equivalent of: #include "../irq_service.h"

extern "C" {
    pub fn dal_irq_service_dcn60_create(
        init_data: *mut crate::irq_service_init_data,
    ) -> *mut crate::irq_service;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
