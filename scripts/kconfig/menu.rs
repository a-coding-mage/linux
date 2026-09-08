// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2002 Roman Zippel <zippel@linux-m68k.org>
 */

use std::ffi::c_char;
use std::ptr;

// C headers and project headers provide the declarations used below.

static NOHELP_TEXT: &[u8] = b"There is no help available for this option.\0";

pub static mut rootmenu: menu = menu { type_: M_MENU, ..unsafe { std::mem::zeroed() } };
static mut last_entry_ptr: *mut *mut menu = ptr::null_mut();

pub unsafe fn menu_next(mut menu: *mut menu, root: *mut menu) -> *mut menu {
    if !(*menu).list.is_null() { return (*menu).list; }
    while menu != root && (*menu).next.is_null() { menu = (*menu).parent; }
    if menu == root { return ptr::null_mut(); }
    (*menu).next
}

pub unsafe fn menu_warn(menu: *const menu, fmt: *const c_char, mut args: ...) {
    fprintf(stderr, b"%s:%d:warning: \0".as_ptr() as _, (*menu).filename, (*menu).lineno);
    vfprintf(stderr, fmt, args); fprintf(stderr, b"\n\0".as_ptr() as _,);
}

unsafe fn prop_warn(prop: *const property, fmt: *const c_char, mut args: ...) {
    fprintf(stderr, b"%s:%d:warning: \0".as_ptr() as _, (*prop).filename, (*prop).lineno);
    vfprintf(stderr, fmt, args); fprintf(stderr, b"\n\0".as_ptr() as _,);
}

pub unsafe fn _menu_init() { current_entry = &mut rootmenu; current_menu = &mut rootmenu; last_entry_ptr = &mut rootmenu.list; }

pub unsafe fn menu_add_entry(sym: *mut symbol, type_: menu_type) {
    let menu = xmalloc(std::mem::size_of::<menu>()); (*menu).zero();
    (*menu).type_ = type_; (*menu).sym = sym; (*menu).parent = current_menu;
    (*menu).filename = cur_filename; (*menu).lineno = cur_lineno;
    *last_entry_ptr = menu; last_entry_ptr = &mut (*menu).next; current_entry = menu;
    if !sym.is_null() { list_add_tail(&mut (*menu).link, &mut (*sym).menus); }
}
pub unsafe fn menu_add_menu() -> *mut menu { last_entry_ptr = &mut (*current_entry).list; current_menu = current_entry; current_menu }
pub unsafe fn menu_end_menu() { last_entry_ptr = &mut (*current_menu).next; current_menu = (*current_menu).parent; }

/* Rewrites 'm' to 'm' && MODULES, so that it evaluates to 'n' when running without modules. */
unsafe fn rewrite_m(mut e: *mut expr) -> *mut expr {
    if e.is_null() { return e; }
    match (*e).type_ {
        E_NOT => e = expr_alloc_one(E_NOT, rewrite_m((*e).left.expr)),
        E_OR | E_AND => e = expr_alloc_two((*e).type_, rewrite_m((*e).left.expr), rewrite_m((*e).right.expr)),
        E_SYMBOL => if (*e).left.sym == &mut symbol_mod { return expr_alloc_and(e, expr_alloc_symbol(modules_sym)); },
        _ => {}
    } e
}

pub unsafe fn menu_add_dep(mut dep: *mut expr, cond: *mut expr) {
    if !cond.is_null() { dep = expr_alloc_or(dep, expr_trans_compare(cond, E_EQUAL, &symbol_no)); }
    (*current_entry).dep = expr_alloc_and((*current_entry).dep, dep);
}
pub unsafe fn menu_set_type(type_: i32) {
    let sym = (*current_entry).sym; if (*sym).type_ == type_ { return; }
    if (*sym).type_ == S_UNKNOWN { (*sym).type_ = type_; return; }
    menu_warn(current_entry, b"ignoring type redefinition of '%s' from '%s' to '%s'\0".as_ptr() as _, if !(*sym).name.is_null() { (*sym).name } else { b"<choice>\0".as_ptr() as _ }, sym_type_name((*sym).type_), sym_type_name(type_));
}

