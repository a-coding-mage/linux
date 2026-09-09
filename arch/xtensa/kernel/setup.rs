/* Translated from arch/xtensa/kernel/setup.c. */

use core::ffi::c_void;

/* C headers provide the types, constants, macros, and external symbols used below. */

#[repr(C)]
pub struct tagtable_t {
    pub tag: u32,
    pub parse: Option<unsafe extern "C" fn(*const bp_tag_t) -> i32>,
}

extern "C" {
    pub static mut command_line: [u8; COMMAND_LINE_SIZE];
    pub static mut loops_per_jiffy: usize;
    pub static mut boot_command_line: [u8; COMMAND_LINE_SIZE];
    pub static mut dtb_start: *mut c_void;
    pub static mut xtensa_kio_paddr: usize;
    pub static mut initrd_start: usize;
    pub static mut initrd_end: usize;
    pub static mut initrd_below_start_ok: i32;
    pub static __tagtable_begin: tagtable_t;
    pub static __tagtable_end: tagtable_t;
    pub static cpu_online_mask: *const c_void;
    pub static ccount_freq: usize;
    pub static conswitchp: *mut c_void;
}

extern "C" {
    fn memblock_add(start: usize, size: usize) -> i32;
    fn memblock_reserve(start: usize, size: usize) -> i32;
    fn strscpy(dst: *mut u8, src: *const u8, size: usize) -> isize;
    fn __va(addr: usize) -> *mut c_void;
    fn __pa(addr: usize) -> usize;
    fn pr_warn(fmt: *const u8, ...);
    fn pr_info(fmt: *const u8, ...);
    fn pr_err(fmt: *const u8, ...);
    fn early_trap_init();
    fn init_mmu();
    fn kasan_early_init();
    fn platform_init(bp_start: *mut bp_tag_t);
    fn platform_setup(cmdline_p: *mut *mut u8);
    fn parse_early_param();
    fn bootmem_init();
    fn kasan_init();
    fn unflatten_and_copy_device_tree();
    fn smp_init_cpus();
    fn paging_init();
    fn zones_init();
    fn local_irq_disable();
    fn smp_send_stop();
    fn do_kernel_restart(cmd: *mut u8);
    fn do_kernel_power_off();
    fn cpu_relax();
    fn local_flush_tlb_all();
    fn invalidate_page_directory();
    fn itlb_probe(addr: usize) -> usize;
    fn invalidate_itlb_entry(entry: usize);
    fn write_itlb_entry(pte: usize, addr: usize);
    fn xtensa_get_sr(reg: usize) -> usize;
    fn num_online_cpus() -> u32;
    fn seq_printf(f: *mut seq_file, fmt: *const u8, ...);
    fn seq_puts(f: *mut seq_file, s: *const u8);
    fn register_cpu(cpu: *mut cpu, nr: i32) -> i32;
    fn of_flat_dt_is_compatible(node: usize, compat: *const u8) -> bool;
    fn of_get_flat_dt_prop(node: usize, name: *const u8, len: *mut i32) -> *const u32;
    fn of_read_ulong(cell: *const u32, count: i32) -> usize;
    fn init_kio();
    fn early_init_dt_scan(params: *mut c_void, pa: usize);
    fn of_scan_flat_dt(cb: unsafe extern "C" fn(usize, *const u8, i32, *mut c_void) -> i32, data: *mut c_void);
}

#[repr(C)] pub struct bp_tag_t { pub id: u32, pub size: usize, pub data: [usize; 0] }
#[repr(C)] pub struct bp_meminfo { pub type_: u32, pub start: usize, pub end: usize }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct cpu { pub hotpluggable: bool }

unsafe fn parse_tag_mem(tag: *const bp_tag_t) -> i32 {
    let mi = (*tag).data.as_ptr() as *const bp_meminfo;
    if (*mi).type_ != MEMORY_TYPE_CONVENTIONAL { return -1; }
    memblock_add((*mi).start, (*mi).end.wrapping_sub((*mi).start))
}

unsafe fn parse_tag_initrd(tag: *const bp_tag_t) -> i32 {
    let mi = (*tag).data.as_ptr() as *const bp_meminfo;
    initrd_start = __va((*mi).start) as usize;
    initrd_end = __va((*mi).end) as usize;
    0
}

unsafe fn parse_tag_fdt(tag: *const bp_tag_t) -> i32 {
    dtb_start = __va((*tag).data[0]); 0
}

unsafe fn parse_tag_cmdline(tag: *const bp_tag_t) -> i32 {
    strscpy(command_line.as_mut_ptr(), (*tag).data.as_ptr() as *const u8, COMMAND_LINE_SIZE); 0
}

