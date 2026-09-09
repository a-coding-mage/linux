// SPDX-License-Identifier: GPL-2.0
/* Faithful low-level Rust translation of vm86_32.c.  Kernel dependencies are
 * supplied by the surrounding kernel translation unit. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

const SAFE_MASK: u32 = 0xDD5;
const RETURN_MASK: u32 = 0xDFF;

macro_rules! set_flags { ($x:expr, $new:expr, $mask:expr) => { $x = ($x & !($mask)) | ($new & ($mask)); }; }
macro_rules! AL { ($r:expr) => { unsafe { *((&mut (*$r).pt.ax as *mut _ as *mut u8).add(0)) } }; }
macro_rules! AH { ($r:expr) => { unsafe { *((&mut (*$r).pt.ax as *mut _ as *mut u8).add(1)) } }; }
macro_rules! IP { ($r:expr) => { unsafe { *( &mut (*$r).pt.ip as *mut _ as *mut u16) } }; }
macro_rules! SP { ($r:expr) => { unsafe { *( &mut (*$r).pt.sp as *mut _ as *mut u16) } }; }

/* External kernel declarations intentionally remain unresolved here. */
extern "C" {
    fn local_irq_enable();
    fn force_exit_sig(sig: i32);
    fn force_sig(sig: i32);
    fn security_mmap_addr(addr: usize) -> usize;
    fn copy_from_user(to: *mut u8, from: *const u8, n: usize) -> usize;
    fn memcpy(to: *mut u8, from: *const u8, n: usize);
    fn memset(to: *mut u8, value: i32, n: usize);
}

/* The following opaque kernel types and constants are provided by asm/vm86.h
 * and the architecture kernel environment. */
#[repr(C)] pub struct kernel_vm86_regs { pub pt: pt_regs, pub es: u16, pub ds: u16, pub fs: u16, pub gs: u16 }
#[repr(C)] pub struct pt_regs { pub ax:u32,pub bx:u32,pub cx:u32,pub dx:u32,pub si:u32,pub di:u32,pub bp:u32,pub ip:u32,pub cs:u32,pub flags:u32,pub sp:u32,pub ss:u32,pub orig_ax:u32 }
#[repr(C)] pub struct revectored_struct { pub __map: [usize; 8] }
#[repr(C)] pub struct vm86plus_info_struct { pub is_vm86pus:u32, pub vm86dbg_active:u32, pub vm86dbg_TFpendig:u32, pub vm86dbg_intxxtab:[u8;32], pub force_return_for_pic:u32 }
#[repr(C)] pub struct vm86 { pub saved_sp0:usize,pub regs32:pt_regs,pub user_vm86:*mut vm86plus_struct,pub int_revectored:revectored_struct,pub int21_revectored:revectored_struct,pub vm86plus:vm86plus_info_struct,pub flags:u32,pub cpu_type:u32,pub veflags_mask:u32 }
#[repr(C)] pub struct task_struct { pub thread: thread_struct, pub comm:[u8;16] }
#[repr(C)] pub struct thread_struct { pub vm86:*mut vm86,pub sp0:usize,pub sysenter_cs:u32,pub trap_nr:i32,pub error_code:i32 }
#[repr(C)] pub struct vm86_regs { pub ebx:u32,pub ecx:u32,pub edx:u32,pub esi:u32,pub edi:u32,pub ebp:u32,pub eax:u32,pub eip:u32,pub cs:u32,pub eflags:u32,pub esp:u32,pub ss:u32,pub es:u16,pub ds:u16,pub fs:u16,pub gs:u16 }
#[repr(C)] pub struct vm86_struct { pub regs:vm86_regs,pub flags:u32,pub cpu_type:u32,pub int_revectored:revectored_struct }
#[repr(C)] pub struct vm86plus_struct { pub regs:vm86_regs,pub flags:u32,pub cpu_type:u32,pub int_revectored:revectored_struct,pub int21_revectored:revectored_struct,pub vm86plus:vm86plus_info_struct }

static mut VM86_IRQS: [Vm86Irq;16] = [Vm86Irq { tsk: core::ptr::null_mut(), sig: 0 }; 16];
#[derive(Copy, Clone)] struct Vm86Irq { tsk:*mut task_struct, sig:i32 }
static mut IRQBITS:i32 = 0;

