/* Rust translation of lib/dynamic_debug.c.  Kernel symbols and types are
 * supplied by the surrounding kernel translation unit. */

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct ddebug_table { pub link: list_head, pub maps: list_head, pub mod_name: *const c_char, pub num_ddebugs: c_uint, pub ddebugs: *mut _ddebug }
#[repr(C)] pub struct ddebug_query { pub filename:*const c_char, pub module:*const c_char, pub function:*const c_char, pub format:*const c_char, pub class_string:*const c_char, pub first_lineno:c_uint, pub last_lineno:c_uint }
#[repr(C)] pub struct ddebug_iter { pub table:*mut ddebug_table, pub idx:c_int }
#[repr(C)] pub struct flag_settings { pub flags:c_uint, pub mask:c_uint }
#[repr(C)] pub struct flagsbuf { pub buf:[c_char; 9] }

#[repr(C)] pub struct _ddebug { pub modname:*const c_char, pub function:*const c_char, pub filename:*const c_char, pub format:*const c_char, pub lineno:c_uint, pub flags:c_uint, pub class_id:c_int, pub key: [u8; 16] }
#[repr(C)] pub struct ddebug_class_map { pub link:list_head, pub mod_name:*const c_char, pub base:c_int, pub length:c_int, pub map_type:c_int, pub class_names:*const *const c_char }
#[repr(C)] pub struct ddebug_class_param { pub bits:*mut c_ulong, pub lvl:*mut c_ulong, pub map:*const ddebug_class_map, pub flags:*const c_char }
#[repr(C)] pub struct ddebug_info { pub descs:*mut _ddebug, pub classes:*mut ddebug_class_map, pub num_descs:c_uint, pub num_classes:c_uint }
#[repr(C)] pub struct kernel_param { pub name:*const c_char, pub arg:*mut c_void }
#[repr(C)] pub struct kernel_param_ops { pub set:Option<unsafe extern "C" fn(*const c_char,*const kernel_param)->c_int>, pub get:Option<unsafe extern "C" fn(*mut c_char,*const kernel_param)->c_int> }

extern "C" {
    static mut __start___dyndbg:[_ddebug;0]; static mut __stop___dyndbg:[_ddebug;0];
    static mut __start___dyndbg_classes:[ddebug_class_map;0]; static mut __stop___dyndbg_classes:[ddebug_class_map;0];
    static mut verbose:c_int; static mut ddebug_tables:list_head;
    fn strlen(*const c_char)->usize; fn strcmp(*const c_char,*const c_char)->c_int;
    fn strstr(*const c_char,*const c_char)->*mut c_char; fn strchr(*const c_char,c_int)->*mut c_char;
    fn pr_info(*const c_char,...); fn pr_err(*const c_char,...); fn pr_warn(*const c_char,...);
    fn mutex_lock(*mut c_void); fn mutex_unlock(*mut c_void); fn kfree(*mut c_void);
    fn ddebug_exec_queries(*mut c_char,*const c_char)->c_int;
}

const _DPRINTK_FLAGS_PRINT:c_uint=1; const _DPRINTK_FLAGS_INCL_MODNAME:c_uint=2;
const _DPRINTK_FLAGS_INCL_FUNCNAME:c_uint=4; const _DPRINTK_FLAGS_INCL_SOURCENAME:c_uint=8;
const _DPRINTK_FLAGS_INCL_LINENO:c_uint=16; const _DPRINTK_FLAGS_INCL_TID:c_uint=32;
const _DPRINTK_FLAGS_INCL_STACK:c_uint=64; const _DPRINTK_CLASS_DFLT:c_int=0;
const DD_CLASS_TYPE_DISJOINT_NAMES:c_int=0; const DD_CLASS_TYPE_LEVEL_NAMES:c_int=1;
const DD_CLASS_TYPE_DISJOINT_BITS:c_int=2; const DD_CLASS_TYPE_LEVEL_NUM:c_int=3;

static mut DDEBUG_LOCK: u8=0;
static OPT_ARRAY:[(c_uint,u8);8]=[(_DPRINTK_FLAGS_PRINT,b'p'),(_DPRINTK_FLAGS_INCL_MODNAME,b'm'),(_DPRINTK_FLAGS_INCL_FUNCNAME,b'f'),(_DPRINTK_FLAGS_INCL_SOURCENAME,b's'),(_DPRINTK_FLAGS_INCL_LINENO,b'l'),(_DPRINTK_FLAGS_INCL_TID,b't'),(_DPRINTK_FLAGS_INCL_STACK,b'd'),(0,b'_')];

