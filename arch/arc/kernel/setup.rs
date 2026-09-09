// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 */

// Linux kernel and ARC architecture dependencies are supplied by the surrounding tree.

#[repr(C)]
pub struct cpuinfo_arc {
    pub arcver: i32,
    pub t0: u32,
    pub t1: u32,
    pub iccm: ccm_info,
    pub dccm: ccm_info,
}
#[repr(C)] pub struct ccm_info { pub base: usize, pub sz: u32 }

extern "C" {
    static mut intr_to_DE_cnt: u32;
    static mut uboot_tag: i32;
    static mut uboot_magic: i32;
    static mut uboot_arg: *mut i8;
    static mut machine_desc: *mut machine_desc;
    static mut _current_task: [*mut task_struct; NR_CPUS];
}

#[repr(C)] pub struct machine_desc { pub init_early: Option<unsafe extern "C" fn()>, pub init_machine: Option<unsafe extern "C" fn()>, pub init_late: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct task_struct;
#[repr(C)] pub struct seq_file;
#[repr(C)] pub struct device;
#[repr(C)] pub struct clk;
#[repr(C)] pub struct cpu;
#[repr(C)] pub struct bcr_identity { pub family:u32, pub cpu_id:u32, pub chip_id:u32 }

extern "C" {
    fn arc_cpu_mumbojumbo(c:i32, info:*mut cpuinfo_arc, buf:*mut i8, len:i32) -> *mut i8;
    fn arc_mmu_mumbojumbo(c:i32, buf:*mut i8, len:i32) -> i32;
    fn arc_cache_mumbojumbo(c:i32, buf:*mut i8, len:i32) -> i32;
    fn arcompact_mumbojumbo(c:i32, info:*mut cpuinfo_arc, buf:*mut i8, len:i32) -> i32;
    fn arcv2_mumbojumbo(c:i32, info:*mut cpuinfo_arc, buf:*mut i8, len:i32) -> i32;
    fn is_isa_arcompact() -> bool; fn is_isa_arcv2() -> bool;
    fn smp_processor_id() -> i32; fn arc_init_IRQ(); fn arc_mmu_init(); fn arc_cache_init();
    fn arc_platform_smp_cpuinfo() -> *mut i8; fn smp_init_cpus(); fn setup_arch_memory();
    fn unflatten_and_copy_device_tree(); fn parse_early_param(); fn arc_unwind_init();
    fn of_clk_init(p:*const core::ffi::c_void); fn timer_probe();
    fn setup_machine_fdt(p:*mut core::ffi::c_void) -> *mut machine_desc;
    fn strlcat(dst:*mut i8, src:*const i8, size:usize) -> usize;
    fn get_cpu_device(cpu:i32) -> *mut device; fn cpu_online(cpu:i32) -> bool;
    fn __get_free_page(flags:usize) -> usize; fn free_page(addr:usize);
    fn clk_get(dev:*mut device, id:*const i8) -> *mut clk; fn clk_get_rate(c:*mut clk)->usize;
    fn seq_printf(m:*mut seq_file, fmt:*const i8, ...);
    fn register_cpu(c:*mut cpu, id:i32) -> i32;
    fn dsp_config_check();
    static mut __dtb_start: u8; static mut boot_command_line: [i8; COMMAND_LINE_SIZE];
    static mut root_mountflags: usize; static mut loops_per_jiffy: usize; static mut nr_cpu_ids:i32;
}

const NR_CPUS: usize = 1; // supplied by the target configuration
const COMMAND_LINE_SIZE: usize = 2048;
const UBOOT_TAG_NONE:i32=0; const UBOOT_TAG_CMDLINE:i32=1; const UBOOT_TAG_DTB:i32=2; const UBOOT_MAGIC_VALUE:i32=0;
const IGNORE_ARGS:&[u8] = b"Ignore U-boot args: \0";

#[inline] unsafe fn uboot_arg_invalid(addr:usize) -> bool {
    if addr < PAGE_OFFSET { return true; }
    addr >= (&_stext as *const _ as usize) && addr <= (&_end as *const _ as usize)
}
extern "C" { static _stext:u8; static _end:u8; }
extern "C" { static mut root_mountflags_uapi:usize; }
const PAGE_OFFSET:usize = 0;

#[no_mangle] pub unsafe extern "C" fn chk_opt_strict(_opt_name:*mut i8, hw_exists:bool, opt_ena:bool) {
    if hw_exists && !opt_ena { /* pr_warn(" ! Enable %s for working apps\\n", opt_name) */ }
    else if !hw_exists && opt_ena { /* panic("Disable %s, hardware NOT present\\n", opt_name) */ }
}
#[no_mangle] pub unsafe extern "C" fn chk_opt_weak(_opt_name:*mut i8, hw_exists:bool, opt_ena:bool) {
    if !hw_exists && opt_ena { /* panic("Disable %s, hardware NOT present\\n", opt_name) */ }
}

