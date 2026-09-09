// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * hung_task_tests.c - Sample code for testing hung tasks with mutex,
 * semaphore, etc.
 *
 * Usage: Load this module and read `<debugfs>/hung_task/mutex`,
 *        `<debugfs>/hung_task/semaphore`, `<debugfs>/hung_task/rw_semaphore_read`,
 *        `<debugfs>/hung_task/rw_semaphore_write`, etc., with 2 or more processes.
 *
 * This is for testing kernel hung_task error messages with various locking
 * mechanisms. Note that this may freeze your system or cause a panic. Use only
 * for testing purposes.
 */

use core::ffi::c_void;

const HUNG_TASK_DIR: &[u8] = b"hung_task\0";
const HUNG_TASK_MUTEX_FILE: &[u8] = b"mutex\0";
const HUNG_TASK_SEM_FILE: &[u8] = b"semaphore\0";
const HUNG_TASK_RWSEM_READ_FILE: &[u8] = b"rw_semaphore_read\0";
const HUNG_TASK_RWSEM_WRITE_FILE: &[u8] = b"rw_semaphore_write\0";
const SLEEP_SECOND: u32 = 256;

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct semaphore {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rw_semaphore {
    _private: [u8; 0],
}

type SsizeT = isize;
type SizeT = usize;
type LoffT = i64;
type ReadFn = unsafe extern "C" fn(*mut file, *mut u8, SizeT, *mut LooffT) -> SsizeT;
type LooffT = LooffTPlaceholder;
type LooffTPlaceholder = LooffTValue;
type LooffTValue = i64;

#[repr(C)]
struct file_operations {
    read: Option<ReadFn>,
}

extern "C" {
    static mut dummy_mutex: mutex;
    static mut dummy_sem: semaphore;
    static mut dummy_rwsem: rw_semaphore;

    fn debugfs_create_dir(name: *const u8, parent: *mut dentry) -> *mut dentry;
    fn debugfs_create_file(
        name: *const u8,
        mode: u32,
        parent: *mut dentry,
        data: *mut c_void,
        fops: *const file_operations,
    ) -> *mut dentry;
    fn debugfs_remove_recursive(dentry: *mut dentry);
    fn ptr_err(ptr: *mut dentry) -> i32;
    fn is_err(ptr: *mut dentry) -> bool;
    fn msleep_interruptible(milliseconds: u32);
    fn down(sem: *mut semaphore);
    fn up(sem: *mut semaphore);
    fn down_read(sem: *mut rw_semaphore);
    fn up_read(sem: *mut rw_semaphore);
    fn down_write(sem: *mut rw_semaphore);
    fn up_write(sem: *mut rw_semaphore);
    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);
    fn simple_read_from_buffer(
        user_buf: *mut u8,
        count: SizeT,
        position: *mut LooffT,
        buffer: *const u8,
        buffer_size: SizeT,
    ) -> SsizeT;
}

static DUMMY_STRING: &[u8] = b"This is a dummy string.\0";
static mut HUNG_TASK_DIR: *mut dentry = core::ptr::null_mut();

/* Mutex-based read function */
unsafe extern "C" fn read_dummy_mutex(
    _file: *mut file,
    user_buf: *mut u8,
    count: SizeT,
    ppos: *mut LooffT,
) -> SsizeT {
    /* Check if data is already read */
    if *ppos >= DUMMY_STRING.len() as LooffT { return 0; }

    /* Second task waits on mutex, entering uninterruptible sleep */
    mutex_lock(&raw mut dummy_mutex);

    /* First task sleeps here, interruptible */
    msleep_interruptible(SLEEP_SECOND.wrapping_mul(1000));

    let result = simple_read_from_buffer(user_buf, count, ppos,
                                         DUMMY_STRING.as_ptr(), DUMMY_STRING.len());
    mutex_unlock(&raw mut dummy_mutex);
    result
}

