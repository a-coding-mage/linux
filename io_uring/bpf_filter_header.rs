// SPDX-License-Identifier: GPL-2.0
//
// Translation of uapi/linux/io_uring/bpf_filter.h declarations.

#[repr(C)]
pub struct io_bpf_filter {
    _private: [u8; 0],
}

#[repr(C)]
pub struct io_kiocb {
    _private: [u8; 0],
}

#[repr(C)]
pub struct io_restriction {
    _private: [u8; 0],
}

#[repr(C)]
pub struct io_uring_bpf {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_IO_URING_BPF")]
extern "C" {
    pub fn __io_uring_run_bpf_filters(
        filters: *mut *mut io_bpf_filter,
        req: *mut io_kiocb,
    ) -> i32;

    pub fn io_register_bpf_filter(
        res: *mut io_restriction,
        arg: *mut io_uring_bpf,
    ) -> i32;

    pub fn io_put_bpf_filters(res: *mut io_restriction);

    pub fn io_bpf_filter_clone(
        dst: *mut io_restriction,
        src: *mut io_restriction,
    );
}

#[cfg(feature = "CONFIG_IO_URING_BPF")]
#[inline]
pub unsafe fn io_uring_run_bpf_filters(
    filters: *mut *mut io_bpf_filter,
    req: *mut io_kiocb,
) -> i32 {
    if !filters.is_null() {
        return __io_uring_run_bpf_filters(filters, req);
    }

    0
}

// CONFIG_IO_URING_BPF disabled: the following inline definitions are the
// corresponding fallback declarations from the C header.
#[cfg(not(feature = "CONFIG_IO_URING_BPF"))]
#[inline]
pub unsafe fn io_register_bpf_filter(
    _res: *mut io_restriction,
    _arg: *mut io_uring_bpf,
) -> i32 {
    -22 // -EINVAL
}

#[cfg(not(feature = "CONFIG_IO_URING_BPF"))]
#[inline]
pub unsafe fn io_uring_run_bpf_filters(
    _filters: *mut *mut io_bpf_filter,
    _req: *mut io_kiocb,
) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_IO_URING_BPF"))]
#[inline]
pub unsafe fn io_put_bpf_filters(_res: *mut io_restriction) {}

#[cfg(not(feature = "CONFIG_IO_URING_BPF"))]
#[inline]
pub unsafe fn io_bpf_filter_clone(
    _dst: *mut io_restriction,
    _src: *mut io_restriction,
) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
