// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2022 Intel Corporation
//
// Authors: Ranjani Sridharan <ranjani.sridharan@linux.intel.com>
//	    Peter Ujfalusi <peter.ujfalusi@linux.intel.com>
//

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_ulong, c_void};

type ssize_t = isize;
type size_t = usize;
type loff_t = i64;
type ktime_t = i64;
type u64_t = u64;
type bool_t = bool;

const MAX_IPC_FLOOD_DURATION_MS: c_ulong = 1000;
const MAX_IPC_FLOOD_COUNT: c_ulong = 10000;
const IPC_FLOOD_TEST_RESULT_LEN: size_t = 512;
const SOF_IPC_CLIENT_SUSPEND_DELAY_MS: c_int = 3000;

const DEBUGFS_IPC_FLOOD_COUNT: &[u8] = b"ipc_flood_count\0";
const DEBUGFS_IPC_FLOOD_DURATION: &[u8] = b"ipc_flood_duration_ms\0";

const SOF_FW_CRASHED: c_int = 0;
const ENODEV: c_int = 19;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const EFAULT: c_int = 14;
const EACCES: c_int = 13;
const GFP_KERNEL: c_uint = 0;
const SOF_IPC_GLB_TEST_MSG: u32 = 0;
const SOF_IPC_TEST_IPC_FLOOD: u32 = 0;
const NSEC_PER_MSEC: u64_t = 1_000_000;
const U64_MAX: u64_t = u64::MAX;
const THIS_MODULE: *mut module = core::ptr::null_mut();

type c_uint = u32;

#[repr(C)]
struct dentry {
    _private: [u8; 0],
}

#[repr(C)]
struct inode {
    i_private: *mut c_void,
}

#[repr(C)]
struct path {
    dentry: *mut dentry,
}

#[repr(C)]
struct file {
    private_data: *mut c_void,
    f_path: path,
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct auxiliary_device {
    dev: device,
    id: c_uint,
}

#[repr(C)]
struct auxiliary_device_id {
    name: *const c_char,
}

#[repr(C)]
struct auxiliary_driver {
    probe: Option<unsafe extern "C" fn(*mut auxiliary_device, *const auxiliary_device_id) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut auxiliary_device)>,
    id_table: *const auxiliary_device_id,
}

#[repr(C)]
struct sof_auxdev {
    dev: device,
}

#[repr(C)]
struct sof_client_dev {
    auxdev: sof_auxdev,
    data: *mut c_void,
}

#[repr(C)]
struct sof_ipc_cmd_hdr {
    cmd: u32,
    size: u32,
}

#[repr(C)]
struct module {
    _private: [u8; 0],
}

#[repr(C)]
struct file_operations {
    open: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    read: Option<unsafe extern "C" fn(*mut file, *mut c_char, size_t, *mut loff_t) -> ssize_t>,
    llseek: Option<unsafe extern "C" fn(*mut file, loff_t, c_int) -> loff_t>,
    write: Option<unsafe extern "C" fn(*mut file, *const c_char, size_t, *mut loff_t) -> ssize_t>,
    release: Option<unsafe extern "C" fn(*mut inode, *mut file) -> c_int>,
    owner: *mut module,
}

#[repr(C)]
struct sof_ipc_flood_priv {
    dfs_root: *mut dentry,
    dfs_link: [*mut dentry; 2],
    buf: *mut c_char,
}

