// SPDX-License-Identifier: GPL-2.0
/*
 * NOOP APIC driver.
 *
 * Does almost nothing and should be substituted by a real apic driver via
 * probe routine.
 *
 * Though in case if apic is disabled (for some reason) we try
 * to not uglify the caller's code and allow to call (some) apic routines
 * like self-ipi, etc...
 *
 * FIXME: Remove this gunk. The above argument which was intentionally left
 * in place is silly to begin with because none of the callbacks except for
 * APIC::read/write() have a WARN_ON_ONCE() in them. Sigh...
 */

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    static apic_is_disabled: bool;
    fn boot_cpu_has(feature: u32) -> bool;
    fn WARN_ON_ONCE(condition: bool);
    fn default_cpu_present_to_apicid(cpu: u32) -> u32;
    fn apic_flat_calc_apicid(cpu: u32) -> u32;
}

static unsafe fn noop_send_IPI(_cpu: i32, _vector: i32) {}

static unsafe fn noop_send_IPI_mask(_cpumask: *const cpumask, _vector: i32) {}

static unsafe fn noop_send_IPI_mask_allbutself(
    _cpumask: *const cpumask,
    _vector: i32,
) {
}

static unsafe fn noop_send_IPI_allbutself(_vector: i32) {}

static unsafe fn noop_send_IPI_all(_vector: i32) {}

static unsafe fn noop_send_IPI_self(_vector: i32) {}

static unsafe fn noop_apic_icr_write(_low: u32, _id: u32) {}

static unsafe fn noop_wakeup_secondary_cpu(
    _apicid: u32,
    _start_eip: usize,
    _cpu: u32,
) -> i32 {
    -1
}

static unsafe fn noop_apic_icr_read() -> u64 {
    0
}

static unsafe fn noop_get_apic_id(_apicid: u32) -> u32 {
    0
}

static unsafe fn noop_apic_eoi() {}

static unsafe fn noop_apic_read(_reg: u32) -> u32 {
    WARN_ON_ONCE(boot_cpu_has(X86_FEATURE_APIC) && !apic_is_disabled);
    0
}

static unsafe fn noop_apic_write(_reg: u32, _val: u32) {
    WARN_ON_ONCE(boot_cpu_has(X86_FEATURE_APIC) && !apic_is_disabled);
}

#[no_mangle]
pub static mut apic_noop: apic = apic {
    name: "noop\0".as_ptr() as *const i8,

    dest_mode_logical: true,

    disable_esr: 0,

    cpu_present_to_apicid: default_cpu_present_to_apicid,

    max_apic_id: 0xFE,
    get_apic_id: noop_get_apic_id,

    calc_dest_apicid: apic_flat_calc_apicid,

    send_IPI: noop_send_IPI,
    send_IPI_mask: noop_send_IPI_mask,
    send_IPI_mask_allbutself: noop_send_IPI_mask_allbutself,
    send_IPI_allbutself: noop_send_IPI_allbutself,
    send_IPI_all: noop_send_IPI_all,
    send_IPI_self: noop_send_IPI_self,

    wakeup_secondary_cpu: noop_wakeup_secondary_cpu,

    read: noop_apic_read,
    write: noop_apic_write,
    eoi: noop_apic_eoi,
    icr_read: noop_apic_icr_read,
    icr_write: noop_apic_icr_write,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
