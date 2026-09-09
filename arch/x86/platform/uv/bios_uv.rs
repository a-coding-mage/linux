// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * BIOS run time interface routines.
 *
 * (C) Copyright 2020 Hewlett Packard Enterprise Development LP
 * Copyright (C) 2007-2017 Silicon Graphics, Inc. All rights reserved.
 * Copyright (c) Russ Anderson <rja@sgi.com>
 */

// External kernel/EFI/UV declarations and macros are supplied by other files.

pub static mut uv_systab_phys: ::core::ffi::c_ulong = EFI_INVALID_TABLE_ADDR;
pub static mut uv_systab: *mut uv_systab = core::ptr::null_mut();

unsafe fn __uv_bios_call(which: uv_bios_cmd, a1: u64, a2: u64, a3: u64,
                         a4: u64, a5: u64) -> i64 {
    let tab = uv_systab;
    if tab.is_null() || (*tab).function.is_none() {
        /* BIOS does not support UV systab */
        return BIOS_STATUS_UNIMPLEMENTED;
    }

    // Equivalent to efi_call_virt_pointer(tab, function, ...).
    ((*tab).function.unwrap())(which as u64, a1, a2, a3, a4, a5)
}

unsafe fn uv_bios_call(which: uv_bios_cmd, a1: u64, a2: u64, a3: u64,
                       a4: u64, a5: u64) -> i64 {
    let mut ret: i64;
    if down_interruptible(&mut __efi_uv_runtime_lock) != 0 {
        return BIOS_STATUS_ABORT;
    }
    ret = __uv_bios_call(which, a1, a2, a3, a4, a5);
    up(&mut __efi_uv_runtime_lock);
    ret
}

unsafe fn uv_bios_call_irqsave(which: uv_bios_cmd, a1: u64, a2: u64, a3: u64,
                               a4: u64, a5: u64) -> i64 {
    let mut bios_flags: ::core::ffi::c_ulong = 0;
    let ret: i64;
    if down_interruptible(&mut __efi_uv_runtime_lock) != 0 {
        return BIOS_STATUS_ABORT;
    }
    local_irq_save(&mut bios_flags);
    ret = __uv_bios_call(which, a1, a2, a3, a4, a5);
    local_irq_restore(bios_flags);
    up(&mut __efi_uv_runtime_lock);
    ret
}

pub static mut sn_partition_id: ::core::ffi::c_long = 0;
pub static mut sn_coherency_id: ::core::ffi::c_long = 0;
pub static mut sn_region_size: ::core::ffi::c_long = 0;
pub static mut system_serial_number: ::core::ffi::c_long = 0;
pub static mut uv_type: ::core::ffi::c_int = 0;

pub unsafe fn uv_bios_get_sn_info(fc: ::core::ffi::c_int,
    uvtype: *mut ::core::ffi::c_int, partid: *mut ::core::ffi::c_long,
    coher: *mut ::core::ffi::c_long, region: *mut ::core::ffi::c_long,
    ssn: *mut ::core::ffi::c_long) -> i64 {
    let mut v0: u64 = 0;
    let mut v1: u64 = 0;
    let mut part: partition_info_u = core::mem::zeroed();
    let ret = uv_bios_call_irqsave(UV_BIOS_GET_SN_INFO, fc as u64,
        (&mut v0 as *mut u64) as u64, (&mut v1 as *mut u64) as u64, 0, 0);
    if ret != BIOS_STATUS_SUCCESS { return ret; }
    part.val = v0;
    if !uvtype.is_null() { *uvtype = part.hub_version as _; }
    if !partid.is_null() { *partid = part.partition_id as _; }
    if !coher.is_null() { *coher = part.coherence_id as _; }
    if !region.is_null() { *region = part.region_size as _; }
    if !ssn.is_null() { *ssn = v1 as _; }
    ret
}

