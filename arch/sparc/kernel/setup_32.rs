// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/arch/sparc/kernel/setup.c
 *
 *  Copyright (C) 1995  David S. Miller (davem@caip.rutgers.edu)
 *  Copyright (C) 2000  Anton Blanchard (anton@samba.org)
 */

// Declarations supplied by the Linux/SPARC environment are intentionally
// left external; the original C file obtains them from its included headers.

extern "C" {
    fn local_irq_save(flags: *mut kernel::ffi::c_ulong);
    fn local_irq_enable();
    fn local_irq_disable();
    fn local_irq_restore(flags: kernel::ffi::c_ulong);
    fn prom_printf(fmt: *const kernel::ffi::c_char, ...);
    fn show_mem();
    fn is_idle_task(task: *mut kernel::ffi::c_void) -> bool;
    fn ksys_sync();
    fn prom_halt() -> !;
    fn prom_write(s: *const kernel::ffi::c_char, n: kernel::ffi::c_uint);
    fn printk(fmt: *const kernel::ffi::c_char, ...);
    fn simple_strtoul(s: *const kernel::ffi::c_char, end: *mut *mut kernel::ffi::c_char, base: kernel::ffi::c_uint) -> kernel::ffi::c_ulong;
    fn strncmp(a: *const kernel::ffi::c_char, b: *const kernel::ffi::c_char, n: usize) -> kernel::ffi::c_int;
    fn strcmp(a: *const kernel::ffi::c_char, b: *const kernel::ffi::c_char) -> kernel::ffi::c_int;
    fn prom_init(rp: *mut linux_romvec);
    fn start_kernel();
    fn prom_getbootargs() -> *mut kernel::ffi::c_char;
    fn strscpy(dst: *mut kernel::ffi::c_char, src: *const kernel::ffi::c_char, size: usize) -> isize;
    fn parse_early_param();
    fn register_console(con: *mut console);
    fn idprom_init();
    fn load_mmu();
    fn old_decode_dev(dev: kernel::ffi::c_ushort) -> kernel::ffi::c_uint;
    fn prom_setsync(f: Option<unsafe extern "C" fn()>);
    fn flushi(addr: kernel::ffi::c_ulong);
    fn paging_init();
    fn smp_setup_cpu_possible_map();
    fn flush_user_windows();
    fn prom_cmdline();
    fn cpu_find_by_instance(instance: kernel::ffi::c_int, arg1: *mut kernel::ffi::c_void, arg2: *mut kernel::ffi::c_void) -> kernel::ffi::c_int;
    fn register_cpu(p: *mut cpu, i: kernel::ffi::c_int) -> kernel::ffi::c_int;
    fn kzalloc(size: usize, flags: kernel::ffi::c_uint) -> *mut kernel::ffi::c_void;
}

const BOOTME_DEBUG: kernel::ffi::c_uint = 0x1;
const RAMDISK_IMAGE_START_MASK: kernel::ffi::c_ushort = 0x07ff;
const RAMDISK_PROMPT_FLAG: kernel::ffi::c_ushort = 0x8000;
const RAMDISK_LOAD_FLAG: kernel::ffi::c_ushort = 0x4000;

#[repr(C)]
pub struct console { pub name: *const kernel::ffi::c_char, pub write: Option<unsafe extern "C" fn(*mut console, *const kernel::ffi::c_char, kernel::ffi::c_uint)>, pub flags: kernel::ffi::c_uint, pub index: kernel::ffi::c_int }
#[repr(C)] pub struct linux_romvec { _private: [u8; 0] }
#[repr(C)] pub struct tt_entry { _private: [u8; 0] }
#[repr(C)] pub struct cpu { _private: [u8; 0] }
#[repr(C)] pub struct cpuid_patch_entry { pub addr: kernel::ffi::c_uint, pub sun4d: [kernel::ffi::c_uint; 3], pub leon: [kernel::ffi::c_uint; 3] }
#[repr(C)] pub struct leon_1insn_patch_entry { pub addr: kernel::ffi::c_uint, pub insn: kernel::ffi::c_uint }

