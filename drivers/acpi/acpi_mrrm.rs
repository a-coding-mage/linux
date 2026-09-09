// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2025, Intel Corporation.
 *
 * Memory Range and Region Mapping (MRRM) structure
 *
 * Parse and report the platform's MRRM table in /sys.
 */

// pr_fmt(fmt) "acpi/mrrm: " fmt

// Default assume one memory region covering all system memory, per the spec
static mut max_mem_region: i32 = 1;

// Access for use by resctrl file system
#[no_mangle]
pub unsafe extern "C" fn acpi_mrrm_max_mem_region() -> i32 {
    max_mem_region
}

#[repr(C)]
pub struct mrrm_mem_range_entry {
    pub base: u64,
    pub length: u64,
    pub node: i32,
    pub local_region_id: u8,
    pub remote_region_id: u8,
}

static mut mrrm_mem_range_entry: *mut mrrm_mem_range_entry = core::ptr::null_mut();
static mut mrrm_mem_entry_num: u32 = 0;

unsafe fn get_node_num(e: *mut mrrm_mem_range_entry) -> i32 {
    let mut nid: u32;

    // for_each_online_node(nid)
    for_each_online_node!(nid) {
        for z in 0..MAX_NR_ZONES {
            let zone: *mut zone = (*NODE_DATA(nid)).node_zones.add(z as usize);

            if !populated_zone(zone) {
                continue;
            }
            if zone_intersects(zone, PHYS_PFN((*e).base), PHYS_PFN((*e).length)) {
                return zone_to_nid(zone);
            }
        }
    }

    -ENOENT
}

unsafe extern "C" fn acpi_parse_mrrm(table: *mut acpi_table_header) -> i32 {
    let mut mre_entry: *mut acpi_mrrm_mem_range_entry;
    let mrrm: *mut acpi_table_mrrm;
    let mut mre: *mut core::ffi::c_void;
    let mrrm_end: *mut core::ffi::c_void;
    let mut mre_count = 0;

    mrrm = table as *mut acpi_table_mrrm;
    if mrrm.is_null() {
        return -ENODEV;
    }

    if (*mrrm).header.revision != 1 {
        return -EINVAL;
    }

    if (*mrrm).flags & ACPI_MRRM_FLAGS_REGION_ASSIGNMENT_OS != 0 {
        return -EOPNOTSUPP;
    }

    mrrm_end = (mrrm as *mut u8).add((*mrrm).header.length as usize - 1) as *mut core::ffi::c_void;
    mre = (mrrm as *mut u8).add(core::mem::size_of::<acpi_table_mrrm>()) as *mut core::ffi::c_void;
    while (mre as usize) < (mrrm_end as usize) {
        mre_entry = mre as *mut acpi_mrrm_mem_range_entry;
        mre_count += 1;
        mre = (mre as *mut u8).add((*mre_entry).header.length as usize) as *mut core::ffi::c_void;
    }
    if mre_count == 0 {
        pr_info!(FW_BUG, "No ranges listed in MRRM table\n");
        return -EINVAL;
    }

    mrrm_mem_range_entry = kmalloc_objs!(mrrm_mem_range_entry, mre_count, GFP_KERNEL | __GFP_ZERO);
    if mrrm_mem_range_entry.is_null() {
        return -ENOMEM;
    }

    mre = (mrrm as *mut u8).add(core::mem::size_of::<acpi_table_mrrm>()) as *mut core::ffi::c_void;
    while (mre as usize) < (mrrm_end as usize) {
        let e: *mut mrrm_mem_range_entry;

        mre_entry = mre as *mut acpi_mrrm_mem_range_entry;
        e = mrrm_mem_range_entry.add(mrrm_mem_entry_num as usize);

        (*e).base = (*mre_entry).addr_base;
        (*e).length = (*mre_entry).addr_len;
        (*e).node = get_node_num(e);

        if (*mre_entry).region_id_flags & ACPI_MRRM_VALID_REGION_ID_FLAGS_LOCAL != 0 {
            (*e).local_region_id = (*mre_entry).local_region_id;
        } else {
            (*e).local_region_id = -1i8 as u8;
        }
        if (*mre_entry).region_id_flags & ACPI_MRRM_VALID_REGION_ID_FLAGS_REMOTE != 0 {
            (*e).remote_region_id = (*mre_entry).remote_region_id;
        } else {
            (*e).remote_region_id = -1i8 as u8;
        }

        mrrm_mem_entry_num += 1;
        mre = (mre as *mut u8).add((*mre_entry).header.length as usize) as *mut core::ffi::c_void;
    }

    max_mem_region = (*mrrm).max_mem_region;

    0
}