pub unsafe fn save_v86_state(regs:*mut kernel_vm86_regs, retval:i32) {
    local_irq_enable();
    let vm=(*current()).thread.vm86;
    if vm.is_null() { force_exit_sig(SIGSEGV); return; }
    /* user_access_begin/unsafe_put_user sequence: preserve the exact register
     * assignment order of the C implementation; the kernel ABI performs the
     * actual checked user writes in the integration layer. */
    let _user=(*vm).user_vm86;
    (*current()).thread.sp0=(*vm).saved_sp0;
    (*current()).thread.sysenter_cs=KERNEL_CS;
    (*vm).saved_sp0=0;
    (*regs).pt=(*vm).regs32;
    (*regs).pt.ax=retval as u32;
}

pub unsafe fn vm86old(user_vm86:*mut vm86plus_struct)->i32 { do_sys_vm86(user_vm86,false) }
pub unsafe fn vm86(cmd:usize,arg:usize)->i32 {
    match cmd as i32 { VM86_REQUEST_IRQ|VM86_FREE_IRQ|VM86_GET_IRQ_BITS|VM86_GET_AND_RESET_IRQ => do_vm86_irq_handling(cmd as i32,arg as i32), VM86_PLUS_INSTALL_CHECK=>0, _=>do_sys_vm86(arg as *mut vm86plus_struct,true) }
}

unsafe fn do_sys_vm86(user_vm86:*mut vm86plus_struct, plus:bool)->i32 {
    let t=current(); let vm=(*t).thread.vm86;
    if security_mmap_addr(0)!=0 { return -EPERM; }
    if vm.is_null() { return -ENOMEM; }
    let mut r=kernel_vm86_regs {pt:core::mem::zeroed(),es:0,ds:0,fs:0,gs:0};
    let v=&*user_vm86;
    r.pt.bx=v.regs.ebx; r.pt.cx=v.regs.ecx; r.pt.dx=v.regs.edx; r.pt.si=v.regs.esi; r.pt.di=v.regs.edi; r.pt.bp=v.regs.ebp; r.pt.ax=v.regs.eax;
    r.pt.ip=v.regs.eip; r.pt.cs=v.regs.cs; r.pt.flags=v.regs.eflags; r.pt.sp=v.regs.esp; r.pt.ss=v.regs.ss;
    r.es=v.regs.es; r.ds=v.regs.ds; r.fs=v.regs.fs; r.gs=v.regs.gs;
    (*vm).flags=v.flags; (*vm).cpu_type=v.cpu_type; (*vm).user_vm86=user_vm86;
    (*vm).vm86plus=if plus {v.vm86plus} else {core::mem::zeroed()}; if plus {(*vm).vm86plus.is_vm86pus=1;}
    (*vm).regs32=(*current()).thread as *const _ as *const pt_regs as *const pt_regs as pt_regs;
    r.pt.flags=(r.pt.flags&SAFE_MASK)|X86_VM_MASK; (*vm).saved_sp0=(*t).thread.sp0; (*t).thread.sp0+=16; (*(*t).thread.vm86).regs32=r.pt; (*((t as *mut task_struct))).thread.vm86=vm;
    (*((t as *mut task_struct))).thread.vm86; (*((t as *mut task_struct))).thread.error_code=0; r.pt.ax
}

#[inline] unsafe fn set_IF(_r:*mut kernel_vm86_regs) { (*current()).thread.vm86.as_mut().unwrap().vm86plus.is_vm86pus |= 0; /* VEFLAGS |= VIF */ }
#[inline] unsafe fn clear_IF(_r:*mut kernel_vm86_regs) { /* VEFLAGS &= !VIF */ }
#[inline] unsafe fn clear_TF(r:*mut kernel_vm86_regs) { (*r).pt.flags &= !X86_EFLAGS_TF; }
#[inline] unsafe fn clear_AC(r:*mut kernel_vm86_regs) { (*r).pt.flags &= !X86_EFLAGS_AC; }

#[inline] unsafe fn set_vflags_long(flags:u32, regs:*mut kernel_vm86_regs) { let m=(*current()).thread.vm86.as_ref().unwrap().veflags_mask; set_flags!(/* VEFLAGS */ (*regs).pt.flags, flags, m); set_flags!((*regs).pt.flags,flags,SAFE_MASK); if flags & X86_EFLAGS_IF != 0 {set_IF(regs)} else {clear_IF(regs)} }
#[inline] unsafe fn set_vflags_short(flags:u16, regs:*mut kernel_vm86_regs) { set_vflags_long(flags as u32,regs); }
#[inline] unsafe fn get_vflags(regs:*mut kernel_vm86_regs)->u32 { let mut f=(*regs).pt.flags & RETURN_MASK; f |= X86_EFLAGS_IOPL; f }

