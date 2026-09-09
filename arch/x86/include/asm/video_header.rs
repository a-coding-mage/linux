/* SPDX-License-Identifier: GPL-2.0 */

// Dependency provided by <linux/types.h>.
// Dependency provided by <asm/page.h>.

pub struct device;

unsafe extern "C" {
    pub fn pgprot_framebuffer(
        prot: pgprot_t,
        vm_start: ::core::ffi::c_ulong,
        vm_end: ::core::ffi::c_ulong,
        offset: ::core::ffi::c_ulong,
    ) -> pgprot_t;
}

// #define pgprot_framebuffer pgprot_framebuffer

// Preserved from the source: this declaration is available only when
// CONFIG_VIDEO is enabled.
#[cfg(CONFIG_VIDEO)]
unsafe extern "C" {
    pub fn video_is_primary_device(dev: *mut device) -> bool;
}

// #define video_is_primary_device video_is_primary_device

// Declarations from <asm-generic/video.h> are supplied by another translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
