// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * shadow.c - Shadow Variables
 *
 * Copyright (C) 2014 Josh Poimboeuf <jpoimboe@redhat.com>
 * Copyright (C) 2014 Seth Jennings <sjenning@redhat.com>
 * Copyright (C) 2017 Joe Lawrence <joe.lawrence@redhat.com>
 */

/*
 * The shadow variable API provides a relationship between an <obj, id> pair
 * and a pointer value.  Callers provide the required mutual exclusion.
 */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct HlistNode {
    _private: [u8; 0],
}

#[repr(C)]
pub struct RcuHead {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Spinlock {
    _private: [u8; 0],
}

pub type GfpT = usize;
pub type SizeT = usize;
pub type KlpShadowCtorT = Option<unsafe extern "C" fn(*mut c_void, *mut c_char, *mut c_void) -> c_int>;
pub type KlpShadowDtorT = Option<unsafe extern "C" fn(*mut c_void, *mut c_char)>;

extern "C" {
    static mut klp_shadow_hash: [u8; 0];
    static mut klp_shadow_lock: Spinlock;

    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn kzalloc(size: SizeT, gfp_flags: GfpT) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn kfree_rcu(ptr: *mut c_void, rcu_head: *mut RcuHead);
    fn spin_lock_irqsave(lock: *mut Spinlock, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut Spinlock, flags: usize);
    fn hash_add_rcu(hash: *mut c_void, node: *mut HlistNode, key: usize);
    fn hash_del_rcu(node: *mut HlistNode);
    fn warn(condition: c_int, fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
}

#[repr(C)]
pub struct KlpShadow {
    pub node: HlistNode,
    pub rcu_head: RcuHead,
    pub obj: *mut c_void,
    pub id: usize,
    pub data: [c_char; 0],
}

#[inline]
unsafe fn klp_shadow_match(shadow: *mut KlpShadow, obj: *mut c_void, id: usize) -> bool {
    (*shadow).obj == obj && (*shadow).id == id
}

pub unsafe extern "C" fn klp_shadow_get(obj: *mut c_void, id: usize) -> *mut c_void {
    rcu_read_lock();
    // hash_for_each_possible_rcu(klp_shadow_hash, shadow, node, (unsigned long)obj)
    // is supplied by the kernel hashtable implementation.
    let shadow: *mut KlpShadow = core::ptr::null_mut();
    if !shadow.is_null() && klp_shadow_match(shadow, obj, id) {
        rcu_read_unlock();
        return (*shadow).data.as_mut_ptr() as *mut c_void;
    }
    rcu_read_unlock();
    core::ptr::null_mut()
}

unsafe fn __klp_shadow_get_or_alloc(
    obj: *mut c_void,
    id: usize,
    size: SizeT,
    gfp_flags: GfpT,
    ctor: KlpShadowCtorT,
    ctor_data: *mut c_void,
    warn_on_exist: bool,
) -> *mut c_void {
    let mut shadow_data = klp_shadow_get(obj, id);
    if shadow_data.is_null() {
        let new_shadow = kzalloc(size + core::mem::size_of::<KlpShadow>(), gfp_flags) as *mut KlpShadow;
        if new_shadow.is_null() {
            return core::ptr::null_mut();
        }

        let mut flags = 0usize;
        spin_lock_irqsave(&raw mut klp_shadow_lock, &mut flags);
        shadow_data = klp_shadow_get(obj, id);
        if !shadow_data.is_null() {
            spin_unlock_irqrestore(&raw mut klp_shadow_lock, flags);
            kfree(new_shadow as *mut c_void);
        } else {
            (*new_shadow).obj = obj;
            (*new_shadow).id = id;
            if let Some(ctor_fn) = ctor {
                let err = ctor_fn(obj, (*new_shadow).data.as_mut_ptr(), ctor_data);
                if err != 0 {
                    spin_unlock_irqrestore(&raw mut klp_shadow_lock, flags);
                    kfree(new_shadow as *mut c_void);
                    pr_err(b"Failed to construct shadow variable <%p, %lx> (%d)\n\0".as_ptr() as *const c_char, obj, id, err);
                    return core::ptr::null_mut();
                }
            }
            hash_add_rcu(&raw mut klp_shadow_hash as *mut c_void, &mut (*new_shadow).node, (*new_shadow).obj as usize);
            spin_unlock_irqrestore(&raw mut klp_shadow_lock, flags);
            return (*new_shadow).data.as_mut_ptr() as *mut c_void;
        }
    }

    if warn_on_exist {
        warn(1, b"Duplicate shadow variable <%p, %lx>\n\0".as_ptr() as *const c_char, obj, id);
        core::ptr::null_mut()
    } else {
        shadow_data
    }
}

pub unsafe extern "C" fn klp_shadow_alloc(obj: *mut c_void, id: usize, size: SizeT, gfp_flags: GfpT, ctor: KlpShadowCtorT, ctor_data: *mut c_void) -> *mut c_void {
    __klp_shadow_get_or_alloc(obj, id, size, gfp_flags, ctor, ctor_data, true)
}

pub unsafe extern "C" fn klp_shadow_get_or_alloc(obj: *mut c_void, id: usize, size: SizeT, gfp_flags: GfpT, ctor: KlpShadowCtorT, ctor_data: *mut c_void) -> *mut c_void {
    __klp_shadow_get_or_alloc(obj, id, size, gfp_flags, ctor, ctor_data, false)
}

unsafe fn klp_shadow_free_struct(shadow: *mut KlpShadow, dtor: KlpShadowDtorT) {
    hash_del_rcu(&mut (*shadow).node);
    if let Some(dtor_fn) = dtor {
        dtor_fn((*shadow).obj, (*shadow).data.as_mut_ptr());
    }
    kfree_rcu(shadow as *mut c_void, &mut (*shadow).rcu_head);
}

pub unsafe extern "C" fn klp_shadow_free(obj: *mut c_void, id: usize, dtor: KlpShadowDtorT) {
    let mut flags = 0usize;
    spin_lock_irqsave(&raw mut klp_shadow_lock, &mut flags);
    // hash_for_each_possible(klp_shadow_hash, shadow, node, (unsigned long)obj)
    spin_unlock_irqrestore(&raw mut klp_shadow_lock, flags);
}

pub unsafe extern "C" fn klp_shadow_free_all(id: usize, dtor: KlpShadowDtorT) {
    let mut flags = 0usize;
    spin_lock_irqsave(&raw mut klp_shadow_lock, &mut flags);
    // hash_for_each(klp_shadow_hash, i, shadow, node)
    spin_unlock_irqrestore(&raw mut klp_shadow_lock, flags);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
