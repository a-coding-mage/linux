// SPDX-License-Identifier: GPL-2.0-only
/*
 * framebuffer-coreboot.c
 *
 * Memory based framebuffer accessed through coreboot table.
 *
 * Copyright 2012-2013 David Herrmann <dh.herrmann@gmail.com>
 * Copyright 2017 Google Inc.
 * Copyright 2017 Samuel Holland <samuel@sholland.org>
 */

// Linux and coreboot definitions used by this translation are supplied by
// the surrounding kernel bindings.
use crate::coreboot_table::*;
use crate::linux::*;

#[cfg(feature = "pci")]
unsafe fn framebuffer_pci_dev_is_enabled(pdev: *mut pci_dev) -> bool {
    /* TODO: Try to integrate this code into the PCI subsystem */
    let mut command: u16 = 0;

    let ret = pci_read_config_word(pdev, PCI_COMMAND, &mut command);
    if ret != PCIBIOS_SUCCESSFUL {
        return false;
    }
    if command & PCI_COMMAND_MEMORY == 0 {
        return false;
    }
    true
}

#[cfg(feature = "pci")]
unsafe fn framebuffer_parent_pci_dev(res: *mut resource) -> *mut pci_dev {
    let mut pdev: *mut pci_dev = core::ptr::null_mut();
    let mut r: *const resource = core::ptr::null();

    while r.is_null() {
        pdev = pci_get_base_class(PCI_BASE_CLASS_DISPLAY, pdev);
        if pdev.is_null() {
            break;
        }
        r = pci_find_resource(pdev, res);
    }

    if r.is_null() || pdev.is_null() {
        return core::ptr::null_mut(); // not found; not an error
    }

    if !framebuffer_pci_dev_is_enabled(pdev) {
        pci_dev_put(pdev);
        return ERR_PTR(-ENODEV);
    }

    pdev
}

#[cfg(not(feature = "pci"))]
unsafe fn framebuffer_parent_pci_dev(_res: *mut resource) -> *mut pci_dev {
    core::ptr::null_mut()
}

unsafe fn framebuffer_parent_dev(res: *mut resource) -> *mut device {
    let pdev = framebuffer_parent_pci_dev(res);
    if IS_ERR(pdev) {
        return ERR_CAST(pdev);
    } else if !pdev.is_null() {
        return &mut (*pdev).dev;
    }

    core::ptr::null_mut()
}

unsafe fn framebuffer_probe(dev: *mut coreboot_device) -> i32 {
    let fb: *mut lb_framebuffer = &mut (*dev).framebuffer;
    let mut parent: *mut device;
    let mut pdev: *mut platform_device;
    let mut res: resource;
    let ret: i32;

    /*
     * On coreboot systems, the advertised LB_TAG_FRAMEBUFFER entry
     * in the coreboot table should only be used if the payload did
     * not pass a framebuffer information to the Linux kernel.
     *
     * If the global screen_info data has been filled, the Generic
     * System Framebuffers (sysfb) will already register a platform
     * device and pass that screen_info as platform_data to a driver
     * that can scan-out using the system provided framebuffer.
     */
    if sysfb_handles_screen_info() {
        return -ENODEV;
    }

    if (*fb).physical_address == 0 {
        return -ENODEV;
    }

    res = DEFINE_RES_MEM(
        (*fb).physical_address,
        PAGE_ALIGN((*fb).y_resolution * (*fb).bytes_per_line),
    );
    if res.end <= res.start {
        return -EINVAL;
    }

    parent = framebuffer_parent_dev(&mut res);
    if IS_ERR(parent) {
        return PTR_ERR(parent);
    }

    #[cfg(feature = "drm_corebootdrm")]
    {
        pdev = platform_device_register_resndata(
            parent, b"coreboot-framebuffer\0".as_ptr() as *const _, 0,
            &mut res, 1, fb as *const _, (*fb).size,
        );
        if IS_ERR(pdev) {
            pr_warn(b"coreboot: could not register framebuffer\n\0".as_ptr() as *const _);
            ret = PTR_ERR(pdev);
            goto_out_put_device_parent(parent, ret)
        }
    }

    #[cfg(not(feature = "drm_corebootdrm"))]
    {
        /* FIXME: Coreboot systems should use a driver that binds to
         * coreboot-framebuffer devices. Remove support for
         * simple-framebuffer at some point. */
        let mut pdata = simplefb_platform_data {
            width: (*fb).x_resolution,
            height: (*fb).y_resolution,
            stride: (*fb).bytes_per_line,
            format: core::ptr::null(),
        };
        let formats = SIMPLEFB_FORMATS;
        let mut i = 0;
        while i < formats.len() {
            let f = &formats[i];
            if (*fb).bits_per_pixel == f.bits_per_pixel
                && (*fb).red_mask_pos == f.red.offset
                && (*fb).red_mask_size == f.red.length
                && (*fb).green_mask_pos == f.green.offset
                && (*fb).green_mask_size == f.green.length
                && (*fb).blue_mask_pos == f.blue.offset
                && (*fb).blue_mask_size == f.blue.length
            {
                pdata.format = f.name;
            }
            i += 1;
        }
        if pdata.format.is_null() {
            ret = -ENODEV;
            goto_out_put_device_parent(parent, ret)
        }
        pdev = platform_device_register_resndata(
            parent, b"simple-framebuffer\0".as_ptr() as *const _, 0,
            &mut res, 1, &pdata as *const _, core::mem::size_of_val(&pdata),
        );
        if IS_ERR(pdev) {
            pr_warn(b"coreboot: could not register framebuffer\n\0".as_ptr() as *const _);
            ret = PTR_ERR(pdev);
            goto_out_put_device_parent(parent, ret)
        }
    }

    ret = 0;
    if !parent.is_null() {
        put_device(parent);
    }
    ret
}

// C designated initializers and module registration retained as declarations
// for the surrounding kernel/module bindings.
static FRAMEBUFFER_IDS: [coreboot_device_id; 2] = [
    coreboot_device_id { tag: CB_TAG_FRAMEBUFFER },
    coreboot_device_id { tag: 0 },
];

static FRAMEBUFFER_DRIVER: coreboot_driver = coreboot_driver {
    probe: Some(framebuffer_probe),
    drv: driver { name: b"framebuffer\0".as_ptr() as *const _ },
    id_table: FRAMEBUFFER_IDS.as_ptr(),
};

module_coreboot_driver!(FRAMEBUFFER_DRIVER);
module_author!(b"Samuel Holland <samuel@sholland.org>\0");
module_description!(b"Memory based framebuffer accessed through coreboot table\0");
module_license!(b"GPL\0");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
