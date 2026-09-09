/*
 * Common functions shared between the various APIC flavours
 *
 * SPDX-License-Identifier: GPL-2.0
 */

// Dependencies supplied by the surrounding kernel translation unit.
extern "C" {
    static mut nr_cpu_ids: i32;
    fn cpu_present(cpu: i32) -> bool;
    fn smp_processor_id() -> i32;
    fn apic_write(reg: u32, value: u32);
    fn apic_read(reg: u32) -> u32;
    fn per_cpu_x86_cpu_to_apicid(cpu: usize) -> u32;
    fn set_apic_logical_id(value: u32) -> u32;
}

pub unsafe fn apic_default_calc_apicid(cpu: u32) -> u32 {
    per_cpu_x86_cpu_to_apicid(cpu as usize)
}

pub unsafe fn apic_flat_calc_apicid(cpu: u32) -> u32 {
    1u32 << cpu
}

pub unsafe fn default_cpu_present_to_apicid(mps_cpu: i32) -> u32 {
    if mps_cpu < nr_cpu_ids && cpu_present(mps_cpu) {
        per_cpu_x86_cpu_to_apicid(mps_cpu as usize)
    } else {
        BAD_APICID
    }
}

/*
 * Set up the logical destination ID when the APIC operates in logical
 * destination mode.
 */
pub unsafe fn default_init_apic_ldr() {
    let mut val: u32;

    apic_write(APIC_DFR, APIC_DFR_FLAT);
    val = apic_read(APIC_LDR) & !APIC_LDR_MASK;
    val |= set_apic_logical_id(1u32 << (smp_processor_id() as u32));
    apic_write(APIC_LDR, val);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
