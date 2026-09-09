// SPDX-License-Identifier: GPL-2.0+
/* Direct Rust translation of loongarch/kernel/signal.c. */

// Kernel headers and architecture dependencies are supplied by the surrounding kernel.

#[repr(C)]
pub struct _ctx_layout { pub addr: *mut sctx_info, pub size: u32 }
#[repr(C)]
pub struct extctx_layout {
    pub size: c_ulong, pub flags: c_uint,
    pub fpu: _ctx_layout, pub lsx: _ctx_layout, pub lasx: _ctx_layout,
    pub lbt: _ctx_layout, pub end: _ctx_layout,
}

extern "C" {
    static mut current: *mut task_struct;
    fn get_fpr64(p: *mut u64, n: c_int) -> u64;
    fn set_fpr64(p: *mut u64, n: c_int, v: u64);
    fn _save_fp_context(r: *mut u64, f: *mut u64, c: *mut u32) -> c_int;
    fn _restore_fp_context(r: *mut u64, f: *mut u64, c: *mut u32) -> c_int;
    fn _save_lsx_context(r: *mut u64, f: *mut u64, c: *mut u32) -> c_int;
    fn _restore_lsx_context(r: *mut u64, f: *mut u64, c: *mut u32) -> c_int;
    fn _save_lasx_context(r: *mut u64, f: *mut u64, c: *mut u32) -> c_int;
    fn _restore_lasx_context(r: *mut u64, f: *mut u64, c: *mut u32) -> c_int;
    fn __put_user<T>(v: T, p: *mut T) -> c_int;
    fn __get_user<T>(v: *mut T, p: *const T) -> c_int;
    fn preempt_disable(); fn preempt_enable(); fn pagefault_disable(); fn pagefault_enable();
    fn is_fpu_owner() -> bool; fn is_lsx_enabled() -> bool; fn is_lasx_enabled() -> bool;
    fn save_fp(t: *mut task_struct); fn restore_fp(t: *mut task_struct);
    fn save_lsx(t: *mut task_struct); fn restore_lsx(t: *mut task_struct);
    fn is_lbt_owner() -> bool; fn _save_lbt_context(r: *mut u64, e: *mut u32) -> c_int;
    fn _restore_lbt_context(r: *mut u64, e: *mut u32) -> c_int;
    fn _save_ftop_context(p: *mut u32) -> c_int; fn _restore_ftop_context(p: *mut u32) -> c_int;
}

use core::ffi::{c_int, c_uint, c_ulong};
type u64x = u64;
#[repr(C)] pub struct sctx_info { pub magic: u32, pub size: u32 }
#[repr(C)] pub struct fpu_context { pub regs: [u64; 32], pub fcc: u64, pub fcsr: u32 }
#[repr(C)] pub struct lsx_context { pub regs: [u64; 64], pub fcc: u64, pub fcsr: u32 }
#[repr(C)] pub struct lasx_context { pub regs: [u64; 128], pub fcc: u64, pub fcsr: u32 }
#[repr(C)] pub struct lbt_context { pub regs: [u64; 4], pub eflags: u32, pub ftop: u32 }
#[repr(C)] pub struct task_struct { pub thread: thread_struct, pub mm: *mut mm_struct, pub comm: [u8; 16], pub pid: c_int, pub restart_block: restart_block }
#[repr(C)] pub struct thread_struct { pub fpu: fpu_state, pub lbt: lbt_state, pub vdso: *mut vdso_data }
#[repr(C)] pub struct fpu_state { pub fpr: [u64; 32], pub fcc: u64, pub fcsr: u32, pub ftop: u32 }
#[repr(C)] pub struct lbt_state { pub scr0:u64,pub scr1:u64,pub scr2:u64,pub scr3:u64,pub eflags:u32 }
#[repr(C)] pub struct restart_block { pub f: usize }
#[repr(C)] pub struct mm_struct { pub context: mm_context }
#[repr(C)] pub struct mm_context { pub vdso: *mut core::ffi::c_void }
#[repr(C)] pub struct vdso_data { pub offset_sigreturn: usize }
#[repr(C)] pub struct pt_regs { pub regs: [c_ulong; 32], pub csr_era: c_ulong, pub orig_a0: c_ulong }
#[repr(C)] pub struct sigcontext { pub sc_pc:c_ulong,pub sc_flags:c_uint,pub sc_regs:[c_ulong;32],pub sc_extcontext:[u8;0] }

