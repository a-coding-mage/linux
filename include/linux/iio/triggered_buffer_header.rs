/* SPDX-License-Identifier: GPL-2.0 */

// Declarations supplied by linux/iio/buffer.h and linux/interrupt.h are
// represented here as external Rust types.

use core::ffi::c_void;

#[repr(C)]
pub struct iio_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct iio_dev_attr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct iio_buffer_setup_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

// C enum iio_buffer_direction; its concrete declaration is supplied by the
// corresponding dependency header.
pub type iio_buffer_direction = i32;

// C irqreturn_t; its concrete declaration is supplied by the dependency
// header.
pub type irqreturn_t = i32;

pub type IioTriggeredHandler = unsafe extern "C" fn(irq: i32, p: *mut c_void) -> irqreturn_t;

extern "C" {
    pub fn iio_triggered_buffer_setup_ext(
        indio_dev: *mut iio_dev,
        h: Option<IioTriggeredHandler>,
        thread: Option<IioTriggeredHandler>,
        direction: iio_buffer_direction,
        setup_ops: *const iio_buffer_setup_ops,
        buffer_attrs: *const *const iio_dev_attr,
    ) -> i32;

    pub fn iio_triggered_buffer_cleanup(indio_dev: *mut iio_dev);

    pub fn devm_iio_triggered_buffer_setup_ext(
        dev: *mut device,
        indio_dev: *mut iio_dev,
        h: Option<IioTriggeredHandler>,
        thread: Option<IioTriggeredHandler>,
        direction: iio_buffer_direction,
        ops: *const iio_buffer_setup_ops,
        buffer_attrs: *const *const iio_dev_attr,
    ) -> i32;
}

// #define iio_triggered_buffer_setup(indio_dev, h, thread, setup_ops) \
//     iio_triggered_buffer_setup_ext((indio_dev), (h), (thread), \
//         IIO_BUFFER_DIRECTION_IN, (setup_ops), NULL)
#[inline]
pub unsafe fn iio_triggered_buffer_setup(
    indio_dev: *mut iio_dev,
    h: Option<IioTriggeredHandler>,
    thread: Option<IioTriggeredHandler>,
    setup_ops: *const iio_buffer_setup_ops,
) -> i32 {
    iio_triggered_buffer_setup_ext(
        indio_dev,
        h,
        thread,
        IIO_BUFFER_DIRECTION_IN,
        setup_ops,
        core::ptr::null(),
    )
}

// The value is provided by linux/iio/buffer.h.
pub const IIO_BUFFER_DIRECTION_IN: iio_buffer_direction = 0;

// #define devm_iio_triggered_buffer_setup(dev, indio_dev, h, thread, setup_ops) \
//     devm_iio_triggered_buffer_setup_ext((dev), (indio_dev), (h), (thread), \
//         IIO_BUFFER_DIRECTION_IN, (setup_ops), NULL)
#[inline]
pub unsafe fn devm_iio_triggered_buffer_setup(
    dev: *mut device,
    indio_dev: *mut iio_dev,
    h: Option<IioTriggeredHandler>,
    thread: Option<IioTriggeredHandler>,
    setup_ops: *const iio_buffer_setup_ops,
) -> i32 {
    devm_iio_triggered_buffer_setup_ext(
        dev,
        indio_dev,
        h,
        thread,
        IIO_BUFFER_DIRECTION_IN,
        setup_ops,
        core::ptr::null(),
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
