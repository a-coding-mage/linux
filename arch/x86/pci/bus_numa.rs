// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel translation.

// #include <linux/init.h>
// #include <linux/pci.h>
// #include <linux/range.h>
// #include "bus_numa.h"

// LIST_HEAD(pci_root_infos);
pub static mut pci_root_infos: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };

unsafe fn x86_find_pci_root_info(bus: i32) -> *mut pci_root_info {
    let mut info: *mut pci_root_info = core::ptr::null_mut();

    list_for_each_entry!(info, &mut pci_root_infos, list, {
        if (*info).busn.start == bus as resource_size_t {
            return info;
        }
    });

    core::ptr::null_mut()
}

pub unsafe fn x86_pci_root_bus_node(bus: i32) -> i32 {
    let info = x86_find_pci_root_info(bus);

    if info.is_null() {
        return NUMA_NO_NODE;
    }

    (*info).node
}

pub unsafe fn x86_pci_root_bus_resources(bus: i32, resources: *mut list_head) {
    let info = x86_find_pci_root_info(bus);
    let mut root_res: *mut pci_root_res;
    let mut window: *mut resource_entry;
    let mut found = false;

    if info.is_null() {
        goto_default_resources!(default_resources);
    }

    printk!(KERN_DEBUG, "PCI: root bus %02x: hardware-probed resources\n", bus);

    /* already added by acpi ? */
    resource_list_for_each_entry!(window, resources, {
        if (*(*window).res).flags & IORESOURCE_BUS != 0 {
            found = true;
            break;
        }
    });

    if !found {
        pci_add_resource(resources, &mut (*info).busn);
    }

    list_for_each_entry!(root_res, &mut (*info).resources, list, {
        pci_add_resource(resources, &mut (*root_res).res);
    });

    return;

    default_resources: {
        /*
         * We don't have any host bridge aperture information from the
         * "native host bridge drivers," e.g., amd_bus or broadcom_bus,
         * so fall back to the defaults historically used by pci_create_bus().
         */
        printk!(KERN_DEBUG, "PCI: root bus %02x: using default resources\n", bus);
        pci_add_resource(resources, &mut ioport_resource);
        pci_add_resource(resources, &mut iomem_resource);
    }
}

pub unsafe fn alloc_pci_root_info(
    bus_min: i32,
    bus_max: i32,
    node: i32,
    link: i32,
) -> *mut pci_root_info {
    let info = kzalloc_obj!(pci_root_info);

    if info.is_null() {
        return info;
    }

    sprintf!((*info).name, "PCI Bus #%02x", bus_min);

    INIT_LIST_HEAD!(&mut (*info).resources);
    (*info).busn.name = (*info).name.as_mut_ptr();
    (*info).busn.start = bus_min as resource_size_t;
    (*info).busn.end = bus_max as resource_size_t;
    (*info).busn.flags = IORESOURCE_BUS;
    (*info).node = node;
    (*info).link = link;

    list_add_tail!(&mut (*info).list, &mut pci_root_infos);

    info
}

pub unsafe fn update_res(
    info: *mut pci_root_info,
    start: resource_size_t,
    end: resource_size_t,
    flags: c_ulong,
    merge: i32,
) {
    let mut res: *mut resource;
    let mut root_res: *mut pci_root_res;

    if start > end || start == RESOURCE_SIZE_MAX {
        return;
    }

    if !merge {
        goto_addit!(addit);
    }

    /* try to merge it with old one */
    list_for_each_entry!(root_res, &mut (*info).resources, list, {
        let mut final_start: resource_size_t;
        let mut final_end: resource_size_t;
        let mut common_start: resource_size_t;
        let mut common_end: resource_size_t;

        res = &mut (*root_res).res;
        if (*res).flags != flags {
            continue;
        }

        common_start = core::cmp::max((*res).start, start);
        common_end = core::cmp::min((*res).end, end);
        if common_start > common_end.wrapping_add(1) {
            continue;
        }

        final_start = core::cmp::min((*res).start, start);
        final_end = core::cmp::max((*res).end, end);
        (*res).start = final_start;
        (*res).end = final_end;
        return;
    });

    addit: {
        /* need to add that */
        root_res = kzalloc_obj!(pci_root_res);
        if root_res.is_null() {
            return;
        }

        res = &mut (*root_res).res;
        (*res).name = (*info).name.as_mut_ptr();
        (*res).flags = flags;
        (*res).start = start;
        (*res).end = end;
        list_add_tail!(&mut (*root_res).list, &mut (*info).resources);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
