// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel translation unit:
// linux/ioport.h, linux/printk.h, asm/e820/api.h, asm/pci_x86.h

unsafe fn resource_clip(
    res: *mut resource,
    start: resource_size_t,
    end: resource_size_t,
) {
    let mut low: resource_size_t = 0;
    let mut high: resource_size_t = 0;

    if (*res).end < start || (*res).start > end {
        return; // no conflict
    }

    if (*res).start < start {
        low = start - (*res).start;
    }

    if (*res).end > end {
        high = (*res).end - end;
    }

    // Keep the area above or below the conflict, whichever is larger
    if low > high {
        (*res).end = start - 1;
    } else {
        (*res).start = end + 1;
    }
}

unsafe fn remove_e820_regions(avail: *mut resource) {
    let mut i: i32;
    let mut entry: *mut e820_entry;
    let mut e820_start: u64;
    let mut e820_end: u64;
    let mut orig: resource = *avail;

    if !pci_use_e820 {
        return;
    }

    i = 0;
    while i < (*e820_table).nr_entries {
        entry = &mut (*e820_table).entries[i as usize];
        e820_start = (*entry).addr;
        e820_end = (*entry).addr + (*entry).size - 1;

        resource_clip(avail, e820_start, e820_end);
        if orig.start != (*avail).start || orig.end != (*avail).end {
            pr_info(
                "resource: avoiding allocation from e820 entry [mem %#010Lx-%#010Lx]\n",
                e820_start,
                e820_end,
            );
            if (*avail).end > (*avail).start {
                /*
                 * Use %pa instead of %pR because "avail"
                 * is typically IORESOURCE_UNSET, so %pR
                 * shows the size instead of addresses.
                 */
                pr_info(
                    "resource: remaining [mem %pa-%pa] available\n",
                    &mut (*avail).start,
                    &mut (*avail).end,
                );
            }
            orig = *avail;
        }
        i += 1;
    }
}

pub unsafe fn arch_remove_reservations(avail: *mut resource) {
    /*
     * Trim out BIOS area (high 2MB) and E820 regions. We do not remove
     * the low 1MB unconditionally, as this area is needed for some ISA
     * cards requiring a memory range.
     */
    if (*avail).flags & IORESOURCE_MEM != 0 {
        resource_clip(avail, BIOS_ROM_BASE, BIOS_ROM_END);

        remove_e820_regions(avail);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
