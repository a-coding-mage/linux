// SPDX-License-Identifier: GPL-2.0-only
// pr_fmt(fmt) = "APIC: " fmt

// The APIC callback static calls are supplied by the kernel static-call
// machinery.  DEFINE_STATIC_CALL_NULL() leaves each callback initially null;
// callbacks are installed during boot.
//
// DEFINE_APIC_CALL(eoi);
// DEFINE_APIC_CALL(native_eoi);
// DEFINE_APIC_CALL(icr_read);
// DEFINE_APIC_CALL(icr_write);
// DEFINE_APIC_CALL(read);
// DEFINE_APIC_CALL(send_IPI);
// DEFINE_APIC_CALL(send_IPI_mask);
// DEFINE_APIC_CALL(send_IPI_mask_allbutself);
// DEFINE_APIC_CALL(send_IPI_allbutself);
// DEFINE_APIC_CALL(send_IPI_self);
// DEFINE_APIC_CALL(send_IPI_all);
// DEFINE_APIC_CALL(wait_icr_idle);
// DEFINE_APIC_CALL(wakeup_secondary_cpu);
// DEFINE_APIC_CALL(wakeup_secondary_cpu_64);
// DEFINE_APIC_CALL(write);

// EXPORT_STATIC_CALL_TRAMP_GPL(apic_call_send_IPI_mask);
// EXPORT_STATIC_CALL_TRAMP_GPL(apic_call_send_IPI_self);

/// The container for function call overrides.
// `struct apic_override __x86_apic_override __initdata;`
static mut __x86_apic_override: apic_override = unsafe { core::mem::zeroed() };

#[inline]
unsafe fn restore_override_callbacks() {
    if __x86_apic_override.eoi.is_some() {
        (*apic).eoi = __x86_apic_override.eoi;
    }
    if __x86_apic_override.native_eoi.is_some() {
        (*apic).native_eoi = __x86_apic_override.native_eoi;
    }
    if __x86_apic_override.write.is_some() {
        (*apic).write = __x86_apic_override.write;
    }
    if __x86_apic_override.read.is_some() {
        (*apic).read = __x86_apic_override.read;
    }
    if __x86_apic_override.send_IPI.is_some() {
        (*apic).send_IPI = __x86_apic_override.send_IPI;
    }
    if __x86_apic_override.send_IPI_mask.is_some() {
        (*apic).send_IPI_mask = __x86_apic_override.send_IPI_mask;
    }
    if __x86_apic_override.send_IPI_mask_allbutself.is_some() {
        (*apic).send_IPI_mask_allbutself = __x86_apic_override.send_IPI_mask_allbutself;
    }
    if __x86_apic_override.send_IPI_allbutself.is_some() {
        (*apic).send_IPI_allbutself = __x86_apic_override.send_IPI_allbutself;
    }
    if __x86_apic_override.send_IPI_all.is_some() {
        (*apic).send_IPI_all = __x86_apic_override.send_IPI_all;
    }
    if __x86_apic_override.send_IPI_self.is_some() {
        (*apic).send_IPI_self = __x86_apic_override.send_IPI_self;
    }
    if __x86_apic_override.icr_read.is_some() {
        (*apic).icr_read = __x86_apic_override.icr_read;
    }
    if __x86_apic_override.icr_write.is_some() {
        (*apic).icr_write = __x86_apic_override.icr_write;
    }
    if __x86_apic_override.wakeup_secondary_cpu.is_some() {
        (*apic).wakeup_secondary_cpu = __x86_apic_override.wakeup_secondary_cpu;
    }
    if __x86_apic_override.wakeup_secondary_cpu_64.is_some() {
        (*apic).wakeup_secondary_cpu_64 = __x86_apic_override.wakeup_secondary_cpu_64;
    }
}

unsafe fn update_static_calls() {
    static_call_update!(apic_call_eoi, (*apic).eoi);
    static_call_update!(apic_call_native_eoi, (*apic).native_eoi);
    static_call_update!(apic_call_write, (*apic).write);
    static_call_update!(apic_call_read, (*apic).read);
    static_call_update!(apic_call_send_IPI, (*apic).send_IPI);
    static_call_update!(apic_call_send_IPI_mask, (*apic).send_IPI_mask);
    static_call_update!(apic_call_send_IPI_mask_allbutself, (*apic).send_IPI_mask_allbutself);
    static_call_update!(apic_call_send_IPI_allbutself, (*apic).send_IPI_allbutself);
    static_call_update!(apic_call_send_IPI_all, (*apic).send_IPI_all);
    static_call_update!(apic_call_send_IPI_self, (*apic).send_IPI_self);
    static_call_update!(apic_call_icr_read, (*apic).icr_read);
    static_call_update!(apic_call_icr_write, (*apic).icr_write);
    static_call_update!(apic_call_wait_icr_idle, (*apic).wait_icr_idle);
    static_call_update!(apic_call_wakeup_secondary_cpu, (*apic).wakeup_secondary_cpu);
    static_call_update!(apic_call_wakeup_secondary_cpu_64, (*apic).wakeup_secondary_cpu_64);
}

pub unsafe fn apic_setup_apic_calls() {
    // Ensure that the default APIC has native_eoi populated.
    (*apic).native_eoi = (*apic).eoi;
    update_static_calls();
    pr_info!("Static calls initialized\n");
}

pub unsafe fn apic_install_driver(driver: *mut apic) {
    if apic == driver {
        return;
    }

    apic = driver;

    if cfg!(feature = "CONFIG_X86_X2APIC") && (*apic).x2apic_set_max_apicid.is_some() {
        (*apic).max_apic_id = x2apic_max_apicid;
    }

    // Copy the original eoi() callback as KVM/HyperV might overwrite it.
    if (*apic).native_eoi.is_none() {
        (*apic).native_eoi = (*apic).eoi;
    }

    // Apply any already installed callback overrides.
    restore_override_callbacks();
    update_static_calls();

    pr_info!("Switched APIC routing to: %s\n", (*driver).name);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
