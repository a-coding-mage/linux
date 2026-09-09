// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by Linux kernel headers and other translation units:
// linux/fault-inject.h, linux/error-injection.h, linux/debugfs.h,
// linux/slab.h, linux/mm.h, and "slab.h".

#[repr(C)]
struct Failslab {
    attr: fault_attr,
    ignore_gfp_reclaim: bool,
    cache_filter: bool,
}

static mut failslab: Failslab = Failslab {
    attr: FAULT_ATTR_INITIALIZER,
    ignore_gfp_reclaim: true,
    cache_filter: false,
};

pub unsafe fn should_failslab(s: *mut kmem_cache, gfpflags: gfp_t) -> i32 {
    let mut flags: i32 = 0;

    /* No fault-injection for bootstrap cache */
    if s == kmem_cache {
        return 0;
    }

    if gfpflags & __GFP_NOFAIL != 0 {
        return 0;
    }

    if failslab.ignore_gfp_reclaim && (gfpflags & __GFP_DIRECT_RECLAIM != 0) {
        return 0;
    }

    if failslab.cache_filter && ((*s).flags & SLAB_FAILSLAB == 0) {
        return 0;
    }

    /*
     * In some cases, it expects to specify __GFP_NOWARN
     * to avoid printing any information(not just a warning),
     * thus avoiding deadlocks. See commit 6b9dbedbe349 for
     * details.
     */
    if gfpflags & __GFP_NOWARN != 0 {
        flags |= FAULT_NOWARN;
    }

    if should_fail_ex(&mut failslab.attr, (*s).object_size, flags) {
        -ENOMEM
    } else {
        0
    }
}

// ALLOW_ERROR_INJECTION(should_failslab, ERRNO)

unsafe fn setup_failslab(str_: *mut core::ffi::c_char) -> i32 {
    setup_fault_attr(&mut failslab.attr, str_)
}

// __setup("failslab=", setup_failslab)

#[cfg(CONFIG_FAULT_INJECTION_DEBUG_FS)]
unsafe fn failslab_debugfs_init() -> i32 {
    let mut dir: *mut dentry;
    let mode: umode_t = S_IFREG | 0o600;

    dir = fault_create_debugfs_attr(
        c"failslab".as_ptr(),
        core::ptr::null_mut(),
        &mut failslab.attr,
    );
    if IS_ERR(dir) {
        return PTR_ERR(dir);
    }

    debugfs_create_bool(
        c"ignore-gfp-wait".as_ptr(),
        mode,
        dir,
        &mut failslab.ignore_gfp_reclaim,
    );
    debugfs_create_bool(
        c"cache-filter".as_ptr(),
        mode,
        dir,
        &mut failslab.cache_filter,
    );

    0
}

// late_initcall(failslab_debugfs_init)

// #endif /* CONFIG_FAULT_INJECTION_DEBUG_FS */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
