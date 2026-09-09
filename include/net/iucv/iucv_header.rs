/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from iucv.h. */

pub const IUCV_IPRMDATA: u8 = 0x80;
pub const IUCV_IPQUSCE: u8 = 0x40;
pub const IUCV_IPBUFLST: u8 = 0x40;
pub const IUCV_IPPRTY: u8 = 0x20;
pub const IUCV_IPANSLST: u8 = 0x08;
pub const IUCV_IPSYNC: u8 = 0x04;
pub const IUCV_IPLOCAL: u8 = 0x01;

#[repr(C, align(8))]
pub struct iucv_array {
    pub address: u32,
    pub length: u32,
}

pub struct bus_type;
pub struct attribute_group;
pub struct device_driver;
pub struct device;
pub struct list_head;
pub struct iucv_handler;

extern "C" {
    pub static iucv_bus: bus_type;

    pub fn iucv_alloc_device(
        attrs: *const *const attribute_group,
        driver: *mut device_driver,
        priv_: *mut core::ffi::c_void,
        fmt: *const core::ffi::c_char,
        ...,
    ) -> *mut device;
}

#[repr(C)]
pub struct iucv_path {
    pub pathid: u16,
    pub msglim: u16,
    pub flags: u8,
    pub private: *mut core::ffi::c_void,
    pub handler: *mut iucv_handler,
    pub list: list_head,
}

#[repr(C, packed)]
pub struct iucv_message {
    pub id: u32,
    pub audit: u32,
    pub class: u32,
    pub tag: u32,
    pub length: u32,
    pub reply_size: u32,
    pub rmmsg: [u8; 8],
    pub flags: u8,
}

#[repr(C)]
pub struct iucv_handler {
    pub path_pending: Option<unsafe extern "C" fn(*mut iucv_path, *mut u8, *mut u8) -> i32>,
    pub path_complete: Option<unsafe extern "C" fn(*mut iucv_path, *mut u8)>,
    pub path_severed: Option<unsafe extern "C" fn(*mut iucv_path, *mut u8)>,
    pub path_quiesced: Option<unsafe extern "C" fn(*mut iucv_path, *mut u8)>,
    pub path_resumed: Option<unsafe extern "C" fn(*mut iucv_path, *mut u8)>,
    pub message_pending: Option<unsafe extern "C" fn(*mut iucv_path, *mut iucv_message)>,
    pub message_complete: Option<unsafe extern "C" fn(*mut iucv_path, *mut iucv_message)>,
    pub list: list_head,
    pub paths: list_head,
}

extern "C" {
    pub fn iucv_register(handler: *mut iucv_handler, smp: i32) -> i32;
    pub fn iucv_unregister(handler: *mut iucv_handler, smp: i32);
}

pub type gfp_t = usize;

#[inline]
pub unsafe fn iucv_path_alloc(msglim: u16, flags: u8, gfp: gfp_t) -> *mut iucv_path {
    // kzalloc_obj is supplied by the surrounding kernel environment.
    let path = kzalloc_obj::<iucv_path>(gfp);
    if !path.is_null() {
        (*path).msglim = msglim;
        (*path).flags = flags;
    }
    path
}

#[inline]
pub unsafe fn iucv_path_free(path: *mut iucv_path) {
    kfree(path);
}

