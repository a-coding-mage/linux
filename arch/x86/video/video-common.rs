/*
 * Copyright (C) 2007 Antonino Daplas <adaplas@gmail.com>
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 *
 */

// Dependencies supplied by the surrounding kernel translation unit.

pub unsafe fn pgprot_framebuffer(
    mut prot: pgprot_t,
    _vm_start: ::core::primitive::c_ulong,
    _vm_end: ::core::primitive::c_ulong,
    _offset: ::core::primitive::c_ulong,
) -> pgprot_t {
    pgprot_val!(prot) &= !(_PAGE_CACHE_MASK);
    if boot_cpu_data.x86 > 3 {
        pgprot_val!(prot) |= cachemode2protval(_PAGE_CACHE_MODE_UC_MINUS);
    }

    prot
}

// EXPORT_SYMBOL(pgprot_framebuffer);

pub unsafe fn video_is_primary_device(dev: *mut device) -> bool {
    // CONFIG_SCREEN_INFO controls the declarations and resource matching
    // block below, as in the original conditional compilation.
    #[cfg(CONFIG_SCREEN_INFO)]
    {
        let si: *mut screen_info = &mut sysfb_primary_display.screen;
        let mut res: [resource; SCREEN_INFO_MAX_RESOURCES] =
            ::core::mem::MaybeUninit::uninit().assume_init();
        let numres: ssize_t = screen_info_resources(
            si,
            res.as_mut_ptr(),
            res.len(),
        );

        if numres > 0 {
            let mut i: ssize_t = 0;
            while i < numres {
                if (res[i as usize].flags & IORESOURCE_MEM) == 0 {
                    i += 1;
                    continue;
                }

                if !pci_find_resource(to_pci_dev(dev), &res[i as usize]) {
                    return true;
                }
                i += 1;
            }

            return false;
        }
    }

    /*
     * No framebuffer was set up by the firmware/bootloader, so fall back
     * to the default VGA device.
     */
    to_pci_dev(dev) == vga_default_device()
}

// EXPORT_SYMBOL(video_is_primary_device);

// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
