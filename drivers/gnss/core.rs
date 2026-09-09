// SPDX-License-Identifier: GPL-2.0
/* GNSS receiver core. Rust translation of core.c. */

// Linux kernel headers provide the types, constants, macros, and functions
// referenced below.

const GNSS_FLAG_HAS_WRITE_RAW: u32 = 1 << 0;
const GNSS_MINORS: u32 = 16;
const GNSS_READ_FIFO_SIZE: usize = 4096;
const GNSS_WRITE_BUF_SIZE: usize = 1024;

static mut gnss_minors: Ida = Ida::new();
static mut gnss_first: DevT = 0;
static mut gnss_class: *mut Class = core::ptr::null_mut();

// These declarations correspond to symbols and types supplied by the kernel
// headers and by the other GNSS sources.
#[repr(C)]
pub struct GnssDevice {
    pub dev: Device,
    pub cdev: Cdev,
    pub rwsem: RwSem,
    pub read_mutex: Mutex,
    pub write_mutex: Mutex,
    pub read_queue: WaitQueue,
    pub read_fifo: Kfifo,
    pub write_buf: *mut u8,
    pub id: i32,
    pub count: u32,
    pub flags: u32,
    pub disconnected: bool,
    pub typ: u32,
    pub ops: *const GnssOperations,
}

#[repr(C)] pub struct Device { pub devt: DevT, pub class: *mut Class, pub parent: *mut Device, pub release: Option<unsafe extern "C" fn(*mut Device)> }
#[repr(C)] pub struct Cdev;
#[repr(C)] pub struct RwSem;
#[repr(C)] pub struct Mutex;
#[repr(C)] pub struct WaitQueue;
#[repr(C)] pub struct Kfifo;
#[repr(C)] pub struct Ida;
#[repr(C)] pub struct Class;
#[repr(C)] pub struct Inode { pub i_cdev: *mut Cdev }
#[repr(C)] pub struct File { pub private_data: *mut core::ffi::c_void, pub f_flags: u32 }
#[repr(C)] pub struct PollTable;
#[repr(C)] pub struct FileOperations;
#[repr(C)] pub struct GnssOperations { pub open: Option<unsafe extern "C" fn(*mut GnssDevice) -> i32>, pub close: Option<unsafe extern "C" fn(*mut GnssDevice)>, pub write_raw: Option<unsafe extern "C" fn(*mut GnssDevice, *mut u8, usize) -> i32> }
pub type DevT = u64;
pub const GNSS_TYPE_COUNT: usize = 4;
pub const GNSS_TYPE_NMEA: usize = 0;
pub const GNSS_TYPE_SIRF: usize = 1;
pub const GNSS_TYPE_UBX: usize = 2;
pub const GNSS_TYPE_MTK: usize = 3;

impl Ida { const fn new() -> Self { Ida } }

unsafe fn gnss_open(inode: *mut Inode, file: *mut File) -> i32 {
    let gdev = ( (*inode).i_cdev as *mut u8).sub(offset_of!(GnssDevice, cdev)) as *mut GnssDevice;
    get_device(&mut (*gdev).dev); stream_open(inode, file); (*file).private_data = gdev as *mut _;
    down_write(&mut (*gdev).rwsem);
    let mut ret = 0;
    if (*gdev).disconnected { ret = -19; }
    else {
        (*gdev).count += 1;
        if (*gdev).count == 1 { ret = ((*(*gdev).ops).open.unwrap())(gdev); if ret != 0 { (*gdev).count -= 1; } }
    }
    up_write(&mut (*gdev).rwsem); if ret != 0 { put_device(&mut (*gdev).dev); } ret
}

unsafe fn gnss_release(_inode: *mut Inode, file: *mut File) -> i32 {
    let gdev = (*file).private_data as *mut GnssDevice; down_write(&mut (*gdev).rwsem);
    if !(*gdev).disconnected { (*gdev).count -= 1; if (*gdev).count == 0 { ((*(*gdev).ops).close.unwrap())(gdev); kfifo_reset(&mut (*gdev).read_fifo); } }
    up_write(&mut (*gdev).rwsem); put_device(&mut (*gdev).dev); 0
}