unsafe extern "C" {
    fn sof_client_get_fw_state(cdev: *mut sof_client_dev) -> c_int;
    fn debugfs_file_get(dentry: *mut dentry) -> c_int;
    fn debugfs_file_put(dentry: *mut dentry);
    fn simple_open(inode: *mut inode, file: *mut file) -> c_int;
    fn ktime_get_ns() -> u64_t;
    fn ktime_get() -> ktime_t;
    fn ktime_to_ns(ktime: ktime_t) -> u64_t;
    fn ktime_sub(lhs: ktime_t, rhs: ktime_t) -> ktime_t;
    fn sof_client_ipc_tx_message_no_reply(cdev: *mut sof_client_dev, hdr: *mut sof_ipc_cmd_hdr)
        -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_ratelimited(dev: *mut device, fmt: *const c_char, ...);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn kzalloc(size: size_t, flags: c_uint) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn copy_from_user(to: *mut c_void, from: *const c_char, n: size_t) -> c_ulong;
    fn copy_to_user(to: *mut c_char, from: *const c_void, n: size_t) -> c_ulong;
    fn debugfs_get_aux_num(file: *mut file) -> c_int;
    fn kstrtoul(s: *const c_char, base: c_uint, res: *mut c_ulong) -> c_int;
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn sof_client_boot_dsp(cdev: *mut sof_client_dev) -> c_int;
    fn pm_runtime_put_autosuspend(dev: *mut device) -> c_int;
    fn default_llseek(file: *mut file, offset: loff_t, whence: c_int) -> loff_t;
    fn auxiliary_dev_to_sof_client_dev(auxdev: *mut auxiliary_device) -> *mut sof_client_dev;
    fn sof_client_get_debugfs_root(cdev: *mut sof_client_dev) -> *mut dentry;
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn devm_kmalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn debugfs_create_dir(name: *const c_char, parent: *mut dentry) -> *mut dentry;
    fn IS_ERR_OR_NULL(ptr: *const c_void) -> bool_t;
    fn debugfs_create_file_aux_num(
        name: *const c_char,
        mode: c_uint,
        parent: *mut dentry,
        data: *mut c_void,
        aux_num: c_int,
        fops: *const file_operations,
    ) -> *mut dentry;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn debugfs_create_symlink(
        name: *const c_char,
        parent: *mut dentry,
        target: *const c_char,
    ) -> *mut dentry;
    fn pm_runtime_set_autosuspend_delay(dev: *mut device, delay: c_int);
    fn pm_runtime_use_autosuspend(dev: *mut device);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_mark_last_busy(dev: *mut device);
    fn pm_runtime_idle(dev: *mut device) -> c_int;
    fn pm_runtime_disable(dev: *mut device);
    fn debugfs_remove(dentry: *mut dentry);
    fn debugfs_remove_recursive(dentry: *mut dentry);
}

unsafe extern "C" fn sof_ipc_flood_dfs_open(inode: *mut inode, file: *mut file) -> c_int {
    let cdev = unsafe { (*inode).i_private as *mut sof_client_dev };
    let mut ret: c_int;

    if unsafe { sof_client_get_fw_state(cdev) } == SOF_FW_CRASHED {
        return -ENODEV;
    }

    ret = unsafe { debugfs_file_get((*file).f_path.dentry) };
    if ret != 0 {
        return ret;
    }

    ret = unsafe { simple_open(inode, file) };
    if ret != 0 {
        unsafe { debugfs_file_put((*file).f_path.dentry) };
    }

    ret
}

/*
 * helper function to perform the flood test. Only one of the two params, ipc_duration_ms
 * or ipc_count, will be non-zero and will determine the type of test
 */
