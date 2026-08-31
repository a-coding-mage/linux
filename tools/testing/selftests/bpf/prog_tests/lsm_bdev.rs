// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Christian Brauner <brauner@kernel.org> */

/*
 * Test BPF LSM block device integrity hooks with dm-verity.
 *
 * Creates a dm-verity device over loopback, which triggers
 * security_bdev_setintegrity() during verity_preresume().
 * Verifies that the BPF program correctly tracks the integrity
 * metadata in its hashmap.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

/* Dependencies from test_progs.h, system headers, and lsm_bdev.skel.h. */
#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct lsm_bdev_maps {
    pub verity_devices: *mut bpf_map,
}

#[repr(C)]
pub struct lsm_bdev_bss {
    pub alloc_count: c_uint,
}

#[repr(C)]
pub struct lsm_bdev {
    pub maps: lsm_bdev_maps,
    pub bss: *mut lsm_bdev_bss,
}

#[repr(C)]
pub struct stat {
    pub st_dev: c_ulong,
    pub st_ino: c_ulong,
    pub st_nlink: c_ulong,
    pub st_mode: c_uint,
    pub st_uid: c_uint,
    pub st_gid: c_uint,
    pub __pad0: c_int,
    pub st_rdev: c_ulong,
    pub st_size: c_long,
    pub st_blksize: c_long,
    pub st_blocks: c_long,
    pub __unused: [c_long; 6],
}