pub unsafe fn uv_bios_mq_watchlist_alloc(addr: ::core::ffi::c_ulong,
    mq_size: ::core::ffi::c_uint, intr_mmr_offset: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_int {
    let mut watchlist: u64 = 0;
    /* bios returns watchlist number or negative error number. */
    let ret = uv_bios_call_irqsave(UV_BIOS_WATCHLIST_ALLOC, addr as u64,
        mq_size as u64, intr_mmr_offset as u64, (&mut watchlist as *mut u64) as u64, 0) as i32;
    if ret < BIOS_STATUS_SUCCESS as i32 { return ret; }
    watchlist as i32
}

pub unsafe fn uv_bios_mq_watchlist_free(blade: i32, watchlist_num: i32) -> i32 {
    uv_bios_call_irqsave(UV_BIOS_WATCHLIST_FREE, blade as u64, watchlist_num as u64, 0, 0, 0) as i32
}

pub unsafe fn uv_bios_change_memprotect(paddr: u64, len: u64, perms: uv_memprotect) -> i64 {
    uv_bios_call_irqsave(UV_BIOS_MEMPROTECT, paddr, len, perms as u64, 0, 0)
}

pub unsafe fn uv_bios_reserved_page_pa(buf: u64, cookie: *mut u64, addr: *mut u64, len: *mut u64) -> i64 {
    uv_bios_call_irqsave(UV_BIOS_GET_PARTITION_ADDR, cookie as u64, addr as u64, buf, len as u64, 0)
}

pub unsafe fn uv_bios_freq_base(clock_type: u64, ticks_per_second: *mut u64) -> i64 {
    uv_bios_call(UV_BIOS_FREQ_BASE, clock_type, ticks_per_second as u64, 0, 0, 0)
}

/* uv_bios_set_legacy_vga_target - Set Legacy VGA I/O Target */
pub unsafe fn uv_bios_set_legacy_vga_target(decode: bool, domain: i32, bus: i32) -> i32 {
    uv_bios_call(UV_BIOS_SET_LEGACY_VGA_TARGET, decode as u64, domain as u64, bus as u64, 0, 0) as i32
}

pub unsafe fn uv_bios_get_master_nasid(size: u64, master_nasid: *mut u64) -> i64 {
    uv_bios_call(UV_BIOS_EXTRA, 0, UV_BIOS_EXTRA_MASTER_NASID as u64, 0, size, master_nasid as u64)
}
pub unsafe fn uv_bios_get_heapsize(nasid: u64, size: u64, heap_size: *mut u64) -> i64 {
    uv_bios_call(UV_BIOS_EXTRA, nasid, UV_BIOS_EXTRA_GET_HEAPSIZE as u64, 0, size, heap_size as u64)
}
pub unsafe fn uv_bios_install_heap(nasid: u64, heap_size: u64, bios_heap: *mut u64) -> i64 {
    uv_bios_call(UV_BIOS_EXTRA, nasid, UV_BIOS_EXTRA_INSTALL_HEAP as u64, 0, heap_size, bios_heap as u64)
}
pub unsafe fn uv_bios_obj_count(nasid: u64, size: u64, objcnt: *mut u64) -> i64 {
    uv_bios_call(UV_BIOS_EXTRA, nasid, UV_BIOS_EXTRA_OBJECT_COUNT as u64, 0, size, objcnt as u64)
}
pub unsafe fn uv_bios_enum_objs(nasid: u64, size: u64, objbuf: *mut u64) -> i64 {
    uv_bios_call(UV_BIOS_EXTRA, nasid, UV_BIOS_EXTRA_ENUM_OBJECTS as u64, 0, size, objbuf as u64)
}
pub unsafe fn uv_bios_enum_ports(nasid: u64, obj_id: u64, size: u64, portbuf: *mut u64) -> i64 {
    uv_bios_call(UV_BIOS_EXTRA, nasid, UV_BIOS_EXTRA_ENUM_PORTS as u64, obj_id, size, portbuf as u64)
}
pub unsafe fn uv_bios_get_geoinfo(nasid: u64, size: u64, buf: *mut u64) -> i64 {
    uv_bios_call(UV_BIOS_GET_GEOINFO, nasid, buf as u64, size, 0, 0)
}
pub unsafe fn uv_bios_get_pci_topology(size: u64, buf: *mut u64) -> i64 {
    uv_bios_call(UV_BIOS_GET_PCI_TOPOLOGY, buf as u64, size, 0, 0, 0)
}

pub unsafe fn get_uv_systab_phys(msg: bool) -> ::core::ffi::c_ulong {
    if uv_systab_phys == EFI_INVALID_TABLE_ADDR || uv_systab_phys == 0 || efi_runtime_disabled() {
        if msg { pr_crit!("UV: UVsystab: missing\n"); }
        return 0;
    }
    uv_systab_phys
}

pub unsafe fn uv_bios_init() -> i32 {
    uv_systab = core::ptr::null_mut();
    let uv_systab_phys_addr = get_uv_systab_phys(true);
    if uv_systab_phys_addr == 0 { return -EEXIST; }
    uv_systab = ioremap(uv_systab_phys_addr, core::mem::size_of::<uv_systab>());
    if uv_systab.is_null() || strncmp((*uv_systab).signature.as_ptr(), UV_SYSTAB_SIG.as_ptr(), 4) != 0 {
        pr_err!("UV: UVsystab: bad signature!\n");
        iounmap(uv_systab);
        return -EINVAL;
    }
    /* Starting with UV4 the UV systab size is variable */
    if (*uv_systab).revision >= UV_SYSTAB_VERSION_UV4 {
        let size = (*uv_systab).size;
        iounmap(uv_systab);
        uv_systab = ioremap(uv_systab_phys_addr, size as usize);
        if uv_systab.is_null() {
            pr_err!("UV: UVsystab: ioremap(%d) failed!\n", size);
            return -EFAULT;
        }
    }
    pr_info!("UV: UVsystab: Revision:%x\n", (*uv_systab).revision);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
