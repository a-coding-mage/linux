/* SPDX-License-Identifier: GPL-2.0 */

// The declarations below are enabled when CONFIG_VIRTIO_ANCHOR is enabled.
#[cfg(CONFIG_VIRTIO_ANCHOR)]
pub mod virtio_anchor {
    #[repr(C)]
    pub struct virtio_device {
        _private: [u8; 0],
    }

    unsafe extern "C" {
        pub fn virtio_require_restricted_mem_acc(dev: *mut virtio_device) -> bool;
        pub static mut virtio_check_mem_acc_cb:
            Option<unsafe extern "C" fn(dev: *mut virtio_device) -> bool>;
    }

    #[inline]
    pub unsafe fn virtio_set_mem_acc_cb(
        func: Option<unsafe extern "C" fn(dev: *mut virtio_device) -> bool>,
    ) {
        unsafe {
            virtio_check_mem_acc_cb = func;
        }
    }
}

// CONFIG_VIRTIO_ANCHOR disabled: the C macro expands to an empty do/while block.
#[cfg(not(CONFIG_VIRTIO_ANCHOR))]
#[inline]
pub fn virtio_set_mem_acc_cb<T>(_func: T) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
