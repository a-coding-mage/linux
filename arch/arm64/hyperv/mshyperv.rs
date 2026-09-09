// SPDX-License-Identifier: GPL-2.0

/*
 * Core routines for interacting with Microsoft's Hyper-V hypervisor,
 * including hypervisor initialization.
 *
 * Copyright (C) 2021, Microsoft, Inc.
 *
 * Author : Michael Kelley <mikelley@microsoft.com>
 */

// Dependencies supplied by the surrounding kernel translation unit.

static mut hyperv_initialized: bool = false;

pub unsafe fn hv_get_hypervisor_version(info: *mut hv_hypervisor_version_info) -> i32 {
    hv_get_vpreg_128(
        HV_REGISTER_HYPERVISOR_VERSION,
        info as *mut hv_get_vp_registers_output,
    );

    0
}

#[cfg(feature = "CONFIG_ACPI")]
unsafe fn hyperv_detect_via_acpi() -> bool {
    if acpi_disabled {
        return false;
    }
    /*
     * Hypervisor ID is only available in ACPI v6+, and the
     * structure layout was extended in v6 to accommodate that
     * new field.
     *
     * At the very minimum, this check makes sure not to read
     * past the FADT structure.
     *
     * It is also needed to catch running in some unknown
     * non-Hyper-V environment that has ACPI 5.x or less.
     * In such a case, it can't be Hyper-V.
     */
    if acpi_gbl_FADT.header.revision < 6 {
        return false;
    }
    strncmp(
        (&acpi_gbl_FADT.hypervisor_id as *const _).cast::<i8>(),
        b"MsHyperV\0".as_ptr().cast::<i8>(),
        8,
    ) == 0
}

#[cfg(not(feature = "CONFIG_ACPI"))]
unsafe fn hyperv_detect_via_acpi() -> bool {
    false
}

unsafe fn hyperv_detect_via_smccc() -> bool {
    let hyperv_uuid = UUID_INIT(
        0x58ba324d,
        0x6447,
        0x24cd,
        0x75,
        0x6c,
        0xef,
        0x8e,
        0x24,
        0x70,
        0x59,
        0x16,
    );

    arm_smccc_hypervisor_has_uuid(&hyperv_uuid)
}

unsafe fn hyperv_init() -> i32 {
    let mut result: hv_get_vp_registers_output = core::mem::zeroed();
    let guest_id: u64;
    let mut ret: i32;

    /*
     * Allow for a kernel built with CONFIG_HYPERV to be running in
     * a non-Hyper-V environment.
     *
     * In such cases, do nothing and return success.
     */
    if !hyperv_detect_via_acpi() && !hyperv_detect_via_smccc() {
        return 0;
    }

    /* Setup the guest ID */
    guest_id = hv_generate_guest_id(LINUX_VERSION_CODE);
    hv_set_vpreg(HV_REGISTER_GUEST_OS_ID, guest_id);

    /* Get the features and hints from Hyper-V */
    hv_get_vpreg_128(HV_REGISTER_PRIVILEGES_AND_FEATURES_INFO, &mut result);
    ms_hyperv.features = result.as32.a;
    ms_hyperv.priv_high = result.as32.b;
    ms_hyperv.misc_features = result.as32.c;

    hv_get_vpreg_128(HV_REGISTER_FEATURES_INFO, &mut result);
    ms_hyperv.hints = result.as32.a;

    pr_info!(
        "Hyper-V: privilege flags low 0x%x, high 0x%x, hints 0x%x, misc 0x%x\n",
        ms_hyperv.features,
        ms_hyperv.priv_high,
        ms_hyperv.hints,
        ms_hyperv.misc_features
    );

    hv_identify_partition_type();

    ret = hv_common_init();
    if ret != 0 {
        return ret;
    }

    ret = cpuhp_setup_state(
        CPUHP_AP_HYPERV_ONLINE,
        b"arm64/hyperv_init:online\0".as_ptr().cast(),
        hv_common_cpu_init,
        hv_common_cpu_die,
    );
    if ret < 0 {
        hv_common_free();
        return ret;
    }

    if ms_hyperv.priv_high & HV_ACCESS_PARTITION_ID != 0 {
        hv_get_partition_id();
    }
    ms_hyperv.vtl = get_vtl();
    if ms_hyperv.vtl > 0 { /* non default VTL */
        pr_info!(
            "Linux runs in Hyper-V Virtual Trust Level %d\n",
            ms_hyperv.vtl
        );
    }

    ms_hyperv_late_init();

    hyperv_initialized = true;
    0
}

// Corresponds to early_initcall(hyperv_init).

pub unsafe fn hv_is_hyperv_initialized() -> bool {
    hyperv_initialized
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
