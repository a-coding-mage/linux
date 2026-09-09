// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2023 WANG Xuerui <git@xen0n.name>
 *
 * Template for XOR operations, instantiated in xor_simd.c.
 *
 * Expected build-time definitions:
 *
 * - LINE_WIDTH
 * - XOR_FUNC_NAME(nr)
 * - LD_INOUT_LINE(buf)
 * - LD_AND_XOR_LINE(buf)
 * - ST_LINE(buf)
 *
 * The C preprocessor-generated function names are represented here by the
 * corresponding numbered Rust functions.  The architecture-specific inline
 * assembly macros remain external build-time dependencies.
 */

#[inline(never)]
pub unsafe extern "C" fn xor_func_name_2(
    bytes: usize,
    mut v1: *mut usize,
    mut v2: *const usize,
) {
    let mut lines = bytes / LINE_WIDTH;

    loop {
        // LD_INOUT_LINE(v1), LD_AND_XOR_LINE(v2), ST_LINE(v1)
        // TODO: expand the architecture-specific inline assembly macros.

        v1 = v1.add(LINE_WIDTH / core::mem::size_of::<usize>());
        v2 = v2.add(LINE_WIDTH / core::mem::size_of::<usize>());
        lines = lines.wrapping_sub(1);
        if lines == 0 {
            break;
        }
    }
}

#[inline(never)]
pub unsafe extern "C" fn xor_func_name_3(
    bytes: usize,
    mut v1: *mut usize,
    mut v2: *const usize,
    mut v3: *const usize,
) {
    let mut lines = bytes / LINE_WIDTH;

    loop {
        // LD_INOUT_LINE(v1), LD_AND_XOR_LINE(v2),
        // LD_AND_XOR_LINE(v3), ST_LINE(v1)
        // TODO: expand the architecture-specific inline assembly macros.

        v1 = v1.add(LINE_WIDTH / core::mem::size_of::<usize>());
        v2 = v2.add(LINE_WIDTH / core::mem::size_of::<usize>());
        v3 = v3.add(LINE_WIDTH / core::mem::size_of::<usize>());
        lines = lines.wrapping_sub(1);
        if lines == 0 {
            break;
        }
    }
}

#[inline(never)]
pub unsafe extern "C" fn xor_func_name_4(
    bytes: usize,
    mut v1: *mut usize,
    mut v2: *const usize,
    mut v3: *const usize,
    mut v4: *const usize,
) {
    let mut lines = bytes / LINE_WIDTH;

    loop {
        // LD_INOUT_LINE(v1), LD_AND_XOR_LINE(v2),
        // LD_AND_XOR_LINE(v3), LD_AND_XOR_LINE(v4), ST_LINE(v1)
        // TODO: expand the architecture-specific inline assembly macros.

        v1 = v1.add(LINE_WIDTH / core::mem::size_of::<usize>());
        v2 = v2.add(LINE_WIDTH / core::mem::size_of::<usize>());
        v3 = v3.add(LINE_WIDTH / core::mem::size_of::<usize>());
        v4 = v4.add(LINE_WIDTH / core::mem::size_of::<usize>());
        lines = lines.wrapping_sub(1);
        if lines == 0 {
            break;
        }
    }
}

#[inline(never)]
pub unsafe extern "C" fn xor_func_name_5(
    bytes: usize,
    mut v1: *mut usize,
    mut v2: *const usize,
    mut v3: *const usize,
    mut v4: *const usize,
    mut v5: *const usize,
) {
    let mut lines = bytes / LINE_WIDTH;

    loop {
        // LD_INOUT_LINE(v1), LD_AND_XOR_LINE(v2),
        // LD_AND_XOR_LINE(v3), LD_AND_XOR_LINE(v4),
        // LD_AND_XOR_LINE(v5), ST_LINE(v1)
        // TODO: expand the architecture-specific inline assembly macros.

        v1 = v1.add(LINE_WIDTH / core::mem::size_of::<usize>());
        v2 = v2.add(LINE_WIDTH / core::mem::size_of::<usize>());
        v3 = v3.add(LINE_WIDTH / core::mem::size_of::<usize>());
        v4 = v4.add(LINE_WIDTH / core::mem::size_of::<usize>());
        v5 = v5.add(LINE_WIDTH / core::mem::size_of::<usize>());
        lines = lines.wrapping_sub(1);
        if lines == 0 {
            break;
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
