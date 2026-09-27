/* SPDX-License-Identifier: GPL-2.0 */

/* Linux header dependencies are supplied by other translated files. */

#[repr(C)]
pub struct swsusp_info {
    pub uts: new_utsname,
    pub version_code: u32,
    pub num_physpages: kernel::ffi::c_ulong,
    pub cpus: kernel::ffi::c_int,
    pub image_pages: kernel::ffi::c_ulong,
    pub pages: kernel::ffi::c_ulong,
    pub size: kernel::ffi::c_ulong,
}
/* C declaration uses __aligned(PAGE_SIZE). */

#[cfg(any(CONFIG_SUSPEND, CONFIG_HIBERNATION))]
extern "C" {
    pub fn pm_sleep_fs_sync() -> kernel::ffi::c_int;
    pub static mut filesystem_freeze_enabled: bool;
}

#[cfg(CONFIG_HIBERNATION)]
extern "C" {
    pub fn hibernate_reserved_size_init();
    pub fn hibernate_image_size_init();
    pub fn swsusp_save() -> kernel::ffi::c_int;
    pub static mut freezer_test_done: bool;
    pub static mut hib_comp_algo: [kernel::ffi::c_char; CRYPTO_MAX_ALG_NAME];
    pub static mut swsusp_header_flags: kernel::ffi::c_uint;
    pub fn hibernation_snapshot(platform_mode: kernel::ffi::c_int) -> kernel::ffi::c_int;
    pub fn hibernation_restore(platform_mode: kernel::ffi::c_int) -> kernel::ffi::c_int;
    pub fn hibernation_platform_enter() -> kernel::ffi::c_int;
    pub fn hibernation_in_progress() -> bool;
}

#[cfg(CONFIG_ARCH_HIBERNATION_HEADER)]
pub const MAX_ARCH_HEADER_SIZE: usize = core::mem::size_of::<new_utsname>() + 4;

#[cfg(CONFIG_ARCH_HIBERNATION_HEADER)]
pub unsafe fn init_header_complete(info: *mut swsusp_info) -> kernel::ffi::c_int {
    arch_hibernation_header_save(info, MAX_ARCH_HEADER_SIZE)
}

#[cfg(CONFIG_ARCH_HIBERNATION_HEADER)]
pub unsafe fn check_image_kernel(info: *mut swsusp_info) -> *const kernel::ffi::c_char {
    if arch_hibernation_header_restore(info) {
        b"architecture specific data\0".as_ptr() as *const kernel::ffi::c_char
    } else {
        core::ptr::null()
    }
}

pub const PAGES_FOR_IO: kernel::ffi::c_ulong = ((4096 * 1024) >> PAGE_SHIFT) as kernel::ffi::c_ulong;
pub const SPARE_PAGES: kernel::ffi::c_ulong = ((1024 * 1024) >> PAGE_SHIFT) as kernel::ffi::c_ulong;

#[cfg(not(CONFIG_HIBERNATION))]
pub fn hibernate_reserved_size_init() {}
#[cfg(not(CONFIG_HIBERNATION))]
pub fn hibernate_image_size_init() {}
#[cfg(not(CONFIG_HIBERNATION))]
pub fn hibernation_in_progress() -> bool { false }

#[cfg(CONFIG_STRICT_KERNEL_RWX)]
extern "C" { pub fn enable_restore_image_protection(); }
#[cfg(not(CONFIG_STRICT_KERNEL_RWX))]
pub fn enable_restore_image_protection() {}

pub static mut image_size: kernel::ffi::c_ulong;
pub static mut reserved_size: kernel::ffi::c_ulong;
pub static mut in_suspend: kernel::ffi::c_int;
pub static mut swsusp_resume_device: dev_t;
pub static mut swsusp_resume_block: sector_t;

extern "C" {
    pub fn create_basic_memory_bitmaps() -> kernel::ffi::c_int;
    pub fn free_basic_memory_bitmaps();
    pub fn hibernate_preallocate_memory() -> kernel::ffi::c_int;
    pub fn clear_or_poison_free_pages();
}

