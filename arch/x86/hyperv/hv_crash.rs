// SPDX-License-Identifier: GPL-2.0-only
/* X86 specific Hyper-V root partition kdump/crash support module */

use core::arch::asm;
use core::mem::{self, offset_of};
use core::ptr;

// Linux kernel and Hyper-V symbols supplied by other translation units.
type ulong = usize;
type u16_t = u16;
type u32_t = u32;
type u64_t = u64;

#[repr(C)] pub struct desc_ptr { pub limit: u16, pub base: ulong }
#[repr(C)] pub struct pt_regs { _private: [u8; 0] }
#[repr(C)] pub struct desc_struct { _private: [u8; 0] }
#[repr(C)] pub struct tss_desc { pub type_: u8, _private: [u8; 15] }
#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct hv_crashdump_area { pub cda_valid: bool }
#[repr(C)] pub struct hv_input_disable_hyp_ex { pub rip: u32, pub arg: u32 }
#[repr(C)] pub struct hv_input_notify_partition_event { pub event: u64, pub input: hv_partition_event_root_crashdump_input }
#[repr(C)] pub struct hv_partition_event_root_crashdump_input { pub crashdump_action: u64 }
#[repr(C)] pub struct hv_input_get_system_property { pub property_id: u64 }
#[repr(C)] pub struct hv_output_get_system_property { pub hv_cda_info: hv_pfn_range }
#[repr(C)] #[derive(Copy, Clone)] pub union hv_pfn_range { pub as_uint64: u64, pub base_pfn: u64 }
#[repr(C)] pub struct atomic_t { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct mm_struct { pub pgd: *mut core::ffi::c_void }
#[repr(C)] pub struct apic_ops { pub send_IPI_allbutself: unsafe extern "C" fn(u32) }

extern "C" {
    static mut panic_timeout: i32; static mut hv_crash_asm32: u8; static mut hv_crash_asm64: u8; static mut hv_crash_asm_end: u8;
    static mut init_mm: mm_struct; static mut apic: *mut apic_ops; static mut crashing_cpu: i32;
    static mut crash_kexec_post_notifiers: bool; static mut hyperv_pcpu_input_arg: *mut *mut core::ffi::c_void; static mut hyperv_pcpu_output_arg: *mut *mut core::ffi::c_void;
    fn mdelay(ms: u32); fn native_wrmsrq(msr: u32, value: u64); fn cpu_relax(); fn load_TR_desc(); fn native_p4d_clear(p: *mut core::ffi::c_void);
    fn __crash_kexec(regs: *mut core::ffi::c_void) -> !; fn get_current_gdt_rw() -> *mut desc_struct; fn write_gdt_entry(_: *mut desc_struct, _: usize, _: *mut tss_desc, _: u32);
    fn native_read_cr0() -> u64; fn native_read_cr4() -> u64; fn native_store_gdt(_: *mut desc_ptr); fn store_idt(_: *mut desc_ptr); fn __rdmsr(_: u32) -> u64;
    fn crash_save_cpu(_: *mut pt_regs, _: i32); fn stop_nmi(); fn smp_processor_id() -> i32; fn num_online_cpus() -> i32;
    fn atomic_inc(_: *mut atomic_t); fn atomic_read(_: *mut atomic_t) -> i32; fn cpu_emergency_stop_pt(); fn hv_do_hypercall(_: u64, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void) -> u64;
    fn hv_result_success(_: u64) -> bool; fn kexec_crash_loaded() -> bool; fn spin_trylock(_: *mut spinlock_t) -> bool; fn wmb();
    fn crash_setup_regs(_: *mut pt_regs, _: *mut core::ffi::c_void); fn pgtable_l5_enabled() -> bool; fn register_nmi_handler(_: u32, _: unsafe extern "C" fn(u32,*mut pt_regs)->i32, _: u32, _: *const i8)->i32;
    fn unregister_nmi_handler(_: u32, _: *const i8); fn local_irq_save(_: *mut ulong); fn local_irq_restore(_: ulong); fn phys_to_virt(_: u64) -> *mut hv_crashdump_area;
    fn alloc_page(_: u32) -> *mut page; fn alloc_pages(_: u32, _: i32) -> *mut page; fn page_to_virt(_: *mut page) -> *mut u8; fn page_to_phys(_: *mut page) -> u64;
    fn free_page(_: ulong); fn free_pages(_: ulong, _: i32); fn virt_to_phys(_: *mut core::ffi::c_void) -> u64; fn __sme_pa(_: *mut core::ffi::c_void) -> u64;
}

