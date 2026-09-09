/* SPDX-License-Identifier: GPL-2.0 */

/* C header guard: _ASM_X86_PAGE_H */
/* linux/types.h, asm/page_types.h, and the architecture-specific page
 * headers are supplied by the surrounding translation unit. */

#[cfg(any())]
mod kernel_only {
    /* CONFIG_X86_64 selects asm/page_64.h; otherwise asm/page_32.h. */
}

#[repr(C)]
pub struct page {
    _private: [u8; 0],
}

#[repr(C)]
pub struct range {
    _private: [u8; 0],
}

extern "C" {
    pub static mut pfn_mapped: [range; 0];
    pub static mut nr_pfn_mapped: ::core::ffi::c_int;

    pub fn copy_page(to: *mut ::core::ffi::c_void, from: *mut ::core::ffi::c_void);
    pub fn vma_alloc_folio(
        gfp: ::core::ffi::c_ulong,
        order: ::core::ffi::c_ulong,
        vma: *mut ::core::ffi::c_void,
        vaddr: ::core::ffi::c_ulong,
    ) -> *mut ::core::ffi::c_void;
    pub fn __phys_addr(x: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    pub fn __phys_addr_nodebug(x: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    pub fn __phys_addr_symbol(x: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    pub fn __virt_addr_valid(kaddr: ::core::ffi::c_ulong) -> bool;
    pub fn pfn_to_page(pfn: ::core::ffi::c_ulong) -> *mut page;
}

#[inline(always)]
pub unsafe fn copy_user_page(
    to: *mut ::core::ffi::c_void,
    from: *mut ::core::ffi::c_void,
    _vaddr: ::core::ffi::c_ulong,
    _topage: *mut page,
) {
    copy_page(to, from);
}

/* vma_alloc_zeroed_movable_folio(vma, vaddr) expands to the following call;
 * GFP_HIGHUSER_MOVABLE, __GFP_ZERO, and the folio/vma types come from Linux. */
#[macro_export]
macro_rules! vma_alloc_zeroed_movable_folio {
    ($vma:expr, $vaddr:expr) => {
        $crate::vma_alloc_folio(
            GFP_HIGHUSER_MOVABLE | __GFP_ZERO,
            0,
            $vma,
            $vaddr,
        )
    };
}

#[inline(always)]
pub unsafe fn __pa<T>(x: *const T) -> ::core::ffi::c_ulong {
    __phys_addr(x as ::core::ffi::c_ulong)
}

#[inline(always)]
pub unsafe fn __pa_nodebug<T>(x: *const T) -> ::core::ffi::c_ulong {
    __phys_addr_nodebug(x as ::core::ffi::c_ulong)
}

/* __pa_symbol uses __phys_reloc_hide in the original C implementation. */
#[inline(always)]
pub unsafe fn __pa_symbol<T>(x: *const T) -> ::core::ffi::c_ulong {
    __phys_addr_symbol(x as ::core::ffi::c_ulong)
}

#[inline(always)]
pub unsafe fn __va(x: ::core::ffi::c_ulong) -> *mut ::core::ffi::c_void {
    (x + PAGE_OFFSET) as *mut ::core::ffi::c_void
}

#[inline(always)]
pub unsafe fn __boot_va(x: ::core::ffi::c_ulong) -> *mut ::core::ffi::c_void {
    __va(x)
}

#[inline(always)]
pub unsafe fn __boot_pa<T>(x: *const T) -> ::core::ffi::c_ulong {
    __pa(x)
}

#[inline(always)]
pub unsafe fn virt_to_page<T>(kaddr: *const T) -> *mut page {
    pfn_to_page(__pa(kaddr) >> PAGE_SHIFT)
}

#[inline(always)]
pub unsafe fn virt_addr_valid<T>(kaddr: *const T) -> bool {
    __virt_addr_valid(kaddr as ::core::ffi::c_ulong)
}

#[inline(always)]
pub unsafe fn pfn_to_kaddr(pfn: ::core::ffi::c_ulong) -> *mut ::core::ffi::c_void {
    __va(pfn << PAGE_SHIFT)
}

#[inline(always)]
pub fn __canonical_address(vaddr: u64, vaddr_bits: u8) -> u64 {
    (((vaddr as i64) << (64 - vaddr_bits)) >> (64 - vaddr_bits)) as u64
}

#[inline(always)]
pub fn __is_canonical_address(vaddr: u64, vaddr_bits: u8) -> u64 {
    (__canonical_address(vaddr, vaddr_bits) == vaddr) as u64
}

/* asm-generic/memory_model.h and asm-generic/getorder.h are supplied by the
 * surrounding translation unit. */

pub const HAVE_ARCH_HUGETLB_UNMAPPED_AREA: bool = true;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
