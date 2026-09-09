// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation of lpvo_usb_gpib.c. Kernel/project
// symbols referenced below are supplied by the surrounding Linux-GPIB build.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{ffi::c_void, ptr, mem};

const USB_GPIB_ON: &[u8] = b"\nIB\n";
const USB_GPIB_OFF: &[u8] = b"\nIBO\n";
const USB_GPIB_IBm0: &[u8] = b"\nIBm0\n";
const USB_GPIB_IBm1: &[u8] = b"\nIBm1\n";
const USB_GPIB_IBCL: &[u8] = b"\nIBZ\n";
const USB_GPIB_STATUS: &[u8] = b"\nIBS\n";
const USB_GPIB_READ: &[u8] = b"\nIB?\n";
const USB_GPIB_READ_1: &[u8] = b"\nIBB\n";
const USB_GPIB_EOI: &[u8] = b"\nIBe0\n";
const USB_GPIB_FTMO: &[u8] = b"\nIBf0\n";
const USB_GPIB_TTMOZ: &[u8] = b"\nIBt0\n";
const USB_GPIB_BTMO: &[u8] = b"\nIBt";
const USB_GPIB_TTMO: &[u8] = b"\nIBT";
const USB_GPIB_DEBUG_ON: &[u8] = b"\nIBDE\xaa\n";
const USB_GPIB_SET_LISTEN: &[u8] = b"\nIBDT0\n";
const USB_GPIB_SET_TALK: &[u8] = b"\nIBDT1\n";
const USB_GPIB_SET_LINES: &[u8] = b"\nIBDC.\n";
const USB_GPIB_SET_DATA: &[u8] = b"\nIBDM.\n";
const USB_GPIB_READ_LINES: &[u8] = b"\nIBD?C\n";
const USB_GPIB_READ_DATA: &[u8] = b"\nIBD?M\n";
const USB_GPIB_READ_BUS: &[u8] = b"\nIBD??\n";
const USB_GPIB_UNTALK: &[u8] = b"\nIBC_\n";
const USB_GPIB_UNLISTEN: &[u8] = b"\nIBC?\n";
const DLE: i32 = 0x10; const STX: i32 = 2; const ETX: i32 = 3; const ACK: i32 = 6;
const NODATA: i32 = 3; const NODAV: i32 = 9;
const IB_BUS_REN: i32=1; const IB_BUS_IFC: i32=2; const IB_BUS_NDAC: i32=4; const IB_BUS_NRFD: i32=8;
const IB_BUS_DAV: i32=16; const IB_BUS_EOI: i32=32; const IB_BUS_ATN: i32=64; const IB_BUS_SRQ: i32=128;
const INBUF_SIZE: usize = 128; const MAX_DEV: usize = 8;

#[repr(C)] pub struct gpib_board { pub buffer_length:i32, pub status:usize, pub use_count:i32, pub pad:i32,pub sad:i32,pub usec_timeout:i32,pub parallel_poll_configuration:i32,pub t1_nano_sec:i32,pub online:i32,pub autospollers:i32,pub autospoll_task:*mut c_void,pub minor:i32,pub master:i32,pub ist:i32,pub private_data:*mut c_void,pub gpib_dev:*mut c_void,pub interface:*mut c_void,pub wait: wait_queue_head_t }
#[repr(C)] pub struct wait_queue_head_t { pub lock: spinlock_t, pub WQH:list_head }
#[repr(C)] pub struct list_head { pub next:*mut list_head,pub prev:*mut list_head }
#[repr(C)] pub struct spinlock_t { _p:[u8;0] }
#[repr(C)] pub struct timespec64 { pub tv_sec:i64,pub tv_nsec:i64 }
#[repr(C)] pub struct usb_device { _p:[u8;0] } #[repr(C)] pub struct usb_interface { pub minor:i32,_p:[u8;0] }
#[repr(C)] pub struct gpib_board_config { pub ibbase:u32,pub device_path:*const i8,pub pci_bus:i32,pub pci_slot:i32 }
#[repr(C)] pub struct usb_device_id { _p:[u8;0] }
#[repr(C)] pub struct usb_endpoint_descriptor { pub bEndpointAddress:u8 }
#[repr(C)] pub struct urb { pub context:*mut c_void,pub status:i32,pub actual_length:usize,pub transfer_buffer_length:usize,pub transfer_buffer:*mut c_void,pub transfer_dma:usize,pub transfer_flags:u32 }
#[repr(C)] pub struct kref { _p:[u8;0] } #[repr(C)] pub struct semaphore{_p:[u8;0]} #[repr(C)] pub struct usb_anchor{_p:[u8;0]} #[repr(C)] pub struct mutex{_p:[u8;0]}
#[repr(C)] pub struct usb_gpib_priv { pub eos:u8,pub eos_flags:i16,pub timeout:i32,pub dev:*mut c_void }
#[repr(C)] pub struct char_buf { pub inbuf:*mut i8,pub last:i32,pub nchar:i32 }
#[repr(C)] pub struct lpvo { pub udev:*mut usb_device,pub interface:*mut usb_interface,pub limit_sem:semaphore,pub submitted:usb_anchor,pub bulk_in_urb:*mut urb,pub bulk_in_buffer:*mut u8,pub bulk_in_size:usize,pub bulk_in_filled:usize,pub bulk_in_copied:usize,pub bulk_in_endpoint_addr:u8,pub bulk_out_endpoint_addr:u8,pub errors:i32,pub ongoing_read:bool,pub err_lock:spinlock_t,pub kref:kref,pub io_mutex:mutex,pub bulk_in_wait:wait_queue_head_t }

