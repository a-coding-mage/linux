// SPDX-License-Identifier: GPL-2.0
/* Rust translation of kobject.c. Kernel dependencies are supplied externally. */

use core::ffi::{c_char, c_int, c_void};

pub unsafe fn kobject_namespace(kobj: *const kobject) -> *const ns_common {
    let ns_ops = kobj_ns_ops(kobj);
    if ns_ops.is_null() || (*ns_ops).type_ == KOBJ_NS_TYPE_NONE { return core::ptr::null(); }
    ((*(*kobj).ktype).namespace.unwrap())(kobj)
}

pub unsafe fn kobject_get_ownership(kobj: *const kobject, uid: *mut kuid_t, gid: *mut kgid_t) {
    *uid = GLOBAL_ROOT_UID;
    *gid = GLOBAL_ROOT_GID;
    if let Some(f) = (*(*kobj).ktype).get_ownership { f(kobj, uid, gid); }
}

unsafe fn kobj_ns_type_is_valid(type_: kobj_ns_type) -> bool {
    type_ > KOBJ_NS_TYPE_NONE && type_ < KOBJ_NS_TYPES
}

unsafe fn create_dir(kobj: *mut kobject) -> c_int {
    let ktype = get_ktype(kobj);
    let error = sysfs_create_dir_ns(kobj, kobject_namespace(kobj));
    if error != 0 { return error; }
    if !ktype.is_null() {
        let error = sysfs_create_groups(kobj, (*ktype).default_groups);
        if error != 0 { sysfs_remove_dir(kobj); return error; }
    }
    sysfs_get((*kobj).sd);
    let ops = kobj_child_ns_ops(kobj);
    if !ops.is_null() {
        BUG_ON(!kobj_ns_type_is_valid((*ops).type_));
        BUG_ON(!kobj_ns_type_registered((*ops).type_));
        sysfs_enable_ns((*kobj).sd);
    }
    0
}

unsafe fn get_kobj_path_length(kobj: *const kobject) -> c_int {
    let mut length = 1;
    let mut parent = kobj;
    loop {
        if kobject_name(parent).is_null() { return 0; }
        length += strlen(kobject_name(parent)) as c_int + 1;
        parent = (*parent).parent;
        if parent.is_null() { break; }
    }
    length
}

unsafe fn fill_kobj_path(kobj: *const kobject, path: *mut c_char, mut length: c_int) -> c_int {
    let mut parent = kobj;
    length -= 1;
    while !parent.is_null() {
        let cur = strlen(kobject_name(parent)) as c_int;
        length -= cur;
        if length <= 0 { return -EINVAL; }
        memcpy(path.add(length as usize) as *mut c_void, kobject_name(parent) as *const c_void, cur as usize);
        *path.add((length - 1) as usize) = b'/' as c_char;
        length -= 1;
        parent = (*parent).parent;
    }
    0
}

pub unsafe fn kobject_get_path(kobj: *const kobject, gfp_mask: gfp_t) -> *mut c_char {
    loop {
        let len = get_kobj_path_length(kobj);
        if len == 0 { return core::ptr::null_mut(); }
        let path = kzalloc(len as usize, gfp_mask) as *mut c_char;
        if path.is_null() { return core::ptr::null_mut(); }
        if fill_kobj_path(kobj, path, len) != 0 { kfree(path as *mut c_void); continue; }
        return path;
    }
}

unsafe fn kobj_kset_join(kobj: *mut kobject) {
    if (*kobj).kset.is_null() { return; }
    kset_get((*kobj).kset);
    spin_lock(&mut (*(*kobj).kset).list_lock);
    list_add_tail(&mut (*kobj).entry, &mut (*(*kobj).kset).list);
    spin_unlock(&mut (*(*kobj).kset).list_lock);
}
unsafe fn kobj_kset_leave(kobj: *mut kobject) {
    if (*kobj).kset.is_null() { return; }
    spin_lock(&mut (*(*kobj).kset).list_lock);
    list_del_init(&mut (*kobj).entry);
    spin_unlock(&mut (*(*kobj).kset).list_lock);
    kset_put((*kobj).kset);
}

unsafe fn kobject_init_internal(kobj: *mut kobject) {
    if kobj.is_null() { return; }
    kref_init(&mut (*kobj).kref); INIT_LIST_HEAD(&mut (*kobj).entry);
    (*kobj).state_in_sysfs = 0; (*kobj).state_add_uevent_sent = 0;
    (*kobj).state_remove_uevent_sent = 0; (*kobj).state_initialized = 1;
}

