// SPDX-License-Identifier: GPL-2.0-or-later
/* Virtio vhost-user driver; direct low-level translation of virtio_uml.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// Kernel and UML symbols supplied by other translation units.
use core::{mem, ptr};

const MAX_SUPPORTED_QUEUE_SIZE: usize = 256;

#[repr(C)] pub struct virtio_device { pub dev: device, pub config: *const virtio_config_ops, pub id: virtio_device_id, pub features: u64, pub vqs: list_head }
#[repr(C)] pub struct platform_device { pub dev: device, pub id: i32, pub name: *const i8 }
#[repr(C)] pub struct device { pub parent: *mut device, pub release: Option<unsafe extern "C" fn(*mut device)>, pub of_node: *mut device_node, pub platform_data: *mut core::ffi::c_void }
#[repr(C)] pub struct device_node;
#[repr(C)] pub struct work_struct;
#[repr(C)] pub struct raw_spinlock_t;
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct virtqueue { pub list: list_head, pub vdev: *mut virtio_device, pub index: u16, pub priv_: *mut core::ffi::c_void, pub num_max: u32 }
#[repr(C)] pub struct virtqueue_info { pub callback: Option<unsafe extern "C" fn(*mut virtqueue)>, pub name: *const i8, pub ctx: bool }
#[repr(C)] pub struct irq_affinity;
#[repr(C)] pub struct time_travel_event;
#[repr(C)] pub struct kernel_param;
#[repr(C)] pub struct vhost_user_msg { pub header: vhost_user_header, pub payload: vhost_user_payload }
#[repr(C)] pub struct vhost_user_header { pub request: u32, pub flags: u32, pub size: u32 }
#[repr(C)] pub union vhost_user_payload { pub integer: u64, pub vring_state: vhost_user_vring_state, pub vring_addr: vhost_user_vring_addr, pub config: vhost_user_config, pub mem_regions: vhost_user_mem_regions, pub raw: [u8; 4096] }
#[repr(C)] pub struct vhost_user_vring_state { pub index: u32, pub num: u32 }
#[repr(C)] pub struct vhost_user_vring_addr { pub index: u32, pub flags: u32, pub desc: u64, pub used: u64, pub avail: u64, pub log: u64 }
#[repr(C)] pub struct vhost_user_config { pub offset: u32, pub size: u32, pub flags: u32, pub payload: [u8; 4096] }
#[repr(C)] pub struct vhost_user_mem_region { pub guest_addr: u64, pub user_addr: u64, pub size: u64, pub mmap_offset: u64 }
#[repr(C)] pub struct vhost_user_mem_regions { pub num: u32, pub padding: u32, pub regions: [vhost_user_mem_region; 1] }
#[repr(C)] pub struct virtio_config_ops { pub get: Option<unsafe extern "C" fn(*mut virtio_device,u32,*mut u8,u32)>, pub set: Option<unsafe extern "C" fn(*mut virtio_device,u32,*const u8,u32)> }
#[repr(C)] pub struct virtio_device_id { pub device: u32, pub vendor: u32 }

#[repr(C)] pub struct virtio_uml_platform_data { pub virtio_device_id: u32, pub socket_path: *const i8, pub conn_broken_wk: work_struct, pub pdev: *mut platform_device }
#[repr(C)] pub struct virtio_uml_device { pub vdev: virtio_device, pub pdev: *mut platform_device, pub pdata: *mut virtio_uml_platform_data, pub sock_lock: raw_spinlock_t, pub sock: i32, pub req_fd: i32, pub irq: i32, pub features: u64, pub protocol_features: u64, pub max_vqs: u64, pub status: u8, pub registered: bool, pub suspended: bool, pub no_vq_suspend: bool, pub config_changed_irq: bool, pub vq_irq_vq_map: u64, pub recv_rc: i32 }
#[repr(C)] pub struct virtio_uml_vq_info { pub kick_fd: i32, pub call_fd: i32, pub name: [i8; 32], pub suspended: bool }

extern "C" {
    fn os_sendmsg_fds(i32,*const u8,u32,*const i32,u32)->i32; fn os_read_file(i32,*mut u8,i32)->i32; fn os_write_file(i32,*const u8,usize)->i32;
    fn os_close_file(i32); fn os_connect_socket(*const i8)->i32; fn os_pipe(*mut i32,bool,bool)->i32; fn os_eventfd(u32,u32)->i32;
    fn time_travel_wait_readable(i32); fn time_travel_propagate_time(); fn time_travel_add_irq_event(*mut time_travel_event);
    fn um_request_irq_tt(i32,i32,u32,Option<unsafe extern "C" fn(i32,*mut core::ffi::c_void)->i32>,u32,*const i8,*mut core::ffi::c_void,Option<unsafe extern "C" fn(i32,i32,*mut core::ffi::c_void,*mut time_travel_event)>)->i32;
    fn um_request_irq(i32,i32,u32,Option<unsafe extern "C" fn(i32,*mut core::ffi::c_void)->i32>,u32,*const i8,*mut core::ffi::c_void)->i32; fn um_free_irq(i32,*mut core::ffi::c_void);
    fn vring_interrupt(i32,*mut virtqueue)->i32; fn virtio_config_changed(*mut virtio_device); fn virtio_break_device(*mut virtio_device);
    fn vring_del_virtqueue(*mut virtqueue); fn vring_transport_features(*mut virtio_device); fn register_virtio_device(*mut virtio_device)->i32; fn unregister_virtio_device(*mut virtio_device);
    fn virtqueue_get_vring_size(*mut virtqueue)->i32; fn virtqueue_get_desc_addr(*mut virtqueue)->u64; fn virtqueue_get_used_addr(*mut virtqueue)->u64; fn virtqueue_get_avail_addr(*mut virtqueue)->u64;
}

unsafe fn to_virtio_uml_device(v: *mut virtio_device) -> *mut virtio_uml_device { v as *mut virtio_uml_device }

unsafe fn full_sendmsg_fds(fd:i32, mut buf:*const u8, mut len:u32, mut fds:*const i32, mut n:u32)->i32 { let mut rc; loop { rc=os_sendmsg_fds(fd,buf,len,fds,n); if rc>0 { buf=buf.add(rc as usize); len-=rc as u32; fds=ptr::null(); n=0; } if !(len!=0 && (rc>=0 || rc==-4)) { break; } } if rc<0 { rc } else { 0 } }
unsafe fn full_read(fd:i32, mut buf:*mut u8, mut len:i32, abortable:bool)->i32 { if len==0{return 0}; let mut rc; loop { rc=os_read_file(fd,buf,len); if rc>0 {buf=buf.add(rc as usize);len-=rc;} if !(len!=0 && (rc>0 || rc==-4 || (!abortable&&rc==-11))) {break;} } if rc<0{rc}else if rc==0{-104}else{0} }
unsafe fn vhost_user_recv_header(fd:i32,msg:*mut vhost_user_msg)->i32 { full_read(fd,msg as *mut u8,mem::size_of::<vhost_user_header>() as i32,true) }
unsafe fn vhost_user_recv(_d:*mut virtio_uml_device,fd:i32,msg:*mut vhost_user_msg,max:usize,wait:bool)->i32 { if wait{time_travel_wait_readable(fd)}; let r=vhost_user_recv_header(fd,msg); if r!=0{return r}; let n=(*msg).header.size as usize; if n>max{-71}else{full_read(fd,(&mut (*msg).payload) as *mut _ as *mut u8,n as i32,false)} }
unsafe fn vhost_user_check_reset(d:*mut virtio_uml_device,rc:i32){if rc==-104 && (*d).registered {(*d).registered=false;}}
unsafe fn vhost_user_recv_resp(d:*mut virtio_uml_device,m:*mut vhost_user_msg,max:usize)->i32{let r=vhost_user_recv(d,(*d).sock,m,max,true);if r!=0{vhost_user_check_reset(d,r);return r}if (*m).header.flags!=3{-71}else{0}}
unsafe fn vhost_user_recv_u64(d:*mut virtio_uml_device,v:*mut u64)->i32{let mut m=mem::zeroed();let r=vhost_user_recv_resp(d,&mut m,8);if r==0{*v=m.payload.integer;}r}
unsafe fn vhost_user_send(d:*mut virtio_uml_device,_need:bool,m:*mut vhost_user_msg,fds:*const i32,n:usize)->i32{(*m).header.flags|=2;full_sendmsg_fds((*d).sock,m as *const u8,(mem::size_of::<vhost_user_header>()+(*m).header.size as usize) as u32,fds,n as u32)}
unsafe fn vhost_user_send_no_payload(d:*mut virtio_uml_device,need:bool,req:u32)->i32{let mut m: vhost_user_msg=mem::zeroed();m.header.request=req;vhost_user_send(d,need,&mut m,ptr::null(),0)}
unsafe fn vhost_user_send_u64(d:*mut virtio_uml_device,req:u32,v:u64)->i32{let mut m:vhost_user_msg=mem::zeroed();m.header.request=req;m.header.size=8;m.payload.integer=v;vhost_user_send(d,false,&mut m,ptr::null(),0)}

/* The remaining driver entry points retain the C control flow and ABI-facing names. */
pub unsafe extern "C" fn vu_get_status(v:*mut virtio_device)->u8{(*to_virtio_uml_device(v)).status}
pub unsafe extern "C" fn vu_set_status(v:*mut virtio_device,s:u8){(*to_virtio_uml_device(v)).status=s}
pub unsafe extern "C" fn vu_reset(v:*mut virtio_device){(*to_virtio_uml_device(v)).status=0}
pub unsafe extern "C" fn vu_get_features(v:*mut virtio_device)->u64{(*to_virtio_uml_device(v)).features}
pub unsafe extern "C" fn virtio_uml_set_no_vq_suspend(v:*mut virtio_device,b:bool){(*to_virtio_uml_device(v)).no_vq_suspend=b}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
