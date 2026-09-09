// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux ACPI/procfs headers and local headers:
// proc_fs.h, seq_file.h, string_choices.h, suspend.h, bcd.h, acpi.h,
// uaccess.h, sleep.h, and internal.h.

unsafe fn acpi_system_wakeup_device_seq_show(
    seq: *mut seq_file,
    _offset: *mut core::ffi::c_void,
) -> i32 {
    let mut dev: *mut acpi_device;
    let mut tmp: *mut acpi_device;

    seq_printf(seq, "Device\tS-state\t  Status   Sysfs node\n");

    mutex_lock(&acpi_device_lock);
    list_for_each_entry_safe(dev, tmp, &mut acpi_wakeup_device_list, wakeup_list) {
        let mut entry: *mut acpi_device_physical_node;

        if !(*dev).wakeup.flags.valid {
            continue;
        }

        seq_printf(
            seq,
            "%s\t  S%llu\t",
            (*dev).pnp.bus_id,
            (*dev).wakeup.sleep_state,
        );

        mutex_lock(&(*dev).physical_node_lock);

        if (*dev).physical_node_count == 0 {
            seq_printf(
                seq,
                "%c%-8s\n",
                if (*dev).wakeup.flags.valid { '*' } else { ' ' },
                str_enabled_disabled(device_may_wakeup(&mut (*dev).dev)),
            );
        } else {
            let mut ldev: *mut device;
            list_for_each_entry!(entry, &(*dev).physical_node_list, node) {
                ldev = get_device((*entry).dev);
                if ldev.is_null() {
                    continue;
                }

                if !core::ptr::eq(
                    &(*entry).node,
                    (*dev).physical_node_list.next,
                ) {
                    seq_printf(seq, "\t\t");
                }

                seq_printf(
                    seq,
                    "%c%-8s  %s:%s\n",
                    if (*dev).wakeup.flags.valid { '*' } else { ' ' },
                    str_enabled_disabled(
                        device_may_wakeup(ldev) || device_may_wakeup(&mut (*dev).dev),
                    ),
                    if !(*ldev).bus.is_null() {
                        (*(*ldev).bus).name
                    } else {
                        "no-bus"
                    },
                    dev_name(ldev),
                );
                put_device(ldev);
            }
        }

        mutex_unlock(&(*dev).physical_node_lock);
    }
    mutex_unlock(&acpi_device_lock);
    0
}

unsafe fn physical_device_enable_wakeup(adev: *mut acpi_device) {
    let mut entry: *mut acpi_device_physical_node;

    mutex_lock(&(*adev).physical_node_lock);

    list_for_each_entry!(entry, &(*adev).physical_node_list, node) {
        if !(*entry).dev.is_null() && device_can_wakeup((*entry).dev) {
            let enable = !device_may_wakeup((*entry).dev);
            device_set_wakeup_enable((*entry).dev, enable);
        }
    }

    mutex_unlock(&(*adev).physical_node_lock);
}

unsafe fn acpi_system_write_wakeup_device(
    _file: *mut file,
    buffer: *const core::ffi::c_char,
    mut count: usize,
    _ppos: *mut loff_t,
) -> isize {
    let mut dev: *mut acpi_device;
    let mut tmp: *mut acpi_device;
    let mut strbuf = [0i8; 5];
    let mut str_ = [0i8; 5];

    if count > 4 {
        count = 4;
    }

    if copy_from_user(strbuf.as_mut_ptr(), buffer, count) != 0 {
        return -EFAULT as isize;
    }
    strbuf[count] = 0;
    sscanf(strbuf.as_ptr(), "%s", str_.as_mut_ptr());

    mutex_lock(&acpi_device_lock);
    list_for_each_entry_safe(dev, tmp, &mut acpi_wakeup_device_list, wakeup_list) {
        if !(*dev).wakeup.flags.valid {
            continue;
        }

        if strncmp((*dev).pnp.bus_id, str_.as_ptr(), 4) == 0 {
            if device_can_wakeup(&mut (*dev).dev) {
                let enable = !device_may_wakeup(&mut (*dev).dev);
                device_set_wakeup_enable(&mut (*dev).dev, enable);
            } else {
                physical_device_enable_wakeup(dev);
            }
            break;
        }
    }
    mutex_unlock(&acpi_device_lock);
    count as isize
}

unsafe fn acpi_system_wakeup_device_open_fs(
    inode: *mut inode,
    file: *mut file,
) -> i32 {
    single_open(
        file,
        Some(acpi_system_wakeup_device_seq_show),
        pde_data(inode),
    )
}

static acpi_system_wakeup_device_proc_ops: proc_ops = proc_ops {
    proc_open: Some(acpi_system_wakeup_device_open_fs),
    proc_read: Some(seq_read),
    proc_write: Some(acpi_system_write_wakeup_device),
    proc_lseek: Some(seq_lseek),
    proc_release: Some(single_release),
};

pub unsafe fn acpi_sleep_proc_init() {
    // 'wakeup device' [R/W]
    proc_create(
        "wakeup",
        0o644,
        acpi_root_dir,
        &acpi_system_wakeup_device_proc_ops,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
