// SPDX-License-Identifier: GPL-2.0
/*
 *    Filename: cfag12864bfb.c
 *     Version: 0.1.0
 * Description: cfag12864b LCD framebuffer driver
 *     Depends: cfag12864b
 *
 *      Author: Copyright (C) Miguel Ojeda <ojeda@kernel.org>
 *        Date: 2006-10-31
 */

// Linux kernel headers supplied by the surrounding translation unit.

const CFAG12864BFB_NAME: *const u8 = b"cfag12864bfb\0".as_ptr();

static CFAG12864BFB_FIX: fb_fix_screeninfo = fb_fix_screeninfo {
    id: *b"cfag12864b\0",
    type_: FB_TYPE_PACKED_PIXELS,
    visual: FB_VISUAL_MONO10,
    xpanstep: 0,
    ypanstep: 0,
    ywrapstep: 0,
    line_length: CFAG12864B_WIDTH / 8,
    accel: FB_ACCEL_NONE,
};

static CFAG12864BFB_VAR: fb_var_screeninfo = fb_var_screeninfo {
    xres: CFAG12864B_WIDTH,
    yres: CFAG12864B_HEIGHT,
    xres_virtual: CFAG12864B_WIDTH,
    yres_virtual: CFAG12864B_HEIGHT,
    bits_per_pixel: 1,
    red: fb_bitfield { offset: 0, length: 1, msb_right: 0 },
    green: fb_bitfield { offset: 0, length: 1, msb_right: 0 },
    blue: fb_bitfield { offset: 0, length: 1, msb_right: 0 },
    left_margin: 0,
    right_margin: 0,
    upper_margin: 0,
    lower_margin: 0,
    vmode: FB_VMODE_NONINTERLACED,
};

unsafe extern "C" fn cfag12864bfb_mmap(
    _info: *mut fb_info,
    vma: *mut vm_area_struct,
) -> c_int {
    let pages: *mut page = virt_to_page(cfag12864b_buffer);

    (*vma).vm_page_prot = pgprot_decrypted((*vma).vm_page_prot);

    vm_map_pages_zero(vma, &pages, 1)
}

static CFAG12864BFB_OPS: fb_ops = fb_ops {
    owner: THIS_MODULE,
    // __FB_DEFAULT_SYSMEM_OPS_RDWR
    // __FB_DEFAULT_SYSMEM_OPS_DRAW
    fb_mmap: Some(cfag12864bfb_mmap),
};

unsafe extern "C" fn cfag12864bfb_probe(device: *mut platform_device) -> c_int {
    let mut ret: c_int = -EINVAL;
    let info: *mut fb_info = framebuffer_alloc(0, &mut (*device).dev);

    if info.is_null() {
        return ret;
    }

    (*info).flags = FBINFO_VIRTFB;
    (*info).screen_buffer = cfag12864b_buffer;
    (*info).screen_size = CFAG12864B_SIZE;
    (*info).fbops = &CFAG12864BFB_OPS;
    (*info).fix = CFAG12864BFB_FIX;
    (*info).var = CFAG12864BFB_VAR;
    (*info).pseudo_palette = core::ptr::null_mut();
    (*info).par = core::ptr::null_mut();

    if register_framebuffer(info) < 0 {
        framebuffer_release(info);
        return ret;
    }

    platform_set_drvdata(device, info);

    fb_info(info, b"%s frame buffer device\n\0".as_ptr(), (*info).fix.id.as_ptr());

    0
}

unsafe extern "C" fn cfag12864bfb_remove(device: *mut platform_device) {
    let info: *mut fb_info = platform_get_drvdata(device);

    if !info.is_null() {
        unregister_framebuffer(info);
        framebuffer_release(info);
    }
}

static mut CFAG12864BFB_DRIVER: platform_driver = platform_driver {
    probe: Some(cfag12864bfb_probe),
    remove: Some(cfag12864bfb_remove),
    driver: driver {
        name: CFAG12864BFB_NAME,
    },
};

static mut CFAG12864BFB_DEVICE: *mut platform_device = core::ptr::null_mut();

unsafe extern "C" fn cfag12864bfb_init() -> c_int {
    let mut ret: c_int = -EINVAL;

    /* cfag12864b_init() must be called first */
    if !cfag12864b_isinited() {
        printk(KERN_ERR, CFAG12864BFB_NAME, b": ERROR: cfag12864b is not initialized\n\0".as_ptr());
        return ret;
    }

    if cfag12864b_enable() != 0 {
        printk(KERN_ERR, CFAG12864BFB_NAME, b": ERROR: can't enable cfag12864b refreshing (being used)\n\0".as_ptr());
        return -ENODEV;
    }

    ret = platform_driver_register(&raw mut CFAG12864BFB_DRIVER);

    if ret == 0 {
        CFAG12864BFB_DEVICE = platform_device_alloc(CFAG12864BFB_NAME, 0);

        if !CFAG12864BFB_DEVICE.is_null() {
            ret = platform_device_add(CFAG12864BFB_DEVICE);
        } else {
            ret = -ENOMEM;
        }

        if ret != 0 {
            platform_device_put(CFAG12864BFB_DEVICE);
            platform_driver_unregister(&raw mut CFAG12864BFB_DRIVER);
        }
    }

    ret
}

unsafe extern "C" fn cfag12864bfb_exit() {
    platform_device_unregister(CFAG12864BFB_DEVICE);
    platform_driver_unregister(&raw mut CFAG12864BFB_DRIVER);
    cfag12864b_disable();
}

// module_init(cfag12864bfb_init);
// module_exit(cfag12864bfb_exit);
// MODULE_LICENSE("GPL v2");
// MODULE_AUTHOR("Miguel Ojeda <ojeda@kernel.org>");
// MODULE_DESCRIPTION("cfag12864b LCD framebuffer driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
