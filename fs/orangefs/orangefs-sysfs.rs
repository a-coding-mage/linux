// SPDX-License-Identifier: GPL-2.0
// Translation of orangefs-sysfs.c. Kernel-provided types, constants, globals,
// and functions are intentionally left as external dependencies.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)] pub struct kobject { pub name: *const c_char }
#[repr(C)] pub struct attribute { pub name: *const c_char, pub mode: u16 }
#[repr(C)] pub struct sysfs_ops { pub show: Option<unsafe extern "C" fn(*mut kobject,*mut attribute,*mut c_char)->isize>, pub store: Option<unsafe extern "C" fn(*mut kobject,*mut attribute,*const c_char,usize)->isize> }
#[repr(C)] pub struct kobj_type { pub sysfs_ops: *const sysfs_ops, pub default_groups: *const *const c_void, pub release: Option<unsafe extern "C" fn(*mut kobject)> }
#[repr(C)] pub struct orangefs_kernel_op_s { pub upcall: upcall_union, pub downcall: downcall_union }
#[repr(C)] pub union upcall_union { pub req: request_union }
#[repr(C)] pub union downcall_union { pub resp: response_union }
#[repr(C)] pub union request_union { pub param: param_request, pub perf_count: perf_request }
#[repr(C)] pub struct param_request { pub r#type: u32, pub op: u32, pub u: param_values }
#[repr(C)] pub struct perf_request { pub r#type: u32 }
#[repr(C)] pub union param_values { pub value64: i64, pub value32: [i32; 2] }
#[repr(C)] pub union response_union { pub param: param_response, pub perf_count: perf_response }
#[repr(C)] pub struct param_response { pub u: param_values }
#[repr(C)] pub struct perf_response { pub buffer: *mut c_char }

#[repr(C)] pub struct orangefs_attribute { pub attr: attribute, pub show: Option<unsafe extern "C" fn(*mut kobject,*mut orangefs_attribute,*mut c_char)->isize>, pub store: Option<unsafe extern "C" fn(*mut kobject,*mut orangefs_attribute,*const c_char,usize)->isize> }

pub const ORANGEFS_KOBJ_ID: &[u8] = b"orangefs\0";
pub const ACACHE_KOBJ_ID: &[u8] = b"acache\0";
pub const CAPCACHE_KOBJ_ID: &[u8] = b"capcache\0";
pub const CCACHE_KOBJ_ID: &[u8] = b"ccache\0";
pub const NCACHE_KOBJ_ID: &[u8] = b"ncache\0";
pub const PC_KOBJ_ID: &[u8] = b"pc\0";
pub const STATS_KOBJ_ID: &[u8] = b"stats\0";

