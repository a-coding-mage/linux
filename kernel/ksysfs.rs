// SPDX-License-Identifier: GPL-2.0-only
/*
 * kernel/ksysfs.c - sysfs attributes in /sys/kernel, which
 *                 are not related to any other subsystem
 *
 * Copyright (C) 2004 Kay Sievers <kay.sievers@vrfy.org>
 */

// Kernel headers supplying the declarations below are external dependencies.

#[cfg(target_endian = "little")]
const CPU_BYTEORDER_STRING: *const u8 = b"little\0".as_ptr();
#[cfg(target_endian = "big")]
const CPU_BYTEORDER_STRING: *const u8 = b"big\0".as_ptr();

extern "C" {
    static uevent_seqnum: Atomic64;
    #[cfg(feature = "uevent_helper")]
    static mut uevent_helper: [u8; UEVENT_HELPER_PATH_LEN];
    #[cfg(feature = "profiling")]
    static mut prof_on: c_int;
    static mut file_caps_enabled: c_int;
    #[cfg(not(feature = "tiny_rcu"))]
    static mut rcu_normal: c_int;
    fn atomic64_read(v: *const Atomic64) -> u64;
    fn sysfs_emit(buf: *mut c_char, fmt: *const c_char, ...) -> isize;
    fn kstrtoint(buf: *const c_char, base: c_uint, out: *mut c_int) -> c_int;
    #[cfg(feature = "profiling")]
    fn profile_setup(buf: *mut c_char);
    #[cfg(feature = "profiling")]
    fn profile_init() -> c_int;
    #[cfg(feature = "profiling")]
    fn create_proc_profile() -> c_int;
}

#[repr(C)] pub struct Atomic64 { _private: [u8; 0] }
#[repr(C)] pub struct Kobject { _private: [u8; 0] }
#[repr(C)] pub struct KobjAttribute { pub attr: Attribute, _private: [u8; 0] }
#[repr(C)] pub struct Attribute { _private: [u8; 0] }
#[repr(C)] pub struct AttributeGroup { pub attrs: *mut *mut Attribute }
#[repr(C)] pub struct BinAttribute { pub private: *mut c_void, pub size: usize, _private: [u8; 0] }
type PhysAddr = usize;
type ssize_t = isize;
type c_int = i32;
type c_uint = u32;
type c_char = i8;
type c_void = core::ffi::c_void;

const ENOENT: ssize_t = -2;
const EINVAL: ssize_t = -22;
const EEXIST: ssize_t = -17;
const ENOMEM: c_int = 12;
#[cfg(feature = "uevent_helper")]
const UEVENT_HELPER_PATH_LEN: usize = 256;

unsafe extern "C" {
    static mut kernel_kobj: *mut Kobject;
    static __start_notes: c_void;
    static __stop_notes: c_void;
    static mut bin_attr_notes: BinAttribute;
    fn paddr_vmcoreinfo_note() -> PhysAddr;
    fn kobject_create_and_add(name: *const c_char, parent: *mut Kobject) -> *mut Kobject;
    fn sysfs_create_group(kobj: *mut Kobject, grp: *const AttributeGroup) -> c_int;
    fn sysfs_create_bin_file(kobj: *mut Kobject, attr: *mut BinAttribute) -> c_int;
    fn sysfs_remove_group(kobj: *mut Kobject, grp: *const AttributeGroup);
    fn kobject_put(kobj: *mut Kobject);
    fn pr_err(fmt: *const c_char, ...);
}

unsafe fn uevent_seqnum_show(_: *mut Kobject, _: *mut KobjAttribute, buf: *mut c_char) -> ssize_t {
    sysfs_emit(buf, b"%llu\n\0".as_ptr() as *const c_char, atomic64_read(&uevent_seqnum))
}
unsafe fn cpu_byteorder_show(_: *mut Kobject, _: *mut KobjAttribute, buf: *mut c_char) -> ssize_t {
    sysfs_emit(buf, b"%s\n\0".as_ptr() as *const c_char, CPU_BYTEORDER_STRING)
}
unsafe fn address_bits_show(_: *mut Kobject, _: *mut KobjAttribute, buf: *mut c_char) -> ssize_t {
    sysfs_emit(buf, b"%zu\n\0".as_ptr() as *const c_char, core::mem::size_of::<*const c_void>() * 8)
}

unsafe fn fscaps_show(_: *mut Kobject, _: *mut KobjAttribute, buf: *mut c_char) -> ssize_t {
    sysfs_emit(buf, b"%d\n\0".as_ptr() as *const c_char, file_caps_enabled)
}

