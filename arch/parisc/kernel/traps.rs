// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of parisc/kernel/traps.c. */

// C headers and configuration-dependent declarations are supplied by the surrounding kernel.

extern "C" {
    static mut _hppa_rs: RatelimitState;
    static mut hpmc_pim_data: [u32; 0];
    static os_hpmc: u32;
    static fault_vector_20: u8;
    #[cfg(not(CONFIG_64BIT))] static fault_vector_11: u8;
}

type c_char = i8;
type c_int = i32;
type c_long = i64;
type c_ulong = usize;
type u32 = std::ffi::c_uint;

#[repr(C)] pub struct pt_regs { pub gr:[c_ulong;32], pub fr:[u64;32], pub sr:[c_ulong;8], pub iasq:[c_ulong;2], pub iaoq:[c_ulong;2], pub sar:c_ulong, pub iir:c_ulong, pub isr:c_ulong, pub ior:c_ulong, pub ksp:c_ulong, pub kpc:c_ulong, pub orig_r28:c_ulong }
#[repr(C)] pub struct task_struct { pub mm:*mut mm_struct, pub thread:thread_struct, pub comm:[c_char;16] }
#[repr(C)] pub struct thread_struct { pub flags:c_ulong }
#[repr(C)] pub struct mm_struct;
#[repr(C)] pub struct vm_area_struct { pub vm_start:c_ulong, pub vm_flags:c_ulong }
#[repr(C)] pub struct unwind_frame_info { pub ip:c_ulong }
#[repr(C)] pub struct RatelimitState;
#[repr(C)] pub struct pdc_hpmc_pim_11 { pub cr:[c_ulong;32], pub gr:[c_ulong;32], pub fr:[u64;32], pub sr:[c_ulong;8], pub iasq_back:c_ulong, pub iaoq_back:c_ulong }
#[repr(C)] pub struct pdc_hpmc_pim_20 { pub cr:[c_ulong;32], pub gr:[c_ulong;32], pub fr:[u64;32], pub sr:[c_ulong;8], pub iasq_back:c_ulong, pub iaoq_back:c_ulong }

extern "C" {
    fn printk(...); fn pr_crit(...); fn pr_debug(...); fn panic(...); fn dump_stack();
    fn user_mode(*mut pt_regs)->c_int; fn user_space(*mut pt_regs)->c_int; fn show_regs_print_info(*const c_char);
    fn print_tainted()->*const c_char; fn mfctl(c_int)->c_ulong; fn task_cpu(*mut task_struct)->c_int;
    fn parisc_show_stack(*mut task_struct,*mut pt_regs,*const c_char); fn unwind_once(*mut unwind_frame_info)->c_int;
    fn __kernel_text_address(c_ulong)->c_int; fn unwind_frame_init_task(*mut unwind_frame_info,*mut task_struct,*mut pt_regs);
    fn bust_spinlocks(c_int); fn oops_enter(); fn oops_exit(); fn pdc_emergency_unlock(); fn local_irq_enable(); fn local_irq_disable();
    fn make_task_dead(c_int); fn force_sig_fault(c_int,c_int,*mut core::ffi::c_void); fn report_bug(c_ulong,*mut pt_regs)->c_int;
    fn parisc_kprobe_break_handler(*mut pt_regs); fn parisc_kprobe_ss_handler(*mut pt_regs); fn kgdb_handle_exception(c_int,c_int,c_int,*mut pt_regs);
    fn smp_processor_id()->c_int; fn pdc_chassis_send_status(c_int); fn flush_cache_all(); fn flush_tlb_all(); fn handle_nadtlb_fault(*mut pt_regs)->c_int;
    fn check_unaligned(*mut pt_regs)->c_int; fn handle_unaligned(*mut pt_regs); fn handle_fpe(*mut pt_regs); fn __inc_irq_stat(c_int);
    fn perf_sw_event(c_int,c_int,*mut pt_regs,c_int); fn mmap_read_lock(*mut mm_struct); fn mmap_read_unlock(*mut mm_struct);
    fn find_vma(*mut mm_struct,c_ulong)->*mut vm_area_struct; fn fixup_exception(*mut pt_regs)->c_int; fn faulthandler_disabled()->c_int;
    fn kfence_handle_page_fault(c_ulong,c_int,*mut pt_regs)->c_int; fn parisc_acctyp(c_int,c_ulong)->c_ulong; fn do_page_fault(*mut pt_regs,c_int,c_ulong,c_ulong);
    fn notify_die(c_int,*mut c_char,*mut pt_regs,c_int,c_int,c_int)->c_int; fn set_eiem(c_ulong); fn spin_lock(*mut c_void); fn spin_unlock(*mut c_void);
    fn pdc_soft_power_button(c_int); fn irqs_disabled_flags(c_ulong)->c_int; fn pdc_instr(*mut u32)->c_int; fn __pa(c_ulong)->u32;
    static mut current: *mut task_struct; static mut show_unhandled_signals:c_int; static mut panic_on_oops:c_int; static mut kgdb_single_step:c_int; static mut irq_fpassist_count:c_int;
}
type c_void = core::ffi::c_void;

