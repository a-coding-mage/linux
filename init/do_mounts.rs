// SPDX-License-Identifier: GPL-2.0-only
//! Root device discovery, filesystem selection, and the boot mount namespace.

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
        "/rust/bindings/init_root_mounts_generated.rs"
    ));
}

mod main_printk;
mod main_setup;

use core::ptr;
use kernel::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use main_printk::main_printk;

/// Flags used by the original root-mount and initrd interfaces.
#[no_mangle]
pub static mut root_mountflags: c_int = (bindings::MS_RDONLY | bindings::MS_SILENT) as c_int;
#[link_section = ".init.data"]
static mut SAVED_ROOT_NAME: [c_char; 64] = [0; 64];
static mut ROOT_WAIT: c_int = 0;

/// Root device selected by architecture setup or the root= boot option.
#[no_mangle]
pub static mut ROOT_DEV: bindings::dev_t = 0;

#[link_section = ".init.data"]
static mut ROOT_MOUNT_DATA: *mut c_char = ptr::null_mut();
#[link_section = ".init.data"]
static mut ROOT_FS_NAMES: *mut c_char = ptr::null_mut();
#[link_section = ".init.data"]
static mut ROOT_DELAY: c_uint = 0;

#[link_section = ".init.text"]
unsafe extern "C" fn readonly(argument: *mut c_char) -> c_int {
    // SAFETY: setup parsing supplies a live NUL-terminated option suffix and
    // serializes updates to the boot flags.
    unsafe {
        if argument.read() != 0 {
            return 0;
        }
        root_mountflags |= bindings::MS_RDONLY as c_int;
    }
    1
}

#[link_section = ".init.text"]
unsafe extern "C" fn readwrite(argument: *mut c_char) -> c_int {
    // SAFETY: same serialized setup parser contract as readonly.
    unsafe {
        if argument.read() != 0 {
            return 0;
        }
        root_mountflags &= !(bindings::MS_RDONLY as c_int);
    }
    1
}

#[link_section = ".init.text"]
unsafe extern "C" fn root_dev_setup(argument: *mut c_char) -> c_int {
    // SAFETY: the source is a setup string; the destination is the entire
    // init-only array. sized_strscpy is strscpy's actual external interface.
    unsafe {
        bindings::sized_strscpy(ptr::addr_of_mut!(SAVED_ROOT_NAME).cast(), argument, 64);
    }
    1
}

#[link_section = ".init.text"]
unsafe extern "C" fn rootwait_setup(argument: *mut c_char) -> c_int {
    // SAFETY: option parsing owns the argument and boot state.
    unsafe {
        if argument.read() != 0 {
            return 0;
        }
        ROOT_WAIT = -1;
    }
    1
}

#[link_section = ".init.text"]
unsafe extern "C" fn rootwait_timeout_setup(argument: *mut c_char) -> c_int {
    let mut seconds: c_int = 0;
    // SAFETY: the parser reads the setup string and initializes its output on
    // success; boot-option callbacks serialize ROOT_WAIT access.
    unsafe {
        if bindings::kstrtoint(argument, 0, &mut seconds) != 0 || seconds < 0 {
            main_printk!(
                "rootwait_timeout_setup",
                b"\x014ignoring invalid rootwait value\n\0"
            );
            ROOT_WAIT = -1;
        } else if let Some(milliseconds) = seconds.checked_mul(bindings::MSEC_PER_SEC as c_int) {
            ROOT_WAIT = milliseconds;
        } else {
            main_printk!(
                "rootwait_timeout_setup",
                b"\x014ignoring excessive rootwait value\n\0"
            );
            ROOT_WAIT = -1;
        }
    }
    1
}

#[link_section = ".init.text"]
unsafe extern "C" fn root_data_setup(argument: *mut c_char) -> c_int {
    // SAFETY: the original command-line storage outlives namespace preparation.
    unsafe { ROOT_MOUNT_DATA = argument };
    1
}

#[link_section = ".init.text"]
unsafe extern "C" fn fs_names_setup(argument: *mut c_char) -> c_int {
    // SAFETY: the original command-line storage outlives namespace preparation.
    unsafe { ROOT_FS_NAMES = argument };
    1
}

#[link_section = ".init.text"]
unsafe extern "C" fn root_delay_setup(argument: *mut c_char) -> c_int {
    // SAFETY: parsing is serialized and the output points to the init-only delay.
    if unsafe { bindings::kstrtouint(argument, 0, ptr::addr_of_mut!(ROOT_DELAY)) } != 0 {
        0
    } else {
        1
    }
}

