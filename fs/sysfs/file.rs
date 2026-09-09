// SPDX-License-Identifier: GPL-2.0
/* fs/sysfs/file.c - sysfs regular (text) file implementation */

// C headers and "sysfs.h" provide the kernel types, constants, and functions
// referenced below.

unsafe fn sysfs_file_kobj(kn: *mut kernfs_node) -> *mut kobject {
    // guard(rcu)();
    (*(*kn).__parent).priv_
}

unsafe fn sysfs_file_ops(kn: *mut kernfs_node) -> *const sysfs_ops {
    let kobj = sysfs_file_kobj(kn);
    if (*kn).flags & KERNFS_LOCKDEP != 0 {
        lockdep_assert_held(kn);
    }
    if !(*(*kobj).ktype).sysfs_ops.is_null() {
        (*(*kobj).ktype).sysfs_ops
    } else {
        core::ptr::null()
    }
}

unsafe extern "C" fn sysfs_kf_seq_show(sf: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    let of = (*sf).private as *mut kernfs_open_file;
    let kobj = sysfs_file_kobj((*of).kn);
    let ops = sysfs_file_ops((*of).kn);
    let mut count: isize;
    let mut buf: *mut i8 = core::ptr::null_mut();

    if (*ops).show.is_none() {
        WARN_ON_ONCE(true);
        return -EINVAL;
    }
    count = seq_get_buf(sf, &mut buf);
    if count < PAGE_SIZE as isize {
        seq_commit(sf, -1);
        return 0;
    }
    core::ptr::write_bytes(buf, 0, PAGE_SIZE);
    count = ((*ops).show.unwrap())(kobj, (*(*of).kn).priv_, buf);
    if count < 0 { return count as i32; }
    if count >= PAGE_SIZE as isize {
        WARN(true, "OOB write or bad count %zd at %pS\n", count, (*ops).show);
        count = PAGE_SIZE as isize - 1;
    }
    seq_commit(sf, count);
    0
}

unsafe extern "C" fn sysfs_kf_bin_read(of: *mut kernfs_open_file, buf: *mut i8,
                                         mut count: usize, pos: loff_t) -> isize {
    let battr = (*of).kn_priv::<bin_attribute>();
    let kobj = sysfs_file_kobj((*of).kn);
    let size = file_inode((*of).file).i_size;
    if count == 0 { return 0; }
    if size != 0 {
        if pos >= size { return 0; }
        if pos + count as i64 > size { count = (size - pos) as usize; }
    }
    if (*battr).read.is_none() { return -EIO as isize; }
    ((*battr).read.unwrap())((*of).file, kobj, battr, buf, pos, count)
}

unsafe extern "C" fn sysfs_kf_read(of: *mut kernfs_open_file, buf: *mut i8,
                                    count: usize, pos: loff_t) -> isize {
    let ops = sysfs_file_ops((*of).kn);
    let kobj = sysfs_file_kobj((*of).kn);
    if buf != (*of).prealloc_buf { WARN_ON_ONCE(true); return 0; }
    let mut len = ((*ops).show.unwrap())(kobj, (*(*of).kn).priv_, buf);
    if len < 0 { return len; }
    if len >= PAGE_SIZE as isize { printk("fill_read_buffer: %pS returned bad count\n", (*ops).show); len = PAGE_SIZE as isize - 1; }
    if pos != 0 {
        if len <= pos as isize { return 0; }
        len -= pos as isize;
        core::ptr::copy(buf.add(pos as usize), buf, len as usize);
    }
    core::cmp::min(count as isize, len)
}

unsafe extern "C" fn sysfs_kf_write(of: *mut kernfs_open_file, buf: *mut i8,
                                     count: usize, _pos: loff_t) -> isize {
    let ops = sysfs_file_ops((*of).kn);
    let kobj = sysfs_file_kobj((*of).kn);
    if count == 0 { return 0; }
    ((*ops).store.unwrap())(kobj, (*(*of).kn).priv_, buf, count)
}

unsafe extern "C" fn sysfs_kf_bin_write(of: *mut kernfs_open_file, buf: *mut i8,
                                         mut count: usize, pos: loff_t) -> isize {
    let battr = (*of).kn_priv::<bin_attribute>();
    let kobj = sysfs_file_kobj((*of).kn);
    let size = file_inode((*of).file).i_size;
    if size != 0 {
        if size <= pos { return -EFBIG as isize; }
        count = core::cmp::min(count, (size - pos) as usize);
    }
    if count == 0 { return 0; }
    if (*battr).write.is_none() { return -EIO as isize; }
    ((*battr).write.unwrap())((*of).file, kobj, battr, buf, pos, count)
}