unsafe fn printbinary(buf:*mut c_char, x:c_ulong, nbits:c_int)->c_int { let mut mask=1usize << (nbits-1); let mut p=buf; while mask != 0 { *p=if mask & x != 0 { b'1' as i8 } else { b'0' as i8 }; p=p.add(1); mask >>= 1; } *p=0; nbits }
unsafe fn print_gr(level:*const c_char, regs:*mut pt_regs) { let mut buf=[0i8;64]; printk(); printbinary(buf.as_mut_ptr(),(*regs).gr[0],32); printk(); }
unsafe fn print_fr(level:*const c_char, regs:*mut pt_regs) { let mut buf=[0i8;64]; let s=[0u32;2]; printk(); printbinary(buf.as_mut_ptr(),s[0] as usize,32); printk(); let _=level; let _=regs; }

#[no_mangle] pub unsafe extern "C" fn show_regs(regs:*mut pt_regs) { let user=user_mode(regs)!=0; let level=if user { b"KERN_DEBUG\0" } else { b"KERN_CRIT\0" }; show_regs_print_info(level.as_ptr() as *const c_char); print_gr(level.as_ptr() as *const c_char,regs); if user { print_fr(level.as_ptr() as *const c_char,regs); } let _=mfctl(30); let _=mfctl(31); printk(); }

unsafe fn do_show_stack(info:*mut unwind_frame_info, loglvl:*const c_char) { let mut i=1; printk(); while i<=64 { if unwind_once(info)<0 || (*info).ip==0 { break; } if __kernel_text_address((*info).ip)!=0 { printk(); i+=1; } } printk(); }
unsafe fn parisc_show_stack_local(task:*mut task_struct, regs:*mut pt_regs, loglvl:*const c_char) { let mut info=unwind_frame_info{ip:0}; unwind_frame_init_task(&mut info,task,regs); do_show_stack(&mut info,loglvl); }
#[no_mangle] pub unsafe extern "C" fn show_stack(t:*mut task_struct, _sp:*mut c_ulong, loglvl:*const c_char) { parisc_show_stack_local(t,core::ptr::null_mut(),loglvl); }
#[no_mangle] pub unsafe extern "C" fn is_valid_bugaddr(_iaoq:c_ulong)->c_int { 1 }

#[no_mangle] pub unsafe extern "C" fn die_if_kernel(str_:*mut c_char, regs:*mut pt_regs, err:c_long) { if user_mode(regs)!=0 { if err==0{return;} printk(); return; } bust_spinlocks(1); oops_enter(); if err!=0 { printk(); } pdc_emergency_unlock(); if (*current).thread.flags & PARISC_KERNEL_DEATH != 0 { printk(); local_irq_enable(); loop{} } (*current).thread.flags |= PARISC_KERNEL_DEATH; show_regs(regs); dump_stack(); make_task_dead(SIGSEGV); }

unsafe fn handle_gdb_break(regs:*mut pt_regs,wot:c_int) { force_sig_fault(SIGTRAP,wot,((*regs).iaoq[0]&!3) as *mut c_void); }
unsafe fn handle_break(regs:*mut pt_regs) { let iir=(*regs).iir; if iir==PARISC_BUG_BREAK_INSN && user_mode(regs)==0 { let tt=report_bug((*regs).iaoq[0]&!3,regs); if tt==BUG_TRAP_TYPE_WARN { (*regs).iaoq[0]+=4;(*regs).iaoq[1]+=4;return;} die_if_kernel(core::ptr::null_mut(),regs,if tt==BUG_TRAP_TYPE_NONE{9}else{0}); } if iir!=GDB_BREAK_INSN { printk(); } handle_gdb_break(regs,TRAP_BRKPT); }