main_setup::setup_param!("ro", READONLY_SETUP, Some(readonly), 0);
main_setup::setup_param!("rw", READWRITE_SETUP, Some(readwrite), 0);
main_setup::setup_param!("root=", ROOT_DEV_SETUP, Some(root_dev_setup), 0);
main_setup::setup_param!("rootwait", ROOTWAIT_SETUP, Some(rootwait_setup), 0);
main_setup::setup_param!(
    "rootwait=",
    ROOTWAIT_TIMEOUT_SETUP,
    Some(rootwait_timeout_setup),
    0
);
main_setup::setup_param!("rootflags=", ROOT_DATA_SETUP, Some(root_data_setup), 0);
main_setup::setup_param!("rootfstype=", FS_NAMES_SETUP, Some(fs_names_setup), 0);
main_setup::setup_param!("rootdelay=", ROOT_DELAY_SETUP, Some(root_delay_setup), 0);

const PAGE_SIZE: usize = bindings::RUST_INIT_ROOT_MOUNTS_PAGE_SIZE as usize;

// Like the header's ssleep, the multiplication takes place in unsigned int.
// Keep its defined unsigned overflow behavior local to this conversion.
#[link_section = ".init.text"]
unsafe fn sleep_seconds(seconds: c_uint) {
    // SAFETY: boot namespace preparation is a sleepable process context.
    unsafe { bindings::msleep(seconds.wrapping_mul(1000)) };
}

#[link_section = ".init.text"]
unsafe fn split_fs_names(page: *mut c_char, size: usize) -> c_int {
    // SAFETY: callers provide an allocated page and a non-null ROOT_FS_NAMES.
    // strscpy terminates even on truncation; replacing delimiters preserves the
    // page extent. A trailing comma deliberately contributes an empty entry.
    unsafe {
        bindings::sized_strscpy(page, ROOT_FS_NAMES, size);
        let mut count = 1;
        let mut cursor = page;
        while cursor.read() != 0 {
            if cursor.read() == b',' as c_char {
                cursor.write(0);
                count += 1;
            }
            cursor = cursor.add(1);
        }
        count
    }
}

#[link_section = ".init.text"]
unsafe fn do_mount_root(
    name: *const c_char,
    filesystem: *const c_char,
    flags: c_int,
    data: *const c_void,
) -> c_int {
    // SAFETY: callers supply live boot strings and serialized init filesystem
    // state. The allocation boundary returns PAGE_SIZE writable bytes. The
    // current task owns its pwd after init_chdir, as in the original C owner.
    unsafe {
        let mut data_page: *mut c_char = ptr::null_mut();
        if !data.is_null() {
            data_page = bindings::rust_init_root_mounts_alloc_data_page();
            if data_page.is_null() {
                return -(bindings::ENOMEM as c_int);
            }
            // Expand only strscpy_pad's inline padding, using its actual copy
            // routine. A truncated copy already terminates the full page.
            let written = bindings::sized_strscpy(data_page, data.cast(), PAGE_SIZE);
            if written >= 0 && (written as usize) < PAGE_SIZE {
                let start = written as usize + 1;
                ptr::write_bytes(data_page.add(start), 0, PAGE_SIZE - start);
            }
        }

        let result = bindings::init_mount(
            name,
            c"/root".as_ptr().cast(),
            filesystem,
            flags as c_ulong,
            data_page.cast(),
        );
        if result == 0 {
            bindings::init_chdir(c"/root".as_ptr().cast());
            // Both structs come from the active kernel's headers. The cast
            // changes only the Rust binding crate's nominal type identity.
            let current = kernel::task::Task::current_raw().cast::<bindings::task_struct>();
            let superblock = (*(*(*current).fs).pwd.dentry).d_sb;
            ROOT_DEV = (*superblock).s_dev;
            let readonly = if (*superblock).s_flags & bindings::SB_RDONLY as c_ulong != 0 {
                c" readonly".as_ptr()
            } else {
                c"".as_ptr()
            };
            main_printk!(
                "do_mount_root",
                b"\x016VFS: Mounted root (%s filesystem)%s on device %u:%u.\n\0",
                (*(*superblock).s_type).name,
                readonly,
                ROOT_DEV >> bindings::MINORBITS,
                ROOT_DEV & bindings::MINORMASK,
            );
        }
        bindings::kfree(data_page.cast());
        result
    }
}