#[repr(C)]
pub struct snapshot_handle {
    pub cur: kernel::ffi::c_uint,
    pub buffer: *mut kernel::ffi::c_void,
    pub sync_read: kernel::ffi::c_int,
}

pub unsafe fn data_of(handle: *mut snapshot_handle) -> *mut kernel::ffi::c_void {
    (*handle).buffer
}

extern "C" {
    pub fn snapshot_additional_pages(zone: *mut zone) -> kernel::ffi::c_uint;
    pub fn snapshot_get_image_size() -> kernel::ffi::c_ulong;
    pub fn snapshot_read_next(handle: *mut snapshot_handle) -> kernel::ffi::c_int;
    pub fn snapshot_write_next(handle: *mut snapshot_handle) -> kernel::ffi::c_int;
    pub fn snapshot_write_finalize(handle: *mut snapshot_handle) -> kernel::ffi::c_int;
    pub fn snapshot_image_loaded(handle: *mut snapshot_handle) -> bool;
    pub fn hibernate_acquire() -> bool;
    pub fn hibernate_release();
    pub fn alloc_swapdev_block(swap: kernel::ffi::c_int) -> sector_t;
    pub fn free_all_swap_pages(swap: kernel::ffi::c_int);
    pub fn swsusp_swap_in_use() -> kernel::ffi::c_int;
}

pub const SF_COMPRESSION_ALG_LZO: kernel::ffi::c_uint = 0;
pub const SF_PLATFORM_MODE: kernel::ffi::c_uint = 1;
pub const SF_NOCOMPRESS_MODE: kernel::ffi::c_uint = 2;
pub const SF_CRC32_MODE: kernel::ffi::c_uint = 4;
pub const SF_HW_SIG: kernel::ffi::c_uint = 8;
pub const SF_COMPRESSION_ALG_LZ4: kernel::ffi::c_uint = 16;

extern "C" {
    pub fn swsusp_check(exclusive: bool) -> kernel::ffi::c_int;
    pub fn swsusp_free();
    pub fn swsusp_read(flags_p: *mut kernel::ffi::c_uint) -> kernel::ffi::c_int;
    pub fn swsusp_write(flags: kernel::ffi::c_uint) -> kernel::ffi::c_int;
    pub fn swsusp_close();
}

#[cfg(CONFIG_SUSPEND)]
extern "C" { pub fn swsusp_unmark() -> kernel::ffi::c_int; }
#[cfg(not(CONFIG_SUSPEND))]
pub fn swsusp_unmark() -> kernel::ffi::c_int { 0 }

extern "C" { pub fn swsusp_show_speed(a: ktime_t, b: ktime_t, c: kernel::ffi::c_uint, d: *mut kernel::ffi::c_char); }

#[cfg(CONFIG_SUSPEND)]
extern "C" {
    pub static pm_labels: *const *const kernel::ffi::c_char;
    pub static pm_states: *const *const kernel::ffi::c_char;
    pub static mem_sleep_states: *const *const kernel::ffi::c_char;
    pub fn suspend_devices_and_enter(state: suspend_state_t) -> kernel::ffi::c_int;
}
#[cfg(not(CONFIG_SUSPEND))]
pub const mem_sleep_current: suspend_state_t = PM_SUSPEND_ON;
#[cfg(not(CONFIG_SUSPEND))]
pub fn suspend_devices_and_enter(_state: suspend_state_t) -> kernel::ffi::c_int { -ENOSYS }

#[cfg(CONFIG_PM_TEST_SUSPEND)]
extern "C" { pub fn suspend_test_start(); pub fn suspend_test_finish(label: *const kernel::ffi::c_char); }
#[cfg(not(CONFIG_PM_TEST_SUSPEND))]
pub fn suspend_test_start() {}
#[cfg(not(CONFIG_PM_TEST_SUSPEND))]
pub fn suspend_test_finish(_label: *const kernel::ffi::c_char) {}

#[cfg(CONFIG_PM_SLEEP)]
extern "C" {
    pub fn pm_notifier_call_chain_robust(val_up: kernel::ffi::c_ulong, val_down: kernel::ffi::c_ulong) -> kernel::ffi::c_int;
    pub fn pm_notifier_call_chain(val: kernel::ffi::c_ulong) -> kernel::ffi::c_int;
}