unsafe fn sof_debug_ipc_flood_test(
    cdev: *mut sof_client_dev,
    flood_duration_test: bool,
    ipc_duration_ms: c_ulong,
    ipc_count: c_ulong,
) -> c_int {
    let priv_ = unsafe { (*cdev).data as *mut sof_ipc_flood_priv };
    let dev = unsafe { &mut (*cdev).auxdev.dev as *mut device };
    let mut hdr = sof_ipc_cmd_hdr { cmd: 0, size: 0 };
    let mut min_response_time: u64_t = U64_MAX;
    let mut start: ktime_t;
    let mut end: ktime_t;
    let mut test_end: u64_t = 0;
    let mut avg_response_time: u64_t = 0;
    let mut max_response_time: u64_t = 0;
    let mut ipc_response_time: u64_t;
    let mut i: c_int = 0;
    let mut ret: c_int;

    /* configure test IPC */
    hdr.cmd = SOF_IPC_GLB_TEST_MSG | SOF_IPC_TEST_IPC_FLOOD;
    hdr.size = core::mem::size_of_val(&hdr) as u32;

    /* set test end time for duration flood test */
    if flood_duration_test {
        test_end = unsafe { ktime_get_ns() }
            .wrapping_add((ipc_duration_ms as u64_t).wrapping_mul(NSEC_PER_MSEC));
    }

    /* send test IPC's */
    loop {
        start = unsafe { ktime_get() };
        ret = unsafe { sof_client_ipc_tx_message_no_reply(cdev, &mut hdr) };
        end = unsafe { ktime_get() };

        if ret < 0 {
            break;
        }

        /* compute min and max response times */
        ipc_response_time = unsafe { ktime_to_ns(ktime_sub(end, start)) };
        min_response_time = core::cmp::min(min_response_time, ipc_response_time);
        max_response_time = core::cmp::max(max_response_time, ipc_response_time);

        /* sum up response times */
        avg_response_time = avg_response_time.wrapping_add(ipc_response_time);
        i += 1;

        /* test complete? */
        if flood_duration_test {
            if unsafe { ktime_to_ns(end) } >= test_end {
                break;
            }
        } else if i == ipc_count as c_int {
            break;
        }
    }

    if ret < 0 {
        unsafe { dev_err(dev, b"ipc flood test failed at %d iterations\n\0".as_ptr().cast(), i) };
    }

    /* return if the first IPC fails */
    if i == 0 {
        return ret;
    }

    /* compute average response time */
    avg_response_time /= i as u64_t;

    /* clear previous test output */
    unsafe { memset((*priv_).buf.cast(), 0, IPC_FLOOD_TEST_RESULT_LEN) };

    if ipc_count == 0 {
        unsafe {
            dev_dbg(
                dev,
                b"IPC Flood test duration: %lums\n\0".as_ptr().cast(),
                ipc_duration_ms,
            )
        };
        unsafe {
            snprintf(
                (*priv_).buf,
                IPC_FLOOD_TEST_RESULT_LEN,
                b"IPC Flood test duration: %lums\n\0".as_ptr().cast(),
                ipc_duration_ms,
            )
        };
    }

    unsafe {
        dev_dbg(
            dev,
            b"IPC Flood count: %d, Avg response time: %lluns\n\0"
                .as_ptr()
                .cast(),
            i,
            avg_response_time,
        )
    };
    unsafe {
        dev_dbg(
            dev,
            b"Max response time: %lluns\n\0".as_ptr().cast(),
            max_response_time,
        )
    };
    unsafe {
        dev_dbg(
            dev,
            b"Min response time: %lluns\n\0".as_ptr().cast(),
            min_response_time,
        )
    };

    /* format output string and save test results */
    unsafe {
        snprintf(
            (*priv_).buf.add(strlen((*priv_).buf)),
            IPC_FLOOD_TEST_RESULT_LEN - strlen((*priv_).buf),
            b"IPC Flood count: %d\nAvg response time: %lluns\n\0"
                .as_ptr()
                .cast(),
            i,
            avg_response_time,
        )
    };

    unsafe {
        snprintf(
            (*priv_).buf.add(strlen((*priv_).buf)),
            IPC_FLOOD_TEST_RESULT_LEN - strlen((*priv_).buf),
            b"Max response time: %lluns\nMin response time: %lluns\n\0"
                .as_ptr()
                .cast(),
            max_response_time,
            min_response_time,
        )
    };

    ret
}

/*
 * Writing to the debugfs entry initiates the IPC flood test based on
 * the IPC count or the duration specified by the user.
 */
unsafe extern "C" fn sof_ipc_flood_dfs_write(
    file: *mut file,
    buffer: *const c_char,
    count: size_t,
    ppos: *mut loff_t,
) -> ssize_t {
    let cdev = unsafe { (*file).private_data as *mut sof_client_dev };
    let dev = unsafe { &mut (*cdev).auxdev.dev as *mut device };
    let mut ipc_duration_ms: c_ulong = 0;
    let mut flood_duration_test = false;
    let mut ipc_count: c_ulong = 0;
    let mut err: c_int;
    let string: *mut c_char;
    let mut ret: c_int;

    if unsafe { *ppos } != 0 {
        return -EINVAL as ssize_t;
    }

    string = unsafe { kzalloc(count + 1, GFP_KERNEL) as *mut c_char };
    if string.is_null() {
        return -ENOMEM as ssize_t;
    }

    if unsafe { copy_from_user(string.cast(), buffer, count) } != 0 {
        ret = -EFAULT;
        goto_out(string, ret)
    } else {
        /*
         * write op is only supported for ipc_flood_count or
         * ipc_flood_duration_ms debugfs entries atm.
         * ipc_flood_count floods the DSP with the number of IPC's specified.
         * ipc_duration_ms test floods the DSP for the time specified
         * in the debugfs entry.
         */
        if unsafe { debugfs_get_aux_num(file) } != 0 {
            flood_duration_test = true;
        }

        /* test completion criterion */
        if flood_duration_test {
            ret = unsafe { kstrtoul(string, 0, &mut ipc_duration_ms) };
        } else {
            ret = unsafe { kstrtoul(string, 0, &mut ipc_count) };
        }
        if ret < 0 {
            return goto_out(string, ret);
        }

        /* limit max duration/ipc count for flood test */
        if flood_duration_test {
            if ipc_duration_ms == 0 {
                ret = count as c_int;
                return goto_out(string, ret);
            }

            /* find the minimum. min() is not used to avoid warnings */
            if ipc_duration_ms > MAX_IPC_FLOOD_DURATION_MS {
                ipc_duration_ms = MAX_IPC_FLOOD_DURATION_MS;
            }
        } else {
            if ipc_count == 0 {
                ret = count as c_int;
                return goto_out(string, ret);
            }

            /* find the minimum. min() is not used to avoid warnings */
            if ipc_count > MAX_IPC_FLOOD_COUNT {
                ipc_count = MAX_IPC_FLOOD_COUNT;
            }
        }

        ret = unsafe { pm_runtime_resume_and_get(dev) };
        if ret < 0 && ret != -EACCES {
            unsafe {
                dev_err_ratelimited(
                    dev,
                    b"debugfs write failed to resume %d\n\0".as_ptr().cast(),
                    ret,
                )
            };
            return goto_out(string, ret);
        }

        ret = unsafe { sof_client_boot_dsp(cdev) };
        if ret == 0 {
            ret = unsafe {
                sof_debug_ipc_flood_test(cdev, flood_duration_test, ipc_duration_ms, ipc_count)
            };
        }

        err = unsafe { pm_runtime_put_autosuspend(dev) };
        if err < 0 {
            unsafe {
                dev_err_ratelimited(
                    dev,
                    b"debugfs write failed to idle %d\n\0".as_ptr().cast(),
                    err,
                )
            };
        }

        /* return count if test is successful */
        if ret >= 0 {
            ret = count as c_int;
        }

        goto_out(string, ret)
    }
}

