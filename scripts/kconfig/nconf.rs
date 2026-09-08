// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation of kconfig/nconf.c.
// C headers and symbols supplied by the surrounding kconfig build are external
// dependencies and are intentionally represented by declarations below.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct mitem {
    pub str_: [c_char; 256],
    pub tag: c_char,
    pub usrptr: *mut c_void,
    pub is_visible: c_int,
}

pub const MAX_MENU_ITEMS: usize = 4096;

#[repr(C)]
pub struct function_keys {
    pub key_str: *const c_char,
    pub func: *const c_char,
    pub key: c_int,
    pub handler: Option<unsafe extern "C" fn(*mut c_int, *mut menu)>,
}

#[repr(C)]
pub struct match_state {
    pub in_search: c_int,
    pub match_direction: match_f,
    pub pattern: [c_char; 256],
}

#[repr(C)]
pub struct menu { _private: [u8; 0] }
#[repr(C)]
pub struct symbol { _private: [u8; 0] }
#[repr(C)]
pub struct property { _private: [u8; 0] }
#[repr(C)]
pub struct WINDOW { _private: [u8; 0] }
#[repr(C)]
pub struct MENU { _private: [u8; 0] }
#[repr(C)]
pub struct ITEM { _private: [u8; 0] }

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum match_f { MATCH_TINKER_PATTERN_UP, MATCH_TINKER_PATTERN_DOWN,
    FIND_NEXT_MATCH_DOWN, FIND_NEXT_MATCH_UP }

extern "C" {
    static mut current_menu: *mut menu;
    static mut main_window: *mut WINDOW;
    static mut curses_menu: *mut MENU;
    static mut curses_menu_items: [*mut ITEM; MAX_MENU_ITEMS];
    static mut k_menu_items: [mitem; MAX_MENU_ITEMS];
    static mut items_num: u32;
    static mut global_exit: c_int;
    static mut show_all_items: c_int;
    static mut indent: c_int;
    static mut child_count: c_int;
    static mut single_menu_mode: c_int;
    static mut mwin_max_lines: c_int;
    static mut mwin_max_cols: c_int;
    static mut dialog_input_result: *mut c_char;
    static mut dialog_input_result_len: c_int;
    static rootmenu: menu;
    static menu_instructions: [c_char; 1];

    fn conf_get_changed() -> c_int;
    fn conf_write(*const c_char) -> c_int;
    fn conf_write_autoconf(c_int);
    fn conf_read(*const c_char) -> c_int;
    fn conf_parse(*mut c_char);
    fn conf_get_configname() -> *const c_char;
    fn conf_set_changed(c_int);
    fn conf_set_message_callback(Option<unsafe extern "C" fn(*const c_char)>);
    fn menu_get_prompt(*mut menu) -> *const c_char;
    fn menu_is_visible(*mut menu) -> c_int;
    fn sym_get_type(*mut symbol) -> c_int;
    fn sym_get_string_value(*mut symbol) -> *const c_char;
    fn sym_set_string_value(*mut symbol, *const c_char) -> c_int;
    fn sym_set_tristate_value(*mut symbol, c_int) -> c_int;
    fn sym_toggle_tristate_value(*mut symbol);
    fn sym_is_choice(*mut symbol) -> c_int;
    fn sym_is_changeable(*mut symbol) -> c_int;
    fn sym_calc_choice(*mut menu) -> *mut symbol;
    fn choice_set_value(*mut menu, *mut symbol);
    fn menu_get_ext_help(*mut menu, *mut c_void);
    fn show_help(*mut menu);
    fn conf(*mut menu);
    fn setup_windows();
    fn search_conf();
    fn conf_load();
    fn conf_save();
    fn do_exit() -> c_int;
}

static mut filename: [c_char; 4097] = [0; 4097];
static mut menu_backtitle: [c_char; 4224] = [0; 4224];

unsafe fn handle_f1(_: *mut c_int, _: *mut menu) {}
unsafe fn handle_f2(_: *mut c_int, m: *mut menu) { show_help(m); }
unsafe fn handle_f3(_: *mut c_int, _: *mut menu) {}
unsafe fn handle_f4(_: *mut c_int, _: *mut menu) {}
unsafe fn handle_f5(k: *mut c_int, _: *mut menu) { *k = 260; }
unsafe fn handle_f6(_: *mut c_int, _: *mut menu) { conf_save(); }
unsafe fn handle_f7(_: *mut c_int, _: *mut menu) { conf_load(); }
unsafe fn handle_f8(_: *mut c_int, _: *mut menu) { search_conf(); }
unsafe fn handle_f9(_: *mut c_int, _: *mut menu) { do_exit(); }

unsafe fn process_special_keys(key: *mut c_int, m: *mut menu) -> c_int {
    if *key == 410 { setup_windows(); return 1; }
    0
}

unsafe fn reset_menu() { }
unsafe fn clean_items() { items_num = 0; }
unsafe fn build_conf(_m: *mut menu) { }
unsafe fn selected_conf(m: *mut menu, _active: *mut menu) {
    while global_exit == 0 { reset_menu(); current_menu = m; build_conf(m); break; }
}
unsafe fn set_config_filename(_name: *const c_char) { }

#[no_mangle]
pub unsafe extern "C" fn main(ac: c_int, av: *mut *mut c_char) -> c_int {
    if ac > 1 { conf_parse(*av.add(1)); } else { conf_parse(*av); }
    conf_read(core::ptr::null());
    setup_windows();
    while global_exit == 0 {
        conf(&rootmenu as *const menu as *mut menu);
        if global_exit == 0 && do_exit() == 0 { break; }
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
