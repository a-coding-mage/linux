/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2002 Roman Zippel <zippel@linux-m68k.org>
 */

// C header dependencies: assert, stdio, stdlib, expr.h, and lkc_proto.h.

use core::ffi::{c_char, c_int, c_void};

pub const SRCTREE: &[u8] = b"srctree\0";

// Build-time CONFIG_ macro, represented here by the corresponding function.
pub unsafe fn CONFIG_prefix() -> *const c_char {
    let name = b"CONFIG_\0";
    let value = libc_getenv(name.as_ptr() as *const c_char);
    if value.is_null() { b"CONFIG_\0".as_ptr() as *const c_char } else { value }
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    _private: [u8; 0],
}
#[repr(C)]
pub struct menu {
    _private: [u8; 0],
}
#[repr(C)]
pub struct property {
    _private: [u8; 0],
}
#[repr(C)]
pub struct expr {
    _private: [u8; 0],
}
#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

pub type tristate = c_int;
pub type menu_type = c_int;
pub type prop_type = c_int;

#[repr(C)]
pub struct gstr {
    pub len: usize,
    pub s: *mut c_char,
    /* when max_width is not zero long lines in string s (if any) get
     * wrapped not to exceed the max_width value */
    pub max_width: c_int,
}

extern "C" {
    fn libc_getenv(name: *const c_char) -> *mut c_char;
    fn libc_fwrite(ptr: *const c_void, size: usize, count: usize, stream: *mut FILE) -> usize;
    fn libc_fprintf(stream: *mut FILE, fmt: *const c_char, ...);
    static mut libc_stderr: *mut FILE;

    pub static mut yylineno: c_int;
    pub fn zconfdump(out: *mut FILE);
    pub fn zconf_starthelp();
    pub fn zconf_fopen(name: *const c_char) -> *mut FILE;
    pub fn zconf_initscan(name: *const c_char);
    pub fn zconf_nextfile(name: *const c_char);

    pub static mut autoconf_cmd: gstr;
    pub fn conf_get_configname() -> *const c_char;

    pub fn file_lookup(name: *const c_char, parent_name: *const c_char,
                       parent_lineno: c_int) -> *const c_char;
    pub fn yylex() -> c_int;

    pub fn str_new() -> gstr;
    pub fn str_free(gs: *mut gstr);
    pub fn str_append(gs: *mut gstr, s: *const c_char);
    pub fn str_printf(gs: *mut gstr, fmt: *const c_char, ...);
    pub fn str_get(gs: *const gstr) -> *mut c_char;

    pub fn menu_next(menu: *mut menu, root: *mut menu) -> *mut menu;
    pub fn _menu_init();
    pub fn menu_warn(menu: *const menu, fmt: *const c_char, ...);
    pub fn menu_add_menu() -> *mut menu;
    pub fn menu_end_menu();
    pub fn menu_add_entry(sym: *mut symbol, ty: menu_type);
    pub fn menu_add_dep(dep: *mut expr, cond: *mut expr);
    pub fn menu_add_visibility(dep: *mut expr);
    pub fn menu_add_prompt(ty: prop_type, prompt: *const c_char, dep: *mut expr) -> *mut property;
    pub fn menu_add_expr(ty: prop_type, expr: *mut expr, dep: *mut expr);
    pub fn menu_add_symbol(ty: prop_type, sym: *mut symbol, dep: *mut expr);
    pub fn menu_finalize();
    pub fn menu_set_type(ty: c_int);
    pub static mut rootmenu: menu;
    pub fn menu_is_empty(menu: *mut menu) -> bool;
    pub fn menu_is_visible(menu: *mut menu) -> bool;
    pub fn menu_has_prompt(menu: *const menu) -> bool;
    pub fn menu_get_prompt(menu: *const menu) -> *const c_char;
    pub fn menu_get_parent_menu(menu: *mut menu) -> *mut menu;
    pub fn menu_get_menu_or_parent_menu(menu: *mut menu) -> *mut menu;
    pub fn get_jump_key_char() -> c_int;
    pub fn get_relations_str(sym_arr: *mut *mut symbol, head: *mut list_head) -> gstr;
    pub fn menu_get_ext_help(menu: *mut menu, help: *mut gstr);
    pub fn menu_dump();

    pub fn sym_clear_all_valid();
    pub fn sym_choice_default(choice: *mut menu) -> *mut symbol;
    pub fn sym_calc_choice(choice: *mut menu) -> *mut symbol;
    pub fn sym_get_range_prop(sym: *mut symbol) -> *mut property;
    pub fn sym_get_string_default(sym: *mut symbol) -> *const c_char;
    pub fn sym_check_deps(sym: *mut symbol) -> *mut symbol;
    pub fn prop_get_symbol(prop: *const property) -> *mut symbol;
}

pub unsafe fn xfwrite(str_: *const c_void, len: usize, count: usize, out: *mut FILE) {
    assert!(len != 0);
    if libc_fwrite(str_, len, count, out) != count {
        libc_fprintf(libc_stderr, b"Error in writing or end of file.\n\0".as_ptr() as *const c_char);
    }
}

// The C menu_for_each_* macros are retained as Rust macro equivalents.
#[macro_export]
macro_rules! menu_for_each_sub_entry {
    ($menu:ident, $root:expr, $body:block) => {{
        $menu = unsafe { $crate::menu_next($root, $root) };
        while !$menu.is_null() {
            $body
            $menu = unsafe { $crate::menu_next($menu, $root) };
        }
    }};
}

pub unsafe fn sym_get_tristate_value(_sym: *const symbol) -> tristate {
    // TODO: access sym->curr.tri after the dependent expr.h symbol layout is available.
    unimplemented!()
}

pub unsafe fn sym_is_choice(_sym: *const symbol) -> bool {
    // TODO: access sym->name after the dependent expr.h symbol layout is available.
    unimplemented!()
}

pub fn sym_is_choice_value(_sym: *const symbol) -> bool { unimplemented!() }

pub unsafe fn sym_has_value(_sym: *const symbol) -> bool {
    // TODO: access sym->flags and SYMBOL_DEF_USER from the dependent expr.h.
    unimplemented!()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
