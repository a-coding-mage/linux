/* SPDX-License-Identifier: GPL-2.0+ */

/* Translated from hid_bpf.h. C includes and build-provided types are external dependencies. */

#[repr(C)]
pub struct hid_bpf_ctx {
    pub hid: *mut hid_device,
    pub allocated_size: __u32,
    pub value: hid_bpf_ctx__bindgen_ty_1,
}

#[repr(C)]
pub union hid_bpf_ctx__bindgen_ty_1 {
    pub retval: __s32,
    pub size: __s32,
}

pub const HID_BPF_MAX_PROGS_PER_DEV: u32 = 64;
pub const HID_BPF_FLAG_MASK: u32 = ((HID_BPF_FLAG_MAX - 1) << 1) - 1;

#[repr(C)]
pub struct hid_ops {
    pub hid_get_report: Option<unsafe extern "C" fn(*mut hid_report_enum, *const u8) -> *mut hid_report>,
    pub hid_hw_raw_request: Option<unsafe extern "C" fn(*mut hid_device, ::std::os::raw::c_uchar, *mut __u8, usize, hid_report_type, hid_class_request, u64, bool) -> ::std::os::raw::c_int>,
    pub hid_hw_output_report: Option<unsafe extern "C" fn(*mut hid_device, *mut __u8, usize, u64, bool) -> ::std::os::raw::c_int>,
    pub hid_input_report: Option<unsafe extern "C" fn(*mut hid_device, hid_report_type, *mut u8, usize, u32, ::std::os::raw::c_int, u64, bool, bool) -> ::std::os::raw::c_int>,
    pub owner: *mut module,
    pub bus_type: *const bus_type,
}

unsafe extern "C" {
    pub static hid_ops: *const hid_ops;
}

#[repr(C)]
pub struct hid_bpf_ops {
    pub hid_id: ::std::os::raw::c_int,
    pub flags: u32,
    pub list: list_head,
    pub hid_device_event: Option<unsafe extern "C" fn(*mut hid_bpf_ctx, hid_report_type, u64) -> ::std::os::raw::c_int>,
    pub hid_rdesc_fixup: Option<unsafe extern "C" fn(*mut hid_bpf_ctx) -> ::std::os::raw::c_int>,
    pub hid_hw_request: Option<unsafe extern "C" fn(*mut hid_bpf_ctx, ::std::os::raw::c_uchar, hid_report_type, hid_class_request, u64) -> ::std::os::raw::c_int>,
    pub hid_hw_output_report: Option<unsafe extern "C" fn(*mut hid_bpf_ctx, u64) -> ::std::os::raw::c_int>,
    pub hdev: *mut hid_device,
}

#[repr(C)]
pub struct hid_bpf {
    pub device_data: *mut u8,
    pub allocated_data: u32,
    pub destroyed: bool,
    pub rdesc_ops: *mut hid_bpf_ops,
    pub prog_list: list_head,
    pub prog_list_lock: mutex,
    pub srcu: srcu_struct,
}

#[cfg(CONFIG_HID_BPF)]
unsafe extern "C" {
    pub fn dispatch_hid_bpf_device_event(hid: *mut hid_device, type_: hid_report_type, data: *mut u8, buf_size: *mut usize, size: *mut u32, interrupt: ::std::os::raw::c_int, source: u64, from_bpf: bool) -> *mut u8;
    pub fn dispatch_hid_bpf_raw_requests(hdev: *mut hid_device, reportnum: ::std::os::raw::c_uchar, buf: *mut __u8, size: u32, rtype: hid_report_type, reqtype: hid_class_request, source: u64, from_bpf: bool) -> ::std::os::raw::c_int;
    pub fn dispatch_hid_bpf_output_report(hdev: *mut hid_device, buf: *mut __u8, size: u32, source: u64, from_bpf: bool) -> ::std::os::raw::c_int;
    pub fn hid_bpf_connect_device(hdev: *mut hid_device) -> ::std::os::raw::c_int;
    pub fn hid_bpf_disconnect_device(hdev: *mut hid_device);
    pub fn hid_bpf_destroy_device(hid: *mut hid_device);
    pub fn hid_bpf_device_init(hid: *mut hid_device) -> ::std::os::raw::c_int;
    pub fn call_hid_bpf_rdesc_fixup(hdev: *mut hid_device, rdesc: *const u8, size: *mut ::std::os::raw::c_uint) -> *const u8;
}

#[cfg(not(CONFIG_HID_BPF))]
pub unsafe fn dispatch_hid_bpf_device_event(_hid: *mut hid_device, _type_: hid_report_type, data: *mut u8, _buf_size: *mut usize, _size: *mut u32, _interrupt: ::std::os::raw::c_int, _source: u64, _from_bpf: bool) -> *mut u8 { data }
#[cfg(not(CONFIG_HID_BPF))]
pub unsafe fn dispatch_hid_bpf_raw_requests(_hdev: *mut hid_device, _reportnum: ::std::os::raw::c_uchar, _buf: *mut u8, _size: u32, _rtype: hid_report_type, _reqtype: hid_class_request, _source: u64, _from_bpf: bool) -> ::std::os::raw::c_int { 0 }
#[cfg(not(CONFIG_HID_BPF))]
pub unsafe fn dispatch_hid_bpf_output_report(_hdev: *mut hid_device, _buf: *mut __u8, _size: u32, _source: u64, _from_bpf: bool) -> ::std::os::raw::c_int { 0 }
#[cfg(not(CONFIG_HID_BPF))]
pub unsafe fn hid_bpf_connect_device(_hdev: *mut hid_device) -> ::std::os::raw::c_int { 0 }
#[cfg(not(CONFIG_HID_BPF))]
pub unsafe fn hid_bpf_disconnect_device(_hdev: *mut hid_device) {}
#[cfg(not(CONFIG_HID_BPF))]
pub unsafe fn hid_bpf_destroy_device(_hid: *mut hid_device) {}
#[cfg(not(CONFIG_HID_BPF))]
pub unsafe fn hid_bpf_device_init(_hid: *mut hid_device) -> ::std::os::raw::c_int { 0 }
#[cfg(not(CONFIG_HID_BPF))]
pub unsafe fn call_hid_bpf_rdesc_fixup(_hdev: *mut hid_device, rdesc: *const u8, _size: *mut ::std::os::raw::c_uint) -> *const u8 { rdesc }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