unsafe extern "C" fn sysfs_kf_bin_mmap(of: *mut kernfs_open_file, vma: *mut vm_area_struct) -> i32 {
    let battr = (*of).kn_priv::<bin_attribute>();
    ((*battr).mmap.unwrap())((*of).file, sysfs_file_kobj((*of).kn), battr, vma)
}

unsafe extern "C" fn sysfs_kf_bin_llseek(of: *mut kernfs_open_file, offset: loff_t, whence: i32) -> loff_t {
    let battr = (*of).kn_priv::<bin_attribute>();
    let kobj = sysfs_file_kobj((*of).kn);
    match (*battr).llseek { Some(f) => f((*of).file, kobj, battr, offset, whence), None => generic_file_llseek((*of).file, offset, whence) }
}

unsafe extern "C" fn sysfs_kf_bin_open(of: *mut kernfs_open_file) -> i32 {
    let battr = (*of).kn_priv::<bin_attribute>();
    if let Some(f) = (*battr).f_mapping { (*of).file.f_mapping = f(); }
    0
}

pub unsafe extern "C" fn sysfs_notify(kobj: *mut kobject, dir: *const i8, attr: *const i8) {
    let mut kn = (*kobj).sd;
    let mut tmp: *mut kernfs_node;
    if !kn.is_null() && !dir.is_null() { kn = kernfs_find_and_get(kn, dir); } else { kernfs_get(kn); }
    if !kn.is_null() && !attr.is_null() { tmp = kernfs_find_and_get(kn, attr); kernfs_put(kn); kn = tmp; }
    if !kn.is_null() { kernfs_notify(kn); kernfs_put(kn); }
}

// The following operation tables correspond directly to the C initializer
// tables; their fields and function-pointer types are supplied by sysfs.h.
static mut sysfs_file_kfops_empty: kernfs_ops = kernfs_ops::empty();
static mut sysfs_file_kfops_ro: kernfs_ops = kernfs_ops { seq_show: Some(sysfs_kf_seq_show), ..kernfs_ops::empty() };
static mut sysfs_file_kfops_wo: kernfs_ops = kernfs_ops { write: Some(sysfs_kf_write), ..kernfs_ops::empty() };
static mut sysfs_file_kfops_rw: kernfs_ops = kernfs_ops { seq_show: Some(sysfs_kf_seq_show), write: Some(sysfs_kf_write), ..kernfs_ops::empty() };
static mut sysfs_prealloc_kfops_ro: kernfs_ops = kernfs_ops { read: Some(sysfs_kf_read), prealloc: true, ..kernfs_ops::empty() };
static mut sysfs_prealloc_kfops_wo: kernfs_ops = kernfs_ops { write: Some(sysfs_kf_write), prealloc: true, ..kernfs_ops::empty() };
static mut sysfs_prealloc_kfops_rw: kernfs_ops = kernfs_ops { read: Some(sysfs_kf_read), write: Some(sysfs_kf_write), prealloc: true, ..kernfs_ops::empty() };
static mut sysfs_bin_kfops_ro: kernfs_ops = kernfs_ops { read: Some(sysfs_kf_bin_read), ..kernfs_ops::empty() };
static mut sysfs_bin_kfops_wo: kernfs_ops = kernfs_ops { write: Some(sysfs_kf_bin_write), ..kernfs_ops::empty() };
static mut sysfs_bin_kfops_rw: kernfs_ops = kernfs_ops { read: Some(sysfs_kf_bin_read), write: Some(sysfs_kf_bin_write), ..kernfs_ops::empty() };
static mut sysfs_bin_kfops_mmap: kernfs_ops = kernfs_ops { read: Some(sysfs_kf_bin_read), write: Some(sysfs_kf_bin_write), mmap: Some(sysfs_kf_bin_mmap), open: Some(sysfs_kf_bin_open), llseek: Some(sysfs_kf_bin_llseek), ..kernfs_ops::empty() };

