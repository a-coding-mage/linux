/*
 * arch/xtensa/platforms/iss/simdisk.c
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001-2013 Tensilica Inc.
 *   Authors\tVictor Prupis
 */

// Kernel and platform dependencies supplied by the surrounding translation.

const SIMDISK_MAJOR: i32 = 240;
const SIMDISK_MINORS: i32 = 1;
const MAX_SIMDISK_COUNT: usize = 10;

#[repr(C)]
struct Simdisk {
    filename: *const i8,
    lock: Spinlock,
    gd: *mut Gendisk,
    procfile: *mut ProcDirEntry,
    users: i32,
    size: usize,
    fd: i32,
}

#[repr(C)] struct Spinlock { _opaque: [u8; 0] }
#[repr(C)] struct Gendisk { major: i32, first_minor: i32, minors: i32, fops: *const BlockDeviceOperations, private_data: *mut core::ffi::c_void, disk_name: [i8; 32] }
#[repr(C)] struct ProcDirEntry { _opaque: [u8; 0] }
#[repr(C)] struct Bio { _opaque: [u8; 0] }
#[repr(C)] struct File { _opaque: [u8; 0] }
#[repr(C)] struct BlockDeviceOperations { owner: *const core::ffi::c_void, submit_bio: Option<unsafe extern "C" fn(*mut Bio)>, open: Option<unsafe extern "C" fn(*mut Gendisk, u32) -> i32>, release: Option<unsafe extern "C" fn(*mut Gendisk)> }
#[repr(C)] struct ProcOps { proc_read: Option<unsafe extern "C" fn(*mut File, *mut i8, usize, *mut i64) -> isize>, proc_write: Option<unsafe extern "C" fn(*mut File, *const i8, usize, *mut i64) -> isize>, proc_lseek: Option<unsafe extern "C" fn(*mut File, i64, i32) -> i64> }
#[repr(C)] struct QueueLimits { features: u64 }

static mut SIMDISK_COUNT: i32 = 0; // CONFIG_BLK_DEV_SIMDISK_COUNT
static mut N_FILES: i32 = 0;
static mut FILENAME: [*mut i8; MAX_SIMDISK_COUNT] = [core::ptr::null_mut(); MAX_SIMDISK_COUNT];
static mut SIMDISK_MAJOR: i32 = SIMDISK_MAJOR;
static mut SDDEV: *mut Simdisk = core::ptr::null_mut();
static mut SIMDISK_PROCDIR: *mut ProcDirEntry = core::ptr::null_mut();

extern "C" {
    fn kstrdup(s: *const i8, flags: u32) -> *mut i8;
    fn kfree(p: *mut core::ffi::c_void);
    fn kmalloc(size: usize, flags: u32) -> *mut i8;
    fn memdup_user_nul(p: *const i8, size: usize) -> *mut i8;
    fn ptr_err(p: *const core::ffi::c_void) -> isize;
    fn simc_open(path: *const i8, flags: i32, mode: i32) -> i32;
    fn simc_close(fd: i32) -> i32;
    fn simc_lseek(fd: i32, offset: usize, whence: i32) -> usize;
    fn simc_read(fd: i32, buf: *mut i8, count: usize) -> usize;
    fn simc_write(fd: i32, buf: *const i8, count: usize) -> usize;
    fn spin_lock(lock: *mut Spinlock);
    fn spin_unlock(lock: *mut Spinlock);
    fn spin_lock_init(lock: *mut Spinlock);
    fn blk_alloc_disk(lim: *const QueueLimits, node: i32) -> *mut Gendisk;
    fn put_disk(disk: *mut Gendisk);
    fn add_disk(disk: *mut Gendisk) -> i32;
    fn del_gendisk(disk: *mut Gendisk);
    fn set_capacity(disk: *mut Gendisk, sectors: usize);
    fn register_blkdev(major: i32, name: *const i8) -> i32;
    fn unregister_blkdev(major: i32, name: *const i8);
    fn proc_mkdir(name: *const i8, parent: *mut ProcDirEntry) -> *mut ProcDirEntry;
    fn proc_create_data(name: *const i8, mode: u32, parent: *mut ProcDirEntry, ops: *const ProcOps, data: *mut core::ffi::c_void) -> *mut ProcDirEntry;
    fn remove_proc_entry(name: *const i8, parent: *mut ProcDirEntry);
    fn pde_data(file: *mut File) -> *mut core::ffi::c_void;
    fn file_inode(file: *mut File) -> *mut core::ffi::c_void;
    fn simple_read_from_buffer(buf: *mut i8, size: usize, pos: *mut i64, src: *const i8, len: usize) -> isize;
}