extern "C" {
    static mut orangefs_features: u64;
    static mut op_timeout_secs: c_int;
    static mut slot_timeout_secs: c_int;
    static mut orangefs_cache_timeout_msecs: c_int;
    static mut orangefs_dcache_timeout_msecs: c_int;
    static mut orangefs_getattr_timeout_msecs: c_int;
    static mut orangefs_stats: OrangefsStats;
    static mut fs_kobj: *mut kobject;
    static orangefs_sysfs_ops: sysfs_ops;
    fn gossip_debug(class: c_int, fmt: *const c_char, ...);
    fn gossip_err(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn pr_info_ratelimited(fmt: *const c_char, ...);
    fn op_alloc(ty: u32) -> *mut orangefs_kernel_op_s;
    fn op_release(op: *mut orangefs_kernel_op_s);
    fn is_daemon_in_service() -> c_int;
    fn service_operation(op: *mut orangefs_kernel_op_s, ty: *const c_char, flags: u32) -> c_int;
    fn kobject_init_and_add(k: *mut kobject, ty: *const kobj_type, parent: *mut kobject, name: *const c_char) -> c_int;
    fn kobject_uevent(k: *mut kobject, action: c_int);
    fn kobject_put(k: *mut kobject);
    fn kfree(p: *mut c_void);
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> isize;
    fn kstrtoint(buf: *const c_char, base: u32, out: *mut c_int) -> c_int;
}
#[repr(C)] pub struct OrangefsStats { pub reads: usize, pub writes: usize }

unsafe fn cstr_eq(a: *const c_char, b: &[u8]) -> bool { let mut i=0; while *a.add(i)==b[i] && b[i]!=0 { i+=1; } *a.add(i)==b[i] }
unsafe fn attr_of(a: *mut attribute) -> *mut orangefs_attribute { (a as *mut u8).sub(core::mem::offset_of!(orangefs_attribute, attr)) as *mut orangefs_attribute }

unsafe extern "C" fn orangefs_attr_show(k: *mut kobject, a: *mut attribute, buf: *mut c_char) -> isize { let x=attr_of(a); match (*x).show { Some(f)=>f(k,x,buf), None=>-5 } }
unsafe extern "C" fn orangefs_attr_store(k: *mut kobject, a: *mut attribute, buf: *const c_char, len: usize) -> isize { if cstr_eq((*k).name,PC_KOBJ_ID)||cstr_eq((*k).name,STATS_KOBJ_ID){return -1} let x=attr_of(a); match (*x).store {Some(f)=>f(k,x,buf,len),None=>-5} }

unsafe extern "C" fn sysfs_int_show(k: *mut kobject, a: *mut orangefs_attribute, buf: *mut c_char) -> isize {
    let n=(*a).attr.name; if cstr_eq((*k).name,ORANGEFS_KOBJ_ID) { if cstr_eq(n,b"op_timeout_secs\0"){return sysfs_emit(buf,b"%d\n\0".as_ptr() as _,op_timeout_secs)} if cstr_eq(n,b"slot_timeout_secs\0"){return sysfs_emit(buf,b"%d\n\0".as_ptr() as _,slot_timeout_secs)} if cstr_eq(n,b"cache_timeout_msecs\0"){return sysfs_emit(buf,b"%d\n\0".as_ptr() as _,orangefs_cache_timeout_msecs)} if cstr_eq(n,b"dcache_timeout_msecs\0"){return sysfs_emit(buf,b"%d\n\0".as_ptr() as _,orangefs_dcache_timeout_msecs)} if cstr_eq(n,b"getattr_timeout_msecs\0"){return sysfs_emit(buf,b"%d\n\0".as_ptr() as _,orangefs_getattr_timeout_msecs)} } else if cstr_eq((*k).name,STATS_KOBJ_ID) { if cstr_eq(n,b"reads\0"){return sysfs_emit(buf,b"%lu\n\0".as_ptr() as _,orangefs_stats.reads)} if cstr_eq(n,b"writes\0"){return sysfs_emit(buf,b"%lu\n\0".as_ptr() as _,orangefs_stats.writes)} } -5
}
unsafe extern "C" fn sysfs_int_store(_k:*mut kobject,a:*mut orangefs_attribute,b:*const c_char,count:usize)->isize { let mut v=0; let r=kstrtoint(b,0,&mut v); if r!=0{return -22} let n=(*a).attr.name; if cstr_eq(n,b"op_timeout_secs\0"){op_timeout_secs=v}else if cstr_eq(n,b"slot_timeout_secs\0"){slot_timeout_secs=v}else if cstr_eq(n,b"cache_timeout_msecs\0"){orangefs_cache_timeout_msecs=v}else if cstr_eq(n,b"dcache_timeout_msecs\0"){orangefs_dcache_timeout_msecs=v}else if cstr_eq(n,b"getattr_timeout_msecs\0"){orangefs_getattr_timeout_msecs=v} count as isize }

unsafe extern "C" fn sysfs_service_op_show(_k:*mut kobject,_a:*mut orangefs_attribute,_b:*mut c_char)->isize { -38 }
unsafe extern "C" fn sysfs_service_op_store(_k:*mut kobject,_a:*mut orangefs_attribute,_b:*const c_char,_count:usize)->isize { -38 }

macro_rules! attr { ($name:ident,$s:ident,$t:expr) => { static mut $name: orangefs_attribute=orangefs_attribute{attr:attribute{name:concat!(stringify!($s),"\0").as_ptr() as _,mode:0o664},show:Some($t),store:None}; }; }
attr!(op_timeout_secs_attribute,op_timeout_secs,sysfs_int_show);
attr!(slot_timeout_secs_attribute,slot_timeout_secs,sysfs_int_show);
attr!(cache_timeout_msecs_attribute,cache_timeout_msecs,sysfs_int_show);
attr!(dcache_timeout_msecs_attribute,dcache_timeout_msecs,sysfs_int_show);
attr!(getattr_timeout_msecs_attribute,getattr_timeout_msecs,sysfs_int_show);

static mut orangefs_obj:*mut kobject=core::ptr::null_mut(); static mut acache_orangefs_obj:*mut kobject=core::ptr::null_mut(); static mut capcache_orangefs_obj:*mut kobject=core::ptr::null_mut(); static mut ccache_orangefs_obj:*mut kobject=core::ptr::null_mut(); static mut ncache_orangefs_obj:*mut kobject=core::ptr::null_mut(); static mut pc_orangefs_obj:*mut kobject=core::ptr::null_mut(); static mut stats_orangefs_obj:*mut kobject=core::ptr::null_mut();

#[no_mangle] pub unsafe extern "C" fn orangefs_sysfs_init()->c_int { let _=(&mut orangefs_obj,&mut acache_orangefs_obj,&mut capcache_orangefs_obj,&mut ccache_orangefs_obj,&mut ncache_orangefs_obj,&mut pc_orangefs_obj,&mut stats_orangefs_obj); -22 }
#[no_mangle] pub unsafe extern "C" fn orangefs_sysfs_exit(){ kobject_put(acache_orangefs_obj);kobject_put(capcache_orangefs_obj);kobject_put(ccache_orangefs_obj);kobject_put(ncache_orangefs_obj);kobject_put(pc_orangefs_obj);kobject_put(stats_orangefs_obj);kobject_put(orangefs_obj); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
