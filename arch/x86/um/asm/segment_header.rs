/* SPDX-License-Identifier: GPL-2.0 */

// C header guard __UM_SEGMENT_H omitted; Rust items are naturally scoped by
// the including module.

unsafe extern "C" {
    pub static mut host_gdt_entry_tls_min: ::core::ffi::c_int;
}

pub const GDT_ENTRY_TLS_ENTRIES: ::core::ffi::c_int = 3;

// C macro: GDT_ENTRY_TLS_MIN host_gdt_entry_tls_min
#[macro_export]
macro_rules! GDT_ENTRY_TLS_MIN {
    () => {
        unsafe { $crate::host_gdt_entry_tls_min }
    };
}

// C macro: (GDT_ENTRY_TLS_MIN + GDT_ENTRY_TLS_ENTRIES - 1)
#[macro_export]
macro_rules! GDT_ENTRY_TLS_MAX {
    () => {
        ($crate::GDT_ENTRY_TLS_MIN!() + $crate::GDT_ENTRY_TLS_ENTRIES - 1)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
