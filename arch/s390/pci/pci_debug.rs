// SPDX-License-Identifier: GPL-2.0
/*
 *  Copyright IBM Corp. 2012,2015
 *
 *  Author(s):
 *    Jan Glauber <jang@linux.vnet.ibm.com>
 */

// Translated from the Linux kernel implementation. Required kernel and
// architecture types, constants, and functions are supplied externally.

static mut debugfs_root: *mut dentry = core::ptr::null_mut();
pub static mut pci_debug_msg_id: *mut debug_info_t = core::ptr::null_mut();
pub static mut pci_debug_err_id: *mut debug_info_t = core::ptr::null_mut();

static mut pci_common_names: [*mut u8; 4] = [
    b"Load operations\0".as_ptr() as *mut u8,
    b"Store operations\0".as_ptr() as *mut u8,
    b"Store block operations\0".as_ptr() as *mut u8,
    b"Refresh operations\0".as_ptr() as *mut u8,
];

static mut pci_fmt0_names: [*mut u8; 2] = [
    b"DMA read bytes\0".as_ptr() as *mut u8,
    b"DMA write bytes\0".as_ptr() as *mut u8,
];

static mut pci_fmt1_names: [*mut u8; 4] = [
    b"Received bytes\0".as_ptr() as *mut u8,
    b"Received packets\0".as_ptr() as *mut u8,
    b"Transmitted bytes\0".as_ptr() as *mut u8,
    b"Transmitted packets\0".as_ptr() as *mut u8,
];

static mut pci_fmt2_names: [*mut u8; 2] = [
    b"Consumed work units\0".as_ptr() as *mut u8,
    b"Maximum work units\0".as_ptr() as *mut u8,
];

static mut pci_fmt3_names: [*mut u8; 1] = [
    b"Transmitted bytes\0".as_ptr() as *mut u8,
];

static mut pci_sw_names: [*mut u8; 5] = [
    b"Mapped pages\0".as_ptr() as *mut u8,
    b"Unmapped pages\0".as_ptr() as *mut u8,
    b"Global RPCITs\0".as_ptr() as *mut u8,
    b"Sync Map RPCITs\0".as_ptr() as *mut u8,
    b"Sync RPCITs\0".as_ptr() as *mut u8,
];

unsafe fn pci_fmb_show(m: *mut seq_file, name: *mut *mut u8, length: i32,
                       mut data: *mut u64) {
    let mut i = 0;
    while i < length {
        seq_printf(m, b"%26s:\t%llu\n\0".as_ptr(), *name.add(i as usize), *data);
        i += 1;
        data = data.add(1);
    }
}

unsafe fn pci_sw_counter_show(m: *mut seq_file) {
    let zdev = (*m).private as *mut zpci_dev;
    let mut flags: ulong = 0;
    spin_lock_irqsave(&mut (*zdev).dom_lock, &mut flags);
    let ctrs = zpci_get_iommu_ctrs((*m).private);
    if !ctrs.is_null() {
        let mut counter = &mut (*ctrs).mapped_pages as *mut atomic64_t;
        let mut i = 0;
        while i < 5 {
            seq_printf(m, b"%26s:\t%llu\n\0".as_ptr(), pci_sw_names[i], atomic64_read(counter));
            counter = counter.add(1);
            i += 1;
        }
    }
    spin_unlock_irqrestore(&mut (*zdev).dom_lock, flags);
}

