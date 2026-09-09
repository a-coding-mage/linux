/* SPDX-License-Identifier: GPL-2.0 */

// Dependency declarations supplied by the corresponding Linux UAPI/kernel headers.

pub const SCREEN_INFO_MAX_RESOURCES: usize = 3;

#[repr(C)]
pub struct pci_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pixel_format {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    _private: [u8; 0],
}

#[inline]
pub fn __screen_info_has_lfb(type_: ::core::ffi::c_uint) -> bool {
    (type_ == VIDEO_TYPE_VLFB) || (type_ == VIDEO_TYPE_EFI)
}

#[inline]
pub unsafe fn __screen_info_lfb_base(si: *const screen_info) -> u64 {
    let mut lfb_base = (*si).lfb_base as u64;

    if (*si).capabilities & VIDEO_CAPABILITY_64BIT_BASE != 0 {
        lfb_base |= ((*si).ext_lfb_base as u64) << 32;
    }

    lfb_base
}

#[inline]
pub unsafe fn __screen_info_set_lfb_base(si: *mut screen_info, lfb_base: u64) {
    (*si).lfb_base = lfb_base & GENMASK_ULL(31, 0);
    (*si).ext_lfb_base = (lfb_base & GENMASK_ULL(63, 32)) >> 32;

    if (*si).ext_lfb_base != 0 {
        (*si).capabilities |= VIDEO_CAPABILITY_64BIT_BASE;
    } else {
        (*si).capabilities &= !VIDEO_CAPABILITY_64BIT_BASE;
    }
}

#[inline]
pub unsafe fn __screen_info_lfb_size(si: *const screen_info, type_: ::core::ffi::c_uint) -> u64 {
    let mut lfb_size = (*si).lfb_size as u64;

    if type_ == VIDEO_TYPE_VLFB {
        lfb_size <<= 16;
    }
    lfb_size
}

#[inline]
pub unsafe fn __screen_info_vbe_mode_nonvga(si: *const screen_info) -> bool {
    /*
     * VESA modes typically run on VGA hardware. Set bit 5 signals that this
     * is not the case. Drivers can then not make use of VGA resources. See
     * Sec 4.4 of the VBE 2.0 spec.
     */
    (*si).vesa_attributes & BIT(5) != 0
}

#[inline]
pub fn __screen_info_video_type(type_: ::core::ffi::c_uint) -> ::core::ffi::c_uint {
    match type_ {
        VIDEO_TYPE_MDA
        | VIDEO_TYPE_CGA
        | VIDEO_TYPE_EGAM
        | VIDEO_TYPE_EGAC
        | VIDEO_TYPE_VGAC
        | VIDEO_TYPE_VLFB
        | VIDEO_TYPE_PICA_S3
        | VIDEO_TYPE_MIPS_G364
        | VIDEO_TYPE_SGI
        | VIDEO_TYPE_TGAC
        | VIDEO_TYPE_SUN
        | VIDEO_TYPE_SUNPCI
        | VIDEO_TYPE_PMAC
        | VIDEO_TYPE_EFI => type_,
        _ => 0,
    }
}

#[inline]
pub unsafe fn screen_info_video_type(si: *const screen_info) -> ::core::ffi::c_uint {
    let type_: ::core::ffi::c_uint;

    // check if display output is on
    if (*si).orig_video_isVGA == 0 {
        return 0;
    }

    // check for a known VIDEO_TYPE_ constant
    type_ = __screen_info_video_type((*si).orig_video_isVGA);
    if type_ != 0 {
        return (*si).orig_video_isVGA;
    }

    // check if text mode has been initialized
    if (*si).orig_video_lines == 0 || (*si).orig_video_cols == 0 {
        return 0;
    }

    // 80x25 text, mono
    if (*si).orig_video_mode == 0x07 {
        if ((*si).orig_video_ega_bx & 0xff) != 0x10 {
            return VIDEO_TYPE_EGAM;
        } else {
            return VIDEO_TYPE_MDA;
        }
    }

    // EGA/VGA, 16 colors
    if ((*si).orig_video_ega_bx & 0xff) != 0x10 {
        if (*si).orig_video_isVGA != 0 {
            return VIDEO_TYPE_VGAC;
        } else {
            return VIDEO_TYPE_EGAC;
        }
    }

    // the rest...
    VIDEO_TYPE_CGA
}

#[inline]
pub unsafe fn __screen_info_vesapm_info_base(si: *const screen_info) -> u32 {
    if (*si).vesapm_seg < 0xc000 {
        return 0;
    }
    ((*si).vesapm_seg << 4) + (*si).vesapm_off
}

extern "C" {
    pub fn screen_info_resources(
        si: *const screen_info,
        r: *mut resource,
        num: usize,
    ) -> isize;

    pub fn __screen_info_lfb_bits_per_pixel(si: *const screen_info) -> u32;
    pub fn screen_info_pixel_format(si: *const screen_info, f: *mut pixel_format) -> ::core::ffi::c_int;

    #[cfg(CONFIG_PCI)]
    pub fn screen_info_apply_fixups();
    #[cfg(CONFIG_PCI)]
    pub fn screen_info_pci_dev(si: *const screen_info) -> *mut pci_dev;
}

#[cfg(not(CONFIG_PCI))]
#[inline]
pub fn screen_info_apply_fixups() {}

#[cfg(not(CONFIG_PCI))]
#[inline]
pub fn screen_info_pci_dev(_si: *const screen_info) -> *mut pci_dev {
    ::core::ptr::null_mut()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
