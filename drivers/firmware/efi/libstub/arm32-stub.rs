// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2013 Linaro Ltd;  <roy.franz@linaro.org>
 */

// Dependencies supplied by the surrounding EFI and ARM stub environment.

static mut CPU_STATE_GUID: efi_guid_t = LINUX_EFI_ARM_CPU_STATE_TABLE_GUID;

static mut efi_entry_state: *mut efi_arm_entry_state = core::ptr::null_mut();

unsafe fn get_cpu_state(cpsr: *mut u32, sctlr: *mut u32) {
    core::arch::asm!("mrs {0}, cpsr", out(reg) *cpsr);
    if (*cpsr & MODE_MASK) == HYP_MODE {
        core::arch::asm!("mrc p15, 4, {0}, c1, c0, 0", out(reg) *sctlr);
    } else {
        core::arch::asm!("mrc p15, 0, {0}, c1, c0, 0", out(reg) *sctlr);
    }
}

pub unsafe fn check_platform_features() -> efi_status_t {
    let mut cpsr: u32 = 0;
    let mut sctlr: u32 = 0;
    let mut status: efi_status_t;
    let block: i32;

    get_cpu_state(&mut cpsr, &mut sctlr);

    efi_info!(
        "Entering in {} mode with MMU {}abled\n",
        if (cpsr & MODE_MASK) == HYP_MODE { "HYP" } else { "SVC" },
        if (sctlr & 1) != 0 { "en" } else { "dis" }
    );

    status = efi_bs_call!(
        allocate_pool,
        EFI_LOADER_DATA,
        core::mem::size_of::<efi_arm_entry_state>(),
        &mut efi_entry_state as *mut _ as *mut *mut core::ffi::c_void
    );
    if status != EFI_SUCCESS {
        efi_err!("allocate_pool() failed\n");
        return status;
    }

    (*efi_entry_state).cpsr_before_ebs = cpsr;
    (*efi_entry_state).sctlr_before_ebs = sctlr;

    status = efi_bs_call!(install_configuration_table, &CPU_STATE_GUID, efi_entry_state);
    if status != EFI_SUCCESS {
        efi_err!("install_configuration_table() failed\n");
        goto_free_state(status);
    }

    // Non-LPAE kernels can run anywhere. Build-time CONFIG_ARM_LPAE controls
    // whether the following compatibility check is present.
    if !IS_ENABLED!(CONFIG_ARM_LPAE) {
        return EFI_SUCCESS;
    }

    // LPAE kernels need compatible hardware.
    block = cpuid_feature_extract(CPUID_EXT_MMFR0, 0);
    if block < 5 {
        efi_err!("This LPAE kernel is not supported by your CPU\n");
        status = EFI_UNSUPPORTED;
        efi_bs_call!(install_configuration_table, &CPU_STATE_GUID, core::ptr::null_mut());
        efi_bs_call!(free_pool, efi_entry_state);
        return status;
    }
    return EFI_SUCCESS;
}

unsafe fn goto_free_state(status: efi_status_t) -> ! {
    efi_bs_call!(install_configuration_table, &CPU_STATE_GUID, core::ptr::null_mut());
    efi_bs_call!(free_pool, efi_entry_state);
    panic!("unreachable: translated C goto returned with status {:?}", status);
}

pub unsafe fn efi_handle_post_ebs_state() {
    get_cpu_state(
        &mut (*efi_entry_state).cpsr_after_ebs,
        &mut (*efi_entry_state).sctlr_after_ebs,
    );
}

pub unsafe fn handle_kernel_image(
    image_addr: *mut c_ulong,
    image_size: *mut c_ulong,
    reserve_addr: *mut c_ulong,
    reserve_size: *mut c_ulong,
    image: *mut efi_loaded_image_t,
    image_handle: efi_handle_t,
) -> efi_status_t {
    let slack: c_int = TEXT_OFFSET - 5 * PAGE_SIZE;
    let mut alloc_size: c_int = MAX_UNCOMP_KERNEL_SIZE + EFI_PHYS_ALIGN;
    let mut alloc_base: c_ulong = 0;
    let mut kernel_base: c_ulong;
    let status: efi_status_t;

    /*
     * Allocate space for the decompressed kernel as low as possible.
     * The region should be 16 MiB aligned, but the first 'slack' bytes
     * are not used by Linux, so we allow those to be occupied by the
     * firmware.
     */
    status = efi_low_alloc_above(alloc_size, EFI_PAGE_SIZE, &mut alloc_base, 0x0);
    if status != EFI_SUCCESS {
        efi_err!("Unable to allocate memory for uncompressed kernel.\n");
        return status;
    }

    if (alloc_base % EFI_PHYS_ALIGN) > slack as c_ulong {
        /*
         * More than 'slack' bytes are already occupied at the base of
         * the allocation, so we need to advance to the next 16 MiB block.
         */
        kernel_base = round_up(alloc_base, EFI_PHYS_ALIGN);
        efi_info!(
            "Free memory starts at 0x%lx, setting kernel_base to 0x%lx\n",
            alloc_base,
            kernel_base
        );
    } else {
        kernel_base = round_down(alloc_base, EFI_PHYS_ALIGN);
    }

    *reserve_addr = kernel_base + slack as c_ulong;
    *reserve_size = MAX_UNCOMP_KERNEL_SIZE;

    /* now free the parts that we will not use */
    if *reserve_addr > alloc_base {
        efi_bs_call!(
            free_pages,
            alloc_base,
            (*reserve_addr - alloc_base) / EFI_PAGE_SIZE
        );
        alloc_size -= (*reserve_addr - alloc_base) as c_int;
    }
    efi_bs_call!(
        free_pages,
        *reserve_addr + MAX_UNCOMP_KERNEL_SIZE,
        (alloc_size - MAX_UNCOMP_KERNEL_SIZE) as c_ulong / EFI_PAGE_SIZE
    );

    *image_addr = kernel_base + TEXT_OFFSET;
    *image_size = 0;

    efi_debug!(
        "image addr == 0x%lx, reserve_addr == 0x%lx\n",
        *image_addr,
        *reserve_addr
    );

    EFI_SUCCESS
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
