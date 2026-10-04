// SPDX-License-Identifier: GPL-2.0
// Rust production owner of the unchanged arch/x86/kernel/ebda.c.
#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    missing_docs,
    unsafe_op_in_unsafe_fn,
    clippy::all,
    unreachable_pub
)]

#[allow(improper_ctypes)]
mod bindings {
    use kernel::ffi;
    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/x86_early_platform_generated.rs"
    ));
}
use bindings as b;
use core::ptr::{addr_of, read_unaligned};
use kernel::ffi::{c_uint, c_ulong};

const BIOS_RAM_SIZE_KB_PTR: c_ulong = 0x413;
// The real-mode segment pointer read by asm/bios_ebda.h:get_bios_ebda().
const BIOS_EBDA_SEGMENT_PTR: c_ulong = 0x40e;
const BIOS_START_MIN: c_uint = 0x20000; // 128K, less than this is insane.
const BIOS_START_MAX: c_uint = 0x9f000; // The original conservative upper bound.

#[inline(always)]
unsafe fn read_bios_u16(address: c_ulong) -> c_uint {
    // asm/page.h: __va(x) = (void *)((unsigned long)(x) + PAGE_OFFSET).
    // On x86-64 PAGE_OFFSET is the configured, possibly randomized
    // page_offset_base. phys_to_virt() uses this same conversion.
    // The original loads are ordinary memory accesses, not volatile I/O;
    // 0x413 is unaligned, so never form a Rust reference to that u16.
    read_unaligned(address.wrapping_add(b::page_offset_base) as *const u16) as c_uint
}

/*
 * Reserve the conventional PC BIOS firmware region. Firmware can fail to
 * subtract the EBDA from its conventional-memory report; the conservative
 * 0x9f000 upper bound also reserves the page before VGA for the AMD768MPX
 * prefetch erratum. Paravirtual platforms can disable this reservation when
 * their memory map already accounts for all required sub-1MB regions.
 * Losing some conventional memory is preferable to reusing firmware/DMA RAM,
 * while retaining room for the SMP boot trampoline.
 */
#[no_mangle]
#[cold]
#[link_section = ".init.text"]
pub unsafe extern "C" fn reserve_bios_regions() {
    if (*addr_of!(b::x86_platform)).legacy.reserve_bios_regions == 0 {
        return;
    }

    // Convert the BIOS conventional-memory size from KiB to bytes.
    let mut bios_start = read_bios_u16(BIOS_RAM_SIZE_KB_PTR) << 10;
    if bios_start < BIOS_START_MIN || bios_start > BIOS_START_MAX {
        bios_start = BIOS_START_MAX;
    }

    // Exact Rust expansion of get_bios_ebda(): a segment times 16, zero if absent.
    let ebda_start = read_bios_u16(BIOS_EBDA_SEGMENT_PTR) << 4;
    if ebda_start >= BIOS_START_MIN && ebda_start < bios_start {
        bios_start = ebda_start;
    }

    // Exact memblock_reserve() inline expansion from linux/memblock.h.
    // Use the configured phys_addr_t and real int-returning declaration;
    // the original caller intentionally ignores the return value.
    let _ = b::__memblock_reserve(
        bios_start as b::phys_addr_t,
        (0x100000 - bios_start) as b::phys_addr_t,
        b::RUST_EARLY_PLATFORM_NUMA_NO_NODE,
        b::MEMBLOCK_NONE,
    );
}