extern "C" {
    static mut boot_command_line: [kernel::ffi::c_char; 0];
    static mut trapbase: [tt_entry; 0];
    static mut cputypval: [kernel::ffi::c_char; 0];
    static mut __cpuid_patch: cpuid_patch_entry;
    static mut __cpuid_patch_end: cpuid_patch_entry;
    static mut __leon_1insn_patch: kernel::ffi::c_ulong;
    static mut __leon_1insn_patch_end: kernel::ffi::c_ulong;
    static mut sparc_cpu_model: sparc_cpu;
    static mut sp_banks: [sp_bank; 0];
    static mut phys_base: kernel::ffi::c_ulong;
    static mut pfn_base: kernel::ffi::c_ulong;
    static mut root_flags: kernel::ffi::c_ushort;
    static mut root_dev: kernel::ffi::c_ushort;
    static mut ram_flags: kernel::ffi::c_ushort;
    static mut root_mountflags: kernel::ffi::c_int;
    static mut linux_dbvec: *mut debug_vec;
    static mut loops_per_jiffy: kernel::ffi::c_ulong;
    static mut ncpus_probed: kernel::ffi::c_int;
    static mut ROOT_DEV: kernel::ffi::c_uint;
    static mut rd_image_start: kernel::ffi::c_ulong;
}

#[repr(C)] pub struct sp_bank { pub base_addr: kernel::ffi::c_ulong, pub num_bytes: kernel::ffi::c_ulong }
#[repr(C)] pub struct debug_vec { pub teach_debugger: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub enum sparc_cpu { sun_unknown, sun4m, sun4d, sun4e, sun4u, sparc_leon }

static mut boot_flags: kernel::ffi::c_uint = 0;
pub static mut cmdline_memory_size: kernel::ffi::c_ulong = 0;
pub static mut boot_cpu_id: kernel::ffi::c_uchar = 0xff;
pub static mut reboot_command: [kernel::ffi::c_char; 256] = [0; 256];
pub static mut sparc_ttable: *mut tt_entry = core::ptr::null_mut();
static mut prom_early_console: console = console { name: b"earlyprom\0".as_ptr() as *const _, write: Some(prom_console_write), flags: 0, index: -1 };

unsafe extern "C" fn prom_sync_me() {
    let mut prom_tbr: kernel::ffi::c_ulong = 0;
    let mut flags: kernel::ffi::c_ulong = 0;
    local_irq_save(&mut flags);
    prom_printf(b"PROM SYNC COMMAND...\n\0".as_ptr() as _,);
    show_mem();
    // The inline SPARC TBR reads/writes are architecture-specific assembly.
    if !is_idle_task(core::ptr::null_mut()) { local_irq_enable(); ksys_sync(); local_irq_disable(); }
    prom_printf(b"Returning to prom\n\0".as_ptr() as _);
    let _ = prom_tbr;
    local_irq_restore(flags);
}

unsafe extern "C" fn prom_console_write(_con: *mut console, s: *const kernel::ffi::c_char, n: kernel::ffi::c_uint) { prom_write(s, n); }

unsafe fn process_switch(c: kernel::ffi::c_char) {
    match c as u8 { b'd' => boot_flags |= BOOTME_DEBUG, b's' => {}, b'h' => { prom_printf(b"boot_flags_init: Halt!\n\0".as_ptr() as _); prom_halt(); }, b'p' => prom_early_console.flags &= !1, _ => printk(b"Unknown boot switch (-%c)\n\0".as_ptr() as _, c as kernel::ffi::c_int) }
}

unsafe fn boot_flags_init(mut commands: *mut kernel::ffi::c_char) {
    while *commands != 0 { while *commands == b' ' as _ { commands = commands.add(1); } if *commands == 0 { break; } if *commands == b'-' as _ { commands = commands.add(1); while *commands != 0 && *commands != b' ' as _ { process_switch(*commands); commands = commands.add(1); } continue; } if strncmp(commands, b"mem=\0".as_ptr() as _, 4) == 0 { cmdline_memory_size = simple_strtoul(commands.add(4), &mut commands, 0); if *commands == b'K' as _ || *commands == b'k' as _ { cmdline_memory_size <<= 10; commands = commands.add(1); } else if *commands == b'M' as _ || *commands == b'm' as _ { cmdline_memory_size <<= 20; commands = commands.add(1); } } while *commands != 0 && *commands != b' ' as _ { commands = commands.add(1); } }
}

unsafe fn per_cpu_patch() {
    if matches!(sparc_cpu_model, sparc_cpu::sun4m) { return; }
    let mut p = &mut __cpuid_patch as *mut cpuid_patch_entry;
    while p < &mut __cpuid_patch_end as *mut _ { let addr = (*p).addr as usize; let insns = match sparc_cpu_model { sparc_cpu::sun4d => (*p).sun4d.as_ptr(), sparc_cpu::sparc_leon => (*p).leon.as_ptr(), _ => { prom_printf(b"Unknown cpu type, halting.\n\0".as_ptr() as _); prom_halt(); } }; for i in 0..3 { *((addr + i * 4) as *mut kernel::ffi::c_uint) = *insns.add(i); flushi((addr + i * 4) as _); } p = p.add(1); }
}

