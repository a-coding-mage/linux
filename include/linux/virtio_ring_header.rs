/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Dependencies supplied by the corresponding Linux Rust bindings are
 * intentionally left external here.
 */

use core::ffi::c_char;

#[repr(C)]
pub struct virtio_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct virtqueue {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

/* The concrete union is supplied by uapi/linux/virtio_ring.h. */
#[repr(C)]
pub union virtio_map {
    pub _opaque: u8,
}

pub type irqreturn_t = i32;
pub type u32 = core::ffi::c_uint;

extern "C" {
    fn virt_mb();
    fn mb();
    fn virt_rmb();
    fn dma_rmb();
    fn virt_wmb();
    fn dma_wmb();
    fn virt_store_mb<T>(p: *mut T, v: T);
    fn WRITE_ONCE<T>(p: *mut T, v: T);

    pub fn vring_create_virtqueue(
        index: core::ffi::c_uint,
        num: core::ffi::c_uint,
        vring_align: core::ffi::c_uint,
        vdev: *mut virtio_device,
        weak_barriers: bool,
        may_reduce_num: bool,
        ctx: bool,
        notify: Option<unsafe extern "C" fn(*mut virtqueue) -> bool>,
        callback: Option<unsafe extern "C" fn(*mut virtqueue)>,
        name: *const c_char,
    ) -> *mut virtqueue;

    pub fn vring_create_virtqueue_map(
        index: core::ffi::c_uint,
        num: core::ffi::c_uint,
        vring_align: core::ffi::c_uint,
        vdev: *mut virtio_device,
        weak_barriers: bool,
        may_reduce_num: bool,
        ctx: bool,
        notify: Option<unsafe extern "C" fn(*mut virtqueue) -> bool>,
        callback: Option<unsafe extern "C" fn(*mut virtqueue)>,
        name: *const c_char,
        map: virtio_map,
    ) -> *mut virtqueue;

    pub fn vring_new_virtqueue(
        index: core::ffi::c_uint,
        num: core::ffi::c_uint,
        vring_align: core::ffi::c_uint,
        vdev: *mut virtio_device,
        weak_barriers: bool,
        ctx: bool,
        pages: *mut core::ffi::c_void,
        notify: Option<unsafe extern "C" fn(*mut virtqueue) -> bool>,
        callback: Option<unsafe extern "C" fn(*mut virtqueue)>,
        name: *const c_char,
    ) -> *mut virtqueue;

    pub fn vring_del_virtqueue(vq: *mut virtqueue);
    pub fn vring_transport_features(vdev: *mut virtio_device);
    pub fn vring_interrupt(irq: i32, _vq: *mut core::ffi::c_void) -> irqreturn_t;
    pub fn vring_notification_data(_vq: *mut virtqueue) -> u32;
}

#[inline]
pub unsafe fn virtio_mb(weak_barriers: bool) {
    if weak_barriers {
        virt_mb();
    } else {
        mb();
    }
}

#[inline]
pub unsafe fn virtio_rmb(weak_barriers: bool) {
    if weak_barriers {
        virt_rmb();
    } else {
        dma_rmb();
    }
}

#[inline]
pub unsafe fn virtio_wmb(weak_barriers: bool) {
    if weak_barriers {
        virt_wmb();
    } else {
        dma_wmb();
    }
}

#[inline]
pub unsafe fn virtio_store_mb<T>(weak_barriers: bool, p: *mut T, v: T) {
    if weak_barriers {
        virt_store_mb(p, v);
    } else {
        WRITE_ONCE(p, v);
        mb();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
