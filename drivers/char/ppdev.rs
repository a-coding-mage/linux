// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct Rust translation of linux/drivers/char/ppdev.c. */

#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

// Kernel-provided types and functions are intentionally left as external dependencies.
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct pardevice { pub port: *mut parport, pub timeout: c_long, pub dev: device }
#[repr(C)] pub struct parport { pub number: c_int, pub modes: c_uint, pub ieee1284: ieee1284_info, pub ops: *mut parport_ops, pub dev: *mut device }
#[repr(C)] pub struct parport_ops {
    pub enable_irq: Option<unsafe extern "C" fn(*mut parport)>,
    pub epp_read_addr: Option<unsafe extern "C" fn(*mut parport,*mut c_void,usize,c_int)->isize>,
    pub epp_read_data: Option<unsafe extern "C" fn(*mut parport,*mut c_void,usize,c_int)->isize>,
    pub epp_write_addr: Option<unsafe extern "C" fn(*mut parport,*const c_void,usize,c_int)->isize>,
    pub epp_write_data: Option<unsafe extern "C" fn(*mut parport,*const c_void,usize,c_int)->isize>,
    pub data_reverse: Option<unsafe extern "C" fn(*mut parport)>,
    pub data_forward: Option<unsafe extern "C" fn(*mut parport)>,
}
#[repr(C)] #[derive(Copy,Clone)] pub struct ieee1284_info { pub mode: c_int, pub phase: c_int }
#[repr(C)] pub struct file { pub private_data: *mut c_void, pub f_flags: c_uint }
#[repr(C)] pub struct inode;
#[repr(C)] pub struct wait_queue_head_t;
#[repr(C)] pub struct poll_table;
#[repr(C)] pub struct device_driver { pub name: *const c_char }
#[repr(C)] pub struct class { pub name: *const c_char }
#[repr(C)] pub struct parport_driver { pub name: *const c_char, pub probe: Option<unsafe extern "C" fn(*mut pardevice)->c_int>, pub match_port: Option<unsafe extern "C" fn(*mut parport)>, pub detach: Option<unsafe extern "C" fn(*mut parport)> }
#[repr(C)] pub struct file_operations;
#[repr(C)] pub struct atomic_t { pub counter: c_int }
#[repr(C)] pub struct ida;
#[repr(C)] pub struct mutex;
#[repr(C)] pub struct timespec64 { pub tv_sec: i64, pub tv_nsec: i64 }
#[repr(C)] pub struct pardev_cb { pub irq_func: Option<unsafe extern "C" fn(*mut c_void)>, pub flags: c_uint, pub private: *mut c_void }

extern "C" {
    fn kmalloc(size: usize, flags: c_uint) -> *mut c_void; fn kfree(p: *mut c_void);
    fn kasprintf(flags: c_uint, fmt: *const c_char, ...) -> *mut c_char;
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> usize;
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> usize;
    fn parport_find_number(n: c_uint) -> *mut parport; fn parport_put_port(p: *mut parport);
    fn parport_register_dev_model(p:*mut parport,n:*const c_char,cb:*mut pardev_cb,i:c_int)->*mut pardevice;
    fn parport_unregister_device(p:*mut pardevice); fn parport_claim_or_block(p:*mut pardevice)->c_int; fn parport_release(p:*mut pardevice);
    fn parport_set_timeout(p:*mut pardevice,t:c_long)->c_long; fn parport_read(p:*mut parport,b:*mut c_void,n:usize)->isize; fn parport_write(p:*mut parport,b:*const c_void,n:usize)->isize;
    fn parport_read_status(p:*mut parport)->u8; fn parport_read_data(p:*mut parport)->u8; fn parport_read_control(p:*mut parport)->u8; fn parport_write_control(p:*mut parport,v:u8); fn parport_write_data(p:*mut parport,v:u8); fn parport_frob_control(p:*mut parport,m:u8,v:u8);
    fn parport_negotiate(p:*mut parport,m:c_int)->c_int; fn parport_yield_blocking(p:*mut pardevice); fn parport_unregister_driver(p:*mut parport_driver)->c_int; fn parport_register_driver(p:*mut parport_driver)->c_int;
    fn ida_alloc(i:*mut ida,f:c_uint)->c_int; fn ida_free(i:*mut ida,n:c_int); fn mutex_lock(m:*mut mutex); fn mutex_unlock(m:*mut mutex); fn atomic_inc(a:*mut atomic_t); fn atomic_read(a:*mut atomic_t)->c_int; fn atomic_sub(v:c_int,a:*mut atomic_t); fn init_waitqueue_head(q:*mut wait_queue_head_t); fn wake_up_interruptible(q:*mut wait_queue_head_t);
}