unsafe fn goto_out(string: *mut c_char, ret: c_int) -> ssize_t {
    unsafe { kfree(string.cast()) };
    ret as ssize_t
}

/* return the result of the last IPC flood test */
unsafe extern "C" fn sof_ipc_flood_dfs_read(
    file: *mut file,
    buffer: *mut c_char,
    mut count: size_t,
    ppos: *mut loff_t,
) -> ssize_t {
    let cdev = unsafe { (*file).private_data as *mut sof_client_dev };
    let priv_ = unsafe { (*cdev).data as *mut sof_ipc_flood_priv };
    let size_ret: size_t;

    if unsafe { *ppos } != 0 {
        return 0;
    }

    count = core::cmp::min(count, unsafe { strlen((*priv_).buf) });
    size_ret = unsafe { copy_to_user(buffer, (*priv_).buf.cast(), count) as size_t };
    if size_ret != 0 {
        return -EFAULT as ssize_t;
    }

    unsafe { *ppos += count as loff_t };
    count as ssize_t
}

unsafe extern "C" fn sof_ipc_flood_dfs_release(_inode: *mut inode, file: *mut file) -> c_int {
    unsafe { debugfs_file_put((*file).f_path.dentry) };

    0
}

static sof_ipc_flood_fops: file_operations = file_operations {
    open: Some(sof_ipc_flood_dfs_open),
    read: Some(sof_ipc_flood_dfs_read),
    llseek: Some(default_llseek),
    write: Some(sof_ipc_flood_dfs_write),
    release: Some(sof_ipc_flood_dfs_release),

    owner: THIS_MODULE,
};

/*
 * The IPC test client creates a couple of debugfs entries that will be used
 * flood tests. Users can write to these entries to execute the IPC flood test
 * by specifying either the number of IPCs to flood the DSP with or the duration
 * (in ms) for which the DSP should be flooded with test IPCs. At the
 * end of each test, the average, min and max response times are reported back.
 * The results of the last flood test can be accessed by reading the debugfs
 * entries.
 */
