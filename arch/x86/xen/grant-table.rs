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
    idx: kernel::ffi::c_int,
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
    frames: *mut kernel::ffi::c_ulong,
    nr_gframes: kernel::ffi::c_ulong,
    _max_nr_gframes: kernel::ffi::c_ulong,
    shared: *mut *mut kernel::ffi::c_void,
) -> kernel::ffi::c_int {
    let mut shared_value = *shared;
    let mut addr: kernel::ffi::c_ulong;
    let mut i: kernel::ffi::c_ulong;

    if shared_value.is_null() {
        (*shared).write((*GNTTAB_SHARED_VM_AREA.area).addr as *mut kernel::ffi::c_void);
        shared_value = *shared;
    }

    addr = shared_value as kernel::ffi::c_ulong;
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
    nr_gframes: kernel::ffi::c_ulong,
    _max_nr_gframes: kernel::ffi::c_ulong,
    shared: *mut *mut GrantStatusT,
) -> kernel::ffi::c_int {
    let mut shared_value = *shared;
    let mut addr: kernel::ffi::c_ulong;
    let mut i: kernel::ffi::c_ulong;

    if shared_value.is_null() {
        (*shared).write((*GNTTAB_STATUS_VM_AREA.area).addr as *mut GrantStatusT);
        shared_value = *shared;
    }

    addr = shared_value as kernel::ffi::c_ulong;
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

pub unsafe fn arch_gnttab_unmap(shared: *mut kernel::ffi::c_void, nr_gframes: kernel::ffi::c_ulong) {
    let ptes: *mut *mut PteT;
    let mut addr: kernel::ffi::c_ulong;
    let mut i: kernel::ffi::c_ulong;

    if shared == (*GNTTAB_STATUS_VM_AREA.area).addr as *mut kernel::ffi::c_void {
        ptes = GNTTAB_STATUS_VM_AREA.ptes;
    } else {
        ptes = GNTTAB_SHARED_VM_AREA.ptes;
    }

    addr = shared as kernel::ffi::c_ulong;
    i = 0;
    while i < nr_gframes {
        set_pte_at(&mut init_mm, addr, *ptes.add(i as usize), __pte(0));
        addr = addr.wrapping_add(PAGE_SIZE);
        i += 1;
    }
}

unsafe fn gnttab_apply(pte: *mut PteT, _addr: kernel::ffi::c_ulong, data: *mut kernel::ffi::c_void) -> kernel::ffi::c_int {
    let area = &mut *(data as *mut GnttabVmArea);
    *area.ptes.add(area.idx as usize) = pte;
    area.idx += 1;
    0
}

unsafe fn arch_gnttab_valloc(area: *mut GnttabVmArea, nr_frames: kernel::ffi::c_uint) -> kernel::ffi::c_int {
    (*area).ptes = kmalloc_objs((*area).ptes, nr_frames);
    if (*area).ptes.is_null() {
        return -ENOMEM;
    }
    (*area).area = get_vm_area(PAGE_SIZE * nr_frames as kernel::ffi::c_ulong, VM_IOREMAP);
    if (*area).area.is_null() {
        kfree((*area).ptes);
        return -ENOMEM;
    }
    if apply_to_page_range(
        &mut init_mm,
        (*(*area).area).addr as kernel::ffi::c_ulong,
        PAGE_SIZE * nr_frames as kernel::ffi::c_ulong,
        gnttab_apply,
        area as *mut kernel::ffi::c_void,
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

pub unsafe fn arch_gnttab_init(nr_shared: kernel::ffi::c_ulong, nr_status: kernel::ffi::c_ulong) -> kernel::ffi::c_int {
    let mut ret: kernel::ffi::c_int;

    if !xen_pv_domain() {
        return 0;
    }

    ret = arch_gnttab_valloc(&mut GNTTAB_SHARED_VM_AREA, nr_shared as kernel::ffi::c_uint);
    if ret < 0 {
        return ret;
    }

    /*
     * Always allocate the space for the status frames in case
     * we're migrated to a host with V2 support.
     */
    ret = arch_gnttab_valloc(&mut GNTTAB_STATUS_VM_AREA, nr_status as kernel::ffi::c_uint);
    if ret < 0 {
        arch_gnttab_vfree(&mut GNTTAB_SHARED_VM_AREA);
        return -ENOMEM;
    }

    0
}

// The following block corresponds to CONFIG_XEN_PVH.
#[cfg(CONFIG_XEN_PVH)]
unsafe fn xen_pvh_gnttab_setup() -> kernel::ffi::c_int {
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