// Remaining exported helpers retain the C control flow and call the external
// kernfs/sysfs interfaces declared by the surrounding translation unit.
pub unsafe extern "C" fn sysfs_add_file_mode_ns(parent: *mut kernfs_node, attr: *const attribute, mode: umode_t, uid: kuid_t, gid: kgid_t, ns: *const ns_common) -> i32 {
    let kobj = (*parent).priv_ as *mut kobject;
    let sysfs_ops = (*(*kobj).ktype).sysfs_ops;
    if sysfs_ops.is_null() { WARN(true, "missing sysfs attribute operations for kobject: %s\n", kobject_name(kobj)); return -EINVAL; }
    let ops = if mode & SYSFS_PREALLOC != 0 { if (*sysfs_ops).show.is_some() && (*sysfs_ops).store.is_some() { &sysfs_prealloc_kfops_rw } else if (*sysfs_ops).show.is_some() { &sysfs_prealloc_kfops_ro } else if (*sysfs_ops).store.is_some() { &sysfs_prealloc_kfops_wo } else { &sysfs_file_kfops_empty } } else if (*sysfs_ops).show.is_some() && (*sysfs_ops).store.is_some() { &sysfs_file_kfops_rw } else if (*sysfs_ops).show.is_some() { &sysfs_file_kfops_ro } else if (*sysfs_ops).store.is_some() { &sysfs_file_kfops_wo } else { &sysfs_file_kfops_empty };
    let kn = __kernfs_create_file(parent, (*attr).name, mode & 0o777, uid, gid, PAGE_SIZE, ops, attr as *mut _, ns, core::ptr::null_mut());
    if IS_ERR(kn) { if PTR_ERR(kn) == -EEXIST { sysfs_warn_dup(parent, (*attr).name); } return PTR_ERR(kn); } 0
}

pub unsafe extern "C" fn sysfs_add_bin_file_mode_ns(parent: *mut kernfs_node, battr: *const bin_attribute, mode: umode_t, size: usize, uid: kuid_t, gid: kgid_t, ns: *const ns_common) -> i32 {
    let attr = &(*battr).attr;
    let ops = if (*battr).mmap.is_some() { &sysfs_bin_kfops_mmap } else if (*battr).read.is_some() && (*battr).write.is_some() { &sysfs_bin_kfops_rw } else if (*battr).read.is_some() { &sysfs_bin_kfops_ro } else if (*battr).write.is_some() { &sysfs_bin_kfops_wo } else { &sysfs_file_kfops_empty };
    let kn = __kernfs_create_file(parent, attr.name, mode & 0o777, uid, gid, size, ops, attr as *const _ as *mut _, ns, core::ptr::null_mut());
    if IS_ERR(kn) { if PTR_ERR(kn) == -EEXIST { sysfs_warn_dup(parent, attr.name); } return PTR_ERR(kn); } 0
}

pub unsafe extern "C" fn sysfs_create_file_ns(kobj: *mut kobject, attr: *const attribute, ns: *const ns_common) -> i32 {
    if WARN_ON(kobj.is_null() || (*kobj).sd.is_null() || attr.is_null()) { return -EINVAL; }
    let (mut uid, mut gid) = (kuid_t::default(), kgid_t::default()); kobject_get_ownership(kobj, &mut uid, &mut gid); sysfs_add_file_mode_ns((*kobj).sd, attr, (*attr).mode, uid, gid, ns)
}

pub unsafe extern "C" fn sysfs_create_files(kobj: *mut kobject, ptr: *const *const attribute) -> i32 {
    let mut err = 0; let mut i = 0; while !(*ptr.add(i)).is_null() && err == 0 { err = sysfs_create_file(kobj, *ptr.add(i)); i += 1; } if err != 0 { while i > 0 { i -= 1; sysfs_remove_file(kobj, *ptr.add(i)); } } err
}

pub unsafe extern "C" fn sysfs_add_file_to_group(kobj: *mut kobject, attr: *const attribute, group: *const i8) -> i32 {
    let parent = if !group.is_null() { kernfs_find_and_get((*kobj).sd, group) } else { kernfs_get((*kobj).sd); (*kobj).sd }; if parent.is_null() { return -ENOENT; }
    let (mut uid, mut gid) = (kuid_t::default(), kgid_t::default()); kobject_get_ownership(kobj, &mut uid, &mut gid); let e = sysfs_add_file_mode_ns(parent, attr, (*attr).mode, uid, gid, core::ptr::null()); kernfs_put(parent); e
}

pub unsafe extern "C" fn sysfs_chmod_file(kobj: *mut kobject, attr: *const attribute, mode: umode_t) -> i32 {
    let kn = kernfs_find_and_get((*kobj).sd, (*attr).name); if kn.is_null() { return -ENOENT; } let mut a = iattr { ia_mode: (mode & S_IALLUGO) | ((*kn).mode & !S_IALLUGO), ia_valid: ATTR_MODE }; let rc = kernfs_setattr(kn, &mut a); kernfs_put(kn); rc
}

pub unsafe extern "C" fn sysfs_break_active_protection(kobj: *mut kobject, attr: *const attribute) -> *mut kernfs_node {
    kobject_get(kobj); let kn = kernfs_find_and_get((*kobj).sd, (*attr).name); if !kn.is_null() { kernfs_break_active_protection(kn); } else { kobject_put(kobj); } kn
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