unsafe extern "C" {
    fn popen(command: *const c_char, type_: *const c_char) -> *mut FILE;
    fn pclose(stream: *mut FILE) -> c_int;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn strcspn(s: *const c_char, reject: *const c_char) -> usize;
    fn getuid() -> c_uint;
    fn printf(format: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn mkstemp(template: *mut c_char) -> c_int;
    fn ftruncate(fd: c_int, length: c_long) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn stat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;

    fn test__skip();
    fn ASSERT_OK_FD(fd: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_GT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_uint, expected: c_uint, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_uint, expected: c_uint, name: *const c_char) -> bool;
    fn ASSERT_LE(actual: c_uint, expected: c_uint, name: *const c_char) -> bool;

    fn lsm_bdev__open_and_load() -> *mut lsm_bdev;
    fn lsm_bdev__attach(skel: *mut lsm_bdev) -> c_int;
    fn lsm_bdev__destroy(skel: *mut lsm_bdev);
    fn bpf_map__lookup_elem(
        map: *mut bpf_map,
        key: *const c_void,
        key_sz: usize,
        value: *mut c_void,
        value_sz: usize,
        flags: c_ulong,
    ) -> c_int;
}

/* Must match the definition in progs/lsm_bdev.c. */
#[repr(C)]
pub struct verity_info {
    pub has_roothash: u8,
    pub sig_valid: u8,
    pub setintegrity_cnt: u32,
}

const DATA_SIZE_MB: c_int = 8;
const HASH_SIZE_MB: c_int = 1;
const DM_NAME: &[u8] = b"bpf_test_verity\0";
const DM_DEV_PATH: &[u8] = b"/dev/mapper/bpf_test_verity\0";

unsafe fn wifexited(status: c_int) -> bool {
    (status & 0x7f) == 0
}

unsafe fn wexitstatus(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn major(dev: c_ulong) -> c_uint {
    (((dev >> 8) & 0xfff) | ((dev >> 32) & !0xfff)) as c_uint
}

unsafe fn minor(dev: c_ulong) -> c_uint {
    ((dev & 0xff) | ((dev >> 12) & !0xff)) as c_uint
}

/* Run a command and optionally capture the first line of stdout. */
unsafe fn run_cmd(cmd: *const c_char, out: *mut c_char, out_sz: usize) -> c_int {
    let fp: *mut FILE;
    let ret: c_int;

    fp = popen(cmd, c"r".as_ptr());
    if fp.is_null() {
        return -1;
    }

    if !out.is_null() && out_sz > 0 {
        if fgets(out, out_sz as c_int, fp).is_null() {
            *out = b'\0' as c_char;
        }
        /* strip trailing newline */
        *out.add(strcspn(out, c"\n".as_ptr())) = b'\0' as c_char;
    }

    ret = pclose(fp);
    if wifexited(ret) {
        wexitstatus(ret)
    } else {
        -1
    }
}

unsafe fn has_prerequisites() -> bool {
    if getuid() != 0 {
        printf(c"SKIP: must be root\n".as_ptr());
        return false;
    }

    if run_cmd(c"modprobe loop 2>/dev/null".as_ptr(), core::ptr::null_mut(), 0) != 0
        && run_cmd(c"ls /dev/loop-control 2>/dev/null".as_ptr(), core::ptr::null_mut(), 0) != 0
    {
        printf(c"SKIP: no loop device support\n".as_ptr());
        return false;
    }

    if run_cmd(c"modprobe dm-verity 2>/dev/null".as_ptr(), core::ptr::null_mut(), 0) != 0
        && run_cmd(
            c"dmsetup targets 2>/dev/null | grep -q verity".as_ptr(),
            core::ptr::null_mut(),
            0,
        ) != 0
    {
        printf(c"SKIP: dm-verity module not available\n".as_ptr());
        return false;
    }

    if run_cmd(c"which veritysetup >/dev/null 2>&1".as_ptr(), core::ptr::null_mut(), 0) != 0 {
        printf(c"SKIP: veritysetup not found\n".as_ptr());
        return false;
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_lsm_bdev() {
    let mut data_img = *b"/tmp/bpf_verity_data_XXXXXX\0";
    let mut hash_img = *b"/tmp/bpf_verity_hash_XXXXXX\0";
    let mut data_loop = [0 as c_char; 64];
    let mut hash_loop = [0 as c_char; 64];
    let mut roothash = [0 as c_char; 256];
    let mut cmd = [0 as c_char; 512];
    let mut data_fd: c_int = -1;
    let mut hash_fd: c_int = -1;
    let mut skel: *mut lsm_bdev = core::ptr::null_mut();
    let mut val: verity_info = core::mem::zeroed();
    let mut st: stat = core::mem::zeroed();
    let mut dev_key: u32;
    let mut err: c_int;

    if !has_prerequisites() {
        test__skip();
        return;
    }

    'cleanup: {
        'teardown: {
            /* Clean up any stale device from a previous crashed run. */
            snprintf(
                cmd.as_mut_ptr(),
                cmd.len(),
                c"dmsetup remove %s 2>/dev/null".as_ptr(),
                DM_NAME.as_ptr(),
            );
            run_cmd(cmd.as_ptr(), core::ptr::null_mut(), 0);

            /* Create temporary image files. */
            data_fd = mkstemp(data_img.as_mut_ptr() as *mut c_char);
            if !ASSERT_OK_FD(data_fd, c"mkstemp data".as_ptr()) {
                break 'cleanup;
            }

            hash_fd = mkstemp(hash_img.as_mut_ptr() as *mut c_char);
            if !ASSERT_OK_FD(hash_fd, c"mkstemp hash".as_ptr()) {
                break 'cleanup;
            }

            if !ASSERT_OK(
                ftruncate(data_fd, (DATA_SIZE_MB * 1024 * 1024) as c_long),
                c"truncate data".as_ptr(),
            ) {
                break 'cleanup;
            }

            if !ASSERT_OK(
                ftruncate(hash_fd, (HASH_SIZE_MB * 1024 * 1024) as c_long),
                c"truncate hash".as_ptr(),
            ) {
                break 'cleanup;
            }

            close(data_fd);
            data_fd = -1;
            close(hash_fd);
            hash_fd = -1;

            /* Set up loop devices. */
            snprintf(
                cmd.as_mut_ptr(),
                cmd.len(),
                c"losetup --find --show %s 2>/dev/null".as_ptr(),
                data_img.as_ptr(),
            );
            if !ASSERT_OK(
                run_cmd(cmd.as_ptr(), data_loop.as_mut_ptr(), data_loop.len()),
                c"losetup data".as_ptr(),
            ) {
                break 'teardown;
            }

            snprintf(
                cmd.as_mut_ptr(),
                cmd.len(),
                c"losetup --find --show %s 2>/dev/null".as_ptr(),
                hash_img.as_ptr(),
            );
            if !ASSERT_OK(
                run_cmd(cmd.as_ptr(), hash_loop.as_mut_ptr(), hash_loop.len()),
                c"losetup hash".as_ptr(),
            ) {
                break 'teardown;
            }

            /* Format the dm-verity device and capture the root hash. */
            snprintf(
                cmd.as_mut_ptr(),
                cmd.len(),
                c"veritysetup format %s %s 2>/dev/null | grep -i 'root hash' | awk '{print $NF}'"
                    .as_ptr(),
                data_loop.as_ptr(),
                hash_loop.as_ptr(),
            );
            if !ASSERT_OK(
                run_cmd(cmd.as_ptr(), roothash.as_mut_ptr(), roothash.len()),
                c"veritysetup format".as_ptr(),
            ) {
                break 'teardown;
            }

            if !ASSERT_GT(
                strlen(roothash.as_ptr()) as c_int,
                0,
                c"roothash not empty".as_ptr(),
            ) {
                break 'teardown;
            }

            /* Load and attach BPF program before activating dm-verity. */
            skel = lsm_bdev__open_and_load();
            if !ASSERT_OK_PTR(skel as *const c_void, c"skel open_and_load".as_ptr()) {
                break 'teardown;
            }

            err = lsm_bdev__attach(skel);
            if !ASSERT_OK(err, c"skel attach".as_ptr()) {
                break 'teardown;
            }

            'remove_dm: {
                /* Activate dm-verity - triggers verity_preresume() hooks. */
                snprintf(
                    cmd.as_mut_ptr(),
                    cmd.len(),
                    c"veritysetup open %s %s %s %s 2>/dev/null".as_ptr(),
                    data_loop.as_ptr(),
                    DM_NAME.as_ptr(),
                    hash_loop.as_ptr(),
                    roothash.as_ptr(),
                );
                if !ASSERT_OK(run_cmd(cmd.as_ptr(), core::ptr::null_mut(), 0), c"veritysetup open".as_ptr()) {
                    break 'teardown;
                }

                /* Get the dm device's dev_t. */
                if !ASSERT_OK(stat(DM_DEV_PATH.as_ptr() as *const c_char, &mut st), c"stat dm dev".as_ptr()) {
                    break 'remove_dm;
                }

                dev_key = (major(st.st_rdev) << 20) | minor(st.st_rdev);

                /* Look up the device in the BPF map and verify. */
                err = bpf_map__lookup_elem(
                    (*skel).maps.verity_devices,
                    &dev_key as *const _ as *const c_void,
                    core::mem::size_of_val(&dev_key),
                    &mut val as *mut _ as *mut c_void,
                    core::mem::size_of_val(&val),
                    0,
                );
                if !ASSERT_OK(err, c"map lookup".as_ptr()) {
                    break 'remove_dm;
                }

                ASSERT_EQ(val.has_roothash as c_uint, 1, c"has_roothash".as_ptr());
                ASSERT_EQ(val.sig_valid as c_uint, 0, c"sig_valid (unsigned)".as_ptr());
                /*
                 * verity_preresume() always calls security_bdev_setintegrity()
                 * for the roothash. The signature-validity call only happens
                 * when CONFIG_DM_VERITY_VERIFY_ROOTHASH_SIG is enabled.
                 */
                ASSERT_GE(val.setintegrity_cnt, 1, c"setintegrity_cnt min".as_ptr());
                ASSERT_LE(val.setintegrity_cnt, 2, c"setintegrity_cnt max".as_ptr());

                /* Verify that the alloc hook fired at least once. */
                ASSERT_GT((*(*skel).bss).alloc_count as c_int, 0, c"alloc_count".as_ptr());
            }

            snprintf(
                cmd.as_mut_ptr(),
                cmd.len(),
                c"dmsetup remove %s 2>/dev/null".as_ptr(),
                DM_NAME.as_ptr(),
            );
            run_cmd(cmd.as_ptr(), core::ptr::null_mut(), 0);
        }

        if data_loop[0] != 0 {
            snprintf(
                cmd.as_mut_ptr(),
                cmd.len(),
                c"losetup -d %s 2>/dev/null".as_ptr(),
                data_loop.as_ptr(),
            );
            run_cmd(cmd.as_ptr(), core::ptr::null_mut(), 0);
        }
        if hash_loop[0] != 0 {
            snprintf(
                cmd.as_mut_ptr(),
                cmd.len(),
                c"losetup -d %s 2>/dev/null".as_ptr(),
                hash_loop.as_ptr(),
            );
            run_cmd(cmd.as_ptr(), core::ptr::null_mut(), 0);
        }
    }

    lsm_bdev__destroy(skel);
    if data_fd >= 0 {
        close(data_fd);
    }
    if hash_fd >= 0 {
        close(hash_fd);
    }
    unlink(data_img.as_ptr() as *const c_char);
    unlink(hash_img.as_ptr() as *const c_char);
}
