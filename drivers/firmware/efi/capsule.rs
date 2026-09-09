// SPDX-License-Identifier: GPL-2.0
/*
 * EFI capsule support.
 *
 * Copyright 2013 Intel Corporation; author Matt Fleming
 */

#[repr(C)]
pub struct efi_capsule_block_desc_t {
    pub length: u64,
    pub data: u64,
}

static mut capsule_pending: bool = false;
static mut stop_capsules: bool = false;
static mut efi_reset_type: i32 = -1;

/*
 * capsule_mutex serialises access to both capsule_pending and
 * efi_reset_type and stop_capsules.
 */
static mut capsule_mutex: () = ();

/**
 * efi_capsule_pending - has a capsule been passed to the firmware?
 * @reset_type: store the type of EFI reset if capsule is pending
 *
 * To ensure that the registered capsule is processed correctly by the
 * firmware we need to perform a specific type of reset. If a capsule is
 * pending return the reset type in @reset_type.
 *
 * This function will race with callers of efi_capsule_update(), for
 * example, calling this function while somebody else is in
 * efi_capsule_update() but hasn't reached efi_capsue_update_locked()
 * will miss the updates to capsule_pending and efi_reset_type after
 * efi_capsule_update_locked() completes.
 *
 * A non-racy use is from platform reboot code because we use
 * system_state to ensure no capsules can be sent to the firmware once
 * we're at SYSTEM_RESTART. See efi_capsule_update_locked().
 */
pub unsafe fn efi_capsule_pending(reset_type: *mut i32) -> bool {
    if !capsule_pending {
        return false;
    }

    if !reset_type.is_null() {
        *reset_type = efi_reset_type;
    }

    true
}

/*
 * Whitelist of EFI capsule flags that we support.
 *
 * We do not handle EFI_CAPSULE_INITIATE_RESET because that would
 * require us to prepare the kernel for reboot. Refuse to load any
 * capsules with that flag and any other flags that we do not know
 * how to handle.
 */
const EFI_CAPSULE_SUPPORTED_FLAG_MASK: u32 =
    EFI_CAPSULE_PERSIST_ACROSS_RESET | EFI_CAPSULE_POPULATE_SYSTEM_TABLE;

/**
 * efi_capsule_supported - does the firmware support the capsule?
 * @guid: vendor guid of capsule
 * @flags: capsule flags
 * @size: size of capsule data
 * @reset: the reset type required for this capsule
 *
 * Check whether a capsule with @flags is supported by the firmware
 * and that @size doesn't exceed the maximum size for a capsule.
 *
 * No attempt is made to check @reset against the reset type required
 * by any pending capsules because of the races involved.
 */
pub unsafe fn efi_capsule_supported(
    guid: efi_guid_t,
    flags: u32,
    size: usize,
    reset: *mut i32,
) -> i32 {
    let mut capsule: efi_capsule_header_t = core::mem::zeroed();
    let mut cap_list: [*mut efi_capsule_header_t; 1] = [&mut capsule];
    let status: efi_status_t;
    let mut max_size: u64 = 0;

    if flags & !EFI_CAPSULE_SUPPORTED_FLAG_MASK != 0 {
        return -EINVAL;
    }

    capsule.headersize = core::mem::size_of::<efi_capsule_header_t>() as _;
    capsule.imagesize = core::mem::size_of::<efi_capsule_header_t>() as _;
    core::ptr::copy_nonoverlapping(
        &guid as *const efi_guid_t,
        &mut capsule.guid as *mut efi_guid_t,
        1,
    );
    capsule.flags = flags;

    status = efi.query_capsule_caps(cap_list.as_mut_ptr(), 1, &mut max_size, reset);
    if status != EFI_SUCCESS {
        return efi_status_to_err(status);
    }

    if size as u64 > max_size {
        return -ENOSPC;
    }

    0
}

/*
 * Every scatter gather list (block descriptor) page must end with a
 * continuation pointer. The last continuation pointer of the last
 * page must be zero to mark the end of the chain.
 */
const SGLIST_PER_PAGE: usize = (PAGE_SIZE / core::mem::size_of::<efi_capsule_block_desc_t>()) - 1;

/*
 * How many scatter gather list (block descriptor) pages do we need
 * to map @count pages?
 */
#[inline]
fn sg_pages_num(count: usize) -> usize {
    (count + SGLIST_PER_PAGE - 1) / SGLIST_PER_PAGE
}

/**
 * efi_capsule_update_locked - pass a single capsule to the firmware
 * @capsule: capsule to send to the firmware
 * @sg_pages: array of scatter gather (block descriptor) pages
 * @reset: the reset type required for @capsule
 *
 * Since this function must be called under capsule_mutex check
 * whether efi_reset_type will conflict with @reset, and atomically
 * set it and capsule_pending if a capsule was successfully sent to
 * the firmware.
 *
 * We also check to see if the system is about to restart, and if so,
 * abort. This avoids races between efi_capsule_update() and
 * efi_capsule_pending().
 */
unsafe fn efi_capsule_update_locked(
    capsule: *mut efi_capsule_header_t,
    sg_pages: *mut *mut page,
    reset: i32,
) -> i32 {
    let sglist_phys: efi_physical_addr_t;
    let status: efi_status_t;

    if efi_reset_type >= 0 && efi_reset_type != reset {
        pr_err!("Conflicting capsule reset type {} ({}).\n", reset, efi_reset_type);
        return -EINVAL;
    }

    if stop_capsules {
        pr_warn!("Capsule update raced with reboot, aborting.\n");
        return -EINVAL;
    }

    sglist_phys = page_to_phys(*sg_pages);
    status = efi.update_capsule(&capsule, 1, sglist_phys);
    if status == EFI_SUCCESS {
        capsule_pending = true;
        efi_reset_type = reset;
    }

    efi_status_to_err(status)
}

