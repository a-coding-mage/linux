/* SPDX-License-Identifier: GPL-2.0 */

// Opaque declarations corresponding to the incomplete C struct types.
#[repr(C)]
pub struct iio_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct iio_buffer_setup_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct iio_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct iio_dev_attr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn iio_kfifo_allocate() -> *mut iio_buffer;
    pub fn iio_kfifo_free(r: *mut iio_buffer);

    pub fn devm_iio_kfifo_buffer_setup_ext(
        dev: *mut device,
        indio_dev: *mut iio_dev,
        setup_ops: *const iio_buffer_setup_ops,
        buffer_attrs: *const *const iio_dev_attr,
    ) -> ::core::ffi::c_int;
}

#[macro_export]
macro_rules! devm_iio_kfifo_buffer_setup {
    ($dev:expr, $indio_dev:expr, $setup_ops:expr) => {
        $crate::devm_iio_kfifo_buffer_setup_ext(
            ($dev),
            ($indio_dev),
            ($setup_ops),
            core::ptr::null(),
        )
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
