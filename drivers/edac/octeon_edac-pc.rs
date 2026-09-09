/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2012 Cavium, Inc.
 *
 * Copyright (C) 2009 Wind River Systems,
 *   written by Ralf Baechle <ralf@linux-mips.org>
 */

// Linux kernel headers and architecture headers supply the following types,
// constants, functions, and macros in the containing translation unit.

extern "C" {
    fn register_co_cache_error_notifier(nb: *mut notifier_block) -> i32;
    fn unregister_co_cache_error_notifier(nb: *mut notifier_block) -> i32;

    static mut cache_err_dcache: [u64; NR_CPUS];

    fn cvmx_get_core_num() -> u32;
    fn smp_processor_id() -> u32;
    fn read_octeon_c0_icacheerr() -> u64;
    fn read_octeon_c0_dcacheerr() -> u64;
    fn read_c0_errorepc() -> u64;
    fn write_octeon_c0_icacheerr(value: u64);
    fn write_octeon_c0_dcacheerr(value: u64);
    fn octeon_is_octeon2() -> bool;

    fn edac_device_printk(
        ed: *mut edac_device_ctl_info,
        level: *const core::ffi::c_char,
        fmt: *const core::ffi::c_char,
        ...
    );
    fn edac_device_handle_ce(
        ed: *mut edac_device_ctl_info,
        cpu: u32,
        layer: u32,
        location: *const core::ffi::c_char,
    );
    fn edac_device_handle_ue(
        ed: *mut edac_device_ctl_info,
        cpu: u32,
        layer: u32,
        location: *const core::ffi::c_char,
    );
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut core::ffi::c_void);
    fn edac_device_alloc_ctl_info(a: u32, name: *const core::ffi::c_char, cpus: u32, cache: *const core::ffi::c_char, layers: u32, instances: u32, index: u32) -> *mut edac_device_ctl_info;
    fn edac_device_alloc_index() -> u32;
    fn edac_device_add_device(ed: *mut edac_device_ctl_info) -> i32;
    fn edac_device_free_ctl_info(ed: *mut edac_device_ctl_info);
    fn edac_device_del_device(dev: *mut device);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut core::ffi::c_void;
}

const NR_CPUS: usize = 1; // Supplied by the kernel build configuration.
const NOTIFY_STOP: i32 = 0x8000;
const ENOMEM: i32 = 12;
const ENXIO: i32 = 6;
const GFP_KERNEL: u32 = 0;

#[repr(C)]
pub struct notifier_block {
    pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, u64, *mut core::ffi::c_void) -> i32>,
}

#[repr(C)]
pub struct device;

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct edac_device_ctl_info {
    pub dev: *mut device,
    pub dev_name: *const core::ffi::c_char,
    pub mod_name: *const core::ffi::c_char,
    pub ctl_name: *const core::ffi::c_char,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub name: *const core::ffi::c_char,
}

#[repr(C)]
struct co_cache_error {
    notifier: notifier_block,
    ed: *mut edac_device_ctl_info,
}

unsafe extern "C" fn co_cache_error_event(
    this: *mut notifier_block,
    event: u64,
    _ptr: *mut core::ffi::c_void,
) -> i32 {
    let p = (this as *mut u8).sub(core::mem::offset_of!(co_cache_error, notifier))
        as *mut co_cache_error;
    let core = cvmx_get_core_num();
    let cpu = smp_processor_id();
    let icache_err = read_octeon_c0_icacheerr();
    let dcache_err: u64;

    if event != 0 {
        dcache_err = cache_err_dcache[core as usize];
        cache_err_dcache[core as usize] = 0;
    } else {
        dcache_err = read_octeon_c0_dcacheerr();
    }

    if icache_err & 1 != 0 {
        edac_device_printk((*p).ed, core::ptr::null(), b"CacheErr (Icache):%llx, core %d/cpu %d, cp0_errorepc == %lx\n\0".as_ptr() as _, icache_err, core, cpu, read_c0_errorepc());
        write_octeon_c0_icacheerr(0);
        edac_device_handle_ce((*p).ed, cpu, 1, b"icache\0".as_ptr() as _);
    }
    if dcache_err & 1 != 0 {
        edac_device_printk((*p).ed, core::ptr::null(), b"CacheErr (Dcache):%llx, core %d/cpu %d, cp0_errorepc == %lx\n\0".as_ptr() as _, dcache_err, core, cpu, read_c0_errorepc());
        if event != 0 { edac_device_handle_ue((*p).ed, cpu, 0, b"dcache\0".as_ptr() as _); }
        else { edac_device_handle_ce((*p).ed, cpu, 0, b"dcache\0".as_ptr() as _); }
        if octeon_is_octeon2() { write_octeon_c0_dcacheerr(1); }
        else { write_octeon_c0_dcacheerr(0); }
    }
    NOTIFY_STOP
}

unsafe extern "C" fn co_cache_error_probe(pdev: *mut platform_device) -> i32 {
    let p = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<co_cache_error>(), GFP_KERNEL)
        as *mut co_cache_error;
    if p.is_null() { return -ENOMEM; }
    (*p).notifier.notifier_call = Some(co_cache_error_event);
    platform_set_drvdata(pdev, p as *mut _);
    (*p).ed = edac_device_alloc_ctl_info(0, b"cpu\0".as_ptr() as _, num_possible_cpus(), b"cache\0".as_ptr() as _, 2, 0, edac_device_alloc_index());
    if (*p).ed.is_null() { return -ENXIO; }
    (*p).ed.as_mut().unwrap().dev = &mut (*pdev).dev;
    (*p).ed.as_mut().unwrap().dev_name = b"octeon\0".as_ptr() as _;
    (*p).ed.as_mut().unwrap().mod_name = b"octeon-cpu\0".as_ptr() as _;
    (*p).ed.as_mut().unwrap().ctl_name = b"cache\0".as_ptr() as _;
    if edac_device_add_device((*p).ed) != 0 {
        edac_device_free_ctl_info((*p).ed);
        return -ENXIO;
    }
    register_co_cache_error_notifier(&mut (*p).notifier);
    0
}
unsafe extern "C" fn co_cache_error_remove(pdev: *mut platform_device) {
    let p = platform_get_drvdata(pdev) as *mut co_cache_error;
    unregister_co_cache_error_notifier(&mut (*p).notifier);
    edac_device_del_device(&mut (*pdev).dev);
    edac_device_free_ctl_info((*p).ed);
}

extern "C" { fn num_possible_cpus() -> u32; }

#[no_mangle]
pub static mut co_cache_error_driver: platform_driver = platform_driver {
    probe: Some(co_cache_error_probe),
    remove: Some(co_cache_error_remove),
    name: b"octeon_pc_edac\0".as_ptr() as _,
};

// module_platform_driver(co_cache_error_driver)
// MODULE_DESCRIPTION("Cavium Octeon Primary Caches EDAC driver")
// MODULE_LICENSE("GPL")
// MODULE_AUTHOR("Ralf Baechle <ralf@linux-mips.org>")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