unsafe fn gnss_read(file: *mut File, buf: *mut u8, count: usize, _pos: *mut i64) -> isize {
    let gdev = (*file).private_data as *mut GnssDevice; mutex_lock(&mut (*gdev).read_mutex);
    while kfifo_is_empty(&(*gdev).read_fifo) { mutex_unlock(&mut (*gdev).read_mutex); if (*gdev).disconnected { return 0; } if (*file).f_flags & 0x800 != 0 { return -11; } let ret = wait_event_interruptible(&mut (*gdev).read_queue); if ret != 0 { return -512; } mutex_lock(&mut (*gdev).read_mutex); }
    let mut copied = 0u32; let ret = kfifo_to_user(&mut (*gdev).read_fifo, buf, count, &mut copied); mutex_unlock(&mut (*gdev).read_mutex); if ret == 0 { copied as isize } else { ret as isize }
}

unsafe fn gnss_write(file: *mut File, buf: *const u8, count: usize, _pos: *mut i64) -> isize {
    let gdev = (*file).private_data as *mut GnssDevice; if (*gdev).disconnected || (*gdev).flags & GNSS_FLAG_HAS_WRITE_RAW == 0 { return -5; } if count == 0 { return 0; }
    if mutex_lock_interruptible(&mut (*gdev).write_mutex) != 0 { return -512; }
    let mut written = 0usize; let mut ret = 0i32;
    while written != count { let n = core::cmp::min(count - written, GNSS_WRITE_BUF_SIZE); core::ptr::copy_nonoverlapping(buf.add(written), (*gdev).write_buf, n); down_read(&mut (*gdev).rwsem); ret = if !(*gdev).disconnected { ((*(*gdev).ops).write_raw.unwrap())(gdev, (*gdev).write_buf, n) } else { -5 }; up_read(&mut (*gdev).rwsem); if ret < 0 { break; } written += ret as usize; }
    mutex_unlock(&mut (*gdev).write_mutex); if written != 0 { written as isize } else { ret as isize }
}

unsafe fn gnss_insert_raw(gdev: *mut GnssDevice, buf: *const u8, count: usize) -> i32 { let ret = kfifo_in(&mut (*gdev).read_fifo, buf, count); wake_up_interruptible(&mut (*gdev).read_queue); ret }

extern "C" {
    fn get_device(*mut Device); fn put_device(*mut Device); fn stream_open(*mut Inode,*mut File); fn down_write(*mut RwSem); fn up_write(*mut RwSem); fn down_read(*mut RwSem); fn up_read(*mut RwSem); fn mutex_lock(*mut Mutex); fn mutex_unlock(*mut Mutex); fn mutex_lock_interruptible(*mut Mutex)->i32; fn wait_event_interruptible(*mut WaitQueue)->i32; fn kfifo_is_empty(*const Kfifo)->bool; fn kfifo_reset(*mut Kfifo); fn kfifo_to_user(*mut Kfifo,*mut u8,usize,*mut u32)->i32; fn kfifo_in(*mut Kfifo,*const u8,usize)->i32; fn wake_up_interruptible(*mut WaitQueue);
}

#[no_mangle]
pub unsafe extern "C" fn gnss_allocate_device(parent: *mut Device) -> *mut GnssDevice {
    let gdev = kzalloc_gnss(); if gdev.is_null() { return core::ptr::null_mut(); }
    let id = ida_alloc_max(&mut gnss_minors, GNSS_MINORS as i32 - 1); if id < 0 { kfree(gdev as *mut _); return core::ptr::null_mut(); }
    (*gdev).id = id; (*gdev).dev.devt = gnss_first + id as u64; (*gdev).dev.parent = parent; (*gdev).dev.release = Some(gnss_device_release);
    init_rwsem(&mut (*gdev).rwsem); mutex_init(&mut (*gdev).read_mutex); mutex_init(&mut (*gdev).write_mutex); init_waitqueue_head(&mut (*gdev).read_queue);
    if kfifo_alloc(&mut (*gdev).read_fifo, GNSS_READ_FIFO_SIZE) != 0 { put_device(&mut (*gdev).dev); return core::ptr::null_mut(); }
    (*gdev).write_buf = kzalloc(GNSS_WRITE_BUF_SIZE); if (*gdev).write_buf.is_null() { put_device(&mut (*gdev).dev); return core::ptr::null_mut(); } gdev
}