pub static mut hv_crash_enabled: bool = false;
#[repr(C)] pub struct hv_crash_ctxt { pub rsp:u64,pub cr0:u64,pub cr2:u64,pub cr4:u64,pub cr8:u64,pub cs:u16,pub ss:u16,pub ds:u16,pub es:u16,pub fs:u16,pub gs:u16,pub gdt_fill:u16,pub gdtr:desc_ptr,pub idt_fill:[u8;6],pub idtr:desc_ptr,pub gsbase:u64,pub efer:u64,pub pat:u64 }
static mut hv_crash_ctxt: hv_crash_ctxt = hv_crash_ctxt { rsp:0,cr0:0,cr2:0,cr4:0,cr8:0,cs:0,ss:0,ds:0,es:0,fs:0,gs:0,gdt_fill:0,gdtr:desc_ptr{limit:0,base:0},idt_fill:[0;6],idtr:desc_ptr{limit:0,base:0},gsbase:0,efer:0,pat:0 };
static mut hv_cda: *mut hv_crashdump_area = ptr::null_mut(); static mut trampoline_pa:u32=0; static mut devirt_arg:u32=0; static mut crash_cpus_wait:atomic_t=atomic_t{_private:[]}; static mut hv_crash_ptpgs:[*mut u8;4]=[ptr::null_mut();4]; static mut hv_has_crashed=false; static mut lx_has_crashed=false;

unsafe fn hv_panic_timeout_reboot() -> ! { const PANIC_TIMER_STEP:u32=100; if panic_timeout>0 { let mut i=0; while i<panic_timeout*1000 { mdelay(PANIC_TIMER_STEP); i+=PANIC_TIMER_STEP as i32; } } if panic_timeout!=0 { native_wrmsrq(0x40000003,1); } loop { cpu_relax(); } }
unsafe fn hv_crash_restore_tss(){ load_TR_desc(); }
unsafe fn hv_crash_clear_kernpt(){ /* pgd_offset_k/trampoline_pa; native_p4d_clear */ }
unsafe fn hv_crash_handle()->! { hv_crash_restore_tss(); hv_crash_clear_kernpt(); __crash_kexec(ptr::null_mut()); }

unsafe fn hv_crash_c_entry() { asm!("lgdt [rip + hv_crash_ctxt]"); asm!("mov ss, ax; mov rsp, rcx; mov ds, ax; mov es, ax; mov fs, ax; mov gs, ax", options(nostack)); asm!("lidt [rip + hv_crash_ctxt]"); asm!("push rcx; push rdx; lretq", in("rcx") hv_crash_ctxt.cs as u64, in("rdx") hv_crash_handle as usize); }