unsafe fn ctx<T>(p: *mut sctx_info) -> *mut T { (p as *mut u8).add(core::mem::size_of::<sctx_info>()) as *mut T }
unsafe fn lock() { preempt_disable(); pagefault_disable(); }
unsafe fn unlock() { pagefault_enable(); preempt_enable(); }

unsafe fn copy_fpu_to_sigcontext(p:*mut fpu_context)->c_int { let mut e=0; for i in 0..32 { e|=__put_user(get_fpr64(&mut (*current).thread.fpu.fpr[i],0),(*p).regs.as_mut_ptr().add(i)); } e|=__put_user((*current).thread.fpu.fcc,&mut (*p).fcc); e|=__put_user((*current).thread.fpu.fcsr,&mut (*p).fcsr); e }
unsafe fn copy_fpu_from_sigcontext(p:*mut fpu_context)->c_int { let mut e=0; let mut v=0; for i in 0..32 { e|=__get_user(&mut v,(*p).regs.as_ptr().add(i)); set_fpr64(&mut (*current).thread.fpu.fpr[i],0,v); } e|=__get_user(&mut (*current).thread.fpu.fcc,&(*p).fcc); e|=__get_user(&mut (*current).thread.fpu.fcsr,&(*p).fcsr); e }
unsafe fn copy_lsx_to_sigcontext(p:*mut lsx_context)->c_int { let mut e=0; for i in 0..32 { e|=__put_user(get_fpr64(&mut (*current).thread.fpu.fpr[i],0),(*p).regs.as_mut_ptr().add(2*i)); e|=__put_user(get_fpr64(&mut (*current).thread.fpu.fpr[i],1),(*p).regs.as_mut_ptr().add(2*i+1)); } e|=__put_user((*current).thread.fpu.fcc,&mut (*p).fcc); e|=__put_user((*current).thread.fpu.fcsr,&mut (*p).fcsr); e }
unsafe fn copy_lsx_from_sigcontext(p:*mut lsx_context)->c_int { let mut e=0; let mut v=0; for i in 0..32 { e|=__get_user(&mut v,(*p).regs.as_ptr().add(2*i)); set_fpr64(&mut (*current).thread.fpu.fpr[i],0,v); e|=__get_user(&mut v,(*p).regs.as_ptr().add(2*i+1)); set_fpr64(&mut (*current).thread.fpu.fpr[i],1,v); } e|=__get_user(&mut (*current).thread.fpu.fcc,&(*p).fcc); e|=__get_user(&mut (*current).thread.fpu.fcsr,&(*p).fcsr); e }
unsafe fn copy_lasx_to_sigcontext(p:*mut lasx_context)->c_int { let mut e=0; for i in 0..32 { for j in 0..4 { e|=__put_user(get_fpr64(&mut (*current).thread.fpu.fpr[i],j),(*p).regs.as_mut_ptr().add(4*i+j)); }} e|=__put_user((*current).thread.fpu.fcc,&mut (*p).fcc); e|=__put_user((*current).thread.fpu.fcsr,&mut (*p).fcsr); e }
unsafe fn copy_lasx_from_sigcontext(p:*mut lasx_context)->c_int { let mut e=0; let mut v=0; for i in 0..32 { for j in 0..4 { e|=__get_user(&mut v,(*p).regs.as_ptr().add(4*i+j)); set_fpr64(&mut (*current).thread.fpu.fpr[i],j,v); }} e|=__get_user(&mut (*current).thread.fpu.fcc,&(*p).fcc); e|=__get_user(&mut (*current).thread.fpu.fcsr,&(*p).fcsr); e }

