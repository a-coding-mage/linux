/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by <asm/page.h>.
// `pgprot_t`, `PHYS_PFN`, and `__phys_mem_access_prot` are intentionally
// referenced here rather than redefined.
use core::ffi::c_ulong;

#[inline]
pub unsafe fn pgprot_framebuffer(
    prot: pgprot_t,
    vm_start: c_ulong,
    vm_end: c_ulong,
    offset: c_ulong,
) -> pgprot_t {
    __phys_mem_access_prot(PHYS_PFN(offset), vm_end.wrapping_sub(vm_start), prot)
}

// #define pgprot_framebuffer pgprot_framebuffer

// Contents supplied by <asm-generic/video.h>.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