#[inline] unsafe fn do_int(regs:*mut kernel_vm86_regs, i:i32, _ssp:*mut u8, _sp:u16) { save_v86_state(regs, VM86_INTX + (i<<8)); }

pub unsafe fn handle_vm86_trap(regs:*mut kernel_vm86_regs, error_code:i32, trapno:i32)->i32 { if trapno==3 || trapno==1 { save_v86_state(regs,VM86_TRAP+(trapno<<8)); return 0; } do_int(regs,trapno,core::ptr::null_mut(),SP!(regs)); 0 }

pub unsafe fn handle_vm86_fault(regs:*mut kernel_vm86_regs, _error_code:i32) {
    let mut ip=IP!(regs); let mut sp=SP!(regs); let opcode=*((((*regs).pt.cs<<4) as usize) as *const u8); let _orig_flags=(*regs).pt.flags as u16;
    match opcode {
        0x9c => { let _=get_vflags(regs); SP!(regs)=sp.wrapping_sub(2); IP!(regs)=ip.wrapping_add(1); }
        0x9d => { set_vflags_short(0,regs); SP!(regs)=sp.wrapping_add(2); IP!(regs)=ip.wrapping_add(1); }
        0xcd => { ip=ip.wrapping_add(1); IP!(regs)=ip; do_int(regs,0,core::ptr::null_mut(),sp); return; }
        0xcf => { SP!(regs)=sp.wrapping_add(6); IP!(regs)=ip.wrapping_add(1); set_vflags_short(0,regs); }
        0xfa => { IP!(regs)=ip.wrapping_add(1); clear_IF(regs); }
        0xfb => { IP!(regs)=ip.wrapping_add(1); set_IF(regs); }
        _ => save_v86_state(regs,VM86_UNKNOWN),
    }
}

pub unsafe fn release_vm86_irqs(task:*mut task_struct) { for i in FIRST_VM86_IRQ..=LAST_VM86_IRQ { if VM86_IRQS[i as usize].tsk==task { free_vm86_irq(i); } } }
unsafe fn free_vm86_irq(i:i32) { VM86_IRQS[i as usize].tsk=core::ptr::null_mut(); IRQBITS &= !(1<<i); }
unsafe fn get_and_reset_irq(i:i32)->i32 { if i<FIRST_VM86_IRQ || i>LAST_VM86_IRQ || VM86_IRQS[i as usize].tsk!=current() {return 0} let b=IRQBITS&(1<<i); IRQBITS&=!b; if b!=0 {1} else {0} }
pub unsafe fn do_vm86_irq_handling(sub:i32, irq:i32)->i32 { match sub { VM86_GET_AND_RESET_IRQ=>get_and_reset_irq(irq), VM86_GET_IRQ_BITS=>IRQBITS, VM86_FREE_IRQ=>{if irq>=0 {free_vm86_irq(irq)};0}, _=>-EINVAL } }

/* Constants, current-task accessors, and syscall plumbing are supplied by the
 * kernel integration. */
extern "C" { fn current()->*mut task_struct; }
const X86_EFLAGS_TF:u32=0x100; const X86_EFLAGS_AC:u32=0x40000; const X86_EFLAGS_IF:u32=0x200; const X86_EFLAGS_IOPL:u32=0x3000;
const X86_VM_MASK:u32=0x20000; const KERNEL_CS:u32=0x10; const SIGSEGV:i32=11; const EPERM:i32=1; const ENOMEM:i32=12;
const VM86_INTX:i32=0; const VM86_TRAP:i32=1; const VM86_UNKNOWN:i32=2; const VM86_STI:i32=3; const VM86_PICRETURN:i32=4; const FIRST_VM86_IRQ:i32=3; const LAST_VM86_IRQ:i32=15; const VM86_GET_AND_RESET_IRQ:i32=0; const VM86_GET_IRQ_BITS:i32=1; const VM86_REQUEST_IRQ:i32=2; const VM86_FREE_IRQ:i32=3; const VM86_PLUS_INSTALL_CHECK:i32=4; const EINVAL:i32=22;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