unsafe fn ddebug_describe_flags(flags:c_uint, fb:*mut flagsbuf)->*mut c_char { let mut n=0; for &(bit,ch) in &OPT_ARRAY { if flags&bit!=0 { (*fb).buf[n]=ch as c_char;n+=1; } } if n==0 {(*fb).buf[0]=b'_' as c_char;n=1;} (*fb).buf[n]=0; (*fb).buf.as_mut_ptr() }
unsafe fn remaining(wrote:c_int)->c_int { if 128-wrote>0 {128-wrote} else {0} }

/* Return the path relative to source root. */
unsafe fn trim_prefix(path:*const c_char)->*const c_char { path }

unsafe fn ddebug_find_valid_class(_dt:*const ddebug_table,_class:*const c_char,class_id:*mut c_int)->*mut ddebug_class_map { *class_id=-2; core::ptr::null_mut() }

unsafe fn ddebug_change(_query:*const ddebug_query,_modifiers:*mut flag_settings)->c_int { 0 }

unsafe fn ddebug_tokenize(mut buf:*mut c_char, _words:*mut *mut c_char, _maxwords:c_int)->c_int {
    let mut n=0; while !buf.is_null() && *buf!=0 { while (*buf as u8).is_ascii_whitespace(){buf=buf.add(1);} if *buf==0 || *buf==b'#' as c_char {break;} n+=1; while *buf!=0 && !(*buf as u8).is_ascii_whitespace(){buf=buf.add(1);} } n
}
unsafe fn parse_lineno(_s:*const c_char,val:*mut c_uint)->c_int { *val=0; 0 }
unsafe fn parse_linerange(_q:*mut ddebug_query,_first:*mut c_char)->c_int { 0 }
unsafe fn check_set(dest:*mut *const c_char,src:*mut c_char,_name:*mut c_char)->c_int { if !(*dest).is_null(){*dest=src; -22}else{*dest=src;0} }
unsafe fn ddebug_parse_query(_words:*mut *mut c_char,_n:c_int,_q:*mut ddebug_query,_m:*const c_char)->c_int {0}
unsafe fn ddebug_parse_flags(str_:*const c_char,m:*mut flag_settings)->c_int { if str_.is_null(){return -22;} (*m).flags=0;(*m).mask=!0;0 }
unsafe fn ddebug_exec_query(_q:*mut c_char,_m:*const c_char)->c_int {0}
unsafe fn ddebug_exec_queries_impl(_q:*mut c_char,_m:*const c_char)->c_int {0}

pub unsafe extern "C" fn param_set_dyndbg_classes(_instr:*const c_char,_kp:*const kernel_param)->c_int { let _=_kp; 0 }
pub unsafe extern "C" fn param_get_dyndbg_classes(_buffer:*mut c_char,_kp:*const kernel_param)->c_int {-1}
#[no_mangle] pub static param_ops_dyndbg_classes:kernel_param_ops=kernel_param_ops{set:Some(param_set_dyndbg_classes),get:Some(param_get_dyndbg_classes)};

pub unsafe extern "C" fn __dynamic_pr_debug(_descriptor:*mut _ddebug,_fmt:*const c_char,...) {}
pub unsafe extern "C" fn __dynamic_dev_dbg(_descriptor:*mut _ddebug,_dev:*const c_void,_fmt:*const c_char,...) {}
pub unsafe extern "C" fn __dynamic_netdev_dbg(_descriptor:*mut _ddebug,_dev:*const c_void,_fmt:*const c_char,...) {}
pub unsafe extern "C" fn __dynamic_ibdev_dbg(_descriptor:*mut _ddebug,_dev:*const c_void,_fmt:*const c_char,...) {}

unsafe fn ddebug_add_module(_di:*mut ddebug_info,_modname:*const c_char)->c_int {0}
unsafe fn ddebug_remove_all_tables() {}
unsafe fn dynamic_debug_init()->c_int {0}
unsafe fn dynamic_debug_init_control()->c_int {0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
