// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2012 IBM Corporation
 *
 * Author: Ashley Lai <ashleydlai@gmail.com>
 *         Nayna Jain <nayna@linux.vnet.ibm.com>
 *
 * Maintained by: <tpmdd-devel@lists.sourceforge.net>
 *
 * Read the event log created by the firmware on PPC64
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

unsafe fn tpm_read_log_memory_region(chip: *mut tpm_chip) -> i32 {
    let mut res: resource = core::mem::zeroed();
    let rc: i32;

    rc = of_reserved_mem_region_to_resource((*(*chip).dev.parent).of_node, 0, &mut res);
    if rc != 0 {
        return rc;
    }

    (*chip).log.bios_event_log = devm_memremap(
        &mut (*chip).dev,
        res.start,
        resource_size(&res),
        MEMREMAP_WB,
    );
    if IS_ERR((*chip).log.bios_event_log) {
        return -ENOMEM;
    }

    (*chip).log.bios_event_log_end =
        (*chip).log.bios_event_log.add(resource_size(&res) as usize);

    if (*chip).flags & TPM_CHIP_FLAG_TPM2 != 0 {
        EFI_TCG2_EVENT_LOG_FORMAT_TCG_2
    } else {
        EFI_TCG2_EVENT_LOG_FORMAT_TCG_1_2
    }
}

pub unsafe fn tpm_read_log_of(chip: *mut tpm_chip) -> i32 {
    let np: *mut device_node;
    let sizep: *const u32;
    let basep: *const u64;
    let log: *mut tpm_bios_log;
    let mut size: u32;
    let mut base: u64;

    log = &mut (*chip).log;
    if !(*chip).dev.parent.is_null() && !(*(*chip).dev.parent).of_node.is_null() {
        np = (*(*chip).dev.parent).of_node;
    } else {
        return -ENODEV;
    }

    if of_property_read_bool(np, c"powered-while-suspended") {
        (*chip).flags |= TPM_CHIP_FLAG_ALWAYS_POWERED;
    }

    sizep = of_get_property(np, c"linux,sml-size", core::ptr::null_mut());
    basep = of_get_property(np, c"linux,sml-base", core::ptr::null_mut());
    if sizep.is_null() && basep.is_null() {
        return tpm_read_log_memory_region(chip);
    }
    if sizep.is_null() || basep.is_null() {
        return -EIO;
    }

    /*
     * For both vtpm/tpm, firmware has log addr and log size in big
     * endian format. But in case of vtpm, there is a method called
     * sml-handover which is run during kernel init even before
     * device tree is setup. This sml-handover function takes care
     * of endianness and writes to sml-base and sml-size in little
     * endian format. For this reason, vtpm doesn't need conversion
     * but physical tpm needs the conversion.
     */
    if of_property_match_string(np, c"compatible", c"IBM,vtpm") < 0
        && of_property_match_string(np, c"compatible", c"IBM,vtpm20") < 0
    {
        size = be32_to_cpup(sizep);
        base = be64_to_cpup(basep);
    } else {
        size = *sizep;
        base = *basep;
    }

    if size == 0 {
        dev_warn(&mut (*chip).dev, c"%s: Event log area empty\n", c"tpm_read_log_of");
        return -EIO;
    }

    (*log).bios_event_log =
        devm_kmemdup(&mut (*chip).dev, __va(base), size as usize, GFP_KERNEL);
    if (*log).bios_event_log.is_null() {
        return -ENOMEM;
    }

    (*log).bios_event_log_end = (*log).bios_event_log.add(size as usize);

    if (*chip).flags & TPM_CHIP_FLAG_TPM2 != 0 {
        return EFI_TCG2_EVENT_LOG_FORMAT_TCG_2;
    }
    EFI_TCG2_EVENT_LOG_FORMAT_TCG_1_2
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
