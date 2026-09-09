/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by linux/file.h in the source tree.

pub const MEMFD_ANON_NAME: &str = "[memfd]";

#[cfg(feature = "CONFIG_MEMFD_CREATE")]
extern "C" {
    pub fn memfd_fcntl(
        file: *mut struct file,
        cmd: ::core::ffi::c_uint,
        arg: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_long;
    pub fn memfd_alloc_folio(
        memfd: *mut struct file,
        idx: pgoff_t,
    ) -> *mut struct folio;

    /*
     * Check for any existing seals on mmap, return an error if access is denied due
     * to sealing, or 0 otherwise.
     *
     * We also update VMA flags if appropriate by manipulating the VMA flags pointed
     * to by vma_flags_ptr.
     */
    pub fn memfd_check_seals_mmap(
        file: *mut struct file,
        vma_flags_ptr: *mut vma_flags_t,
    ) -> ::core::ffi::c_int;
    pub fn memfd_alloc_file(
        name: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_uint,
    ) -> *mut struct file;
    pub fn memfd_get_seals(file: *mut struct file) -> ::core::ffi::c_int;
    pub fn memfd_add_seals(
        file: *mut struct file,
        seals: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
}

#[cfg(not(feature = "CONFIG_MEMFD_CREATE"))]
#[inline]
pub unsafe fn memfd_fcntl(
    _f: *mut struct file,
    _c: ::core::ffi::c_uint,
    _a: ::core::ffi::c_uint,
) -> ::core::ffi::c_long {
    -EINVAL as ::core::ffi::c_long
}

#[cfg(not(feature = "CONFIG_MEMFD_CREATE"))]
#[inline]
pub unsafe fn memfd_alloc_folio(
    _memfd: *mut struct file,
    _idx: pgoff_t,
) -> *mut struct folio {
    ERR_PTR(-EINVAL)
}

#[cfg(not(feature = "CONFIG_MEMFD_CREATE"))]
#[inline]
pub unsafe fn memfd_check_seals_mmap(
    _file: *mut struct file,
    _vma_flags_ptr: *mut vma_flags_t,
) -> ::core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_MEMFD_CREATE"))]
#[inline]
pub unsafe fn memfd_alloc_file(
    _name: *const ::core::ffi::c_char,
    _flags: ::core::ffi::c_uint,
) -> *mut struct file {
    ERR_PTR(-EINVAL)
}

#[cfg(not(feature = "CONFIG_MEMFD_CREATE"))]
#[inline]
pub unsafe fn memfd_get_seals(_file: *mut struct file) -> ::core::ffi::c_int {
    -EINVAL as ::core::ffi::c_int
}

#[cfg(not(feature = "CONFIG_MEMFD_CREATE"))]
#[inline]
pub unsafe fn memfd_add_seals(
    _file: *mut struct file,
    _seals: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    -EINVAL as ::core::ffi::c_int
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
