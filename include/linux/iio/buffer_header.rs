/* SPDX-License-Identifier: GPL-2.0-only */
/* The industrial I/O core - generic buffer interfaces.
 *
 * Copyright (c) 2008 Jonathan Cameron
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external, corresponding to linux/sysfs.h and linux/iio/iio.h.

#[repr(C)]
pub struct iio_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct iio_dev {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iio_buffer_direction {
    IIO_BUFFER_DIRECTION_IN,
    IIO_BUFFER_DIRECTION_OUT,
}

extern "C" {
    pub fn iio_push_to_buffers(indio_dev: *mut iio_dev, data: *const core::ffi::c_void) -> i32;

    pub fn iio_pop_from_buffer(
        buffer: *mut iio_buffer,
        data: *mut core::ffi::c_void,
    ) -> i32;

    pub fn iio_push_to_buffers_with_ts_unaligned(
        indio_dev: *mut iio_dev,
        data: *const core::ffi::c_void,
        data_sz: usize,
        timestamp: i64,
    ) -> i32;

    pub fn iio_validate_scan_mask_onehot(
        indio_dev: *mut iio_dev,
        mask: *const usize,
    ) -> bool;

    pub fn iio_device_attach_buffer(
        indio_dev: *mut iio_dev,
        buffer: *mut iio_buffer,
    ) -> i32;
}

/**
 * iio_push_to_buffers_with_timestamp() - push data and timestamp to buffers
 * @indio_dev:        iio_dev structure for device.
 * @data:             sample data
 * @timestamp:        timestamp for the sample data
 *
 * DEPRECATED: Use iio_push_to_buffers_with_ts() instead.
 *
 * Returns 0 on success, a negative error code otherwise.
 */
#[inline]
pub unsafe fn iio_push_to_buffers_with_timestamp(
    indio_dev: *mut iio_dev,
    data: *mut u8,
    timestamp: i64,
) -> i32 {
    // ACCESS_PRIVATE(indio_dev, scan_timestamp)
    // and ACCESS_PRIVATE(indio_dev, scan_timestamp_offset) are supplied by
    // the enclosing IIO implementation.
    if ACCESS_PRIVATE(indio_dev, scan_timestamp) {
        let ts_offset: usize = ACCESS_PRIVATE(indio_dev, scan_timestamp_offset);

        /*
         * The size of indio_dev->scan_bytes is always aligned to the
         * largest scan element's alignment (see iio_compute_scan_bytes()).
         * So there may be padding after the timestamp. ts_offset contains
         * the offset in bytes that was already computed for correctly
         * aligning the timestamp.
         */
        *(data.add(ts_offset) as *mut i64) = timestamp;
    }

    iio_push_to_buffers(indio_dev, data as *const core::ffi::c_void)
}

/**
 * iio_push_to_buffers_with_ts() - push data and timestamp to buffers
 * @indio_dev:        iio_dev structure for device.
 * @data:             Pointer to sample data buffer.
 * @data_total_len:  The size of @data in bytes.
 * @timestamp:       Timestamp for the sample data.
 *
 * Pushes data to the IIO device's buffers. If timestamps are enabled for the
 * device the function will store the supplied timestamp as the last element in
 * the sample data buffer before pushing it to the device buffers. The sample
 * data buffer needs to be large enough to hold the additional timestamp
 * (usually the buffer should be at least indio->scan_bytes bytes large).
 *
 * Context: Any context.
 * Return: 0 on success, a negative error code otherwise.
 */
#[inline]
pub unsafe fn iio_push_to_buffers_with_ts(
    indio_dev: *mut iio_dev,
    data: *mut u8,
    data_total_len: usize,
    timestamp: i64,
) -> i32 {
    if unlikely(data_total_len < (*indio_dev).scan_bytes) {
        dev_err!(&(*indio_dev).dev, "Undersized storage pushed to buffer\n");
        return -ENOSPC;
    }

    iio_push_to_buffers_with_timestamp(indio_dev, data, timestamp)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
