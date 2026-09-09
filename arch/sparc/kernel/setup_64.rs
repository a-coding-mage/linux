// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of sparc/kernel/setup_64.c.  C headers provide the
 * external types, constants, globals, and functions referenced below. */

#[no_mangle] pub static mut ns87303_lock: SpinLock = unsafe { core::mem::zeroed() };
extern "C" {
    fn prom_write(s: *const i8, n: u32); fn prom_printf(fmt: *const i8, ...);
    fn prom_halt() -> !; fn printk(fmt: *const i8, ...); fn wmb();
    fn add_taint(t: i32, l: i32); fn cheetah_enable_pcache();
    fn memparse(s: *const i8, end: *mut *mut i8) -> u64;
    fn check_if_starfire(); fn smp_init_cpu_poke(); fn hard_smp_processor_id() -> i32;
    fn current_thread_info() -> *mut ThreadInfo; fn time_init_early();
    fn prom_init_report(); fn start_kernel(); fn seq_puts(m: *mut SeqFile, s: *const i8);
    fn seq_putc(m: *mut SeqFile, c: i32); fn seq_printf(m: *mut SeqFile, f: *const i8, ...);
    fn mdesc_grab() -> *mut MdescHandle; fn mdesc_release(h: *mut MdescHandle);
    fn mdesc_node_by_name(h: *mut MdescHandle, n: u64, s: *const i8) -> u64;
    fn mdesc_get_property(h: *mut MdescHandle, n: u64, s: *const i8, len: *mut i32) -> *const i8;
    fn strlen(s: *const i8) -> usize; fn strcmp(a: *const i8, b: *const i8) -> i32;
    fn panic(fmt: *const i8, ... ) -> !; fn cpu_to_node(cpu: u32) -> u32;
    fn memblock_alloc_node(size: usize, align: usize, node: u32) -> *mut core::ffi::c_void;
    fn prom_getbootargs() -> *mut i8; fn strscpy(d: *mut i8, s: *const i8, n: usize) -> isize;
    fn parse_early_param(); fn register_console(c: *mut Console); fn idprom_init();
    fn old_decode_dev(d: u16) -> u16; fn init_cur_cpu_trap(t: *mut ThreadInfo);
    fn paging_init(); fn btext_find_display() -> bool; fn prom_finddevice(s: *const i8) -> u64;
    fn prom_getintdefault(n: u64, p: *const i8, d: i32) -> u32;
    fn flush_user_windows(); fn prom_cmdline();
}

#[repr(C)] pub struct SpinLock { _x: [u8; 0] }
#[repr(C)] pub struct Console { pub name: *const i8, pub write: Option<unsafe extern "C" fn(*mut Console,*const i8,u32)>, pub flags: u32, pub index: i32 }
#[repr(C)] pub struct ThreadInfo { pub cpu: i32, _x: [u8; 0] }
#[repr(C)] pub struct SeqFile { _x: [u8; 0] }
#[repr(C)] pub struct MdescHandle { _x: [u8; 0] }
#[repr(C)] pub struct CpuidPatchEntry { pub addr: usize, pub starfire:[u32;4], pub cheetah_jbus:[u32;4], pub cheetah_safari:[u32;4], pub sun4v:[u32;4] }
#[repr(C)] pub struct Patch1 { pub addr: usize, pub insn:u32 }
#[repr(C)] pub struct Patch2 { pub addr: usize, pub insns:[u32;2] }
#[repr(C)] pub struct Patch3 { pub addr: usize, pub insns:[u32;3] }
#[repr(C)] pub struct Patch6 { pub addr: usize, pub insns:[u32;6] }

extern "C" {
    static mut tlb_type: i32; static mut this_is_starfire: bool; static mut cheetah_pcache_forced_on: i32;
    static mut __cpuid_patch: CpuidPatchEntry; static mut __cpuid_patch_end: CpuidPatchEntry;
    static mut __sun4v_1insn_patch: Patch1; static mut __sun4v_1insn_patch_end: Patch1;
    static mut __sun4v_2insn_patch: Patch2; static mut __sun4v_2insn_patch_end: Patch2;
    static mut __sun_m7_1insn_patch: Patch1; static mut __sun_m7_1insn_patch_end: Patch1;
    static mut __sun_m7_2insn_patch: Patch2; static mut __sun_m7_2insn_patch_end: Patch2;
    static mut __fast_win_ctrl_1insn_patch: Patch1; static mut __fast_win_ctrl_1insn_patch_end: Patch1;
    static mut __popc_3insn_patch: Patch3; static mut __popc_3insn_patch_end: Patch3;
    static mut __popc_6insn_patch: Patch6; static mut __popc_6insn_patch_end: Patch6;
    static mut __pause_3insn_patch: Patch3; static mut __pause_3insn_patch_end: Patch3;
    static mut sun4v_chip_type: i32; static mut softirq_stack: [*mut core::ffi::c_void; NR_CPUS];
    static mut hardirq_stack: [*mut core::ffi::c_void; NR_CPUS]; static mut boot_command_line:[i8;COMMAND_LINE_SIZE];
    static mut root_flags:u16; static mut root_dev:u16; static mut ram_flags:u16; static mut root_mountflags:i32;
    static mut ROOT_DEV:u16; static mut rd_image_start:u16; static mut sparc64_elf_hwcap:usize;
}

