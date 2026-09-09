/*
 * Copyright 2012-16 Advanced Micro Devices, Inc.
 * Copyright 2019 Raptor Engineering, LLC
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 *
 */

// C header guard: _OS_TYPES_H_

// C dependencies:
// linux/slab.h, linux/kgdb.h, linux/delay.h, linux/mm.h, linux/vmalloc.h
// asm/byteorder.h
// drm/display/drm_dp_helper.h, drm/drm_device.h, drm/drm_print.h
// cgs_common.h

// BIGENDIAN_CPU/LITTLEENDIAN_CPU are selected by the target build's
// __BIG_ENDIAN/__LITTLE_ENDIAN configuration.

// C: #undef FRAME_SIZE

/// Equivalent of `dm_output_to_console(fmt, ...)`.
#[macro_export]
macro_rules! dm_output_to_console {
    ($($arg:tt)*) => { DRM_DEBUG_KMS!($($arg)*) };
}

/// Equivalent of `dm_error(fmt, ...)`.
#[macro_export]
macro_rules! dm_error {
    ($($arg:tt)*) => { DRM_ERROR!($($arg)*) };
}

// CONFIG_DRM_AMD_DC_FP conditionally includes amdgpu_dm/dc_fpu.h.

/*
 * On Linux this is provided by <linux/kconfig.h> and evaluates Kconfig
 * options for both built-in (=y) and module (=m) cases. Windows has no
 * Kconfig, so config options are never set here and this always yields 0.
 */
#[macro_export]
macro_rules! IS_ENABLED {
    ($option:ident) => { 0 };
}

/*
 *
 * general debug capabilities
 *
 */
#[cfg(CONFIG_DEBUG_KERNEL_DC)]
#[macro_export]
macro_rules! dc_breakpoint {
    () => { kgdb_breakpoint() };
}

#[cfg(not(CONFIG_DEBUG_KERNEL_DC))]
#[macro_export]
macro_rules! dc_breakpoint {
    () => {{}};
}

#[macro_export]
macro_rules! ASSERT_CRITICAL {
    ($expr:expr) => {{
        if WARN_ON!(!($expr)) {
            dc_breakpoint!();
        }
    }};
}

#[macro_export]
macro_rules! ASSERT {
    ($expr:expr) => {{
        if WARN_ON_ONCE!(!($expr)) {
            dc_breakpoint!();
        }
    }};
}

#[macro_export]
macro_rules! BREAK_TO_DEBUGGER {
    () => {{
        DRM_DEBUG_DRIVER!("{}():{}\n", module_path!(), line!());
        dc_breakpoint!();
    }};
}

#[macro_export]
macro_rules! DC_ERR {
    ($($arg:tt)*) => {{
        dm_error!($($arg)*);
        BREAK_TO_DEBUGGER!();
    }};
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