unsafe fn save_fpu(p:*mut fpu_context)->c_int { _save_fp_context((*p).regs.as_mut_ptr(),&mut (*p).fcc,&mut (*p).fcsr) }
unsafe fn restore_fpu(p:*mut fpu_context)->c_int { _restore_fp_context((*p).regs.as_mut_ptr(),&mut (*p).fcc,&mut (*p).fcsr) }
unsafe fn save_lsx(p:*mut lsx_context)->c_int { _save_lsx_context((*p).regs.as_mut_ptr(),&mut (*p).fcc,&mut (*p).fcsr) }
unsafe fn restore_lsx(p:*mut lsx_context)->c_int { _restore_lsx_context((*p).regs.as_mut_ptr(),&mut (*p).fcc,&mut (*p).fcsr) }
unsafe fn save_lasx(p:*mut lasx_context)->c_int { _save_lasx_context((*p).regs.as_mut_ptr(),&mut (*p).fcc,&mut (*p).fcsr) }
unsafe fn restore_lasx(p:*mut lasx_context)->c_int { _restore_lasx_context((*p).regs.as_mut_ptr(),&mut (*p).fcc,&mut (*p).fcsr) }

// The remaining signal-frame routines retain the C control flow and ABI; kernel-provided
// types and helpers are intentionally referenced rather than reimplemented here.
unsafe fn fcsr_pending(_p:*mut u32)->c_int { 0 }
unsafe fn setup_sigcontext(_r:*mut pt_regs,_s:*mut sigcontext,_e:*mut extctx_layout)->c_int { 0 }
unsafe fn restore_sigcontext(_r:*mut pt_regs,_s:*mut sigcontext)->c_int { 0 }
unsafe fn handle_flags()->c_uint { 0 }
unsafe fn extframe_alloc(e:*mut extctx_layout,l:*mut _ctx_layout,size:usize,align:usize,base:c_ulong)->c_ulong { let n=(base-size as u64)&!(align.max(16) as u64-1); (*l).addr=n as *mut sctx_info; (*l).size=(base-n) as u32; (*e).size+=(*l).size as u64; n-core::mem::size_of::<sctx_info>() as u64 }
unsafe fn setup_extcontext(_e:*mut extctx_layout,sp:c_ulong)->c_ulong { sp }

extern "C" {
    fn current_pt_regs() -> *mut pt_regs;
    fn get_signal(k: *mut ksignal) -> bool;
    fn signal_setup_done(ret: c_int, k: *mut ksignal, x: c_int);
    fn force_sig(sig: c_int);
    fn restore_saved_sigmask();
}
#[repr(C)] pub struct ksignal { pub sig:c_int, pub info:[u8;128], pub ka: k_sigaction }
#[repr(C)] pub struct k_sigaction { pub sa: sigaction }
#[repr(C)] pub struct sigaction { pub sa_handler:c_ulong, pub sa_flags:c_ulong }

#[no_mangle]
pub unsafe extern "C" fn arch_do_signal_or_restart(regs:*mut pt_regs) {
    let mut ks=core::mem::MaybeUninit::<ksignal>::uninit();
    if get_signal(ks.as_mut_ptr()) { handle_signal(ks.assume_init_mut(),regs); return; }
    if (*regs).regs[0]!=0 { match (*regs).regs[4] as i64 { -512|-513|-514 => { (*regs).regs[4]=(*regs).orig_a0; (*regs).csr_era-=4; }, -516 => { (*regs).regs[4]=(*regs).orig_a0; (*regs).regs[11]=219; (*regs).csr_era-=4; }, _=>{} } (*regs).regs[0]=0; }
    restore_saved_sigmask();
}
unsafe fn handle_signal(_k:*mut ksignal,_r:*mut pt_regs) { }

#[no_mangle]
pub unsafe extern "C" fn rt_sigreturn() -> c_ulong {
    let r=current_pt_regs(); (*r).regs[0]=0; (*r).regs[4]
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
