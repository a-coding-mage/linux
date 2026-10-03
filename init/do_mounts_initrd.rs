// SPDX-License-Identifier: GPL-2.0
//! Original initrd address ownership, boot options and image loading sequence.

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
        "/rust/bindings/init_mounts_generated.rs"
    ));
}

mod main_printk;
mod main_setup;

use core::ptr;
use kernel::ffi::{c_char, c_int, c_ulong};
use main_printk::main_printk;

/// Virtual start of the initial RAM disk, also consumed by architecture setup.
#[no_mangle]
#[link_section = ".bss"]
pub static mut initrd_start: c_ulong = 0;
/// Virtual end of the initial RAM disk.
#[no_mangle]
#[link_section = ".bss"]
pub static mut initrd_end: c_ulong = 0;
/// Whether the initial RAM disk may lie below the normal memory start.
#[no_mangle]
#[link_section = ".bss"]
pub static mut initrd_below_start_ok: c_int = 0;

#[link_section = ".init.data"]
static mut MOUNT_INITRD: c_int = 1;

/// Physical initial RAM disk address used before its virtual mapping exists.
#[no_mangle]
#[link_section = ".init.data"]
pub static mut phys_initrd_start: bindings::phys_addr_t = 0;
/// Size of the physical initial RAM disk.
#[no_mangle]
#[link_section = ".init.data"]
pub static mut phys_initrd_size: c_ulong = 0;

#[link_section = ".init.text"]
unsafe extern "C" fn no_initrd(_argument: *mut c_char) -> c_int {
    // SAFETY: boot-option parsing serializes this init-only flag and printk
    // consumes a constant, NUL-terminated format without variadic arguments.
    unsafe {
        main_printk!(
            "no_initrd",
            b"\x014noinitrd option is deprecated and will be removed soon\n\0"
        );
        MOUNT_INITRD = 0;
    }
    1
}

#[link_section = ".init.text"]
unsafe extern "C" fn early_initrdmem(argument: *mut c_char) -> c_int {
    let mut end = core::mem::MaybeUninit::<*mut c_char>::uninit();
    // SAFETY: the early parser provides the same live C option string as the
    // original callback; memparse always initializes its end-pointer output.
    unsafe {
        let start = bindings::memparse(argument, end.as_mut_ptr()) as bindings::phys_addr_t;
        let end = end.assume_init();
        if end.read() == b',' as c_char {
            let size = bindings::memparse(end.add(1), ptr::null_mut()) as c_ulong;
            phys_initrd_start = start;
            phys_initrd_size = size;
        }
    }
    0
}

#[link_section = ".init.text"]
unsafe extern "C" fn early_initrd(argument: *mut c_char) -> c_int {
    // SAFETY: this alias forwards the original early option parser contract.
    unsafe { early_initrdmem(argument) }
}

main_setup::setup_param!("noinitrd", NOINITRD_SETUP, Some(no_initrd), 0);
main_setup::setup_param!("initrdmem", INITRDMEM_SETUP, Some(early_initrdmem), 1);
main_setup::setup_param!("initrd", INITRD_SETUP, Some(early_initrd), 1);

/// Original do_mounts.h create_dev, including the ignored unlink result and
/// the kernel's unsigned new_encode_dev representation.
#[link_section = ".init.text"]
unsafe fn create_dev(name: *const c_char, device: bindings::dev_t) -> c_int {
    let major = device >> bindings::MINORBITS;
    let minor = device & bindings::MINORMASK;
    let encoded = (minor & 0xff) | (major << 8) | ((minor & !0xff) << 12);
    // SAFETY: the boot caller supplies a live pathname and serialized init
    // filesystem context; these are the actual init syscall interfaces.
    unsafe {
        bindings::init_unlink(name);
        bindings::init_mknod(
            name,
            (bindings::S_IFBLK | 0o600) as bindings::umode_t,
            encoded,
        )
    }
}

/// Load the legacy initial RAM disk and remove its temporary input pathname.
///
/// # Safety
/// Called during serialized namespace setup with the init filesystem context;
/// all init-only option state and optional RAM disk support are still live.
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn initrd_load() {
    // SAFETY: the caller provides the same boot/filesystem phase as C.
    unsafe {
        if MOUNT_INITRD != 0 {
            create_dev(
                c"/dev/ram".as_ptr().cast(),
                bindings::Root_RAM0 as bindings::dev_t,
            );
            #[cfg(CONFIG_BLK_DEV_RAM = "y")]
            let loaded = bindings::rd_load_image();
            #[cfg(not(CONFIG_BLK_DEV_RAM = "y"))]
            let loaded = 0;
            // C retains this printk index record even when its inline
            // rd_load_image fallback makes the logging branch unreachable.
            if loaded != 0 {
                main_printk!(
                    "initrd_load",
                    concat!(
                    "\x014using deprecated initrd support, will be removed in January 2027; ",
                    "use initramfs instead or (as a last resort) /sys/firmware/initrd; ",
                    "see section \"Workaround\" in ",
                    "https://lore.kernel.org/lkml/20251010094047.3111495-1-safinaskar@gmail.com\n\0"
                )
                    .as_bytes()
                );
            }
        }
        bindings::init_unlink(c"/initrd.image".as_ptr().cast());
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