unsafe fn leon_patch() { if matches!(sparc_cpu_model, sparc_cpu::sparc_leon) { return; } let mut p = __leon_1insn_patch as *mut leon_1insn_patch_entry; let end = __leon_1insn_patch_end as *mut leon_1insn_patch_entry; while p < end { *( (*p).addr as *mut kernel::ffi::c_uint) = (*p).insn; flushi((*p).addr as _); p = p.add(1); } }

pub unsafe extern "C" fn sparc32_start_kernel(rp: *mut linux_romvec) { prom_init(rp); sparc_cpu_model = sparc_cpu::sun_unknown; if strcmp(cputypval.as_ptr(), b"sun4m\0".as_ptr() as _) == 0 || strcmp(cputypval.as_ptr(), b"sun4s\0".as_ptr() as _) == 0 { sparc_cpu_model = sparc_cpu::sun4m; } if strcmp(cputypval.as_ptr(), b"sun4d\0".as_ptr() as _) == 0 { sparc_cpu_model = sparc_cpu::sun4d; } if strcmp(cputypval.as_ptr(), b"sun4e\0".as_ptr() as _) == 0 { sparc_cpu_model = sparc_cpu::sun4e; } if strcmp(cputypval.as_ptr(), b"sun4u\0".as_ptr() as _) == 0 { sparc_cpu_model = sparc_cpu::sun4u; } if strncmp(cputypval.as_ptr(), b"leon\0".as_ptr() as _, 4) == 0 { sparc_cpu_model = sparc_cpu::sparc_leon; } leon_patch(); start_kernel(); }

pub unsafe extern "C" fn sun_do_break() { if STOP_A_ENABLED == 0 { return; } printk(b"\n\0".as_ptr() as _); flush_user_windows(); prom_cmdline(); }
static mut STOP_A_ENABLED: kernel::ffi::c_int = 1;

pub unsafe extern "C" fn setup_arch(cmdline_p: *mut *mut kernel::ffi::c_char) {
    sparc_ttable = trapbase.as_mut_ptr();
    *cmdline_p = prom_getbootargs();
    strscpy(boot_command_line.as_mut_ptr(), *cmdline_p, 256);
    parse_early_param();
    boot_flags_init(*cmdline_p);
    register_console(&mut prom_early_console);
    idprom_init();
    load_mmu();
    phys_base = 0xffff_ffff;
    let mut highest_paddr: kernel::ffi::c_ulong = 0;
    let mut i = 0usize;
    while sp_banks[i].num_bytes != 0 {
        if sp_banks[i].base_addr < phys_base { phys_base = sp_banks[i].base_addr; }
        let top = sp_banks[i].base_addr + sp_banks[i].num_bytes;
        if highest_paddr < top { highest_paddr = top; }
        i += 1;
    }
    pfn_base = phys_base >> 12;
    if root_flags == 0 { root_mountflags &= !1; }
    ROOT_DEV = old_decode_dev(root_dev);
    // CONFIG_BLK_DEV_RAM controls this assignment in the original build.
    rd_image_start = (ram_flags & RAMDISK_IMAGE_START_MASK) as kernel::ffi::c_ulong;
    prom_setsync(Some(prom_sync_me));
    per_cpu_patch();
    paging_init();
    smp_setup_cpu_possible_map();
}

unsafe fn topology_init() -> kernel::ffi::c_int {
    let mut ncpus = 0;
    while cpu_find_by_instance(ncpus, core::ptr::null_mut(), core::ptr::null_mut()) == 0 { ncpus += 1; }
    ncpus_probed = ncpus;
    let mut err = 0;
    // for_each_online_cpu(i)
    for i in 0..ncpus {
        let p = kzalloc(core::mem::size_of::<cpu>(), 0) as *mut cpu;
        if p.is_null() { err = -12; } else { register_cpu(p, i); }
    }
    err
}

// subsys_initcall(topology_init);
// CONFIG_SPARC32 && !CONFIG_SMP:
pub unsafe extern "C" fn arch_cpu_finalize_init() {
    // cpu_data(0).udelay_val = loops_per_jiffy;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
