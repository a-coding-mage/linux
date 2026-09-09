/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Assembler helper macros to generate .byte/.word code for instructions
 * that are unknown to older binutils versions.
 *
 * The original declarations are assembler-only.  These Rust macros preserve
 * their register-number mapping for use by translated low-level code.
 */

/// GR_NUM - Retrieve general-purpose register number.
///
/// `gr` is either a register number or a register designator represented by
/// the corresponding Rust token (`r0` through `r15`).
#[macro_export]
macro_rules! GR_NUM {
    ($r:ident) => {{
        $crate::GR_NUM!(@map $r)
    }};
    (@map r0) => { 0u32 };
    (@map r1) => { 1u32 };
    (@map r2) => { 2u32 };
    (@map r3) => { 3u32 };
    (@map r4) => { 4u32 };
    (@map r5) => { 5u32 };
    (@map r6) => { 6u32 };
    (@map r7) => { 7u32 };
    (@map r8) => { 8u32 };
    (@map r9) => { 9u32 };
    (@map r10) => { 10u32 };
    (@map r11) => { 11u32 };
    (@map r12) => { 12u32 };
    (@map r13) => { 13u32 };
    (@map r14) => { 14u32 };
    (@map r15) => { 15u32 };
}

/// VX_NUM - Retrieve vector register number.
///
/// `vxr` is either a register number or a register designator represented by
/// the corresponding Rust token (`v0` through `v31`).  The result is used as
/// both the instruction input register number and to compute the RXB field.
#[macro_export]
macro_rules! VX_NUM {
    ($r:ident) => {{
        $crate::VX_NUM!(@map $r)
    }};
    (@map v0) => { 0u32 };
    (@map v1) => { 1u32 };
    (@map v2) => { 2u32 };
    (@map v3) => { 3u32 };
    (@map v4) => { 4u32 };
    (@map v5) => { 5u32 };
    (@map v6) => { 6u32 };
    (@map v7) => { 7u32 };
    (@map v8) => { 8u32 };
    (@map v9) => { 9u32 };
    (@map v10) => { 10u32 };
    (@map v11) => { 11u32 };
    (@map v12) => { 12u32 };
    (@map v13) => { 13u32 };
    (@map v14) => { 14u32 };
    (@map v15) => { 15u32 };
    (@map v16) => { 16u32 };
    (@map v17) => { 17u32 };
    (@map v18) => { 18u32 };
    (@map v19) => { 19u32 };
    (@map v20) => { 20u32 };
    (@map v21) => { 21u32 };
    (@map v22) => { 22u32 };
    (@map v23) => { 23u32 };
    (@map v24) => { 24u32 };
    (@map v25) => { 25u32 };
    (@map v26) => { 26u32 };
    (@map v27) => { 27u32 };
    (@map v28) => { 28u32 };
    (@map v29) => { 29u32 };
    (@map v30) => { 30u32 };
    (@map v31) => { 31u32 };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
