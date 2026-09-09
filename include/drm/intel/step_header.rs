/* SPDX-License-Identifier: MIT */
/* Copyright © 2026 Intel Corporation */

// The C preprocessor helpers are represented as Rust macros.
macro_rules! STEP_ENUM_VAL {
    ($name:ident) => {
        $name,
    };
}

macro_rules! STEP_NAME_LIST {
    ($func:ident) => {
        $func!(STEP_A0)
        $func!(STEP_A1)
        $func!(STEP_A2)
        $func!(STEP_A3)
        $func!(STEP_B0)
        $func!(STEP_B1)
        $func!(STEP_B2)
        $func!(STEP_B3)
        $func!(STEP_C0)
        $func!(STEP_C1)
        $func!(STEP_C2)
        $func!(STEP_C3)
        $func!(STEP_D0)
        $func!(STEP_D1)
        $func!(STEP_D2)
        $func!(STEP_D3)
        $func!(STEP_E0)
        $func!(STEP_E1)
        $func!(STEP_E2)
        $func!(STEP_E3)
        $func!(STEP_F0)
        $func!(STEP_F1)
        $func!(STEP_F2)
        $func!(STEP_F3)
        $func!(STEP_G0)
        $func!(STEP_G1)
        $func!(STEP_G2)
        $func!(STEP_G3)
        $func!(STEP_H0)
        $func!(STEP_H1)
        $func!(STEP_H2)
        $func!(STEP_H3)
        $func!(STEP_I0)
        $func!(STEP_I1)
        $func!(STEP_I2)
        $func!(STEP_I3)
        $func!(STEP_J0)
        $func!(STEP_J1)
        $func!(STEP_J2)
        $func!(STEP_J3)
    };
}

/*
 * Symbolic steppings that do not match the hardware. These are valid both as gt
 * and display steppings as symbolic names.
 */
#[repr(C)]
enum intel_step {
    STEP_NONE = 0,
    STEP_NAME_LIST!(STEP_ENUM_VAL)
    STEP_FUTURE,
    STEP_FOREVER,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
