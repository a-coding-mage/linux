/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* industrial I/O buffer definitions needed both in and out of kernel */

/* linux/types.h */

/* Flags for iio_dmabuf.flags */
pub const IIO_BUFFER_DMABUF_CYCLIC: u32 = 1 << 0;
pub const IIO_BUFFER_DMABUF_SUPPORTED_FLAGS: u32 = 0x00000001;

/**
 * struct iio_dmabuf - Descriptor for a single IIO DMABUF object
 * @fd:         file descriptor of the DMABUF object
 * @flags:      one or more IIO_BUFFER_DMABUF_* flags
 * @bytes_used: number of bytes used in this DMABUF for the data transfer.
 *              Should generally be set to the DMABUF's size.
 */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_dmabuf {
    pub fd: u32,
    pub flags: u32,
    pub bytes_used: u64,
}

/*
 * The _IOWR and _IOW ioctl encoding macros are supplied by the Linux ioctl
 * definitions.  They are retained here as external macro dependencies.
 */
pub const IIO_BUFFER_GET_FD_IOCTL: usize =
    _IOWR!(b'i', 0x91, core::ffi::c_int);
pub const IIO_BUFFER_DMABUF_ATTACH_IOCTL: usize =
    _IOW!(b'i', 0x92, core::ffi::c_int);
pub const IIO_BUFFER_DMABUF_DETACH_IOCTL: usize =
    _IOW!(b'i', 0x93, core::ffi::c_int);
pub const IIO_BUFFER_DMABUF_ENQUEUE_IOCTL: usize =
    _IOW!(b'i', 0x94, iio_dmabuf);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
