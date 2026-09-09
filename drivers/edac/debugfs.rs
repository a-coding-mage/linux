// SPDX-License-Identifier: GPL-2.0-only

// Declarations supplied by the Linux kernel and EDAC headers are intentionally
// left as external dependencies.

static mut edac_debugfs: *mut dentry = core::ptr::null_mut();

unsafe extern "C" {
    fn to_mci(dev: *mut device) -> *mut mem_ctl_info;
    fn printk(format: *const core::ffi::c_char, ...);
    fn edac_mc_handle_error(
        error_type: hw_event_mc_err_type,
        mci: *mut mem_ctl_info,
        error_count: u16,
        page_frame_number: u64,
        offset: u32,
        syndrome: u32,
        layer0: u32,
        layer1: u32,
        layer2: u32,
        label: *const core::ffi::c_char,
        detail: *const core::ffi::c_char,
    );
    fn simple_open(file: *mut file, inode: *mut inode) -> i32;
    fn generic_file_llseek(file: *mut file, offset: i64, whence: i32) -> i64;
    fn debugfs_create_dir(name: *const core::ffi::c_char, parent: *mut dentry) -> *mut dentry;
    fn debugfs_remove_recursive(dentry: *mut dentry);
    fn sprintf(buffer: *mut core::ffi::c_char, format: *const core::ffi::c_char, ...);
    fn debugfs_create_u8(name: *const core::ffi::c_char, mode: umode_t,
                         parent: *mut dentry, value: *mut u8) -> *mut dentry;
    fn debugfs_create_bool(name: *const core::ffi::c_char, mode: umode_t,
                           parent: *mut dentry, value: *mut bool) -> *mut dentry;
    fn debugfs_create_u16(name: *const core::ffi::c_char, mode: umode_t,
                          parent: *mut dentry, value: *mut u16) -> *mut dentry;
    fn debugfs_create_file(name: *const core::ffi::c_char, mode: umode_t,
                           parent: *mut dentry, data: *mut core::ffi::c_void,
                           fops: *const file_operations) -> *mut dentry;
    fn debugfs_create_x8(name: *const core::ffi::c_char, mode: umode_t,
                          parent: *mut dentry, value: *mut u8) -> *mut dentry;
    fn debugfs_create_x16(name: *const core::ffi::c_char, mode: umode_t,
                           parent: *mut dentry, value: *mut u16) -> *mut dentry;
    fn debugfs_create_x32(name: *const core::ffi::c_char, mode: umode_t,
                           parent: *mut dentry, value: *mut u32) -> *mut dentry;
}

static mut debug_fake_inject_fops: file_operations = file_operations {
    open: Some(simple_open),
    write: Some(edac_fake_inject_write),
    llseek: Some(generic_file_llseek),
};

unsafe extern "C" fn edac_fake_inject_write(
    file: *mut file,
    _data: *const core::ffi::c_char,
    count: usize,
    _ppos: *mut i64,
) -> isize {
    let dev = (*file).private_data as *mut device;
    let mci = to_mci(dev);
    static mut type_: hw_event_mc_err_type = HW_EVENT_ERR_CORRECTED;
    let mut errcount = (*mci).fake_inject_count;

    if errcount == 0 {
        errcount = 1;
    }

    type_ = if (*mci).fake_inject_ue {
        HW_EVENT_ERR_UNCORRECTED
    } else {
        HW_EVENT_ERR_CORRECTED
    };

    printk(b"Generating %d %s fake error%s to %d.%d.%d to test core handling. NOTE: this won't test the driver-specific decoding logic.\n\0".as_ptr() as *const _,
        errcount,
        if type_ == HW_EVENT_ERR_UNCORRECTED { b"UE\0".as_ptr() } else { b"CE\0".as_ptr() },
        if errcount == 1 { b"\0".as_ptr() } else { b"s\0".as_ptr() },
        (*mci).fake_inject_layer[0], (*mci).fake_inject_layer[1], (*mci).fake_inject_layer[2]);

    edac_mc_handle_error(type_, mci, errcount, 0, 0, 0,
        (*mci).fake_inject_layer[0], (*mci).fake_inject_layer[1],
        (*mci).fake_inject_layer[2], b"FAKE ERROR\0".as_ptr() as *const _,
        b"for EDAC testing only\0".as_ptr() as *const _);

    count as isize
}

