// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Generic System Framebuffers
 * Copyright (c) 2012-2013 David Herrmann <dh.herrmann@gmail.com>
 *
 * EFI Quirks Copyright (c) 2006 Edgar Hucek <gimli@dark-green.com>
 */

/* EFI Quirks: several EFI systems do not correctly advertise boot framebuffers. */

// Kernel dependencies supplied by other translation units.

const OVERRIDE_NONE: u32 = 0x0;
const OVERRIDE_BASE: u32 = 0x1;
const OVERRIDE_STRIDE: u32 = 0x2;
const OVERRIDE_HEIGHT: u32 = 0x4;
const OVERRIDE_WIDTH: u32 = 0x8;

#[repr(C)]
pub struct EfifbDmiInfo {
    pub optname: *const core::ffi::c_char,
    pub base: u64,
    pub stride: u32,
    pub width: u32,
    pub height: u32,
    pub flags: u32,
}

// M_* identifiers and struct screen_info are supplied by the kernel bindings.
pub static mut efifb_dmi_list: [EfifbDmiInfo; M_UNKNOWN as usize + 1] = [
    EfifbDmiInfo { optname: b"i17\0".as_ptr() as _, base: 0x80010000, stride: 1472 * 4, width: 1440, height: 900, flags: OVERRIDE_NONE },
    EfifbDmiInfo { optname: b"i20\0".as_ptr() as _, base: 0x80010000, stride: 1728 * 4, width: 1680, height: 1050, flags: OVERRIDE_NONE }, // guess
    EfifbDmiInfo { optname: b"imac7\0".as_ptr() as _, base: 0x40010000, stride: 1728 * 4, width: 1680, height: 1050, flags: OVERRIDE_NONE },
    EfifbDmiInfo { optname: b"i24\0".as_ptr() as _, base: 0x80010000, stride: 2048 * 4, width: 1920, height: 1200, flags: OVERRIDE_NONE }, // guess
    EfifbDmiInfo { optname: b"imac8\0".as_ptr() as _, base: 0xc0060000, stride: 2048 * 4, width: 1920, height: 1200, flags: OVERRIDE_NONE },
    EfifbDmiInfo { optname: b"imac10\0".as_ptr() as _, base: 0xc0010000, stride: 2048 * 4, width: 1920, height: 1080, flags: OVERRIDE_NONE },
    EfifbDmiInfo { optname: b"imac11\0".as_ptr() as _, base: 0xc0010000, stride: 2560 * 4, width: 2560, height: 1440, flags: OVERRIDE_NONE },
    EfifbDmiInfo { optname: b"mini\0".as_ptr() as _, base: 0x80000000, stride: 2048 * 4, width: 1024, height: 768, flags: OVERRIDE_NONE },
    EfifbDmiInfo { optname: b"mini31\0".as_ptr() as _, base: 0x40010000, stride: 1024 * 4, width: 1024, height: 768, flags: OVERRIDE_NONE },
    EfifbDmiInfo { optname: b"mini41\0".as_ptr() as _, base: 0xc0010000, stride: 2048 * 4, width: 1920, height: 1200, flags: OVERRIDE_NONE },
    EfifbDmiInfo { optname: b"macbook\0".as_ptr() as _, base: 0x80000000, stride: 2048 * 4, width: 1280, height: 800, flags: OVERRIDE_NONE },
    EfifbDmiInfo { optname: b"macbook51\0".as_ptr() as _, base: 0x80010000, stride: 2048 * 4, width: 1280, height: 800, flags: OVERRIDE_NONE },
    EfifbDmiInfo { optname: b"macbook61\0".as_ptr() as _, base: 0x80010000, stride: 2048 * 4, width: 1280, height: 800, flags: OVERRIDE_NONE },
    EfifbDmiInfo { optname: b"macbook71\0".as_ptr() as _, base: 0x80010000, stride: 2048 * 4, width: 1280, height: 800, flags: OVERRIDE_NONE },
    EfifbDmiInfo { optname: b"mba\0".as_ptr() as _, base: 0x80000000, stride: 2048 * 4, width: 1280, height: 800, flags: OVERRIDE_NONE },
    EfifbDmiInfo { optname: b"mba3\0".as_ptr() as _, base: 0, stride: 2048 * 4, width: 0, height: 0, flags: OVERRIDE_STRIDE }, // 11" Macbook Air 3,1 passes the wrong stride
    EfifbDmiInfo { optname: b"mbp\0".as_ptr() as _, base: 0x80010000, stride: 1472 * 4, width: 1440, height: 900, flags: OVERRIDE_NONE },
    EfifbDmiInfo { optname: b"mbp2\0".as_ptr() as _, base: 0, stride: 0, width: 0, height: 0, flags: OVERRIDE_NONE }, // placeholder
    EfifbDmiInfo { optname: b"mbp22\0".as_ptr() as _, base: 0x80010000, stride: 1472 * 4, width: 1440, height: 900, flags: OVERRIDE_NONE },
    EfifbDmiInfo { optname: b"mbp3\0".as_ptr() as _, base: 0x80030000, stride: 2048 * 4, width: 1440, height: 900, flags: OVERRIDE_NONE },
    EfifbDmiInfo { optname: b"mbp4\0".as_ptr() as _, base: 0xc0060000, stride: 2048 * 4, width: 1920, height: 1200, flags: OVERRIDE_NONE },
    EfifbDmiInfo { optname: b"mbp51\0".as_ptr() as _, base: 0xc0010000, stride: 2048 * 4, width: 1440, height: 900, flags: OVERRIDE_NONE },
    EfifbDmiInfo { optname: b"mbp52\0".as_ptr() as _, base: 0xc0010000, stride: 2048 * 4, width: 1920, height: 1200, flags: OVERRIDE_NONE },
    EfifbDmiInfo { optname: b"mbp53\0".as_ptr() as _, base: 0xd0010000, stride: 2048 * 4, width: 1440, height: 900, flags: OVERRIDE_NONE },
    EfifbDmiInfo { optname: b"mbp61\0".as_ptr() as _, base: 0x90030000, stride: 2048 * 4, width: 1920, height: 1200, flags: OVERRIDE_NONE },
    EfifbDmiInfo { optname: b"mbp62\0".as_ptr() as _, base: 0x90030000, stride: 2048 * 4, width: 1680, height: 1050, flags: OVERRIDE_NONE },
    EfifbDmiInfo { optname: b"mbp71\0".as_ptr() as _, base: 0xc0010000, stride: 2048 * 4, width: 1280, height: 800, flags: OVERRIDE_NONE },
    EfifbDmiInfo { optname: b"mbp82\0".as_ptr() as _, base: 0x90010000, stride: 1472 * 4, width: 1440, height: 900, flags: OVERRIDE_NONE },
    EfifbDmiInfo { optname: core::ptr::null(), base: 0, stride: 0, width: 0, height: 0, flags: OVERRIDE_NONE },
];

