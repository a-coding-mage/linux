/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * vma_internal.h
 *
 * Header providing userland wrappers and shims for the functionality provided
 * by mm/vma_internal.h.
 *
 * We make the header guard the same as mm/vma_internal.h, so if this shim
 * header is included, it precludes the inclusion of the kernel one.
 */

/* Header guard removed in Rust: __MM_VMA_INTERNAL_H. */

/* Dependency intent from C includes:
 * - <stdlib.h>
 * - <linux/args.h>
 * - <linux/atomic.h>
 * - <linux/bitmap.h>
 * - <linux/list.h>
 * - <linux/maple_tree.h>
 * - <linux/mm.h>
 * - <linux/rbtree.h>
 * - <linux/refcount.h>
 * - <linux/slab.h>
 * - "include/stubs.h"
 * - "include/dup.h"
 * - "include/custom.h"
 */

pub const CONFIG_MMU: i32 = 1;
pub const CONFIG_PER_VMA_LOCK: i32 = 1;

/* C undefines __CONCAT here when already provided by included headers. */

/*
 * DUPLICATE typedef definitions from kernel source that have to be declared
 * ahead of all other headers.
 */

/* C macro marker: #define __private */

/* NUM_MM_FLAG_BITS defined by test code.
 * C field declaration:
 *   __private DECLARE_BITMAP(__mm_flags, NUM_MM_FLAG_BITS);
 * DECLARE_BITMAP is supplied by dependencies and expands to the bitmap storage
 * layout. This translation preserves the intended field name and element type.
 */
#[repr(C)]
pub struct mm_flags_t {
    pub __mm_flags: [::std::os::raw::c_ulong; ((NUM_MM_FLAG_BITS + BITS_PER_LONG - 1) / BITS_PER_LONG) as usize],
}

/* NUM_VMA_FLAG_BITS defined by test code.
 * C field declaration:
 *   DECLARE_BITMAP(__vma_flags, NUM_VMA_FLAG_BITS);
 */
#[repr(C)]
pub struct vma_flags_t {
    pub __vma_flags: [::std::os::raw::c_ulong; ((NUM_VMA_FLAG_BITS + BITS_PER_LONG - 1) / BITS_PER_LONG) as usize],
}

pub type vm_flags_t = ::std::os::raw::c_ulong;
/* C macro: #define pgoff_t unsigned long */
pub type pgoff_t = ::std::os::raw::c_ulong;
pub type pgprotval_t = ::std::os::raw::c_ulong;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pgprot {
    pub pgprot: pgprotval_t,
}

pub type pgprot_t = pgprot;

/* C typedef: typedef __bitwise unsigned int vm_fault_t; */
pub type vm_fault_t = ::std::os::raw::c_uint;

#[inline]
pub unsafe fn VM_WARN_ON(_expr: bool) -> ::std::os::raw::c_int {
    unsafe { WARN_ON(_expr) }
}

#[inline]
pub unsafe fn VM_WARN_ON_ONCE(_expr: bool) -> ::std::os::raw::c_int {
    unsafe { WARN_ON_ONCE(_expr) }
}

#[inline]
pub unsafe fn VM_WARN_ON_ONCE_VMA<T>(_expr: bool, _vma: *mut T) -> ::std::os::raw::c_int {
    unsafe { WARN_ON_ONCE(_expr) }
}

#[inline]
pub unsafe fn VM_WARN_ON_VMG<T>(_expr: bool, _vmg: *mut T) -> ::std::os::raw::c_int {
    unsafe { WARN_ON(_expr) }
}

#[inline]
pub unsafe fn VM_BUG_ON(_expr: bool) {
    unsafe { BUG_ON(_expr) }
}

#[inline]
pub unsafe fn VM_BUG_ON_VMA<T>(_expr: bool, _vma: *mut T) {
    unsafe { BUG_ON(_expr) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
