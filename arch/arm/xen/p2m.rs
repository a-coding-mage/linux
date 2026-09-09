// SPDX-License-Identifier: GPL-2.0-only
// Dependencies supplied by the Linux and Xen headers are intentionally left external.

#[repr(C)]
pub struct xen_p2m_entry {
    pub pfn: ::core::ffi::c_ulong,
    pub mfn: ::core::ffi::c_ulong,
    pub nr_pages: ::core::ffi::c_ulong,
    pub rbnode_phys: rb_node,
}

static mut p2m_lock: rwlock_t = rwlock_t { _private: [] };
#[no_mangle]
pub static mut phys_to_mach: rb_root = RB_ROOT;

unsafe fn xen_add_phys_to_mach_entry(new: *mut xen_p2m_entry) -> ::core::ffi::c_int {
    let mut link: *mut *mut rb_node = &mut (*phys_to_mach.rb_node);
    let mut parent: *mut rb_node = core::ptr::null_mut();
    let mut entry: *mut xen_p2m_entry;
    let mut rc: ::core::ffi::c_int = 0;

    while !(*link).is_null() {
        parent = *link;
        entry = rb_entry(parent, xen_p2m_entry, rbnode_phys);

        if (*new).pfn == (*entry).pfn {
            rc = -EINVAL;
            pr_warn("%s: cannot add pfn=%pa -> mfn=%pa: pfn=%pa -> mfn=%pa already exists\n",
                __func__, &(*new).pfn, &(*new).mfn, &(*entry).pfn, &(*entry).mfn);
            return rc;
        }

        if (*new).pfn < (*entry).pfn {
            link = &mut (*(*link)).rb_left;
        } else {
            link = &mut (*(*link)).rb_right;
        }
    }
    rb_link_node(&mut (*new).rbnode_phys, parent, link);
    rb_insert_color(&mut (*new).rbnode_phys, &mut phys_to_mach);
    rc
}

#[no_mangle]
pub unsafe extern "C" fn __pfn_to_mfn(pfn: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    let mut n: *mut rb_node;
    let mut entry: *mut xen_p2m_entry;
    let mut irqflags: ::core::ffi::c_ulong = 0;

    read_lock_irqsave(&mut p2m_lock, &mut irqflags);
    n = phys_to_mach.rb_node;
    while !n.is_null() {
        entry = rb_entry(n, xen_p2m_entry, rbnode_phys);
        if (*entry).pfn <= pfn && (*entry).pfn.wrapping_add((*entry).nr_pages) > pfn {
            let mfn = (*entry).mfn.wrapping_add(pfn.wrapping_sub((*entry).pfn));
            read_unlock_irqrestore(&mut p2m_lock, irqflags);
            return mfn;
        }
        if pfn < (*entry).pfn { n = (*n).rb_left; } else { n = (*n).rb_right; }
    }
    read_unlock_irqrestore(&mut p2m_lock, irqflags);
    INVALID_P2M_ENTRY
}

#[no_mangle]
pub unsafe extern "C" fn set_foreign_p2m_mapping(
    map_ops: *mut gnttab_map_grant_ref, _kmap_ops: *mut gnttab_map_grant_ref,
    _pages: *mut *mut page, count: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    for i in 0..count {
        let op = &mut *map_ops.add(i as usize);
        let mut unmap: gnttab_unmap_grant_ref;
        let mut rc: ::core::ffi::c_int;
        if op.status != 0 { continue; }
        if likely(set_phys_to_machine(op.host_addr >> XEN_PAGE_SHIFT, op.dev_bus_addr >> XEN_PAGE_SHIFT)) { continue; }
        op.status = GNTST_general_error;
        unmap.host_addr = op.host_addr;
        unmap.handle = op.handle;
        op.handle = INVALID_GRANT_HANDLE;
        unmap.dev_bus_addr = if op.flags & GNTMAP_device_map != 0 { op.dev_bus_addr } else { 0 };
        unmap.status = 1;
        rc = HYPERVISOR_grant_table_op(GNTTABOP_unmap_grant_ref, &mut unmap, 1);
        if rc != 0 || unmap.status != GNTST_okay { pr_err_once("gnttab unmap failed: rc=%d st=%d\n", rc, unmap.status); }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn clear_foreign_p2m_mapping(
    unmap_ops: *mut gnttab_unmap_grant_ref, _kunmap_ops: *mut gnttab_unmap_grant_ref,
    _pages: *mut *mut page, count: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    for i in 0..count { set_phys_to_machine((*unmap_ops.add(i as usize)).host_addr >> XEN_PAGE_SHIFT, INVALID_P2M_ENTRY); }
    0
}

#[no_mangle]
pub unsafe extern "C" fn __set_phys_to_machine_multi(
    pfn: ::core::ffi::c_ulong, mfn: ::core::ffi::c_ulong, nr_pages: ::core::ffi::c_ulong,
) -> bool {
    let mut irqflags: ::core::ffi::c_ulong = 0;
    if mfn == INVALID_P2M_ENTRY {
        write_lock_irqsave(&mut p2m_lock, &mut irqflags);
        let mut n = phys_to_mach.rb_node;
        while !n.is_null() {
            let entry = rb_entry(n, xen_p2m_entry, rbnode_phys);
            if (*entry).pfn <= pfn && (*entry).pfn.wrapping_add((*entry).nr_pages) > pfn {
                rb_erase(&mut (*entry).rbnode_phys, &mut phys_to_mach);
                write_unlock_irqrestore(&mut p2m_lock, irqflags);
                kfree(entry as *mut core::ffi::c_void);
                return true;
            }
            if pfn < (*entry).pfn { n = (*n).rb_left; } else { n = (*n).rb_right; }
        }
        write_unlock_irqrestore(&mut p2m_lock, irqflags);
        return true;
    }
    let p2m_entry = kzalloc_obj::<xen_p2m_entry>(GFP_NOWAIT);
    if p2m_entry.is_null() { return false; }
    (*p2m_entry).pfn = pfn; (*p2m_entry).nr_pages = nr_pages; (*p2m_entry).mfn = mfn;
    write_lock_irqsave(&mut p2m_lock, &mut irqflags);
    let rc = xen_add_phys_to_mach_entry(p2m_entry);
    if rc < 0 { write_unlock_irqrestore(&mut p2m_lock, irqflags); kfree(p2m_entry as *mut core::ffi::c_void); return false; }
    write_unlock_irqrestore(&mut p2m_lock, irqflags);
    true
}

#[no_mangle]
pub unsafe extern "C" fn __set_phys_to_machine(pfn: ::core::ffi::c_ulong, mfn: ::core::ffi::c_ulong) -> bool {
    __set_phys_to_machine_multi(pfn, mfn, 1)
}

unsafe fn p2m_init() -> ::core::ffi::c_int { rwlock_init(&mut p2m_lock); 0 }
// arch_initcall(p2m_init);


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