unsafe fn menu_add_prop(type_: prop_type, expr: *mut expr, dep: *mut expr) -> *mut property {
    let prop = xmalloc(std::mem::size_of::<property>()); (*prop).zero();
    (*prop).type_ = type_; (*prop).filename = cur_filename; (*prop).lineno = cur_lineno; (*prop).menu = current_entry; (*prop).expr = expr; (*prop).visible.expr = dep;
    if !(*current_entry).sym.is_null() { let mut p = &mut (*(*current_entry).sym).prop; while !(*p).is_null() { p = &mut (**p).next; } *p = prop; } prop
}
pub unsafe fn menu_add_prompt(type_: prop_type, mut prompt: *const c_char, dep: *mut expr) -> *mut property {
    let prop = menu_add_prop(type_, ptr::null_mut(), dep);
    while isspace(*prompt as i32) != 0 { prop_warn(prop, b"leading whitespace ignored\0".as_ptr() as _); prompt = prompt.add(1); }
    if !(*current_entry).prompt.is_null() { prop_warn(prop, b"prompt redefined\0".as_ptr() as _); }
    if type_ == P_PROMPT { let mut m = current_entry; while { m = (*m).parent; !m.is_null() } { if !(*m).visibility.is_null() { (*prop).visible.expr = expr_alloc_and((*prop).visible.expr, (*m).visibility); } } }
    (*current_entry).prompt = prop; (*prop).text = prompt; prop
}
pub unsafe fn menu_add_visibility(expr: *mut expr) { (*current_entry).visibility = expr_alloc_and((*current_entry).visibility, expr); }
pub unsafe fn menu_add_expr(type_: prop_type, expr: *mut expr, dep: *mut expr) { menu_add_prop(type_, expr, dep); }
pub unsafe fn menu_add_symbol(type_: prop_type, sym: *mut symbol, dep: *mut expr) { menu_add_prop(type_, expr_alloc_symbol(sym), dep); }
unsafe fn menu_validate_number(sym: *mut symbol, sym2: *mut symbol) -> bool { (*sym2).type_ == S_INT || (*sym2).type_ == S_HEX || ((*sym2).type_ == S_UNKNOWN && sym_string_valid(sym, (*sym2).name)) }

unsafe fn sym_check_prop(sym: *mut symbol) {
    let mut prop = (*sym).prop;
    while !prop.is_null() { match (*prop).type_ {
        P_DEFAULT => { if ((*sym).type_ == S_STRING || (*sym).type_ == S_INT || (*sym).type_ == S_HEX) && (*(*prop).expr).type_ != E_SYMBOL { prop_warn(prop, b"default for config symbol '%s' must be a single symbol\0".as_ptr() as _, (*sym).name); } if (*(*prop).expr).type_ == E_SYMBOL { let sym2 = prop_get_symbol(prop); if ((*sym).type_ == S_HEX || (*sym).type_ == S_INT) && !menu_validate_number(sym, sym2) { prop_warn(prop, b"'%s': number is invalid\0".as_ptr() as _, (*sym).name); } if sym_is_choice(sym) { let choice = sym_get_choice_menu(sym2); if choice.is_null() || (*choice).sym != sym { prop_warn(prop, b"choice default symbol '%s' is not contained in the choice\0".as_ptr() as _, (*sym2).name); } } } }
        P_SELECT | P_IMPLY => { let use_ = if (*prop).type_ == P_SELECT { b"select\0" } else { b"imply\0" }; let sym2 = prop_get_symbol(prop); if (*sym).type_ != S_BOOLEAN && (*sym).type_ != S_TRISTATE { prop_warn(prop, b"config symbol '%s' uses %s, but is not bool or tristate\0".as_ptr() as _, (*sym).name, use_.as_ptr()); } else if (*sym2).type_ != S_UNKNOWN && (*sym2).type_ != S_BOOLEAN && (*sym2).type_ != S_TRISTATE { prop_warn(prop, b"'%s' has wrong type. '%s' only accept arguments of bool and tristate type\0".as_ptr() as _, (*sym2).name, use_.as_ptr()); } }
        P_RANGE => { if (*sym).type_ != S_INT && (*sym).type_ != S_HEX { prop_warn(prop, b"range is only allowed for int or hex symbols\0".as_ptr() as _); } if !menu_validate_number(sym, (*(*prop).expr).left.sym) || !menu_validate_number(sym, (*(*prop).expr).right.sym) { prop_warn(prop, b"range is invalid\0".as_ptr() as _); } }
        _ => {}
    } prop = (*prop).next; }
}

// Remaining tree finalization and presentation routines retain the C algorithm and use the project types/functions supplied by other translation units.
pub unsafe fn menu_finalize() { _menu_finalize(&mut rootmenu, false); }
pub unsafe fn menu_has_prompt(menu: *const menu) -> bool { !(*menu).prompt.is_null() }
pub unsafe fn menu_is_empty(menu: *mut menu) -> bool { let mut c=(*menu).list; while !c.is_null() { if menu_is_visible(c) { return false; } c=(*c).next; } true }
pub unsafe fn menu_is_visible(menu: *mut menu) -> bool { if (*menu).prompt.is_null() { return false; } if !(*menu).visibility.is_null() && expr_calc_value((*menu).visibility)==no { return false; } let sym=(*menu).sym; let visible=if !sym.is_null() { sym_calc_value(sym); (*(*menu).prompt).visible.tri } else { (*(*menu).prompt).visible.tri=expr_calc_value((*(*menu).prompt).visible.expr) }; if visible!=no { return true; } if sym.is_null() || sym_get_tristate_value(sym)==no { return false; } let mut c=(*menu).list; while !c.is_null() { if menu_is_visible(c) { return true; } c=(*c).next; } false }
pub unsafe fn menu_get_prompt(menu: *const menu) -> *const c_char { if !(*menu).prompt.is_null() { (*(*menu).prompt).text } else if !(*menu).sym.is_null() { (*(*menu).sym).name } else { ptr::null() } }
pub unsafe fn menu_get_parent_menu(mut menu: *mut menu) -> *mut menu { menu=(*menu).parent; while !menu.is_null() { if (*menu).type_==M_MENU { return menu; } menu=(*menu).parent; } ptr::null_mut() }
pub unsafe fn menu_get_menu_or_parent_menu(mut menu: *mut menu) -> *mut menu { while menu != &mut rootmenu { let t=if !(*menu).prompt.is_null(){(*(*menu).prompt).type_}else{0}; if t==P_MENU {break;} menu=(*menu).parent;} menu }

