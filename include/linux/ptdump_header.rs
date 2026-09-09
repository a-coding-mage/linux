/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency supplied by the surrounding kernel translation. */

#[repr(C)]
pub struct ptdump_range {
    pub start: core::ffi::c_ulong,
    pub end: core::ffi::c_ulong,
}

#[repr(C)]
pub struct ptdump_state {
    pub note_page_pte: Option<unsafe extern "C" fn(
        st: *mut ptdump_state,
        addr: core::ffi::c_ulong,
        pte: pte_t,
    )>,
    pub note_page_pmd: Option<unsafe extern "C" fn(
        st: *mut ptdump_state,
        addr: core::ffi::c_ulong,
        pmd: pmd_t,
    )>,
    pub note_page_pud: Option<unsafe extern "C" fn(
        st: *mut ptdump_state,
        addr: core::ffi::c_ulong,
        pud: pud_t,
    )>,
    pub note_page_p4d: Option<unsafe extern "C" fn(
        st: *mut ptdump_state,
        addr: core::ffi::c_ulong,
        p4d: p4d_t,
    )>,
    pub note_page_pgd: Option<unsafe extern "C" fn(
        st: *mut ptdump_state,
        addr: core::ffi::c_ulong,
        pgd: pgd_t,
    )>,
    pub note_page_flush: Option<unsafe extern "C" fn(st: *mut ptdump_state)>,
    pub effective_prot_pte: Option<unsafe extern "C" fn(
        st: *mut ptdump_state,
        pte: pte_t,
    )>,
    pub effective_prot_pmd: Option<unsafe extern "C" fn(
        st: *mut ptdump_state,
        pmd: pmd_t,
    )>,
    pub effective_prot_pud: Option<unsafe extern "C" fn(
        st: *mut ptdump_state,
        pud: pud_t,
    )>,
    pub effective_prot_p4d: Option<unsafe extern "C" fn(
        st: *mut ptdump_state,
        p4d: p4d_t,
    )>,
    pub effective_prot_pgd: Option<unsafe extern "C" fn(
        st: *mut ptdump_state,
        pgd: pgd_t,
    )>,
    pub range: *const ptdump_range,
}

extern "C" {
    pub fn ptdump_walk_pgd_level_core(
        m: *mut seq_file,
        mm: *mut mm_struct,
        pgd: *mut pgd_t,
        checkwx: bool,
        dmesg: bool,
    ) -> bool;
    pub fn ptdump_walk_pgd(st: *mut ptdump_state, mm: *mut mm_struct, pgd: *mut pgd_t);
    pub fn ptdump_check_wx() -> bool;
}

pub unsafe fn debug_checkwx() {
    /* CONFIG_DEBUG_WX is a build-time condition supplied by the surrounding build. */
    if cfg!(feature = "CONFIG_DEBUG_WX") {
        ptdump_check_wx();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
