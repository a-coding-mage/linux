// SPDX-License-Identifier: MIT
//
// Copyright 2024-2025 Advanced Micro Devices, Inc.

// Dependency supplied by dml2_internal_shared_types.h.

/// Declaration of `dml2_core_dcn5_funcs_validate_solution`.
///
/// The referenced types are supplied by the translated shared type
/// declarations.
pub(crate) unsafe extern "C" {
    pub(crate) fn dml2_core_dcn5_funcs_validate_solution(
        core: *mut crate::dml2_core_instance,
        solution: *const crate::dml2_display_solution,
        result: *mut crate::dml2_validation_result,
    ) -> crate::dml2_status;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
