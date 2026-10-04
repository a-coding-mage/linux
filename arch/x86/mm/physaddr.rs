// SPDX-License-Identifier: GPL-2.0
// Rust owner of the unchanged physaddr.c x86-64 implementation.
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

#[cfg(not(all(CONFIG_X86_64, CONFIG_SPARSEMEM, CONFIG_SPARSEMEM_EXTREME)))]
compile_error!("Rust physaddr currently requires x86-64 SPARSEMEM_EXTREME");
#[cfg(CONFIG_HAVE_ARCH_PFN_VALID)]
compile_error!("Rust physaddr does not implement an architecture-specific pfn_valid override");

#[allow(improper_ctypes)]
mod bindings {
    use kernel::ffi;
    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/x86_physaddr_generated.rs"
    ));
}
use bindings as b;
use core::ptr::{addr_of, null_mut};
use kernel::ffi::c_ulong;

#[path = "../../../rust/ffi_export.rs"]
mod ffi_export;

const START_KERNEL_MAP: c_ulong = b::RUST_PHYSADDR_START_KERNEL_MAP;
const KERNEL_IMAGE_SIZE: c_ulong = b::RUST_PHYSADDR_KERNEL_IMAGE_SIZE;
const PAGE_SHIFT: u32 = b::RUST_PHYSADDR_PAGE_SHIFT;

// arch/x86/mm/physaddr.h: test the CPU's physical-address width, not MAXMEM.
#[inline(always)]
unsafe fn phys_addr_valid(addr: b::resource_size_t) -> bool {
    #[cfg(CONFIG_PHYS_ADDR_T_64BIT)]
    {
        (addr >> b::boot_cpu_data.x86_phys_bits) == 0
    }
    #[cfg(not(CONFIG_PHYS_ADDR_T_64BIT))]
    {
        let _ = addr;
        true
    }
}

// Ordinary kernel pgtable_l5_enabled() is the patched CPU-feature predicate,
// not the compressed-boot __pgtable_l5_enabled variable. Both values below
// are evaluated from native NR_MEM_SECTIONS, including SECTION_SIZE_BITS.
#[inline(always)]
unsafe fn nr_mem_sections() -> c_ulong {
    if b::rust_physaddr_cpu_has_la57() {
        b::RUST_PHYSADDR_NR_MEM_SECTIONS_L5
    } else {
        b::RUST_PHYSADDR_NR_MEM_SECTIONS_L4
    }
}

// include/linux/mmzone.h::__nr_to_section. Null checks precede pointer math.
#[inline(always)]
unsafe fn nr_to_section(nr: c_ulong) -> *mut b::mem_section {
    let root = nr / b::RUST_PHYSADDR_SECTIONS_PER_ROOT;
    let nr_roots = if b::rust_physaddr_cpu_has_la57() {
        b::RUST_PHYSADDR_NR_SECTION_ROOTS_L5
    } else {
        b::RUST_PHYSADDR_NR_SECTION_ROOTS_L4
    };
    if root >= nr_roots || b::mem_section.is_null() {
        return null_mut();
    }
    let sections = *b::mem_section.add(root as usize);
    if sections.is_null() {
        return null_mut();
    }
    sections.add((nr & b::RUST_PHYSADDR_SECTION_ROOT_MASK) as usize)
}

// Translate the RCU-sched sequence itself; native leaves only expand the
// preemption/lockdep compiler macros. Do not substitute the notrace variants.
#[inline(always)]
unsafe fn rcu_read_lock_sched() {
    b::rust_physaddr_preempt_disable();
    #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
    b::rust_physaddr_rcu_lock_acquire();
    #[cfg(CONFIG_PROVE_RCU)]
    b::rust_physaddr_rcu_lock_warn();
}

#[inline(always)]
unsafe fn rcu_read_unlock_sched() {
    #[cfg(CONFIG_PROVE_RCU)]
    b::rust_physaddr_rcu_unlock_warn();
    #[cfg(CONFIG_DEBUG_LOCK_ALLOC)]
    b::rust_physaddr_rcu_lock_release();
    b::rust_physaddr_preempt_enable();
}

#[inline(always)]
unsafe fn pfn_section_valid(ms: *mut b::mem_section, pfn: c_ulong) -> bool {
    #[cfg(CONFIG_SPARSEMEM_VMEMMAP)]
    {
        let idx =
            (pfn & !b::RUST_PHYSADDR_PAGE_SECTION_MASK) / b::RUST_PHYSADDR_PAGES_PER_SUBSECTION;
        let usage = b::rust_physaddr_read_usage(addr_of!((*ms).usage));
        !usage.is_null() && b::rust_physaddr_test_bit(idx, addr_of!((*usage).subsection_map).cast())
    }
    #[cfg(not(CONFIG_SPARSEMEM_VMEMMAP))]
    {
        let _ = (ms, pfn);
        true
    }
}

// include/linux/mmzone.h::pfn_valid, retaining the native RCU lifetime and
// early-section behavior (which accepts holes within the section).
#[inline(always)]
unsafe fn pfn_valid(pfn: c_ulong) -> bool {
    let phys = (pfn as b::phys_addr_t).wrapping_shl(PAGE_SHIFT);
    if ((phys >> PAGE_SHIFT) as c_ulong) != pfn {
        return false;
    }
    let section_nr = pfn >> b::RUST_PHYSADDR_PFN_SECTION_SHIFT;
    if section_nr >= nr_mem_sections() {
        return false;
    }
    let ms = nr_to_section(section_nr);
    rcu_read_lock_sched();
    if ms.is_null() || ((*ms).section_mem_map & b::RUST_PHYSADDR_SECTION_HAS_MEM_MAP) == 0 {
        rcu_read_unlock_sched();
        return false;
    }
    let valid = ((*ms).section_mem_map & b::RUST_PHYSADDR_SECTION_IS_EARLY) != 0
        || pfn_section_valid(ms, pfn);
    rcu_read_unlock_sched();
    valid
}

#[cfg(CONFIG_DEBUG_VIRTUAL)]
#[no_mangle]
pub unsafe extern "C" fn __phys_addr(mut x: c_ulong) -> c_ulong {
    let y = x.wrapping_sub(START_KERNEL_MAP);
    // Unsigned carry distinguishes the kernel-image and direct-map ranges.
    if x > y {
        x = y.wrapping_add(b::phys_base);
        if y >= KERNEL_IMAGE_SIZE {
            b::rust_physaddr_bug_image();
        }
    } else {
        x = y.wrapping_add(START_KERNEL_MAP.wrapping_sub(b::page_offset_base));
        if x > y || !phys_addr_valid(x as b::resource_size_t) {
            b::rust_physaddr_bug_direct_map();
        }
    }
    x
}
#[cfg(CONFIG_DEBUG_VIRTUAL)]
ffi_export::export_symbol!(__phys_addr, __phys_addr, "", "");

#[no_mangle]
pub unsafe extern "C" fn __virt_addr_valid(mut x: c_ulong) -> bool {
    let y = x.wrapping_sub(START_KERNEL_MAP);
    if x > y {
        x = y.wrapping_add(b::phys_base);
        if y >= KERNEL_IMAGE_SIZE {
            return false;
        }
    } else {
        x = y.wrapping_add(START_KERNEL_MAP.wrapping_sub(b::page_offset_base));
        if x > y || !phys_addr_valid(x as b::resource_size_t) {
            return false;
        }
    }
    pfn_valid(x >> PAGE_SHIFT)
}
ffi_export::export_symbol!(__virt_addr_valid, __virt_addr_valid, "", "");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
