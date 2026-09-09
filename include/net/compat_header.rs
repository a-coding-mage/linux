/* SPDX-License-Identifier: GPL-2.0 */

// Translated from net/compat.h.  linux/compat.h supplies the compatibility
// integer and pointer types used below.

#[repr(C)]
pub struct sock;

#[repr(C)]
pub struct compat_msghdr {
    pub msg_name: compat_uptr_t, /* void * */
    pub msg_namelen: compat_int_t,
    pub msg_iov: compat_uptr_t, /* struct compat_iovec * */
    pub msg_iovlen: compat_size_t,
    pub msg_control: compat_uptr_t, /* void * */
    pub msg_controllen: compat_size_t,
    pub msg_flags: compat_uint_t,
}

#[repr(C)]
pub struct compat_mmsghdr {
    pub msg_hdr: compat_msghdr,
    pub msg_len: compat_uint_t,
}

#[repr(C)]
pub struct compat_cmsghdr {
    pub cmsg_len: compat_size_t,
    pub cmsg_level: compat_int_t,
    pub cmsg_type: compat_int_t,
}

#[repr(C)]
pub struct compat_rtentry {
    pub rt_pad1: u32,
    pub rt_dst: sockaddr, /* target address */
    pub rt_gateway: sockaddr, /* gateway addr (RTF_GATEWAY) */
    pub rt_genmask: sockaddr, /* target network mask (IP) */
    pub rt_flags: u16,
    pub rt_pad2: i16,
    pub rt_pad3: u32,
    pub rt_tos: u8,
    pub rt_class: u8,
    pub rt_pad4: i16,
    pub rt_metric: i16, /* +1 for binary compatibility! */
    pub rt_dev: compat_uptr_t, /* forcing the device at add */
    pub rt_mtu: u32, /* per route MTU/Window */
    pub rt_window: u32, /* Window clamping */
    pub rt_irtt: u16, /* Initial RTT */
}

extern "C" {
    pub fn __get_compat_msghdr(
        kmsg: *mut msghdr,
        msg: *mut compat_msghdr,
        save_addr: *mut *mut sockaddr,
    ) -> i32;
    pub fn get_compat_msghdr(
        _: *mut msghdr,
        _: *mut compat_msghdr,
        _: *mut *mut sockaddr,
        _: *mut *mut iovec,
    ) -> i32;
    pub fn put_cmsg_compat(_: *mut msghdr, _: i32, _: i32, _: i32, _: *mut core::ffi::c_void) -> i32;
    pub fn cmsghdr_from_user_compat_to_kern(
        _: *mut msghdr,
        _: *mut sock,
        _: *mut u8,
        _: i32,
    ) -> i32;
}

#[repr(C, packed)]
pub struct compat_group_req {
    pub gr_interface: u32,
    pub gr_group: __kernel_sockaddr_storage,
}

#[repr(C, packed)]
pub struct compat_group_source_req {
    pub gsr_interface: u32,
    pub gsr_group: __kernel_sockaddr_storage,
    pub gsr_source: __kernel_sockaddr_storage,
}

#[repr(C, packed)]
pub struct compat_group_filter_aux {
    pub gf_interface_aux: u32,
    pub gf_group_aux: __kernel_sockaddr_storage,
    pub gf_fmode_aux: u32,
    pub gf_numsrc_aux: u32,
    pub gf_slist: [__kernel_sockaddr_storage; 1],
}

#[repr(C, packed)]
pub struct compat_group_filter_flex {
    pub gf_interface: u32,
    pub gf_group: __kernel_sockaddr_storage,
    pub gf_fmode: u32,
    pub gf_numsrc: u32,
    pub gf_slist_flex: [__kernel_sockaddr_storage; 0],
}

#[repr(C, packed)]
pub union compat_group_filter {
    pub aux: compat_group_filter_aux,
    pub flex: compat_group_filter_flex,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
