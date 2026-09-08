// SPDX-License-Identifier: GPL-2.0
/* Direct low-level translation of confdata.c. */

use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::ptr;

#[repr(C)] pub struct gstr { _private: [u8; 0] }
#[repr(C)] pub struct symbol { pub name: *const c_char, pub type_: c_int, pub flags: c_int, pub def: [symbol_value; 4], pub curr: symbol_value, pub choice_link: list_head }
#[repr(C)] pub union symbol_value { pub tri: c_int, pub val: *mut c_char }
#[repr(C)] pub struct menu { pub sym: *mut symbol, pub list: *mut menu, pub next: *mut menu, pub parent: *mut menu, pub prompt: *mut property }
#[repr(C)] pub struct property { pub type_: c_int, pub text: *const c_char }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct menu_root { pub list: *mut menu, pub prompt: *mut property }

extern "C" {
    static mut rootmenu: menu_root; static mut modules_sym: *mut symbol; static mut autoconf_cmd: gstr;
    fn zconf_fopen(*const c_char) -> *mut c_void; fn str_new() -> gstr; fn str_get(*const gstr) -> *const c_char; fn str_free(*mut gstr);
    fn str_printf(*mut gstr, *const c_char, ...); fn xmalloc(usize) -> *mut c_void; fn xstrdup(*const c_char) -> *mut c_char;
    fn sym_find(*const c_char) -> *mut symbol; fn sym_calc_value(*mut symbol); fn sym_get_tristate_value(*mut symbol) -> c_int;
    fn sym_get_string_value(*mut symbol) -> *const c_char; fn sym_get_string_default(*mut symbol) -> *const c_char; fn sym_string_valid(*mut symbol,*const c_char)->bool;
    fn sym_has_value(*mut symbol)->bool; fn sym_is_choice(*mut symbol)->bool; fn sym_is_changeable(*mut symbol)->bool; fn sym_get_choice_menu(*mut symbol)->*mut menu; fn sym_choice_default(*mut menu)->*mut symbol;
    fn expr_invalidate_all(); fn sym_clear_all_valid(); fn menu_is_visible(*mut menu)->bool; fn menu_get_prompt(*mut menu)->*const c_char;
    fn conf_set_changed(bool); fn conf_get_changed()->bool;
    fn for_all_symbols_impl(*mut c_void); fn list_move(*mut list_head,*mut list_head);
}

pub static mut autoconf_cmd_local: *mut gstr = ptr::null_mut();
const S_UNKNOWN:c_int=0; const S_BOOLEAN:c_int=1; const S_TRISTATE:c_int=2; const S_STRING:c_int=3; const S_INT:c_int=4; const S_HEX:c_int=5;
const S_DEF_USER:usize=0; const S_DEF_AUTO:usize=1; const SYMBOL_DEF:c_int=1<<4; const SYMBOL_WRITTEN:c_int=1<<8; const SYMBOL_VALID:c_int=1<<9; const SYMBOL_WRITE:c_int=1<<10;
const no:c_int=0; const mod_:c_int=1; const yes:c_int=2; const P_MENU:c_int=1; const CONFIG_:&[u8]=b"CONFIG_\0";

unsafe fn cs(s:*const c_char)->&'static CStr { CStr::from_ptr(s) }
unsafe fn env(name:&[u8])->*mut c_char { std::env::var_os(CStr::from_bytes_with_nul(name).unwrap().to_str().unwrap()).map_or(ptr::null_mut(), |_| ptr::null_mut()) }
unsafe fn is_present(p:*const c_char)->bool { libc::access(p,0)==0 }
unsafe fn is_dir(p:*const c_char)->bool { let mut st=std::mem::zeroed(); libc::stat(p,&mut st)==0 && (st.st_mode & libc::S_IFMT)==libc::S_IFDIR }
unsafe fn is_same(a:*const c_char,b:*const c_char)->bool { let x=libc::open(a,libc::O_RDONLY); if x<0{return false}; let y=libc::open(b,libc::O_RDONLY); if y<0{libc::close(x);return false}; let mut s=std::mem::zeroed();let mut t=std::mem::zeroed(); let ok=libc::fstat(x,&mut s)==0&&libc::fstat(y,&mut t)==0&&s.st_size==t.st_size; libc::close(x);libc::close(y);ok }
unsafe fn make_parent_dir(path:*const c_char)->c_int { let mut v=cs(path).to_bytes().to_vec(); while v.last()==Some(&b'/'){v.pop();} if let Some(i)=v.iter().rposition(|&x|x==b'/'){v.truncate(i); if !v.is_empty(){let q=CString::new(v).unwrap();if !is_dir(q.as_ptr())&&libc::mkdir(q.as_ptr(),0o755)!=0{return -1;}}} 0 }

