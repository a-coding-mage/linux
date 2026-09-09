// SPDX-License-Identifier: GPL-2.0-only
/* Inject a hwpoison memory failure on a arbitrary pfn */
// C dependencies: Linux kernel headers and "internal.h" provide the external
// types, constants, macros, functions, and debugfs/module registration APIs.

use core::ffi::c_void;

// Build-time CONFIG_MEMCG condition is preserved here.
#[cfg(CONFIG_MEMCG)]
static mut hwpoison_filter_memcg: u64 = 0;

static mut hwpoison_filter_enable: u32 = 0;
static mut hwpoison_filter_dev_major: u32 = !0u32;
static mut hwpoison_filter_dev_minor: u32 = !0u32;
static mut hwpoison_filter_flags_mask: u64 = 0;
static mut hwpoison_filter_flags_value: u64 = 0;

unsafe fn hwpoison_filter_dev(p: *mut page) -> i32 {
    let folio: *mut folio = page_folio(p);
    let mapping: *mut address_space;
    let dev: dev_t;

    if hwpoison_filter_dev_major == !0u32 && hwpoison_filter_dev_minor == !0u32 {
        return 0;
    }

    mapping = folio_mapping(folio);
    if mapping.is_null() || (*mapping).host.is_null() {
        return -EINVAL;
    }

    dev = (*(*mapping).host).i_sb.as_ref().unwrap().s_dev;
    if hwpoison_filter_dev_major != !0u32
        && hwpoison_filter_dev_major != MAJOR(dev)
    {
        return -EINVAL;
    }
    if hwpoison_filter_dev_minor != !0u32
        && hwpoison_filter_dev_minor != MINOR(dev)
    {
        return -EINVAL;
    }

    0
}

unsafe fn hwpoison_filter_flags(p: *mut page) -> i32 {
    if hwpoison_filter_flags_mask == 0 {
        return 0;
    }

    if (stable_page_flags(p) & hwpoison_filter_flags_mask)
        == hwpoison_filter_flags_value
    {
        0
    } else {
        -EINVAL
    }
}

/*
 * This allows stress tests to limit test scope to a collection of tasks
 * by putting them under some memcg. This prevents killing unrelated/important
 * processes such as /sbin/init. Note that the target task may share clean
 * pages with init (eg. libc text), which is harmless. If the target task
 * share _dirty_ pages with another task B, the test scheme must make sure B
 * is also included in the memcg. At last, due to race conditions this filter
 * can only guarantee that the page either belongs to the memcg tasks, or is
 * a freed page.
 */
#[cfg(CONFIG_MEMCG)]
unsafe fn hwpoison_filter_task(p: *mut page) -> i32 {
    if hwpoison_filter_memcg == 0 {
        return 0;
    }

    if page_cgroup_ino(p) != hwpoison_filter_memcg {
        return -EINVAL;
    }

    0
}

#[cfg(not(CONFIG_MEMCG))]
unsafe fn hwpoison_filter_task(_p: *mut page) -> i32 { 0 }

unsafe fn hwpoison_filter(p: *mut page) -> i32 {
    if hwpoison_filter_enable == 0 {
        return 0;
    }

    if hwpoison_filter_dev(p) != 0 {
        return -EINVAL;
    }
    if hwpoison_filter_flags(p) != 0 {
        return -EINVAL;
    }
    if hwpoison_filter_task(p) != 0 {
        return -EINVAL;
    }

    0
}

static mut hwpoison_dir: *mut dentry = core::ptr::null_mut();

unsafe fn hwpoison_inject(_data: *mut c_void, val: u64) -> i32 {
    let pfn: c_ulong = val as c_ulong;
    let p: *mut page;
    let folio: *mut folio;
    let mut err: i32;

    if capable(CAP_SYS_ADMIN) == 0 {
        return -EPERM;
    }
    if pfn_valid(pfn) == 0 {
        return -ENXIO;
    }

    p = pfn_to_page(pfn);
    folio = page_folio(p);

    if hwpoison_filter_enable != 0 {
        shake_folio(folio);
        /* This implies unable to support non-LRU pages except free page. */
        if !folio_test_lru(folio) && !folio_test_hugetlb(folio)
            && is_free_buddy_page(p) == 0
        {
            return 0;
        }

        /*
         * do a racy check to make sure PG_hwpoison will only be set for
         * the targeted owner (or on a free page).
         * memory_failure() will redo the check reliably inside page lock.
         */
        err = hwpoison_filter(&mut (*folio).page);
        if err != 0 {
            return 0;
        }
    }

    pr_info!("Injecting memory failure at pfn %#lx\n", pfn);
    err = memory_failure(pfn, MF_SW_SIMULATED);
    if err == -EOPNOTSUPP { 0 } else { err }
}

unsafe fn hwpoison_unpoison(_data: *mut c_void, val: u64) -> i32 {
    if capable(CAP_SYS_ADMIN) == 0 {
        return -EPERM;
    }
    unpoison_memory(val)
}

// DEFINE_DEBUGFS_ATTRIBUTE(hwpoison_fops, NULL, hwpoison_inject, "%lli\n");
// DEFINE_DEBUGFS_ATTRIBUTE(unpoison_fops, NULL, hwpoison_unpoison, "%lli\n");
extern "C" {
    static mut hwpoison_fops: file_operations;
    static mut unpoison_fops: file_operations;
}

unsafe fn pfn_inject_exit() {
    hwpoison_filter_enable = 0;
    hwpoison_filter_unregister();
    debugfs_remove_recursive(hwpoison_dir);
}

unsafe fn pfn_inject_init() -> i32 {
    hwpoison_dir = debugfs_create_dir(c"hwpoison".as_ptr(), core::ptr::null_mut());

    /*
     * Note that the below poison/unpoison interfaces do not involve
     * hardware status change, hence do not require hardware support.
     * They are mainly for testing hwpoison in software level.
     */
    debugfs_create_file(c"corrupt-pfn".as_ptr(), 0o200, hwpoison_dir,
                        core::ptr::null_mut(), &hwpoison_fops);
    debugfs_create_file(c"unpoison-pfn".as_ptr(), 0o200, hwpoison_dir,
                        core::ptr::null_mut(), &unpoison_fops);
    debugfs_create_u32(c"corrupt-filter-enable".as_ptr(), 0o600, hwpoison_dir,
                       &raw mut hwpoison_filter_enable);
    debugfs_create_u32(c"corrupt-filter-dev-major".as_ptr(), 0o600, hwpoison_dir,
                       &raw mut hwpoison_filter_dev_major);
    debugfs_create_u32(c"corrupt-filter-dev-minor".as_ptr(), 0o600, hwpoison_dir,
                       &raw mut hwpoison_filter_dev_minor);
    debugfs_create_u64(c"corrupt-filter-flags-mask".as_ptr(), 0o600, hwpoison_dir,
                       &raw mut hwpoison_filter_flags_mask);
    debugfs_create_u64(c"corrupt-filter-flags-value".as_ptr(), 0o600, hwpoison_dir,
                       &raw mut hwpoison_filter_flags_value);
    #[cfg(CONFIG_MEMCG)]
    debugfs_create_u64(c"corrupt-filter-memcg".as_ptr(), 0o600, hwpoison_dir,
                       &raw mut hwpoison_filter_memcg);

    hwpoison_filter_register(hwpoison_filter);
    0
}

// module_init(pfn_inject_init);
// module_exit(pfn_inject_exit);
// MODULE_DESCRIPTION("HWPoison pages injector");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
