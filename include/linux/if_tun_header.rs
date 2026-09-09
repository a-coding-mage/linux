/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  Universal TUN/TAP device driver.
 *  Copyright (C) 1999-2000 Maxim Krasnyansky <max_mk@yahoo.com>
 */

// C dependencies: <uapi/linux/if_tun.h> and <uapi/linux/virtio_net.h>.

pub const TUN_XDP_FLAG: usize = 0x1;

pub const TUN_MSG_UBUF: u16 = 1;
pub const TUN_MSG_PTR: u16 = 2;

#[repr(C)]
pub struct tun_msg_ctl {
    pub type_: u16,
    pub num: u16,
    pub ptr: *mut core::ffi::c_void,
}

// Preserves: defined when CONFIG_TUN or CONFIG_TUN_MODULE is enabled.
#[cfg(any(CONFIG_TUN, CONFIG_TUN_MODULE))]
extern "C" {
    pub fn tun_get_socket(f: *mut file) -> *mut socket;
    pub fn tun_get_tx_ring(file: *mut file) -> *mut ptr_ring;
    // Callers must hold the consumer_lock of the ring of file.
    pub fn tun_wake_queue(file: *mut file, consumed: core::ffi::c_int);

    pub fn tun_ptr_free(ptr: *mut core::ffi::c_void);
}

#[cfg(any(CONFIG_TUN, CONFIG_TUN_MODULE))]
#[inline]
pub unsafe fn tun_is_xdp_frame(ptr: *mut core::ffi::c_void) -> bool {
    (ptr as usize & TUN_XDP_FLAG) != 0
}

#[cfg(any(CONFIG_TUN, CONFIG_TUN_MODULE))]
#[inline]
pub unsafe fn tun_xdp_to_ptr(xdp: *mut xdp_frame) -> *mut core::ffi::c_void {
    ((xdp as usize) | TUN_XDP_FLAG) as *mut core::ffi::c_void
}

#[cfg(any(CONFIG_TUN, CONFIG_TUN_MODULE))]
#[inline]
pub unsafe fn tun_ptr_to_xdp(ptr: *mut core::ffi::c_void) -> *mut xdp_frame {
    ((ptr as usize) & !TUN_XDP_FLAG) as *mut xdp_frame
}

// C dependencies declared by the including translation unit.
#[cfg(any(CONFIG_TUN, CONFIG_TUN_MODULE))]
pub enum file {}
#[cfg(any(CONFIG_TUN, CONFIG_TUN_MODULE))]
pub enum socket {}
#[cfg(any(CONFIG_TUN, CONFIG_TUN_MODULE))]
pub enum ptr_ring {}
#[cfg(any(CONFIG_TUN, CONFIG_TUN_MODULE))]
pub enum xdp_frame {}

// Preserves the CONFIG_TUN-disabled branch.  ERR_PTR(-EINVAL) is represented
// by the corresponding error pointer value (-EINVAL).
#[cfg(not(any(CONFIG_TUN, CONFIG_TUN_MODULE)))]
pub enum file {}
#[cfg(not(any(CONFIG_TUN, CONFIG_TUN_MODULE)))]
pub enum socket {}
#[cfg(not(any(CONFIG_TUN, CONFIG_TUN_MODULE)))]
pub enum ptr_ring {}
#[cfg(not(any(CONFIG_TUN, CONFIG_TUN_MODULE)))]
pub enum xdp_frame {}

#[cfg(not(any(CONFIG_TUN, CONFIG_TUN_MODULE)))]
#[inline]
pub unsafe fn tun_get_socket(_f: *mut file) -> *mut socket {
    (-22isize) as *mut socket
}

#[cfg(not(any(CONFIG_TUN, CONFIG_TUN_MODULE)))]
#[inline]
pub unsafe fn tun_get_tx_ring(_f: *mut file) -> *mut ptr_ring {
    (-22isize) as *mut ptr_ring
}

#[cfg(not(any(CONFIG_TUN, CONFIG_TUN_MODULE)))]
#[inline]
pub unsafe fn tun_wake_queue(_f: *mut file, _consumed: core::ffi::c_int) {}

#[cfg(not(any(CONFIG_TUN, CONFIG_TUN_MODULE)))]
#[inline]
pub unsafe fn tun_is_xdp_frame(_ptr: *mut core::ffi::c_void) -> bool {
    false
}

#[cfg(not(any(CONFIG_TUN, CONFIG_TUN_MODULE)))]
#[inline]
pub unsafe fn tun_xdp_to_ptr(_xdp: *mut xdp_frame) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

#[cfg(not(any(CONFIG_TUN, CONFIG_TUN_MODULE)))]
#[inline]
pub unsafe fn tun_ptr_to_xdp(_ptr: *mut core::ffi::c_void) -> *mut xdp_frame {
    core::ptr::null_mut()
}

#[cfg(not(any(CONFIG_TUN, CONFIG_TUN_MODULE)))]
#[inline]
pub unsafe fn tun_ptr_free(_ptr: *mut core::ffi::c_void) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