#[cfg(CONFIG_HIGHMEM)]
extern "C" { pub fn restore_highmem() -> kernel::ffi::c_int; }
#[cfg(not(CONFIG_HIGHMEM))]
pub fn count_highmem_pages() -> kernel::ffi::c_uint { 0 }
#[cfg(not(CONFIG_HIGHMEM))]
pub fn restore_highmem() -> kernel::ffi::c_int { 0 }

pub const TEST_NONE: kernel::ffi::c_uint = 0;
pub const TEST_CORE: kernel::ffi::c_uint = 1;
pub const TEST_CPUS: kernel::ffi::c_uint = 2;
pub const TEST_PLATFORM: kernel::ffi::c_uint = 3;
pub const TEST_DEVICES: kernel::ffi::c_uint = 4;
pub const TEST_FREEZER: kernel::ffi::c_uint = 5;
pub const __TEST_AFTER_LAST: kernel::ffi::c_uint = 6;
pub const TEST_FIRST: kernel::ffi::c_uint = TEST_NONE;
pub const TEST_MAX: kernel::ffi::c_uint = __TEST_AFTER_LAST - 1;

#[cfg(CONFIG_PM_SLEEP_DEBUG)]
extern "C" { pub static mut pm_test_level: kernel::ffi::c_int; }
#[cfg(not(CONFIG_PM_SLEEP_DEBUG))]
pub const pm_test_level: kernel::ffi::c_uint = TEST_NONE;

#[cfg(CONFIG_SUSPEND_FREEZER)]
pub unsafe fn suspend_freeze_processes() -> kernel::ffi::c_int {
    let mut error = freeze_processes();
    if error != 0 { return error; }
    error = freeze_kernel_threads();
    if error != 0 { thaw_processes(); }
    error
}
#[cfg(CONFIG_SUSPEND_FREEZER)]
pub unsafe fn suspend_thaw_processes() { thaw_processes(); }
#[cfg(not(CONFIG_SUSPEND_FREEZER))]
pub fn suspend_freeze_processes() -> kernel::ffi::c_int { 0 }
#[cfg(not(CONFIG_SUSPEND_FREEZER))]
pub fn suspend_thaw_processes() {}

#[cfg(CONFIG_PM_AUTOSLEEP)]
extern "C" {
    pub fn pm_autosleep_init() -> kernel::ffi::c_int;
    pub fn pm_autosleep_lock() -> kernel::ffi::c_int;
    pub fn pm_autosleep_unlock();
    pub fn pm_autosleep_state() -> suspend_state_t;
    pub fn pm_autosleep_set_state(state: suspend_state_t) -> kernel::ffi::c_int;
}
#[cfg(not(CONFIG_PM_AUTOSLEEP))]
pub fn pm_autosleep_init() -> kernel::ffi::c_int { 0 }
#[cfg(not(CONFIG_PM_AUTOSLEEP))]
pub fn pm_autosleep_lock() -> kernel::ffi::c_int { 0 }
#[cfg(not(CONFIG_PM_AUTOSLEEP))]
pub fn pm_autosleep_unlock() {}
#[cfg(not(CONFIG_PM_AUTOSLEEP))]
pub fn pm_autosleep_state() -> suspend_state_t { PM_SUSPEND_ON }

#[cfg(CONFIG_PM_WAKELOCKS)]
extern "C" {
    pub fn pm_show_wakelocks(buf: *mut kernel::ffi::c_char, show_active: bool) -> ssize_t;
    pub fn pm_wake_lock(buf: *const kernel::ffi::c_char) -> kernel::ffi::c_int;
    pub fn pm_wake_unlock(buf: *const kernel::ffi::c_char) -> kernel::ffi::c_int;
}

pub unsafe fn pm_sleep_disable_secondary_cpus() -> kernel::ffi::c_int {
    cpuidle_pause();
    suspend_disable_secondary_cpus()
}
pub unsafe fn pm_sleep_enable_secondary_cpus() {
    suspend_enable_secondary_cpus();
    cpuidle_resume();
}

extern "C" { pub fn dpm_save_errno(err: kernel::ffi::c_int); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
