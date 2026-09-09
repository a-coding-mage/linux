// SPDX-License-Identifier: GPL-2.0
/*  Copyright(c) 2016-20 Intel Corporation. */

// C dependencies supplied by the surrounding kernel and SGX implementation.

pub static mut sgx_attributes_reserved_mask: u64 = 0;
pub static mut sgx_xfrm_reserved_mask: u64 = !0x3u64;
pub static mut sgx_misc_reserved_mask: u32 = 0;

unsafe extern "C" {
    fn kzalloc_obj<T>() -> *mut T;
    fn kfree<T>(ptr: *mut T);
    fn kref_init(refcount: *mut Kref);
    fn xa_init(array: *mut Xarray);
    fn mutex_init(lock: *mut Mutex);
    fn INIT_LIST_HEAD(list: *mut ListHead);
    fn spin_lock_init(lock: *mut Spinlock);
    fn init_srcu_struct(srcu: *mut SrcuStruct) -> i32;
    fn sgx_inc_usage_count() -> i32;
    fn sgx_dec_usage_count();
    fn synchronize_srcu(srcu: *mut SrcuStruct);
    fn mmu_notifier_unregister(notifier: *mut MmuNotifier, mm: *mut MmStruct);
    fn sgx_encl_release(refcount: *mut Kref);
    fn sgx_encl_may_map(encl: *mut SgxEncl, start: usize, end: usize, flags: usize) -> i32;
    fn sgx_encl_mm_add(encl: *mut SgxEncl, mm: *mut MmStruct) -> i32;
    fn sgx_ioctl(file: *mut File, cmd: u32, arg: usize) -> i64;
    fn mm_get_unmapped_area(file: *mut File, addr: usize, len: usize, pgoff: usize, flags: usize) -> usize;
    fn cpu_feature_enabled(feature: u32) -> bool;
    fn cpuid_count(leaf: u32, subleaf: u32, eax: *mut u32, ebx: *mut u32, ecx: *mut u32, edx: *mut u32);
    fn misc_register(device: *mut Miscdevice) -> i32;
    fn list_empty(list: *const ListHead) -> bool;
    fn list_first_entry(list: *const ListHead) -> *mut SgxEnclMm;
    fn list_del_rcu(list: *mut ListHead);
}

#[repr(C)] pub struct Kref { _private: [u8; 0] }
#[repr(C)] pub struct Xarray { _private: [u8; 0] }
#[repr(C)] pub struct Mutex { _private: [u8; 0] }
#[repr(C)] pub struct ListHead { next: *mut ListHead, prev: *mut ListHead }
#[repr(C)] pub struct Spinlock { _private: [u8; 0] }
#[repr(C)] pub struct SrcuStruct { _private: [u8; 0] }
#[repr(C)] pub struct MmuNotifier { _private: [u8; 0] }
#[repr(C)] pub struct MmStruct { _private: [u8; 0] }
#[repr(C)] pub struct SgxEnclMm { list: ListHead, mmu_notifier: MmuNotifier, mm: *mut MmStruct }
#[repr(C)] pub struct SgxEncl {
    refcount: Kref,
    page_array: Xarray,
    lock: Mutex,
    va_pages: ListHead,
    mm_list: ListHead,
    mm_lock: Spinlock,
    srcu: SrcuStruct,
}
#[repr(C)] pub struct Inode { _private: [u8; 0] }
#[repr(C)] pub struct File { private_data: *mut core::ffi::c_void }
#[repr(C)] pub struct VmAreaStruct { vm_start: usize, vm_end: usize, vm_flags: usize, vm_mm: *mut MmStruct, vm_ops: *const VmOperations, vm_private_data: *mut core::ffi::c_void }
#[repr(C)] pub struct VmOperations { _private: [u8; 0] }
#[repr(C)] pub struct FileOperations {
    owner: *const core::ffi::c_void,
    open: Option<unsafe fn(*mut Inode, *mut File) -> i32>,
    release: Option<unsafe fn(*mut Inode, *mut File) -> i32>,
    unlocked_ioctl: Option<unsafe fn(*mut File, u32, usize) -> i64>,
    compat_ioctl: Option<unsafe fn(*mut File, u32, usize) -> i64>,
    mmap: Option<unsafe fn(*mut File, *mut VmAreaStruct) -> i32>,
    get_unmapped_area: Option<unsafe fn(*mut File, usize, usize, usize, usize) -> usize>,
}
#[repr(C)] pub struct Miscdevice { minor: i32, name: *const u8, nodename: *const u8, fops: *const FileOperations }

unsafe extern "C" {
    static sgx_vm_ops: VmOperations;
}

const ENOMEM: i32 = 12;
const ENODEV: i32 = 19;
const MAP_TYPE: usize = 0x0f;
const MAP_PRIVATE: usize = 0x02;
const MAP_FIXED: usize = 0x10;
const VM_PFNMAP: usize = 0;
const VM_DONTEXPAND: usize = 0;
const VM_DONTDUMP: usize = 0;
const VM_IO: usize = 0;
const MISC_DYNAMIC_MINOR: i32 = 255;
const X86_FEATURE_SGX_LC: u32 = 0;
const X86_FEATURE_OSXSAVE: u32 = 0;
const SGX_CPUID: u32 = 0;
const SGX_MISC_RESERVED_MASK: u32 = 0;
const SGX_ATTR_RESERVED_MASK: u64 = 0;