unsafe extern "C" fn pci_perf_show(m: *mut seq_file, _v: *mut core::ffi::c_void) -> i32 {
    let zdev = (*m).private as *mut zpci_dev;
    if zdev.is_null() { return 0; }
    mutex_lock(&mut (*zdev).fmb_lock);
    if (*zdev).fmb.is_null() {
        mutex_unlock(&mut (*zdev).fmb_lock);
        seq_puts(m, b"FMB statistics disabled\n\0".as_ptr());
        return 0;
    }
    seq_printf(m, b"Update interval: %u ms\n\0".as_ptr(), (*zdev).fmb_update);
    seq_printf(m, b"Samples: %u\n\0".as_ptr(), (*(*zdev).fmb).samples);
    seq_printf(m, b"Last update TOD: %Lx\n\0".as_ptr(), (*(*zdev).fmb).last_update);
    pci_fmb_show(m, pci_common_names.as_mut_ptr() as *mut *mut u8, 4, &mut (*(*zdev).fmb).ld_ops);
    match (*(*zdev).fmb).format {
        0 => if (*(*zdev).fmb).fmt_ind & ZPCI_FMB_DMA_COUNTER_VALID != 0 { pci_fmb_show(m, pci_fmt0_names.as_mut_ptr() as *mut *mut u8, 2, &mut (*(*zdev).fmb).fmt0.dma_rbytes); },
        1 => pci_fmb_show(m, pci_fmt1_names.as_mut_ptr() as *mut *mut u8, 4, &mut (*(*zdev).fmb).fmt1.rx_bytes),
        2 => pci_fmb_show(m, pci_fmt2_names.as_mut_ptr() as *mut *mut u8, 2, &mut (*(*zdev).fmb).fmt2.consumed_work_units),
        3 => pci_fmb_show(m, pci_fmt3_names.as_mut_ptr() as *mut *mut u8, 1, &mut (*(*zdev).fmb).fmt3.tx_bytes),
        _ => seq_puts(m, b"Unknown format\n\0".as_ptr()),
    }
    pci_sw_counter_show(m);
    mutex_unlock(&mut (*zdev).fmb_lock);
    0
}

unsafe extern "C" fn pci_perf_seq_write(file: *mut file, ubuf: *const u8,
                                         count: usize, _off: *mut loff_t) -> isize {
    let zdev = (*(file).private_data as *mut seq_file).private as *mut zpci_dev;
    if zdev.is_null() { return 0; }
    let mut val: ulong = 0;
    let mut rc = kstrtoul_from_user(ubuf, count, 10, &mut val);
    if rc != 0 { return rc as isize; }
    mutex_lock(&mut (*zdev).fmb_lock);
    match val { 0 => rc = zpci_fmb_disable_device(zdev), 1 => rc = zpci_fmb_enable_device(zdev), _ => {} }
    mutex_unlock(&mut (*zdev).fmb_lock);
    if rc != 0 { rc as isize } else { count as isize }
}

unsafe extern "C" fn pci_perf_seq_open(_inode: *mut inode, filp: *mut file) -> i32 {
    single_open(filp, pci_perf_show, file_inode(filp).i_private)
}

static debugfs_pci_perf_fops: file_operations = file_operations {
    open: Some(pci_perf_seq_open), read: Some(seq_read), write: Some(pci_perf_seq_write),
    llseek: Some(seq_lseek), release: Some(single_release),
};

pub unsafe extern "C" fn zpci_debug_init_device(zdev: *mut zpci_dev, name: *const u8) {
    (*zdev).debugfs_dev = debugfs_create_dir(name, debugfs_root);
    debugfs_create_file(b"statistics\0".as_ptr(), S_IFREG | S_IRUGO | S_IWUSR,
                        (*zdev).debugfs_dev, zdev as *mut core::ffi::c_void,
                        &debugfs_pci_perf_fops);
}

pub unsafe extern "C" fn zpci_debug_exit_device(zdev: *mut zpci_dev) {
    debugfs_remove_recursive((*zdev).debugfs_dev);
}

pub unsafe extern "C" fn zpci_debug_init() -> i32 {
    pci_debug_msg_id = debug_register(b"pci_msg\0".as_ptr(), 8, 1, 8 * core::mem::size_of::<c_long>());
    if pci_debug_msg_id.is_null() { return -EINVAL; }
    debug_register_view(pci_debug_msg_id, &debug_sprintf_view);
    debug_set_level(pci_debug_msg_id, 3);
    pci_debug_err_id = debug_register(b"pci_error\0".as_ptr(), 2, 1, 16);
    if pci_debug_err_id.is_null() { return -EINVAL; }
    debug_register_view(pci_debug_err_id, &debug_hex_ascii_view);
    debug_set_level(pci_debug_err_id, 3);
    debugfs_root = debugfs_create_dir(b"pci\0".as_ptr(), core::ptr::null_mut());
    0
}

pub unsafe extern "C" fn zpci_debug_exit() {
    debug_unregister(pci_debug_msg_id);
    debug_unregister(pci_debug_err_id);
    debugfs_remove(debugfs_root);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
