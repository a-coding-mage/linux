/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2008 Nir Tzachar <nir.tzachar@gmail.com>
 *
 * Derived from menuconfig.
 */

// C header dependencies are supplied by the surrounding translation unit.

macro_rules! max {
    ($a:expr, $b:expr) => {{
        let _a = $a;
        let _b = $b;
        if _a > _b { _a } else { _b }
    }};
}

macro_rules! min {
    ($a:expr, $b:expr) => {{
        let _a = $a;
        let _b = $b;
        if _a < _b { _a } else { _b }
    }};
}

extern "C" {
    pub static mut attr_normal: ::std::os::raw::c_int;
    pub static mut attr_main_heading: ::std::os::raw::c_int;
    pub static mut attr_main_menu_box: ::std::os::raw::c_int;
    pub static mut attr_main_menu_fore: ::std::os::raw::c_int;
    pub static mut attr_main_menu_back: ::std::os::raw::c_int;
    pub static mut attr_main_menu_grey: ::std::os::raw::c_int;
    pub static mut attr_main_menu_heading: ::std::os::raw::c_int;
    pub static mut attr_scrollwin_text: ::std::os::raw::c_int;
    pub static mut attr_scrollwin_heading: ::std::os::raw::c_int;
    pub static mut attr_scrollwin_box: ::std::os::raw::c_int;
    pub static mut attr_dialog_text: ::std::os::raw::c_int;
    pub static mut attr_dialog_menu_fore: ::std::os::raw::c_int;
    pub static mut attr_dialog_menu_back: ::std::os::raw::c_int;
    pub static mut attr_dialog_box: ::std::os::raw::c_int;
    pub static mut attr_input_box: ::std::os::raw::c_int;
    pub static mut attr_input_heading: ::std::os::raw::c_int;
    pub static mut attr_input_text: ::std::os::raw::c_int;
    pub static mut attr_input_field: ::std::os::raw::c_int;
    pub static mut attr_function_text: ::std::os::raw::c_int;
    pub static mut attr_function_highlight: ::std::os::raw::c_int;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum function_key {
    F_HELP = 1,
    F_SYMBOL = 2,
    F_INSTS = 3,
    F_CONF = 4,
    F_BACK = 5,
    F_SAVE = 6,
    F_LOAD = 7,
    F_SEARCH = 8,
    F_EXIT = 9,
}

// External C library types and functions are supplied by the surrounding translation unit.
pub type WINDOW = ::std::os::raw::c_void;

pub type extra_key_cb_fn = Option<unsafe extern "C" fn(
    ::std::os::raw::c_int,
    usize,
    usize,
    *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int>;

extern "C" {
    pub fn set_colors();

    /* this changes the windows attributes !!! */
    pub fn print_in_middle(
        win: *mut WINDOW,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        str_: *const ::std::os::raw::c_char,
        attrs: ::std::os::raw::c_int,
    );
    pub fn get_line_length(line: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn get_line_no(text: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn get_line(
        text: *const ::std::os::raw::c_char,
        line_no: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
    pub fn fill_window(win: *mut WINDOW, text: *const ::std::os::raw::c_char);
    pub fn btn_dialog(
        main_window: *mut WINDOW,
        msg: *const ::std::os::raw::c_char,
        btn_num: ::std::os::raw::c_int,
        ...,
    ) -> ::std::os::raw::c_int;
    pub fn dialog_inputbox(
        main_window: *mut WINDOW,
        title: *const ::std::os::raw::c_char,
        prompt: *const ::std::os::raw::c_char,
        init: *const ::std::os::raw::c_char,
        resultp: *mut *mut ::std::os::raw::c_char,
        result_len: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn refresh_all_windows(main_window: *mut WINDOW);
    pub fn show_scroll_win_ext(
        main_window: *mut WINDOW,
        title: *const ::std::os::raw::c_char,
        text: *mut ::std::os::raw::c_char,
        vscroll: *mut ::std::os::raw::c_int,
        hscroll: *mut ::std::os::raw::c_int,
        extra_key_cb: extra_key_cb_fn,
        data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn show_scroll_win(
        main_window: *mut WINDOW,
        title: *const ::std::os::raw::c_char,
        text: *const ::std::os::raw::c_char,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