unsafe fn simdisk_param_set_filename(val: *const i8) -> i32 {
    if N_FILES as usize >= FILENAME.len() { return -22; }
    FILENAME[N_FILES as usize] = val as *mut i8;
    N_FILES += 1;
    0
}

unsafe fn simdisk_param_free_filename() {
    for i in 0..N_FILES { kfree(FILENAME[i as usize] as *mut _); }
}

unsafe fn simdisk_transfer(dev: *mut Simdisk, mut sector: usize, mut nsect: usize, mut buffer: *mut i8, write: bool) {
    let mut offset = sector.wrapping_shl(9);
    let mut nbytes = nsect.wrapping_shl(9);
    if offset > (*dev).size || (*dev).size - offset < nbytes { return; }
    spin_lock(&mut (*dev).lock);
    while nbytes > 0 {
        simc_lseek((*dev).fd, offset, 0);
        let io = if write { simc_write((*dev).fd, buffer, nbytes) } else { simc_read((*dev).fd, buffer, nbytes) };
        if io == usize::MAX { break; }
        buffer = buffer.add(io); offset = offset.wrapping_add(io); nbytes -= io;
    }
    spin_unlock(&mut (*dev).lock);
}

unsafe fn simdisk_open(disk: *mut Gendisk, _mode: u32) -> i32 { let dev = (*disk).private_data as *mut Simdisk; spin_lock(&mut (*dev).lock); (*dev).users += 1; spin_unlock(&mut (*dev).lock); 0 }
unsafe fn simdisk_release(disk: *mut Gendisk) { let dev = (*disk).private_data as *mut Simdisk; spin_lock(&mut (*dev).lock); (*dev).users -= 1; spin_unlock(&mut (*dev).lock); }

unsafe fn simdisk_attach(dev: *mut Simdisk, name: *const i8) -> i32 {
    let filename = kstrdup(name, 0); if filename.is_null() { return -12; }
    spin_lock(&mut (*dev).lock);
    if (*dev).fd != -1 { kfree(filename as *mut _); spin_unlock(&mut (*dev).lock); return -16; }
    (*dev).fd = simc_open(filename, 2, 0);
    if (*dev).fd == -1 { kfree(filename as *mut _); spin_unlock(&mut (*dev).lock); return -19; }
    (*dev).size = simc_lseek((*dev).fd, 0, 2); set_capacity((*dev).gd, (*dev).size >> 9); (*dev).filename = filename;
    spin_unlock(&mut (*dev).lock); 0
}

unsafe fn simdisk_detach(dev: *mut Simdisk) -> i32 {
    spin_lock(&mut (*dev).lock);
    if (*dev).users != 0 { spin_unlock(&mut (*dev).lock); return -16; }
    if (*dev).fd != -1 { if simc_close((*dev).fd) != 0 { spin_unlock(&mut (*dev).lock); return -5; } (*dev).fd = -1; kfree((*dev).filename as *mut _); (*dev).filename = core::ptr::null(); }
    spin_unlock(&mut (*dev).lock); 0
}

unsafe fn proc_read_simdisk(_file: *mut File, buf: *mut i8, size: usize, pos: *mut i64) -> isize { let dev = pde_data(_file) as *mut Simdisk; if !(*dev).filename.is_null() { let mut len = 0usize; while *(*dev).filename.add(len) != 0 { len += 1; } let temp = kmalloc(len + 2, 0); if temp.is_null() { return -12; } core::ptr::copy_nonoverlapping((*dev).filename, temp, len); *temp.add(len) = b'\n' as i8; *temp.add(len + 1) = 0; let result = simple_read_from_buffer(buf, size, pos, temp, len + 1); kfree(temp as *mut _); result } else { simple_read_from_buffer(buf, size, pos, b"\n\0".as_ptr() as *const i8, 1) } }

