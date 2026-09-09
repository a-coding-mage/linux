// SPDX-License-Identifier: GPL-2.0-only
/*
 * NVIDIA GHES vendor record handler
 *
 * Copyright (c) 2026, NVIDIA CORPORATION & AFFILIATES. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel Rust bindings.

static nvidia_sec_guid: guid_t = GUID_INIT!(
    0x6d5244f2,
    0x2712,
    0x11ec,
    0xbe,
    0xa7,
    0xcb,
    0x3f,
    0xdb,
    0x95,
    0xc7,
    0x86,
);

#[repr(C)]
struct cper_sec_nvidia {
    signature: [c_char; 16],
    error_type: __le16,
    error_instance: __le16,
    severity: u8,
    socket: u8,
    number_regs: u8,
    reserved: u8,
    instance_base: __le64,
    regs: [cper_sec_nvidia_reg; 0],
}

#[repr(C)]
struct cper_sec_nvidia_reg {
    addr: __le64,
    val: __le64,
}

#[repr(C)]
struct nvidia_ghes_private {
    nb: notifier_block,
    dev: *mut device,
}

unsafe fn nvidia_ghes_print_error(
    dev: *mut device,
    nvidia_err: *const cper_sec_nvidia,
    error_data_length: usize,
    fatal: bool,
) {
    let level = if fatal { KERN_ERR } else { KERN_INFO };
    let min_size: usize;

    dev_printk!(level, dev, "signature: %.16s\n", (*nvidia_err).signature);
    dev_printk!(level, dev, "error_type: %u\n", le16_to_cpu((*nvidia_err).error_type));
    dev_printk!(level, dev, "error_instance: %u\n", le16_to_cpu((*nvidia_err).error_instance));
    dev_printk!(level, dev, "severity: %u\n", (*nvidia_err).severity);
    dev_printk!(level, dev, "socket: %u\n", (*nvidia_err).socket);
    dev_printk!(level, dev, "number_regs: %u\n", (*nvidia_err).number_regs);
    dev_printk!(
        level,
        dev,
        "instance_base: 0x%016llx\n",
        le64_to_cpu((*nvidia_err).instance_base)
    );

    if (*nvidia_err).number_regs == 0 {
        return;
    }

    /*
     * Validate that all registers fit within error_data_length.
     * Each register pair is two little-endian u64s.
     */
    min_size = struct_size!(
        cper_sec_nvidia,
        regs,
        (*nvidia_err).number_regs
    );
    if error_data_length < min_size {
        dev_err!(
            dev,
            "Invalid number_regs %u (section size %zu, need %zu)\n",
            (*nvidia_err).number_regs,
            error_data_length,
            min_size
        );
        return;
    }

    for i in 0..(*nvidia_err).number_regs {
        let reg = (*nvidia_err).regs.as_ptr().add(i as usize);
        dev_printk!(
            level,
            dev,
            "register[%d]: address=0x%016llx value=0x%016llx\n",
            i,
            le64_to_cpu((*reg).addr),
            le64_to_cpu((*reg).val)
        );
    }
}

unsafe extern "C" fn nvidia_ghes_notify(
    nb: *mut notifier_block,
    event: c_ulong,
    data: *mut c_void,
) -> c_int {
    let gdata = data as *mut acpi_hest_generic_data;
    let priv_: *mut nvidia_ghes_private;
    let nvidia_err: *const cper_sec_nvidia;
    let mut sec_guid: guid_t = core::mem::zeroed();

    import_guid(&mut sec_guid, (*gdata).section_type.as_ptr());
    if !guid_equal(&sec_guid, &nvidia_sec_guid) {
        return NOTIFY_DONE;
    }

    priv_ = container_of!(nb, nvidia_ghes_private, nb);

    if acpi_hest_get_error_length(gdata) < core::mem::size_of::<cper_sec_nvidia>() {
        dev_err!(
            (*priv_).dev,
            "Section too small (%d < %zu)\n",
            acpi_hest_get_error_length(gdata),
            core::mem::size_of::<cper_sec_nvidia>()
        );
        return NOTIFY_OK;
    }

    nvidia_err = acpi_hest_get_payload(gdata) as *const cper_sec_nvidia;

    if event >= GHES_SEV_RECOVERABLE {
        dev_err!(
            (*priv_).dev,
            "NVIDIA CPER section, error_data_length: %u\n",
            acpi_hest_get_error_length(gdata)
        );
    } else {
        dev_info!(
            (*priv_).dev,
            "NVIDIA CPER section, error_data_length: %u\n",
            acpi_hest_get_error_length(gdata)
        );
    }

    nvidia_ghes_print_error(
        (*priv_).dev,
        nvidia_err,
        acpi_hest_get_error_length(gdata),
        event >= GHES_SEV_RECOVERABLE,
    );

    NOTIFY_OK
}

unsafe extern "C" fn nvidia_ghes_probe(pdev: *mut platform_device) -> c_int {
    let priv_: *mut nvidia_ghes_private;
    let ret: c_int;

    priv_ = devm_kmalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<nvidia_ghes_private>(),
        GFP_KERNEL,
    ) as *mut nvidia_ghes_private;
    if priv_.is_null() {
        return -ENOMEM;
    }

    *priv_ = nvidia_ghes_private {
        nb: notifier_block {
            notifier_call: Some(nvidia_ghes_notify),
            ..core::mem::zeroed()
        },
        dev: &mut (*pdev).dev,
    };

    ret = devm_ghes_register_vendor_record_notifier(&mut (*pdev).dev, &mut (*priv_).nb);
    if ret != 0 {
        return dev_err_probe(
            &mut (*pdev).dev,
            ret,
            "Failed to register NVIDIA GHES vendor record notifier\n",
        );
    }

    0
}

static nvidia_ghes_acpi_match: [acpi_device_id; 2] = [
    acpi_device_id { name: *b"NVDA2012\0", driver_data: 0 },
    acpi_device_id { name: [0; 9], driver_data: 0 },
];

static mut nvidia_ghes_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"nvidia-ghes\0".as_ptr() as *const c_char,
        acpi_match_table: nvidia_ghes_acpi_match.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(nvidia_ghes_probe),
    ..unsafe { core::mem::zeroed() }
};

module_platform_driver!(nvidia_ghes_driver);

MODULE_AUTHOR!("Kai-Heng Feng <kaihengf@nvidia.com>");
MODULE_DESCRIPTION!("NVIDIA GHES vendor CPER record handler");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