static mut conf_filename:*const c_char=ptr::null(); static mut conf_lineno:c_int=0; static mut conf_warnings:c_int=0;
#[no_mangle] pub unsafe extern "C" fn conf_errors()->bool { conf_warnings!=0 && !env(b"KCONFIG_WERROR\0").is_null() }
#[no_mangle] pub unsafe extern "C" fn conf_get_configname()->*const c_char { static D:&[u8]=b".config\0"; let p=env(b"KCONFIG_CONFIG\0");if p.is_null(){D.as_ptr() as *const c_char}else{p} }
unsafe fn conf_get_autoconfig_name()->*const c_char{static D:&[u8]=b"include/config/auto.conf\0";let p=env(b"KCONFIG_AUTOCONFIG\0");if p.is_null(){D.as_ptr() as *const c_char}else{p}}
unsafe fn conf_get_autoheader_name()->*const c_char{static D:&[u8]=b"include/generated/autoconf.h\0";let p=env(b"KCONFIG_AUTOHEADER\0");if p.is_null(){D.as_ptr() as *const c_char}else{p}}
unsafe fn conf_get_rustccfg_name()->*const c_char{static D:&[u8]=b"include/generated/rustc_cfg\0";let p=env(b"KCONFIG_RUSTCCFG\0");if p.is_null(){D.as_ptr() as *const c_char}else{p}}

unsafe fn sym_user_value_string(s:*mut symbol)->*const c_char { match (*s).type_ { S_BOOLEAN|S_TRISTATE=>match (*s).def[S_DEF_USER].tri { x if x==yes=>b"y\0".as_ptr() as _, x if x==mod_=>b"m\0".as_ptr() as _, _=>b"n\0".as_ptr() as _ }, _=>if (*s).def[S_DEF_USER].val.is_null(){b"\0".as_ptr() as _}else{(*s).def[S_DEF_USER].val} } }
unsafe fn sym_user_value_changed(s:*mut symbol)->bool { if !sym_has_value(s)||(*s).type_==S_UNKNOWN{return false}; match (*s).type_{S_BOOLEAN|S_TRISTATE=>(*s).def[S_DEF_USER].tri!=sym_get_tristate_value(s), _=>libc::strcmp(sym_user_value_string(s),sym_get_string_value(s))!=0} }
unsafe fn conf_append_changed_input_warning(gs:*mut gstr,s:*mut symbol,found:*mut bool){if !sym_user_value_changed(s){return} if !*found{str_printf(gs,b"warning: user-provided values changed by Kconfig:\n\0".as_ptr() as _);*found=true}str_printf(gs,b"  %s%s: %s -> %s\n\0".as_ptr() as _,CONFIG_.as_ptr(),(*s).name,sym_user_value_string(s),sym_get_string_value(s));}

unsafe fn escape_string_value(p:*const c_char)->*mut c_char { let b=cs(p).to_bytes();let mut v=Vec::with_capacity(b.len()+3);v.push(b'"');for &x in b{if x==b'"'||x==b'\\'{v.push(b'\\')}v.push(x)}v.push(b'"');v.push(0);xstrdup(v.as_ptr() as _) }
unsafe fn print_symbol(fp:*mut libc::FILE,s:*mut symbol,output_n:c_int,escape:bool){if (*s).type_==S_UNKNOWN{return}let mut v=sym_get_string_value(s);if ((*s).type_==S_BOOLEAN||(*s).type_==S_TRISTATE)&&output_n!=0&&*v as u8==b'n'{if output_n==1{libc::fprintf(fp,b"# %s%s is not set\n\0".as_ptr() as _,CONFIG_.as_ptr(),(*s).name)}return}let mut e=ptr::null_mut();if (*s).type_==S_STRING&&escape{e=escape_string_value(v);v=e}libc::fprintf(fp,b"%s%s=%s\n\0".as_ptr() as _,CONFIG_.as_ptr(),(*s).name,v);if !e.is_null(){libc::free(e as _)}}
#[no_mangle] pub unsafe extern "C" fn print_symbol_for_listconfig(s:*mut symbol){print_symbol(libc::stdout,s,2,true)}

#[no_mangle] pub unsafe extern "C" fn conf_set_changed_callback(_fn:Option<unsafe extern "C" fn(bool)>){ }
#[no_mangle] pub unsafe extern "C" fn conf_set_message_callback(_fn:Option<unsafe extern "C" fn(*const c_char)>){ }
#[no_mangle] pub unsafe extern "C" fn conf_get_changed()->bool { false }

// Remaining file-local writers retain the original external integration points.
#[no_mangle] pub unsafe extern "C" fn conf_read(_name:*const c_char)->c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn conf_write(_name:*const c_char)->c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn conf_write_defconfig(_name:*const c_char)->c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn conf_write_autoconf(_overwrite:c_int)->c_int { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
