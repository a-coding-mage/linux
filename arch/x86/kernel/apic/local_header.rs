/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Historical copyright notices:
 *
 * Copyright 2004 James Cleverdon, IBM.
 * (c) 1995 Alan Cox, Building #3 <alan@redhat.com>
 * (c) 1998-99, 2000 Ingo Molnar <mingo@redhat.com>
 * (c) 2002,2003 Andi Kleen, SuSE Labs.
 */

// Dependencies supplied by the surrounding kernel translation.

/* X2APIC */
extern "C" {
    pub fn x2apic_get_apic_id(id: u32) -> u32;

    pub fn x2apic_send_IPI_all(vector: i32);
    pub fn x2apic_send_IPI_allbutself(vector: i32);
    pub fn x2apic_send_IPI_self(vector: i32);
    pub static mut x2apic_max_apicid: u32;

    pub fn default_init_apic_ldr();

    pub fn apic_mem_wait_icr_idle();
    pub fn apic_mem_wait_icr_idle_timeout() -> u32;

    /*
     * This is used to send an IPI with no shorthand notation (the destination is
     * specified in bits 56 to 63 of the ICR).
     */
    pub fn __default_send_IPI_dest_field(mask: u32, vector: i32, dest: u32);

    pub fn default_send_IPI_single(cpu: i32, vector: i32);
    pub fn default_send_IPI_single_phys(cpu: i32, vector: i32);
    pub fn default_send_IPI_mask_sequence_phys(mask: *const crate::cpumask, vector: i32);
    pub fn default_send_IPI_mask_allbutself_phys(mask: *const crate::cpumask, vector: i32);
    pub fn default_send_IPI_allbutself(vector: i32);
    pub fn default_send_IPI_all(vector: i32);
    pub fn default_send_IPI_self(vector: i32);

    #[cfg(CONFIG_X86_32)]
    pub fn default_send_IPI_mask_sequence_logical(mask: *const crate::cpumask, vector: i32);
    #[cfg(CONFIG_X86_32)]
    pub fn default_send_IPI_mask_allbutself_logical(mask: *const crate::cpumask, vector: i32);
    #[cfg(CONFIG_X86_32)]
    pub fn default_send_IPI_mask_logical(mask: *const crate::cpumask, vector: i32);
}

/* IPI */

// Equivalent of DECLARE_STATIC_KEY_FALSE(apic_use_ipi_shorthand).
extern "C" {
    pub static mut apic_use_ipi_shorthand: crate::static_key_false;
}

#[inline]
pub unsafe fn __prepare_ICR(shortcut: u32, vector: i32, dest: u32) -> u32 {
    let mut icr = shortcut | dest;

    match vector {
        crate::NMI_VECTOR => {
            icr |= crate::APIC_DM_NMI;
        }
        _ => {
            icr |= crate::APIC_DM_FIXED | vector as u32;
        }
    }
    icr
}

#[cfg(CONFIG_X86_X2APIC)]
#[inline]
pub unsafe fn __x2apic_send_IPI_dest(apicid: u32, vector: i32, dest: u32) {
    crate::native_x2apic_icr_write(__prepare_ICR(0, vector, dest), apicid);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
