/* SPDX-License-Identifier: GPL-2.0 */

pub type libapi_print_fn_t =
    Option<unsafe extern "C" fn(fmt: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int>;

unsafe extern "C" {
    pub fn libapi_set_print(
        warn: libapi_print_fn_t,
        info: libapi_print_fn_t,
        debug: libapi_print_fn_t,
    );
}
