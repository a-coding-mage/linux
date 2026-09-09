/* SPDX-License-Identifier: GPL-2.0 */

/* Linux header dependencies are supplied by other translated files. */

#[repr(C)]
pub struct swsusp_info {
    pub uts: new_utsname,
    pub version_code: u32,
    pub num_physpages: libc::c_ulong,
    pub cpus: libc::c_int,
    pub image_pages: libc::c_ulong,
    pub pages: libc::c_ulong,
    pub size: libc::c_ulong,
}
/* C declaration uses __aligned(PAGE_SIZE). */

#[cfg(any(CONFIG_SUSPEND, CONFIG_HIBERNATION))]
extern "C" {
    pub fn pm_sleep_fs_sync() -> libc::c_int;
    pub static mut filesystem_freeze_enabled: bool;
}

#[cfg(CONFIG_HIBERNATION)]
extern "C" {
    pub fn hibernate_reserved_size_init();
    pub fn hibernate_image_size_init();
    pub fn swsusp_save() -> libc::c_int;
    pub static mut freezer_test_done: bool;
    pub static mut hib_comp_algo: [libc::c_char; CRYPTO_MAX_ALG_NAME];
    pub static mut swsusp_header_flags: libc::c_uint;
    pub fn hibernation_snapshot(platform_mode: libc::c_int) -> libc::c_int;
    pub fn hibernation_restore(platform_mode: libc::c_int) -> libc::c_int;
    pub fn hibernation_platform_enter() -> libc::c_int;
    pub fn hibernation_in_progress() -> bool;
}

#[cfg(CONFIG_ARCH_HIBERNATION_HEADER)]
pub const MAX_ARCH_HEADER_SIZE: usize = core::mem::size_of::<new_utsname>() + 4;

#[cfg(CONFIG_ARCH_HIBERNATION_HEADER)]
pub unsafe fn init_header_complete(info: *mut swsusp_info) -> libc::c_int {
    arch_hibernation_header_save(info, MAX_ARCH_HEADER_SIZE)
}

#[cfg(CONFIG_ARCH_HIBERNATION_HEADER)]
pub unsafe fn check_image_kernel(info: *mut swsusp_info) -> *const libc::c_char {
    if arch_hibernation_header_restore(info) {
        b"architecture specific data\0".as_ptr() as *const libc::c_char
    } else {
        core::ptr::null()
    }
}

pub const PAGES_FOR_IO: libc::c_ulong = ((4096 * 1024) >> PAGE_SHIFT) as libc::c_ulong;
pub const SPARE_PAGES: libc::c_ulong = ((1024 * 1024) >> PAGE_SHIFT) as libc::c_ulong;

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

pub static mut image_size: libc::c_ulong;
pub static mut reserved_size: libc::c_ulong;
pub static mut in_suspend: libc::c_int;
pub static mut swsusp_resume_device: dev_t;
pub static mut swsusp_resume_block: sector_t;

extern "C" {
    pub fn create_basic_memory_bitmaps() -> libc::c_int;
    pub fn free_basic_memory_bitmaps();
    pub fn hibernate_preallocate_memory() -> libc::c_int;
    pub fn clear_or_poison_free_pages();
}

#[repr(C)]
pub struct snapshot_handle {
    pub cur: libc::c_uint,
    pub buffer: *mut libc::c_void,
    pub sync_read: libc::c_int,
}

pub unsafe fn data_of(handle: *mut snapshot_handle) -> *mut libc::c_void {
    (*handle).buffer
}

extern "C" {
    pub fn snapshot_additional_pages(zone: *mut zone) -> libc::c_uint;
    pub fn snapshot_get_image_size() -> libc::c_ulong;
    pub fn snapshot_read_next(handle: *mut snapshot_handle) -> libc::c_int;
    pub fn snapshot_write_next(handle: *mut snapshot_handle) -> libc::c_int;
    pub fn snapshot_write_finalize(handle: *mut snapshot_handle) -> libc::c_int;
    pub fn snapshot_image_loaded(handle: *mut snapshot_handle) -> bool;
    pub fn hibernate_acquire() -> bool;
    pub fn hibernate_release();
    pub fn alloc_swapdev_block(swap: libc::c_int) -> sector_t;
    pub fn free_all_swap_pages(swap: libc::c_int);
    pub fn swsusp_swap_in_use() -> libc::c_int;
}

pub const SF_COMPRESSION_ALG_LZO: libc::c_uint = 0;
pub const SF_PLATFORM_MODE: libc::c_uint = 1;
pub const SF_NOCOMPRESS_MODE: libc::c_uint = 2;
pub const SF_CRC32_MODE: libc::c_uint = 4;
pub const SF_HW_SIG: libc::c_uint = 8;
pub const SF_COMPRESSION_ALG_LZ4: libc::c_uint = 16;

extern "C" {
    pub fn swsusp_check(exclusive: bool) -> libc::c_int;
    pub fn swsusp_free();
    pub fn swsusp_read(flags_p: *mut libc::c_uint) -> libc::c_int;
    pub fn swsusp_write(flags: libc::c_uint) -> libc::c_int;
    pub fn swsusp_close();
}

#[cfg(CONFIG_SUSPEND)]
extern "C" { pub fn swsusp_unmark() -> libc::c_int; }
#[cfg(not(CONFIG_SUSPEND))]
pub fn swsusp_unmark() -> libc::c_int { 0 }

