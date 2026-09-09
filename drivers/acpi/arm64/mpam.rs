// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2025 Arm Ltd.

/* Parse the MPAM ACPI table feeding the discovered nodes into the driver */

// C dependencies: linux/acpi.h, linux/arm_mpam.h, linux/bits.h, linux/cpu.h,
// linux/cpumask.h, linux/platform_device.h, and acpi/processor.h.

const ACPI_MPAM_MSC_IRQ_MODE: u32 = 1 << 0;
const ACPI_MPAM_MSC_IRQ_TYPE_MASK: u32 = 0b11 << 1;
const ACPI_MPAM_MSC_IRQ_TYPE_WIRED: u32 = 0;
const ACPI_MPAM_MSC_IRQ_AFFINITY_TYPE_MASK: u32 = 1 << 3;
const ACPI_MPAM_MSC_IRQ_AFFINITY_TYPE_PROCESSOR: u32 = 0;
const ACPI_MPAM_MSC_IRQ_AFFINITY_TYPE_PROCESSOR_CONTAINER: u32 = 1;
const ACPI_MPAM_MSC_IRQ_AFFINITY_VALID: u32 = 1 << 4;

const ACPI_MPAM_MSC_IFACE_MMIO: u8 = 0x00;
const ACPI_MPAM_MSC_IFACE_PCC: u8 = 0x0a;

#[inline]
unsafe fn _is_ppi_partition(flags: u32) -> bool {
    let is_ppi = (flags & ACPI_MPAM_MSC_IRQ_AFFINITY_VALID) >> 4;
    if is_ppi == 0 {
        return false;
    }
    let aff_type = (flags & ACPI_MPAM_MSC_IRQ_AFFINITY_TYPE_MASK) >> 3;
    let ret = aff_type == ACPI_MPAM_MSC_IRQ_AFFINITY_TYPE_PROCESSOR_CONTAINER;
    if ret {
        pr_err_once!("Partitioned interrupts not supported\n");
    }
    ret
}

unsafe fn acpi_mpam_register_irq(
    pdev: *mut platform_device,
    intid: u32,
    flags: u32,
) -> i32 {
    if intid == 0 {
        return -EINVAL;
    }
    if _is_ppi_partition(flags) {
        return -EINVAL;
    }
    let trigger = (flags & ACPI_MPAM_MSC_IRQ_MODE) as i32;
    let int_type = (flags & ACPI_MPAM_MSC_IRQ_TYPE_MASK) >> 1;
    if int_type != ACPI_MPAM_MSC_IRQ_TYPE_WIRED {
        return -EINVAL;
    }
    let irq = acpi_register_gsi(&mut (*pdev).dev, intid, trigger, ACPI_ACTIVE_HIGH);
    if irq < 0 {
        pr_err_once!("Failed to register interrupt 0x%x with ACPI\n", intid);
    }
    irq
}

unsafe fn acpi_mpam_parse_irqs(
    pdev: *mut platform_device,
    tbl_msc: *mut acpi_mpam_msc_node,
    res: *mut resource,
    res_idx: *mut i32,
) {
    let irq = acpi_mpam_register_irq(pdev, (*tbl_msc).overflow_interrupt,
        (*tbl_msc).overflow_interrupt_flags);
    if irq > 0 {
        *res.add(*res_idx as usize) = DEFINE_RES_IRQ_NAMED!(irq, "overflow");
        *res_idx += 1;
    }
    let irq = acpi_mpam_register_irq(pdev, (*tbl_msc).error_interrupt,
        (*tbl_msc).error_interrupt_flags);
    if irq > 0 {
        *res.add(*res_idx as usize) = DEFINE_RES_IRQ_NAMED!(irq, "error");
        *res_idx += 1;
    }
}

