// SPDX-License-Identifier: GPL-2.0
// Rust owner of the unchanged resource.c algorithms and public C ABI.
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

#[cfg(not(CONFIG_X86_64))]
compile_error!("Rust x86 resource currently requires x86-64");

#[allow(improper_ctypes)]
mod bindings {
    use kernel::ffi;
    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/x86_resource_generated.rs"
    ));
}
use bindings as b;
use core::ptr::addr_of;
use kernel::ffi::c_char;

// printk.h's !PRINTK inline still evaluates arguments; it has no external
// symbol. PRINTK_INDEX is explicitly rejected by resource-rust.mk.
macro_rules! printk {
    ($($arg:expr),* $(,)?) => {{
        #[cfg(CONFIG_PRINTK)]
        { b::_printk($($arg),*); }
        #[cfg(not(CONFIG_PRINTK))]
        { let _ = ($($arg),*); }
    }};
}

unsafe fn resource_clip(res: *mut b::resource, start: b::resource_size_t, end: b::resource_size_t) {
    let mut low: b::resource_size_t = 0;
    let mut high: b::resource_size_t = 0;
    if (*res).end < start || (*res).start > end {
        return;
    }
    if (*res).start < start {
        low = start.wrapping_sub((*res).start);
    }
    if (*res).end > end {
        high = (*res).end.wrapping_sub(end);
    }
    // Preserve the original tie-break: equal remnants keep the upper range.
    if low > high {
        (*res).end = start.wrapping_sub(1);
    } else {
        (*res).start = end.wrapping_add(1);
    }
}

unsafe fn remove_e820_regions(avail: *mut b::resource) {
    // pci_x86.h defines a constant false unless both PCI and ACPI are enabled.
    #[cfg(all(CONFIG_PCI, CONFIG_ACPI))]
    let use_e820 = b::pci_use_e820;
    #[cfg(not(all(CONFIG_PCI, CONFIG_ACPI)))]
    let use_e820 = false;
    if !use_e820 {
        return;
    }

    // Only these two fields of the original struct snapshot are observed.
    let mut orig_start = (*avail).start;
    let mut orig_end = (*avail).end;
    let mut i = 0;
    while i < (*b::e820_table).nr_entries {
        // Firmware tables may have a reallocated trailing array. Use raw
        // pointers, never a reference to the entire original fixed-size table.
        let entry = addr_of!((*b::e820_table).entries)
            .cast::<b::e820_entry>()
            .add(i as usize);
        let e820_start = (*entry).addr;
        let e820_end = e820_start.wrapping_add((*entry).size).wrapping_sub(1);
        resource_clip(
            avail,
            e820_start as b::resource_size_t,
            e820_end as b::resource_size_t,
        );
        if orig_start != (*avail).start || orig_end != (*avail).end {
            printk!(
                b"\x016resource: avoiding allocation from e820 entry [mem %#010Lx-%#010Lx]\n\0"
                    .as_ptr()
                    .cast::<c_char>(),
                e820_start,
                e820_end,
            );
            if (*avail).end > (*avail).start {
                // %pa prints the addresses even for an IORESOURCE_UNSET range.
                printk!(
                    b"\x016resource: remaining [mem %pa-%pa] available\n\0"
                        .as_ptr()
                        .cast::<c_char>(),
                    addr_of!((*avail).start),
                    addr_of!((*avail).end),
                );
            }
            orig_start = (*avail).start;
            orig_end = (*avail).end;
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn arch_remove_reservations(avail: *mut b::resource) {
    // Preserve the high 2 MiB BIOS exclusion; low ISA memory remains usable.
    if (*avail).flags & b::RUST_RESOURCE_IORESOURCE_MEM != 0 {
        resource_clip(
            avail,
            b::RUST_RESOURCE_BIOS_ROM_BASE,
            b::RUST_RESOURCE_BIOS_ROM_END,
        );
        remove_e820_regions(avail);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
