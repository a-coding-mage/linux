// SPDX-License-Identifier: GPL-2.0
/*
 * dev-path-parser.c - EFI Device Path parser
 * Copyright (C) 2016 Lukas Wunner <lukas@wunner.de>
 */

// Linux EFI, ACPI, and PCI declarations are supplied by the surrounding build.

unsafe fn parse_acpi_path(
    node: *const efi_dev_path,
    _parent: *mut device,
    child: *mut *mut device,
) -> libc::c_long {
    let adev: *mut acpi_device;
    let phys_dev: *mut device;
    let mut hid = [0i8; ACPI_ID_LEN as usize];

    if (*node).header.length != 12 {
        return -EINVAL as libc::c_long;
    }

    sprintf(
        hid.as_mut_ptr(),
        b"%c%c%c%04X\0".as_ptr() as *const libc::c_char,
        b'A' as libc::c_int + (((*node).acpi.hid >> 10) & 0x1f) as libc::c_int - 1,
        b'A' as libc::c_int + (((*node).acpi.hid >> 5) & 0x1f) as libc::c_int - 1,
        b'A' as libc::c_int + (((*node).acpi.hid >> 0) & 0x1f) as libc::c_int - 1,
        (*node).acpi.hid >> 16,
    );

    adev = core::ptr::null_mut();
    // for_each_acpi_dev_match(adev, hid, NULL, -1)
    // The kernel iterator macro is retained as the corresponding external operation.
    for_each_acpi_dev_match!(adev, hid.as_ptr(), core::ptr::null(), -1, {
        if acpi_dev_uid_match(adev, (*node).acpi.uid) {
            break;
        }
        if acpi_device_uid(adev).is_null() && (*node).acpi.uid == 0 {
            break;
        }
    });
    if adev.is_null() {
        return -ENODEV as libc::c_long;
    }

    phys_dev = acpi_get_first_physical_node(adev);
    if !phys_dev.is_null() {
        *child = get_device(phys_dev);
        acpi_dev_put(adev);
    } else {
        *child = &mut (*adev).dev;
    }

    0
}

unsafe fn match_pci_dev(dev: *mut device, data: *const libc::c_void) -> libc::c_int {
    let devfn = *(data as *const libc::c_uint);
    (dev_is_pci(dev) && (*to_pci_dev(dev)).devfn == devfn) as libc::c_int
}

unsafe fn parse_pci_path(
    node: *const efi_dev_path,
    parent: *mut device,
    child: *mut *mut device,
) -> libc::c_long {
    let devfn: libc::c_uint;

    if (*node).header.length != 6 {
        return -EINVAL as libc::c_long;
    }
    if parent.is_null() {
        return -EINVAL as libc::c_long;
    }

    devfn = PCI_DEVFN((*node).pci.dev, (*node).pci.fn);
    *child = device_find_child(parent, &devfn as *const _ as *const libc::c_void, match_pci_dev);
    if (*child).is_null() {
        return -ENODEV as libc::c_long;
    }
    0
}

/*
 * Insert parsers for further node types here.
 *
 * Each parser takes a pointer to the @node and to the @parent (will be NULL
 * for the first device path node). If a device corresponding to @node was
 * found below @parent, its reference count should be incremented and the
 * device returned in @child.
 */
unsafe fn parse_end_path(
    node: *const efi_dev_path,
    parent: *mut device,
    child: *mut *mut device,
) -> libc::c_long {
    if (*node).header.length != 4 {
        return -EINVAL as libc::c_long;
    }
    if (*node).header.sub_type != EFI_DEV_END_INSTANCE
        && (*node).header.sub_type != EFI_DEV_END_ENTIRE
    {
        return -EINVAL as libc::c_long;
    }
    if parent.is_null() {
        return -ENODEV as libc::c_long;
    }
    *child = get_device(parent);
    (*node).header.sub_type as libc::c_long
}

/// efi_get_device_by_path - find device by EFI Device Path
pub unsafe fn efi_get_device_by_path(
    node: *mut *const efi_dev_path,
    len: *mut usize,
) -> *mut device {
    let mut parent: *mut device = core::ptr::null_mut();
    let mut child: *mut device;
    let mut ret: libc::c_long = 0;

    if *len == 0 {
        return core::ptr::null_mut();
    }

    while ret == 0 {
        if *len < 4 || *len < (*(*node)).header.length as usize {
            ret = -EINVAL as libc::c_long;
        } else if (*(*node)).header.type_ == EFI_DEV_ACPI
            && (*(*node)).header.sub_type == EFI_DEV_BASIC_ACPI
        {
            ret = parse_acpi_path(*node, parent, &mut child);
        } else if (*(*node)).header.type_ == EFI_DEV_HW
            && (*(*node)).header.sub_type == EFI_DEV_PCI
        {
            ret = parse_pci_path(*node, parent, &mut child);
        } else if (*(*node)).header.type_ == EFI_DEV_END_PATH
            || (*(*node)).header.type_ == EFI_DEV_END_PATH2
        {
            ret = parse_end_path(*node, parent, &mut child);
        } else {
            ret = -ENOTSUPP as libc::c_long;
        }

        put_device(parent);
        if ret < 0 {
            return ERR_PTR(ret);
        }

        parent = child;
        *node = ((*node as *const u8).add((*(*node)).header.length as usize)) as *const efi_dev_path;
        *len -= (*(*node)).header.length as usize;
    }

    if ret == EFI_DEV_END_ENTIRE as libc::c_long {
        *len = 0;
    }
    child
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
