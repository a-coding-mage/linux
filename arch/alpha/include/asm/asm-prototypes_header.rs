// Translated from asm-prototypes.h.
//
// C dependencies:
// #include <linux/spinlock.h>
// #include <asm/checksum.h>
// #include <asm/console.h>
// #include <asm/page.h>
// #include <asm/string.h>
// #include <linux/uaccess.h>
// #include <asm-generic/asm-prototypes.h>

unsafe extern "C" {
    pub fn __divl();
    pub fn __reml();
    pub fn __divq();
    pub fn __remq();
    pub fn __divlu();
    pub fn __remlu();
    pub fn __divqu();
    pub fn __remqu();
    pub fn __udiv_qrnnd(
        __p: *mut ::core::ffi::c_ulong,
        __n1: ::core::ffi::c_ulong,
        __n0: ::core::ffi::c_ulong,
        __d: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
