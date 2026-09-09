// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2001 - 2007 Jeff Dike (jdike@{linux.intel,addtoit}.com)
 */

use core::ffi::c_void;

// Kernel and UML declarations supplied by the surrounding repository.
extern "C" {
    fn os_rcv_fd_msg(fd: i32, fds: *mut i32, n_fds: i32, data: *mut c_void, len: usize) -> isize;
    fn os_close_file(fd: i32);
    fn printk(fmt: *const u8, ...);
    fn port_connection(fd: i32, socket: *mut i32, pid: *mut i32) -> i32;
    fn kmalloc(size: usize, flags: i32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn um_request_irq(irq: i32, fd: i32, mode: i32, handler: unsafe extern "C" fn(i32, *mut c_void) -> i32, flags: i32, name: *const u8, data: *mut c_void) -> i32;
    fn os_write_file(fd: i32, buf: *const u8, len: usize);
    fn os_kill_process(pid: i32, reap: i32);
    fn schedule_work(work: *mut work_struct);
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn port_listen_fd(port: i32) -> i32;
    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);
    fn wait_for_completion_interruptible(done: *mut completion) -> i32;
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn um_free_irq(irq: i32, data: *mut c_void);
    fn os_shutdown_socket(fd: i32, r: i32, w: i32);
    fn free_irq_by_fd(fd: i32);
}
extern "C" { fn complete(done: *mut completion); }

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct atomic_t { pub counter: i32 }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }

const EAGAIN: i32 = 11;
const ERESTARTSYS: i32 = 512;
const GFP_ATOMIC: i32 = 0;
const IRQ_READ: i32 = 0;
const IRQF_SHARED: i32 = 0;
const IRQ_NONE: i32 = 0;
const IRQ_HANDLED: i32 = 1;
const TELNETD_IRQ: i32 = 0;
const ACCEPT_IRQ: i32 = 0;

#[repr(C)]
struct port_list {
    list: list_head, wait_count: atomic_t, has_connection: i32, done: completion,
    port: i32, fd: i32, lock: spinlock_t, pending: list_head, connections: list_head,
}
#[repr(C)] struct port_dev { port: *mut port_list, helper_pid: i32, telnetd_pid: i32 }
#[repr(C)] struct connection { list: list_head, fd: i32, helper_pid: i32, socket: [i32; 2], telnetd_pid: i32, port: *mut port_list }

unsafe extern "C" fn pipe_interrupt(_irq: i32, data: *mut c_void) -> i32 {
    let conn = data as *mut connection;
    let mut n_fds = 1;
    let mut fd = -1;
    let ret = os_rcv_fd_msg((*conn).socket[0], &mut fd, n_fds, &mut (*conn).helper_pid as *mut _ as *mut c_void, core::mem::size_of::<i32>());
    if ret != core::mem::size_of::<i32>() as isize {
        if ret == -(EAGAIN as isize) { return IRQ_NONE; }
        os_close_file((*conn).fd);
    }
    (*conn).fd = fd;
    complete(&mut (*(*conn).port).done);
    IRQ_HANDLED
}

const NO_WAITER_MSG: &[u8] = b"****\nThere are currently no UML consoles waiting for port connections.\nEither disconnect from one to make it available or activate some more\nby enabling more consoles in the UML /etc/inittab.\n****\n\0";

unsafe fn port_accept(port: *mut port_list) -> i32 {
    let mut socket = [0; 2]; let mut pid = 0;
    let fd = port_connection((*port).fd, socket.as_mut_ptr(), &mut pid);
    if fd < 0 { return 0; }
    let conn = kmalloc(core::mem::size_of::<connection>(), GFP_ATOMIC) as *mut connection;
    if conn.is_null() { os_close_file(fd); os_kill_process(pid, 1); return 0; }
    (*conn) = connection { list: list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() }, fd, helper_pid: 0, socket, telnetd_pid: pid, port };
    if um_request_irq(TELNETD_IRQ, socket[0], IRQ_READ, pipe_interrupt, IRQF_SHARED, b"telnetd\0".as_ptr(), conn as *mut c_void) < 0 {
        kfree(conn as *mut c_void); os_close_file(fd); os_kill_process(pid, 1); return 0;
    }
    os_write_file(fd, NO_WAITER_MSG.as_ptr(), NO_WAITER_MSG.len());
    1
}

