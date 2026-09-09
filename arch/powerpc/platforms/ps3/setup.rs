// SPDX-License-Identifier: GPL-2.0-only
/*
 *  PS3 platform setup routines.
 *
 *  Copyright (C) 2006 Sony Computer Entertainment Inc.
 *  Copyright 2006 Sony Corp.
 */

// C includes are supplied by the surrounding kernel translation unit.

#[cfg(debug_assertions)]
macro_rules! DBG { ($($arg:tt)*) => { unsafe { udbg_printf($($arg)*) } } }
#[cfg(not(debug_assertions))]
macro_rules! DBG { ($($arg:tt)*) => { unsafe { pr_debug($($arg)*) } } }

/* mutex synchronizing GPU accesses and video mode changes */
#[no_mangle]
pub static mut ps3_gpu_mutex: mutex = unsafe { core::mem::zeroed() };

static mut ps3_firmware_version: ps3_firmware_version_union = unsafe { core::mem::zeroed() };
static mut ps3_firmware_version_str: [core::ffi::c_char; 16] = [0; 16];

#[no_mangle]
pub unsafe extern "C" fn ps3_get_firmware_version(v: *mut ps3_firmware_version_union) {
    *v = ps3_firmware_version;
}

#[no_mangle]
pub unsafe extern "C" fn ps3_compare_firmware_version(major: u16, minor: u16, rev: u16) -> i32 {
    let mut x: ps3_firmware_version_union = core::mem::zeroed();
    (*(&mut x as *mut _)).pad = 0;
    (*(&mut x as *mut _)).major = major;
    (*(&mut x as *mut _)).minor = minor;
    (*(&mut x as *mut _)).rev = rev;

    (ps3_firmware_version.raw > x.raw) as i32
        - (ps3_firmware_version.raw < x.raw) as i32
}

unsafe fn ps3_power_save() {
    /*
     * lv1_pause() puts the PPE thread into inactive state until an
     * irq on an unmasked plug exists. MSR[EE] has no effect.
     * flags: 0 = wake on DEC interrupt, 1 = ignore DEC interrupt.
     */
    lv1_pause(0);
}

unsafe extern "C" fn ps3_restart(cmd: *mut core::ffi::c_char) -> ! {
    DBG!("%s:%d cmd '%s'\n", "ps3_restart", line!(), cmd);
    smp_send_stop();
    ps3_sys_manager_restart();
    core::hint::unreachable_unchecked()
}

unsafe extern "C" fn ps3_power_off() -> ! {
    DBG!("%s:%d\n", "ps3_power_off", line!());
    smp_send_stop();
    ps3_sys_manager_power_off();
    core::hint::unreachable_unchecked()
}

unsafe extern "C" fn ps3_halt() -> ! {
    DBG!("%s:%d\n", "ps3_halt", line!());
    smp_send_stop();
    ps3_sys_manager_halt();
    core::hint::unreachable_unchecked()
}

unsafe extern "C" fn ps3_panic(str_: *mut core::ffi::c_char) {
    DBG!("%s:%d %s\n", "ps3_panic", line!(), str_);
    smp_send_stop();
    printk("\n");
    printk("   System does not reboot automatically.\n");
    printk("   Please press POWER button.\n");
    printk("\n");
    panic_flush_kmsg_end();
    loop { lv1_pause(1); }
}

#[cfg(any(feature = "CONFIG_FB_PS3", feature = "CONFIG_FB_PS3_MODULE", feature = "CONFIG_PS3_FLASH", feature = "CONFIG_PS3_FLASH_MODULE"))]
unsafe fn prealloc(p: *mut ps3_prealloc) {
    if (*p).size == 0 { return; }
    (*p).address = memblock_alloc_or_panic((*p).size, (*p).align);
    printk(KERN_INFO "%s: %lu bytes at %p\n", (*p).name, (*p).size, (*p).address);
}

#[cfg(any(feature = "CONFIG_FB_PS3", feature = "CONFIG_FB_PS3_MODULE"))]
#[no_mangle]
pub static mut ps3fb_videomemory: ps3_prealloc = ps3_prealloc {
    name: "ps3fb videomemory\0".as_ptr() as *const core::ffi::c_char,
    size: CONFIG_FB_PS3_DEFAULT_SIZE_M * 1024 * 1024,
    align: 1024 * 1024,
    address: core::ptr::null_mut(),
};

#[cfg(any(feature = "CONFIG_FB_PS3", feature = "CONFIG_FB_PS3_MODULE"))]
unsafe fn prealloc_ps3fb_videomemory() { prealloc(&mut ps3fb_videomemory); }
#[cfg(not(any(feature = "CONFIG_FB_PS3", feature = "CONFIG_FB_PS3_MODULE")))]
unsafe fn prealloc_ps3fb_videomemory() {}

