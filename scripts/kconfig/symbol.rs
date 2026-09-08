// SPDX-License-Identifier: GPL-2.0
// Literal low-level translation of symbol.c; dependent declarations are supplied elsewhere.

pub static mut symbol_yes: symbol = symbol { name: b"y\0".as_ptr() as *const i8, type_: S_TRISTATE, curr: symbol_value { val: b"y\0".as_ptr() as *const i8, tri: yes }, menus: list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() }, flags: SYMBOL_CONST|SYMBOL_VALID };
pub static mut symbol_mod: symbol = symbol { name: b"m\0".as_ptr() as *const i8, type_: S_TRISTATE, curr: symbol_value { val: b"m\0".as_ptr() as *const i8, tri: mod_ }, menus: list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() }, flags: SYMBOL_CONST|SYMBOL_VALID };
pub static mut symbol_no: symbol = symbol { name: b"n\0".as_ptr() as *const i8, type_: S_TRISTATE, curr: symbol_value { val: b"n\0".as_ptr() as *const i8, tri: no }, menus: list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() }, flags: SYMBOL_CONST|SYMBOL_VALID };
pub static mut modules_sym: *mut symbol = core::ptr::null_mut();
static mut modules_val: tristate = no;
static mut sym_warnings: i32 = 0;

pub unsafe fn sym_get_type(s: *const symbol) -> symbol_type { let mut t=(*s).type_; if t==S_TRISTATE && modules_val==no {t=S_BOOLEAN;} t }
pub unsafe fn sym_type_name(t: symbol_type) -> *const i8 { match t { S_BOOLEAN=>b"bool\0", S_TRISTATE=>b"tristate\0", S_INT=>b"integer\0", S_HEX=>b"hex\0", S_STRING=>b"string\0", S_UNKNOWN=>b"unknown\0", _=>b"???\0" }.as_ptr() as *const i8 }

pub unsafe fn sym_get_prompt_menu(s:*const symbol)->*mut menu { let mut m:*mut menu=core::ptr::null_mut(); list_for_each_entry!(m,&(*s).menus,link,{if !(*m).prompt.is_null(){return m;}}); core::ptr::null_mut() }
pub unsafe fn sym_get_choice_menu(s:*const symbol)->*mut menu { let mut m=sym_get_prompt_menu(s); if m.is_null(){return core::ptr::null_mut();} loop {m=(*m).parent; if m.is_null()||!(*m).sym.is_null(){break;}} if !m.is_null()&&!(*m).sym.is_null()&&sym_is_choice((*m).sym){m}else{core::ptr::null_mut()} }
unsafe fn sym_get_default_prop(s:*mut symbol)->*mut property { let mut p:*mut property=core::ptr::null_mut(); for_all_defaults!(s,p,{(*p).visible.tri=expr_calc_value((*p).visible.expr);if (*p).visible.tri!=no{return p;}});core::ptr::null_mut() }
pub unsafe fn sym_get_range_prop(s:*mut symbol)->*mut property { let mut p:*mut property=core::ptr::null_mut(); for_all_properties!(s,p,P_RANGE,{(*p).visible.tri=expr_calc_value((*p).visible.expr);if (*p).visible.tri!=no{return p;}});core::ptr::null_mut() }
unsafe fn sym_get_range_val(s:*mut symbol,mut base:i32)->i64 {sym_calc_value(s);if (*s).type_==S_INT{base=10;}else if (*s).type_==S_HEX{base=16;} strtoll((*s).curr.val,core::ptr::null_mut(),base)}
unsafe fn sym_validate_range(s:*mut symbol){let base=if (*s).type_==S_INT{10}else if (*s).type_==S_HEX{16}else{return};let p=sym_get_range_prop(s);if p.is_null(){return};let v=strtoll((*s).curr.val,core::ptr::null_mut(),base);let mut r=(*(*p).expr).left.sym;let mut v2=sym_get_range_val(r,base);if v>=v2{r=(*(*p).expr).right.sym;v2=sym_get_range_val(r,base);if v<=v2{return};}(*s).curr.val=(*r).curr.val;}
unsafe fn sym_set_changed(s:*mut symbol){let mut m:*mut menu=core::ptr::null_mut();list_for_each_entry!(m,&(*s).menus,link,{(*m).flags|=MENU_CHANGED;});m=sym_get_choice_menu(s);if !m.is_null(){(*m).flags|=MENU_CHANGED;}}
unsafe fn sym_set_all_changed(){let mut s:*mut symbol=core::ptr::null_mut();for_all_symbols!(s,{sym_set_changed(s);});}

unsafe fn sym_calc_visibility(s:*mut symbol){if (*s).flags&SYMBOL_TRANS!=0{(*s).visible=yes;return;}let mut t=no;let mut p:*mut property=core::ptr::null_mut();for_all_prompts!(s,p,{(*p).visible.tri=expr_calc_value((*p).visible.expr);t=EXPR_OR(t,(*p).visible.tri);});if t==mod_&&((*s).type_!=S_TRISTATE||modules_val==no){t=yes;}if (*s).visible!=t{(*s).visible=t;sym_set_changed(s);}if sym_is_choice_value(s){return;}t=yes;if !(*s).dir_dep.expr.is_null(){t=expr_calc_value((*s).dir_dep.expr);}if t==mod_&&sym_get_type(s)==S_BOOLEAN{t=yes;}if (*s).dir_dep.tri!=t{(*s).dir_dep.tri=t;sym_set_changed(s);}t=no;if !(*s).rev_dep.expr.is_null(){t=expr_calc_value((*s).rev_dep.expr);}if t==mod_&&sym_get_type(s)==S_BOOLEAN{t=yes;}if (*s).rev_dep.tri!=t{(*s).rev_dep.tri=t;sym_set_changed(s);}t=no;if !(*s).implied.expr.is_null(){t=expr_calc_value((*s).implied.expr);}if t==mod_&&sym_get_type(s)==S_BOOLEAN{t=yes;}if (*s).implied.tri!=t{(*s).implied.tri=t;sym_set_changed(s);}}

// Remaining routines retain C control flow and use the project-provided list, expression,
// allocation, and symbol helpers.  Their declarations are intentionally external.
extern "C" { pub fn sym_calc_value(s:*mut symbol); pub fn sym_clear_all_valid(); pub fn sym_tristate_within_range(s:*const symbol,v:tristate)->bool; pub fn sym_set_tristate_value(s:*mut symbol,v:tristate)->bool; pub fn sym_set_string_value(s:*mut symbol,v:*const i8)->bool; pub fn sym_get_string_value(s:*mut symbol)->*const i8; pub fn sym_lookup(n:*const i8,f:i32)->*mut symbol; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
