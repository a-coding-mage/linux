// SPDX-License-Identifier: GPL-2.0-only
/*
 * init/noinitramfs.c
 *
 * Copyright (C) 2006, NXP Semiconductors, All Rights Reserved
 * Author: Jean-Paul Saman <jean-paul.saman@nxp.com>
 */

//! Default root filesystem creation when no initramfs support is configured.

#[allow(
    clippy::all,
    dead_code,
    missing_docs,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    improper_ctypes,
    unreachable_pub,
    unsafe_op_in_unsafe_fn
)]
mod bindings {
    use kernel::ffi;

    include!(concat!(
        env!("OBJTREE"),
        "/rust/bindings/init_noinitramfs_generated.rs"
    ));
}

mod main_printk;

use kernel::ffi::c_int;
use main_printk::main_printk;

#[link_section = ".init.text"]
#[cfg_attr(
    not(all(CONFIG_LTO_CLANG, CONFIG_HAVE_ARCH_PREL32_RELOCATIONS)),
    linkage = "internal"
)]
#[cfg_attr(
    all(CONFIG_LTO_CLANG, CONFIG_HAVE_ARCH_PREL32_RELOCATIONS),
    export_name = "__initstub__kmod_noinitramfs__0_42_default_rootfsrootfs"
)]
unsafe extern "C" fn default_rootfs() -> c_int {
    // SAFETY: the rootfs initcall runs during serialized boot. These are the
    // original usermode-helper state transition and init syscall interfaces.
    unsafe {
        bindings::__usermodehelper_set_disable_depth(bindings::umh_disable_depth_UMH_ENABLED);
        let mut error = bindings::init_mkdir(c"/dev".as_ptr().cast(), 0o755);
        if error >= 0 {
            // Inline MKDEV and new_encode_dev with their canonical unsigned
            // dev_t widths; neither C inline defines an external symbol.
            let device: bindings::dev_t = (5 << bindings::MINORBITS) | 1;
            let major = device >> bindings::MINORBITS;
            let minor = device & bindings::MINORMASK;
            let encoded = (minor & 0xff) | (major << 8) | ((minor & !0xff) << 12);
            error = bindings::init_mknod(
                c"/dev/console".as_ptr().cast(),
                (bindings::S_IFCHR | bindings::S_IRUSR | bindings::S_IWUSR) as bindings::umode_t,
                encoded,
            );
            if error >= 0 {
                error = bindings::init_mkdir(c"/root".as_ptr().cast(), 0o700);
            }
        }
        if error < 0 {
            main_printk!("default_rootfs", b"\x014Failed to create a rootfs\n\0");
            error
        } else {
            0
        }
    }
}

// Preserve rootfs_initcall's original source identity, ordering and relocation
// representation. Its callback and record are reclaimed with init memory.
#[cfg(CONFIG_HAVE_ARCH_PREL32_RELOCATIONS)]
#[used]
#[link_section = ".discard.addressable"]
static ADDRESSABLE: unsafe extern "C" fn() -> c_int = default_rootfs;
#[cfg(all(CONFIG_HAVE_ARCH_PREL32_RELOCATIONS, not(CONFIG_LTO_CLANG)))]
core::arch::global_asm!(
    ".pushsection .initcallrootfs.init,\"a\"",
    "__initcall__kmod_noinitramfs__0_42_default_rootfsrootfs:",
    ".long {callback} - .",
    ".popsection",
    callback = sym default_rootfs,
);
#[cfg(all(CONFIG_HAVE_ARCH_PREL32_RELOCATIONS, CONFIG_LTO_CLANG))]
core::arch::global_asm!(
    ".pushsection .initcallrootfs.init..kmod_noinitramfs__0_42_default_rootfs,\"a\"",
    "__initcall__kmod_noinitramfs__0_42_default_rootfsrootfs:",
    ".long {callback} - .",
    ".popsection",
    callback = sym default_rootfs,
);

#[cfg(not(CONFIG_HAVE_ARCH_PREL32_RELOCATIONS))]
#[used]
#[linkage = "internal"]
#[export_name = "__initcall__kmod_noinitramfs__0_42_default_rootfsrootfs"]
#[cfg_attr(not(CONFIG_LTO_CLANG), link_section = ".initcallrootfs.init")]
#[cfg_attr(
    CONFIG_LTO_CLANG,
    link_section = ".initcallrootfs.init..kmod_noinitramfs__0_42_default_rootfs"
)]
static mut INITCALL: unsafe extern "C" fn() -> c_int = default_rootfs;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