unsafe fn parse_bootparam(mut tag: *const bp_tag_t) -> i32 {
    if (*tag).id != BP_TAG_FIRST { pr_warn(b"Invalid boot parameters!\0".as_ptr()); return 0; }
    tag = ((tag as usize).wrapping_add(core::mem::size_of::<bp_tag_t>()).wrapping_add((*tag).size)) as *const bp_tag_t;
    while !tag.is_null() && (*tag).id != BP_TAG_LAST {
        let mut t = &__tagtable_begin as *const tagtable_t;
        while t < &__tagtable_end as *const tagtable_t {
            if (*tag).id == (*t).tag { if let Some(parse) = (*t).parse { parse(tag); } break; }
            t = t.add(1);
        }
        if t == &__tagtable_end as *const tagtable_t { pr_warn(b"Ignoring tag 0x%08x\n\0".as_ptr(), (*tag).id); }
        tag = ((tag as usize).wrapping_add(core::mem::size_of::<bp_tag_t>()).wrapping_add((*tag).size)) as *const bp_tag_t;
    }
    0
}

pub unsafe extern "C" fn init_arch(bp_start: *mut bp_tag_t) {
    if IS_ENABLED(CONFIG_KASAN) || IS_ENABLED(CONFIG_XTENSA_LOAD_STORE) { early_trap_init(); }
    init_mmu(); kasan_early_init();
    if !bp_start.is_null() { parse_bootparam(bp_start); }
    #[cfg(CONFIG_USE_OF)] early_init_devtree(dtb_start);
    #[cfg(CONFIG_CMDLINE_BOOL)] if command_line[0] == 0 { strscpy(command_line.as_mut_ptr(), default_command_line.as_ptr(), COMMAND_LINE_SIZE); }
    platform_init(bp_start);
}

unsafe fn mem_reserve(start: usize, end: usize) -> i32 { memblock_reserve(start, end.wrapping_sub(start)) }

pub unsafe extern "C" fn setup_arch(cmdline_p: *mut *mut u8) {
    pr_info(b"config ID: %08x:%08x\n\0".as_ptr(), xtensa_get_sr(SREG_EPC), xtensa_get_sr(SREG_EXCSAVE));
    if xtensa_get_sr(SREG_EPC) != XCHAL_HW_CONFIGID0 || xtensa_get_sr(SREG_EXCSAVE) != XCHAL_HW_CONFIGID1 { pr_info(b"built for config ID: %08x:%08x\n\0".as_ptr(), XCHAL_HW_CONFIGID0, XCHAL_HW_CONFIGID1); }
    *cmdline_p = command_line.as_mut_ptr(); platform_setup(cmdline_p); strscpy(boot_command_line.as_mut_ptr(), *cmdline_p, COMMAND_LINE_SIZE);
    #[cfg(CONFIG_BLK_DEV_INITRD)] if initrd_start < initrd_end && mem_reserve(__pa(initrd_start), __pa(initrd_end)) == 0 { initrd_below_start_ok = 1; } else { initrd_start = 0; }
    mem_reserve(__pa(_stext as usize), __pa(_end as usize));
    parse_early_param(); bootmem_init(); kasan_init(); unflatten_and_copy_device_tree();
    #[cfg(CONFIG_SMP)] smp_init_cpus(); paging_init(); zones_init();
}

pub unsafe extern "C" fn cpu_reset() -> ! {
    #[cfg(all(XCHAL_HAVE_PTP_MMU, CONFIG_MMU))] { local_irq_disable(); local_flush_tlb_all(); invalidate_page_directory(); }
    core::arch::asm!("movi a2, 0", "wsr a2, icountlevel", "movi a2, 0", "wsr a2, icount", "movi a2, 0x1f", "wsr a2, ps", "isync", "jx {reset}", reset = const XCHAL_RESET_VECTOR_VADDR, options(noreturn));
}

pub unsafe extern "C" fn machine_restart(cmd: *mut u8) -> ! { local_irq_disable(); smp_send_stop(); do_kernel_restart(cmd); pr_err(b"Reboot failed -- System halted\n\0".as_ptr()); loop { cpu_relax(); } }
pub unsafe extern "C" fn machine_halt() -> ! { local_irq_disable(); smp_send_stop(); do_kernel_power_off(); loop { cpu_relax(); } }
pub unsafe extern "C" fn machine_power_off() -> ! { local_irq_disable(); smp_send_stop(); do_kernel_power_off(); loop { cpu_relax(); } }

#[cfg(CONFIG_USE_OF)]
pub unsafe extern "C" fn early_init_devtree(params: *mut c_void) { early_init_dt_scan(params, __pa(params as usize)); of_scan_flat_dt(xtensa_dt_io_area, core::ptr::null_mut()); if command_line[0] == 0 { strscpy(command_line.as_mut_ptr(), boot_command_line.as_ptr(), COMMAND_LINE_SIZE); } }