/// Try the requested filesystems, then retry read-only before diagnosing failure.
///
/// # Safety
/// Called during boot namespace setup with live C device-name strings and
/// exclusive access to the root-mount option state and init filesystem context.
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn mount_root_generic(
    name: *mut c_char,
    pretty_name: *mut c_char,
    mut flags: c_int,
) {
    // SAFETY: the caller provides the original serialized boot context. All
    // cursor walks stay in the allocated NUL-separated filesystem list.
    unsafe {
        let filesystems = bindings::rust_init_root_mounts_alloc_fs_names();
        if filesystems.is_null() {
            bindings::panic(
                c"VFS: Unable to mount root fs: not enough memory"
                    .as_ptr()
                    .cast(),
            );
        }
        let mut device_name = [0 as c_char; bindings::BDEVNAME_SIZE as usize];
        bindings::scnprintf(
            device_name.as_mut_ptr(),
            device_name.len(),
            c"unknown-block(%u,%u)".as_ptr().cast(),
            ROOT_DEV >> bindings::MINORBITS,
            ROOT_DEV & bindings::MINORMASK,
        );
        let mut count = if !ROOT_FS_NAMES.is_null() {
            split_fs_names(filesystems, PAGE_SIZE)
        } else {
            bindings::list_bdev_fs_names(filesystems, PAGE_SIZE)
        };

        loop {
            let mut filesystem = filesystems;
            for _ in 0..count {
                if filesystem.read() != 0 {
                    let error = do_mount_root(name, filesystem, flags, ROOT_MOUNT_DATA.cast());
                    if error == 0 {
                        bindings::kfree(filesystems.cast());
                        return;
                    }
                    if error == -(bindings::EACCES as c_int)
                        || error == -(bindings::EINVAL as c_int)
                    {
                        #[cfg(CONFIG_BLOCK)]
                        {
                            // Original init_flush_fput inline, in this order.
                            bindings::flush_delayed_fput();
                            bindings::task_work_run();
                        }
                    } else {
                        main_printk!(
                            "mount_root_generic",
                            b"VFS: Cannot open root device \"%s\" or %s: error %d\n\0",
                            pretty_name,
                            device_name.as_ptr(),
                            error,
                        );
                        main_printk!(
                            "mount_root_generic",
                            b"Please append a correct \"root=\" boot option; here are the available partitions:\n\0"
                        );
                        #[cfg(CONFIG_BLOCK)]
                        bindings::printk_all_partitions();
                        if !ROOT_FS_NAMES.is_null() {
                            count = bindings::list_bdev_fs_names(filesystems, PAGE_SIZE);
                        }
                        if count == 0 {
                            main_printk!(
                                "mount_root_generic",
                                b"\x013Can't find any bdev filesystem to be used for mount!\n\0"
                            );
                        } else {
                            main_printk!(
                                "mount_root_generic",
                                b"\x013List of all bdev filesystems:\n\0"
                            );
                            let mut cursor = filesystems;
                            for _ in 0..count {
                                main_printk!("mount_root_generic", b"\x013 %s\0", cursor);
                                cursor = cursor.add(bindings::strlen(cursor) + 1);
                            }
                            main_printk!("mount_root_generic", b"\x013\n\0");
                        }
                        bindings::panic(
                            c"VFS: Unable to mount root fs on %s".as_ptr().cast(),
                            device_name.as_ptr(),
                        );
                    }
                }
                filesystem = filesystem.add(bindings::strlen(filesystem) + 1);
            }

            if flags & bindings::SB_RDONLY as c_int == 0 {
                flags |= bindings::SB_RDONLY as c_int;
                continue;
            }
            main_printk!("mount_root_generic", b"List of all partitions:\n\0");
            #[cfg(CONFIG_BLOCK)]
            bindings::printk_all_partitions();
            main_printk!(
                "mount_root_generic",
                b"No filesystem could mount root, tried: \0"
            );
            let mut cursor = filesystems;
            for _ in 0..count {
                main_printk!("mount_root_generic", b" %s\0", cursor);
                cursor = cursor.add(bindings::strlen(cursor) + 1);
            }
            main_printk!("mount_root_generic", b"\n\0");
            bindings::panic(
                c"VFS: Unable to mount root fs on \"%s\" or %s"
                    .as_ptr()
                    .cast(),
                pretty_name,
                device_name.as_ptr(),
            );
        }
    }
}

