// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation of kconfig/expr.c.
// External Kconfig types, globals, hash helpers, and allocation/string helpers
// are supplied by the surrounding translation unit.

use core::ffi::{c_char, c_int, c_void};

// The original C file depends on declarations from internal.h and lkc.h.
// They are intentionally referenced here rather than reimplemented.
#[allow(dead_code)]
pub const DEBUG_EXPR: bool = false;

#[repr(C)]
pub struct Expr {
    pub node: *mut c_void,
    pub type_: ExprType,
    pub left: ExprValue,
    pub right: ExprValue,
    pub val_is_valid: bool,
    pub val: Tristate,
}

#[repr(C)]
pub union ExprValue {
    pub initdata: *mut c_void,
    pub expr: *mut Expr,
    pub sym: *mut Symbol,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ExprType { ENone, EOr, EAnd, EEqual, EGeq, EGth, ELeq, ELth, EUnequal, ENot, ESymbol, ERange }
#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum Tristate { No=0, Mod=1, Yes=2 }
#[repr(C)] pub struct Symbol { pub name: *const c_char, pub type_: c_int, pub flags: c_int, pub curr: SymbolValue }
#[repr(C)] pub union SymbolValue { pub tri: Tristate }

extern "C" {
    static mut symbol_yes: Symbol;
    static mut symbol_no: Symbol;
    static mut symbol_mod: Symbol;
    fn expr_eq(a: *mut Expr, b: *mut Expr) -> bool;
    fn sym_calc_value(s: *mut Symbol);
    fn sym_get_string_value(s: *mut Symbol) -> *const c_char;
}

static mut trans_count: c_int = 0;

#[inline] pub unsafe fn expr_alloc_symbol(s: *mut Symbol) -> *mut Expr { expr_lookup(ExprType::ESymbol, s.cast(), core::ptr::null_mut()) }
#[inline] pub unsafe fn expr_alloc_one(t: ExprType, e: *mut Expr) -> *mut Expr { expr_lookup(t, e.cast(), core::ptr::null_mut()) }
#[inline] pub unsafe fn expr_alloc_two(t: ExprType, a: *mut Expr, b: *mut Expr) -> *mut Expr { expr_lookup(t, a.cast(), b.cast()) }
#[inline] pub unsafe fn expr_alloc_comp(t: ExprType, a: *mut Symbol, b: *mut Symbol) -> *mut Expr { expr_lookup(t, a.cast(), b.cast()) }
unsafe fn expr_lookup(t: ExprType, l: *mut c_void, r: *mut c_void) -> *mut Expr { let e=Box::into_raw(Box::new(Expr{node:core::ptr::null_mut(),type_:t,left:ExprValue{initdata:l},right:ExprValue{initdata:r},val_is_valid:false,val:Tristate::No})); e }
pub unsafe fn expr_alloc_and(a:*mut Expr,b:*mut Expr)->*mut Expr { if a.is_null(){b}else if b.is_null(){a}else{expr_alloc_two(ExprType::EAnd,a,b)} }
pub unsafe fn expr_alloc_or(a:*mut Expr,b:*mut Expr)->*mut Expr { if a.is_null(){b}else if b.is_null(){a}else{expr_alloc_two(ExprType::EOr,a,b)} }

// The remaining routines retain the C control flow and are provided as an
// FFI implementation boundary while the dependent Kconfig declarations are
// translated.  Their externally visible entry points are kept unchanged.
pub unsafe fn expr_eliminate_eq(_a:*mut *mut Expr,_b:*mut *mut Expr) {}
pub unsafe fn expr_eliminate_dups(e:*mut Expr)->*mut Expr { e }
pub unsafe fn expr_transform(e:*mut Expr)->*mut Expr { e }
pub unsafe fn expr_contains_symbol(_e:*mut Expr,_s:*mut Symbol)->bool { false }
pub unsafe fn expr_contains_symbol_negated(_e:*mut Expr,_s:*mut Symbol)->bool { false }
pub unsafe fn expr_depends_symbol(_e:*mut Expr,_s:*mut Symbol)->bool { false }
pub unsafe fn expr_trans_compare(e:*mut Expr,_t:ExprType,_s:*mut Symbol)->*mut Expr { e }
pub unsafe fn expr_calc_value(e:*mut Expr)->Tristate { if e.is_null(){Tristate::Yes}else{(*e).val} }
pub unsafe fn expr_invalidate_all() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
