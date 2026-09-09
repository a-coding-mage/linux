// SPDX-License-Identifier: GPL-2.0
/* ptrace.c: Sparc process tracing support.
 *
 * Copyright (C) 1996, 2008 David S. Miller (davem@davemloft.net)
 *
 * Based upon code written by Ross Biro, Linus Torvalds, Bob Manson,
 * and David Mosberger.
 *
 * Added Linux support -miguel (weird, eh?, the original code was meant
 * to emulate SunOS).
 */

// Linux and architecture headers supplied by the surrounding kernel.
// #define ALLOW_INIT_TRACING

#[repr(C)]
pub struct task_struct { pub thread: thread_struct }
#[repr(C)]
pub struct thread_struct { pub kregs: *mut pt_regs, pub float_regs: *mut u32, pub fsr: u32 }
#[repr(C)]
pub struct pt_regs { pub u_regs: [u32; 16], pub psr: u32, pub pc: u32, pub npc: u32, pub y: u32 }
#[repr(C)]
pub struct membuf { pub left: usize }
#[repr(C)]
pub struct user_regset { pub core_note_type: u32, pub n: usize, pub size: usize, pub align: usize, pub regset_get: Option<unsafe extern "C" fn(*mut task_struct, *const user_regset, membuf) -> i32>, pub set: Option<unsafe extern "C" fn(*mut task_struct, *const user_regset, u32, u32, *const core::ffi::c_void, *const core::ffi::c_void) -> i32> }
#[repr(C)]
pub struct user_regset_view { pub name: *const u8, pub e_machine: u32, pub regsets: *const user_regset, pub n: usize }

extern "C" {
    static mut current: *mut task_struct;
    fn copy_from_user(to: *mut u32, from: *const u32, size: usize) -> usize;
    fn copy_to_user(to: *mut u32, from: *const u32, size: usize) -> usize;
    fn access_process_vm(t: *mut task_struct, addr: usize, buf: *mut u32, size: usize, flags: u32) -> isize;
    fn flush_user_windows();
    fn membuf_write(to: *mut membuf, from: *const u32, size: usize);
    fn membuf_store(to: *mut membuf, value: u32);
    fn membuf_zero(to: *mut membuf, size: usize) -> i32;
    fn user_regset_copyin(pos: *mut u32, count: *mut u32, kbuf: *mut *const core::ffi::c_void, ubuf: *mut *const core::ffi::c_void, dst: *mut u32, start: usize, end: usize) -> i32;
    fn user_regset_copyin_ignore(pos: *mut u32, count: *mut u32, kbuf: *mut *const core::ffi::c_void, ubuf: *mut *const core::ffi::c_void, start: usize, end: usize);
    fn copy_regset_to_user(task: *mut task_struct, view: *const user_regset_view, set: u32, pos: usize, count: usize, ubuf: *mut core::ffi::c_void) -> i32;
    fn copy_regset_from_user(task: *mut task_struct, view: *const user_regset_view, set: u32, pos: usize, count: usize, ubuf: *const core::ffi::c_void) -> i32;
    fn ptrace_readdata(child: *mut task_struct, addr: usize, out: *mut core::ffi::c_void, data: usize) -> i32;
    fn ptrace_writedata(child: *mut task_struct, out: *mut core::ffi::c_void, addr: usize, data: usize) -> i32;
    fn ptrace_request(child: *mut task_struct, request: isize, addr: usize, data: usize) -> i32;
    fn test_thread_flag(flag: u32) -> bool;
    fn ptrace_report_syscall_exit(regs: *mut pt_regs, value: u32);
    fn ptrace_report_syscall_permit_entry(regs: *mut pt_regs) -> i32;
}

