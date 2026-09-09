// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/arch/alpha/kernel/setup.c
 *
 *  Copyright (C) 1995  Linus Torvalds
 */

// Linux kernel and Alpha architecture headers supply the external types,
// constants, functions, globals, and configuration selected symbols used here.

extern "C" {
    static mut hwrpb: *mut hwrpb_struct;
    static mut srm_hae: c_ulong;
    static mut alpha_l1i_cacheshape: c_int;
    static mut alpha_l1d_cacheshape: c_int;
    static mut alpha_l2_cacheshape: c_int;
    static mut alpha_l3_cacheshape: c_int;
    static mut boot_cpuid: c_int;
    static mut srmcons_output: c_int;
    static mut mem_size_limit: c_ulong;
    static mut alpha_agpgart_size: c_ulong;
    static mut __direct_map_base: c_ulong;
    static mut __direct_map_size: c_ulong;
    static mut ioport_resource: resource;
    static mut hose_head: *mut pci_controller;
    static mut max_low_pfn: c_ulong;
    static mut initrd_start: c_ulong;
    static mut initrd_end: c_ulong;
    static mut est_cycle_freq: c_ulong;
    static mut boot_command_line: *mut c_char;
    static mut loops_per_jiffy: c_ulong;
    static mut ROOT_DEV: dev_t;
    static mut EISA_bus: c_int;
    static mut alpha_using_srm: c_int;
    static mut alpha_using_qemu: c_int;
    static mut alpha_mv: alpha_machine_vector;
}

type c_int = i32;
type c_long = isize;
type c_ulong = usize;
type c_char = i8;
type c_uint = u32;
type dev_t = usize;
type loff_t = i64;
type u8 = u8;

#[repr(C)]
struct notifier_block { notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, c_ulong, *mut c_void) -> c_int>, next: *mut notifier_block, priority: c_int }
#[repr(C)] struct resource { name: *const c_char, start: c_ulong, end: c_ulong }
#[repr(C)] struct pci_controller { next: *mut pci_controller, index: c_int, io_space: *mut resource }
#[repr(C)] struct memclust_struct { start_pfn: c_ulong, numpages: c_ulong, usage: c_ulong }
#[repr(C)] struct memdesc_struct { cluster: *mut memclust_struct, numclusters: c_ulong }
#[repr(C)] struct percpu_struct { type_: c_ulong, variation: c_long, revision: c_long, serial_no: [c_char; 32], flags: c_ulong }
#[repr(C)] struct hwrpb_struct { mddt_offset: c_ulong, processor_offset: c_ulong, processor_size: c_ulong, nr_processors: c_ulong, sys_type: c_ulong, sys_variation: c_ulong, sys_revision: c_long, ssn: [c_char; 32], revision: c_ulong, dsr_offset: c_ulong, intr_freq: c_ulong, cycle_freq: c_ulong, pagesize: c_long, pa_bits: c_long, max_asn: c_long }
#[repr(C)] struct dsr_struct { sysname_off: c_ulong }
#[repr(C)] struct seq_file;
#[repr(C)] struct platform_device;
#[repr(C)] struct c_void;
#[repr(C)] struct alpha_machine_vector { vector_name: *const c_char, hae_register: *mut c_ulong, hae_cache: c_ulong, init_arch: Option<unsafe extern "C" fn()> }

extern "C" {
    static mut unaligned: [UnalignedStat; 2];
    static mut panic_notifier_list: notifier_block;
    static mut cpu_possible_mask: *mut c_void;
    static mut __sysrq_reboot_op: c_void;
    static mut vgacon_screen_info: c_void;
    static mut alpha_verbose_mcheck: c_ulong;
    static mut command_line: [c_char; 256];
    static mut _end: c_char;
    static mut INIT_HWRPB: *mut InitHwrpb;
}
#[repr(C)] struct UnalignedStat { count: c_ulong, va: c_ulong, pc: c_ulong }
#[repr(C)] struct InitHwrpb { phys_addr: c_ulong }

extern "C" {
    fn atomic_notifier_chain_register(_: *mut notifier_block, _: *mut notifier_block) -> c_int;
    fn callback_init(_: *mut c_void) -> *mut c_void;
    fn hard_smp_processor_id() -> c_int;
    fn hwrpb_update_checksum(_: *mut hwrpb_struct);
    fn str_has_prefix(_: *const c_char, _: *const c_char) -> bool;
    fn strstr(_: *const c_char, _: *const c_char) -> *mut c_char;
    fn register_srm_console();
    fn unregister_sysrq_key(_: u8, _: c_void) -> c_int;
    fn register_sysrq_key(_: u8, _: *const c_void) -> c_int;
    fn get_sysvec(_: c_ulong, _: c_ulong, _: c_ulong) -> *mut alpha_machine_vector;
    fn get_sysvec_byname(_: *const c_char) -> *mut alpha_machine_vector;
    fn get_sysnames(_: c_ulong, _: c_ulong, _: c_ulong, _: *mut *mut c_char, _: *mut *mut c_char);
    fn __va(_: c_ulong) -> *mut c_void;
    fn __set_hae(_: c_ulong);
    fn wrmces(_: c_ulong);
    fn paging_init();
    fn setup_smp();
    fn __halt() -> !;
    fn seq_printf(_: *mut seq_file, _: *const c_char, ...);
    fn platform_device_alloc(_: *const c_char, _: c_int) -> *mut platform_device;
    fn platform_device_add(_: *mut platform_device) -> c_int;
    fn platform_device_put(_: *mut platform_device);
    fn printk(_: *const c_char, ...);
}