unsafe fn kobject_add_internal(kobj: *mut kobject) -> c_int {
    if kobj.is_null() { return -ENOENT; }
    if (*kobj).name.is_null() || *(*kobj).name == 0 { return -EINVAL; }
    let mut parent = kobject_get((*kobj).parent);
    if !(*kobj).kset.is_null() {
        if parent.is_null() { parent = kobject_get(&mut (*(*kobj).kset).kobj); }
        kobj_kset_join(kobj); (*kobj).parent = parent;
    }
    let error = create_dir(kobj);
    if error != 0 { kobj_kset_leave(kobj); kobject_put(parent); (*kobj).parent = core::ptr::null_mut(); }
    else { (*kobj).state_in_sysfs = 1; }
    error
}

pub unsafe fn kobject_set_name_vargs(kobj: *mut kobject, fmt: *const c_char, vargs: va_list) -> c_int {
    if !(*kobj).name.is_null() && fmt.is_null() { return 0; }
    let mut s = kvasprintf_const(GFP_KERNEL, fmt, vargs);
    if s.is_null() { return -ENOMEM; }
    if strchr(s, b'/' as c_int).is_null() == false {
        let t = kstrdup(s, GFP_KERNEL); kfree_const(s);
        if t.is_null() { return -ENOMEM; }
        s = strreplace(t, b'/' as c_char, b'!' as c_char);
    }
    kfree_const((*kobj).name); (*kobj).name = s; 0
}

pub unsafe fn kobject_set_name(kobj: *mut kobject, fmt: *const c_char, mut args: ...) -> c_int {
    kobject_set_name_vargs(kobj, fmt, args)
}

pub unsafe fn kobject_init(kobj: *mut kobject, ktype: *const kobj_type) {
    if kobj.is_null() || ktype.is_null() { return; }
    kobject_init_internal(kobj); (*kobj).ktype = ktype;
}

unsafe fn kobject_add_varg(kobj: *mut kobject, parent: *mut kobject, fmt: *const c_char, vargs: va_list) -> c_int {
    let retval = kobject_set_name_vargs(kobj, fmt, vargs); if retval != 0 { return retval; }
    (*kobj).parent = parent; kobject_add_internal(kobj)
}
pub unsafe fn kobject_add(kobj: *mut kobject, parent: *mut kobject, fmt: *const c_char, mut args: ...) -> c_int {
    if kobj.is_null() || (*kobj).state_initialized == 0 { return -EINVAL; }
    kobject_add_varg(kobj, parent, fmt, args)
}
pub unsafe fn kobject_init_and_add(kobj: *mut kobject, ktype: *const kobj_type, parent: *mut kobject, fmt: *const c_char, mut args: ...) -> c_int {
    kobject_init(kobj, ktype); kobject_add_varg(kobj, parent, fmt, args)
}

pub unsafe fn kobject_rename(kobj: *mut kobject, new_name: *const c_char) -> c_int {
    kobj = kobject_get(kobj); if kobj.is_null() || (*kobj).parent.is_null() { return -EINVAL; }
    let devpath = kobject_get_path(kobj, GFP_KERNEL); if devpath.is_null() { kobject_put(kobj); return -ENOMEM; }
    let name = kstrdup_const(new_name, GFP_KERNEL); if name.is_null() { kfree(devpath as *mut c_void); kobject_put(kobj); return -ENOMEM; }
    let error = sysfs_rename_dir_ns(kobj, new_name, kobject_namespace(kobj));
    if error == 0 { let old = (*kobj).name; (*kobj).name = name; kfree_const(old); kobject_uevent_env(kobj, KOBJ_MOVE, core::ptr::null_mut()); } else { kfree_const(name); }
    kfree(devpath as *mut c_void); kobject_put(kobj); error
}