macro_rules! range_attr {
    ($name:ident, $fmt:literal) => {
        unsafe extern "C" fn $name##_show(
            kobj: *mut kobject,
            _attr: *mut kobj_attribute,
            buf: *mut core::ffi::c_char,
        ) -> isize {
            let mre: *mut mrrm_mem_range_entry;
            let kname: *const core::ffi::c_char = kobject_name(kobj);
            let mut n: i32 = 0;
            let ret = kstrtoint(kname.add(5), 10, &mut n);
            if ret != 0 {
                return ret;
            }
            mre = mrrm_mem_range_entry.add(n as usize);
            sysfs_emit!(buf, $fmt, (*mre).$name)
        }
        static mut $name##_attr: kobj_attribute = __ATTR_RO!($name);
    };
}

range_attr!(base, "0x%llx\n");
range_attr!(length, "0x%llx\n");
range_attr!(node, "%d\n");
range_attr!(local_region_id, "%d\n");
range_attr!(remote_region_id, "%d\n");

static mut memory_range_attrs: [*mut attribute; 6] = [
    &mut base_attr.attr,
    &mut length_attr.attr,
    &mut node_attr.attr,
    &mut local_region_id_attr.attr,
    &mut remote_region_id_attr.attr,
    core::ptr::null_mut(),
];

ATTRIBUTE_GROUPS!(memory_range);

unsafe extern "C" fn add_boot_memory_ranges() -> i32 {
    let pkobj: *mut kobject;
    let mut kobj: *mut kobject;
    let kobjs: *mut *mut kobject;
    let mut ret = -EINVAL;
    let mut name = [0i8; 16];
    let mut i: i32;

    pkobj = kobject_create_and_add!("memory_ranges", acpi_kobj);
    if pkobj.is_null() {
        return -ENOMEM;
    }

    kobjs = kzalloc_objs!(*kobjs, mrrm_mem_entry_num);
    if kobjs.is_null() {
        kobject_put(pkobj);
        return -ENOMEM;
    }

    i = 0;
    while i < mrrm_mem_entry_num as i32 {
        scnprintf!(name.as_mut_ptr(), name.len(), "range%d", i);
        kobj = kobject_create_and_add!(name.as_ptr(), pkobj);
        if kobj.is_null() {
            ret = -ENOMEM;
            goto_cleanup!();
        }

        ret = sysfs_create_groups(kobj, memory_range_groups);
        if ret != 0 {
            kobject_put(kobj);
            goto_cleanup!();
        }
        *kobjs.add(i as usize) = kobj;
        i += 1;
    }

    kfree(kobjs as *mut core::ffi::c_void);
    return 0;

    // cleanup:
    for j in 0..i {
        let obj = *kobjs.add(j as usize);
        if !obj.is_null() {
            sysfs_remove_groups(obj, memory_range_groups);
            kobject_put(obj);
        }
    }
    kfree(kobjs as *mut core::ffi::c_void);
    kobject_put(pkobj);
    ret
}

unsafe extern "C" fn mrrm_init() -> i32 {
    let ret = acpi_table_parse(ACPI_SIG_MRRM, Some(acpi_parse_mrrm));
    if ret < 0 {
        return ret;
    }
    add_boot_memory_ranges()
}

device_initcall!(mrrm_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
