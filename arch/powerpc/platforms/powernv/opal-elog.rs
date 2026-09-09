// SPDX-License-Identifier: GPL-2.0-or-later
/* Error log support on PowerNV. */

use core::ffi::{c_char, c_int, c_void};

// Kernel declarations and macros supplied by the surrounding translation unit.
extern "C" {
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> isize;
    fn sysfs_remove_file_self(kobj: *mut kobject, attr: *mut attribute) -> bool;
    fn opal_send_ack_elog(id: u64);
    fn kobject_put(kobj: *mut kobject);
    fn kobject_get(kobj: *mut kobject);
    fn kobject_init(kobj: *mut kobject, ktype: *const kobj_type);
    fn kobject_add(kobj: *mut kobject, parent: *mut kobject, fmt: *const c_char, ...) -> c_int;
    fn sysfs_create_bin_file(kobj: *mut kobject, attr: *mut bin_attribute) -> c_int;
    fn kobject_uevent(kobj: *mut kobject, action: c_int) -> c_int;
    fn opal_read_elog(addr: u64, size: usize, id: u64) -> c_int;
    fn opal_get_elog_size(id: *mut u64, size: *mut u64, typ: *mut u64) -> c_int;
    fn opal_check_token(token: c_int) -> bool;
    fn kset_create_and_add(name: *const c_char, parent: *mut c_void, kobj: *mut kobject) -> *mut kset;
    fn opal_event_request(event: c_int) -> c_int;
    fn request_threaded_irq(irq: c_int, handler: *mut c_void, thread_fn: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_int, name: *const c_char, dev: *mut c_void) -> c_int;
    fn opal_resend_pending_logs();
    fn kset_find_obj(kset: *mut kset, name: *const c_char) -> *mut kobject;
    fn kzalloc_obj<T>() -> *mut T;
    fn kzalloc(size: usize, flags: c_int) -> *mut c_char;
    fn kfree(ptr: *mut c_void);
    fn memcpy(dst: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
}

#[repr(C)] pub struct kobject { pub kset: *mut kset }
#[repr(C)] pub struct kset;
#[repr(C)] pub struct attribute { pub name: *const c_char, pub mode: u16 }
#[repr(C)] pub struct bin_attribute { pub attr: attribute, pub size: usize, pub read: Option<unsafe extern "C" fn(*mut file, *mut kobject, *const bin_attribute, *mut c_char, i64, usize) -> isize> }
#[repr(C)] pub struct file;
#[repr(C)] pub struct kobj_type { pub sysfs_ops: *const sysfs_ops, pub release: Option<unsafe extern "C" fn(*mut kobject)> }
#[repr(C)] pub struct sysfs_ops { pub show: Option<unsafe extern "C" fn(*mut kobject, *mut attribute, *mut c_char) -> isize>, pub store: Option<unsafe extern "C" fn(*mut kobject, *mut attribute, *const c_char, usize) -> isize> }
#[repr(C)] pub struct elog_obj { pub kobj: kobject, pub raw_attr: bin_attribute, pub id: u64, pub typ: u64, pub size: usize, pub buffer: *mut c_char }
#[repr(C)] pub struct elog_attribute { pub attr: attribute, pub show: Option<unsafe extern "C" fn(*mut elog_obj, *mut elog_attribute, *mut c_char) -> isize>, pub store: Option<unsafe extern "C" fn(*mut elog_obj, *mut elog_attribute, *const c_char, usize) -> isize> }
pub type irqreturn_t = c_int;

const OPAL_SUCCESS: c_int = 0;
const OPAL_MAX_ERRLOG_SIZE: usize = 16384;
const EIO: isize = 5;
const IRQ_HANDLED: irqreturn_t = 1;
const KOBJ_ADD: c_int = 0;
static mut elog_kset: *mut kset = core::ptr::null_mut();

unsafe fn elog_id_show(e: *mut elog_obj, _a: *mut elog_attribute, b: *mut c_char) -> isize { sysfs_emit(b, b"0x%llx\n\0".as_ptr() as _, (*e).id) }
unsafe fn elog_type_to_string(t: u64) -> *const c_char { if t == 0 { b"PEL\0".as_ptr() as _ } else { b"unknown\0".as_ptr() as _ } }
unsafe fn elog_type_show(e: *mut elog_obj, _a: *mut elog_attribute, b: *mut c_char) -> isize { sysfs_emit(b, b"0x%llx %s\n\0".as_ptr() as _, (*e).typ, elog_type_to_string((*e).typ)) }
unsafe fn elog_ack_show(_e: *mut elog_obj, _a: *mut elog_attribute, b: *mut c_char) -> isize { sysfs_emit(b, b"ack - acknowledge log message\n\0".as_ptr() as _) }
unsafe fn elog_ack_store(e: *mut elog_obj, a: *mut elog_attribute, _b: *const c_char, count: usize) -> isize { if sysfs_remove_file_self(&mut (*e).kobj, &mut (*a).attr) { opal_send_ack_elog((*e).id); kobject_put(&mut (*e).kobj); } count as isize }

unsafe extern "C" fn elog_attr_show(k: *mut kobject, a: *mut attribute, b: *mut c_char) -> isize { let ea = a as *mut elog_attribute; let e = k as *mut elog_obj; match (*ea).show { Some(f) => f(e, ea, b), None => -EIO } }
unsafe extern "C" fn elog_attr_store(k: *mut kobject, a: *mut attribute, b: *const c_char, n: usize) -> isize { let ea = a as *mut elog_attribute; let e = k as *mut elog_obj; match (*ea).store { Some(f) => f(e, ea, b, n), None => -EIO } }
unsafe extern "C" fn elog_release(k: *mut kobject) { let e = k as *mut elog_obj; kfree((*e).buffer as _); kfree(e as _); }

unsafe extern "C" fn raw_attr_read(_f: *mut file, k: *mut kobject, _a: *const bin_attribute, b: *mut c_char, pos: i64, count: usize) -> isize { let e = k as *mut elog_obj; if (*e).buffer.is_null() { (*e).buffer = kzalloc((*e).size, 0); if (*e).buffer.is_null() { return -EIO; } if opal_read_elog((*e).buffer as u64, (*e).size, (*e).id) != OPAL_SUCCESS { kfree((*e).buffer as _); (*e).buffer = core::ptr::null_mut(); return -EIO; } } memcpy(b, (*e).buffer.add(pos as usize), count); count as isize }

unsafe fn create_elog_obj(id: u64, size: usize, typ: u64) { let e = kzalloc_obj::<elog_obj>(); if e.is_null() { return; } (*e).kobj.kset = elog_kset; (*e).id = id; (*e).size = size; (*e).typ = typ; (*e).buffer = kzalloc(size, 0); if !(*e).buffer.is_null() && opal_read_elog((*e).buffer as u64, size, id) != OPAL_SUCCESS { kfree((*e).buffer as _); (*e).buffer = core::ptr::null_mut(); } if kobject_add(&mut (*e).kobj, core::ptr::null_mut(), b"0x%llx\0".as_ptr() as _, id) != 0 { kobject_put(&mut (*e).kobj); return; } kobject_get(&mut (*e).kobj); if sysfs_create_bin_file(&mut (*e).kobj, &mut (*e).raw_attr) == 0 { kobject_uevent(&mut (*e).kobj, KOBJ_ADD); } else { kobject_put(&mut (*e).kobj); } kobject_put(&mut (*e).kobj); }

unsafe extern "C" fn elog_event(_irq: c_int, _data: *mut c_void) -> irqreturn_t { let mut size=0u64; let mut id=0u64; let mut typ=0u64; if opal_get_elog_size(&mut id,&mut size,&mut typ) != OPAL_SUCCESS { return IRQ_HANDLED; } if size >= OPAL_MAX_ERRLOG_SIZE as u64 { size=OPAL_MAX_ERRLOG_SIZE as u64; } create_elog_obj(id,size as usize,typ); IRQ_HANDLED }

pub unsafe extern "C" fn opal_elog_init() -> c_int { if !opal_check_token(0) { return -1; } elog_kset=kset_create_and_add(b"elog\0".as_ptr() as _,core::ptr::null_mut(),core::ptr::null_mut()); if elog_kset.is_null() { return -1; } let irq=opal_event_request(0); if irq==0 { return irq; } let rc=request_threaded_irq(irq,core::ptr::null_mut(),elog_event,0,b"opal-elog\0".as_ptr() as _,core::ptr::null_mut()); if rc!=0 { return rc; } if opal_check_token(0) { opal_resend_pending_logs(); } 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
