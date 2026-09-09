// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Initial setup-routines for HP 9000 based hardware.
 *
 * This is a source-level Rust translation of setup.c. Symbols supplied by
 * the Linux kernel and PA-RISC headers remain external dependencies.
 */

static mut COMMAND_LINE: [u8; COMMAND_LINE_SIZE] = [0; COMMAND_LINE_SIZE];

unsafe fn setup_cmdline(cmdline_p: *mut *mut u8) {
    unsafe extern "C" {
        static mut boot_args: [u32; 0];
    }
    let mut p: *mut u8;

    *cmdline_p = COMMAND_LINE.as_mut_ptr();

    // boot_args[0] is free-mem start, boot_args[1] is ptr to command line
    if boot_args[0] < 64 {
        return; // return if called from hpux boot loader
    }

    // Collect stuff passed in from the boot loader
    strscpy(boot_command_line, __va(boot_args[1] as usize) as *const u8, COMMAND_LINE_SIZE);

    // autodetect console type (if not done by palo yet)
    p = boot_command_line;
    if !str_has_prefix(p, b"console=\0".as_ptr()) && strstr(p, b" console=\0".as_ptr()).is_null() {
        strlcat(p, b" console=\0".as_ptr(), COMMAND_LINE_SIZE);
        if (*PAGE0).mem_cons.cl_class == CL_DUPLEX {
            strlcat(p, b"ttyS0\0".as_ptr(), COMMAND_LINE_SIZE);
        } else {
            strlcat(p, b"tty0\0".as_ptr(), COMMAND_LINE_SIZE);
        }
    }

    // default to use early console
    if strstr(p, b"earlycon\0".as_ptr()).is_null() {
        strlcat(p, b" earlycon=pdc\0".as_ptr(), COMMAND_LINE_SIZE);
    }

    // CONFIG_BLK_DEV_INITRD: did palo pass us a ramdisk?
    if boot_args[2] != 0 {
        initrd_start = __va(boot_args[2] as usize) as usize;
        initrd_end = __va(boot_args[3] as usize) as usize;
    }

    strscpy(COMMAND_LINE.as_mut_ptr(), boot_command_line, COMMAND_LINE_SIZE);
}

// CONFIG_PA11
unsafe fn dma_ops_init() {
    match boot_cpu_data.cpu_type {
        pcx => panic!("PA-RISC Linux currently only supports machines that conform to\nPA-RISC 1.1 or 2.0 architecture specification.\n"),
        pcxl2 => {},
        _ => {},
    }
}

pub unsafe fn setup_arch(cmdline_p: *mut *mut u8) {
    unwind_init();
    init_per_cpu(smp_processor_id()); // Set Modes & Enable FP

    printk(KERN_INFO, b"The kernel has started...\0".as_ptr());
    printk(KERN_INFO, b"Kernel default page size is %d KB. Huge pages \0".as_ptr(), (PAGE_SIZE / 1024) as i32);
    printk(KERN_CONT, b".\n\0".as_ptr());

    setup_pdc();
    setup_cmdline(cmdline_p);
    collect_boot_cpu_data();
    do_memory_inventory(); // probe for physical memory
    parisc_cache_init();
    paging_init();
    dma_ops_init();
    clear_sched_clock_stable();
}

/* Display CPU info for all CPUs. */
unsafe extern "C" fn c_start(_m: *mut seq_file, pos: *mut loff_t) -> *mut core::ffi::c_void {
    // The caller repeatedly invokes this until zero signals EOF. We print all
    // CPU info in show_cpuinfo, so only one position is permitted.
    if (*pos as i64) < 1 { 1 as *mut _ } else { core::ptr::null_mut() }
}

unsafe extern "C" fn c_next(m: *mut seq_file, _v: *mut core::ffi::c_void, pos: *mut loff_t) -> *mut core::ffi::c_void {
    *pos += 1;
    c_start(m, pos)
}

unsafe extern "C" fn c_stop(_m: *mut seq_file, _v: *mut core::ffi::c_void) {}

