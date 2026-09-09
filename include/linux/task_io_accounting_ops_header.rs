/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Task I/O accounting operations
 */

/* Dependency: declarations from <linux/sched.h> are supplied externally. */

#[cfg(CONFIG_TASK_IO_ACCOUNTING)]
#[inline]
pub unsafe fn task_io_account_read(bytes: usize) {
    (*current).ioac.read_bytes = (*current).ioac.read_bytes.wrapping_add(bytes);
}

/*
 * We approximate number of blocks, because we account bytes only.
 * A 'block' is 512 bytes
 */
#[cfg(CONFIG_TASK_IO_ACCOUNTING)]
#[inline]
pub unsafe fn task_io_get_inblock(p: *const task_struct) -> ::core::ffi::c_ulong {
    ((*p).ioac.read_bytes >> 9) as ::core::ffi::c_ulong
}

#[cfg(CONFIG_TASK_IO_ACCOUNTING)]
#[inline]
pub unsafe fn task_io_account_write(bytes: usize) {
    (*current).ioac.write_bytes = (*current).ioac.write_bytes.wrapping_add(bytes);
}

/*
 * We approximate number of blocks, because we account bytes only.
 * A 'block' is 512 bytes
 */
#[cfg(CONFIG_TASK_IO_ACCOUNTING)]
#[inline]
pub unsafe fn task_io_get_oublock(p: *const task_struct) -> ::core::ffi::c_ulong {
    ((*p).ioac.write_bytes >> 9) as ::core::ffi::c_ulong
}

#[cfg(CONFIG_TASK_IO_ACCOUNTING)]
#[inline]
pub unsafe fn task_io_account_cancelled_write(bytes: usize) {
    (*current).ioac.cancelled_write_bytes =
        (*current).ioac.cancelled_write_bytes.wrapping_add(bytes);
}

#[cfg(CONFIG_TASK_IO_ACCOUNTING)]
#[inline]
pub unsafe fn task_io_accounting_init(ioac: *mut task_io_accounting) {
    ::core::ptr::write_bytes(ioac, 0, 1);
}

#[cfg(CONFIG_TASK_IO_ACCOUNTING)]
#[inline]
pub unsafe fn task_blk_io_accounting_add(
    dst: *mut task_io_accounting,
    src: *mut task_io_accounting,
) {
    (*dst).read_bytes = (*dst).read_bytes.wrapping_add((*src).read_bytes);
    (*dst).write_bytes = (*dst).write_bytes.wrapping_add((*src).write_bytes);
    (*dst).cancelled_write_bytes = (*dst)
        .cancelled_write_bytes
        .wrapping_add((*src).cancelled_write_bytes);
}

#[cfg(not(CONFIG_TASK_IO_ACCOUNTING))]
#[inline]
pub unsafe fn task_io_account_read(_bytes: usize) {}

#[cfg(not(CONFIG_TASK_IO_ACCOUNTING))]
#[inline]
pub unsafe fn task_io_get_inblock(_p: *const task_struct) -> ::core::ffi::c_ulong {
    0
}

#[cfg(not(CONFIG_TASK_IO_ACCOUNTING))]
#[inline]
pub unsafe fn task_io_account_write(_bytes: usize) {}

#[cfg(not(CONFIG_TASK_IO_ACCOUNTING))]
#[inline]
pub unsafe fn task_io_get_oublock(_p: *const task_struct) -> ::core::ffi::c_ulong {
    0
}

#[cfg(not(CONFIG_TASK_IO_ACCOUNTING))]
#[inline]
pub unsafe fn task_io_account_cancelled_write(_bytes: usize) {}

#[cfg(not(CONFIG_TASK_IO_ACCOUNTING))]
#[inline]
pub unsafe fn task_io_accounting_init(_ioac: *mut task_io_accounting) {}

#[cfg(not(CONFIG_TASK_IO_ACCOUNTING))]
#[inline]
pub unsafe fn task_blk_io_accounting_add(
    _dst: *mut task_io_accounting,
    _src: *mut task_io_accounting,
) {
}

#[cfg(CONFIG_TASK_XACCT)]
#[inline]
pub unsafe fn task_chr_io_accounting_add(
    dst: *mut task_io_accounting,
    src: *mut task_io_accounting,
) {
    (*dst).rchar = (*dst).rchar.wrapping_add((*src).rchar);
    (*dst).wchar = (*dst).wchar.wrapping_add((*src).wchar);
    (*dst).syscr = (*dst).syscr.wrapping_add((*src).syscr);
    (*dst).syscw = (*dst).syscw.wrapping_add((*src).syscw);
}

#[cfg(not(CONFIG_TASK_XACCT))]
#[inline]
pub unsafe fn task_chr_io_accounting_add(
    _dst: *mut task_io_accounting,
    _src: *mut task_io_accounting,
) {
}

#[inline]
pub unsafe fn task_io_accounting_add(
    dst: *mut task_io_accounting,
    src: *mut task_io_accounting,
) {
    task_chr_io_accounting_add(dst, src);
    task_blk_io_accounting_add(dst, src);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
