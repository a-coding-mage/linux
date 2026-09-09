// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the Linux kernel headers are intentionally left as
// external Rust names.

#[repr(C)]
struct SbkRealloc {
    attr: fault_attr,
    devname: [c_char; IFNAMSIZ],
    filtered: bool,
}

static mut skb_realloc: SbkRealloc = SbkRealloc {
    attr: FAULT_ATTR_INITIALIZER,
    devname: [0; IFNAMSIZ],
    filtered: false,
};

unsafe fn should_fail_net_realloc_skb(skb: *mut sk_buff) -> bool {
    let net: *mut net_device = (*skb).dev;

    if skb_realloc.filtered
        && strncmp((*net).name.as_ptr(), skb_realloc.devname.as_ptr(), IFNAMSIZ) != 0
    {
        // device name filter set, but names do not match
        return false;
    }

    if !should_fail(&mut skb_realloc.attr, 1) {
        return false;
    }

    true
}

// ALLOW_ERROR_INJECTION(should_fail_net_realloc_skb, TRUE);

pub unsafe fn skb_might_realloc(skb: *mut sk_buff) {
    if !should_fail_net_realloc_skb(skb) {
        return;
    }

    pskb_expand_head(skb, 0, 0, GFP_ATOMIC);
}

// EXPORT_SYMBOL(skb_might_realloc);

unsafe extern "C" fn fail_skb_realloc_setup(str_: *mut c_char) -> c_int {
    setup_fault_attr(&mut skb_realloc.attr, str_)
}

// __setup("fail_skb_realloc=", fail_skb_realloc_setup);

unsafe fn reset_settings() {
    skb_realloc.filtered = false;
    memset(
        skb_realloc.devname.as_mut_ptr() as *mut c_void,
        0,
        IFNAMSIZ,
    );
}

unsafe extern "C" fn devname_write(
    _file: *mut file,
    buffer: *const c_char,
    count: usize,
    ppos: *mut loff_t,
) -> isize {
    let mut ret: isize;

    reset_settings();
    ret = simple_write_to_buffer(
        skb_realloc.devname.as_mut_ptr() as *mut c_void,
        IFNAMSIZ,
        ppos,
        buffer as *const c_void,
        count,
    );
    if ret < 0 {
        return ret;
    }

    skb_realloc.devname[IFNAMSIZ - 1] = 0;
    // Remove a possible \n at the end of devname
    strim(skb_realloc.devname.as_mut_ptr());

    if strnlen(skb_realloc.devname.as_ptr(), IFNAMSIZ) != 0 {
        skb_realloc.filtered = true;
    }

    count as isize
}

unsafe extern "C" fn devname_read(
    _file: *mut file,
    buffer: *mut c_char,
    size: usize,
    ppos: *mut loff_t,
) -> isize {
    if !skb_realloc.filtered {
        return 0;
    }

    simple_read_from_buffer(
        buffer as *mut c_void,
        size,
        ppos,
        skb_realloc.devname.as_ptr() as *const c_void,
        strlen(skb_realloc.devname.as_ptr()),
    )
}

static devname_ops: file_operations = file_operations {
    write: Some(devname_write),
    read: Some(devname_read),
};

unsafe extern "C" fn fail_skb_realloc_debugfs() -> c_int {
    let mode: umode_t = S_IFREG | 0o600;
    let dir: *mut dentry;

    dir = fault_create_debugfs_attr(
        b"fail_skb_realloc\0".as_ptr() as *const c_char,
        core::ptr::null_mut(),
        &mut skb_realloc.attr,
    );
    if IS_ERR(dir) {
        return PTR_ERR(dir);
    }

    debugfs_create_file(
        b"devname\0".as_ptr() as *const c_char,
        mode,
        dir,
        core::ptr::null_mut(),
        &devname_ops,
    );

    0
}

// late_initcall(fail_skb_realloc_debugfs);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
