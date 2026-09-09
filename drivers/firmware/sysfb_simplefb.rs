// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Generic System Framebuffers
 * Copyright (c) 2012-2013 David Herrmann <dh.herrmann@gmail.com>
 */

/*
 * simple-framebuffer probing
 * Try to convert "screen_info" into a "simple-framebuffer" compatible mode.
 * If the mode is incompatible, we return "false" and let the caller create
 * legacy nodes instead.
 */

// Linux kernel dependencies and build-time definitions are supplied externally.

static simplefb_resname: &[u8] = b"BOOTFB\0";
static formats: [simplefb_format; ARRAY_SIZE(SIMPLEFB_FORMATS)] = SIMPLEFB_FORMATS;

/* try parsing screen_info into a simple-framebuffer mode struct */
#[inline]
pub unsafe fn sysfb_parse_mode(
    si: *const screen_info,
    mode: *mut simplefb_platform_data,
) -> bool {
    let r#type: __u8;
    let bits_per_pixel: u32;

    r#type = (*si).orig_video_isVGA;
    if r#type != VIDEO_TYPE_VLFB && r#type != VIDEO_TYPE_EFI {
        return false;
    }

    bits_per_pixel = __screen_info_lfb_bits_per_pixel(si);

    let mut i = 0;
    while i < formats.len() {
        let f: *const simplefb_format = &formats[i];

        if (*f).transp.length != 0 {
            i += 1;
            continue; /* transparent formats are unsupported by VESA/EFI */
        }

        if bits_per_pixel == (*f).bits_per_pixel
            && (*si).red_size == (*f).red.length
            && (*si).red_pos == (*f).red.offset
            && (*si).green_size == (*f).green.length
            && (*si).green_pos == (*f).green.offset
            && (*si).blue_size == (*f).blue.length
            && (*si).blue_pos == (*f).blue.offset
        {
            (*mode).format = (*f).name;
            (*mode).width = (*si).lfb_width;
            (*mode).height = (*si).lfb_height;
            (*mode).stride = (*si).lfb_linelength;
            return true;
        }

        i += 1;
    }

    false
}

#[inline]
pub unsafe fn sysfb_create_simplefb(
    si: *const screen_info,
    mode: *const simplefb_platform_data,
    parent: *mut device,
) -> *mut platform_device {
    let pd: *mut platform_device;
    let mut res: resource = core::mem::zeroed();
    let mut base: u64;
    let mut size: u64;
    let mut length: u32;
    let mut ret: i32;

    /*
     * If the 64BIT_BASE capability is set, ext_lfb_base will contain the
     * upper half of the base address. Assemble the address, then make sure
     * it is valid and we can actually access it.
     */
    base = (*si).lfb_base as u64;
    if (*si).capabilities & VIDEO_CAPABILITY_64BIT_BASE != 0 {
        base |= ((*si).ext_lfb_base as u64) << 32;
    }
    if base == 0 || (base as resource_size_t as u64) != base {
        printk!(KERN_DEBUG, "sysfb: inaccessible VRAM base\n");
        return ERR_PTR(-EINVAL);
    }

    /*
     * Don't use lfb_size as IORESOURCE size, since it may contain the
     * entire VMEM, and thus require huge mappings. Use just the part we
     * need, that is, the part where the framebuffer is located. But verify
     * that it does not exceed the advertised VMEM.
     * Note that in case of VBE, the lfb_size is shifted by 16 bits for
     * historical reasons.
     */
    size = (*si).lfb_size as u64;
    if (*si).orig_video_isVGA == VIDEO_TYPE_VLFB {
        size <<= 16;
    }
    length = (*mode).height * (*mode).stride;
    if (length as u64 > size) {
        printk!(KERN_WARNING, "sysfb: VRAM smaller than advertised\n");
        return ERR_PTR(-EINVAL);
    }
    length = PAGE_ALIGN(length);

    /* setup IORESOURCE_MEM as framebuffer memory */
    res.flags = IORESOURCE_MEM;
    res.name = simplefb_resname.as_ptr() as *const _;
    res.start = base;
    res.end = res.start + length as u64 - 1;
    if res.end <= res.start {
        return ERR_PTR(-EINVAL);
    }

    pd = platform_device_alloc(b"simple-framebuffer\0".as_ptr() as *const _, 0);
    if pd.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    (*pd).dev.parent = parent;

    sysfb_set_efifb_fwnode(si, pd);

    ret = platform_device_add_resources(pd, &res, 1);
    if ret != 0 {
        goto err_put_device;
    }

    ret = platform_device_add_data(pd, mode as *const _, core::mem::size_of::<simplefb_platform_data>());
    if ret != 0 {
        goto err_put_device;
    }

    ret = platform_device_add(pd);
    if ret != 0 {
        goto err_put_device;
    }

    return pd;

err_put_device:
    platform_device_put(pd);
    ERR_PTR(ret)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