pub unsafe fn efifb_setup_from_dmi(si: *mut screen_info, opt: *const core::ffi::c_char) {
    let mut i = 0;
    while i < M_UNKNOWN {
        let info = &efifb_dmi_list[i as usize];
        if info.base != 0 && strcmp(opt, info.optname) == 0 {
            (*si).lfb_base = info.base;
            (*si).lfb_linelength = info.stride;
            (*si).lfb_width = info.width;
            (*si).lfb_height = info.height;
        }
        i += 1;
    }
}

#[inline]
unsafe fn choose_value<T: Copy + PartialEq + From<u8>>(dmivalue: T, fwvalue: T, field: u32, flags: u32) -> T {
    if flags & field != 0 || fwvalue == T::from(0) { dmivalue } else { fwvalue }
}

#[cfg(feature = "CONFIG_EFI")]
pub unsafe fn efifb_set_system(si: *mut screen_info, id: *const dmi_system_id) -> i32 {
    let info = (*id).driver_data as *const EfifbDmiInfo;
    if (*info).base == 0 && (*info).height == 0 && (*info).width == 0 && (*info).stride == 0 { return 0; }
    if (*si).lfb_base == 0 && (*info).base != 0 {
        (*si).lfb_base = choose_value((*info).base, (*si).lfb_base, OVERRIDE_BASE, (*info).flags);
    }
    if (*si).lfb_base != 0 {
        (*si).lfb_linelength = choose_value((*info).stride, (*si).lfb_linelength, OVERRIDE_STRIDE, (*info).flags);
        (*si).lfb_width = choose_value((*info).width, (*si).lfb_width, OVERRIDE_WIDTH, (*info).flags);
        (*si).lfb_height = choose_value((*info).height, (*si).lfb_height, OVERRIDE_HEIGHT, (*info).flags);
        if (*si).orig_video_isVGA == 0 { (*si).orig_video_isVGA = VIDEO_TYPE_EFI; }
    } else {
        (*si).lfb_linelength = 0; (*si).lfb_width = 0; (*si).lfb_height = 0; (*si).orig_video_isVGA = 0; return 0;
    }
    printk(KERN_INFO, b"efifb: dmi detected %s - framebuffer at 0x%08x (%dx%d, stride %d)\n\0".as_ptr() as _, (*id).ident, (*si).lfb_base, (*si).lfb_width, (*si).lfb_height, (*si).lfb_linelength);
    1
}