/**
 * efi_capsule_update - send a capsule to the firmware
 * @capsule: capsule to send to firmware
 * @pages: an array of capsule data pages
 *
 * Build a scatter gather list with EFI capsule block descriptors to
 * map the capsule described by @capsule with its data in @pages and
 * send it to the firmware via the UpdateCapsule() runtime service.
 *
 * @capsule must be a virtual mapping of the complete capsule update in
 * the kernel address space, as the capsule can be consumed immediately.
 * A capsule_header_t that describes the entire contents of the capsule
 * must be at the start of the first data page.
 *
 * Even though this function will validate that the firmware supports
 * the capsule guid, users will likely want to check that
 * efi_capsule_supported() returns true before calling this function
 * because it makes it easier to print helpful error messages.
 *
 * If the capsule is successfully submitted to the firmware, any
 * subsequent calls to efi_capsule_pending() will return true. @pages
 * must not be released or modified if this function returns
 * successfully.
 *
 * Callers must be prepared for this function to fail, which can
 * happen if we raced with system reboot or if there is already a
 * pending capsule that has a reset type that conflicts with the one
 * required by @capsule. Do NOT use efi_capsule_pending() to detect
 * this conflict since that would be racy. Instead, submit the capsule
 * to efi_capsule_update() and check the return value.
 *
 * Return 0 on success, a converted EFI status code on failure.
 */
pub unsafe fn efi_capsule_update(
    capsule: *mut efi_capsule_header_t,
    mut pages: *mut phys_addr_t,
) -> i32 {
    let mut imagesize: u32 = (*capsule).imagesize;
    let guid: efi_guid_t = (*capsule).guid;
    let count: usize;
    let sg_count: usize;
    let sg_pages: *mut *mut page;
    let mut rv: i32;
    let mut reset_type: i32 = 0;

    rv = efi_capsule_supported(guid, (*capsule).flags, imagesize as usize, &mut reset_type);
    if rv != 0 {
        return rv;
    }

    count = (imagesize as usize + PAGE_SIZE - 1) / PAGE_SIZE;
    sg_count = sg_pages_num(count);
    sg_pages = kzalloc_objs::<*mut page>(sg_count);
    if sg_pages.is_null() {
        return -ENOMEM;
    }

    for i in 0..sg_count {
        *sg_pages.add(i) = alloc_page(GFP_KERNEL);
        if (*sg_pages.add(i)).is_null() {
            rv = -ENOMEM;
            goto_out(sg_pages, sg_count, rv);
        }
    }

    let mut remaining = count;
    for i in 0..sg_count {
        let sglist = kmap_atomic(*sg_pages.add(i)) as *mut efi_capsule_block_desc_t;
        let mut j = 0;
        while j < SGLIST_PER_PAGE && remaining > 0 {
            let sz = core::cmp::min(imagesize as u64, PAGE_SIZE as u64 - (*pages as u64 % PAGE_SIZE as u64));
            (*sglist.add(j)).length = sz;
            (*sglist.add(j)).data = *pages;
            pages = pages.add(1);
            imagesize = imagesize.wrapping_sub(sz as u32);
            remaining -= 1;
            j += 1;
        }

        /* Continuation pointer */
        (*sglist.add(j)).length = 0;
        (*sglist.add(j)).data = if i + 1 == sg_count {
            0
        } else {
            page_to_phys(*sg_pages.add(i + 1))
        };

        #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
        {
            /*
             * At runtime, the firmware has no way to find out where the
             * sglist elements are mapped, if they are mapped in the first
             * place. Therefore, on architectures that can only perform
             * cache maintenance by virtual address, the firmware is
             * unable to perform this maintenance, and so it is up to the
             * OS to do it instead.
             */
            efi_capsule_flush_cache_range(sglist as *mut _, PAGE_SIZE);
        }

        kunmap_atomic(sglist as *mut _);
    }

    mutex_lock(&mut capsule_mutex);
    rv = efi_capsule_update_locked(capsule, sg_pages, reset_type);
    mutex_unlock(&mut capsule_mutex);

    goto_out(sg_pages, sg_count, rv);
    rv
}

unsafe fn goto_out(sg_pages: *mut *mut page, sg_count: usize, rv: i32) {
    if rv != 0 {
        for i in 0..sg_count {
            if !(*sg_pages.add(i)).is_null() {
                __free_page(*sg_pages.add(i));
            }
        }
    }
    kfree(sg_pages as *mut _);
}

unsafe fn capsule_reboot_notify(_nb: *mut notifier_block, _event: u64, _cmd: *mut core::ffi::c_void) -> i32 {
    mutex_lock(&mut capsule_mutex);
    stop_capsules = true;
    mutex_unlock(&mut capsule_mutex);
    NOTIFY_DONE
}

static mut capsule_reboot_nb: notifier_block = notifier_block {
    notifier_call: Some(capsule_reboot_notify),
};

unsafe fn capsule_reboot_register() -> i32 {
    register_reboot_notifier(&mut capsule_reboot_nb)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
