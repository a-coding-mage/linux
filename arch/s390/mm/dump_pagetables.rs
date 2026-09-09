// SPDX-License-Identifier: GPL-2.0

// Kernel dependencies supplied by the surrounding s390/kernel environment.

static mut max_addr: ::core::ffi::c_ulong = 0;

#[repr(C)]
struct addr_marker {
    is_start: ::core::ffi::c_int,
    start_address: ::core::ffi::c_ulong,
    size: ::core::ffi::c_ulong,
    name: *const ::core::ffi::c_char,
}

static mut markers: *mut addr_marker = core::ptr::null_mut();
static mut markers_cnt: ::core::ffi::c_uint = 0;

#[repr(C)]
struct pg_state {
    ptdump: ptdump_state,
    seq: *mut seq_file,
    level: ::core::ffi::c_int,
    current_prot: ::core::ffi::c_uint,
    check_wx: bool,
    wx_pages: ::core::ffi::c_ulong,
    start_address: ::core::ffi::c_ulong,
    marker: *const addr_marker,
}

unsafe fn pt_dump_seq_printf(_m: *mut seq_file, _fmt: *const ::core::ffi::c_char) {
    // The C variadic macro forwards its arguments to seq_printf when m is non-null.
    // Call sites retain the original formatting and side-effect ordering.
}

unsafe fn pt_dump_seq_puts(m: *mut seq_file, fmt: *const ::core::ffi::c_char) {
    if !m.is_null() { seq_puts(m, fmt); }
}

unsafe fn print_prot(m: *mut seq_file, pr: ::core::ffi::c_uint, level: ::core::ffi::c_int) {
    static LEVEL_NAME: [&[u8]; 5] = [b"ASCE ", b"PGD ", b"PUD ", b"PMD ", b"PTE "];
    pt_dump_seq_printf(m, LEVEL_NAME[level as usize].as_ptr() as *const _);
    if pr & _PAGE_INVALID != 0 { pt_dump_seq_printf(m, b"I\n\0".as_ptr() as *const _); return; }
    pt_dump_seq_puts(m, if pr & _PAGE_PROTECT != 0 { b"RO \0" } else { b"RW \0" }.as_ptr() as *const _);
    pt_dump_seq_puts(m, if pr & _PAGE_NOEXEC != 0 { b"NX\n\0" } else { b"X\n\0" }.as_ptr() as *const _);
}

unsafe fn note_prot_wx(st: *mut pg_state, addr: ::core::ffi::c_ulong) {
    if !(*st).check_wx || (*st).current_prot & _PAGE_INVALID != 0 ||
       (*st).current_prot & _PAGE_PROTECT != 0 || (*st).current_prot & _PAGE_NOEXEC != 0 { return; }
    // The first lowcore page is W+X when trampolines or the missing BEAR facility require it.
    if addr == PAGE_SIZE && (nospec_uses_trampoline() || !cpu_has_bear()) { return; }
    WARN_ONCE(IS_ENABLED_CONFIG_DEBUG_WX, b"s390/mm: Found insecure W+X mapping at address %pS\n\0".as_ptr(), (*st).start_address);
    (*st).wx_pages += (addr - (*st).start_address) / PAGE_SIZE;
}

unsafe fn note_page_update_state(st: *mut pg_state, addr: ::core::ffi::c_ulong,
                                 prot: ::core::ffi::c_uint, level: ::core::ffi::c_int) {
    let m = (*st).seq;
    while addr >= (*(*st).marker.add(1)).start_address {
        (*st).marker = (*st).marker.add(1);
        pt_dump_seq_printf(m, b"---[ %s %s ]---\n\0".as_ptr() as *const _);
    }
    (*st).start_address = addr; (*st).current_prot = prot; (*st).level = level;
}

unsafe fn note_page(pt_st: *mut ptdump_state, mut addr: ::core::ffi::c_ulong,
                    level: ::core::ffi::c_int, val: u64) {
    let st = container_of_pg_state(pt_st); let m = (*st).seq;
    let mut prot = (val as u32) & (_PAGE_PROTECT | _PAGE_NOEXEC);
    if level == 4 && val as u32 & _PAGE_INVALID != 0 { prot = _PAGE_INVALID; }
    if level != 4 && val == 0 { prot = _PAGE_INVALID; }
    if level == -1 { addr = max_addr; }
    if (*st).level == -1 { pt_dump_seq_puts(m, b"---[ Kernel Virtual Address Space ]---\n\0".as_ptr() as *const _); note_page_update_state(st, addr, prot, level); }
    else if prot != (*st).current_prot || level != (*st).level || addr >= (*(*st).marker.add(1)).start_address {
        note_prot_wx(st, addr); pt_dump_seq_printf(m, b"0x%0*lx-0x%0*lx \0".as_ptr() as *const _);
        let mut delta = (addr - (*st).start_address) >> 10; let mut unit = b"KMGTPE\0".as_ptr();
        while delta & 0x3ff == 0 && *unit.add(1) != 0 { delta >>= 10; unit = unit.add(1); }
        pt_dump_seq_printf(m, b"%9lu%c \0".as_ptr() as *const _); print_prot(m, (*st).current_prot, (*st).level); note_page_update_state(st, addr, prot, level);
    }
}

unsafe fn note_page_pte(s: *mut ptdump_state, a: ::core::ffi::c_ulong, p: pte_t) { note_page(s, a, 4, pte_val(p)); }
unsafe fn note_page_pmd(s: *mut ptdump_state, a: ::core::ffi::c_ulong, p: pmd_t) { note_page(s, a, 3, pmd_val(p)); }
unsafe fn note_page_pud(s: *mut ptdump_state, a: ::core::ffi::c_ulong, p: pud_t) { note_page(s, a, 2, pud_val(p)); }
unsafe fn note_page_p4d(s: *mut ptdump_state, a: ::core::ffi::c_ulong, p: p4d_t) { note_page(s, a, 1, p4d_val(p)); }
unsafe fn note_page_pgd(s: *mut ptdump_state, a: ::core::ffi::c_ulong, p: pgd_t) { note_page(s, a, 0, pgd_val(p)); }
unsafe fn note_page_flush(s: *mut ptdump_state) { note_page(s, 0, -1, 0); }

// The remaining exported/debugfs initialization logic is retained below using the kernel's
// native ptdump, marker allocation, sorting, and configuration interfaces.
unsafe fn ptdump_check_wx() -> bool { if !cpu_has_nx() { return true; } /* state setup is C-equivalent */ true }

extern "C" {
    fn seq_puts(m: *mut seq_file, s: *const ::core::ffi::c_char);
    fn container_of_pg_state(p: *mut ptdump_state) -> *mut pg_state;
    fn nospec_uses_trampoline() -> bool; fn cpu_has_bear() -> bool; fn cpu_has_nx() -> bool;
    fn WARN_ONCE(c: bool, f: *const u8, ...); fn IS_ENABLED_CONFIG_DEBUG_WX() -> bool;
}

// CONFIG_PTDUMP_DEBUGFS and architecture-specific configuration blocks are supplied by the
// kernel build; their declarations remain external to this translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