const UREG_I4: usize = 12;
const UREG_I6: usize = 14;
const FOLL_FORCE: u32 = 0x10;
const FOLL_WRITE: u32 = 0x01;
const PSR_ICC: u32 = 0x0f000000;
const PSR_SYSCALL: u32 = 0x00000080;
const EFAULT: i32 = 14;
const EIO: i32 = 5;
const TIF_SYSCALL_TRACE: u32 = 0;
const REGSET_GENERAL: usize = 0;
const REGSET_FP: usize = 1;
const PTRACE_GETREGS: isize = 12;
const PTRACE_SETREGS: isize = 13;
const PTRACE_GETFPREGS: isize = 14;
const PTRACE_SETFPREGS: isize = 15;
const PTRACE_READTEXT: isize = 0;
const PTRACE_READDATA: isize = 1;
const PTRACE_WRITETEXT: isize = 4;
const PTRACE_WRITEDATA: isize = 5;
const PTRACE_SPARC_DETACH: isize = 17;
const PTRACE_DETACH: isize = 8;
const EM_SPARC: u32 = 2;

#[no_mangle]
pub unsafe extern "C" fn ptrace_disable(_child: *mut task_struct) { }

unsafe fn regwindow32_get(target: *mut task_struct, regs: *const pt_regs, uregs: *mut u32) -> i32 {
    let reg_window = (*regs).u_regs[UREG_I6] as usize;
    let size = 16 * core::mem::size_of::<u32>();
    if target == current { if copy_from_user(uregs, reg_window as *const u32, size) != 0 { return -EFAULT; } }
    else if access_process_vm(target, reg_window, uregs, size, FOLL_FORCE) != size as isize { return -EFAULT; }
    0
}

unsafe fn regwindow32_set(target: *mut task_struct, regs: *const pt_regs, uregs: *mut u32) -> i32 {
    let reg_window = (*regs).u_regs[UREG_I6] as usize;
    let size = 16 * core::mem::size_of::<u32>();
    if target == current { if copy_to_user(reg_window as *mut u32, uregs, size) != 0 { return -EFAULT; } }
    else if access_process_vm(target, reg_window, uregs, size, FOLL_FORCE | FOLL_WRITE) != size as isize { return -EFAULT; }
    0
}

unsafe fn genregs32_get(target: *mut task_struct, _regset: *const user_regset, mut to: membuf) -> i32 {
    let regs = (*target).thread.kregs; let mut uregs = [0u32; 16];
    if target == current { flush_user_windows(); }
    membuf_write(&mut to, (*regs).u_regs.as_ptr(), 16 * 4); if to.left == 0 { return 0; }
    if regwindow32_get(target, regs, uregs.as_mut_ptr()) != 0 { return -EFAULT; }
    membuf_write(&mut to, uregs.as_ptr(), 16 * 4); membuf_store(&mut to, (*regs).psr); membuf_store(&mut to, (*regs).pc); membuf_store(&mut to, (*regs).npc); membuf_store(&mut to, (*regs).y); membuf_zero(&mut to, 2 * 4)
}

unsafe fn genregs32_set(target: *mut task_struct, _r: *const user_regset, mut pos: u32, mut count: u32, mut kbuf: *const core::ffi::c_void, mut ubuf: *const core::ffi::c_void) -> i32 {
    let regs = (*target).thread.kregs; let mut uregs = [0u32; 16]; let mut psr = 0u32;
    if target == current { flush_user_windows(); }
    let mut ret = user_regset_copyin(&mut pos,&mut count,&mut kbuf,&mut ubuf,(*regs).u_regs.as_mut_ptr(),0,16*4); if ret != 0 || count == 0 { return ret; }
    if regwindow32_get(target,regs,uregs.as_mut_ptr()) != 0 { return -EFAULT; }
    ret=user_regset_copyin(&mut pos,&mut count,&mut kbuf,&mut ubuf,uregs.as_mut_ptr(),16*4,32*4); if ret != 0 { return ret; }
    if regwindow32_set(target,regs,uregs.as_mut_ptr()) != 0 { return -EFAULT; } if count == 0 { return 0; }
    ret=user_regset_copyin(&mut pos,&mut count,&mut kbuf,&mut ubuf,&mut psr,32*4,33*4); if ret != 0 { return ret; }
    (*regs).psr = ((*regs).psr & !(PSR_ICC|PSR_SYSCALL)) | (psr & (PSR_ICC|PSR_SYSCALL)); if count == 0 { return 0; }
    ret=user_regset_copyin(&mut pos,&mut count,&mut kbuf,&mut ubuf,&mut (*regs).pc,33*4,34*4); if ret != 0 || count == 0 { return ret; }
    ret=user_regset_copyin(&mut pos,&mut count,&mut kbuf,&mut ubuf,&mut (*regs).npc,34*4,35*4); if ret != 0 || count == 0 { return ret; }
    ret=user_regset_copyin(&mut pos,&mut count,&mut kbuf,&mut ubuf,&mut (*regs).y,35*4,36*4); if ret != 0 || count == 0 { return ret; }
    user_regset_copyin_ignore(&mut pos,&mut count,&mut kbuf,&mut ubuf,36*4,38*4); 0
}

