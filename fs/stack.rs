// SPDX-License-Identifier: GPL-2.0-only

// Declarations supplied by the Linux filesystem headers.
pub type loff_t = i64;
pub type blkcnt_t = i64;

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timespec64 {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

#[repr(C)]
pub struct inode {
    pub i_mode: u16,
    pub i_uid: u32,
    pub i_gid: u32,
    pub i_rdev: u64,
    pub i_atime: timespec64,
    pub i_mtime: timespec64,
    pub i_ctime: timespec64,
    pub i_blkbits: u8,
    pub i_flags: u32,
    pub i_nlink: u32,
    pub i_lock: spinlock_t,
    pub i_blocks: blkcnt_t,
}

extern "C" {
    fn i_size_read(inode: *const inode) -> loff_t;
    fn i_size_write(inode: *mut inode, size: loff_t);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn inode_set_atime_to_ts(inode: *mut inode, ts: timespec64);
    fn inode_get_atime(inode: *const inode) -> timespec64;
    fn inode_set_mtime_to_ts(inode: *mut inode, ts: timespec64);
    fn inode_get_mtime(inode: *const inode) -> timespec64;
    fn inode_set_ctime_to_ts(inode: *mut inode, ts: timespec64);
    fn inode_get_ctime(inode: *const inode) -> timespec64;
    fn set_nlink(inode: *mut inode, nlink: u32);
}

/* does _NOT_ require i_rwsem to be held.
 *
 * This function cannot be inlined since i_size_{read,write} is rather
 * heavy-weight on 32-bit systems
 */
#[no_mangle]
pub unsafe extern "C" fn fsstack_copy_inode_size(dst: *mut inode, src: *mut inode) {
    let i_size: loff_t;
    let i_blocks: blkcnt_t;

    /*
     * i_size_read() includes its own seqlocking and protection from
     * preemption (see include/linux/fs.h): we need nothing extra for
     * that here, and prefer to avoid nesting locks than attempt to keep
     * i_size and i_blocks in sync together.
     */
    i_size = i_size_read(src);

    /* See the corresponding locking rationale in the C implementation. */
    if core::mem::size_of::<blkcnt_t>() > core::mem::size_of::<libc_long>() {
        spin_lock(&mut (*src).i_lock);
    }
    i_blocks = (*src).i_blocks;
    if core::mem::size_of::<blkcnt_t>() > core::mem::size_of::<libc_long>() {
        spin_unlock(&mut (*src).i_lock);
    }

    if core::mem::size_of::<loff_t>() > core::mem::size_of::<libc_long>()
        || core::mem::size_of::<blkcnt_t>() > core::mem::size_of::<libc_long>()
    {
        spin_lock(&mut (*dst).i_lock);
    }
    i_size_write(dst, i_size);
    (*dst).i_blocks = i_blocks;
    if core::mem::size_of::<loff_t>() > core::mem::size_of::<libc_long>()
        || core::mem::size_of::<blkcnt_t>() > core::mem::size_of::<libc_long>()
    {
        spin_unlock(&mut (*dst).i_lock);
    }
}

pub type libc_long = isize;

/* copy all attributes */
#[no_mangle]
pub unsafe extern "C" fn fsstack_copy_attr_all(dest: *mut inode, src: *const inode) {
    (*dest).i_mode = (*src).i_mode;
    (*dest).i_uid = (*src).i_uid;
    (*dest).i_gid = (*src).i_gid;
    (*dest).i_rdev = (*src).i_rdev;
    inode_set_atime_to_ts(dest, inode_get_atime(src));
    inode_set_mtime_to_ts(dest, inode_get_mtime(src));
    inode_set_ctime_to_ts(dest, inode_get_ctime(src));
    (*dest).i_blkbits = (*src).i_blkbits;
    (*dest).i_flags = (*src).i_flags;
    set_nlink(dest, (*src).i_nlink);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
