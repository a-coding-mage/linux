/* SPDX-License-Identifier: GPL-2.0 */
/*
 * mediabay.h: definitions for using the media bay
 * on PowerBook 3400 and similar computers.
 *
 * Copyright (C) 1997 Paul Mackerras.
 */

/* The original declarations are available only when building the kernel. */

pub const MB_FD: ::core::ffi::c_int = 0; /* media bay contains floppy drive (automatic eject ?) */
pub const MB_FD1: ::core::ffi::c_int = 1; /* media bay contains floppy drive (manual eject ?) */
pub const MB_SOUND: ::core::ffi::c_int = 2; /* sound device ? */
pub const MB_CD: ::core::ffi::c_int = 3; /* media bay contains ATA drive such as CD or ZIP */
pub const MB_PCI: ::core::ffi::c_int = 5; /* media bay contains a PCI device */
pub const MB_POWER: ::core::ffi::c_int = 6; /* media bay contains a Power device (???) */
pub const MB_NO: ::core::ffi::c_int = 7; /* media bay contains nothing */

#[repr(C)]
pub struct macio_dev {
    _private: [u8; 0],
}

/* CONFIG_PMAC_MEDIABAY selects the external implementations below. */
#[cfg(feature = "CONFIG_PMAC_MEDIABAY")]
extern "C" {
    /* Check the content type of the bay, returns MB_NO if the bay is still
     * transitionning
     */
    pub fn check_media_bay(bay: *mut macio_dev) -> ::core::ffi::c_int;

    /* The ATA driver uses the calls below to temporarily hold on the
     * media bay callbacks while initializing the interface
     */
    pub fn lock_media_bay(bay: *mut macio_dev);
    pub fn unlock_media_bay(bay: *mut macio_dev);
}

/* When CONFIG_PMAC_MEDIABAY is not enabled, preserve the C inline fallbacks. */
#[cfg(not(feature = "CONFIG_PMAC_MEDIABAY"))]
#[inline]
pub unsafe fn check_media_bay(_bay: *mut macio_dev) -> ::core::ffi::c_int {
    MB_NO
}

#[cfg(not(feature = "CONFIG_PMAC_MEDIABAY"))]
#[inline]
pub unsafe fn lock_media_bay(_bay: *mut macio_dev) {}

#[cfg(not(feature = "CONFIG_PMAC_MEDIABAY"))]
#[inline]
pub unsafe fn unlock_media_bay(_bay: *mut macio_dev) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