pub unsafe extern "C" fn edac_debugfs_init() {
    edac_debugfs = debugfs_create_dir(b"edac\0".as_ptr() as *const _, core::ptr::null_mut());
}

pub unsafe extern "C" fn edac_debugfs_exit() {
    debugfs_remove_recursive(edac_debugfs);
}

pub unsafe extern "C" fn edac_create_debugfs_nodes(mci: *mut mem_ctl_info) {
    let parent = debugfs_create_dir((*mci).dev.kobj.name, edac_debugfs);
    let mut name = [0i8; 80];
    let mut i = 0;
    while i < (*mci).n_layers {
        sprintf(name.as_mut_ptr(), b"fake_inject_%s\0".as_ptr() as *const _,
            edac_layer_name[(*mci).layers[i].type_]);
        debugfs_create_u8(name.as_ptr(), S_IRUGO | S_IWUSR, parent,
            &mut (*mci).fake_inject_layer[i]);
        i += 1;
    }
    debugfs_create_bool(b"fake_inject_ue\0".as_ptr() as *const _, S_IRUGO | S_IWUSR,
        parent, &mut (*mci).fake_inject_ue);
    debugfs_create_u16(b"fake_inject_count\0".as_ptr() as *const _, S_IRUGO | S_IWUSR,
        parent, &mut (*mci).fake_inject_count);
    debugfs_create_file(b"fake_inject\0".as_ptr() as *const _, S_IWUSR, parent,
        &mut (*mci).dev as *mut _ as *mut _, &debug_fake_inject_fops);
    (*mci).debugfs = parent;
}

// Create a toplevel dir under EDAC's debugfs hierarchy
pub unsafe extern "C" fn edac_debugfs_create_dir(dirname: *const core::ffi::c_char) -> *mut dentry {
    if edac_debugfs.is_null() { return core::ptr::null_mut(); }
    debugfs_create_dir(dirname, edac_debugfs)
}

// Create a toplevel dir under EDAC's debugfs hierarchy with parent @parent
pub unsafe extern "C" fn edac_debugfs_create_dir_at(dirname: *const core::ffi::c_char, parent: *mut dentry) -> *mut dentry {
    debugfs_create_dir(dirname, parent)
}

pub unsafe extern "C" fn edac_debugfs_create_file(name: *const core::ffi::c_char, mode: umode_t,
    mut parent: *mut dentry, data: *mut core::ffi::c_void, fops: *const file_operations) -> *mut dentry {
    if parent.is_null() { parent = edac_debugfs; }
    debugfs_create_file(name, mode, parent, data, fops)
}

pub unsafe extern "C" fn edac_debugfs_create_x8(name: *const core::ffi::c_char, mode: umode_t,
    mut parent: *mut dentry, value: *mut u8) {
    if parent.is_null() { parent = edac_debugfs; }
    debugfs_create_x8(name, mode, parent, value);
}

pub unsafe extern "C" fn edac_debugfs_create_x16(name: *const core::ffi::c_char, mode: umode_t,
    mut parent: *mut dentry, value: *mut u16) {
    if parent.is_null() { parent = edac_debugfs; }
    debugfs_create_x16(name, mode, parent, value);
}

pub unsafe extern "C" fn edac_debugfs_create_x32(name: *const core::ffi::c_char, mode: umode_t,
    mut parent: *mut dentry, value: *mut u32) {
    if parent.is_null() { parent = edac_debugfs; }
    debugfs_create_x32(name, mode, parent, value);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
