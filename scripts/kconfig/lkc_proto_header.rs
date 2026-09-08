/* SPDX-License-Identifier: GPL-2.0 */

// Declarations translated from lkc_proto.h.

extern "C" {
    /* confdata.c */
    pub fn conf_parse(name: *const ::std::os::raw::c_char);
    pub fn conf_read(name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn conf_read_simple(name: *const ::std::os::raw::c_char, arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn conf_write_defconfig(name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn conf_write(name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn conf_write_autoconf(overwrite: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn conf_set_changed(val: bool);
    pub fn conf_get_changed() -> bool;
    pub fn conf_set_changed_callback(fn_: Option<unsafe extern "C" fn(bool)>);
    pub fn conf_set_message_callback(fn_: Option<unsafe extern "C" fn(*const ::std::os::raw::c_char)>);
    pub fn conf_errors() -> bool;

    /* symbol.c */
    pub fn sym_lookup(name: *const ::std::os::raw::c_char, flags: ::std::os::raw::c_int) -> *mut symbol;
    pub fn sym_find(name: *const ::std::os::raw::c_char) -> *mut symbol;
    pub fn print_symbol_for_listconfig(sym: *mut symbol);
    pub fn sym_re_search(pattern: *const ::std::os::raw::c_char) -> *mut *mut symbol;
    pub fn sym_type_name(type_: symbol_type) -> *const ::std::os::raw::c_char;
    pub fn sym_calc_value(sym: *mut symbol);
    pub fn sym_dep_errors() -> bool;
    pub fn sym_get_type(sym: *const symbol) -> symbol_type;
    pub fn sym_tristate_within_range(sym: *const symbol, tri: tristate) -> bool;
    pub fn sym_set_tristate_value(sym: *mut symbol, tri: tristate) -> bool;
    pub fn choice_set_value(choice: *mut menu, sym: *mut symbol);
    pub fn sym_toggle_tristate_value(sym: *mut symbol) -> tristate;
    pub fn sym_string_valid(sym: *mut symbol, newval: *const ::std::os::raw::c_char) -> bool;
    pub fn sym_string_within_range(sym: *mut symbol, str_: *const ::std::os::raw::c_char) -> bool;
    pub fn sym_set_string_value(sym: *mut symbol, newval: *const ::std::os::raw::c_char) -> bool;
    pub fn sym_is_changeable(sym: *const symbol) -> bool;
    pub fn sym_get_prompt_menu(sym: *const symbol) -> *mut menu;
    pub fn sym_get_choice_menu(sym: *const symbol) -> *mut menu;
    pub fn sym_get_string_value(sym: *mut symbol) -> *const ::std::os::raw::c_char;

    pub fn prop_get_type_name(type_: prop_type) -> *const ::std::os::raw::c_char;

    /* expr.c */
    pub fn expr_print(
        e: *const expr,
        fn_: Option<unsafe extern "C" fn(*mut ::std::ffi::c_void, *mut symbol, *const ::std::os::raw::c_char)>,
        data: *mut ::std::ffi::c_void,
        prevtoken: ::std::os::raw::c_int,
    );
}

// `symbol`, `menu`, `expr`, `symbol_type`, `prop_type`, and `tristate` are
// supplied by the corresponding translated dependencies.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
