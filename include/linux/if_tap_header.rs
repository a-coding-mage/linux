/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding Linux/Rust bindings:
// net/sock.h, linux/skb_array.h, linux/err.h, and linux/errno.h.

pub const MAX_TAP_QUEUES: usize = 256;

pub struct file;
pub struct socket;
pub struct ptr_ring;
pub struct net_device;
pub struct list_head;
pub struct sock;
pub struct sk_buff;
pub struct cdev;
pub struct module;

pub type dev_t = usize;
pub type netdev_features_t = usize;
pub type rx_handler_result_t = i32;

#[repr(C)]
pub struct tap_queue;

#[repr(C)]
pub struct tap_dev {
    pub dev: *mut net_device,
    pub flags: u16,
    /* This array tracks active taps. */
    pub taps: [*mut tap_queue; MAX_TAP_QUEUES],
    /* This list tracks all taps (both enabled and disabled) */
    pub queue_list: list_head,
    pub numvtaps: i32,
    pub numqueues: i32,
    pub tap_features: netdev_features_t,
    pub minor: i32,
    pub update_features: Option<unsafe extern "C" fn(*mut tap_dev, netdev_features_t)>,
    pub count_tx_dropped: Option<unsafe extern "C" fn(*mut tap_dev)>,
    pub count_rx_dropped: Option<unsafe extern "C" fn(*mut tap_dev)>,
}

#[repr(C)]
pub struct tap_queue {
    pub sk: sock,
    pub sock: socket,
    pub vnet_hdr_sz: i32,
    pub tap: *mut tap_dev,
    pub file: *mut file,
    pub flags: u32,
    pub queue_index: u16,
    pub enabled: bool,
    pub next: list_head,
    pub ring: ptr_ring,
}

// CONFIG_TAP selects the externally provided implementations. When disabled,
// the C header's ERR_PTR(-EINVAL) inline implementations are used.
#[cfg(CONFIG_TAP)]
extern "C" {
    pub fn tap_get_socket(file: *mut file) -> *mut socket;
    pub fn tap_get_ptr_ring(file: *mut file) -> *mut ptr_ring;
}

#[cfg(not(CONFIG_TAP))]
#[inline]
pub unsafe fn tap_get_socket(_f: *mut file) -> *mut socket {
    (-22isize) as *mut socket
}

#[cfg(not(CONFIG_TAP))]
#[inline]
pub unsafe fn tap_get_ptr_ring(_f: *mut file) -> *mut ptr_ring {
    (-22isize) as *mut ptr_ring
}

extern "C" {
    pub fn tap_handle_frame(pskb: *mut *mut sk_buff) -> rx_handler_result_t;
    pub fn tap_del_queues(tap: *mut tap_dev);
    pub fn tap_get_minor(major: dev_t, tap: *mut tap_dev) -> i32;
    pub fn tap_free_minor(major: dev_t, tap: *mut tap_dev);
    pub fn tap_queue_resize(tap: *mut tap_dev) -> i32;
    pub fn tap_create_cdev(
        tap_cdev: *mut cdev,
        tap_major: *mut dev_t,
        device_name: *const i8,
        module: *mut module,
    ) -> i32;
    pub fn tap_destroy_cdev(major: dev_t, tap_cdev: *mut cdev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