unsafe fn acpi_mpam_parse_resource(msc: *mut mpam_msc, res: *mut acpi_mpam_resource_node) -> i32 {
    match (*res).locator_type {
        ACPI_MPAM_LOCATION_TYPE_PROCESSOR_CACHE => {
            let cache_id = (*res).locator.cache_locator.cache_reference;
            let level = find_acpi_cache_level_from_id(cache_id);
            if level <= 0 {
                pr_err_once!("Bad level (%d) for cache with id %u\n", level, cache_id);
                return -EINVAL;
            }
            mpam_ris_create(msc, (*res).ris_index, MPAM_CLASS_CACHE, level, cache_id)
        }
        ACPI_MPAM_LOCATION_TYPE_MEMORY => {
            let mut nid = pxm_to_node((*res).locator.memory_locator.proximity_domain);
            if nid == NUMA_NO_NODE {
                pr_debug!("Bad proximity domain %lld, using node 0 instead\n",
                    (*res).locator.memory_locator.proximity_domain);
                nid = 0;
            }
            mpam_ris_create(msc, (*res).ris_index, MPAM_CLASS_MEMORY,
                MPAM_CLASS_ID_DEFAULT, nid)
        }
        _ => 0, /* These get discovered later and are treated as unknown */
    }
}

pub unsafe fn acpi_mpam_parse_resources(
    msc: *mut mpam_msc,
    tbl_msc: *mut acpi_mpam_msc_node,
) -> i32 {
    let table_end = (tbl_msc as *mut u8).add((*tbl_msc).length as usize);
    let mut ptr = tbl_msc.add(1) as *mut u8;
    for _ in 0..(*tbl_msc).num_resource_nodes {
        let resource = ptr as *mut acpi_mpam_resource_node;
        if ptr.add(core::mem::size_of::<acpi_mpam_resource_node>()) > table_end {
            return -EINVAL;
        }
        let remaining_table = table_end.offset_from(ptr) as u64;
        let max_deps = remaining_table / core::mem::size_of::<acpi_mpam_func_deps>() as u64;
        if (*resource).num_functional_deps as u64 > max_deps {
            pr_debug!("MSC has impossible number of functional dependencies\n");
            return -EINVAL;
        }
        let err = acpi_mpam_parse_resource(msc, resource);
        if err != 0 { return err; }
        ptr = ptr.add(core::mem::size_of::<acpi_mpam_resource_node>());
        ptr = ptr.add((*resource).num_functional_deps as usize * core::mem::size_of::<acpi_mpam_func_deps>());
    }
    0
}

unsafe fn parse_msc_pm_link(tbl: *mut acpi_mpam_msc_node, pdev: *mut platform_device, acpi_id: *mut u32) -> bool {
    let mut hid = [0i8; core::mem::size_of::<[i8; 8]>()];
    core::ptr::copy_nonoverlapping((*tbl).hardware_id_linked_device.as_ptr(), hid.as_mut_ptr(), hid.len());
    let mut valid = false;
    if strcmp(hid.as_ptr(), ACPI_PROCESSOR_CONTAINER_HID) == 0 {
        *acpi_id = (*tbl).instance_id_linked_device;
        valid = true;
    }
    let mut uid = [0i8; 11];
    let len = snprintf(uid.as_mut_ptr(), uid.len(), "%u\0".as_ptr() as *const i8,
        (*tbl).instance_id_linked_device);
    if len >= uid.len() as i32 {
        pr_debug!("Failed to convert uid of device for power management.");
        return valid;
    }
    let buddy = acpi_dev_get_first_match_dev(hid.as_ptr(), uid.as_ptr(), -1);
    if !buddy.is_null() {
        device_link_add(&mut (*pdev).dev, &mut (*buddy).dev, DL_FLAG_STATELESS);
        acpi_dev_put(buddy);
    }
    valid
}

unsafe fn decode_interface_type(tbl: *mut acpi_mpam_msc_node, iface: *mut mpam_msc_iface) -> i32 {
    match (*tbl).interface_type {
        ACPI_MPAM_MSC_IFACE_MMIO => { *iface = MPAM_IFACE_MMIO; 0 }
        ACPI_MPAM_MSC_IFACE_PCC => { *iface = MPAM_IFACE_PCC; 0 }
        _ => -EINVAL,
    }
}

