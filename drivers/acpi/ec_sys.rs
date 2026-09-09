// SPDX-License-Identifier: GPL-2.0-only
/*
 * ec_sys.c
 *
 * Copyright (C) 2010 SUSE Products GmbH/Novell
 * Author:
 *      Thomas Renninger <trenn@suse.de>
 */

// Dependencies supplied by the surrounding kernel translation.

// MODULE_AUTHOR("Thomas Renninger <trenn@suse.de>");
// MODULE_DESCRIPTION("ACPI EC sysfs access driver");
// MODULE_LICENSE("GPL");

static mut write_support: bool = false;
// module_param_hw(write_support, bool, other, 0644);
// MODULE_PARM_DESC(write_support, "Dangerous, reboot and removal of battery may be needed.");

const EC_SPACE_SIZE: usize = 256;

static mut acpi_ec_debugfs_dir: *mut dentry = core::ptr::null_mut();

unsafe fn acpi_ec_read_io(
    _f: *mut file,
    buf: *mut core::ffi::c_char,
    mut count: usize,
    off: *mut loff_t,
) -> ssize_t {
    /* Use this if support reading/writing multiple ECs exists in ec.c:
     * struct acpi_ec *ec = ((struct seq_file *)f->private_data)->private;
     */
    let mut size: usize = EC_SPACE_SIZE;
    let init_off: loff_t = *off;
    let mut err: c_int;

    if *off >= size as loff_t {
        return 0;
    }
    if *off + count as loff_t >= size as loff_t {
        size = (size as loff_t - *off) as usize;
        count = size;
    } else {
        size = count;
    }

    while size != 0 {
        let mut byte_read: u8 = 0;
        err = ec_read(*off, &mut byte_read);
        if err != 0 {
            return err as ssize_t;
        }
        if put_user(byte_read, buf.offset((*off - init_off) as isize) as *mut u8) != 0 {
            if *off - init_off != 0 {
                return *off - init_off; /* partial read */
            }
            return -EFAULT as ssize_t;
        }
        *off += 1;
        size -= 1;
    }
    count as ssize_t
}

unsafe fn acpi_ec_write_io(
    _f: *mut file,
    buf: *const core::ffi::c_char,
    mut count: usize,
    off: *mut loff_t,
) -> ssize_t {
    /* Use this if support reading/writing multiple ECs exists in ec.c:
     * struct acpi_ec *ec = ((struct seq_file *)f->private_data)->private;
     */

    let mut size: usize = count;
    let init_off: loff_t = *off;
    let mut err: c_int;

    if !write_support {
        return -EINVAL as ssize_t;
    }

    if *off >= EC_SPACE_SIZE as loff_t {
        return 0;
    }
    if *off + count as loff_t >= EC_SPACE_SIZE as loff_t {
        size = (EC_SPACE_SIZE as loff_t - *off) as usize;
        count = size;
    }

    while size != 0 {
        let mut byte_write: u8 = 0;
        if get_user(&mut byte_write, buf.offset((*off - init_off) as isize) as *const u8) != 0 {
            if *off - init_off != 0 {
                return *off - init_off; /* partial write */
            }
            return -EFAULT as ssize_t;
        }
        err = ec_write(*off, byte_write);
        if err != 0 {
            return err as ssize_t;
        }

        *off += 1;
        size -= 1;
    }
    count as ssize_t
}

static acpi_ec_io_ops: file_operations = file_operations {
    .owner = THIS_MODULE,
    .open = simple_open,
    .read = acpi_ec_read_io,
    .write = acpi_ec_write_io,
    .llseek = default_llseek,
};

unsafe fn acpi_ec_add_debugfs(ec: *mut acpi_ec, ec_device_count: c_uint) {
    let mut dev_dir: *mut dentry;
    let mut name = [0 as core::ffi::c_char; 64];
    let mut mode: umode_t = 0o400;

    if ec_device_count == 0 {
        acpi_ec_debugfs_dir = debugfs_create_dir(c"ec".as_ptr(), core::ptr::null_mut());
    }

    sprintf(name.as_mut_ptr(), c"ec%u".as_ptr(), ec_device_count);
    dev_dir = debugfs_create_dir(name.as_ptr(), acpi_ec_debugfs_dir);

    debugfs_create_x32(c"gpe".as_ptr(), 0o444, dev_dir, &mut (*first_ec).gpe);
    debugfs_create_bool(c"use_global_lock".as_ptr(), 0o444, dev_dir,
                        &mut (*first_ec).global_lock);

    if write_support {
        mode = 0o600;
    }
    debugfs_create_file(c"io".as_ptr(), mode, dev_dir, ec, &acpi_ec_io_ops);
}

unsafe fn acpi_ec_sys_init() -> c_int {
    if !first_ec.is_null() {
        acpi_ec_add_debugfs(first_ec, 0);
    }
    0
}

unsafe fn acpi_ec_sys_exit() {
    debugfs_remove_recursive(acpi_ec_debugfs_dir);
}

// module_init(acpi_ec_sys_init);
// module_exit(acpi_ec_sys_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
