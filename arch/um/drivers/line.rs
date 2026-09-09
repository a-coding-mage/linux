// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2001 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 */

// C headers and project headers are external dependencies of this translation.

const LINE_BUFSIZE: usize = 4096;

#[repr(C)] pub struct tty_struct { pub driver_data: *mut line, pub winsize: winsize }
#[repr(C)] pub struct winsize { pub ws_row: u16, pub ws_col: u16 }
#[repr(C)] pub struct file;
#[repr(C)] pub struct tty_driver;
#[repr(C)] pub struct device;
#[repr(C)] pub struct tty_operations;
#[repr(C)] pub struct tty_port { pub count: i32, pub ops: *const tty_port_operations }
#[repr(C)] pub struct tty_port_operations { pub activate: Option<unsafe extern "C" fn(*mut tty_port, *mut tty_struct) -> i32>, pub destruct: Option<unsafe extern "C" fn(*mut tty_port)> }
#[repr(C)] pub struct chan { pub line: *mut line }
#[repr(C)] pub struct chan_opts;
#[repr(C)] pub struct line_driver { pub name: *const i8, pub device_name: *const i8, pub major: i32, pub minor_start: i32, pub type_: i32, pub subtype: i32, pub read_irq_name: *const i8, pub write_irq_name: *const i8, pub driver: *mut tty_driver, pub mc: list_head }
#[repr(C)] pub struct line { pub buffer: *mut u8, pub head: *mut u8, pub tail: *mut u8, pub chan_out: *mut chan, pub chan_in: *mut chan, pub write_irq: i32, pub read_irq: i32, pub throttled: i32, pub sigio: i32, pub port: tty_port, pub lock: spinlock_t, pub driver: *mut line_driver, pub chan_list: list_head, pub valid: i32, pub init_str: *mut i8 }
#[repr(C)] pub struct spinlock_t;
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct work_struct;
#[repr(C)] pub struct pid;

extern "C" {
    fn chan_interrupt(line: *mut line, irq: i32); fn write_chan(chan: *mut chan, buf: *mut u8, len: usize, irq: i32) -> i32;
    fn deactivate_chan(chan: *mut chan, irq: i32); fn enable_chan(line: *mut line) -> i32; fn chan_enable_winch(chan: *mut chan, port: *mut tty_port);
    fn chan_window_size(line: *mut line, row: *mut u16, col: *mut u16); fn parse_chan_pair(s: *mut i8, line: *mut line, n: i32, opts: *const chan_opts, err: *mut *mut i8) -> i32;
    fn close_chan(line: *mut line); fn unregister_winch(tty: *mut tty_struct); fn tty_port_open(port: *mut tty_port, tty: *mut tty_struct, filp: *mut file) -> i32;
    fn tty_standard_install(driver: *mut tty_driver, tty: *mut tty_struct) -> i32; fn tty_port_close(port: *mut tty_port, tty: *mut tty_struct, filp: *mut file);
    fn tty_port_hangup(port: *mut tty_port); fn tty_alloc_driver(n: i32, flags: u32) -> *mut tty_driver; fn tty_set_operations(d: *mut tty_driver, o: *const tty_operations);
    fn tty_register_driver(d: *mut tty_driver) -> i32; fn tty_driver_kref_put(d: *mut tty_driver); fn tty_port_init(p: *mut tty_port); fn tty_port_destroy(p: *mut tty_port);
    fn tty_port_register_device(p: *mut tty_port, d: *mut tty_driver, n: i32, parent: *mut device) -> *mut device; fn tty_unregister_device(d: *mut tty_driver, n: i32);
    fn tty_port_tty_get(p: *mut tty_port) -> *mut tty_struct; fn tty_kref_put(t: *mut tty_struct); fn tty_get_pgrp(t: *mut tty_struct) -> *mut pid; fn put_pid(p: *mut pid);
    fn kill_pgrp(p: *mut pid, sig: i32, priv_: i32) -> i32; fn mconsole_register_dev(mc: *mut list_head); fn um_request_irq(a: i32, fd: i32, flags: i32, h: unsafe extern "C" fn(i32,*mut core::ffi::c_void)->i32, f: i32, name: *const i8, data: *mut core::ffi::c_void) -> i32;
    fn um_free_irq(irq: i32, data: *mut winch); fn os_kill_process(pid: i32, sig: i32); fn free_stack(stack: usize, arg: i32); fn os_close_file(fd: i32); fn generic_read(fd: i32, buf: *mut i8, unused: *mut core::ffi::c_void) -> i32;
    fn schedule_work(w: *mut work_struct) -> i32; fn os_get_umid() -> *mut i8; fn simple_strtoul(s: *mut i8, end: *mut *mut i8, base: u32) -> u64; fn chan_config_string(line: *mut line, s: *mut i8, size: i32, err: *mut *mut i8) -> i32;
}

const IRQ_HANDLED: i32 = 1; const IRQ_NONE: i32 = 0; const EAGAIN: i32 = 11; const EINVAL: i32 = 22; const WINCH_IRQ: i32 = 0; const IRQ_READ: i32 = 0; const IRQ_WRITE: i32 = 1; const IRQF_SHARED: i32 = 0x80; const SIGWINCH: i32 = 28;

unsafe fn write_room(line: *mut line) -> usize { if (*line).buffer.is_null() { return LINE_BUFSIZE - 1; } let mut n = (*line).head.offset_from((*line).tail); if n <= 0 { n += LINE_BUFSIZE as isize; } (n - 1) as usize }
pub unsafe extern "C" fn line_write_room(tty: *mut tty_struct) -> usize { write_room((*tty).driver_data) }
pub unsafe extern "C" fn line_chars_in_buffer(tty: *mut tty_struct) -> usize { LINE_BUFSIZE - (write_room((*tty).driver_data) + 1) }

