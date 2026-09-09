// SPDX-License-Identifier: GPL-2.0

// Dependency supplied by <generated/utsrelease.h>.
unsafe extern "C" {
    pub static UTS_RELEASE: *const core::ffi::c_char;
}

// WARNING userspace tools like batctl were relying on
// /sys/module/batman_adv/version to check if the module was loaded. If it
// isn't present, they usually error out before finishing setup of the batadv
// interface. It should be kept until it is unlikely that there are active
// installations of these "broken" versions of these tools with recent kernels.
//
// Equivalent of MODULE_VERSION(UTS_RELEASE): expose the kernel module version
// metadata using the release string supplied by the build environment.
pub unsafe fn module_version() -> *const core::ffi::c_char {
    UTS_RELEASE
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