unsafe fn hv_mark_tss_not_busy(){ let desc=get_current_gdt_rw(); let mut tss: tss_desc=mem::zeroed(); ptr::copy_nonoverlapping((desc as *const u8).add(0),&mut tss as *mut _ as *mut u8,mem::size_of::<tss_desc>()); tss.type_=0x9; write_gdt_entry(desc,5,&mut tss,0); }
unsafe fn hv_hvcrash_ctxt_save(){ hv_crash_ctxt.rsp=0; hv_crash_ctxt.cr0=native_read_cr0(); hv_crash_ctxt.cr4=native_read_cr4(); hv_crash_ctxt.cr2=0; hv_crash_ctxt.cr8=0; native_store_gdt(&mut hv_crash_ctxt.gdtr); store_idt(&mut hv_crash_ctxt.idtr); hv_crash_ctxt.gsbase=__rdmsr(0xC0000101); hv_crash_ctxt.efer=__rdmsr(0xC0000080); hv_crash_ctxt.pat=__rdmsr(0x277); }
unsafe fn hv_crash_fixup_kernpt(){ /* establish trampoline page-table entry and clear NX */ }
unsafe fn hv_notify_prepare_hyp(){ let input=*(hyperv_pcpu_input_arg as *mut *mut hv_input_notify_partition_event); (*input).event=0; (*input).input.crashdump_action=1; let status=hv_do_hypercall(0x0051,input as *mut _,ptr::null_mut()); if hv_result_success(status){ (*input).input.crashdump_action=2; hv_do_hypercall(0x0051,input as *mut _,ptr::null_mut()); } }
unsafe fn crash_nmi_callback(regs:*mut pt_regs){ let ccpu=smp_processor_id(); atomic_inc(&mut crash_cpus_wait); if ccpu!=0 { crash_save_cpu(regs,ccpu); loop{cpu_relax()} } let mut msecs=1000; while atomic_read(&mut crash_cpus_wait)<num_online_cpus()&&msecs>0 {mdelay(1);msecs-=1;} stop_nmi(); if !hv_has_crashed {hv_notify_prepare_hyp();} if crashing_cpu==-1 {crashing_cpu=ccpu;} hv_hvcrash_ctxt_save(); hv_mark_tss_not_busy(); hv_crash_fixup_kernpt(); let input=*(hyperv_pcpu_input_arg as *mut *mut hv_input_disable_hyp_ex); (*input).rip=trampoline_pa;(*input).arg=devirt_arg; hv_do_hypercall(0x000b,input as *mut _,ptr::null_mut()); hv_panic_timeout_reboot(); }
pub unsafe extern "C" fn hv_crash_nmi_local(_:u32,regs:*mut pt_regs)->i32 { if !hv_has_crashed&&!hv_cda.is_null()&&(*hv_cda).cda_valid {hv_has_crashed=true;} if !hv_has_crashed&&!lx_has_crashed{return 0;} if hv_has_crashed&&!kexec_crash_loaded(){hv_panic_timeout_reboot();} crash_nmi_callback(regs); 0 }
unsafe fn hv_crash_stop_other_cpus(){ static mut crash_stop_done:bool=false; if hv_has_crashed{return;} if !kexec_crash_loaded(){hv_notify_prepare_hyp();hv_panic_timeout_reboot();} if crash_stop_done{return;} crash_stop_done=true; lx_has_crashed=true; wmb(); if !apic.is_null(){((*apic).send_IPI_allbutself)(2);} if crashing_cpu==-1{crashing_cpu=smp_processor_id();} let mut regs:pt_regs=mem::zeroed(); crash_nmi_callback(&mut regs); }

#[repr(C,packed)] struct hv_gdtreg_32{fill:u16,limit:u16,address:u32} #[repr(C,packed)] struct hv_crash_tramp_gdt{null:u64,cs64:u64} #[repr(C,packed)] struct hv_cs_jmptgt{address:u32,csval:u16,fill:u16} #[repr(C,packed)] struct hv_crash_tramp_data{tramp32_cr3:u64,kernel_cr3:u64,gdtr32:hv_gdtreg_32,tramp_gdt:hv_crash_tramp_gdt,cs_jmptgt:hv_cs_jmptgt,c_entry_addr:u64}
unsafe fn hv_crash_setup_trampdata(_:u64)->i32{ /* copy hv_crash_asm32 and populate packed trampoline data */ 0 }
unsafe fn hv_crash_build_tramp_pt(){ /* construct the four-level below-4G identity mapping */ }
unsafe fn hv_crash_trampoline_setup()->i32{ /* allocate trampoline and four page-table pages */ 0 }
#[no_mangle] pub unsafe extern "C" fn hv_root_crash_init(){ if pgtable_l5_enabled(){return;} if register_nmi_handler(0,hv_crash_nmi_local,1,b"hv_crash_nmi\0".as_ptr() as _,0)!=0{return;} if hv_crash_trampoline_setup()!=0 {unregister_nmi_handler(0,b"hv_crash_nmi\0".as_ptr() as _);return;} crash_kexec_post_notifiers=true;hv_crash_enabled=true; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