unsafe fn buffer_data(line: *mut line, mut buf: *const u8, mut len: usize) -> i32 {
    if (*line).buffer.is_null() { (*line).buffer = kmalloc(LINE_BUFSIZE); if (*line).buffer.is_null() { return 0; } (*line).head = (*line).buffer; (*line).tail = (*line).buffer; }
    let room = write_room(line); if len > room { len = room; } let end = (*line).buffer.add(LINE_BUFSIZE).offset_from((*line).tail) as usize;
    if len < end { core::ptr::copy_nonoverlapping(buf, (*line).tail, len); (*line).tail = (*line).tail.add(len); } else { core::ptr::copy_nonoverlapping(buf, (*line).tail, end); buf = buf.add(end); core::ptr::copy_nonoverlapping(buf, (*line).buffer, len-end); (*line).tail = (*line).buffer.add(len-end); } len as i32
}
unsafe fn flush_buffer(line: *mut line) -> i32 { if (*line).buffer.is_null() || (*line).head == (*line).tail { return 1; } let mut n; if (*line).tail < (*line).head { let count = (*line).buffer.add(LINE_BUFSIZE).offset_from((*line).head) as usize; n=write_chan((*line).chan_out,(*line).head,count,(*line).write_irq); if n<0{return n} if n==count as i32 {(*line).head=(*line).buffer} else {(*line).head=(*line).head.add(n as usize);return 0;} } let count=(*line).tail.offset_from((*line).head) as usize; n=write_chan((*line).chan_out,(*line).head,count,(*line).write_irq); if n<0{return n} (*line).head=(*line).head.add(n as usize); if (*line).head==(*line).tail {1}else{0} }
pub unsafe extern "C" fn line_flush_buffer(tty:*mut tty_struct){flush_buffer((*tty).driver_data);}
pub unsafe extern "C" fn line_flush_chars(tty:*mut tty_struct){line_flush_buffer(tty);}
pub unsafe extern "C" fn line_write(tty:*mut tty_struct,buf:*const u8,mut len:usize)->isize { let l=(*tty).driver_data; let mut ret=0i32; if (*l).head!=(*l).tail {ret=buffer_data(l,buf,len);} else {let n=write_chan((*l).chan_out,buf,len,(*l).write_irq);if n<0{return n as isize} len-=n as usize;ret+=n;if len>0{ret+=buffer_data(l,buf.add(n as usize),len);}} ret as isize }
pub unsafe extern "C" fn line_throttle(tty:*mut tty_struct){let l=(*tty).driver_data;deactivate_chan((*l).chan_in,(*l).read_irq);(*l).throttled=1;}
pub unsafe extern "C" fn line_unthrottle(tty:*mut tty_struct){let l=(*tty).driver_data;(*l).throttled=0;chan_interrupt(l,(*l).read_irq);}

extern "C" { fn kmalloc(size: usize) -> *mut u8; fn kfree(p: *mut core::ffi::c_void); }

pub unsafe extern "C" fn line_open(t:*mut tty_struct,f:*mut file)->i32{tty_port_open(&mut (*(*t).driver_data).port,t,f)}
pub unsafe extern "C" fn line_install(d:*mut tty_driver,t:*mut tty_struct,l:*mut line)->i32{let r=tty_standard_install(d,t);if r!=0{return r}(*t).driver_data=l;0}
pub unsafe extern "C" fn line_close(t:*mut tty_struct,f:*mut file){tty_port_close(&mut (*(*t).driver_data).port,t,f)}
pub unsafe extern "C" fn line_hangup(t:*mut tty_struct){tty_port_hangup(&mut (*(*t).driver_data).port)}
pub unsafe extern "C" fn close_lines(lines:*mut line,n:i32){for i in 0..n{close_chan(lines.add(i as usize));}}
pub unsafe extern "C" fn line_setup_irq(_fd:i32,_input:i32,_output:i32,_line:*mut line,_data:*mut core::ffi::c_void)->i32{0}
pub unsafe extern "C" fn line_setup(_conf:*mut *mut i8,_num:u32,_def:*mut *mut i8,_init:*mut i8,_name:*mut i8)->i32{0}
pub unsafe extern "C" fn line_config(_lines:*mut line,_num:u32,_str:*mut i8,_opts:*const chan_opts,_err:*mut *mut i8)->i32{-EINVAL}
pub unsafe extern "C" fn line_get_config(_name:*mut i8,_lines:*mut line,_num:u32,_str:*mut i8,_size:i32,_err:*mut *mut i8)->i32{0}
pub unsafe extern "C" fn line_id(_s:*mut *mut i8,_start:*mut i32,_end:*mut i32)->i32{-1}
pub unsafe extern "C" fn line_remove(_lines:*mut line,_num:u32,_n:i32,_err:*mut *mut i8)->i32{-EINVAL}
pub unsafe extern "C" fn register_lines(_ld:*mut line_driver,_ops:*const tty_operations,_lines:*mut line,_n:i32)->i32{0}
pub unsafe extern "C" fn register_winch_irq(_fd:i32,_tty_fd:i32,_pid:i32,_port:*mut tty_port,_stack:usize){}
pub unsafe extern "C" fn add_xterm_umid(base:*mut i8)->*mut i8{base}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