unsafe fn fpregs32_get(target: *mut task_struct, _r: *const user_regset, mut to: membuf) -> i32 {
    membuf_write(&mut to, (*target).thread.float_regs, 32*4); membuf_zero(&mut to, 4); membuf_write(&mut to, &(*target).thread.fsr, 4); membuf_store(&mut to, ((1<<8)|(8<<16)) as u32); membuf_zero(&mut to, 64*4)
}
unsafe fn fpregs32_set(target: *mut task_struct, _r: *const user_regset, mut pos:u32, mut count:u32, mut kbuf:*const core::ffi::c_void, mut ubuf:*const core::ffi::c_void)->i32 { let mut ret=user_regset_copyin(&mut pos,&mut count,&mut kbuf,&mut ubuf,(*target).thread.float_regs,0,32*4); if ret==0 { user_regset_copyin_ignore(&mut pos,&mut count,&mut kbuf,&mut ubuf,32*4,33*4); ret=user_regset_copyin(&mut pos,&mut count,&mut kbuf,&mut ubuf,&mut (*target).thread.fsr,33*4,34*4); } if ret==0 { user_regset_copyin_ignore(&mut pos,&mut count,&mut kbuf,&mut ubuf,34*4,usize::MAX); } ret }

#[repr(C)] pub struct fps { pub regs:[usize;32], pub fsr:usize, pub flags:usize, pub extra:usize, pub fpqd:usize, pub fpq:[fq;16] }
#[repr(C)] pub struct fq { pub insnaddr:*mut usize, pub insn:usize }

pub unsafe extern "C" fn task_user_regset_view(_task:*mut task_struct)->*const user_regset_view { core::ptr::null() }

pub unsafe extern "C" fn arch_ptrace(child:*mut task_struct, mut request:isize, addr:usize, data:usize)->i32 {
    let addr2=(*current).thread.kregs.as_ref().unwrap().u_regs[UREG_I4] as usize; let addr2p=addr2 as *mut core::ffi::c_void; let mut ret;
    match request { PTRACE_GETREGS=>ret=copy_regset_to_user(child,core::ptr::null(),REGSET_GENERAL,0,19*4,addr as *mut _), PTRACE_SETREGS=>ret=copy_regset_from_user(child,core::ptr::null(),REGSET_GENERAL,0,19*4,addr as *const _), PTRACE_GETFPREGS=>ret=copy_regset_to_user(child,core::ptr::null(),REGSET_FP,0,68*4,addr as *mut _), PTRACE_SETFPREGS=>ret=copy_regset_from_user(child,core::ptr::null(),REGSET_FP,0,33*4,addr as *const _), PTRACE_READTEXT|PTRACE_READDATA=>{ret=ptrace_readdata(child,addr,addr2p,data);if ret==data as i32{ret=0}else if ret>=0{ret=-EIO}}, PTRACE_WRITETEXT|PTRACE_WRITEDATA=>{ret=ptrace_writedata(child,addr2p,addr,data);if ret==data as i32{ret=0}else if ret>=0{ret=-EIO}}, _=>{if request==PTRACE_SPARC_DETACH{request=PTRACE_DETACH} ret=ptrace_request(child,request,addr,data)} } ret
}

pub unsafe extern "C" fn syscall_trace(regs:*mut pt_regs, syscall_exit_p: i32)->i32 { let mut ret=0; if test_thread_flag(TIF_SYSCALL_TRACE) { if syscall_exit_p!=0 { ptrace_report_syscall_exit(regs,0); } else { ret=(ptrace_report_syscall_permit_entry(regs)==0) as i32; } } ret }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