#[cfg(CONFIG_USE_OF)]
unsafe extern "C" fn xtensa_dt_io_area(node: usize, _uname: *const u8, depth: i32, _data: *mut c_void) -> i32 { if depth > 1 || !of_flat_dt_is_compatible(node, b"simple-bus\0".as_ptr()) { return 0; } let mut len = 0; let ranges = of_get_flat_dt_prop(node, b"ranges\0".as_ptr(), &mut len); if ranges.is_null() || len == 0 { return 1; } xtensa_kio_paddr = of_read_ulong(ranges.add(1), 1) & 0xf0000000; init_kio(); 1 }

static mut cpu_data: cpu = cpu { hotpluggable: false };

unsafe extern "C" fn topology_init() -> i32 {
    let mut i = 0;
    while i < num_possible_cpus() {
        cpu_data.hotpluggable = i != 0;
        register_cpu(&mut cpu_data, i);
        i += 1;
    }
    0
}

#[cfg(CONFIG_PROC_FS)]
unsafe extern "C" fn c_start(_f: *mut seq_file, pos: *mut i64) -> *mut c_void { if *pos == 0 { 1usize as *mut c_void } else { core::ptr::null_mut() } }
#[cfg(CONFIG_PROC_FS)]
unsafe extern "C" fn c_next(f: *mut seq_file, _v: *mut c_void, pos: *mut i64) -> *mut c_void { *pos += 1; c_start(f, pos) }
#[cfg(CONFIG_PROC_FS)]
unsafe extern "C" fn c_stop(_f: *mut seq_file, _v: *mut c_void) {}

#[cfg(CONFIG_PROC_FS)]
unsafe extern "C" fn c_show(f: *mut seq_file, _slot: *mut c_void) -> i32 {
    seq_printf(f, b"CPU count\t: %u\nvendor_id\t: Tensilica\nmodel\t\t: Xtensa %s\ncore ID\t\t: %s\nbuild ID\t: 0x%x\nconfig ID\t: %08x:%08x\nbyte order\t: %s\ncpu MHz\t\t: %lu.%02lu\nbogomips\t: %lu.%02lu\n\0".as_ptr(), num_online_cpus(), XCHAL_HW_VERSION_NAME, XCHAL_CORE_ID, XCHAL_BUILD_UNIQUE_ID, xtensa_get_sr(SREG_EPC), xtensa_get_sr(SREG_EXCSAVE), if XCHAL_HAVE_BE { b"big\0".as_ptr() } else { b"little\0".as_ptr() }, ccount_freq / 1_000_000, (ccount_freq / 10_000) % 100, loops_per_jiffy / (500_000 / HZ), (loops_per_jiffy / (5_000 / HZ)) % 100);
    seq_puts(f, b"flags\t\t: "
        /* Feature strings are selected by the same build-time configuration macros as C. */
        b"\n\0".as_ptr());
    seq_printf(f, b"physical aregs\t: %d\nmisc regs\t: %d\nibreak\t\t: %d\ndbreak\t\t: %d\nperf counters\t: %d\n\0".as_ptr(), XCHAL_NUM_AREGS, XCHAL_NUM_MISC_REGS, XCHAL_NUM_IBREAK, XCHAL_NUM_DBREAK, XCHAL_NUM_PERF_COUNTERS);
    seq_printf(f, b"num ints\t: %d\next ints\t: %d\nint levels\t: %d\ntimers\t\t: %d\ndebug level\t: %d\n\0".as_ptr(), XCHAL_NUM_INTERRUPTS, XCHAL_NUM_EXTINTERRUPTS, XCHAL_NUM_INTLEVELS, XCHAL_NUM_TIMERS, XCHAL_DEBUGLEVEL);
    seq_printf(f, b"icache line size: %d\nicache ways\t: %d\nicache size\t: %d\nicache flags\t: \ndcache line size: %d\ndcache ways\t: %d\ndcache size\t: %d\ndcache flags\t: \n\0".as_ptr(), XCHAL_ICACHE_LINESIZE, XCHAL_ICACHE_WAYS, XCHAL_ICACHE_SIZE, XCHAL_DCACHE_LINESIZE, XCHAL_DCACHE_WAYS, XCHAL_DCACHE_SIZE);
    0
}

#[cfg(CONFIG_PROC_FS)]
#[repr(C)] pub struct seq_operations { pub start: unsafe extern "C" fn(*mut seq_file, *mut i64) -> *mut c_void, pub next: unsafe extern "C" fn(*mut seq_file, *mut c_void, *mut i64) -> *mut c_void, pub stop: unsafe extern "C" fn(*mut seq_file, *mut c_void), pub show: unsafe extern "C" fn(*mut seq_file, *mut c_void) -> i32 }
#[cfg(CONFIG_PROC_FS)]
#[no_mangle] pub static cpuinfo_op: seq_operations = seq_operations { start: c_start, next: c_next, stop: c_stop, show: c_show };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