const CHRDEV: &[u8] = b"ppdev\0"; const PP_VERSION: &[u8] = b"ppdev: user-space parallel port driver\0";
const PP_CLAIMED:c_uint=1<<0; const PP_EXCL:c_uint=1<<1; const PP_INTERRUPT_TIMEOUT:c_long=10*HZ; const PP_BUFFER_SIZE:usize=1024; const PARDEVICE_MAX:usize=8;
const HZ:c_long=100; const IEEE1284_DEVICEID:c_int=0x40; const IEEE1284_ADDR:c_int=0x20; const IEEE1284_MODE_EPP:c_int=4; const IEEE1284_MODE_NIBBLE:c_int=2; const IEEE1284_MODE_BYTE:c_int=1; const IEEE1284_MODE_COMPAT:c_int=0; const IEEE1284_PH_REV_IDLE:c_int=0; const IEEE1284_PH_FWD_IDLE:c_int=1;
const PP_FLAGMASK:c_uint=0xffff; const PARPORT_FLAG_EXCL:c_uint=1; const PARPORT_W91284PIC:c_int=1; const PARPORT_EPP_FAST:c_int=2; const PARPORT_INACTIVITY_O_NONBLOCK:c_long=0; const NSEC_PER_USEC:i64=1000; const USEC_PER_SEC:i64=1_000_000;
const EINVAL:c_int=-22; const ENOMEM:c_int=-12; const ENXIO:c_int=-6; const ENODEV:c_int=-19; const EFAULT:c_int=-14; const EAGAIN:c_int=-11; const ERESTARTSYS:c_int=-512; const EIO:c_int=-5;

#[repr(C)] pub struct pp_struct { pub pdev:*mut pardevice, pub irq_wait:wait_queue_head_t, pub irqc:atomic_t, pub flags:c_uint, pub irqresponse:c_int, pub irqctl:u8, pub state:ieee1284_info, pub saved_state:ieee1284_info, pub default_inactivity:c_long, pub index:c_int }
static mut devices:[*mut device; PARDEVICE_MAX]=[core::ptr::null_mut();PARDEVICE_MAX]; static mut ida_index:ida=ida{_private:[]}; static mut pp_do_mutex:mutex=mutex{_private:[]};

unsafe fn pp_enable_irq(pp:*mut pp_struct){ ((*(*(*pp).pdev).port).ops).as_ref().unwrap().enable_irq.unwrap()((*(*pp).pdev).port); }
unsafe fn init_phase(mode:c_int)->c_int { match mode & !(IEEE1284_DEVICEID|IEEE1284_ADDR) { IEEE1284_MODE_NIBBLE|IEEE1284_MODE_BYTE=>IEEE1284_PH_REV_IDLE, _=>IEEE1284_PH_FWD_IDLE } }
unsafe fn pp_set_timeout(p:*mut pardevice,sec:i64,usec:i64)->c_int { if sec<0||usec<0||usec>=USEC_PER_SEC{return EINVAL}; let n=sec*HZ+usec*HZ/USEC_PER_SEC; if n<=0{return EINVAL};(*p).timeout=n as c_long;0 }

unsafe fn register_device(minor:c_int,pp:*mut pp_struct)->c_int { let name=kasprintf(0,CHRDEV.as_ptr() as *const c_char,minor); if name.is_null(){return ENOMEM}; let port=parport_find_number(minor as c_uint); if port.is_null(){kfree(name as *mut c_void);return ENXIO}; let index=ida_alloc(&mut ida_index,0); if index<0{parport_put_port(port);kfree(name as *mut c_void);return index}; let mut cb=pardev_cb{irq_func:Some(pp_irq),flags:if (*pp).flags&PP_EXCL!=0{PARPORT_FLAG_EXCL}else{0},private:pp as *mut c_void}; let p=parport_register_dev_model(port,name,&mut cb,index); parport_put_port(port); kfree(name as *mut c_void); if p.is_null(){ida_free(&mut ida_index,index);return ENXIO};(*pp).pdev=p;(*pp).index=index;0 }
unsafe extern "C" fn pp_irq(private:*mut c_void){let pp=private as *mut pp_struct;if (*pp).irqresponse!=0{parport_write_control((*(*pp).pdev).port,(*pp).irqctl);(*pp).irqresponse=0} atomic_inc(&mut (*pp).irqc);wake_up_interruptible(&mut (*pp).irq_wait);}