unsafe fn proc_write_simdisk(file: *mut File, buf: *const i8, count: usize, _ppos: *mut i64) -> isize { if count == 0 || count > 4096 { return -22; } let tmp = memdup_user_nul(buf, count); if tmp.is_null() { return -12; } let dev = pde_data(file) as *mut Simdisk; let mut err = simdisk_detach(dev); if err == 0 { if *tmp.add(count - 1) == b'\n' as i8 { *tmp.add(count - 1) = 0; } if *tmp != 0 { err = simdisk_attach(dev, tmp); } if err == 0 { err = count as i32; } } kfree(tmp as *mut _); err as isize }

unsafe fn simdisk_setup(dev: *mut Simdisk, which: i32, procdir: *mut ProcDirEntry) -> i32 { (*dev).fd = -1; (*dev).filename = core::ptr::null(); spin_lock_init(&mut (*dev).lock); (*dev).users = 0; let lim = QueueLimits { features: 1 }; (*dev).gd = blk_alloc_disk(&lim, -1); if (*dev).gd.is_null() { return -12; } (*dev).gd.as_mut().unwrap().major = SIMDISK_MAJOR; (*dev).gd.as_mut().unwrap().first_minor = which; (*dev).gd.as_mut().unwrap().minors = SIMDISK_MINORS; (*dev).gd.as_mut().unwrap().private_data = dev as *mut _; set_capacity((*dev).gd, 0); if add_disk((*dev).gd) != 0 { put_disk((*dev).gd); return -12; } let name = [b'0' as i8 + which as i8, 0]; proc_create_data(name.as_ptr(), 0o644, procdir, core::ptr::null(), dev as *mut _); 0 }

unsafe fn simdisk_init() -> i32 { let name = b"simdisk\0"; if register_blkdev(SIMDISK_MAJOR, name.as_ptr() as *const i8) < 0 { return -5; } if SIMDISK_COUNT > MAX_SIMDISK_COUNT as i32 { SIMDISK_COUNT = MAX_SIMDISK_COUNT as i32; } let count = SIMDISK_COUNT as usize; SDDEV = kmalloc(core::mem::size_of::<Simdisk>() * count, 0) as *mut Simdisk; if SDDEV.is_null() { unregister_blkdev(SIMDISK_MAJOR, name.as_ptr() as *const i8); return -12; } SIMDISK_PROCDIR = proc_mkdir(b"simdisk\0".as_ptr() as *const i8, core::ptr::null_mut()); if SIMDISK_PROCDIR.is_null() { kfree(SDDEV as *mut _); unregister_blkdev(SIMDISK_MAJOR, name.as_ptr() as *const i8); return -12; } for i in 0..count { let dev = SDDEV.add(i); if simdisk_setup(dev, i as i32, SIMDISK_PROCDIR) == 0 && i < N_FILES as usize && !FILENAME[i].is_null() { simdisk_attach(dev, FILENAME[i]); } } 0 }

unsafe fn simdisk_teardown(dev: *mut Simdisk, which: i32, procdir: *mut ProcDirEntry) { simdisk_detach(dev); if !(*dev).gd.is_null() { del_gendisk((*dev).gd); put_disk((*dev).gd); } let name = [b'0' as i8 + which as i8, 0]; remove_proc_entry(name.as_ptr(), procdir); }

unsafe fn simdisk_exit() { for i in 0..SIMDISK_COUNT as usize { simdisk_teardown(SDDEV.add(i), i as i32, SIMDISK_PROCDIR); } remove_proc_entry(b"simdisk\0".as_ptr() as *const i8, core::ptr::null_mut()); kfree(SDDEV as *mut _); unregister_blkdev(SIMDISK_MAJOR, b"simdisk\0".as_ptr() as *const i8); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