#[cfg(CONFIG_ROOT_NFS)]
#[link_section = ".init.text"]
unsafe fn mount_nfs_root() {
    // SAFETY: nfs_root_data returns boot-lifetime mount strings; namespace
    // preparation is sleepable and owns the global mount state.
    unsafe {
        let mut root_device = ptr::null_mut();
        let mut root_data = ptr::null_mut();
        if bindings::nfs_root_data(&mut root_device, &mut root_data) == 0 {
            let mut timeout = 5;
            // The C loop makes six attempts and sleeps only between attempts:
            // 5, 10, 20, 30, 30 seconds.
            for attempt in 1..=6 {
                if do_mount_root(
                    root_device,
                    c"nfs".as_ptr().cast(),
                    root_mountflags,
                    root_data.cast(),
                ) == 0
                {
                    return;
                }
                if attempt == 6 {
                    break;
                }
                sleep_seconds(timeout);
                timeout = (timeout << 1).min(30);
            }
        }
        main_printk!(
            "mount_nfs_root",
            b"\x013VFS: Unable to mount root fs via NFS.\n\0"
        );
    }
}

#[cfg(CONFIG_CIFS_ROOT)]
#[link_section = ".init.text"]
unsafe fn mount_cifs_root() {
    // SAFETY: cifs_root_data supplies boot-lifetime strings under the same
    // serialized namespace preparation contract as NFS.
    unsafe {
        let mut root_device = ptr::null_mut();
        let mut root_data = ptr::null_mut();
        if bindings::cifs_root_data(&mut root_device, &mut root_data) == 0 {
            let mut timeout = 5;
            for attempt in 1..=6 {
                if do_mount_root(
                    root_device,
                    c"cifs".as_ptr().cast(),
                    root_mountflags,
                    root_data.cast(),
                ) == 0
                {
                    return;
                }
                if attempt == 6 {
                    break;
                }
                sleep_seconds(timeout);
                timeout = (timeout << 1).min(30);
            }
        }
        main_printk!(
            "mount_cifs_root",
            b"\x013VFS: Unable to mount root fs via SMB.\n\0"
        );
    }
}

#[link_section = ".init.text"]
unsafe fn fs_is_nodev(filesystem: *mut c_char) -> bool {
    // SAFETY: get_fs_type obtains the reference released after reading flags.
    unsafe {
        let fs = bindings::get_fs_type(filesystem);
        if fs.is_null() {
            return false;
        }
        let nodev = (*fs).fs_flags & bindings::FS_REQUIRES_DEV as c_int == 0;
        bindings::put_filesystem(fs);
        nodev
    }
}

#[link_section = ".init.text"]
unsafe fn mount_nodev_root(root_device_name: *mut c_char) -> c_int {
    // SAFETY: caller checks ROOT_FS_NAMES and holds boot namespace ownership.
    unsafe {
        let filesystems = bindings::rust_init_root_mounts_alloc_nodev_names();
        if filesystems.is_null() {
            // Deliberately preserve the original allocation-failure errno.
            return -(bindings::EINVAL as c_int);
        }
        let count = split_fs_names(filesystems, PAGE_SIZE);
        let mut filesystem = filesystems;
        let mut error = -(bindings::EINVAL as c_int);
        for _ in 0..count {
            if filesystem.read() != 0 && fs_is_nodev(filesystem) {
                error = do_mount_root(
                    root_device_name,
                    filesystem,
                    root_mountflags,
                    ROOT_MOUNT_DATA.cast(),
                );
                if error == 0 {
                    break;
                }
            }
            filesystem = filesystem.add(bindings::strlen(filesystem) + 1);
        }
        bindings::kfree(filesystems.cast());
        error
    }
}

#[cfg(CONFIG_BLOCK)]
#[link_section = ".init.text"]
unsafe fn mount_block_root(root_device_name: *mut c_char) {
    // SAFETY: init syscalls operate in the caller's serialized boot filesystem
    // context; encoding exactly matches do_mounts.h's create_dev inline.
    unsafe {
        let major = ROOT_DEV >> bindings::MINORBITS;
        let minor = ROOT_DEV & bindings::MINORMASK;
        let encoded = (minor & 0xff) | (major << 8) | ((minor & !0xff) << 12);
        bindings::init_unlink(c"/dev/root".as_ptr().cast());
        let error = bindings::init_mknod(
            c"/dev/root".as_ptr().cast(),
            (bindings::S_IFBLK | 0o600) as bindings::umode_t,
            encoded,
        );
        if error < 0 {
            main_printk!(
                "mount_block_root",
                b"\x010Failed to create /dev/root: %d\n\0",
                error
            );
        }
        mount_root_generic(
            c"/dev/root".as_ptr().cast_mut().cast(),
            root_device_name,
            root_mountflags,
        );
    }
}

