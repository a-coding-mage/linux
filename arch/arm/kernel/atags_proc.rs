// SPDX-License-Identifier: GPL-2.0
//
// Dependencies supplied by the Linux kernel headers are intentionally left as
// external names: linux/slab.h, linux/proc_fs.h, asm/setup.h, asm/types.h,
// and asm/page.h.

#[repr(C)]
struct buffer {
    size: usize,
    data: [core::ffi::c_char; 0],
}

unsafe fn atags_read(
    file: *mut file,
    buf: *mut core::ffi::c_char,
    count: usize,
    ppos: *mut loff_t,
) -> ssize_t {
    let b = unsafe { pde_data(unsafe { file_inode(file) }) } as *mut buffer;
    unsafe {
        simple_read_from_buffer(
            buf,
            count,
            ppos,
            (*b).data.as_ptr(),
            (*b).size,
        )
    }
}

static atags_proc_ops: proc_ops = proc_ops {
    proc_read: Some(atags_read),
    proc_lseek: Some(default_llseek),
};

const BOOT_PARAMS_SIZE: usize = 1536;
static mut atags_copy: [core::ffi::c_char; BOOT_PARAMS_SIZE] = [0; BOOT_PARAMS_SIZE];

unsafe fn save_atags(tags: *const tag) {
    unsafe {
        memcpy(
            atags_copy.as_mut_ptr(),
            tags as *const core::ffi::c_void,
            core::mem::size_of::<[core::ffi::c_char; BOOT_PARAMS_SIZE]>(),
        );
    }
}

unsafe fn init_atags_procfs() -> c_int {
    /*
     * This cannot go into save_atags() because kmalloc and proc don't work
     * yet when it is called.
     */
    let mut tags_entry: *mut proc_dir_entry;
    let mut tag = unsafe { atags_copy.as_mut_ptr() as *mut tag };
    let mut b: *mut buffer = core::ptr::null_mut();
    let size: usize;

    if unsafe { (*tag).hdr.tag } != ATAG_CORE {
        unsafe { pr_info(c"No ATAGs?\n".as_ptr()); }
        return -EINVAL;
    }

    while unsafe { (*tag).hdr.size } != 0 {
        tag = unsafe { tag_next(tag) };
    }

    /* include the terminating ATAG_NONE */
    size = (tag as *mut core::ffi::c_char as usize)
        - (unsafe { atags_copy.as_mut_ptr() as usize })
        + core::mem::size_of::<tag_header>();

    unsafe { WARN_ON((*tag).hdr.tag != ATAG_NONE); }

    b = unsafe { kmalloc_flex(size) };
    if b.is_null() {
        unsafe { kfree(b as *mut core::ffi::c_void); }
        unsafe { pr_err(c"Exporting ATAGs: not enough memory\n".as_ptr()); }
        return -ENOMEM;
    }

    unsafe {
        (*b).size = size;
        memcpy(
            (*b).data.as_mut_ptr(),
            atags_copy.as_ptr() as *const core::ffi::c_void,
            size,
        );
    }

    tags_entry = unsafe {
        proc_create_data(
            c"atags".as_ptr(),
            0o400,
            core::ptr::null_mut(),
            &atags_proc_ops,
            b as *mut core::ffi::c_void,
        )
    };
    if tags_entry.is_null() {
        unsafe { kfree(b as *mut core::ffi::c_void); }
        unsafe { pr_err(c"Exporting ATAGs: not enough memory\n".as_ptr()); }
        return -ENOMEM;
    }

    return 0;
}

// arch_initcall(init_atags_procfs);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
