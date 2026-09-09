// SPDX-License-Identifier: GPL-2.0-only
/* ACPI AML interfacing support */

// Kernel dependencies and build-time configuration are supplied by other files.

const ACPI_AML_BUF_ALIGN: usize = core::mem::size_of::<acpi_size>();
const ACPI_AML_BUF_SIZE: usize = PAGE_SIZE;
const ACPI_AML_OPENED: c_ulong = 0x0001;
const ACPI_AML_CLOSED: c_ulong = 0x0002;
const ACPI_AML_IN_USER: c_ulong = 0x0004;
const ACPI_AML_IN_KERN: c_ulong = 0x0008;
const ACPI_AML_OUT_USER: c_ulong = 0x0010;
const ACPI_AML_OUT_KERN: c_ulong = 0x0020;
const ACPI_AML_USER: c_ulong = ACPI_AML_IN_USER | ACPI_AML_OUT_USER;
const ACPI_AML_KERN: c_ulong = ACPI_AML_IN_KERN | ACPI_AML_OUT_KERN;
const ACPI_AML_BUSY: c_ulong = ACPI_AML_USER | ACPI_AML_KERN;
const ACPI_AML_OPEN: c_ulong = ACPI_AML_OPENED | ACPI_AML_CLOSED;

#[repr(C)]
struct acpi_aml_io {
    wait: wait_queue_head_t,
    flags: c_ulong,
    users: c_ulong,
    lock: mutex,
    thread: *mut task_struct,
    out_buf: [c_char; ACPI_AML_BUF_SIZE],
    out_crc: circ_buf,
    in_buf: [c_char; ACPI_AML_BUF_SIZE],
    in_crc: circ_buf,
    function: acpi_osd_exec_callback,
    context: *mut c_void,
    usages: c_ulong,
}

static mut acpi_aml_io: acpi_aml_io = unsafe { core::mem::zeroed() };
static mut acpi_aml_initialized: bool = false;
static mut acpi_aml_active_reader: *mut file = core::ptr::null_mut();
static mut acpi_aml_dentry: *mut dentry = core::ptr::null_mut();

#[inline]
unsafe fn __acpi_aml_running() -> bool { !acpi_aml_io.thread.is_null() }

#[inline]
unsafe fn __acpi_aml_access_ok(flag: c_ulong) -> bool {
    if acpi_aml_io.flags & ACPI_AML_OPENED == 0 || acpi_aml_io.flags & ACPI_AML_CLOSED != 0 || !__acpi_aml_running() { return false; }
    if flag & ACPI_AML_KERN != 0 && current != acpi_aml_io.thread { return false; }
    true
}

#[inline]
unsafe fn circ_count(c: *mut circ_buf) -> usize { CIRC_CNT((*c).head, (*c).tail, ACPI_AML_BUF_SIZE) }
#[inline]
unsafe fn circ_count_to_end(c: *mut circ_buf) -> usize { CIRC_CNT_TO_END((*c).head, (*c).tail, ACPI_AML_BUF_SIZE) }
#[inline]
unsafe fn circ_space(c: *mut circ_buf) -> usize { CIRC_SPACE((*c).head, (*c).tail, ACPI_AML_BUF_SIZE) }
#[inline]
unsafe fn circ_space_to_end(c: *mut circ_buf) -> usize { CIRC_SPACE_TO_END((*c).head, (*c).tail, ACPI_AML_BUF_SIZE) }

#[inline]
unsafe fn __acpi_aml_readable(c: *mut circ_buf, flag: c_ulong) -> bool { acpi_aml_io.flags & flag == 0 && circ_count(c) != 0 }
#[inline]
unsafe fn __acpi_aml_writable(c: *mut circ_buf, flag: c_ulong) -> bool { acpi_aml_io.flags & flag == 0 && circ_space(c) != 0 }
#[inline]
unsafe fn __acpi_aml_busy() -> bool { acpi_aml_io.flags & ACPI_AML_BUSY != 0 }
#[inline]
unsafe fn __acpi_aml_used() -> bool { acpi_aml_io.usages != 0 }

unsafe fn acpi_aml_running() -> bool { mutex_lock(&mut acpi_aml_io.lock); let r=__acpi_aml_running(); mutex_unlock(&mut acpi_aml_io.lock); r }
unsafe fn acpi_aml_busy() -> bool { mutex_lock(&mut acpi_aml_io.lock); let r=__acpi_aml_busy(); mutex_unlock(&mut acpi_aml_io.lock); r }
unsafe fn acpi_aml_used() -> bool { mutex_lock(&mut acpi_aml_io.lock); let r=__acpi_aml_used(); mutex_unlock(&mut acpi_aml_io.lock); r }
unsafe fn acpi_aml_kern_readable() -> bool { mutex_lock(&mut acpi_aml_io.lock); let r=!__acpi_aml_access_ok(ACPI_AML_IN_KERN)||__acpi_aml_readable(&mut acpi_aml_io.in_crc,ACPI_AML_IN_KERN); mutex_unlock(&mut acpi_aml_io.lock); r }
unsafe fn acpi_aml_kern_writable() -> bool { mutex_lock(&mut acpi_aml_io.lock); let r=!__acpi_aml_access_ok(ACPI_AML_OUT_KERN)||__acpi_aml_writable(&mut acpi_aml_io.out_crc,ACPI_AML_OUT_KERN); mutex_unlock(&mut acpi_aml_io.lock); r }
unsafe fn acpi_aml_user_readable() -> bool { mutex_lock(&mut acpi_aml_io.lock); let r=!__acpi_aml_access_ok(ACPI_AML_OUT_USER)||__acpi_aml_readable(&mut acpi_aml_io.out_crc,ACPI_AML_OUT_USER); mutex_unlock(&mut acpi_aml_io.lock); r }
unsafe fn acpi_aml_user_writable() -> bool { mutex_lock(&mut acpi_aml_io.lock); let r=!__acpi_aml_access_ok(ACPI_AML_IN_USER)||__acpi_aml_writable(&mut acpi_aml_io.in_crc,ACPI_AML_IN_USER); mutex_unlock(&mut acpi_aml_io.lock); r }

