// SPDX-License-Identifier: GPL-2.0
// Linux kernel dependencies supplied by the surrounding translation unit.

use core::ffi::c_void;

// Sockopts only registered and called from user context, so net locking would
// be overkill. Also, [gs]etsockopt calls may sleep.
static mut NF_SOCKOPT_MUTEX: c_void = c_void { };
static mut NF_SOCKOPTS: c_void = c_void { };

#[repr(C)]
pub struct sock;
#[repr(C)]
pub struct nf_sockopt_ops {
    pub list: c_void,
    pub pf: u8,
    pub set_optmin: i32,
    pub set_optmax: i32,
    pub get_optmin: i32,
    pub get_optmax: i32,
    pub owner: *mut c_void,
    pub set: Option<unsafe extern "C" fn(*mut sock, i32, sockptr_t, u32) -> i32>,
    pub get: Option<unsafe extern "C" fn(*mut sock, i32, *mut i8, *mut i32) -> i32>,
}

pub type sockptr_t = *mut c_void;

extern "C" {
    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);
    fn list_add(new: *mut c_void, head: *mut c_void);
    fn list_del(entry: *mut c_void);
    fn try_module_get(module: *mut c_void) -> bool;
    fn module_put(module: *mut c_void);
    fn ptr_err(ptr: *const c_void) -> i32;
}

// Do exclusive ranges overlap?
#[inline]
unsafe fn overlap(min1: i32, max1: i32, min2: i32, max2: i32) -> i32 {
    (max1 > min2 && min1 < max2) as i32
}

// Functions to register sockopt ranges (exclusive).
#[no_mangle]
pub unsafe extern "C" fn nf_register_sockopt(reg: *mut nf_sockopt_ops) -> i32 {
    let mut ret: i32 = 0;

    mutex_lock(&mut NF_SOCKOPT_MUTEX as *mut _ as *mut c_void);
    // list_for_each_entry(ops, &nf_sockopts, list)
    let mut ops: *mut nf_sockopt_ops = core::ptr::null_mut();
    while !ops.is_null() {
        if (*ops).pf == (*reg).pf
            && (overlap((*ops).set_optmin, (*ops).set_optmax,
                        (*reg).set_optmin, (*reg).set_optmax) != 0
                || overlap((*ops).get_optmin, (*ops).get_optmax,
                            (*reg).get_optmin, (*reg).get_optmax) != 0)
        {
            ret = -16; // -EBUSY
            break;
        }
        break;
    }

    if ret == 0 {
        list_add(&mut (*reg).list as *mut _ as *mut c_void,
                 &mut NF_SOCKOPTS as *mut _ as *mut c_void);
    }
    mutex_unlock(&mut NF_SOCKOPT_MUTEX as *mut _ as *mut c_void);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn nf_unregister_sockopt(reg: *mut nf_sockopt_ops) {
    mutex_lock(&mut NF_SOCKOPT_MUTEX as *mut _ as *mut c_void);
    list_del(&mut (*reg).list as *mut _ as *mut c_void);
    mutex_unlock(&mut NF_SOCKOPT_MUTEX as *mut _ as *mut c_void);
}

unsafe fn nf_sockopt_find(_sk: *mut sock, pf: u8, val: i32, get: i32) -> *mut nf_sockopt_ops {
    mutex_lock(&mut NF_SOCKOPT_MUTEX as *mut _ as *mut c_void);
    // list_for_each_entry(ops, &nf_sockopts, list)
    let mut ops: *mut nf_sockopt_ops = core::ptr::null_mut();
    while !ops.is_null() {
        if (*ops).pf == pf {
            if !try_module_get((*ops).owner) {
                break;
            }
            let matched = if get != 0 {
                val >= (*ops).get_optmin && val < (*ops).get_optmax
            } else {
                val >= (*ops).set_optmin && val < (*ops).set_optmax
            };
            if matched {
                mutex_unlock(&mut NF_SOCKOPT_MUTEX as *mut _ as *mut c_void);
                return ops;
            }
            module_put((*ops).owner);
        }
        break;
    }
    mutex_unlock(&mut NF_SOCKOPT_MUTEX as *mut _ as *mut c_void);
    (-92isize) as *mut nf_sockopt_ops // ERR_PTR(-ENOPROTOOPT)
}

#[no_mangle]
pub unsafe extern "C" fn nf_setsockopt(sk: *mut sock, pf: u8, val: i32,
                                         opt: sockptr_t, len: u32) -> i32 {
    let ops = nf_sockopt_find(sk, pf, val, 0);
    if (ops as isize) < 0 { return ops as isize as i32; }
    let ret = ((*ops).set.unwrap())(sk, val, opt, len);
    module_put((*ops).owner);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn nf_getsockopt(sk: *mut sock, pf: u8, val: i32,
                                         opt: *mut i8, len: *mut i32) -> i32 {
    let ops = nf_sockopt_find(sk, pf, val, 1);
    if (ops as isize) < 0 { return ops as isize as i32; }
    let ret = ((*ops).get.unwrap())(sk, val, opt, len);
    module_put((*ops).owner);
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
