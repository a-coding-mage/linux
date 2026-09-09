// SPDX-License-Identifier: GPL-2.0

extern "C" {
    fn epapr_platform_init(
        r3: core::ffi::c_ulong,
        r4: core::ffi::c_ulong,
        r5: core::ffi::c_ulong,
        r6: core::ffi::c_ulong,
        r7: core::ffi::c_ulong,
    );
}

pub extern "C" fn platform_init(
    r3: core::ffi::c_ulong,
    r4: core::ffi::c_ulong,
    r5: core::ffi::c_ulong,
    r6: core::ffi::c_ulong,
    r7: core::ffi::c_ulong,
) {
    unsafe {
        epapr_platform_init(r3, r4, r5, r6, r7);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
