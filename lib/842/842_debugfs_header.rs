/* SPDX-License-Identifier: GPL-2.0 */

// Translated from 842_debugfs.h.
// Original dependency: <linux/debugfs.h>

static mut SW842_TEMPLATE_COUNTS: bool = false;
// Original: module_param_named(template_counts, sw842_template_counts, bool, 0444);

// `atomic_t`, `dentry`, `OPS_MAX`, `MODULE_NAME`, debugfs helpers, and errno
// constants are supplied by the surrounding kernel translation.
static mut TEMPLATE_COUNT: [atomic_t; OPS_MAX] = [atomic_t::new(0); OPS_MAX];
static mut TEMPLATE_REPEAT_COUNT: atomic_t = atomic_t::new(0);
static mut TEMPLATE_ZEROS_COUNT: atomic_t = atomic_t::new(0);
static mut TEMPLATE_SHORT_DATA_COUNT: atomic_t = atomic_t::new(0);
static mut TEMPLATE_END_COUNT: atomic_t = atomic_t::new(0);

static mut SW842_DEBUGFS_ROOT: *mut dentry = core::ptr::null_mut();

unsafe fn sw842_debugfs_create() -> i32 {
    let m: umode_t = S_IRUGO | S_IWUSR;
    let mut i: i32;

    if !debugfs_initialized() {
        return -ENODEV;
    }

    SW842_DEBUGFS_ROOT = debugfs_create_dir(MODULE_NAME, core::ptr::null_mut());

    i = 0;
    while (i as usize) < TEMPLATE_COUNT.len() {
        let mut name = [0i8; 32];

        snprintf(name.as_mut_ptr(), 32, c"template_%02x".as_ptr(), i);
        debugfs_create_atomic_t(
            name.as_ptr(),
            m,
            SW842_DEBUGFS_ROOT,
            &mut TEMPLATE_COUNT[i as usize],
        );
        i += 1;
    }
    debugfs_create_atomic_t(
        c"template_repeat".as_ptr(),
        m,
        SW842_DEBUGFS_ROOT,
        &mut TEMPLATE_REPEAT_COUNT,
    );
    debugfs_create_atomic_t(
        c"template_zeros".as_ptr(),
        m,
        SW842_DEBUGFS_ROOT,
        &mut TEMPLATE_ZEROS_COUNT,
    );
    debugfs_create_atomic_t(
        c"template_short_data".as_ptr(),
        m,
        SW842_DEBUGFS_ROOT,
        &mut TEMPLATE_SHORT_DATA_COUNT,
    );
    debugfs_create_atomic_t(
        c"template_end".as_ptr(),
        m,
        SW842_DEBUGFS_ROOT,
        &mut TEMPLATE_END_COUNT,
    );

    0
}

unsafe fn sw842_debugfs_remove() {
    debugfs_remove_recursive(SW842_DEBUGFS_ROOT);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