/// Select NFS, CIFS, generic, nodev, or block root mounting.
///
/// # Safety
/// Called during boot with a live device-name string and exclusive access to
/// root option state and the init task's filesystem context.
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn mount_root(root_device_name: *mut c_char) {
    // SAFETY: all branches forward the caller's boot namespace contract.
    unsafe {
        match ROOT_DEV {
            device if device == bindings::Root_NFS as bindings::dev_t => {
                // A disabled feature is the original empty C inline.
                #[cfg(CONFIG_ROOT_NFS)]
                mount_nfs_root();
            }
            device if device == bindings::Root_CIFS as bindings::dev_t => {
                #[cfg(CONFIG_CIFS_ROOT)]
                mount_cifs_root();
            }
            device if device == bindings::Root_Generic as bindings::dev_t => {
                mount_root_generic(root_device_name, root_device_name, root_mountflags);
            }
            device => {
                if device == 0
                    && !root_device_name.is_null()
                    && !ROOT_FS_NAMES.is_null()
                    && mount_nodev_root(root_device_name) == 0
                {
                    return;
                }
                #[cfg(CONFIG_BLOCK)]
                mount_block_root(root_device_name);
            }
        }
    }
}

#[link_section = ".init.text"]
unsafe fn lookup_root_device(name: *const c_char, device: *mut bindings::dev_t) -> c_int {
    #[cfg(CONFIG_BLOCK)]
    // SAFETY: callers provide a live name and writable output storage.
    unsafe {
        bindings::early_lookup_bdev(name, device)
    }
    #[cfg(not(CONFIG_BLOCK))]
    {
        // Exact !BLOCK early_lookup_bdev header behavior: no output write.
        let _ = (name, device);
        -(bindings::EINVAL as c_int)
    }
}

#[link_section = ".init.text"]
unsafe fn wait_for_root(root_device_name: *mut c_char) {
    // SAFETY: boot serialization owns ROOT_DEV and ROOT_WAIT; probing and async
    // synchronization are called from sleepable init context.
    unsafe {
        if ROOT_DEV != 0 {
            return;
        }
        main_printk!(
            "wait_for_root",
            b"\x016Waiting for root device %s...\n\0",
            root_device_name
        );
        // ktime_add_ms takes u64 milliseconds, so the indefinite -1 sentinel
        // converts to u64 and both arithmetic steps are unsigned modulo 2^64.
        // Preserve that specific C conversion without disabling overflow checks.
        let end = (bindings::ktime_get_raw() as u64)
            .wrapping_add((ROOT_WAIT as u64).wrapping_mul(bindings::NSEC_PER_MSEC as u64))
            as bindings::ktime_t;
        while !bindings::driver_probe_done()
            || lookup_root_device(root_device_name, ptr::addr_of_mut!(ROOT_DEV)) < 0
        {
            bindings::msleep(5);
            if ROOT_WAIT > 0 && bindings::ktime_get_raw() > end {
                break;
            }
        }
        bindings::async_synchronize_full();
    }
}

#[link_section = ".init.text"]
unsafe fn parse_root_device(root_device_name: *mut c_char) -> bindings::dev_t {
    // SAFETY: namespace setup passes the terminated saved root= string.
    unsafe {
        if bindings::strncmp(root_device_name, c"mtd".as_ptr().cast(), 3) == 0
            || bindings::strncmp(root_device_name, c"ubi".as_ptr().cast(), 3) == 0
        {
            return bindings::Root_Generic as bindings::dev_t;
        }
        if bindings::strcmp(root_device_name, c"/dev/nfs".as_ptr().cast()) == 0 {
            return bindings::Root_NFS as bindings::dev_t;
        }
        if bindings::strcmp(root_device_name, c"/dev/cifs".as_ptr().cast()) == 0 {
            return bindings::Root_CIFS as bindings::dev_t;
        }
        if bindings::strcmp(root_device_name, c"/dev/ram".as_ptr().cast()) == 0 {
            return bindings::Root_RAM0 as bindings::dev_t;
        }
        let mut device = 0;
        let error = lookup_root_device(root_device_name, &mut device);
        if error != 0 {
            if error == -(bindings::EINVAL as c_int) && ROOT_WAIT != 0 {
                main_printk!(
                    "parse_root_device",
                    b"\x013Disabling rootwait; root= is invalid.\n\0"
                );
                ROOT_WAIT = 0;
            }
            return 0;
        }
        device
    }
}