unsafe fn acpi_mpam_parse_msc(tbl: *mut acpi_mpam_msc_node) -> *mut platform_device {
    let pdev = platform_device_alloc("mpam_msc\0".as_ptr() as *const i8, (*tbl).identifier);
    if pdev.is_null() { return ERR_PTR(-ENOMEM); }
    let mut res = [core::mem::zeroed::<resource>(); 3];
    let mut props = [core::mem::zeroed::<property_entry>(); 4];
    let mut next_res = 0usize;
    let mut next_prop = 0usize;
    let mut iface = core::mem::zeroed::<mpam_msc_iface>();
    let mut uid = [0i8; 16];
    let n = snprintf(uid.as_mut_ptr(), uid.len(), "%u\0".as_ptr() as *const i8, (*tbl).identifier);
    if n > 0 && n < uid.len() as i32 {
        let companion = acpi_dev_get_first_match_dev("ARMHAA5C\0".as_ptr() as *const i8, uid.as_ptr(), -1);
        if !companion.is_null() { ACPI_COMPANION_SET!(&mut (*pdev).dev, companion); acpi_dev_put(companion); }
        else { pr_debug!("MSC.%u: missing namespace entry\n", (*tbl).identifier); }
    }
    if decode_interface_type(tbl, &mut iface) != 0 { pr_debug!("MSC.%u: unknown interface type\n", (*tbl).identifier); return ERR_PTR(-EINVAL); }
    if iface == MPAM_IFACE_MMIO {
        res[next_res] = DEFINE_RES_MEM_NAMED!((*tbl).base_address, (*tbl).mmio_size, "MPAM:MSC"); next_res += 1;
    } else if iface == MPAM_IFACE_PCC {
        props[next_prop] = PROPERTY_ENTRY_U32!("pcc-channel", (*tbl).base_address); next_prop += 1;
    }
    acpi_mpam_parse_irqs(pdev, tbl, res.as_mut_ptr(), &mut next_res as *mut usize as *mut i32);
    let err = platform_device_add_resources(pdev, res.as_mut_ptr(), next_res as u32); if err != 0 { return ERR_PTR(err); }
    props[next_prop] = PROPERTY_ENTRY_U32!("arm,not-ready-us", (*tbl).max_nrdy_usec); next_prop += 1;
    let mut acpi_id = 0u32;
    if parse_msc_pm_link(tbl, pdev, &mut acpi_id) { props[next_prop] = PROPERTY_ENTRY_U32!("cpu_affinity", acpi_id); next_prop += 1; }
    let err = device_create_managed_software_node(&mut (*pdev).dev, props.as_mut_ptr(), core::ptr::null_mut()); if err != 0 { return ERR_PTR(err); }
    let err = platform_device_add_data(pdev, tbl as *const _, (*tbl).length as usize); if err != 0 { return ERR_PTR(err); }
    let err = platform_device_add(pdev); if err != 0 { return ERR_PTR(err); }
    pdev
}

unsafe fn acpi_mpam_parse() -> i32 {
    if acpi_disabled || !system_supports_mpam() { return 0; }
    let table = acpi_get_table_pointer(ACPI_SIG_MPAM, 0); if IS_ERR!(table) { return 0; }
    if (*table).revision < 1 { return 0; }
    let end = (table as *mut u8).add((*table).length as usize); let mut off = (table.add(1)) as *mut u8;
    while off < end {
        let tbl = off as *mut acpi_mpam_msc_node;
        if off.add(core::mem::size_of::<acpi_mpam_msc_node>()) > end || off.add((*tbl).length as usize) > end { pr_err!("MSC entry overlaps end of ACPI table\n"); return -EINVAL; }
        off = off.add((*tbl).length as usize);
        if (*tbl).reserved != 0 || (*tbl).reserved1 != 0 || (*tbl).reserved2 != 0 { pr_err_once!("Unrecognised MSC, MPAM not usable\n"); continue; }
        if (*tbl).mmio_size == 0 { continue; }
        let pdev = acpi_mpam_parse_msc(tbl); if IS_ERR!(pdev) { return PTR_ERR!(pdev); }
    }
    0
}

pub unsafe fn acpi_mpam_count_msc() -> i32 {
    if acpi_disabled || !system_supports_mpam() { return 0; }
    let table = acpi_get_table_pointer(ACPI_SIG_MPAM, 0); if IS_ERR!(table) || (*table).revision < 1 { return 0; }
    let end = (table as *mut u8).add((*table).length as usize); let mut off = table.add(1) as *mut u8; let mut count = 0;
    while off < end { let tbl = off as *mut acpi_mpam_msc_node;
        if off.add(core::mem::size_of::<acpi_mpam_msc_node>()) > end || (*tbl).length < core::mem::size_of::<acpi_mpam_msc_node>() as u32 || off.add((*tbl).length as usize) > end { return -EINVAL; }
        off = off.add((*tbl).length as usize); if (*tbl).mmio_size != 0 { count += 1; }
    }
    count
}

// Equivalent to subsys_initcall_sync(acpi_mpam_parse).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