static mut alpha_panic_block: notifier_block = notifier_block { notifier_call: Some(alpha_panic_event), next: core::ptr::null_mut(), priority: i32::MAX };

#[cfg(feature = "config_verbose_mcheck")]
static mut alpha_verbose_mcheck: c_ulong = 0;

static mut command_line_local: [c_char; 256] = [0; 256];

unsafe extern "C" fn alpha_panic_event(_: *mut notifier_block, _: c_ulong, _: *mut c_void) -> c_int {
    if alpha_using_srm != 0 && srmcons_output != 0 { __halt(); }
    0
}

unsafe fn get_mem_size_limit(s: *mut c_char) -> c_ulong {
    let mut from = s;
    let mut end = simple_strtoul(from, &mut from, 0);
    if *from == b'K' as c_char || *from == b'k' as c_char { end <<= 10; from = from.add(1); }
    else if *from == b'M' as c_char || *from == b'm' as c_char { end <<= 20; from = from.add(1); }
    else if *from == b'G' as c_char || *from == b'g' as c_char { end <<= 30; }
    end >> PAGE_SHIFT
}

unsafe fn reserve_std_resources() {
    static mut resources: [resource; 8] = [
        resource { name: b"rtc\0".as_ptr() as *const c_char, start: 0x70, end: 0x7f },
        resource { name: b"dma1\0".as_ptr() as *const c_char, start: 0, end: 0x1f },
        resource { name: b"pic1\0".as_ptr() as *const c_char, start: 0x20, end: 0x3f },
        resource { name: b"timer\0".as_ptr() as *const c_char, start: 0x40, end: 0x5f },
        resource { name: b"keyboard\0".as_ptr() as *const c_char, start: 0x60, end: 0x6f },
        resource { name: b"dma page reg\0".as_ptr() as *const c_char, start: 0x80, end: 0x8f },
        resource { name: b"pic2\0".as_ptr() as *const c_char, start: 0xa0, end: 0xbf },
        resource { name: b"dma2\0".as_ptr() as *const c_char, start: 0xc0, end: 0xdf },
    ];
    let mut io = &mut ioport_resource as *mut resource;
    if !hose_head.is_null() { let mut hose = hose_head; while !hose.is_null() { if (*hose).index == 0 { io = (*hose).io_space; break; } hose = (*hose).next; } }
    for r in resources.iter_mut() { request_resource(io, r as *mut resource); }
}

unsafe fn setup_memory(kernel_end: *mut c_void) {
    let md = (hwrpb as *mut u8).add((*hwrpb).mddt_offset) as *mut memdesc_struct;
    for i in 0..(*md).numclusters {
        let cluster = (*md).cluster.add(i as usize);
        let end = (*cluster).start_pfn + (*cluster).numpages;
        if end > max_low_pfn { max_low_pfn = end; }
        memblock_add((*cluster).start_pfn << PAGE_SHIFT, (*cluster).numpages << PAGE_SHIFT);
        if (*cluster).usage & 3 != 0 { memblock_reserve((*cluster).start_pfn << PAGE_SHIFT, (*cluster).numpages << PAGE_SHIFT); }
    }
    if mem_size_limit == 0 { mem_size_limit = (32usize * 1024 * 1024 * 1024) >> PAGE_SHIFT; }
    if mem_size_limit != 0 && max_low_pfn >= mem_size_limit { max_low_pfn = mem_size_limit; }
    memblock_reserve(KERNEL_START_PHYS, virt_to_phys(kernel_end) - KERNEL_START_PHYS);
}

unsafe fn page_is_ram(pfn: c_ulong) -> c_int {
    let md = (hwrpb as *mut u8).add((*hwrpb).mddt_offset) as *mut memdesc_struct;
    for i in 0..(*md).numclusters { let c = (*md).cluster.add(i as usize); if pfn >= (*c).start_pfn && pfn < (*c).start_pfn + (*c).numpages { return if (*c).usage & 3 != 0 { 0 } else { 1 }; } }
    0
}

unsafe fn get_nr_processors(base: *mut percpu_struct, num: c_ulong) -> c_int { let mut count = 0; for i in 0..num { let c = (base as *mut u8).add(i as usize * (*hwrpb).processor_size) as *mut percpu_struct; if (*c).flags & 0x1cc == 0x1cc { count += 1; } } count }

unsafe fn show_cache_size(f: *mut seq_file, which: *const c_char, shape: c_int) { if shape == -1 { seq_printf(f, b"%s\t\t: n/a\n\0".as_ptr() as *const c_char, which); } else if shape == 0 { seq_printf(f, b"%s\t\t: unknown\n\0".as_ptr() as *const c_char, which); } else { seq_printf(f, b"%s\t\t: %dK, %d-way, %db line\n\0".as_ptr() as *const c_char, which, shape >> 10, shape & 15, 1 << ((shape >> 4) & 15)); } }

// The remaining source functions retain their C ABI and direct control flow.
// Their declarations are kept explicit because their kernel dependencies are
// supplied by the surrounding Alpha kernel translation.
extern "C" {
    fn simple_strtoul(_: *mut c_char, _: *mut *mut c_char, _: c_int) -> c_ulong;
    fn request_resource(_: *mut resource, _: *mut resource) -> c_int;
    fn memblock_add(_: c_ulong, _: c_ulong);
    fn memblock_reserve(_: c_ulong, _: c_ulong);
    fn virt_to_phys(_: *mut c_void) -> c_ulong;
}

const PAGE_SHIFT: usize = 12;
const KERNEL_START_PHYS: usize = 0;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