#[cfg(feature = "profiling")]
unsafe fn profiling_show(_: *mut Kobject, _: *mut KobjAttribute, buf: *mut c_char) -> ssize_t {
    sysfs_emit(buf, b"%d\n\0".as_ptr() as *const c_char, prof_on)
}
#[cfg(feature = "profiling")]
unsafe fn profiling_store(_: *mut Kobject, _: *mut KobjAttribute, buf: *const c_char, count: usize) -> ssize_t {
    if prof_on != 0 { return EEXIST; }
    profile_setup(buf as *mut c_char);
    let mut ret = profile_init();
    if ret != 0 { return ret as ssize_t; }
    ret = create_proc_profile();
    if ret != 0 { return ret as ssize_t; }
    count as ssize_t
}

#[cfg(feature = "vmcore_info")]
unsafe fn vmcoreinfo_show(_: *mut Kobject, _: *mut KobjAttribute, buf: *mut c_char) -> ssize_t {
    let vmcore_base = paddr_vmcoreinfo_note();
    sysfs_emit(buf, b"%pa %x\n\0".as_ptr() as *const c_char, &vmcore_base, VMCOREINFO_NOTE_SIZE as c_uint)
}

#[cfg(not(feature = "tiny_rcu"))]
unsafe fn rcu_expedited_show(_: *mut Kobject, _: *mut KobjAttribute, buf: *mut c_char) -> ssize_t {
    sysfs_emit(buf, b"%d\n\0".as_ptr() as *const c_char, core::ptr::read_volatile(&rcu_expedited))
}
#[cfg(not(feature = "tiny_rcu"))]
unsafe fn rcu_normal_show(_: *mut Kobject, _: *mut KobjAttribute, buf: *mut c_char) -> ssize_t {
    sysfs_emit(buf, b"%d\n\0".as_ptr() as *const c_char, core::ptr::read_volatile(&rcu_normal))
}

#[cfg(feature = "uevent_helper")]
unsafe fn uevent_helper_store(_: *mut Kobject, _: *mut KobjAttribute, buf: *const c_char, count: usize) -> ssize_t {
    if count + 1 > UEVENT_HELPER_PATH_LEN { return ENOENT; }
    core::ptr::copy_nonoverlapping(buf as *const u8, uevent_helper.as_mut_ptr(), count);
    uevent_helper[count] = 0;
    if count != 0 && uevent_helper[count - 1] == b'\n' { uevent_helper[count - 1] = 0; }
    count as ssize_t
}

#[cfg(not(feature = "tiny_rcu"))]
static mut rcu_expedited: c_int = 0;
#[cfg(not(feature = "tiny_rcu"))]
unsafe fn rcu_expedited_store(_: *mut Kobject, _: *mut KobjAttribute, buf: *const c_char, count: usize) -> ssize_t {
    if kstrtoint(buf, 0, &mut rcu_expedited) != 0 { return EINVAL; }
    count as ssize_t
}
#[cfg(not(feature = "tiny_rcu"))]
unsafe fn rcu_normal_store(_: *mut Kobject, _: *mut KobjAttribute, buf: *const c_char, count: usize) -> ssize_t {
    if kstrtoint(buf, 0, &mut rcu_normal) != 0 { return EINVAL; }
    count as ssize_t
}

static mut kernel_attrs: [*mut Attribute; 1] = [core::ptr::null_mut()];
static kernel_attr_group: AttributeGroup = AttributeGroup { attrs: core::ptr::null_mut() };

#[no_mangle]
pub unsafe extern "C" fn ksysfs_init() {
    let mut error: c_int;
    kernel_kobj = kobject_create_and_add(b"kernel\0".as_ptr() as *const c_char, core::ptr::null_mut());
    if kernel_kobj.is_null() { error = -ENOMEM; pr_err(b"failed to initialize the kernel kobject: %d\n\0".as_ptr() as *const c_char, error); return; }
    error = sysfs_create_group(kernel_kobj, &kernel_attr_group);
    if error != 0 { kobject_put(kernel_kobj); pr_err(b"failed to initialize the kernel kobject: %d\n\0".as_ptr() as *const c_char, error); return; }
    let notes_size = (&__stop_notes as *const c_void as usize).wrapping_sub(&__start_notes as *const c_void as usize);
    if notes_size > 0 {
        bin_attr_notes.private = &__start_notes as *const c_void as *mut c_void;
        bin_attr_notes.size = notes_size;
        error = sysfs_create_bin_file(kernel_kobj, &mut bin_attr_notes);
        if error != 0 { sysfs_remove_group(kernel_kobj, &kernel_attr_group); kobject_put(kernel_kobj); pr_err(b"failed to initialize the kernel kobject: %d\n\0".as_ptr() as *const c_char, error); }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
