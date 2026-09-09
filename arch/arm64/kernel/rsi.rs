// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2023 ARM Ltd.
 */

// Linux and architecture-specific declarations are supplied by the surrounding
// translation unit.

static mut config: realm_config = realm_config {
    // The layout is supplied by the external RSI definitions.
};

pub static mut prot_ns_shared: c_ulong = 0;

// DEFINE_STATIC_KEY_FALSE_RO(rsi_present);
pub static rsi_present: static_key_false = static_key_false {};

unsafe fn rsi_version_matches() -> bool {
    let mut ver_lower: c_ulong;
    let mut ver_higher: c_ulong;
    let ret: c_ulong = rsi_request_version(
        RSI_ABI_VERSION,
        &mut ver_lower,
        &mut ver_higher,
    );

    if ret == SMCCC_RET_NOT_SUPPORTED {
        return false;
    }

    if ret != RSI_SUCCESS {
        pr_err!(
            "RME: RMM doesn't support RSI version %lu.%lu. Supported range: %lu.%lu-%lu.%lu\n",
            RSI_ABI_VERSION_MAJOR,
            RSI_ABI_VERSION_MINOR,
            RSI_ABI_VERSION_GET_MAJOR(ver_lower),
            RSI_ABI_VERSION_GET_MINOR(ver_lower),
            RSI_ABI_VERSION_GET_MAJOR(ver_higher),
            RSI_ABI_VERSION_GET_MINOR(ver_higher),
        );
        return false;
    }

    pr_info!(
        "RME: Using RSI version %lu.%lu\n",
        RSI_ABI_VERSION_GET_MAJOR(ver_lower),
        RSI_ABI_VERSION_GET_MINOR(ver_lower),
    );

    true
}

unsafe fn arm64_rsi_setup_memory() {
    let mut i: u64;
    let mut start: phys_addr_t;
    let mut end: phys_addr_t;

    /*
     * Iterate over the available memory ranges and convert the state to
     * protected memory. We should take extra care to ensure that we DO NOT
     * permit any "DESTROYED" pages to be converted to "RAM".
     *
     * panic() is used because if the attempt to switch the memory to
     * protected has failed here, then future accesses to the memory are
     * simply going to be reflected as a SEA (Synchronous External Abort)
     * which we can't handle.  Bailing out early prevents the guest limping
     * on and dying later.
     */
    for_each_mem_range!(i, &mut start, &mut end) {
        if rsi_set_memory_range_protected_safe(start, end) != 0 {
            panic!(
                "Failed to set memory range to protected: %pa-%pa",
                &start,
                &end
            );
        }
    }
}

/*
 * Check if a given PA range is Trusted (e.g., Protected memory, a Trusted Device
 * mapping, or an MMIO emulated in the Realm world).
 *
 * We can rely on the RIPAS value of the region to detect if a given region is
 * protected.
 *
 *  RIPAS_DEV - A trusted device memory or a trusted emulated MMIO (in the Realm
 *\t world
 *  RIPAS_RAM - Memory (RAM), protected by the RMM guarantees. (e.g., Firmware
 *\t reserved regions for data sharing).
 *
 *  RIPAS_DESTROYED is a special case of one of the above, where the host did
 *  something without our permission and as such we can't do anything about it.
 *
 * The only case where something is emulated by the untrusted hypervisor or is
 * backed by shared memory is indicated by RSI_RIPAS_EMPTY.
 */
#[no_mangle]
pub unsafe extern "C" fn arm64_rsi_is_protected(
    mut base: phys_addr_t,
    size: size_t,
) -> bool {
    let mut ripas: ripas;
    let mut end: phys_addr_t;
    let mut top: phys_addr_t;

    /* Overflow ? */
    if WARN_ON!(base.wrapping_add(size) <= base) {
        return false;
    }

    end = ALIGN!(base.wrapping_add(size), RSI_GRANULE_SIZE);
    base = ALIGN_DOWN!(base, RSI_GRANULE_SIZE);

    while base < end {
        if WARN_ON!(rsi_ipa_state_get(base, end, &mut ripas, &mut top)) {
            break;
        }
        if WARN_ON!(top <= base) {
            break;
        }
        if ripas == RSI_RIPAS_EMPTY {
            break;
        }
        base = top;
    }

    base >= end
}

unsafe fn realm_ioremap_hook(
    phys: phys_addr_t,
    size: size_t,
    prot: *mut pgprot_t,
) -> c_int {
    if arm64_rsi_is_protected(phys, size) {
        *prot = pgprot_encrypted(*prot);
    } else {
        *prot = pgprot_decrypted(*prot);
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn arm64_rsi_init() {
    if arm_smccc_1_1_get_conduit() != SMCCC_CONDUIT_SMC {
        return;
    }
    if !rsi_version_matches() {
        return;
    }
    if WARN_ON!(rsi_get_realm_config(lm_alias!(&mut config))) {
        return;
    }
    prot_ns_shared = __phys_to_pte_val(BIT!(config.ipa_bits - 1));

    if arm64_ioremap_prot_hook_register(realm_ioremap_hook) != 0 {
        return;
    }

    if realm_register_memory_enc_ops() != 0 {
        return;
    }

    arm64_rsi_setup_memory();

    static_branch_enable!(&rsi_present);
}

static mut rsi_dev: platform_device = platform_device {
    name: RSI_PDEV_NAME,
    id: PLATFORM_DEVID_NONE,
};

unsafe fn arm64_create_dummy_rsi_dev() -> c_int {
    if is_realm_world() && platform_device_register(&mut rsi_dev) != 0 {
        pr_err!("failed to register rsi platform device\n");
    }
    0
}

// arch_initcall(arm64_create_dummy_rsi_dev)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