/// Mount the selected root and pivot the init task into its namespace.
///
/// # Safety
/// Must run once in the sleepable init task, before reclaiming init memory,
/// after the initial rootfs and kernel device probing machinery exist.
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn prepare_namespace() {
    // SAFETY: this function's boot-phase contract supplies exclusive option
    // state and the filesystem context expected by all init syscalls.
    unsafe {
        if ROOT_DELAY != 0 {
            main_printk!(
                "prepare_namespace",
                b"\x016Waiting %d sec before mounting root device...\n\0",
                ROOT_DELAY
            );
            sleep_seconds(ROOT_DELAY);
        }
        bindings::wait_for_device_probe();
        #[cfg(CONFIG_BLK_DEV_MD = "y")]
        bindings::md_run_setup();

        let root_name = ptr::addr_of_mut!(SAVED_ROOT_NAME).cast::<c_char>();
        if root_name.read() != 0 {
            ROOT_DEV = parse_root_device(root_name);
        }
        #[cfg(CONFIG_BLK_DEV_INITRD)]
        bindings::initrd_load();
        if ROOT_WAIT != 0 {
            wait_for_root(root_name);
        }
        mount_root(root_name);
        #[cfg(CONFIG_DEVTMPFS)]
        bindings::devtmpfs_mount();

        if bindings::init_pivot_root(c".".as_ptr().cast(), c".".as_ptr().cast()) != 0 {
            main_printk!(
                "prepare_namespace",
                b"\x013VFS: Failed to pivot into new rootfs\n\0"
            );
            return;
        }
        if bindings::init_umount(c".".as_ptr().cast(), bindings::MNT_DETACH as c_int) != 0 {
            main_printk!(
                "prepare_namespace",
                b"\x013VFS: Failed to unmount old rootfs\n\0"
            );
            return;
        }
        main_printk!(
            "prepare_namespace",
            b"\x016VFS: Pivoted into new rootfs\n\0"
        );
    }
}

#[cfg(CONFIG_TMPFS)]
static mut IS_TMPFS: bool = false;

// This callback is permanent: rootfs_fs_type remains registered after init.
unsafe extern "C" fn rootfs_init_fs_context(context: *mut bindings::fs_context) -> c_int {
    // SAFETY: the VFS supplies a live filesystem context. The selector is
    // initialized before registration and is never changed after boot.
    unsafe {
        #[cfg(CONFIG_TMPFS)]
        if IS_TMPFS {
            return bindings::shmem_init_fs_context(context);
        }
        bindings::ramfs_init_fs_context(context)
    }
}

/// The original rootfs filesystem registration, with a header-derived layout.
#[no_mangle]
pub static mut rootfs_fs_type: bindings::file_system_type = {
    // SAFETY: C's omitted initializer fields are all zero (nullable pointers,
    // callback Options, integer flags, hash nodes and lock-class keys). The
    // actual generated struct, including configuration-dependent fields, is
    // used rather than a hand-written layout or a partial initializer.
    let mut filesystem: bindings::file_system_type = unsafe { core::mem::zeroed() };
    filesystem.name = c"rootfs".as_ptr().cast();
    filesystem.init_fs_context = Some(rootfs_init_fs_context);
    filesystem.kill_sb = Some(bindings::kill_anon_super);
    filesystem
};

/// Decide whether the initial rootfs uses tmpfs or ramfs.
///
/// # Safety
/// Called during serialized boot, after parsing root options and before any
/// filesystem context may read the persistent rootfs selector.
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn init_rootfs() {
    #[cfg(CONFIG_TMPFS)]
    // SAFETY: the boot-only caller owns the saved option storage and selector.
    unsafe {
        if (ptr::addr_of!(SAVED_ROOT_NAME).cast::<c_char>().read() == 0 && ROOT_FS_NAMES.is_null())
            || (!ROOT_FS_NAMES.is_null()
                && !bindings::strstr(ROOT_FS_NAMES, c"tmpfs".as_ptr().cast()).is_null())
        {
            IS_TMPFS = true;
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