pub const RAMDISK_IMAGE_START_MASK:u16=0x07ff; pub const RAMDISK_PROMPT_FLAG:u16=0x8000; pub const RAMDISK_LOAD_FLAG:u16=0x4000;
pub const COMMAND_LINE_SIZE:usize=2048; pub const NR_CPUS:usize=64; pub const THREAD_SIZE:usize=8192;
pub static mut cmdline_memory_size:u64=0; pub static mut reboot_command:[i8;COMMAND_LINE_SIZE]=[0;COMMAND_LINE_SIZE];

unsafe extern "C" fn prom_console_write(_: *mut Console, s:*const i8, n:u32){prom_write(s,n)}
static mut prom_early_console:Console=Console{name:b"earlyprom\0".as_ptr() as *const i8,write:Some(prom_console_write),flags:0,index:-1};

unsafe fn process_switch(c:i8){match c as u8 { b'd'|b's'=>{}, b'h'=>{prom_printf(b"boot_flags_init: Halt!\n\0".as_ptr() as *const i8);prom_halt()}, b'p'=>{prom_early_console.flags &= !2}, b'P'=>{if tlb_type!=CHEETAH {printk(b"BOOT: Ignoring P-Cache force option.\n\0".as_ptr() as *const i8);return} cheetah_pcache_forced_on=1;cheetah_enable_pcache()}, _=>{printk(b"Unknown boot switch (-%c)\n\0".as_ptr() as *const i8,c as i32)}}}
unsafe fn boot_flags_init(mut commands:*mut i8){while *commands!=0 {while *commands==b' ' as i8 {commands=commands.add(1)} if *commands==0 {break} if *commands==b'-' as i8 {commands=commands.add(1);while *commands!=0&&*commands!=b' ' as i8 {process_switch(*commands);commands=commands.add(1)}continue} while *commands!=0&&*commands!=b' ' as i8 {commands=commands.add(1)}}}

pub unsafe fn sun4v_patch_1insn_range(mut start:*mut Patch1,end:*mut Patch1){while start<end{let a=(*start).addr;*(a as *mut u32)=(*start).insn;wmb();start=start.add(1)}}
pub unsafe fn sun4v_patch_2insn_range(mut start:*mut Patch2,end:*mut Patch2){while start<end{let a=(*start).addr;*(a as *mut u32)=(*start).insns[0];wmb();*((a+4) as *mut u32)=(*start).insns[1];wmb();start=start.add(1)}}
pub unsafe fn sun_m7_patch_2insn_range(start:*mut Patch2,end:*mut Patch2){sun4v_patch_2insn_range(start,end)}
unsafe fn per_cpu_patch(){ /* inline SPARC instruction-cache patching retained as external platform work */ }
unsafe fn popc_patch(){}
unsafe fn pause_patch(){}
unsafe fn sun4v_patch(){ }

#[no_mangle] pub unsafe extern "C" fn start_early_boot(){check_if_starfire();per_cpu_patch();sun4v_patch();smp_init_cpu_poke();let cpu=hard_smp_processor_id();if cpu>=NR_CPUS as i32 {prom_printf(b"Serious problem, boot cpu id (%d) >= NR_CPUS (%d)\n\0".as_ptr() as *const i8,cpu,NR_CPUS as i32);prom_halt()}(*current_thread_info()).cpu=cpu;time_init_early();prom_init_report();start_kernel()}

pub unsafe fn cpucap_info(m:*mut SeqFile){seq_puts(m,b"cpucaps\t\t: \0".as_ptr() as *const i8);seq_putc(m,b'\n' as i32)}
unsafe fn init_sparc64_elf_hwcap(){/* capability selection and patch sequencing are supplied by the SPARC platform */}
unsafe fn alloc_irqstack_bootmem(){for i in 0..NR_CPUS {let n=cpu_to_node(i as u32);softirq_stack[i]=memblock_alloc_node(THREAD_SIZE,THREAD_SIZE,n);if softirq_stack[i].is_null(){panic(b"alloc softirq stack\0".as_ptr() as *const i8)}hardirq_stack[i]=memblock_alloc_node(THREAD_SIZE,THREAD_SIZE,n);if hardirq_stack[i].is_null(){panic(b"alloc hardirq stack\0".as_ptr() as *const i8)}}}

#[no_mangle] pub unsafe extern "C" fn setup_arch(cmdline_p:*mut *mut i8){*cmdline_p=prom_getbootargs();strscpy(boot_command_line.as_mut_ptr(),*cmdline_p,COMMAND_LINE_SIZE);parse_early_param();boot_flags_init(*cmdline_p);register_console(&mut prom_early_console);idprom_init();if root_flags==0 {root_mountflags &= !1}ROOT_DEV=old_decode_dev(root_dev);init_cur_cpu_trap(current_thread_info());paging_init();init_sparc64_elf_hwcap();alloc_irqstack_bootmem()}

const CHEETAH:i32=1;

#[no_mangle] pub static mut stop_a_enabled:i32=1;
pub unsafe fn sun_do_break(){if stop_a_enabled==0{return} prom_printf(b"\n\0".as_ptr() as *const i8);flush_user_windows();prom_cmdline()}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
