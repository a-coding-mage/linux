// SPDX-License-Identifier: GPL-2.0-or-later
/* Helpers for initial module or kernel cmdline parsing. */

/* Kernel headers and configuration-dependent definitions are supplied externally. */

#[repr(C)]
pub struct KmallocedParam { pub list: ListHead, pub val: [u8; 0] }
static mut KMALLOCED_PARAMS: ListHead = ListHead { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut KMALLOCED_PARAMS_LOCK: Spinlock = Spinlock {};

unsafe fn kmalloc_parameter(size: u32) -> *mut core::ffi::c_void {
    let p = kmalloc(size_add(core::mem::size_of::<KmallocedParam>() as u32, size), GFP_KERNEL) as *mut KmallocedParam;
    if p.is_null() { return core::ptr::null_mut(); }
    spin_lock(&raw mut KMALLOCED_PARAMS_LOCK);
    list_add(&mut (*p).list, &raw mut KMALLOCED_PARAMS);
    spin_unlock(&raw mut KMALLOCED_PARAMS_LOCK);
    (*p).val.as_mut_ptr() as *mut core::ffi::c_void
}

unsafe fn maybe_kfree_parameter(param: *mut core::ffi::c_void) {
    spin_lock(&raw mut KMALLOCED_PARAMS_LOCK);
    let mut p: *mut KmallocedParam = core::ptr::null_mut();
    list_for_each_entry(&mut p, &raw mut KMALLOCED_PARAMS, list, {
        if (*p).val.as_mut_ptr() as *mut core::ffi::c_void == param {
            list_del(&mut (*p).list); kfree(p as *mut core::ffi::c_void); break;
        }
    });
    spin_unlock(&raw mut KMALLOCED_PARAMS_LOCK);
}

unsafe fn dash2underscore(c: i8) -> i8 { if c == b'-' as i8 { b'_' as i8 } else { c } }

#[no_mangle]
pub unsafe extern "C" fn parameqn(a: *const i8, b: *const i8, n: usize) -> bool {
    for i in 0..n { if dash2underscore(*a.add(i)) != dash2underscore(*b.add(i)) { return false; } }
    true
}
#[no_mangle]
pub unsafe extern "C" fn parameq(a: *const i8, b: *const i8) -> bool { parameqn(a, b, strlen(a) + 1) }

unsafe fn param_check_unsafe(kp: *const KernelParam) -> bool {
    if (*kp).flags & KERNEL_PARAM_FL_HWPARAM != 0 && security_locked_down(LOCKDOWN_MODULE_PARAMETERS) { return false; }
    if (*kp).flags & KERNEL_PARAM_FL_UNSAFE != 0 {
        pr_notice(c"Setting dangerous option %s - tainting kernel\n", (*kp).name);
        add_taint(TAINT_USER, LOCKDEP_STILL_OK);
    }
    true
}

unsafe fn parse_one(param: *mut i8, val: *mut i8, doing: *const i8, params: *const KernelParam, num: u32, min: i16, max: i16, arg: *mut core::ffi::c_void, handle_unknown: ParseUnknownFn) -> i32 {
    for i in 0..num {
        let kp = params.add(i as usize);
        if parameq(param, (*kp).name) {
            if (*kp).level < min || (*kp).level > max { return 0; }
            if val.is_null() && (*(*kp).ops).flags & KERNEL_PARAM_OPS_FL_NOARG == 0 { return -EINVAL; }
            pr_debug(c"handling %s with value '%s'\n", param, if !val.is_null() { val } else { c"no-arg".as_ptr() });
            kernel_param_lock((*kp).mod_);
            let err = if param_check_unsafe(kp) { ((*(*kp).ops).set)(val, kp) } else { -EPERM };
            kernel_param_unlock((*kp).mod_); return err;
        }
    }
    if let Some(f) = handle_unknown { pr_debug(c"doing %s: %s='%s'\n", doing, param, val); return f(param, val, doing, arg); }
    pr_debug(c"Unknown argument '%s'\n", param); -ENOENT
}

#[no_mangle]
pub unsafe extern "C" fn parse_args(doing: *const i8, mut args: *mut i8, params: *const KernelParam, num: u32, min: i16, max: i16, arg: *mut core::ffi::c_void, unknown: ParseUnknownFn) -> *mut i8 {
    let mut err: *mut i8 = core::ptr::null_mut(); args = skip_spaces(args);
    while *args != 0 { let mut param = core::ptr::null_mut(); let mut val = core::ptr::null_mut(); args = next_arg(args, &mut param, &mut val); if val.is_null() && strcmp(param, c"--".as_ptr()) == 0 { return if !err.is_null() { err } else { args }; }
        let ret = parse_one(param, val, doing, params, num, min, max, arg, unknown);
        if ret == 0 { continue; }
        pr_err(c"%s: invalid for parameter `%s'\n", doing, param); err = ERR_PTR(ret);
    } err
}

macro_rules! standard_param_def { ($n:ident, $t:ty, $set:ident, $get:ident) => {
    #[no_mangle] pub unsafe extern "C" fn $n##_set(val: *const i8, kp: *const KernelParam) -> i32 { $set(val, 0, (*kp).arg as *mut $t) }
}; }

#[no_mangle] pub unsafe extern "C" fn param_set_uint_minmax(val: *const i8, kp: *const KernelParam, min: u32, max: u32) -> i32 { if val.is_null() { return -EINVAL; } let mut n=0; let r=kstrtouint(val,0,&mut n); if r!=0{return r} if n<min||n>max{-EINVAL}else{*( (*kp).arg as *mut u32)=n;0} }

#[no_mangle] pub unsafe extern "C" fn param_set_bool(val: *const i8, kp: *const KernelParam) -> i32 { kstrtobool(if val.is_null(){c"1".as_ptr()}else{val}, (*kp).arg as *mut bool) }
#[no_mangle] pub unsafe extern "C" fn param_get_bool(buffer:*mut i8,kp:*const KernelParam)->i32 { sprintf(buffer,c"%c\n".as_ptr(),if *((*kp).arg as *mut bool){b'Y'}else{b'N'}) }
#[no_mangle] pub unsafe extern "C" fn param_set_bool_enable_only(val:*const i8,kp:*const KernelParam)->i32 { let old=*((*kp).arg as *mut bool); let mut new=false; let mut d=*kp; d.arg=&mut new as *mut _ as *mut _; let e=param_set_bool(val,&d); if e!=0{return e} if !new&&old{-EROFS}else if new{param_set_bool(val,kp)}else{0} }
#[no_mangle] pub unsafe extern "C" fn param_set_invbool(val:*const i8,kp:*const KernelParam)->i32 { let mut v=false; let mut d=*kp; d.arg=&mut v as *mut _ as *mut _; let r=param_set_bool(val,&d); if r==0{*((*kp).arg as *mut bool)=!v} r }
#[no_mangle] pub unsafe extern "C" fn param_get_invbool(b:*mut i8,kp:*const KernelParam)->i32 { sprintf(b,c"%c\n".as_ptr(),if *((*kp).arg as *mut bool){b'N'}else{b'Y'}) }
#[no_mangle] pub unsafe extern "C" fn param_set_bint(val:*const i8,kp:*const KernelParam)->i32 { let mut v=false; let mut d=*kp; d.arg=&mut v as *mut _ as *mut _; let r=param_set_bool(val,&d); if r==0{*((*kp).arg as *mut i32)=v as i32} r }

/* The remaining parameter-array, string, and CONFIG_SYSFS/MODULES routines retain the C ABI and kernel operations. */
extern "C" {
    fn strlen(_: *const i8)->usize; fn strcmp(_: *const i8,_:*const i8)->i32; fn skip_spaces(_: *mut i8)->*mut i8; fn next_arg(_: *mut i8,_:*mut *mut i8,_:*mut *mut i8)->*mut i8;
    fn kstrtobool(*const i8,*mut bool)->i32; fn kstrtouint(*const i8,u32,*mut u32)->i32; fn sprintf(*mut i8,*const i8,...)->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