/* Semaphore-based read function */
unsafe extern "C" fn read_dummy_semaphore(
    _file: *mut file, user_buf: *mut u8, count: SizeT, ppos: *mut LooffT,
) -> SsizeT {
    if *ppos >= DUMMY_STRING.len() as LooffT { return 0; }
    down(&raw mut dummy_sem);
    msleep_interruptible(SLEEP_SECOND.wrapping_mul(1000));
    up(&raw mut dummy_sem);
    simple_read_from_buffer(user_buf, count, ppos, DUMMY_STRING.as_ptr(), DUMMY_STRING.len())
}

/* Read-write semaphore read function */
unsafe extern "C" fn read_dummy_rwsem_read(
    _file: *mut file, user_buf: *mut u8, count: SizeT, ppos: *mut LooffT,
) -> SsizeT {
    if *ppos >= DUMMY_STRING.len() as LooffT { return 0; }
    down_read(&raw mut dummy_rwsem);
    msleep_interruptible(SLEEP_SECOND.wrapping_mul(1000));
    up_read(&raw mut dummy_rwsem);
    simple_read_from_buffer(user_buf, count, ppos, DUMMY_STRING.as_ptr(), DUMMY_STRING.len())
}

/* Read-write semaphore write function */
unsafe extern "C" fn read_dummy_rwsem_write(
    _file: *mut file, user_buf: *mut u8, count: SizeT, ppos: *mut LooffT,
) -> SsizeT {
    if *ppos >= DUMMY_STRING.len() as LooffT { return 0; }
    down_write(&raw mut dummy_rwsem);
    msleep_interruptible(SLEEP_SECOND.wrapping_mul(1000));
    up_write(&raw mut dummy_rwsem);
    simple_read_from_buffer(user_buf, count, ppos, DUMMY_STRING.as_ptr(), DUMMY_STRING.len())
}

/* File operations for mutex */
static HUNG_TASK_MUTEX_FOPS: file_operations = file_operations { read: Some(read_dummy_mutex) };
/* File operations for semaphore */
static HUNG_TASK_SEM_FOPS: file_operations = file_operations { read: Some(read_dummy_semaphore) };
/* File operations for rw_semaphore read */
static HUNG_TASK_RWSEM_READ_FOPS: file_operations = file_operations { read: Some(read_dummy_rwsem_read) };
/* File operations for rw_semaphore write */
static HUNG_TASK_RWSEM_WRITE_FOPS: file_operations = file_operations { read: Some(read_dummy_rwsem_write) };

unsafe extern "C" fn hung_task_tests_init() -> i32 {
    HUNG_TASK_DIR = debugfs_create_dir(HUNG_TASK_DIR.as_ptr(), core::ptr::null_mut());
    if is_err(HUNG_TASK_DIR) { return ptr_err(HUNG_TASK_DIR); }

    debugfs_create_file(HUNG_TASK_MUTEX_FILE.as_ptr(), 0o400, HUNG_TASK_DIR,
                        core::ptr::null_mut(), &HUNG_TASK_MUTEX_FOPS);
    debugfs_create_file(HUNG_TASK_SEM_FILE.as_ptr(), 0o400, HUNG_TASK_DIR,
                        core::ptr::null_mut(), &HUNG_TASK_SEM_FOPS);
    debugfs_create_file(HUNG_TASK_RWSEM_READ_FILE.as_ptr(), 0o400, HUNG_TASK_DIR,
                        core::ptr::null_mut(), &HUNG_TASK_RWSEM_READ_FOPS);
    debugfs_create_file(HUNG_TASK_RWSEM_WRITE_FILE.as_ptr(), 0o400, HUNG_TASK_DIR,
                        core::ptr::null_mut(), &HUNG_TASK_RWSEM_WRITE_FOPS);
    0
}

unsafe extern "C" fn hung_task_tests_exit() {
    debugfs_remove_recursive(HUNG_TASK_DIR);
}

// module_init(hung_task_tests_init);
// module_exit(hung_task_tests_exit);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Masami Hiramatsu <mhiramat@kernel.org>");
// MODULE_AUTHOR("Zi Li <amaindex@outlook.com>");
// MODULE_DESCRIPTION("Simple sleep under lock files for testing hung task");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