#[repr(C)]
pub struct EfifbModeFixup { pub width: u32, pub height: u32, pub linelength: u32 }

pub unsafe fn efifb_swap_width_height(_id: *const dmi_system_id) -> i32 {
    let si = &mut sysfb_primary_display.screen;
    let bpp = __screen_info_lfb_bits_per_pixel(si);
    core::mem::swap(&mut si.lfb_width, &mut si.lfb_height);
    si.lfb_linelength = bpp * si.lfb_width / BITS_PER_BYTE;
    1
}

pub unsafe fn efifb_check_and_swap_width_height(id: *const dmi_system_id) -> i32 {
    let data = (*id).driver_data as *const EfifbModeFixup;
    let si = &mut sysfb_primary_display.screen;
    if (*data).width == si.lfb_width && (*data).height == si.lfb_height {
        core::mem::swap(&mut si.lfb_width, &mut si.lfb_height);
        si.lfb_linelength = (*data).linelength;
        si.lfb_size = (*data).linelength * (*data).width;
    }
    1
}

pub static efifb_steamdeck_mode_fixup: EfifbModeFixup = EfifbModeFixup { width: 1280, height: 800, linelength: 3328 };

pub unsafe fn efifb_overlaps_pci_range(si: *const screen_info, range: *const of_pci_range) -> bool {
    let mut fb_base = (*si).lfb_base;
    if (*si).capabilities & VIDEO_CAPABILITY_64BIT_BASE != 0 { fb_base |= ((*si).ext_lfb_base as u64) << 32; }
    fb_base >= (*range).cpu_addr && fb_base < (*range).cpu_addr + (*range).size
}

// The DMI tables and device-tree/PCI iteration macros are represented by the
// corresponding kernel bindings in the final integration.
pub unsafe fn find_pci_overlap_node() -> *mut device_node { core::ptr::null_mut() }

pub unsafe fn efifb_add_links(fwnode: *mut fwnode_handle) -> i32 {
    let sup_np = find_pci_overlap_node();
    if sup_np.is_null() { return 0; }
    fwnode_link_add(fwnode, of_fwnode_handle(sup_np), 0);
    of_node_put(sup_np);
    0
}

#[cfg(feature = "CONFIG_EFI")]
pub unsafe fn sysfb_apply_efi_quirks(si: *mut screen_info) {
    if (*si).orig_video_isVGA != VIDEO_TYPE_EFI || (*si).capabilities & VIDEO_CAPABILITY_SKIP_QUIRKS == 0 { dmi_check_system(efifb_dmi_system_table.as_ptr()); }
    if (*si).orig_video_isVGA == VIDEO_TYPE_EFI { dmi_check_system(efifb_dmi_swap_width_height.as_ptr()); }
}

#[cfg(feature = "CONFIG_EFI")]
pub unsafe fn sysfb_set_efifb_fwnode(si: *const screen_info, pd: *mut platform_device) {
    if (*si).orig_video_isVGA == VIDEO_TYPE_EFI && IS_ENABLED_CONFIG_PCI { fwnode_init(&mut efifb_fwnode, &efifb_fwnode_ops); (*pd).dev.fwnode = &mut efifb_fwnode; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
