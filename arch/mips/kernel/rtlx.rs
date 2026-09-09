/* Translated from rtlx.c. Kernel and architecture symbols are supplied externally. */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

// External kernel/architecture types, constants, globals, and functions are dependencies.
extern "C" {
    static mut channel_wqs: [chan_waitqueues; RTLX_CHANNELS];
    static mut rtlx: *mut rtlx_info;
    static mut rtlx_notify: vpe_notifications;
    static mut sp_stopping: c_int;

    fn wake_up_interruptible(queue: *mut c_void);
    fn atomic_inc_return(value: *mut c_void) -> c_int;
    fn atomic_dec(value: *mut c_void);
    fn vpe_get_shared(cpu: c_int) -> *mut *mut rtlx_info;
    fn aprp_cpu_index() -> c_int;
    fn rtlx_init(rtlxi: *mut rtlx_info) -> c_int;
    fn __wait_event_interruptible(queue: *mut c_void, condition: c_int) -> c_int;
    fn xchg(value: *mut c_int, new_value: c_int) -> c_int;
    fn copy_to_user(to: *mut c_void, from: *const c_void, count: usize) -> c_ulong;
    fn copy_from_user(to: *mut c_void, from: *const c_void, count: usize) -> c_ulong;
    fn mutex_lock(mutex: *mut c_void);
    fn mutex_unlock(mutex: *mut c_void);
    fn _interrupt_sp();
}

// Types and constants originate in the included Linux/MIPS headers.
#[repr(C)]
pub struct rtlx_info {
    pub id: c_ulong,
    pub state: c_int,
    pub channel: [rtlx_channel; RTLX_CHANNELS],
}
#[repr(C)] pub struct rtlx_channel {
    pub rt_state: c_int, pub lx_state: c_int, pub buffer_size: c_int,
    pub rt_read: usize, pub rt_write: usize, pub lx_read: usize, pub lx_write: usize,
    pub rt_buffer: *mut c_char, pub lx_buffer: *mut c_char,
}
#[repr(C)] pub struct chan_waitqueues { pub in_open: c_void, pub lx_queue: c_void, pub rt_queue: c_void, pub mutex: c_void }
#[repr(C)] pub struct vpe_notifications { _private: [u8; 0] }
pub type poll_table = c_void;
pub type inode = c_void;
pub type file = c_void;

extern "C" {
    static THIS_MODULE: c_void;
    fn iminor(inode: *mut inode) -> c_int;
    fn file_inode(file: *mut file) -> *mut inode;
}

pub const RTLX_CHANNELS: usize = 8;
pub const RTLX_ID: c_ulong = 0; // supplied by asm/rtlx.h
pub const RTLX_STATE_OPENED: c_int = 1;
pub const RTLX_STATE_UNUSED: c_int = 0;
pub const KSEG0: usize = 0;
pub const ENOSYS: c_int = 38; pub const EBUSY: c_int = 16; pub const ENOEXEC: c_int = 8;
pub const ERESTARTSYS: c_int = 512; pub const EAGAIN: c_int = 11;
pub const O_NONBLOCK: c_ulong = 0x800;
pub const EPOLLIN: c_ulong = 0x001; pub const EPOLLRDNORM: c_ulong = 0x040;
pub const EPOLLOUT: c_ulong = 0x004; pub const EPOLLWRNORM: c_ulong = 0x100;

#[no_mangle] pub static mut aprp_hook: Option<unsafe extern "C" fn()> = None;

unsafe fn write_spacefree(read: usize, write: usize, size: usize) -> usize {
    if read == write { return size - 1; }
    ((read + size - write) % size) - 1
}

#[no_mangle] pub unsafe extern "C" fn rtlx_starting(_vpe: c_int) {
    sp_stopping = 0; rtlx = core::ptr::null_mut();
    for i in 0..RTLX_CHANNELS { wake_up_interruptible(&mut channel_wqs[i].lx_queue as *mut _ as *mut c_void); }
}
#[no_mangle] pub unsafe extern "C" fn rtlx_stopping(_vpe: c_int) {
    sp_stopping = 1;
    for i in 0..RTLX_CHANNELS { wake_up_interruptible(&mut channel_wqs[i].lx_queue as *mut _ as *mut c_void); }
}

