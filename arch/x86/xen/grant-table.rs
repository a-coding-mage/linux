// SPDX-License-Identifier: GPL-2.0 OR MIT
/*****************************************************************************
 * grant_table.c
 * x86 specific part
 *
 * Granting foreign access to our memory reservation.
 *
 * Copyright (c) 2005-2006, Christopher Clark
 * Copyright (c) 2004-2005, K A Fraser
 * Copyright (c) 2008 Isaku Yamahata <yamahata at valinux co jp>
 *                    VA Linux Systems Japan. Split out x86 specific part.
 */

// Linux/Xen headers are external dependencies of this translation unit.

#[repr(C)]
struct GnttabVmArea {
    area: *mut VmStruct,
    ptes: *mut *mut PteT,
    idx: libc::c_int,
}

static mut GNTTAB_SHARED_VM_AREA: GnttabVmArea = GnttabVmArea {
    area: core::ptr::null_mut(),
    ptes: core::ptr::null_mut(),
    idx: 0,
};
static mut GNTTAB_STATUS_VM_AREA: GnttabVmArea = GnttabVmArea {
    area: core::ptr::null_mut(),
    ptes: core::ptr::null_mut(),
    idx: 0,
};

pub unsafe fn arch_gnttab_map_shared(
    frames: *mut libc::c_ulong,
    nr_gframes: libc::c_ulong,
    _max_nr_gframes: libc::c_ulong,
    shared: *mut *mut libc::c_void,
) -> libc::c_int {
    let mut shared_value = *shared;
    let mut addr: libc::c_ulong;
    let mut i: libc::c_ulong;

    if shared_value.is_null() {
        (*shared).write((*GNTTAB_SHARED_VM_AREA.area).addr as *mut libc::c_void);
        shared_value = *shared;
    }

    addr = shared_value as libc::c_ulong;
    i = 0;
    while i < nr_gframes {
        set_pte_at(
            &mut init_mm,
            addr,
            *GNTTAB_SHARED_VM_AREA.ptes.add(i as usize),
            mfn_pte(*frames.add(i as usize), PAGE_KERNEL),
        );
        addr = addr.wrapping_add(PAGE_SIZE);
        i += 1;
    }

    0
}

pub unsafe fn arch_gnttab_map_status(
    frames: *mut u64,
    nr_gframes: libc::c_ulong,
    _max_nr_gframes: libc::c_ulong,
    shared: *mut *mut GrantStatusT,
) -> libc::c_int {
    let mut shared_value = *shared;
    let mut addr: libc::c_ulong;
    let mut i: libc::c_ulong;

    if shared_value.is_null() {
        (*shared).write((*GNTTAB_STATUS_VM_AREA.area).addr as *mut GrantStatusT);
        shared_value = *shared;
    }

    addr = shared_value as libc::c_ulong;
    i = 0;
    while i < nr_gframes {
        set_pte_at(
            &mut init_mm,
            addr,
            *GNTTAB_STATUS_VM_AREA.ptes.add(i as usize),
            mfn_pte(*frames.add(i as usize), PAGE_KERNEL),
        );
        addr = addr.wrapping_add(PAGE_SIZE);
        i += 1;
    }

    0
}

pub unsafe fn arch_gnttab_unmap(shared: *mut libc::c_void, nr_gframes: libc::c_ulong) {
    let ptes: *mut *mut PteT;
    let mut addr: libc::c_ulong;
    let mut i: libc::c_ulong;

    if shared == (*GNTTAB_STATUS_VM_AREA.area).addr as *mut libc::c_void {
        ptes = GNTTAB_STATUS_VM_AREA.ptes;
    } else {
        ptes = GNTTAB_SHARED_VM_AREA.ptes;
    }

    addr = shared as libc::c_ulong;
    i = 0;
    while i < nr_gframes {
        set_pte_at(&mut init_mm, addr, *ptes.add(i as usize), __pte(0));
        addr = addr.wrapping_add(PAGE_SIZE);
        i += 1;
    }
}

unsafe fn gnttab_apply(pte: *mut PteT, _addr: libc::c_ulong, data: *mut libc::c_void) -> libc::c_int {
    let area = &mut *(data as *mut GnttabVmArea);
    *area.ptes.add(area.idx as usize) = pte;
    area.idx += 1;
    0
}

unsafe fn arch_gnttab_valloc(area: *mut GnttabVmArea, nr_frames: libc::c_uint) -> libc::c_int {
    (*area).ptes = kmalloc_objs((*area).ptes, nr_frames);
    if (*area).ptes.is_null() {
        return -ENOMEM;
    }
    (*area).area = get_vm_area(PAGE_SIZE * nr_frames as libc::c_ulong, VM_IOREMAP);
    if (*area).area.is_null() {
        kfree((*area).ptes);
        return -ENOMEM;
    }
    if apply_to_page_range(
        &mut init_mm,
        (*(*area).area).addr as libc::c_ulong,
        PAGE_SIZE * nr_frames as libc::c_ulong,
        gnttab_apply,
        area as *mut libc::c_void,
    ) != 0 {
        free_vm_area((*area).area);
        kfree((*area).ptes);
        return -ENOMEM;
    }
    0
}

unsafe fn arch_gnttab_vfree(area: *mut GnttabVmArea) {
    free_vm_area((*area).area);
    kfree((*area).ptes);
}

pub unsafe fn arch_gnttab_init(nr_shared: libc::c_ulong, nr_status: libc::c_ulong) -> libc::c_int {
    let mut ret: libc::c_int;

    if !xen_pv_domain() {
        return 0;
    }

    ret = arch_gnttab_valloc(&mut GNTTAB_SHARED_VM_AREA, nr_shared as libc::c_uint);
    if ret < 0 {
        return ret;
    }

    /*
     * Always allocate the space for the status frames in case
     * we're migrated to a host with V2 support.
     */
    ret = arch_gnttab_valloc(&mut GNTTAB_STATUS_VM_AREA, nr_status as libc::c_uint);
    if ret < 0 {
        arch_gnttab_vfree(&mut GNTTAB_SHARED_VM_AREA);
        return -ENOMEM;
    }

    0
}

// The following block corresponds to CONFIG_XEN_PVH.
#[cfg(feature = "CONFIG_XEN_PVH")]
unsafe fn xen_pvh_gnttab_setup() -> libc::c_int {
    if !xen_pvh_domain() {
        return -ENODEV;
    }

    xen_auto_xlat_grant_frames.count = gnttab_max_grant_frames();

    xen_xlate_map_ballooned_pages(
        &mut xen_auto_xlat_grant_frames.pfn,
        &mut xen_auto_xlat_grant_frames.vaddr,
        xen_auto_xlat_grant_frames.count,
    )
}

// core_initcall(xen_pvh_gnttab_setup): call before __gnttab_init because
// xen_auto_xlat_grant_frames must be initialized first.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
