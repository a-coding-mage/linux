// SPDX-License-Identifier: GPL-2.0-or-later
/* Common boot and setup code for both 32-bit and 64-bit. */

// Kernel headers and configuration supplied by the surrounding translation unit.

extern "C" {
    static mut ppc_md: machdep_calls;
    static mut machine_id: *mut machdep_calls;
    static mut boot_cpuid: c_int;
    static mut dcache_bsize: c_int;
    static mut icache_bsize: c_int;
    static mut of_i8042_kbd_irq: c_int;
    static mut of_i8042_aux_irq: c_int;
    static mut pm_power_off: Option<unsafe extern "C" fn()>;
}

#[repr(C)] pub struct machdep_calls { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct notifier_block { _private: [u8; 0] }
#[repr(C)] pub struct seq_buf { pub buffer: *mut c_char, pub size: usize, pub len: usize }
pub type c_int = i32; pub type c_char = i8; pub type c_uint = u32;
pub type c_ulong = usize; pub type loff_t = i64; pub type __be32 = u32;

#[no_mangle] pub static mut boot_core_hwid: c_int = -1;
#[no_mangle] pub static mut boot_cpu_hwid: c_int = -1;
#[no_mangle] pub static mut crashing_cpu: c_int = -1;
#[no_mangle] pub static mut ppc_hw_desc_buf: [c_char; 128] = [0; 128];
#[no_mangle] pub static mut ppc_hw_desc: seq_buf = seq_buf { buffer: unsafe { ppc_hw_desc_buf.as_mut_ptr() }, size: 128, len: 0 };

extern "C" {
    fn fadump_cleanup(); fn local_irq_disable(); fn smp_send_stop(); fn do_kernel_restart(*mut c_char);
    fn do_kernel_power_off(); fn mdelay(c_uint); fn hard_irq_disable(); fn kaslr_offset() -> c_ulong;
    fn should_fadump_crash() -> bool; fn crash_fadump(*mut core::ffi::c_void, *mut core::ffi::c_void);
    fn kmsg_dump_desc(c_int, *mut c_char); fn pr_emerg(*const c_char, ...); fn pr_info(*const c_char, ...);
    fn pr_err(*const c_char, ...); fn printk(c_int, *const c_char, ...); fn seq_printf(*mut seq_file, *const c_char, ...);
    fn seq_puts(*mut seq_file, *const c_char); fn seq_putc(*mut seq_file, c_int);
    fn of_find_node_by_path(*const c_char) -> *mut device_node; fn of_node_put(*mut device_node);
    fn of_get_property(*mut device_node, *const c_char, *mut c_int) -> *const c_char;
    fn cpu_has_feature(c_int) -> bool; fn cpumask_last(*const core::ffi::c_void) -> c_ulong;
    fn cpumask_next(c_ulong, *const core::ffi::c_void) -> c_ulong; fn mfspr(c_int) -> c_uint;
    fn cpu_temp(c_ulong) -> c_uint; fn cpu_temp_both(c_ulong) -> c_uint;
    fn be32_to_cpu(u32) -> u32; fn PVR_VER(u32) -> u32; fn PVR_MIN(u32) -> u16; fn PVR_MAJ(u32) -> u16;
    fn of_device_is_available(*mut device_node) -> bool; fn of_property_match_string(*mut device_node,*const c_char,*const c_char)->c_int;
    fn of_get_parent(*mut device_node)->*mut device_node; fn irq_of_parse_and_map(*mut device_node,c_int)->c_int;
    fn of_find_compatible_node(*mut device_node,*mut device_node,*const c_char)->*mut device_node;
    fn of_find_node_by_type(*mut device_node,*const c_char)->*mut device_node; fn of_find_node_by_name(*mut device_node,*const c_char)->*mut device_node;
    fn of_node_is_type(*mut device_node,*const c_char)->bool; fn of_machine_is_compatible(*const c_char)->bool;
    fn of_machine_compatible_match(*const *const c_char)->bool; fn dump_stack_set_arch_desc(*mut c_char);
    fn seq_buf_puts(*mut seq_buf,*const c_char); fn atomic_notifier_chain_register(*mut notifier_block,*mut notifier_block)->c_int;
    fn setup_power_save(); fn find_legacy_serial_ports(); fn register_early_udbg_console(); fn xmon_setup();
    fn check_smt_enabled(); fn mem_topology_setup(); fn __va(usize)->*mut core::ffi::c_void; fn setup_tlb_core_data();
    fn klp_init_thread_info(*mut core::ffi::c_void); fn setup_initial_init_mm(*mut c_char,*mut c_char,*mut c_char,*mut c_char);
    fn smp_processor_id()->c_int; fn cpumask_test_cpu(c_int,*mut core::ffi::c_void)->bool; fn cpumask_set_cpu(c_int,*mut core::ffi::c_void);
    fn inc_mm_active_cpus(*mut core::ffi::c_void); fn mm_iommu_init(*mut core::ffi::c_void); fn irqstack_early_init(); fn exc_lvl_early_init();
    fn emergency_stack_init(); fn mce_init(); fn smp_release_cpus(); fn initmem_init(); fn early_memtest(usize,usize);
    fn setup_barrier_nospec(); fn setup_spectre_v2(); fn paging_init(); fn mmu_context_init(); fn panic(*const c_char,...);
}