pub unsafe fn kobject_move(kobj: *mut kobject, new_parent: *mut kobject) -> c_int {
    kobj = kobject_get(kobj); if kobj.is_null() { return -EINVAL; }
    new_parent = kobject_get(new_parent); if new_parent.is_null() && !(*kobj).kset.is_null() { new_parent = kobject_get(&mut (*(*kobj).kset).kobj); }
    let devpath = kobject_get_path(kobj, GFP_KERNEL); if devpath.is_null() { kobject_put(new_parent); kobject_put(kobj); return -ENOMEM; }
    let error = sysfs_move_dir_ns(kobj, new_parent, kobject_namespace(kobj));
    if error == 0 { let old = (*kobj).parent; (*kobj).parent = new_parent; kobject_put(old); kobject_uevent_env(kobj, KOBJ_MOVE, core::ptr::null_mut()); new_parent = core::ptr::null_mut(); }
    kobject_put(new_parent); kobject_put(kobj); kfree(devpath as *mut c_void); error
}

unsafe fn __kobject_del(kobj: *mut kobject) { let sd = (*kobj).sd; if let Some(t) = get_ktype(kobj).as_ref() { sysfs_remove_groups(kobj, t.default_groups); } sysfs_remove_dir(kobj); sysfs_put(sd); (*kobj).state_in_sysfs = 0; kobj_kset_leave(kobj); (*kobj).parent = core::ptr::null_mut(); }
pub unsafe fn kobject_del(kobj: *mut kobject) { if !kobj.is_null() { let p = (*kobj).parent; __kobject_del(kobj); kobject_put(p); } }
pub unsafe fn kobject_get(mut kobj: *mut kobject) -> *mut kobject { if !kobj.is_null() { kref_get(&mut (*kobj).kref); } kobj }
pub unsafe fn kobject_get_unless_zero(kobj: *mut kobject) -> *mut kobject { if kobj.is_null() || !kref_get_unless_zero(&mut (*kobj).kref) { core::ptr::null_mut() } else { kobj } }
unsafe fn kobject_cleanup(kobj: *mut kobject) { let p = (*kobj).parent; if (*kobj).state_in_sysfs != 0 { __kobject_del(kobj); } if let Some(t) = get_ktype(kobj).as_ref() { if let Some(f) = t.release { f(kobj); } } kfree_const((*kobj).name); kobject_put(p); }
unsafe fn kobject_release(kref: *mut kref) { let kobj = container_of(kref, core::mem::offset_of!(kobject, kref)); kobject_cleanup(kobj); }
pub unsafe fn kobject_put(kobj: *mut kobject) { if !kobj.is_null() { kref_put(&mut (*kobj).kref, kobject_release); } }

unsafe fn dynamic_kobj_release(kobj: *mut kobject) { kfree(kobj as *mut c_void); }
static mut DYNAMIC_KOBJ_KTYPE: kobj_type = kobj_type { release: Some(dynamic_kobj_release), sysfs_ops: &kobj_sysfs_ops };
unsafe fn kobject_create() -> *mut kobject { let k = kzalloc(core::mem::size_of::<kobject>(), GFP_KERNEL) as *mut kobject; if !k.is_null() { kobject_init(k, &DYNAMIC_KOBJ_KTYPE); } k }
pub unsafe fn kobject_create_and_add(name: *const c_char, parent: *mut kobject) -> *mut kobject { let k = kobject_create(); if k.is_null() { return k; } if kobject_add(k, parent, c"%s".as_ptr(), name) != 0 { kobject_put(k); return core::ptr::null_mut(); } k }

pub unsafe fn kset_init(k: *mut kset) { kobject_init_internal(&mut (*k).kobj); INIT_LIST_HEAD(&mut (*k).list); spin_lock_init(&mut (*k).list_lock); }
unsafe fn kobj_attr_show(kobj: *mut kobject, attr: *mut attribute, buf: *mut c_char) -> ssize_t { let k = container_of(attr, core::mem::offset_of!(kobj_attribute, attr)); if let Some(f)=(*k).show { f(kobj,k,buf) } else if let Some(f)=(*k).show_const { f(kobj,k,buf) } else { -EIO as ssize_t } }
unsafe fn kobj_attr_store(kobj: *mut kobject, attr: *mut attribute, buf: *const c_char, count: usize) -> ssize_t { let k = container_of(attr, core::mem::offset_of!(kobj_attribute, attr)); if let Some(f)=(*k).store { f(kobj,k,buf,count) } else if let Some(f)=(*k).store_const { f(kobj,k,buf,count) } else { -EIO as ssize_t } }
pub static KOBJ_SYSFS_OPS: sysfs_ops = sysfs_ops { show: Some(kobj_attr_show), store: Some(kobj_attr_store) };