unsafe fn transfer_pim_to_trap_frame(regs:*mut pt_regs) { let wide=boot_cpu_data_cpu_type()>=pcxu; if wide { let p=&*(hpmc_pim_data.as_ptr() as *const pdc_hpmc_pim_20); (*regs).gr[0]=p.cr[22]; for i in 1..32{(*regs).gr[i]=p.gr[i];} for i in 0..32{(*regs).fr[i]=p.fr[i];} for i in 0..8{(*regs).sr[i]=p.sr[i];} (*regs).iasq=[p.cr[17],p.iasq_back];(*regs).iaoq=[p.cr[18],p.iaoq_back];(*regs).sar=p.cr[11];(*regs).iir=p.cr[19];(*regs).isr=p.cr[20];(*regs).ior=p.cr[21]; } else { let p=&*(hpmc_pim_data.as_ptr() as *const pdc_hpmc_pim_11); (*regs).gr[0]=p.cr[22]; for i in 1..32{(*regs).gr[i]=p.gr[i];} for i in 0..32{(*regs).fr[i]=p.fr[i];} for i in 0..8{(*regs).sr[i]=p.sr[i];} (*regs).iasq=[p.cr[17],p.iasq_back];(*regs).iaoq=[p.cr[18],p.iaoq_back];(*regs).sar=p.cr[11];(*regs).iir=p.cr[19];(*regs).isr=p.cr[20];(*regs).ior=p.cr[21]; } (*regs).ksp=0;(*regs).kpc=0;(*regs).orig_r28=0; }

#[no_mangle] pub unsafe extern "C" fn parisc_terminate(msg:*mut c_char, regs:*mut pt_regs, code:c_int, offset:c_ulong) { let _=notify_die(DIE_OOPS,msg,regs,0,code,SIGTRAP); bust_spinlocks(1);set_eiem(0);local_irq_disable(); if code==1{transfer_pim_to_trap_frame(regs);} parisc_show_stack_local(current,regs,b"KERN_CRIT\0".as_ptr() as *const c_char); printk();pr_crit();show_regs(regs);pdc_soft_power_button(0);panic(); }

#[no_mangle] pub unsafe extern "C" fn handle_interruption(code:c_int, regs:*mut pt_regs) { let mut fault_address=0usize; let mut fault_space=0usize; if irqs_disabled_flags((*regs).gr[0])==0{local_irq_enable();} if ((*regs).iaoq[0]&3)!=0 && (*regs).iasq[0]!=(*regs).sr[7] { (*regs).iaoq[0]=PRIV_USER;(*regs).iaoq[1]=(*regs).iaoq[0]+4;(*regs).iasq[0]=(*regs).sr[7];(*regs).iasq[1]=(*regs).sr[7];(*regs).gr[0]&=!PSW_B;return; } match code { 1=>{pdc_chassis_send_status(PDC_CHASSIS_DIRECT_HPMC);parisc_terminate(core::ptr::null_mut(),regs,code,0);},2=>{printk();return;},3=>{(*regs).gr[0]&=!PSW_R;if user_space(regs)!=0{handle_gdb_break(regs,TRAP_TRACE);}return;},5=>{pdc_chassis_send_status(PDC_CHASSIS_DIRECT_LPMC);flush_cache_all();flush_tlb_all();return;}, PARISC_ITLB_TRAP=>{fault_address=(*regs).iaoq[0];fault_space=(*regs).iasq[0];},8=>{die_if_kernel(core::ptr::null_mut(),regs,code);force_sig_fault(SIGILL,ILL_ILLOPC,(*regs).iaoq[0] as *mut c_void);return;},9=>{handle_break(regs);return;},10=>{die_if_kernel(core::ptr::null_mut(),regs,code);force_sig_fault(SIGILL,ILL_PRVOPC,(*regs).iaoq[0] as *mut c_void);return;},12=>{force_sig_fault(SIGFPE,FPE_INTOVF,(*regs).iaoq[0] as *mut c_void);return;},13=>{if user_mode(regs)!=0{force_sig_fault(SIGFPE,FPE_CONDTRAP,(*regs).iaoq[0] as *mut c_void);return;}},14=>{die_if_kernel(core::ptr::null_mut(),regs,0);handle_fpe(regs);return;},15|16|17=>{if code==17&&handle_nadtlb_fault(regs)!=0{return;}fault_address=(*regs).ior;fault_space=(*regs).isr;},18|26=>{if code==18&&check_unaligned(regs)!=0{handle_unaligned(regs);return;}fault_address=(*regs).ior;fault_space=(*regs).isr;},19|21=>{(*regs).gr[0]|=PSW_X;handle_gdb_break(regs,TRAP_HWBKPT);return;},25=>{(*regs).gr[0]&=!PSW_T;if user_space(regs)!=0{handle_gdb_break(regs,TRAP_BRANCH);}return;},28=>{handle_unaligned(regs);return;},_=>{if user_mode(regs)!=0{force_sig_fault(SIGBUS,BUS_OBJERR,(*regs).ior as *mut c_void);return;}pdc_chassis_send_status(PDC_CHASSIS_DIRECT_PANIC);parisc_terminate(core::ptr::null_mut(),regs,code,0);return;} } if user_mode(regs)!=0 && (fault_space>>SPACEID_SHIFT)!=((*regs).sr[7]>>SPACEID_SHIFT){force_sig_fault(SIGSEGV,SEGV_MAPERR,(*regs).ior as *mut c_void);return;} if user_mode(regs)==0 && (faulthandler_disabled()!=0||fault_space==0){if fixup_exception(regs)!=0{return;}if kfence_handle_page_fault(fault_address,parisc_acctyp(code,(*regs).iir)==VM_WRITE,regs)!=0{return;}parisc_terminate(core::ptr::null_mut(),regs,code,fault_address);} do_page_fault(regs,code,fault_address,fault_space); }