#[repr(C)]
pub struct seq_operations {
    pub start: unsafe extern "C" fn(*mut seq_file, *mut loff_t) -> *mut core::ffi::c_void,
    pub next: unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void, *mut loff_t) -> *mut core::ffi::c_void,
    pub stop: unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void),
    pub show: unsafe extern "C" fn(*mut seq_file, *mut core::ffi::c_void) -> i32,
}

pub static cpuinfo_op: seq_operations = seq_operations { start: c_start, next: c_next, stop: c_stop, show: show_cpuinfo };

static mut central_bus: resource = resource { name: b"Central Bus\0".as_ptr(), start: F_EXTEND(0xfff80000), end: F_EXTEND(0xfffaffff), flags: IORESOURCE_MEM };
static mut local_broadcast: resource = resource { name: b"Local Broadcast\0".as_ptr(), start: F_EXTEND(0xfffb0000), end: F_EXTEND(0xfffdffff), flags: IORESOURCE_MEM };
static mut global_broadcast: resource = resource { name: b"Global Broadcast\0".as_ptr(), start: F_EXTEND(0xfffe0000), end: F_EXTEND(0xffffffff), flags: IORESOURCE_MEM };

unsafe fn parisc_init_resources() -> i32 {
    let mut result = request_resource(&mut iomem_resource, &mut central_bus);
    if result < 0 { printk(KERN_ERR, b"%s: failed to claim %s address space!\n\0".as_ptr(), __FILE__, central_bus.name); return result; }
    result = request_resource(&mut iomem_resource, &mut local_broadcast);
    if result < 0 { printk(KERN_ERR, b"%s: failed to claim %s address space!\n\0".as_ptr(), __FILE__, local_broadcast.name); return result; }
    result = request_resource(&mut iomem_resource, &mut global_broadcast);
    if result < 0 { printk(KERN_ERR, b"%s: failed to claim %s address space!\n\0".as_ptr(), __FILE__, global_broadcast.name); return result; }
    0
}

unsafe fn parisc_init() -> i32 {
    let mut osid: u32 = (OS_ID_LINUX << 16) as u32;
    parisc_init_resources();
    do_device_inventory(); // probe for hardware
    parisc_pdc_chassis_init();
    pdc_chassis_send_status(PDC_CHASSIS_DIRECT_BSTART);
    pdc_stable_write(0x40, &mut osid, core::mem::size_of::<u32>());
    flush_cache_all_local();
    flush_tlb_all_local(core::ptr::null_mut());
    processor_init();
    pr_info!("CPU(s): %s at %d.%06d MHz\n", boot_cpu_data.cpu_name, boot_cpu_data.cpu_hz / 1000000, boot_cpu_data.cpu_hz % 1000000);
    apply_alternatives_all();
    parisc_setup_cache_timing();
    0
}

unsafe fn start_parisc() {
    let mut coproc_cfg: pdc_coproc_cfg = core::mem::zeroed();
    let warn1 = b"CRITICAL: Kernel may crash because KERNEL_INITIAL_ORDER is too small.\n";
    if __pa((&_end as *const _ as usize)) >= KERNEL_INITIAL_SIZE { pdc_iodc_print(warn1.as_ptr(), warn1.len() - 1); }
    running_on_qemu = memcmp(&(*PAGE0).pad0 as *const _ as *const _, b"SeaBIOS\0".as_ptr() as *const _, 8) == 0;
    let cpunum = smp_processor_id();
    init_cpu_topology();
    set_firmware_width_unlocked();
    let ret = pdc_coproc_cfg_unlocked(&mut coproc_cfg);
    if ret >= 0 && coproc_cfg.ccr_functional {
        mtctl(coproc_cfg.ccr_functional, 10);
        per_cpu(cpu_data, cpunum).fp_rev = coproc_cfg.revision;
        per_cpu(cpu_data, cpunum).fp_model = coproc_cfg.model;
        // asm volatile ("fstd %fr0,8(%sp)");
    } else { panic!("must have an fpu to boot linux"); }
    early_trap_init(); // initialize checksum of fault_vector
    start_kernel();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
