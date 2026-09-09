// SPDX-License-Identifier: GPL-2.0
// Dependencies are supplied by the surrounding kernel/Xen Rust bindings.

/*
 * PVH variables.
 *
 * The variable xen_pvh needs to live in a data segment since it is used
 * after startup_{32|64} is invoked, which will clear the .bss segment.
 */
pub static mut xen_pvh: bool = false;

#[cfg(CONFIG_XEN_DOM0)]
pub unsafe fn xen_pvh_setup_gsi(gsi: i32, trigger: i32, polarity: i32) -> i32 {
    let mut setup_gsi = physdev_setup_gsi {
        gsi,
        triggering: if trigger == ACPI_EDGE_SENSITIVE { 0 } else { 1 },
        polarity: if polarity == ACPI_ACTIVE_HIGH { 0 } else { 1 },
    };

    let mut ret = HYPERVISOR_physdev_op(PHYSDEVOP_setup_gsi, &mut setup_gsi);
    if ret == -EEXIST {
        xen_raw_printk("Already setup the GSI :%d\n", gsi);
        ret = 0;
    } else if ret != 0 {
        xen_raw_printk("Fail to setup GSI (%d)!\n", gsi);
    }

    ret
}

/*
 * Reserve e820 UNUSABLE regions to inflate the memory balloon.
 *
 * On PVH dom0 the host memory map is used, RAM regions available to dom0 are
 * located as the same place as in the native memory map, but since dom0 gets
 * less memory than the total amount of host RAM the ranges that can't be
 * populated are converted from RAM -> UNUSABLE.  Use such regions (up to the
 * ratio signaled in EXTRA_MEM_RATIO) in order to inflate the balloon driver at
 * boot.  Doing so prevents the guest (even if just temporary) from using holes
 * in the memory map in order to map grants or foreign addresses, and
 * hopefully limits the risk of a clash with a device MMIO region.  Ideally the
 * hypervisor should notify us which memory ranges are suitable for creating
 * foreign mappings, but that's not yet implemented.
 */
unsafe fn pvh_reserve_extra_memory() {
    let bootp: *mut boot_params = &raw mut boot_params;
    let mut ram_pages: u32 = 0;
    let mut extra_pages: u32;

    for i in 0..(*bootp).e820_entries {
        let e: *mut boot_e820_entry = (*bootp).e820_table.as_mut_ptr().add(i as usize);
        if (*e).type_ != E820_TYPE_RAM {
            continue;
        }
        ram_pages = ram_pages.wrapping_add(
            PFN_DOWN((*e).addr.wrapping_add((*e).size)).wrapping_sub(PFN_UP((*e).addr)),
        );
    }

    /* Max amount of extra memory. */
    extra_pages = EXTRA_MEM_RATIO.wrapping_mul(ram_pages);

    /*
     * Convert UNUSABLE ranges to RAM and reserve them for foreign mapping
     * purposes.
     */
    let mut i = 0;
    while i < (*bootp).e820_entries && extra_pages != 0 {
        let e: *mut boot_e820_entry = (*bootp).e820_table.as_mut_ptr().add(i as usize);
        let mut pages: u32;

        if (*e).type_ != E820_TYPE_UNUSABLE {
            i += 1;
            continue;
        }

        let region_pages = PFN_DOWN((*e).addr.wrapping_add((*e).size))
            .wrapping_sub(PFN_UP((*e).addr));
        pages = core::cmp::min(extra_pages, region_pages);

        if pages != region_pages {
            if (*bootp).e820_entries == (*bootp).e820_table.len() as _ {
                /* No space left to split - skip region. */
                i += 1;
                continue;
            }

            /* Split entry. */
            let next = e.add(1);
            core::ptr::copy(e, next, ((*bootp).e820_entries - i) as usize);
            (*bootp).e820_entries += 1;
            (*next).addr = PAGE_ALIGN((*e).addr).wrapping_add(PFN_PHYS(pages));
            (*e).size = (*next).addr.wrapping_sub((*e).addr);
            (*next).size = (*next).size.wrapping_sub((*e).size);
        }
        (*e).type_ = E820_TYPE_RAM;
        extra_pages = extra_pages.wrapping_sub(pages);

        xen_add_extra_mem(PFN_UP((*e).addr), pages);
        i += 1;
    }
}

unsafe fn pvh_arch_setup() {
    pvh_reserve_extra_memory();

    if xen_initial_domain() {
        xen_add_preferred_consoles();
        /*
         * Disable usage of CPU idle and frequency drivers: when
         * running as hardware domain the exposed native ACPI tables
         * causes idle and/or frequency drivers to attach and
         * malfunction.  It's Xen the entity that controls the idle and
         * frequency states.
         *
         * For unprivileged domains the exposed ACPI tables are
         * fabricated and don't contain such data.
         */
        disable_cpuidle();
        disable_cpufreq();
        WARN_ON(xen_set_default_idle());
    }
}

pub unsafe fn xen_pvh_init(boot_params: *mut boot_params) {
    xen_pvh = true;
    xen_domain_type = XEN_HVM_DOMAIN;
    xen_start_flags = pvh_start_info.flags;

    x86_init.oem.arch_setup = Some(pvh_arch_setup);
    x86_init.oem.banner = Some(xen_banner);

    xen_efi_init(boot_params);

    if xen_initial_domain() {
        let mut op = xen_platform_op {
            cmd: XENPF_get_dom0_console,
            ..core::mem::zeroed()
        };
        let ret = HYPERVISOR_platform_op(&mut op);

        if ret > 0 {
            xen_init_vga(
                &mut op.u.dom0_console,
                core::cmp::min(
                    (ret as usize).wrapping_mul(core::mem::size_of::<core::ffi::c_char>()),
                    core::mem::size_of_val(&op.u.dom0_console),
                ),
                &mut (*boot_params).screen_info,
            );
        }
    }
}

pub unsafe fn mem_map_via_hcall(boot_params_p: *mut boot_params) {
    let mut memmap: xen_memory_map = core::mem::zeroed();
    memmap.nr_entries = (*boot_params_p).e820_table.len() as _;
    set_xen_guest_handle!(memmap.buffer, (*boot_params_p).e820_table.as_mut_ptr());
    let rc = HYPERVISOR_memory_op(XENMEM_memory_map, &mut memmap);
    if rc != 0 {
        xen_raw_printk("XENMEM_memory_map failed (%d)\n", rc);
        BUG!();
    }
    (*boot_params_p).e820_entries = memmap.nr_entries;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
