// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2001 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

// Dependencies supplied by the surrounding kernel/UML sources are intentionally
// left as external Rust items.

use core::ffi::c_void;

#[repr(C)]
pub struct completion {
    _private: [u8; 0],
}

type ssize_t = isize;

extern "C" {
    fn os_rcv_fd_msg(
        fd: i32,
        fds: *mut i32,
        n_fds: i32,
        data: *mut c_void,
        data_len: usize,
    ) -> ssize_t;
    fn complete(x: *mut completion);
    fn init_completion(x: *mut completion);
    fn wait_for_completion(x: *mut completion);
    fn um_request_irq(
        irq: i32,
        fd: i32,
        irq_type: i32,
        handler: unsafe extern "C" fn(i32, *mut c_void) -> irqreturn_t,
        flags: i32,
        name: *const u8,
        dev: *mut c_void,
    ) -> i32;
    fn um_free_irq(irq: i32, dev: *mut c_void);
    fn printk(fmt: *const u8, ...);
    fn kmalloc(size: usize, flags: usize) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
}

type irqreturn_t = i32;

extern "C" {
    static XTERM_IRQ: i32;
    static IRQ_READ: i32;
    static IRQF_SHARED: i32;
    static IRQ_NONE: irqreturn_t;
    static IRQ_HANDLED: irqreturn_t;
    static EAGAIN: i32;
    static EMSGSIZE: i32;
    static ENOMEM: i32;
    static KERN_ERR: *const u8;
}

#[repr(C)]
struct xterm_wait {
    ready: completion,
    fd: i32,
    pid: i32,
    new_fd: i32,
}

unsafe extern "C" fn xterm_interrupt(_irq: i32, data: *mut c_void) -> irqreturn_t {
    let xterm = data as *mut xterm_wait;
    let mut fd: i32 = -1;
    let n_fds: i32 = 1;
    let ret: ssize_t;

    ret = os_rcv_fd_msg(
        (*xterm).fd,
        &mut fd,
        n_fds,
        &mut (*xterm).pid as *mut i32 as *mut c_void,
        core::mem::size_of::<i32>(),
    );
    if ret == -(EAGAIN as ssize_t) {
        return IRQ_NONE;
    }

    if ret < 0 {
        fd = ret as i32;
    } else if ret != core::mem::size_of::<i32>() as ssize_t {
        fd = -EMSGSIZE;
    }

    (*xterm).new_fd = fd;
    complete(&mut (*xterm).ready);

    IRQ_HANDLED
}

#[no_mangle]
pub unsafe extern "C" fn xterm_fd(socket: i32, pid_out: *mut i32) -> i32 {
    let data = kmalloc(core::mem::size_of::<xterm_wait>(), 0) as *mut xterm_wait;
    let err: i32;
    let ret: i32;

    if data.is_null() {
        printk(b"%s xterm_fd : failed to allocate xterm_wait\n\0".as_ptr(), KERN_ERR);
        return -ENOMEM;
    }

    // This is a locked semaphore...
    core::ptr::write(
        data,
        xterm_wait {
            ready: core::mem::zeroed(),
            fd: socket,
            pid: -1,
            new_fd: -1,
        },
    );
    init_completion(&mut (*data).ready);

    err = um_request_irq(
        XTERM_IRQ,
        socket,
        IRQ_READ,
        xterm_interrupt,
        IRQF_SHARED,
        b"xterm\0".as_ptr(),
        data as *mut c_void,
    );
    if err < 0 {
        printk(
            b"%s xterm_fd : failed to get IRQ for xterm, err = %d\n\0".as_ptr(),
            KERN_ERR,
            err,
        );
        ret = err;
        kfree(data as *mut c_void);
        return ret;
    }

    // ... so here we wait for an xterm interrupt.
    //
    // XXX Note, if the xterm doesn't work for some reason (eg. DISPLAY
    // isn't set) this will hang...
    wait_for_completion(&mut (*data).ready);

    um_free_irq(XTERM_IRQ, data as *mut c_void);

    ret = (*data).new_fd;
    *pid_out = (*data).pid;
    kfree(data as *mut c_void);

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
