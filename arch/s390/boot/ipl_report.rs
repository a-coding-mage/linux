// SPDX-License-Identifier: GPL-2.0
//
// C dependencies supplied by the surrounding kernel source are intentionally
// left as external Rust names.

extern "C" {
    static mut ipl_secure_flag: ::core::ffi::c_int;
    static mut ipl_cert_list_addr: c_ulong;
    static mut ipl_cert_list_size: c_ulong;
    static mut early_ipl_comp_list_addr: c_ulong;
    static mut early_ipl_comp_list_size: c_ulong;
    static mut ipl_block_valid: bool;
    static mut ipl_block: ipl_block;

    fn get_lowcore() -> *mut lowcore;
    fn intersects(addr: c_ulong, size: c_ulong, other_addr: c_ulong, other_size: c_ulong) -> bool;
    fn physmem_reserve(kind: c_ulong, addr: c_ulong, size: c_ulong);
    fn physmem_alloc_or_die(kind: c_ulong, size: c_ulong, align: c_ulong) -> c_ulong;
    fn physmem_free(kind: c_ulong);
}

type c_ulong = ::core::ffi::c_ulong;

static mut certs: *mut ipl_rb_certificates = core::ptr::null_mut();
static mut comps: *mut ipl_rb_components = core::ptr::null_mut();
static mut ipl_report_needs_saving: bool = false;

unsafe fn get_cert_comp_list_size() -> c_ulong {
    let mut cert: *mut ipl_rb_certificate_entry;
    let mut comp: *mut ipl_rb_component_entry;

    // Find the length for the IPL report boot data
    early_ipl_comp_list_size = 0;
    comp = (*comps).entries;
    while (comp as *mut u8).add(core::mem::size_of::<ipl_rb_component_entry>())
        <= (comps as *mut u8).add((*comps).len as usize)
    {
        early_ipl_comp_list_size = early_ipl_comp_list_size
            .wrapping_add(core::mem::size_of::<ipl_rb_component_entry>() as c_ulong);
        comp = comp.add(1);
    }
    ipl_cert_list_size = 0;
    cert = (*certs).entries;
    while (cert as *mut u8).add(core::mem::size_of::<ipl_rb_certificate_entry>())
        <= (certs as *mut u8).add((*certs).len as usize)
    {
        ipl_cert_list_size = ipl_cert_list_size
            .wrapping_add(core::mem::size_of::<u32>() as c_ulong)
            .wrapping_add((*cert).len as c_ulong);
        cert = cert.add(1);
    }
    ipl_cert_list_size.wrapping_add(early_ipl_comp_list_size)
}

pub unsafe fn ipl_report_certs_intersects(
    addr: c_ulong,
    size: c_ulong,
    intersection_start: *mut c_ulong,
) -> bool {
    if !ipl_report_needs_saving {
        return false;
    }
    let mut cert = (*certs).entries;
    while (cert as *mut u8).add(core::mem::size_of::<ipl_rb_certificate_entry>())
        <= (certs as *mut u8).add((*certs).len as usize)
    {
        if intersects(addr, size, (*cert).addr, (*cert).len) {
            *intersection_start = (*cert).addr;
            return true;
        }
        cert = cert.add(1);
    }
    false
}

unsafe fn copy_components_bootdata() {
    let mut ptr = early_ipl_comp_list_addr as *mut ipl_rb_component_entry;
    let mut comp = (*comps).entries;
    while (comp as *mut u8).add(core::mem::size_of::<ipl_rb_component_entry>())
        <= (comps as *mut u8).add((*comps).len as usize)
    {
        core::ptr::copy_nonoverlapping(comp, ptr, 1);
        ptr = ptr.add(1);
        comp = comp.add(1);
    }
}

unsafe fn copy_certificates_bootdata() {
    let mut ptr = ipl_cert_list_addr as *mut u8;
    let mut cert = (*certs).entries;
    while (cert as *mut u8).add(core::mem::size_of::<ipl_rb_certificate_entry>())
        <= (certs as *mut u8).add((*certs).len as usize)
    {
        *(ptr as *mut u32) = (*cert).len as u32;
        ptr = ptr.add(core::mem::size_of::<u32>());
        core::ptr::copy_nonoverlapping((*cert).addr as *const u8, ptr, (*cert).len as usize);
        ptr = ptr.add((*cert).len as usize);
        cert = cert.add(1);
    }
}

pub unsafe fn read_ipl_report() -> ::core::ffi::c_int {
    let mut tmp: c_ulong;
    if !ipl_block_valid || (ipl_block.hdr.flags & IPL_PL_FLAG_IPLSR) == 0 {
        return -1;
    }
    ipl_secure_flag = ((ipl_block.hdr.flags & IPL_PL_FLAG_SIPL) != 0) as ::core::ffi::c_int;
    tmp = (*get_lowcore()).ipl_parmblock_ptr as c_ulong;
    let pl_hdr = tmp as *mut ipl_pl_hdr;
    tmp = (tmp.wrapping_add((*pl_hdr).len as c_ulong).wrapping_add(7)) & !7;
    let rl_hdr = tmp as *mut ipl_rl_hdr;
    certs = core::ptr::null_mut();
    comps = core::ptr::null_mut();
    let rl_end = (rl_hdr as *mut u8).add((*rl_hdr).len as usize);
    let mut rb_hdr = (rl_hdr as *mut u8).add(core::mem::size_of::<ipl_rl_hdr>()) as *mut ipl_rb_hdr;
    while (rb_hdr as *mut u8).add(core::mem::size_of::<ipl_rb_hdr>()) < rl_end
        && (rb_hdr as *mut u8).add((*rb_hdr).len as usize) <= rl_end
    {
        match (*rb_hdr).rbt {
            IPL_RBT_CERTIFICATES => certs = rb_hdr as *mut ipl_rb_certificates,
            IPL_RBT_COMPONENTS => comps = rb_hdr as *mut ipl_rb_components,
            _ => {}
        }
        rb_hdr = (rb_hdr as *mut u8).add((*rb_hdr).len as usize) as *mut ipl_rb_hdr;
    }
    if certs.is_null() || comps.is_null() {
        certs = core::ptr::null_mut();
        return -1;
    }
    ipl_report_needs_saving = true;
    physmem_reserve(RR_IPLREPORT, pl_hdr as c_ulong, rl_end as c_ulong - pl_hdr as c_ulong);
    0
}

pub unsafe fn save_ipl_cert_comp_list() {
    if !ipl_report_needs_saving {
        return;
    }
    let size = get_cert_comp_list_size();
    early_ipl_comp_list_addr = physmem_alloc_or_die(RR_CERT_COMP_LIST, size, core::mem::size_of::<i32>() as c_ulong);
    ipl_cert_list_addr = early_ipl_comp_list_addr.wrapping_add(early_ipl_comp_list_size);
    copy_components_bootdata();
    copy_certificates_bootdata();
    physmem_free(RR_IPLREPORT);
    ipl_report_needs_saving = false;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
