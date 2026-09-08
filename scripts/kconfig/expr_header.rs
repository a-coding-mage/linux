/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2002 Roman Zippel <zippel@linux-m68k.org>
 */

/* Translated from expr.h. External types are supplied by other headers. */

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tristate {
    no,
    mod_,
    yes,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum expr_type {
    E_NONE,
    E_OR,
    E_AND,
    E_NOT,
    E_EQUAL,
    E_UNEQUAL,
    E_LTH,
    E_LEQ,
    E_GTH,
    E_GEQ,
    E_SYMBOL,
    E_RANGE,
}

#[repr(C)]
pub union expr_data {
    pub expr: *mut expr,
    pub sym: *mut symbol,
    pub _initdata: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct expr {
    pub node: hlist_node,
    pub type_: expr_type,
    pub val: tristate,
    pub val_is_valid: bool,
    pub left: expr_data,
    pub right: expr_data,
}

#[inline]
pub const fn EXPR_OR(dep1: tristate, dep2: tristate) -> tristate {
    if (dep1 as i32) > (dep2 as i32) { dep1 } else { dep2 }
}

#[inline]
pub const fn EXPR_AND(dep1: tristate, dep2: tristate) -> tristate {
    if (dep1 as i32) < (dep2 as i32) { dep1 } else { dep2 }
}

#[inline]
pub const fn EXPR_NOT(dep: tristate) -> tristate {
    match dep {
        tristate::no => tristate::yes,
        tristate::mod_ => tristate::mod_,
        tristate::yes => tristate::no,
    }
}

#[repr(C)]
pub struct expr_value {
    pub expr: *mut expr,
    pub tri: tristate,
}

#[repr(C)]
pub struct symbol_value {
    pub val: *mut core::ffi::c_void,
    pub tri: tristate,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum symbol_type {
    S_UNKNOWN,
    S_BOOLEAN,
    S_TRISTATE,
    S_INT,
    S_HEX,
    S_STRING,
}

pub const S_DEF_USER: usize = 0;
pub const S_DEF_AUTO: usize = 1;
pub const S_DEF_DEF3: usize = 2;
pub const S_DEF_DEF4: usize = 3;
pub const S_DEF_COUNT: usize = 4;

#[repr(C)]
pub struct symbol {
    pub node: hlist_node,
    pub name: *mut core::ffi::c_char,
    pub type_: symbol_type,
    pub curr: symbol_value,
    pub def: [symbol_value; S_DEF_COUNT],
    pub visible: tristate,
    pub menus: list_head,
    pub choice_link: list_head,
    pub flags: i32,
    pub prop: *mut property,
    pub dir_dep: expr_value,
    pub rev_dep: expr_value,
    pub implied: expr_value,
}

pub const SYMBOL_CONST: i32 = 0x0001;
pub const SYMBOL_CHECK: i32 = 0x0008;
pub const SYMBOL_VALID: i32 = 0x0080;
pub const SYMBOL_TRANS: i32 = 0x0100;
pub const SYMBOL_WRITE: i32 = 0x0200;
pub const SYMBOL_WRITTEN: i32 = 0x0800;
pub const SYMBOL_CHECKED: i32 = 0x2000;
pub const SYMBOL_WARNED: i32 = 0x8000;
pub const SYMBOL_DEF: i32 = 0x10000;
pub const SYMBOL_DEF_USER: i32 = 0x10000;
pub const SYMBOL_DEF_AUTO: i32 = 0x20000;
pub const SYMBOL_DEF3: i32 = 0x40000;
pub const SYMBOL_DEF4: i32 = 0x80000;
pub const SYMBOL_MAXLENGTH: usize = 256;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prop_type {
    P_UNKNOWN,
    P_PROMPT,
    P_COMMENT,
    P_MENU,
    P_DEFAULT,
    P_SELECT,
    P_IMPLY,
    P_RANGE,
}

#[repr(C)]
pub struct property {
    pub next: *mut property,
    pub type_: prop_type,
    pub text: *const core::ffi::c_char,
    pub visible: expr_value,
    pub expr: *mut expr,
    pub menu: *mut menu,
    pub filename: *const core::ffi::c_char,
    pub lineno: i32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum menu_type {
    M_CHOICE,
    M_COMMENT,
    M_IF,
    M_MENU,
    M_NORMAL,
}

#[repr(C)]
pub struct menu {
    pub type_: menu_type,
    pub next: *mut menu,
    pub parent: *mut menu,
    pub list: *mut menu,
    pub sym: *mut symbol,
    pub link: list_head,
    pub choice_members: list_head,
    pub prompt: *mut property,
    pub visibility: *mut expr,
    pub dep: *mut expr,
    pub flags: u32,
    pub help: *mut core::ffi::c_char,
    pub filename: *const core::ffi::c_char,
    pub lineno: i32,
    pub data: *mut core::ffi::c_void,
}

pub const MENU_CHANGED: u32 = 0x0001;
pub const MENU_ROOT: u32 = 0x0002;

#[repr(C)]
pub struct jump_key {
    pub entries: list_head,
    pub offset: usize,
    pub target: *mut menu,
}

extern "C" {
    pub static mut symbol_yes: symbol;
    pub static mut symbol_no: symbol;
    pub static mut symbol_mod: symbol;
    pub static mut modules_sym: *mut symbol;
    pub static mut cdebug: i32;

    pub fn expr_alloc_symbol(sym: *mut symbol) -> *mut expr;
    pub fn expr_alloc_one(type_: expr_type, ce: *mut expr) -> *mut expr;
    pub fn expr_alloc_two(type_: expr_type, e1: *mut expr, e2: *mut expr) -> *mut expr;
    pub fn expr_alloc_comp(type_: expr_type, s1: *mut symbol, s2: *mut symbol) -> *mut expr;
    pub fn expr_alloc_and(e1: *mut expr, e2: *mut expr) -> *mut expr;
    pub fn expr_alloc_or(e1: *mut expr, e2: *mut expr) -> *mut expr;
    pub fn expr_eliminate_eq(ep1: *mut *mut expr, ep2: *mut *mut expr);
    pub fn expr_eq(e1: *mut expr, e2: *mut expr) -> bool;
    pub fn expr_calc_value(e: *mut expr) -> tristate;
    pub fn expr_eliminate_dups(e: *mut expr) -> *mut expr;
    pub fn expr_transform(e: *mut expr) -> *mut expr;
    pub fn expr_contains_symbol(dep: *mut expr, sym: *mut symbol) -> bool;
    pub fn expr_contains_symbol_negated(dep: *mut expr, sym: *mut symbol) -> bool;
    pub fn expr_depends_symbol(dep: *mut expr, sym: *mut symbol) -> bool;
    pub fn expr_trans_compare(e: *mut expr, type_: expr_type, sym: *mut symbol) -> *mut expr;
    pub fn expr_fprint(e: *mut expr, out: *mut core::ffi::c_void);
    pub fn expr_gstr_print(e: *const expr, gs: *mut gstr);
    pub fn expr_gstr_print_revdep(e: *mut expr, gs: *mut gstr, pr_type: tristate, title: *const core::ffi::c_char);
}

#[repr(C)]
pub struct hlist_node { _private: [u8; 0] }
#[repr(C)]
pub struct list_head { _private: [u8; 0] }
#[repr(C)]
pub struct gstr { _private: [u8; 0] }

#[inline]
pub unsafe fn expr_is_yes(e: *const expr) -> bool {
    e.is_null() || ((*e).type_ == expr_type::E_SYMBOL && (*e).left.sym == core::ptr::addr_of_mut!(symbol_yes))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