pub unsafe extern "C" fn gnss_put_device(gdev: *mut GnssDevice) { put_device(&mut (*gdev).dev); }

pub unsafe extern "C" fn gnss_register_device(gdev: *mut GnssDevice) -> i32 {
    if !(*(*gdev).ops).write_raw.is_none() { (*gdev).flags |= GNSS_FLAG_HAS_WRITE_RAW; }
    cdev_device_add(&mut (*gdev).cdev, &mut (*gdev).dev)
}

pub unsafe extern "C" fn gnss_deregister_device(gdev: *mut GnssDevice) {
    down_write(&mut (*gdev).rwsem); (*gdev).disconnected = true; if (*gdev).count != 0 { wake_up_interruptible(&mut (*gdev).read_queue); ((*(*gdev).ops).close.unwrap())(gdev); } up_write(&mut (*gdev).rwsem); cdev_device_del(&mut (*gdev).cdev, &mut (*gdev).dev);
}

unsafe extern "C" fn gnss_device_release(dev: *mut Device) { let gdev = dev as *mut GnssDevice; kfree((*gdev).write_buf as *mut _); kfifo_free(&mut (*gdev).read_fifo); ida_free(&mut gnss_minors, (*gdev).id); kfree(gdev as *mut _); }

static gnss_type_names: [Option<&'static [u8]>; GNSS_TYPE_COUNT] = [Some(b"NMEA\0"), Some(b"SiRF\0"), Some(b"UBX\0"), Some(b"MTK\0")];

unsafe fn gnss_type_name(gdev: *const GnssDevice) -> *const u8 { if ((*gdev).typ as usize) < GNSS_TYPE_COUNT { if let Some(s) = gnss_type_names[(*gdev).typ as usize] { return s.as_ptr(); } } core::ptr::null() }

unsafe extern "C" fn type_show(dev: *mut Device, _attr: *mut core::ffi::c_void, buf: *mut u8) -> isize { let n = gnss_type_name(dev as *const _); sprintf(buf, b"%s\n\0".as_ptr(), n) }

unsafe extern "C" fn gnss_poll(file: *mut File, _wait: *mut PollTable) -> u32 { let gdev = (*file).private_data as *mut GnssDevice; let mut mask = 0; if !kfifo_is_empty(&(*gdev).read_fifo) { mask |= 0x001 | 0x040; } if (*gdev).disconnected { mask |= 0x010; } mask }

#[repr(C)]
static mut gnss_fops: FileOperations = FileOperations;

#[no_mangle]
pub unsafe extern "C" fn gnss_module_init() -> i32 { alloc_chrdev_region(&mut gnss_first, GNSS_MINORS, b"gnss\0".as_ptr()); 0 }

pub unsafe extern "C" fn gnss_module_exit() { class_destroy(gnss_class); unregister_chrdev_region(gnss_first, GNSS_MINORS); ida_destroy(&mut gnss_minors); }

extern "C" {
    fn kzalloc_gnss() -> *mut GnssDevice; fn kfree(*mut core::ffi::c_void); fn ida_alloc_max(*mut Ida,i32)->i32; fn ida_free(*mut Ida,i32); fn ida_destroy(*mut Ida); fn init_rwsem(*mut RwSem); fn mutex_init(*mut Mutex); fn init_waitqueue_head(*mut WaitQueue); fn kfifo_alloc(*mut Kfifo,usize)->i32; fn kfifo_free(*mut Kfifo); fn kzalloc(usize)->*mut u8; fn cdev_device_add(*mut Cdev,*mut Device)->i32; fn cdev_device_del(*mut Cdev,*mut Device); fn sprintf(*mut u8,*const u8,...)->isize; fn alloc_chrdev_region(*mut DevT,u32,*const u8)->i32; fn unregister_chrdev_region(DevT,u32); fn class_destroy(*mut Class);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