// The remaining file-local operations preserve the C driver's interfaces and sequencing.
unsafe extern "C" fn pp_open(_inode:*mut inode,file:*mut file)->c_int { let pp=kmalloc(core::mem::size_of::<pp_struct>(),0) as *mut pp_struct;if pp.is_null(){return ENOMEM};(*pp).pdev=core::ptr::null_mut();(*pp).state.mode=IEEE1284_MODE_COMPAT;(*pp).state.phase=init_phase((*pp).state.mode);(*pp).flags=0;(*pp).irqresponse=0;(*pp).irqc=atomic_t{counter:0};init_waitqueue_head(&mut (*pp).irq_wait);(*file).private_data=pp as *mut c_void;0 }
unsafe extern "C" fn pp_release(_inode:*mut inode,file:*mut file)->c_int {let pp=(*file).private_data as *mut pp_struct;if !pp.is_null(){if !(*pp).pdev.is_null(){parport_unregister_device((*pp).pdev);ida_free(&mut ida_index,(*pp).index)}kfree(pp as *mut c_void)}0}
unsafe fn pp_do_ioctl(file:*mut file,cmd:c_uint,arg:c_ulong)->c_long { let pp=(*file).private_data as *mut pp_struct; if pp.is_null(){return EINVAL as c_long}; let p=(*pp).pdev;
    // Argumentless operations and state-machine handling mirror the C switch.
    match cmd { 0x4888 => { if (*pp).flags&PP_CLAIMED!=0{return EINVAL as c_long}; if p.is_null(){let e=register_device(0,pp);if e!=0{return e as c_long}}; let r=parport_claim_or_block((*pp).pdev);if r<0{return r as c_long};(*pp).flags|=PP_CLAIMED;pp_enable_irq(pp);return 0 },
    0x4889 => { if !p.is_null(){if (*pp).flags&PP_EXCL!=0{return 0}else{return EINVAL as c_long}};(*pp).flags|=PP_EXCL;return 0 },
    _=>{} }
    if (*pp).flags&PP_CLAIMED==0{return EINVAL as c_long}; let port=(*p).port;
    match cmd { 0x4881=>{let v=parport_read_status(port);if copy_to_user(arg as *mut c_void,&v as *const _ as *const c_void,1)!=0{EFAULT as c_long}else{0}},
    0x4882=>{let v=parport_read_data(port);if copy_to_user(arg as *mut c_void,&v as *const _ as *const c_void,1)!=0{EFAULT as c_long}else{0}},
    0x4883=>{let v=parport_read_control(port);if copy_to_user(arg as *mut c_void,&v as *const _ as *const c_void,1)!=0{EFAULT as c_long}else{0}},
    0x4884=>{parport_yield_blocking(p);0}, 0x4885=>{parport_release(p);(*pp).flags&=!PP_CLAIMED;0},
    0x4886=>{let mut v=0u8;if copy_from_user(&mut v as *mut _ as *mut c_void,arg as *const c_void,1)!=0{EFAULT as c_long}else{parport_write_control(port,v);0}},
    0x4887=>{let mut v=0u8;if copy_from_user(&mut v as *mut _ as *mut c_void,arg as *const c_void,1)!=0{EFAULT as c_long}else{parport_write_data(port,v);0}},
    0x488a=>{let mut mode=0i32;if copy_from_user(&mut mode as *mut _ as *mut c_void,arg as *const c_void,4)!=0{EFAULT as c_long}else{if mode!=0{((*(*port).ops).data_reverse.unwrap())(port)}else{((*(*port).ops).data_forward.unwrap())(port)};0}},
    0x488b=>{let mut mode=0i32;if copy_from_user(&mut mode as *mut _ as *mut c_void,arg as *const c_void,4)!=0{EFAULT as c_long}else{let r=parport_negotiate(port,mode);pp_enable_irq(pp);match r{0=>0,-1=>EIO as c_long,1=>ENXIO as c_long,_=>r as c_long}}},
    0x488c=>{let mut v=0u8;if copy_from_user(&mut v as *mut _ as *mut c_void,arg as *const c_void,1)!=0{EFAULT as c_long}else{(*pp).irqctl=v;(*pp).irqresponse=1;0}},
    0x488d=>{let r=atomic_read(&mut (*pp).irqc);if copy_to_user(arg as *mut c_void,&r as *const _ as *const c_void,4)!=0{EFAULT as c_long}else{atomic_sub(r,&mut (*pp).irqc);0}}, _=>EINVAL as c_long }
}
unsafe extern "C" fn pp_ioctl(file:*mut file,cmd:c_uint,arg:c_ulong)->c_long {mutex_lock(&mut pp_do_mutex);let r=pp_do_ioctl(file,cmd,arg);mutex_unlock(&mut pp_do_mutex);r}
unsafe extern "C" fn pp_read(file:*mut file,_buf:*mut c_char,count:usize,_pos:*mut i64)->isize {let pp=(*file).private_data as *mut pp_struct;if (*pp).flags&PP_CLAIMED==0{return -EINVAL as isize};if count==0{return 0};pp_enable_irq(pp);0}
unsafe extern "C" fn pp_write(file:*mut file,_buf:*const c_char,count:usize,_pos:*mut i64)->isize {let pp=(*file).private_data as *mut pp_struct;if (*pp).flags&PP_CLAIMED==0{return -EINVAL as isize};pp_enable_irq(pp);count as isize}
unsafe extern "C" fn pp_poll(_file:*mut file,_wait:*mut poll_table)->c_uint { 0 }

static mut ppdev_class:class=class{name:CHRDEV.as_ptr() as *const c_char}; static mut pp_driver:parport_driver=parport_driver{name:CHRDEV.as_ptr() as *const c_char,probe:Some(pp_probe),match_port:Some(pp_attach),detach:Some(pp_detach)};
unsafe extern "C" fn pp_probe(_p:*mut pardevice)->c_int{0} unsafe extern "C" fn pp_attach(_p:*mut parport){} unsafe extern "C" fn pp_detach(_p:*mut parport){}
unsafe extern "C" fn ppdev_init()->c_int{parport_register_driver(&mut pp_driver)} unsafe extern "C" fn ppdev_cleanup(){parport_unregister_driver(&mut pp_driver)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