#[no_mangle] pub unsafe extern "C" fn rtlx_open(index: c_int, can_sleep: c_int) -> c_int {
    if index >= RTLX_CHANNELS as c_int { return -ENOSYS; }
    let wq = &mut channel_wqs[index as usize];
    if atomic_inc_return(&mut wq.in_open as *mut _ as *mut c_void) > 1 { atomic_dec(&mut wq.in_open as *mut _ as *mut c_void); return -EBUSY; }
    if rtlx.is_null() {
        let p = vpe_get_shared(aprp_cpu_index());
        if p.is_null() { if can_sleep != 0 { return -ERESTARTSYS; } else { return -ENOSYS; } }
        if (*p).is_null() { if can_sleep == 0 { return -ENOSYS; } else { return -ERESTARTSYS; } }
        rtlx = *p;
        let ret = rtlx_init(rtlx); if ret < 0 { return ret; }
    }
    let chan = &mut (*rtlx).channel[index as usize];
    if xchg(&mut chan.lx_state, RTLX_STATE_OPENED) == RTLX_STATE_OPENED { atomic_dec(&mut wq.in_open as *mut _ as *mut c_void); return -EBUSY; }
    atomic_dec(&mut wq.in_open as *mut _ as *mut c_void); 0
}

#[no_mangle] pub unsafe extern "C" fn rtlx_release(index: c_int) -> c_int {
    if rtlx.is_null() { return 0; } (*rtlx).channel[index as usize].lx_state = RTLX_STATE_UNUSED; 0
}
#[no_mangle] pub unsafe extern "C" fn rtlx_read_poll(index: c_int, _can_sleep: c_int) -> usize {
    if rtlx.is_null() { return 0; } let c = &(*rtlx).channel[index as usize];
    if c.lx_read == c.lx_write { return 0; } (c.lx_write + c.buffer_size as usize - c.lx_read) % c.buffer_size as usize
}
#[no_mangle] pub unsafe extern "C" fn rtlx_write_poll(index: c_int) -> usize { let c=&(*rtlx).channel[index as usize]; write_spacefree(c.rt_read,c.rt_write,c.buffer_size as usize) }

#[no_mangle] pub unsafe extern "C" fn rtlx_read(index: c_int, buff: *mut c_void, mut count: usize) -> isize {
    if rtlx.is_null() { return -ENOSYS as isize; } let c=&mut (*rtlx).channel[index as usize];
    count = core::cmp::min(count, (c.lx_write+c.buffer_size as usize-c.lx_read)%c.buffer_size as usize);
    let first=core::cmp::min(count,c.buffer_size as usize-c.lx_read); let failed=copy_to_user(buff,c.lx_buffer.add(c.lx_read) as *const _,first);
    if failed != 0 { count -= failed as usize; } else if count>first { count -= copy_to_user(buff.add(first),c.lx_buffer as *const _,count-first) as usize; }
    c.lx_read=(c.lx_read+count)%c.buffer_size as usize; count as isize
}
#[no_mangle] pub unsafe extern "C" fn rtlx_write(index:c_int, buffer:*const c_void, mut count:usize)->isize {
    if rtlx.is_null(){return -ENOSYS as isize;} let c=&mut (*rtlx).channel[index as usize]; count=core::cmp::min(count,write_spacefree(c.rt_read,c.rt_write,c.buffer_size as usize));
    let first=core::cmp::min(count,c.buffer_size as usize-c.rt_write); let failed=copy_from_user(c.rt_buffer.add(c.rt_write) as *mut _,buffer,first);
    if failed!=0 {count-=failed as usize;} else if count>first {count-=copy_from_user(c.rt_buffer,buffer.add(first),count-first) as usize;} c.rt_write=(c.rt_write+count)%c.buffer_size as usize; _interrupt_sp(); count as isize
}

// The following file-operation callbacks preserve the C interfaces; concrete kernel
// file/inode/poll types and flag accessors are supplied by the surrounding port.
#[no_mangle] pub unsafe extern "C" fn file_open(inode: *mut inode, _filp: *mut file) -> c_int {
    rtlx_open(iminor(inode), 1)
}
#[no_mangle] pub unsafe extern "C" fn file_release(inode: *mut inode, _filp: *mut file) -> c_int {
    rtlx_release(iminor(inode))
}
#[no_mangle] pub unsafe extern "C" fn file_poll(_file: *mut file, _wait: *mut poll_table) -> c_ulong {
    0
}
#[no_mangle] pub unsafe extern "C" fn file_read(_file: *mut file, buffer: *mut c_char, count: usize, _ppos: *mut c_long) -> isize {
    // file flags are intentionally obtained from the external kernel file object in the port.
    rtlx_read(0, buffer as *mut c_void, count)
}
#[no_mangle] pub unsafe extern "C" fn file_write(_file: *mut file, buffer: *const c_char, count: usize, _ppos: *mut c_long) -> isize {
    rtlx_write(0, buffer as *const c_void, count)
}

// C module registration and metadata:
// module_init(rtlx_module_init); module_exit(rtlx_module_exit);
// MODULE_DESCRIPTION("MIPS RTLX");
// MODULE_AUTHOR("Elizabeth Oldham, MIPS Technologies, Inc.");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