unsafe fn __sgx_open(_inode: *mut Inode, file: *mut File) -> i32 {
    let encl = kzalloc_obj::<SgxEncl>();
    if encl.is_null() { return -ENOMEM; }
    kref_init(&mut (*encl).refcount);
    xa_init(&mut (*encl).page_array);
    mutex_init(&mut (*encl).lock);
    INIT_LIST_HEAD(&mut (*encl).va_pages);
    INIT_LIST_HEAD(&mut (*encl).mm_list);
    spin_lock_init(&mut (*encl).mm_lock);
    let ret = init_srcu_struct(&mut (*encl).srcu);
    if ret != 0 { kfree(encl); return ret; }
    (*file).private_data = encl.cast();
    0
}

unsafe fn sgx_open(inode: *mut Inode, file: *mut File) -> i32 {
    let ret = sgx_inc_usage_count();
    if ret != 0 { return ret; }
    let ret = __sgx_open(inode, file);
    if ret != 0 { sgx_dec_usage_count(); return ret; }
    0
}

unsafe fn sgx_release(_inode: *mut Inode, file: *mut File) -> i32 {
    let encl = (*file).private_data.cast::<SgxEncl>();
    loop {
        // Drain the remaining mm_list entries. At this point the list contains entries for processes,
        // which have closed the enclave file but have not exited yet.
        // The processes, which have exited, are gone from the list by sgx_mmu_notifier_release().
        // List manipulation is supplied by the kernel dependency.
        let encl_mm: *mut SgxEnclMm;
        if list_empty(&(*encl).mm_list) {
            encl_mm = core::ptr::null_mut();
        } else {
            encl_mm = list_first_entry(&(*encl).mm_list);
            list_del_rcu(&mut (*encl_mm).list);
        }
        if encl_mm.is_null() { break; }
        synchronize_srcu(&mut (*encl).srcu);
        mmu_notifier_unregister(&mut (*encl_mm).mmu_notifier, (*encl_mm).mm);
        kfree(encl_mm);
        kref_put(&mut (*encl).refcount, sgx_encl_release);
    }
    kref_put(&mut (*encl).refcount, sgx_encl_release);
    0
}

unsafe extern "C" { fn kref_put(refcount: *mut Kref, release: unsafe extern "C" fn(*mut Kref)); }

unsafe fn sgx_mmap(file: *mut File, vma: *mut VmAreaStruct) -> i32 {
    let encl = (*file).private_data.cast::<SgxEncl>();
    let ret = sgx_encl_may_map(encl, (*vma).vm_start, (*vma).vm_end, (*vma).vm_flags);
    if ret != 0 { return ret; }
    let ret = sgx_encl_mm_add(encl, (*vma).vm_mm);
    if ret != 0 { return ret; }
    (*vma).vm_ops = &sgx_vm_ops;
    (*vma).vm_flags |= VM_PFNMAP | VM_DONTEXPAND | VM_DONTDUMP | VM_IO;
    (*vma).vm_private_data = encl.cast();
    0
}

unsafe fn sgx_get_unmapped_area(file: *mut File, addr: usize, len: usize, pgoff: usize, flags: usize) -> usize {
    if flags & MAP_TYPE == MAP_PRIVATE { return (-22i32) as usize; }
    if flags & MAP_FIXED != 0 { return addr; }
    mm_get_unmapped_area(file, addr, len, pgoff, flags)
}

#[cfg(feature = "CONFIG_COMPAT")]
unsafe fn sgx_compat_ioctl(filep: *mut File, cmd: u32, arg: usize) -> i64 { sgx_ioctl(filep, cmd, arg) }

#[no_mangle]
pub unsafe extern "C" fn sgx_drv_init() -> i32 {
    let (mut eax, mut ebx, mut ecx, mut edx) = (0u32, 0u32, 0u32, 0u32);
    let (mut attr_mask, mut xfrm_mask): (u64, u64);
    if !cpu_feature_enabled(X86_FEATURE_SGX_LC) { return -ENODEV; }
    cpuid_count(SGX_CPUID, 0, &mut eax, &mut ebx, &mut ecx, &mut edx);
    if eax & 1 == 0 { return -ENODEV; }
    sgx_misc_reserved_mask = !ebx | SGX_MISC_RESERVED_MASK;
    cpuid_count(SGX_CPUID, 1, &mut eax, &mut ebx, &mut ecx, &mut edx);
    attr_mask = ((ebx as u64) << 32) + eax as u64;
    sgx_attributes_reserved_mask = !attr_mask | SGX_ATTR_RESERVED_MASK;
    if cpu_feature_enabled(X86_FEATURE_OSXSAVE) {
        xfrm_mask = ((edx as u64) << 32) + ecx as u64;
        sgx_xfrm_reserved_mask = !xfrm_mask;
    }
    let ret = misc_register(&mut sgx_dev_enclave);
    if ret != 0 { return ret; }
    0
}

#[no_mangle]
pub static sgx_encl_fops: FileOperations = FileOperations {
    owner: core::ptr::null(),
    open: Some(sgx_open),
    release: Some(sgx_release),
    unlocked_ioctl: Some(sgx_ioctl),
    compat_ioctl: None,
    mmap: Some(sgx_mmap),
    get_unmapped_area: Some(sgx_get_unmapped_area),
};

#[no_mangle]
pub static mut sgx_dev_enclave: Miscdevice = Miscdevice {
    minor: MISC_DYNAMIC_MINOR,
    name: b"sgx_enclave\0".as_ptr(),
    nodename: b"sgx_enclave\0".as_ptr(),
    fops: &sgx_encl_fops,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