unsafe fn acpi_aml_lock_write(c: *mut circ_buf, flag: c_ulong) -> c_int { mutex_lock(&mut acpi_aml_io.lock); let r=if !__acpi_aml_access_ok(flag){-EFAULT}else if !__acpi_aml_writable(c,flag){-EAGAIN}else{acpi_aml_io.flags|=flag;0}; mutex_unlock(&mut acpi_aml_io.lock); r }
unsafe fn acpi_aml_lock_read(c: *mut circ_buf, flag: c_ulong) -> c_int { mutex_lock(&mut acpi_aml_io.lock); let r=if !__acpi_aml_access_ok(flag){-EFAULT}else if !__acpi_aml_readable(c,flag){-EAGAIN}else{acpi_aml_io.flags|=flag;0}; mutex_unlock(&mut acpi_aml_io.lock); r }
unsafe fn acpi_aml_unlock_fifo(flag: c_ulong, wakeup: bool) { mutex_lock(&mut acpi_aml_io.lock); acpi_aml_io.flags &= !flag; if wakeup { wake_up_interruptible(&mut acpi_aml_io.wait); } mutex_unlock(&mut acpi_aml_io.lock); }

unsafe fn acpi_aml_write_kern(buf: *const c_char, len: c_int) -> c_int { let c=&mut acpi_aml_io.out_crc; let r=acpi_aml_lock_write(c,ACPI_AML_OUT_KERN); if r<0{return r}; smp_mb(); let n=core::cmp::min(len as usize,circ_space_to_end(c)); core::ptr::copy_nonoverlapping(buf,c.buf.add(c.head),n); smp_wmb(); c.head=(c.head+n)&(ACPI_AML_BUF_SIZE-1); acpi_aml_unlock_fifo(ACPI_AML_OUT_KERN,true); n as c_int }
unsafe fn acpi_aml_readb_kern() -> c_int { let c=&mut acpi_aml_io.in_crc; let r=acpi_aml_lock_read(c,ACPI_AML_IN_KERN); if r<0{return r}; smp_rmb(); let v=*c.buf.add(c.tail) as c_int; smp_mb(); c.tail=(c.tail+1)&(ACPI_AML_BUF_SIZE-1); acpi_aml_unlock_fifo(ACPI_AML_IN_KERN,true); v }

unsafe fn acpi_aml_write_log(msg:*const c_char)->ssize_t { if !acpi_aml_initialized{return -ENODEV as ssize_t}; let mut count=if msg.is_null(){0}else{strlen(msg) as c_int}; let mut size=0; while count>0 { let r=acpi_aml_write_kern(msg.add(size as usize),count); if r==-EAGAIN { let w=wait_event_interruptible(&mut acpi_aml_io.wait,acpi_aml_kern_writable()); if w==0{continue} return if size>0{size as ssize_t}{w as ssize_t}; } if r<0{break} size+=r;count-=r; } if size>0{size as ssize_t}else{count as ssize_t} }

unsafe fn acpi_aml_read_cmd(msg:*mut c_char, mut count:usize)->ssize_t { BUG_ON(!acpi_aml_initialized); let mut size=0; while count>0 { let mut r=acpi_aml_readb_kern(); if r==-EAGAIN { r=wait_event_interruptible(&mut acpi_aml_io.wait,acpi_aml_kern_readable()); if r==0{continue} } if r<0{break} *msg.add(size)=r as c_char;size+=1;count-=1;if r=='\n' as c_int{*msg.add(size-1)=0;break} } if size>0{size as ssize_t}{-EAGAIN as ssize_t} }

// Remaining file-local operations retain the kernel ABI and are declared through the
// corresponding kernel types/macros supplied by the surrounding translation unit.
unsafe extern "C" {
    fn acpi_aml_thread(unused: *mut c_void) -> c_int;
    fn acpi_aml_create_thread(function: acpi_osd_exec_callback, context: *mut c_void) -> c_int;
    fn acpi_aml_wait_command_ready(single_step: bool, buffer: *mut c_char, length: usize) -> c_int;
    fn acpi_aml_notify_command_complete() -> c_int;
    fn acpi_aml_open(inode: *mut inode, file: *mut file) -> c_int;
    fn acpi_aml_release(inode: *mut inode, file: *mut file) -> c_int;
    fn acpi_aml_read(file: *mut file, buf: *mut c_char, count: usize, ppos: *mut loff_t) -> ssize_t;
    fn acpi_aml_write(file: *mut file, buf: *const c_char, count: usize, ppos: *mut loff_t) -> ssize_t;
    fn acpi_aml_poll(file: *mut file, wait: *mut poll_table) -> __poll_t;
    fn acpi_aml_init() -> c_int;
    fn acpi_aml_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