unsafe extern "C" fn sof_ipc_flood_probe(
    auxdev: *mut auxiliary_device,
    _id: *const auxiliary_device_id,
) -> c_int {
    let cdev = unsafe { auxiliary_dev_to_sof_client_dev(auxdev) };
    let debugfs_root = unsafe { sof_client_get_debugfs_root(cdev) };
    let dev = unsafe { &mut (*auxdev).dev as *mut device };
    let priv_: *mut sof_ipc_flood_priv;

    /* allocate memory for client data */
    priv_ = unsafe {
        devm_kzalloc(dev, core::mem::size_of::<sof_ipc_flood_priv>(), GFP_KERNEL)
            as *mut sof_ipc_flood_priv
    };
    if priv_.is_null() {
        return -ENOMEM;
    }

    unsafe {
        (*priv_).buf = devm_kmalloc(dev, IPC_FLOOD_TEST_RESULT_LEN, GFP_KERNEL) as *mut c_char;
    }
    if unsafe { (*priv_).buf.is_null() } {
        return -ENOMEM;
    }

    unsafe { (*cdev).data = priv_.cast() };

    /* create debugfs root folder with device name under parent SOF dir */
    unsafe {
        (*priv_).dfs_root = debugfs_create_dir(dev_name(dev), debugfs_root);
    }
    if !unsafe { IS_ERR_OR_NULL((*priv_).dfs_root.cast()) } {
        /* create read-write ipc_flood_count debugfs entry */
        unsafe {
            debugfs_create_file_aux_num(
                DEBUGFS_IPC_FLOOD_COUNT.as_ptr().cast(),
                0o644,
                (*priv_).dfs_root,
                cdev.cast(),
                0,
                &sof_ipc_flood_fops,
            )
        };

        /* create read-write ipc_flood_duration_ms debugfs entry */
        unsafe {
            debugfs_create_file_aux_num(
                DEBUGFS_IPC_FLOOD_DURATION.as_ptr().cast(),
                0o644,
                (*priv_).dfs_root,
                cdev.cast(),
                1,
                &sof_ipc_flood_fops,
            )
        };

        if unsafe { (*auxdev).id } == 0 {
            /*
             * Create symlinks for backwards compatibility to the
             * first IPC flood test instance
             */
            let mut target = [0 as c_char; 100];

            unsafe {
                snprintf(
                    target.as_mut_ptr(),
                    100,
                    b"%s/ipc_flood_count\0".as_ptr().cast(),
                    dev_name(dev),
                )
            };
            unsafe {
                (*priv_).dfs_link[0] = debugfs_create_symlink(
                    DEBUGFS_IPC_FLOOD_COUNT.as_ptr().cast(),
                    debugfs_root,
                    target.as_ptr(),
                )
            };

            unsafe {
                snprintf(
                    target.as_mut_ptr(),
                    100,
                    b"%s/ipc_flood_duration_ms\0".as_ptr().cast(),
                    dev_name(dev),
                )
            };
            unsafe {
                (*priv_).dfs_link[1] = debugfs_create_symlink(
                    DEBUGFS_IPC_FLOOD_DURATION.as_ptr().cast(),
                    debugfs_root,
                    target.as_ptr(),
                )
            };
        }
    }

    /* enable runtime PM */
    unsafe { pm_runtime_set_autosuspend_delay(dev, SOF_IPC_CLIENT_SUSPEND_DELAY_MS) };
    unsafe { pm_runtime_use_autosuspend(dev) };
    unsafe { pm_runtime_enable(dev) };
    unsafe { pm_runtime_mark_last_busy(dev) };
    unsafe { pm_runtime_idle(dev) };

    0
}

unsafe extern "C" fn sof_ipc_flood_remove(auxdev: *mut auxiliary_device) {
    let cdev = unsafe { auxiliary_dev_to_sof_client_dev(auxdev) };
    let priv_ = unsafe { (*cdev).data as *mut sof_ipc_flood_priv };

    unsafe { pm_runtime_disable(&mut (*auxdev).dev) };

    if unsafe { (*auxdev).id } == 0 {
        unsafe { debugfs_remove((*priv_).dfs_link[0]) };
        unsafe { debugfs_remove((*priv_).dfs_link[1]) };
    }

    unsafe { debugfs_remove_recursive((*priv_).dfs_root) };
}

static sof_ipc_flood_client_id_table: [auxiliary_device_id; 2] = [
    auxiliary_device_id {
        name: b"snd_sof.ipc_flood\0".as_ptr().cast(),
    },
    auxiliary_device_id {
        name: core::ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(auxiliary, sof_ipc_flood_client_id_table); */

/*
 * No need for driver pm_ops as the generic pm callbacks in the auxiliary bus
 * type are enough to ensure that the parent SOF device resumes to bring the DSP
 * back to D0.
 * Driver name will be set based on KBUILD_MODNAME.
 */
static mut sof_ipc_flood_client_drv: auxiliary_driver = auxiliary_driver {
    probe: Some(sof_ipc_flood_probe),
    remove: Some(sof_ipc_flood_remove),

    id_table: sof_ipc_flood_client_id_table.as_ptr(),
};

/* module_auxiliary_driver(sof_ipc_flood_client_drv); */

/* MODULE_LICENSE("GPL"); */
/* MODULE_DESCRIPTION("SOF IPC Flood Test Client Driver"); */
/* MODULE_IMPORT_NS("SND_SOC_SOF_CLIENT"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