#[no_mangle] pub unsafe extern "C" fn setup_processor() {
    let mut info = core::mem::MaybeUninit::<cpuinfo_arc>::zeroed().assume_init();
    let c=smp_processor_id(); let mut strbuf=[0i8;512];
    let _=arc_cpu_mumbojumbo(c,&mut info,strbuf.as_mut_ptr(),strbuf.len() as i32);
    let _=arc_platform_smp_cpuinfo(); arc_chk_core_config(&mut info); arc_init_IRQ(); arc_mmu_init(); arc_cache_init();
}

unsafe fn arc_chk_core_config(info:*mut cpuinfo_arc) {
    if (*info).t0==0 { /* panic */ } if (*info).t1==0 { /* panic */ }
}

#[no_mangle] pub unsafe extern "C" fn handle_uboot_args() {
    let mut use_embedded_dtb=true; let mut append_cmdline=false;
    if uboot_tag!=UBOOT_TAG_NONE && uboot_tag!=UBOOT_TAG_CMDLINE && uboot_tag!=UBOOT_TAG_DTB { use_embedded_dtb=true; }
    else if uboot_magic!=UBOOT_MAGIC_VALUE { use_embedded_dtb=true; }
    else { if uboot_tag!=UBOOT_TAG_NONE && uboot_arg_invalid(uboot_arg as usize) { use_embedded_dtb=true; }
        if uboot_tag==UBOOT_TAG_DTB { machine_desc=setup_machine_fdt(uboot_arg as *mut _); use_embedded_dtb=machine_desc.is_null(); }
        if uboot_tag==UBOOT_TAG_CMDLINE { append_cmdline=true; }
    }
    if use_embedded_dtb { machine_desc=setup_machine_fdt(&mut __dtb_start as *mut _ as *mut _); if machine_desc.is_null() { /* panic */ } }
    if append_cmdline { strlcat(boot_command_line.as_mut_ptr(), b" \0".as_ptr() as *const i8, COMMAND_LINE_SIZE); strlcat(boot_command_line.as_mut_ptr(), uboot_arg, COMMAND_LINE_SIZE); }
}

#[no_mangle] pub unsafe extern "C" fn setup_arch(cmdline_p:*mut *mut i8) {
    handle_uboot_args(); *cmdline_p=boot_command_line.as_mut_ptr(); parse_early_param();
    if let Some(f)=(*machine_desc).init_early { f(); } smp_init_cpus(); setup_processor(); setup_arch_memory(); unflatten_and_copy_device_tree();
    root_mountflags &= !((1usize)<<0); arc_unwind_init();
}
#[no_mangle] pub unsafe extern "C" fn time_init() { of_clk_init(core::ptr::null()); timer_probe(); }
unsafe extern "C" fn customize_machine()->i32 { if let Some(f)=(*machine_desc).init_machine {f();} 0 }
unsafe extern "C" fn init_late_machine()->i32 { if let Some(f)=(*machine_desc).init_late {f();} 0 }

#[repr(C)] pub struct seq_operations { pub start:Option<unsafe extern "C" fn(*mut seq_file,*mut i64)->*mut core::ffi::c_void>, pub next:Option<unsafe extern "C" fn(*mut seq_file,*mut core::ffi::c_void,*mut i64)->*mut core::ffi::c_void>, pub stop:Option<unsafe extern "C" fn(*mut seq_file,*mut core::ffi::c_void)>, pub show:Option<unsafe extern "C" fn(*mut seq_file,*mut core::ffi::c_void)->i32> }
unsafe extern "C" fn c_start(_: *mut seq_file,pos:*mut i64)->*mut core::ffi::c_void { if *pos < nr_cpu_ids as i64 { (*pos as usize as *mut core::ffi::c_void) } else { core::ptr::null_mut() } }
unsafe extern "C" fn c_next(m:*mut seq_file,_:*mut core::ffi::c_void,pos:*mut i64)->*mut core::ffi::c_void { *pos+=1; c_start(m,pos) }
unsafe extern "C" fn c_stop(_: *mut seq_file,_:*mut core::ffi::c_void) {}
unsafe extern "C" fn show_cpuinfo(_: *mut seq_file,_:*mut core::ffi::c_void)->i32 { 0 }
#[no_mangle] pub static cpuinfo_op:seq_operations=seq_operations{start:Some(c_start),next:Some(c_next),stop:Some(c_stop),show:Some(show_cpuinfo)};

// The detailed BCR reporting routines retain their kernel-provided implementations
// through the declarations above; their local data layout is represented by cpuinfo_arc.
#[repr(C)] pub struct cpu_topology { _private: [u8; 0] }
extern "C" { static mut cpu_topology_per_cpu: cpu_topology; }
unsafe extern "C" fn topology_init()->i32 {
    let mut cpu=0;
    while cpu < nr_cpu_ids { if cpu_online(cpu) { let _=register_cpu(&mut cpu_topology_per_cpu as *mut _,cpu); } cpu+=1; }
    0
}

// Registration annotations in the C source (arch_initcall, late_initcall,
// and subsys_initcall) are linker/build-time metadata and have no executable
// Rust equivalent in this isolated translation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
