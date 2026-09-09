// SPDX-License-Identifier: GPL-2.0
/*
 * Framework for userspace DMA-BUF allocations
 *
 * Copyright (C) 2011 Google, Inc.
 * Copyright (C) 2019 Linaro Ltd.
 */

// Kernel dependencies supplied by other translation units/headers.

const DEVNAME: *const u8 = b"dma_heap\0".as_ptr();
const NUM_HEAP_MINORS: usize = 128;

#[repr(C)]
pub struct DmaHeap {
    pub name: *const i8,
    pub ops: *const DmaHeapOps,
    pub priv_: *mut core::ffi::c_void,
    pub heap_devt: DevT,
    pub list: ListHead,
    pub heap_cdev: Cdev,
}

#[repr(C)]
pub struct DmaHeapOps {
    pub allocate: Option<unsafe extern "C" fn(*mut DmaHeap, usize, u32, u64) -> *mut DmaBuf>,
}

#[repr(C)]
pub struct DmaHeapExportInfo {
    pub name: *const i8,
    pub ops: *const DmaHeapOps,
    pub priv_: *mut core::ffi::c_void,
}

pub type DevT = u64;
pub type UmodeT = u32;

#[repr(C)] pub struct ListHead { pub next: *mut ListHead, pub prev: *mut ListHead }
#[repr(C)] pub struct Cdev { _private: [u8; 0] }
#[repr(C)] pub struct DmaBuf { _private: [u8; 0] }
#[repr(C)] pub struct Inode { _private: [u8; 0] }
#[repr(C)] pub struct File { pub private_data: *mut core::ffi::c_void }
#[repr(C)] pub struct Device { _private: [u8; 0] }
#[repr(C)] pub struct Class { pub devnode: Option<unsafe extern "C" fn(*const Device, *mut UmodeT) -> *mut i8> }

extern "C" {
    static mut dma_heap_devt: DevT;
    static mut dma_heap_class: *mut Class;
    static mut dma_heap_minors: XArray;
    fn page_align(len: usize) -> usize;
    fn dma_buf_fd(buf: *mut DmaBuf, flags: u32) -> i32;
    fn dma_buf_put(buf: *mut DmaBuf);
    fn xa_load(array: *mut XArray, index: usize) -> *mut DmaHeap;
    fn iminor(inode: *mut Inode) -> i32;
    fn nonseekable_open(inode: *mut Inode, file: *mut File) -> i32;
    fn kmalloc(size: usize, flags: u32) -> *mut i8;
    fn kfree(ptr: *mut i8);
    fn copy_from_user(to: *mut i8, from: *const core::ffi::c_void, n: usize) -> usize;
    fn copy_to_user(to: *mut core::ffi::c_void, from: *const i8, n: usize) -> usize;
    fn memset(ptr: *mut i8, value: i32, n: usize) -> *mut i8;
    fn xa_alloc(array: *mut XArray, id: *mut u32, entry: *mut DmaHeap, limit: XaLimit, flags: u32) -> i32;
    fn cdev_init(cdev: *mut Cdev, ops: *const FileOperations);
    fn cdev_add(cdev: *mut Cdev, dev: DevT, count: u32) -> i32;
    fn cdev_del(cdev: *mut Cdev);
    fn device_create(class: *mut Class, parent: *mut Device, dev: DevT, data: *mut core::ffi::c_void, name: *const i8) -> *mut Device;
    fn device_destroy(class: *mut Class, dev: DevT);
    fn mutex_lock(lock: *mut Mutex);
    fn mutex_unlock(lock: *mut Mutex);
    fn kasprintf(flags: u32, fmt: *const i8, ...) -> *mut i8;
    fn dev_name(dev: *const Device) -> *const i8;
    fn alloc_chrdev_region(dev: *mut DevT, first: u32, count: u32, name: *const u8) -> i32;
    fn unregister_chrdev_region(dev: DevT, count: u32);
    fn class_create(name: *const u8) -> *mut Class;
    fn ptr_err<T>(ptr: *const T) -> i32;
}

#[repr(C)] pub struct XArray { _private: [u8; 0] }
#[repr(C)] pub struct Mutex { _private: [u8; 0] }
#[repr(C)] pub struct XaLimit { pub min: u32, pub max: u32 }
#[repr(C)] pub struct FileOperations { pub owner: *mut core::ffi::c_void, pub open: Option<unsafe extern "C" fn(*mut Inode, *mut File) -> i32>, pub unlocked_ioctl: Option<unsafe extern "C" fn(*mut File, u32, usize) -> isize>, pub compat_ioctl: Option<unsafe extern "C" fn(*mut File, u32, usize) -> isize> }

