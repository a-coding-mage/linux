/* SPDX-License-Identifier: GPL-2.0 */

pub const EOF: i32 = -1;

unsafe extern "C" {
    pub fn xmon_set_pagination_lpp(lpp: ::kernel::ffi::c_ulong);
    pub fn xmon_start_pagination();
    pub fn xmon_end_pagination();
    pub fn xmon_putchar(c: ::kernel::ffi::c_int) -> ::kernel::ffi::c_int;
    pub fn xmon_puts(s: *const ::kernel::ffi::c_char);
    pub fn xmon_gets(s: *mut ::kernel::ffi::c_char, n: ::kernel::ffi::c_int)
        -> *mut ::kernel::ffi::c_char;
    // C printf format attribute: __printf(1, 2)
    pub fn xmon_printf(fmt: *const ::kernel::ffi::c_char, ...) -> ();
}

// #define printf xmon_printf
pub use xmon_printf as printf;

// #define putchar xmon_putchar
pub use xmon_putchar as putchar;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
