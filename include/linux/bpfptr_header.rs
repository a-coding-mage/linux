/* SPDX-License-Identifier: GPL-2.0-only */
/* A pointer that can point to either kernel or userspace memory. */

// Dependencies supplied by the corresponding Linux memory and sockptr headers
// are intentionally left external to this translation.

pub type bpfptr_t = sockptr_t;

#[inline]
pub unsafe fn bpfptr_is_kernel(bpfptr: bpfptr_t) -> bool {
    bpfptr.is_kernel
}

#[inline]
pub unsafe fn KERNEL_BPFPTR(p: *mut core::ffi::c_void) -> bpfptr_t {
    bpfptr_t {
        kernel: p,
        is_kernel: true,
    }
}

#[inline]
pub unsafe fn USER_BPFPTR(p: *mut core::ffi::c_void) -> bpfptr_t {
    bpfptr_t {
        user: p,
        is_kernel: false,
    }
}

#[inline]
pub unsafe fn make_bpfptr(addr: u64, is_kernel: bool) -> bpfptr_t {
    if is_kernel {
        KERNEL_BPFPTR(addr as usize as *mut core::ffi::c_void)
    } else {
        USER_BPFPTR(u64_to_user_ptr(addr))
    }
}

#[inline]
pub unsafe fn bpfptr_is_null(bpfptr: bpfptr_t) -> bool {
    if bpfptr_is_kernel(bpfptr) {
        bpfptr.kernel.is_null()
    } else {
        bpfptr.user.is_null()
    }
}

#[inline]
pub unsafe fn bpfptr_add(bpfptr: *mut bpfptr_t, val: usize) {
    if bpfptr_is_kernel(*bpfptr) {
        (*bpfptr).kernel = (*bpfptr).kernel.add(val);
    } else {
        (*bpfptr).user = (*bpfptr).user.add(val);
    }
}

#[inline]
pub unsafe fn copy_from_bpfptr_offset(
    dst: *mut core::ffi::c_void,
    src: bpfptr_t,
    offset: usize,
    size: usize,
) -> i32 {
    if !bpfptr_is_kernel(src) {
        copy_from_user(dst, src.user.add(offset), size)
    } else {
        copy_from_kernel_nofault(dst, src.kernel.add(offset), size)
    }
}

#[inline]
pub unsafe fn copy_from_bpfptr(
    dst: *mut core::ffi::c_void,
    src: bpfptr_t,
    size: usize,
) -> i32 {
    copy_from_bpfptr_offset(dst, src, 0, size)
}

#[inline]
pub unsafe fn copy_to_bpfptr_offset(
    dst: bpfptr_t,
    offset: usize,
    src: *const core::ffi::c_void,
    size: usize,
) -> i32 {
    copy_to_sockptr_offset(dst, offset, src, size)
}

#[inline]
pub unsafe fn kvmemdup_bpfptr_noprof(
    src: bpfptr_t,
    len: usize,
) -> *mut core::ffi::c_void {
    let p = kvmalloc_node_align_noprof(
        len,
        1,
        GFP_USER | __GFP_NOWARN,
        NUMA_NO_NODE,
    );

    if p.is_null() {
        return ERR_PTR(-ENOMEM);
    }
    if copy_from_bpfptr(p, src, len) != 0 {
        kvfree(p);
        return ERR_PTR(-EFAULT);
    }
    p
}

// C macro: #define kvmemdup_bpfptr(...) alloc_hooks(kvmemdup_bpfptr_noprof(__VA_ARGS__))

#[inline]
pub unsafe fn strncpy_from_bpfptr(
    dst: *mut core::ffi::c_char,
    src: bpfptr_t,
    count: usize,
) -> isize {
    if bpfptr_is_kernel(src) {
        strncpy_from_kernel_nofault(dst, src.kernel, count)
    } else {
        strncpy_from_user(dst, src.user, count)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