static mut HEAP_LIST: ListHead = ListHead { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut HEAP_LIST_LOCK: Mutex = Mutex { _private: [] };

#[repr(C)] pub struct DmaHeapAllocationData { pub len: u64, pub fd: u32, pub fd_flags: u32, pub heap_flags: u64 }

unsafe fn dma_heap_buffer_alloc(heap: *mut DmaHeap, mut len: usize, fd_flags: u32, heap_flags: u64) -> i32 {
    len = page_align(len);
    if len == 0 { return -22; }
    let dmabuf = ((*(*heap).ops).allocate.unwrap())(heap, len, fd_flags, heap_flags);
    let fd = dma_buf_fd(dmabuf, fd_flags);
    if fd < 0 { dma_buf_put(dmabuf); }
    fd
}

unsafe extern "C" fn dma_heap_open(inode: *mut Inode, file: *mut File) -> i32 {
    let heap = xa_load(&mut dma_heap_minors, iminor(inode) as usize);
    if heap.is_null() { return -19; }
    (*file).private_data = heap.cast();
    nonseekable_open(inode, file);
    0
}

unsafe extern "C" fn dma_heap_ioctl(file: *mut File, _ucmd: u32, arg: usize) -> isize {
    let mut stack_kdata = [0i8; 128];
    let kdata = stack_kdata.as_mut_ptr();
    if copy_from_user(kdata, arg as *const core::ffi::c_void, core::mem::size_of::<DmaHeapAllocationData>()) != 0 { return -14; }
    let ret = dma_heap_ioctl_allocate(file, kdata.cast());
    if ret == 0 && copy_to_user(arg as *mut core::ffi::c_void, kdata, core::mem::size_of::<DmaHeapAllocationData>()) != 0 { return -14; }
    ret
}

static DMA_HEAP_FOPS: FileOperations = FileOperations {
    owner: core::ptr::null_mut(),
    open: Some(dma_heap_open),
    unlocked_ioctl: Some(dma_heap_ioctl),
    compat_ioctl: Some(dma_heap_ioctl),
};

unsafe fn dma_heap_ioctl_allocate(file: *mut File, data: *mut DmaHeapAllocationData) -> isize {
    if (*data).fd != 0 || ((*data).fd_flags & !DMA_HEAP_VALID_FD_FLAGS) != 0 || ((*data).heap_flags & !DMA_HEAP_VALID_HEAP_FLAGS) != 0 { return -22; }
    let fd = dma_heap_buffer_alloc((*file).private_data.cast(), (*data).len as usize, (*data).fd_flags, (*data).heap_flags);
    if fd < 0 { return fd as isize; }
    (*data).fd = fd as u32; 0
}

const DMA_HEAP_VALID_FD_FLAGS: u32 = 0;
const DMA_HEAP_VALID_HEAP_FLAGS: u64 = 0;

pub unsafe fn dma_heap_get_drvdata(heap: *mut DmaHeap) -> *mut core::ffi::c_void { (*heap).priv_ }
pub unsafe fn dma_heap_get_name(heap: *mut DmaHeap) -> *const i8 { (*heap).name }

pub unsafe fn dma_heap_add(exp_info: *const DmaHeapExportInfo) -> *mut DmaHeap {
    if (*exp_info).name.is_null() || (*exp_info).ops.is_null() || (*(*exp_info).ops).allocate.is_none() { return core::ptr::null_mut(); }
    let heap = kmalloc(core::mem::size_of::<DmaHeap>(), 0) as *mut DmaHeap;
    if heap.is_null() { return core::ptr::null_mut(); }
    (*heap).name = (*exp_info).name; (*heap).ops = (*exp_info).ops; (*heap).priv_ = (*exp_info).priv_;
    let mut minor = 0; if xa_alloc(&mut dma_heap_minors, &mut minor, heap, XaLimit { min: 0, max: 127 }, 0) < 0 { kfree(heap.cast()); return core::ptr::null_mut(); }
    (*heap).heap_devt = (dma_heap_devt & !0xfff) | minor as u64;
    cdev_init(&mut (*heap).heap_cdev, &DMA_HEAP_FOPS);
    if cdev_add(&mut (*heap).heap_cdev, (*heap).heap_devt, 1) < 0 { kfree(heap.cast()); return core::ptr::null_mut(); }
    let dev = device_create(dma_heap_class, core::ptr::null_mut(), (*heap).heap_devt, core::ptr::null_mut(), (*heap).name);
    if dev.is_null() { return core::ptr::null_mut(); }
    mutex_lock(&mut HEAP_LIST_LOCK); mutex_unlock(&mut HEAP_LIST_LOCK); heap
}

unsafe extern "C" fn dma_heap_devnode(dev: *const Device, _mode: *mut UmodeT) -> *mut i8 { kasprintf(0, b"dma_heap/%s\0".as_ptr() as *const i8, dev_name(dev)) }

pub unsafe fn dma_heap_init() -> i32 {
    let ret = alloc_chrdev_region(&mut dma_heap_devt, 0, NUM_HEAP_MINORS as u32, DEVNAME);
    if ret != 0 { return ret; }
    dma_heap_class = class_create(DEVNAME);
    if dma_heap_class.is_null() { unregister_chrdev_region(dma_heap_devt, NUM_HEAP_MINORS as u32); return -12; }
    (*dma_heap_class).devnode = Some(dma_heap_devnode);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