extern "C" { pub fn swsusp_show_speed(a: ktime_t, b: ktime_t, c: libc::c_uint, d: *mut libc::c_char); }

#[cfg(CONFIG_SUSPEND)]
extern "C" {
    pub static pm_labels: *const *const libc::c_char;
    pub static pm_states: *const *const libc::c_char;
    pub static mem_sleep_states: *const *const libc::c_char;
    pub fn suspend_devices_and_enter(state: suspend_state_t) -> libc::c_int;
}
#[cfg(not(CONFIG_SUSPEND))]
pub const mem_sleep_current: suspend_state_t = PM_SUSPEND_ON;
#[cfg(not(CONFIG_SUSPEND))]
pub fn suspend_devices_and_enter(_state: suspend_state_t) -> libc::c_int { -ENOSYS }

#[cfg(CONFIG_PM_TEST_SUSPEND)]
extern "C" { pub fn suspend_test_start(); pub fn suspend_test_finish(label: *const libc::c_char); }
#[cfg(not(CONFIG_PM_TEST_SUSPEND))]
pub fn suspend_test_start() {}
#[cfg(not(CONFIG_PM_TEST_SUSPEND))]
pub fn suspend_test_finish(_label: *const libc::c_char) {}

#[cfg(CONFIG_PM_SLEEP)]
extern "C" {
    pub fn pm_notifier_call_chain_robust(val_up: libc::c_ulong, val_down: libc::c_ulong) -> libc::c_int;
    pub fn pm_notifier_call_chain(val: libc::c_ulong) -> libc::c_int;
}

#[cfg(CONFIG_HIGHMEM)]
extern "C" { pub fn restore_highmem() -> libc::c_int; }
#[cfg(not(CONFIG_HIGHMEM))]
pub fn count_highmem_pages() -> libc::c_uint { 0 }
#[cfg(not(CONFIG_HIGHMEM))]
pub fn restore_highmem() -> libc::c_int { 0 }

pub const TEST_NONE: libc::c_uint = 0;
pub const TEST_CORE: libc::c_uint = 1;
pub const TEST_CPUS: libc::c_uint = 2;
pub const TEST_PLATFORM: libc::c_uint = 3;
pub const TEST_DEVICES: libc::c_uint = 4;
pub const TEST_FREEZER: libc::c_uint = 5;
pub const __TEST_AFTER_LAST: libc::c_uint = 6;
pub const TEST_FIRST: libc::c_uint = TEST_NONE;
pub const TEST_MAX: libc::c_uint = __TEST_AFTER_LAST - 1;

#[cfg(CONFIG_PM_SLEEP_DEBUG)]
extern "C" { pub static mut pm_test_level: libc::c_int; }
#[cfg(not(CONFIG_PM_SLEEP_DEBUG))]
pub const pm_test_level: libc::c_uint = TEST_NONE;

#[cfg(CONFIG_SUSPEND_FREEZER)]
pub unsafe fn suspend_freeze_processes() -> libc::c_int {
    let mut error = freeze_processes();
    if error != 0 { return error; }
    error = freeze_kernel_threads();
    if error != 0 { thaw_processes(); }
    error
}
#[cfg(CONFIG_SUSPEND_FREEZER)]
pub unsafe fn suspend_thaw_processes() { thaw_processes(); }
#[cfg(not(CONFIG_SUSPEND_FREEZER))]
pub fn suspend_freeze_processes() -> libc::c_int { 0 }
#[cfg(not(CONFIG_SUSPEND_FREEZER))]
pub fn suspend_thaw_processes() {}

#[cfg(CONFIG_PM_AUTOSLEEP)]
extern "C" {
    pub fn pm_autosleep_init() -> libc::c_int;
    pub fn pm_autosleep_lock() -> libc::c_int;
    pub fn pm_autosleep_unlock();
    pub fn pm_autosleep_state() -> suspend_state_t;
    pub fn pm_autosleep_set_state(state: suspend_state_t) -> libc::c_int;
}
#[cfg(not(CONFIG_PM_AUTOSLEEP))]
pub fn pm_autosleep_init() -> libc::c_int { 0 }
#[cfg(not(CONFIG_PM_AUTOSLEEP))]
pub fn pm_autosleep_lock() -> libc::c_int { 0 }
#[cfg(not(CONFIG_PM_AUTOSLEEP))]
pub fn pm_autosleep_unlock() {}
#[cfg(not(CONFIG_PM_AUTOSLEEP))]
pub fn pm_autosleep_state() -> suspend_state_t { PM_SUSPEND_ON }

#[cfg(CONFIG_PM_WAKELOCKS)]
extern "C" {
    pub fn pm_show_wakelocks(buf: *mut libc::c_char, show_active: bool) -> ssize_t;
    pub fn pm_wake_lock(buf: *const libc::c_char) -> libc::c_int;
    pub fn pm_wake_unlock(buf: *const libc::c_char) -> libc::c_int;
}

pub unsafe fn pm_sleep_disable_secondary_cpus() -> libc::c_int {
    cpuidle_pause();
    suspend_disable_secondary_cpus()
}
pub unsafe fn pm_sleep_enable_secondary_cpus() {
    suspend_enable_secondary_cpus();
    cpuidle_resume();
}

extern "C" { pub fn dpm_save_errno(err: libc::c_int); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