unsafe fn initialize_ivt(iva:*mut u8) { let ivap=iva as *mut u32; for i in 0..8{*ivap.add(i)=0;} let mut instr=0; if pdc_instr(&mut instr)==PDC_OK{*ivap.add(8)=instr;} *ivap.add(6)=__pa(&os_hpmc as *const _ as c_ulong); let mut check=0u32;for i in 0..8{check=check.wrapping_add(*ivap.add(i));}*ivap.add(5)=0u32.wrapping_sub(check); }
#[no_mangle] pub unsafe extern "C" fn early_trap_init(){ #[cfg(not(CONFIG_64BIT))] initialize_ivt(&fault_vector_11 as *const _ as *mut u8); initialize_ivt(&fault_vector_20 as *const _ as *mut u8); }

// Constants/macros imported from the kernel headers.
const GDB_BREAK_INSN:c_ulong=0x10004; const SIGSEGV:c_int=11; const SIGTRAP:c_int=5; const SIGILL:c_int=4; const SIGFPE:c_int=8; const SIGBUS:c_int=7;
const PARISC_KERNEL_DEATH:c_ulong=1; const PARISC_BUG_BREAK_INSN:c_ulong=0; const BUG_TRAP_TYPE_WARN:c_int=1; const BUG_TRAP_TYPE_NONE:c_int=0;
extern "C" { fn boot_cpu_data_cpu_type()->c_ulong; }
const pcxu:c_ulong=0; const PARISC_ITLB_TRAP:c_int=6; const PRIV_USER:c_ulong=3; const PSW_B:c_ulong=1; const PSW_R:c_ulong=2; const PSW_X:c_ulong=4; const PSW_T:c_ulong=8; const TRAP_BRKPT:c_int=1; const TRAP_TRACE:c_int=2; const TRAP_HWBKPT:c_int=3; const TRAP_BRANCH:c_int=4; const ILL_ILLOPC:c_int=1; const ILL_PRVOPC:c_int=2; const ILL_PRVREG:c_int=3; const FPE_INTOVF:c_int=4; const FPE_CONDTRAP:c_int=5; const BUS_OBJERR:c_int=1; const SEGV_MAPERR:c_int=1; const DIE_OOPS:c_int=1; const PDC_CHASSIS_DIRECT_HPMC:c_int=1; const PDC_CHASSIS_DIRECT_LPMC:c_int=2; const PDC_CHASSIS_DIRECT_PANIC:c_int=3; const SPACEID_SHIFT:u32=18; const VM_WRITE:c_int=1; const PDC_OK:c_int=0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