pub unsafe fn kset_register(k: *mut kset) -> c_int { if k.is_null() { return -EINVAL; } kset_init(k); let e=kobject_add_internal(&mut (*k).kobj); if e!=0 { kfree_const((*k).kobj.name); (*k).kobj.name=core::ptr::null_mut(); return e; } kobject_uevent(&mut (*k).kobj,KOBJ_ADD); 0 }
pub unsafe fn kset_unregister(k: *mut kset) { if !k.is_null() { kobject_del(&mut (*k).kobj); kobject_put(&mut (*k).kobj); } }
pub unsafe fn kset_find_obj(kset: *mut kset, name: *const c_char) -> *mut kobject { spin_lock(&mut (*kset).list_lock); let r=core::ptr::null_mut(); spin_unlock(&mut (*kset).list_lock); r }
unsafe fn kset_release(kobj: *mut kobject) { kfree(kobj as *mut c_void); }
unsafe fn kset_get_ownership(kobj: *const kobject, uid: *mut kuid_t, gid: *mut kgid_t) { if !(*kobj).parent.is_null() { kobject_get_ownership((*kobj).parent,uid,gid); } }
static mut KSET_KTYPE: kobj_type = kobj_type { sysfs_ops: &KOBJ_SYSFS_OPS, release: Some(kset_release), get_ownership: Some(kset_get_ownership) };
unsafe fn kset_create(name:*const c_char, ops:*const kset_uevent_ops, parent:*mut kobject)->*mut kset { let k=kzalloc(core::mem::size_of::<kset>(),GFP_KERNEL) as *mut kset; if k.is_null(){return k;} if kobject_set_name(&mut (*k).kobj,c"%s".as_ptr(),name)!=0 {kfree(k as *mut c_void);return core::ptr::null_mut();} (*k).uevent_ops=ops;(*k).kobj.parent=parent;(*k).kobj.ktype=&KSET_KTYPE;k }
pub unsafe fn kset_create_and_add(n:*const c_char,o:*const kset_uevent_ops,p:*mut kobject)->*mut kset { let k=kset_create(n,o,p); if k.is_null(){return k;} if kset_register(k)!=0 {kfree(k as *mut c_void);return core::ptr::null_mut();} k }

static mut KOBJ_NS_OPS_TBL: [*const kobj_ns_type_operations; KOBJ_NS_TYPES as usize] = [core::ptr::null(); KOBJ_NS_TYPES as usize];
pub unsafe fn kobj_ns_type_register(ops:*const kobj_ns_type_operations)->c_int { let t=(*ops).type_; if !kobj_ns_type_is_valid(t){return -EINVAL;} if !KOBJ_NS_OPS_TBL[t as usize].is_null(){return -EBUSY;} KOBJ_NS_OPS_TBL[t as usize]=ops;0 }
pub unsafe fn kobj_ns_type_registered(t:kobj_ns_type)->c_int { if kobj_ns_type_is_valid(t)&&!KOBJ_NS_OPS_TBL[t as usize].is_null(){1}else{0} }
pub unsafe fn kobj_child_ns_ops(p:*const kobject)->*const kobj_ns_type_operations { if !p.is_null()&&!(*p).ktype.is_null(){if let Some(f)=(*(*p).ktype).child_ns_type{return f(p);}} core::ptr::null() }
pub unsafe fn kobj_ns_ops(k:*const kobject)->*const kobj_ns_type_operations { kobj_child_ns_ops((*k).parent) }
pub unsafe fn kobj_ns_current_may_mount(t:kobj_ns_type)->bool { if kobj_ns_type_is_valid(t)&&!KOBJ_NS_OPS_TBL[t as usize].is_null(){((*KOBJ_NS_OPS_TBL[t as usize]).current_may_mount)()}else{true} }
pub unsafe fn kobj_ns_grab_current(t:kobj_ns_type)->*mut ns_common { if kobj_ns_type_is_valid(t)&&!KOBJ_NS_OPS_TBL[t as usize].is_null(){((*KOBJ_NS_OPS_TBL[t as usize]).grab_current_ns)()}else{core::ptr::null_mut()} }
pub unsafe fn kobj_ns_drop(t:kobj_ns_type,ns:*mut ns_common){if kobj_ns_type_is_valid(t)&&!KOBJ_NS_OPS_TBL[t as usize].is_null(){if let Some(f)=(*KOBJ_NS_OPS_TBL[t as usize]).drop_ns{f(ns);}}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
