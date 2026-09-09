/* SPDX-License-Identifier: GPL-2.0 */

/// External cuboot initialization routine.
extern "C" {
    pub fn cuboot_init(
        r4: ::core::ffi::c_ulong,
        r5: ::core::ffi::c_ulong,
        r6: ::core::ffi::c_ulong,
        r7: ::core::ffi::c_ulong,
        end_of_ram: ::core::ffi::c_ulong,
    );
}

/// CUBOOT_INIT macro translation.
///
/// This macro intentionally refers to the surrounding scope's `bd`, `bd_t`,
/// `r3`, `r4`, `r5`, `r6`, and `r7`, matching the original C macro's implicit
/// inputs and side effects.
#[macro_export]
macro_rules! CUBOOT_INIT {
    () => {{
        unsafe {
            ::core::ptr::copy_nonoverlapping(
                (r3 as *const bd_t),
                &mut bd,
                1,
            );
            cuboot_init(
                r4,
                r5,
                r6,
                r7,
                bd.bi_memstart + bd.bi_memsize,
            );
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