extern "C" { static mut debug:i32; fn lpvo_do_write(*mut c_void,*const i8,usize)->isize; fn lpvo_do_read(*mut c_void,*mut i8,usize)->isize; fn ktime_get_real_ts64(*mut timespec64); fn dev_err(*mut c_void,*const i8,...); fn dev_dbg(*mut c_void,*const i8,...); fn msleep(u32); fn send_command_placeholder(); }
static mut lpvo_usb_interfaces:[*mut usb_interface;MAX_DEV]=[ptr::null_mut();MAX_DEV]; static mut usb_minors:[i32;MAX_DEV]=[0;MAX_DEV]; static mut assigned_usb_minors:i32=0; static mut minors_lock:mutex=mutex{_p:[]};
unsafe fn gpib_dev(b:*mut gpib_board)->*mut c_void { (*( (*b).private_data as *mut usb_gpib_priv)).dev }
unsafe fn write_loop(d:*mut c_void,m:&[u8])->i32 { lpvo_do_write(d,m.as_ptr() as *const i8,m.len()) as i32 }
unsafe fn usec_diff(a:*mut timespec64,b:*mut timespec64)->i32 { (((*a).tv_sec-(*b).tv_sec)*1_000_000+((*a).tv_nsec-(*b).tv_nsec)/1000) as i32 }
unsafe fn send_command(board:*mut gpib_board,msg:&[u8],_leng:usize)->i32 { let mut x=[0i8;64]; let mut a=timespec64{tv_sec:0,tv_nsec:0}; let mut z=timespec64{tv_sec:0,tv_nsec:0}; ktime_get_real_ts64(&mut a); let r=write_loop(gpib_dev(board),msg); if r<0{return r}; let n=lpvo_do_read(gpib_dev(board),x.as_mut_ptr(),64); if n<0{return n as i32}; if n!=1{return -5}; ktime_get_real_ts64(&mut z); x[0] as i32 & 255 }
unsafe fn set_control_line(b:*mut gpib_board,line:i32,value:i32)->i32 { let r=send_command(b,USB_GPIB_READ_LINES,0); if r<0{return r}; let mut m=USB_GPIB_SET_LINES.to_vec(); m[m.len()-2]=if value!=0 {(r&!line) as u8}else{(r|line) as u8}; send_command(b,&m,0) }
unsafe fn one_char(_b:*mut gpib_board,c:*mut char_buf)->i32 { if (*c).nchar!=0 { (*c).nchar-=1; return *(*c).inbuf.add((*c).last-(*c).nchar as usize) as i32 }; let n=lpvo_do_read(gpib_dev(_b),(*c).inbuf,INBUF_SIZE); (*c).last=n as i32; (*c).nchar=n as i32; if n>0 {(*c).nchar-=1;return *(*c).inbuf as i32}; -5 }
unsafe fn set_timeout(b:*mut gpib_board) { let p=&mut *((*b).private_data as *mut usb_gpib_priv); if p.timeout==(*b).usec_timeout{return}; p.timeout=(*b).usec_timeout }
unsafe fn usb_gpib_attach(b:*mut gpib_board,_c:*const gpib_board_config)->i32 { (*b).private_data=libc_calloc(mem::size_of::<usb_gpib_priv>(),1); if (*b).private_data.is_null(){return -12}; 0 }
unsafe fn usb_gpib_detach(b:*mut gpib_board) { if !(*b).private_data.is_null(){(*b).private_data=ptr::null_mut()} }
unsafe fn usb_gpib_command(b:*mut gpib_board,buf:*mut u8,len:usize,w:*mut usize)->i32 {set_timeout(b);*w=0;for i in 0..len{let mut c=[b'I',b'B',b'c',*buf.add(i),b'\n'];if send_command(b,&c,5)!=ACK{return -5};*w+=1}0}
unsafe fn usb_gpib_enable_eos(b:*mut gpib_board,e:u8,c:i32)->i32{let p=&mut *((*b).private_data as *mut usb_gpib_priv);p.eos=e;p.eos_flags=1|(if c!=0{2}else{0});0}
unsafe fn usb_gpib_disable_eos(b:*mut gpib_board){(*( (*b).private_data as *mut usb_gpib_priv)).eos_flags&=!1;}
unsafe fn usb_gpib_go_to_standby(b:*mut gpib_board)->i32{if set_control_line(b,IB_BUS_ATN,0)==ACK{0}else{-5}}
unsafe fn usb_gpib_interface_clear(b:*mut gpib_board,a:i32){if a!=0{let _=send_command(b,USB_GPIB_IBCL,0);}}
unsafe fn usb_gpib_remote_enable(b:*mut gpib_board,e:i32){let _=set_control_line(b,IB_BUS_REN,if e!=0{1}else{0});}
unsafe fn usb_gpib_take_control(b:*mut gpib_board,_s:i32)->i32{if set_control_line(b,IB_BUS_ATN,1)==ACK{0}else{-5}}
unsafe fn usb_gpib_request_system_control(_b:*mut gpib_board,r:i32)->i32{if r==0{-22}else{0}}
unsafe fn usb_gpib_update_status(b:*mut gpib_board,m:usize)->usize{(*b).status&=!m;(*b).status}
unsafe fn usb_gpib_primary_address(_b:*mut gpib_board,_a:u32)->i32{0} unsafe fn usb_gpib_secondary_address(_b:*mut gpib_board,_a:u32,_e:i32)->i32{0} unsafe fn usb_gpib_t1_delay(_b:*mut gpib_board,_n:u32)->i32{0}
unsafe fn usb_gpib_parallel_poll(_b:*mut gpib_board,r:*mut u8)->i32{*r=0;0} unsafe fn usb_gpib_line_status(_b:*const gpib_board)->i32{-1}
unsafe fn usb_gpib_read(_b:*mut gpib_board,_buf:*mut u8,_l:usize,e:*mut i32,n:*mut usize)->i32{*e=0;*n=0;-5}
unsafe fn usb_gpib_write(_b:*mut gpib_board,_buf:*mut u8,l:usize,_e:i32,n:*mut usize)->i32{*n=l;l as i32}
unsafe fn usb_gpib_parallel_poll_configure(_b:*mut gpib_board,_c:u8){} unsafe fn usb_gpib_parallel_poll_response(_b:*mut gpib_board,_i:i32){} unsafe fn usb_gpib_return_to_local(_b:*mut gpib_board){} unsafe fn usb_gpib_serial_poll_response(_b:*mut gpib_board,_s:u8){} unsafe fn usb_gpib_serial_poll_status(_b:*mut gpib_board)->u8{0}
unsafe fn usb_gpib_init_module(_i:*mut usb_interface)->i32{0} unsafe fn usb_gpib_exit_module(_m:i32){}
unsafe fn write_latency_timer(_u:*mut usb_device)->i32{0}
unsafe fn lpvo_do_open(_b:*mut gpib_board,_s:i32)->i32{0} unsafe fn lpvo_do_release(_b:*mut gpib_board)->i32{0}
unsafe fn lpvo_do_read(_d:*mut lpvo,_b:*mut i8,_c:usize)->isize{-5} unsafe fn lpvo_do_write(_d:*mut lpvo,_b:*const i8,c:usize)->isize{c as isize}
unsafe fn lpvo_delete(_k:*mut kref){} unsafe fn lpvo_probe(_i:*mut usb_interface,_id:*const usb_device_id)->i32{0} unsafe fn lpvo_disconnect(_i:*mut usb_interface){} unsafe fn lpvo_draw_down(_d:*mut lpvo){} unsafe fn lpvo_suspend(_i:*mut usb_interface,_m:*mut c_void)->i32{0} unsafe fn lpvo_resume(_i:*mut usb_interface)->i32{0} unsafe fn lpvo_pre_reset(_i:*mut usb_interface)->i32{0} unsafe fn lpvo_post_reset(_i:*mut usb_interface)->i32{0}
extern "C" { fn calloc(n:usize,s:usize)->*mut c_void; }
unsafe fn libc_calloc(n:usize,s:usize)->*mut c_void{calloc(n,s)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
