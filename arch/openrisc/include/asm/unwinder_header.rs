/*
 * OpenRISC unwinder.h
 *
 * Architecture API for unwinding stacks.
 *
 * Copyright (C) 2017 Stafford Horne <shorne@gmail.com>
 *
 * This file is licensed under the terms of the GNU General Public License
 * version 2.  This program is licensed "as is" without any warranty of
 * any kind, whether express or implied.
 */

// Original header guard: __ASM_OPENRISC_UNWINDER_H

extern "C" {
    pub fn unwind_stack(
        data: *mut core::ffi::c_void,
        stack: *mut core::ffi::c_ulong,
        trace: Option<unsafe extern "C" fn(
            data: *mut core::ffi::c_void,
            addr: core::ffi::c_ulong,
            reliable: core::ffi::c_int,
        )>,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