#[no_mangle] pub unsafe extern "C" fn machine_shutdown() { fadump_cleanup(); }
unsafe fn machine_hang() -> ! { pr_emerg(b"System Halted, OK to turn off power\n\0".as_ptr() as _); local_irq_disable(); loop {} }
#[no_mangle] pub unsafe extern "C" fn machine_restart(cmd:*mut c_char) { machine_shutdown(); smp_send_stop(); do_kernel_restart(cmd); mdelay(1000); machine_hang(); }
#[no_mangle] pub unsafe extern "C" fn machine_power_off() { machine_shutdown(); do_kernel_power_off(); smp_send_stop(); machine_hang(); }
#[no_mangle] pub unsafe extern "C" fn arch_get_random_seed_longs(v:*mut c_ulong,max:c_ulong)->c_ulong { if max != 0 { let _=v; } 0 }
#[no_mangle] pub unsafe extern "C" fn machine_halt() { machine_shutdown(); smp_send_stop(); machine_hang(); }

unsafe fn c_start(_m:*mut seq_file,pos:*mut loff_t)->*mut core::ffi::c_void { *pos=cpumask_next((*pos-1) as _, core::ptr::null()); if *pos < 0 { (*pos+1) as *mut _ } else { core::ptr::null_mut() } }
unsafe extern "C" fn c_next(m:*mut seq_file,_v:*mut core::ffi::c_void,pos:*mut loff_t)->*mut core::ffi::c_void { *pos+=1; c_start(m,pos) }
unsafe extern "C" fn c_stop(_m:*mut seq_file,_v:*mut core::ffi::c_void) {}

#[no_mangle] pub unsafe extern "C" fn check_legacy_ioport(_base_port:c_ulong)->c_int { -19 }

unsafe extern "C" fn ppc_panic_fadump_handler(_t:*mut notifier_block,_e:c_ulong,ptr:*mut core::ffi::c_void)->c_int { hard_irq_disable(); if should_fadump_crash(){kmsg_dump_desc(1,ptr as _);} crash_fadump(core::ptr::null_mut(),ptr); 0 }
unsafe extern "C" fn dump_kernel_offset(_s:*mut notifier_block,_v:c_ulong,_p:*mut core::ffi::c_void)->c_int { pr_emerg(b"Kernel Offset: 0x%lx\n\0".as_ptr() as _,kaslr_offset()); 0 }
unsafe extern "C" fn ppc_panic_platform_handler(_t:*mut notifier_block, _e:c_ulong, _p:*mut core::ffi::c_void)->c_int { 0 }

#[no_mangle] pub unsafe extern "C" fn ppc_printk_progress(s:*mut c_char,_hex:u16){pr_info(b"%s\n\0".as_ptr() as _,s);}
#[no_mangle] pub unsafe extern "C" fn setup_arch(cmdline_p:*mut *mut c_char) { kasan_init(); *cmdline_p=boot_command_line; loops_per_jiffy=500000000/HZ; unflatten_device_tree(); initialize_cache_info(); rtas_initialize(); check_for_initrd(); probe_machine(); setup_panic(); setup_power_save(); find_legacy_serial_ports(); register_early_udbg_console(); smp_setup_cpu_maps(); xmon_setup(); check_smt_enabled(); mem_topology_setup(); high_memory=__va(max_low_pfn*PAGE_SIZE); setup_tlb_core_data(); print_system_info(); klp_init_thread_info(core::ptr::null_mut()); setup_initial_init_mm(core::ptr::null_mut(),core::ptr::null_mut(),core::ptr::null_mut(),core::ptr::null_mut()); irqstack_early_init(); exc_lvl_early_init(); emergency_stack_init(); mce_init(); smp_release_cpus(); initmem_init(); early_memtest(min_low_pfn<<PAGE_SHIFT,max_low_pfn<<PAGE_SHIFT); setup_barrier_nospec(); setup_spectre_v2(); paging_init(); mmu_context_init(); }

extern "C" { fn kasan_init(); fn unflatten_device_tree(); fn initialize_cache_info(); fn rtas_initialize(); fn check_for_initrd(); fn probe_machine(); fn setup_panic(); fn smp_setup_cpu_maps(); fn print_system_info(); static mut boot_command_line:*mut c_char; static mut loops_per_jiffy:usize; static HZ:usize; static mut high_memory:*mut core::ffi::c_void; static max_low_pfn:usize; static min_low_pfn:usize; static PAGE_SIZE:usize; static PAGE_SHIFT:usize; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