#[cfg(any(feature = "CONFIG_PS3_FLASH", feature = "CONFIG_PS3_FLASH_MODULE"))]
#[no_mangle]
pub static mut ps3flash_bounce_buffer: ps3_prealloc = ps3_prealloc {
    name: "ps3flash bounce buffer\0".as_ptr() as *const core::ffi::c_char,
    size: 256 * 1024,
    align: 256 * 1024,
    address: core::ptr::null_mut(),
};

#[cfg(any(feature = "CONFIG_PS3_FLASH", feature = "CONFIG_PS3_FLASH_MODULE"))]
unsafe fn prealloc_ps3flash_bounce_buffer() { prealloc(&mut ps3flash_bounce_buffer); }
#[cfg(not(any(feature = "CONFIG_PS3_FLASH", feature = "CONFIG_PS3_FLASH_MODULE")))]
unsafe fn prealloc_ps3flash_bounce_buffer() {}

unsafe fn ps3_set_dabr(mut dabr: c_ulong, mut dabrx: c_ulong) -> i32 {
    if dabrx == 0 && dabr == 0 { dabrx = DABRX_USER; }
    dabrx &= DABRX_BTI | DABRX_KERNEL | DABRX_USER;
    if lv1_set_dabr(dabr, dabrx) != 0 { -1 } else { 0 }
}

unsafe extern "C" fn ps3_fw_version_show(_kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    sysfs_emit(buf, "%s\n", ps3_firmware_version_str.as_ptr())
}

unsafe fn ps3_setup_sysfs() -> i32 {
    static mut attr: kobj_attribute = __ATTR(fw-version, S_IRUGO, ps3_fw_version_show, None);
    static mut kobj: *mut kobject = core::ptr::null_mut();
    kobj = kobject_create_and_add("ps3\0".as_ptr() as *const c_char, firmware_kobj);
    if kobj.is_null() { pr_warn("%s:%d: kobject_create_and_add failed.\n", "ps3_setup_sysfs", line!()); return -ENOMEM; }
    let result = sysfs_create_file(kobj, &mut attr.attr);
    if result != 0 { pr_warn("%s:%d: sysfs_create_file failed.\n", "ps3_setup_sysfs", line!()); kobject_put(kobj); return -ENOMEM; }
    0
}

unsafe fn ps3_setup_arch() {
    let mut tmp: u64 = 0;
    DBG!(" -> %s:%d\n", "ps3_setup_arch", line!());
    lv1_get_version_info(&mut ps3_firmware_version.raw, &mut tmp);
    snprintf(ps3_firmware_version_str.as_mut_ptr(), ps3_firmware_version_str.len(), "%u.%u.%u\0".as_ptr() as *const c_char, ps3_firmware_version.major, ps3_firmware_version.minor, ps3_firmware_version.rev);
    printk(KERN_INFO "PS3 firmware version %s\n", ps3_firmware_version_str.as_ptr());
    ps3_spu_set_platform();
    #[cfg(feature = "CONFIG_SMP")]
    smp_init_ps3();
    prealloc_ps3fb_videomemory();
    prealloc_ps3flash_bounce_buffer();
    ppc_md.power_save = Some(ps3_power_save);
    ps3_os_area_init();
    DBG!(" <- %s:%d\n", "ps3_setup_arch", line!());
}

unsafe fn ps3_progress(s: *mut c_char, hex: c_ushort) { printk("*** %04x : %s\n", hex, if s.is_null() { "".as_ptr() as *const c_char } else { s }); }

#[no_mangle]
pub unsafe extern "C" fn ps3_early_mm_init() { let mut htab_size = 0; ps3_mm_init(); ps3_mm_vas_create(&mut htab_size); ps3_hpte_init(htab_size); }

unsafe fn ps3_probe() -> i32 { DBG!(" -> %s:%d\n", "ps3_probe", line!()); ps3_os_area_save_params(); pm_power_off = Some(ps3_power_off); DBG!(" <- %s:%d\n", "ps3_probe", line!()); 1 }

#[cfg(feature = "CONFIG_KEXEC_CORE")]
unsafe fn ps3_kexec_cpu_down(_crash_shutdown: i32, _secondary: i32) {
    let cpu = smp_processor_id();
    DBG!(" -> %s:%d: (%d)\n", "ps3_kexec_cpu_down", line!(), cpu);
    ps3_smp_cleanup_cpu(cpu);
    ps3_shutdown_IRQ(cpu);
    DBG!(" <- %s:%d\n", "ps3_kexec_cpu_down", line!());
}

// define_machine(ps3) {
//     .name = "PS3",
//     .compatible = "sony,ps3",
//     .probe = ps3_probe,
//     .setup_arch = ps3_setup_arch,
//     .init_IRQ = ps3_init_IRQ,
//     .panic = ps3_panic,
//     .get_boot_time = ps3_get_boot_time,
//     .set_dabr = ps3_set_dabr,
//     .calibrate_decr = ps3_calibrate_decr,
//     .progress = ps3_progress,
//     .restart = ps3_restart,
//     .halt = ps3_halt,
//     #[cfg(feature = "CONFIG_KEXEC_CORE")]
//     .kexec_cpu_down = ps3_kexec_cpu_down,
// }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
