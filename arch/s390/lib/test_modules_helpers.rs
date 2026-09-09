// SPDX-License-Identifier: GPL-2.0+

// The Linux export declarations and `REPEAT_10000` macro are supplied by
// external dependencies in the surrounding translation unit.

macro_rules! define_return {
    ($i:literal) => {
        #[no_mangle]
        pub extern "C" fn test_modules_return_$i() -> ::core::ffi::c_int {
            1$i - 10000
        }
    };
}

REPEAT_10000!(define_return);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