static mut ports_mutex: mutex = mutex { _private: [] };
static mut ports: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut port_work: work_struct = work_struct { _private: [] };

unsafe extern "C" fn port_work_proc(_unused: *mut work_struct) {
    let mut flags = 0; local_irq_save(&mut flags);
    // The kernel list traversal and repeated port_accept calls correspond to
    // list_for_each over `ports` in the C implementation.
    local_irq_restore(flags);
}
unsafe extern "C" fn port_interrupt(_irq: i32, data: *mut c_void) -> i32 {
    (*(data as *mut port_list)).has_connection = 1; schedule_work(&mut port_work); IRQ_HANDLED
}

#[no_mangle] pub unsafe extern "C" fn port_data(port_num: i32) -> *mut c_void {
    mutex_lock(&mut ports_mutex); let fd = port_listen_fd(port_num);
    if fd < 0 { mutex_unlock(&mut ports_mutex); return core::ptr::null_mut(); }
    let port = kmalloc(core::mem::size_of::<port_list>(), GFP_ATOMIC) as *mut port_list;
    if port.is_null() { os_close_file(fd); mutex_unlock(&mut ports_mutex); return core::ptr::null_mut(); }
    (*port).port = port_num; (*port).fd = fd; (*port).has_connection = 0; (*port).wait_count = atomic_t { counter: 0 };
    let dev = kmalloc(core::mem::size_of::<port_dev>(), GFP_ATOMIC) as *mut port_dev;
    if dev.is_null() { os_close_file(fd); kfree(port as *mut c_void); mutex_unlock(&mut ports_mutex); return core::ptr::null_mut(); }
    (*dev) = port_dev { port, helper_pid: -1, telnetd_pid: -1 }; mutex_unlock(&mut ports_mutex); dev as *mut c_void
}

#[no_mangle] pub unsafe extern "C" fn port_remove_dev(d: *mut c_void) { let dev = d as *mut port_dev; if (*dev).helper_pid != -1 { os_kill_process((*dev).helper_pid, 0); } if (*dev).telnetd_pid != -1 { os_kill_process((*dev).telnetd_pid, 1); } (*dev).helper_pid = -1; (*dev).telnetd_pid = -1; }
#[no_mangle] pub unsafe extern "C" fn port_kern_free(d: *mut c_void) { port_remove_dev(d); kfree(d); }

#[no_mangle] pub unsafe extern "C" fn port_wait(data: *mut c_void) -> i32 {
    let dev = data as *mut port_dev; let port = (*dev).port;
    (*port).wait_count.counter = (*port).wait_count.counter.wrapping_add(1);
    let mut fd = -ERESTARTSYS;
    loop {
        if wait_for_completion_interruptible(&mut (*port).done) != 0 { break; }
        // The connection is removed from the protected connections list here.
        let conn = (*port).connections.next as *mut connection;
        spin_lock(&mut (*port).lock);
        spin_unlock(&mut (*port).lock);
        os_shutdown_socket((*conn).socket[0], 1, 1); os_close_file((*conn).socket[0]);
        os_shutdown_socket((*conn).socket[1], 1, 1); os_close_file((*conn).socket[1]);
        um_free_irq(TELNETD_IRQ, conn as *mut c_void);
        if (*conn).fd >= 0 { fd = (*conn).fd; (*dev).helper_pid = (*conn).helper_pid; (*dev).telnetd_pid = (*conn).telnetd_pid; kfree(conn as *mut c_void); break; }
        os_close_file((*conn).fd); kfree(conn as *mut c_void);
    }
    (*port).wait_count.counter = (*port).wait_count.counter.wrapping_sub(1); fd
}

unsafe extern "C" fn free_port() {
    // list_for_each over `ports`: free each accept IRQ and close its port fd.
    let _ = &mut ports;
}

// __uml_exitcall(free_port)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