extern "C" {
    pub fn iucv_path_accept(path: *mut iucv_path, handler: *mut iucv_handler,
        userdata: *mut u8, private: *mut core::ffi::c_void) -> i32;
    pub fn iucv_path_connect(path: *mut iucv_path, handler: *mut iucv_handler,
        userid: *mut u8, system: *mut u8, userdata: *mut u8,
        private: *mut core::ffi::c_void) -> i32;
    pub fn iucv_path_quiesce(path: *mut iucv_path, userdata: *mut u8) -> i32;
    pub fn iucv_path_resume(path: *mut iucv_path, userdata: *mut u8) -> i32;
    pub fn iucv_path_sever(path: *mut iucv_path, userdata: *mut u8) -> i32;
    pub fn iucv_message_purge(path: *mut iucv_path, msg: *mut iucv_message, srccls: u32) -> i32;
    pub fn iucv_message_receive(path: *mut iucv_path, msg: *mut iucv_message, flags: u8,
        buffer: *mut core::ffi::c_void, size: usize, residual: *mut usize) -> i32;
    pub fn __iucv_message_receive(path: *mut iucv_path, msg: *mut iucv_message, flags: u8,
        buffer: *mut core::ffi::c_void, size: usize, residual: *mut usize) -> i32;
    pub fn iucv_message_reject(path: *mut iucv_path, msg: *mut iucv_message) -> i32;
    pub fn iucv_message_reply(path: *mut iucv_path, msg: *mut iucv_message, flags: u8,
        reply: *mut core::ffi::c_void, size: usize) -> i32;
    pub fn iucv_message_send(path: *mut iucv_path, msg: *mut iucv_message, flags: u8,
        srccls: u32, buffer: *mut core::ffi::c_void, size: usize) -> i32;
    pub fn __iucv_message_send(path: *mut iucv_path, msg: *mut iucv_message, flags: u8,
        srccls: u32, buffer: *mut core::ffi::c_void, size: usize) -> i32;
    pub fn iucv_message_send2way(path: *mut iucv_path, msg: *mut iucv_message, flags: u8,
        srccls: u32, buffer: *mut core::ffi::c_void, size: usize,
        answer: *mut core::ffi::c_void, asize: usize, residual: *mut usize) -> i32;
}

#[repr(C)]
pub struct iucv_interface {
    pub message_receive: Option<unsafe extern "C" fn(*mut iucv_path, *mut iucv_message, u8, *mut core::ffi::c_void, usize, *mut usize) -> i32>,
    pub __message_receive: Option<unsafe extern "C" fn(*mut iucv_path, *mut iucv_message, u8, *mut core::ffi::c_void, usize, *mut usize) -> i32>,
    pub message_reply: Option<unsafe extern "C" fn(*mut iucv_path, *mut iucv_message, u8, *mut core::ffi::c_void, usize) -> i32>,
    pub message_reject: Option<unsafe extern "C" fn(*mut iucv_path, *mut iucv_message) -> i32>,
    pub message_send: Option<unsafe extern "C" fn(*mut iucv_path, *mut iucv_message, u8, u32, *mut core::ffi::c_void, usize) -> i32>,
    pub __message_send: Option<unsafe extern "C" fn(*mut iucv_path, *mut iucv_message, u8, u32, *mut core::ffi::c_void, usize) -> i32>,
    pub message_send2way: Option<unsafe extern "C" fn(*mut iucv_path, *mut iucv_message, u8, u32, *mut core::ffi::c_void, usize, *mut core::ffi::c_void, usize, *mut usize) -> i32>,
    pub message_purge: Option<unsafe extern "C" fn(*mut iucv_path, *mut iucv_message, u32) -> i32>,
    pub path_accept: Option<unsafe extern "C" fn(*mut iucv_path, *mut iucv_handler, *mut u8, *mut core::ffi::c_void) -> i32>,
    pub path_connect: Option<unsafe extern "C" fn(*mut iucv_path, *mut iucv_handler, *mut u8, *mut u8, *mut u8, *mut core::ffi::c_void) -> i32>,
    pub path_quiesce: Option<unsafe extern "C" fn(*mut iucv_path, *mut u8) -> i32>,
    pub path_resume: Option<unsafe extern "C" fn(*mut iucv_path, *mut u8) -> i32>,
    pub path_sever: Option<unsafe extern "C" fn(*mut iucv_path, *mut u8) -> i32>,
    pub iucv_register: Option<unsafe extern "C" fn(*mut iucv_handler, i32) -> i32>,
    pub iucv_unregister: Option<unsafe extern "C" fn(*mut iucv_handler, i32)>,
    pub bus: *const bus_type,
    pub root: *mut device,
}

extern "C" {
    pub static mut iucv_if: iucv_interface;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