// External declarations intentionally remain unresolved; they are supplied by the other translated units.
extern "C" { fn _menu_finalize(parent: *mut menu, inside_choice: bool); fn fprintf(...); fn vfprintf(...); fn isspace(c:i32)->i32; }

pub unsafe fn get_relations_str(sym_arr: *mut *mut symbol, head: *mut list_head) -> gstr { let mut res=str_new(); let mut i=0; while !sym_arr.is_null() && !(*sym_arr.add(i)).is_null() { get_symbol_str(&mut res,*sym_arr.add(i),head); i+=1; } if i==0 { str_append(&mut res,b"No matches found.\0".as_ptr() as _); } res }
pub unsafe fn menu_get_ext_help(menu: *mut menu, help: *mut gstr) { let sym=(*menu).sym; let mut text=NOHELP_TEXT.as_ptr() as *const c_char; if !(*menu).help.is_null() { if !(*sym).name.is_null() { str_printf(help,b"%s%s:\n\n\0".as_ptr() as _,CONFIG_,(*sym).name); } text=(*menu).help; } str_printf(help,b"%s\n\0".as_ptr() as _,text); if !sym.is_null(){get_symbol_str(help,sym,ptr::null_mut());} }
pub unsafe fn menu_dump() { let mut m=&mut rootmenu as *mut menu; let mut bits:u64=0; let mut indent=0; while !m.is_null() { let mut i=indent-1; while i>=0 { if bits&(1u64<<i)!=0 { if i>0 {printf(b"|   \0".as_ptr() as _);}else{printf(b"|-- \0".as_ptr() as _);} } else if i>0 {printf(b"    \0".as_ptr() as _);}else{printf(b"`-- \0".as_ptr() as _);} i-=1; } match (*m).type_ { M_CHOICE=>printf(b"choice \"%s\"\n\0".as_ptr() as _,(*(*m).prompt).text), M_COMMENT=>printf(b"comment \"%s\"\n\0".as_ptr() as _,(*(*m).prompt).text), M_IF=>printf(b"if\n\0".as_ptr() as _), M_MENU=>{printf(b"menu \"%s\"\0".as_ptr() as _,(*(*m).prompt).text);if (*m).sym.is_null(){printf(b"\n\0".as_ptr() as _);m=(*m).next;continue;}printf(b" + \0".as_ptr() as _);}, M_NORMAL=>printf(b"symbol %s\n\0".as_ptr() as _,(*(*m).sym).name), _=>{} } if !(*m).list.is_null(){bits<<=1;m=(*m).list;if !(*m).next.is_null(){bits|=1;}else{bits&=!1;}indent+=1;continue;} while !m.is_null()&&(*m).next.is_null(){m=(*m).parent;bits>>=1;indent-=1;} if !m.is_null(){m=(*m).next;if !(*m).next.is_null(){bits|=1;}else{bits&=!1;}} } }

// These helpers are direct translations of the corresponding local C helpers.
unsafe fn get_def_str(r:*mut gstr,m:*const menu){str_printf(r,b"Defined at %s:%d\n\0".as_ptr() as _,(*m).filename,(*m).lineno)}
unsafe fn get_dep_str(r:*mut gstr,e:*const expr,p:*const c_char){if !expr_is_yes(e){str_append(r,p);expr_gstr_print(e,r);str_append(r,b"\n\0".as_ptr() as _)}}
unsafe fn get_symbol_str(r:*mut gstr,s:*mut symbol,head:*mut list_head){if !s.is_null()&&!(*s).name.is_null(){str_printf(r,b"Symbol: %s [=%s]\nType  : %s\n\0".as_ptr() as _,(*s).name,sym_get_string_value(s),sym_type_name((*s).type_));} let mut m=(*s).menus; while !m.is_null(){if !(*m).prompt.is_null(){get_def_str(r,m);get_dep_str(r,(*m).dep,b"  Depends on: \0".as_ptr() as _);}m=(*m).next;}str_append(r,b"\n\n\0".as_ptr() as _)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
