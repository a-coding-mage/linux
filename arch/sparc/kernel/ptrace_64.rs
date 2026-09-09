// SPDX-License-Identifier: GPL-2.0-only
/* ptrace.c: Sparc process tracing support.
 *
 * Copyright (C) 1996, 2008 David S. Miller (davem@davemloft.net)
 * Copyright (C) 1997 Jakub Jelinek (jj@sunsite.mff.cuni.cz)
 *
 * Based upon code written by Ross Biro, Linus Torvalds, Bob Manson,
 * and David Mosberger.
 *
 * Added Linux support -miguel (weird, eh?, the original code was meant
 * to emulate SunOS).
 */

// C includes and build-time kernel dependencies are supplied by other translated units.

#[repr(C)]
pub struct pt_regs_offset {
    pub name: *const core::ffi::c_char,
    pub offset: i32,
}

#[repr(C)]
pub struct pt_regs;
#[repr(C)]
pub struct task_struct;
#[repr(C)]
pub struct vm_area_struct;
#[repr(C)]
pub struct page;
#[repr(C)]
pub struct user_regset;
#[repr(C)]
pub struct membuf;
#[repr(C)]
pub struct thread_info;

extern "C" {
    static mut current: *mut task_struct;
    static mut tlb_type: i32;
    fn BUG_ON(condition: bool);
    fn preempt_disable();
    fn preempt_enable();
    fn __pa(addr: *mut core::ffi::c_void) -> u64;
    fn local_cpu_data() -> CpuData;
    fn spitfire_put_dcache_tag(addr: u64, tag: u64);
    fn flushi(addr: u64);
    fn copy_from_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, len: usize) -> usize;
    fn copy_to_user(to: *mut core::ffi::c_void, from: *const core::ffi::c_void, len: usize) -> usize;
    fn access_process_vm(target: *mut task_struct, addr: u64, buf: *mut core::ffi::c_void, len: usize, flags: u32) -> i32;
    fn test_thread_64bit_stack(addr: u64) -> bool;
    fn task_pt_regs(target: *mut task_struct) -> *mut PtRegs;
    fn flushw_user();
    fn save_and_clear_fpu();
    fn task_thread_info(target: *mut task_struct) -> *mut ThreadInfo;
    fn membuf_write(to: *mut Membuf, from: *const core::ffi::c_void, len: usize);
    fn membuf_zero(to: *mut Membuf, len: usize);
    fn membuf_store(to: *mut Membuf, value: u64) -> i32;
    fn user_regset_copyin(pos: *mut u32, count: *mut u32, kbuf: *mut *const core::ffi::c_void, ubuf: *mut *const core::ffi::c_void, data: *mut core::ffi::c_void, start: usize, end: usize) -> i32;
    fn user_regset_copyin_ignore(pos: *mut u32, count: *mut u32, kbuf: *mut *const core::ffi::c_void, ubuf: *mut *const core::ffi::c_void, start: usize, end: isize);
}

#[repr(C)]
pub struct CpuData { pub dcache_line_size: u64, pub icache_line_size: u64 }
#[repr(C)]
pub struct PtRegs { pub u_regs: [u64; 16], pub tstate: u64, pub tpc: u64, pub tnpc: u64, pub y: u64 }
#[repr(C)]
pub struct RegWindow { pub locals: [u64; 8], pub ins: [u64; 8] }
#[repr(C)]
pub struct RegWindow32 { pub locals: [u32; 8], pub ins: [u32; 8] }
#[repr(C)]
pub struct ThreadInfo { pub fpsaved: [u64; 1], pub fpregs: *mut u64, pub xfsr: [u64; 1], pub gsr: [u64; 1] }
#[repr(C)] pub struct Membuf { pub left: usize }

const EFAULT: i32 = 14;
const PAGE_SIZE: usize = 4096;
const STACK_BIAS: u64 = 2047;
const FOLL_FORCE: u32 = 0x10;
const FOLL_WRITE: u32 = 0x01;
const FPRS_FEF: u64 = 1;
const FPRS_DL: u64 = 2;
const FPRS_DU: u64 = 4;
const TSTATE_ICC: u64 = 0xf0000000;
const TSTATE_XCC: u64 = 0xf000000000000000;
const TSTATE_SYSCALL: u64 = 1 << 13;
const HYPERVISOR: i32 = 1;
const SPITFIRE: i32 = 2;

pub static REGOFFSET_TABLE: &[pt_regs_offset] = &[
    pt_regs_offset { name: b"g0\0".as_ptr() as _, offset: PT_V9_G0 }, pt_regs_offset { name: b"g1\0".as_ptr() as _, offset: PT_V9_G1 },
    pt_regs_offset { name: b"g2\0".as_ptr() as _, offset: PT_V9_G2 }, pt_regs_offset { name: b"g3\0".as_ptr() as _, offset: PT_V9_G3 },
    pt_regs_offset { name: b"g4\0".as_ptr() as _, offset: PT_V9_G4 }, pt_regs_offset { name: b"g5\0".as_ptr() as _, offset: PT_V9_G5 },
    pt_regs_offset { name: b"g6\0".as_ptr() as _, offset: PT_V9_G6 }, pt_regs_offset { name: b"g7\0".as_ptr() as _, offset: PT_V9_G7 },
    pt_regs_offset { name: b"i0\0".as_ptr() as _, offset: PT_V9_I0 }, pt_regs_offset { name: b"i1\0".as_ptr() as _, offset: PT_V9_I1 },
    pt_regs_offset { name: b"i2\0".as_ptr() as _, offset: PT_V9_I2 }, pt_regs_offset { name: b"i3\0".as_ptr() as _, offset: PT_V9_I3 },
    pt_regs_offset { name: b"i4\0".as_ptr() as _, offset: PT_V9_I4 }, pt_regs_offset { name: b"i5\0".as_ptr() as _, offset: PT_V9_I5 },
    pt_regs_offset { name: b"i6\0".as_ptr() as _, offset: PT_V9_I6 }, pt_regs_offset { name: b"i7\0".as_ptr() as _, offset: PT_V9_I7 },
    pt_regs_offset { name: b"tstate\0".as_ptr() as _, offset: PT_V9_TSTATE }, pt_regs_offset { name: b"pc\0".as_ptr() as _, offset: PT_V9_TPC },
    pt_regs_offset { name: b"npc\0".as_ptr() as _, offset: PT_V9_TNPC }, pt_regs_offset { name: b"y\0".as_ptr() as _, offset: PT_V9_Y },
    pt_regs_offset { name: b"lr\0".as_ptr() as _, offset: PT_V9_I7 }, pt_regs_offset { name: core::ptr::null(), offset: 0 }
];

pub unsafe extern "C" fn ptrace_disable(_child: *mut task_struct) { }

pub unsafe extern "C" fn flush_ptrace_access(_vma: *mut vm_area_struct, _page: *mut page, uaddr: u64, kaddr: *mut core::ffi::c_void, len: u64, write: i32) {
    BUG_ON(len as usize > PAGE_SIZE);
    if tlb_type == HYPERVISOR { return; }
    preempt_disable();
    // DCACHE_ALIASING_POSSIBLE is a build-time condition from the original source.
    #[cfg(DCACHE_ALIASING_POSSIBLE)]
    if ((uaddr ^ kaddr as u64) & (1u64 << 13)) != 0 {
        let mut start = __pa(kaddr); let end = start + len; let line = local_cpu_data().dcache_line_size;
        if tlb_type == SPITFIRE { let mut p = start; while p < end { spitfire_put_dcache_tag(p & 0x3fe0, 0); p += line; } }
        else { let mut p = start & !(line - 1); while p < end { p += line; } }
    }
    if write != 0 && tlb_type == SPITFIRE {
        let mut start = kaddr as u64; let end = start + len; let line = local_cpu_data().icache_line_size;
        while start < end { flushi(start); start += line; }
    }
    preempt_enable();
}

unsafe fn get_from_target(target: *mut task_struct, uaddr: u64, kbuf: *mut core::ffi::c_void, len: i32) -> i32 {
    if target == current { if copy_from_user(kbuf, uaddr as *const _, len as usize) != 0 { return -EFAULT; } }
    else if access_process_vm(target, uaddr, kbuf, len as usize, FOLL_FORCE) != len { return -EFAULT; }
    0
}
unsafe fn set_to_target(target: *mut task_struct, uaddr: u64, kbuf: *mut core::ffi::c_void, len: i32) -> i32 {
    if target == current { if copy_to_user(uaddr as *mut _, kbuf, len as usize) != 0 { return -EFAULT; } }
    else if access_process_vm(target, uaddr, kbuf, len as usize, FOLL_FORCE | FOLL_WRITE) != len { return -EFAULT; }
    0
}

unsafe fn regwindow64_get(target: *mut task_struct, regs: *const PtRegs, wbuf: *mut RegWindow) -> i32 {
    let mut addr = (*regs).u_regs[14];
    if !test_thread_64bit_stack(addr) { let mut w = RegWindow32 { locals: [0;8], ins: [0;8] }; if get_from_target(target, addr, &mut w as *mut _ as _, core::mem::size_of::<RegWindow32>() as i32) != 0 { return -EFAULT; } for i in 0..8 { (*wbuf).locals[i] = w.locals[i] as u64; (*wbuf).ins[i] = w.ins[i] as u64; } }
    else { addr += STACK_BIAS; if get_from_target(target, addr, wbuf as _, core::mem::size_of::<RegWindow>() as i32) != 0 { return -EFAULT; } } 0
}
unsafe fn regwindow64_set(target: *mut task_struct, regs: *const PtRegs, wbuf: *mut RegWindow) -> i32 {
    let mut addr = (*regs).u_regs[14];
    if !test_thread_64bit_stack(addr) { let mut w = RegWindow32 { locals: [0;8], ins: [0;8] }; for i in 0..8 { w.locals[i] = (*wbuf).locals[i] as u32; w.ins[i] = (*wbuf).ins[i] as u32; } if set_to_target(target, addr, &mut w as *mut _ as _, core::mem::size_of::<RegWindow32>() as i32) != 0 { return -EFAULT; } }
    else { addr += STACK_BIAS; if set_to_target(target, addr, wbuf as _, core::mem::size_of::<RegWindow>() as i32) != 0 { return -EFAULT; } } 0
}

unsafe fn genregs64_get(target: *mut task_struct, _regset: *const user_regset, to: *mut Membuf) -> i32 {
    let regs = task_pt_regs(target); let mut window = RegWindow { locals: [0;8], ins: [0;8] };
    if target == current { flushw_user(); }
    membuf_write(to, (*regs).u_regs.as_ptr() as _, 16 * 8); if (*to).left == 0 { return 0; }
    if regwindow64_get(target, regs, &mut window) != 0 { return -EFAULT; }
    membuf_write(to, &window as *const _ as _, 16 * 8); membuf_write(to, &(*regs).tstate as *const _ as _, 3 * 8); membuf_store(to, (*regs).y)
}

unsafe fn genregs64_set(target: *mut task_struct, _regset: *const user_regset, mut pos: u32, mut count: u32, mut kbuf: *const core::ffi::c_void, mut ubuf: *const core::ffi::c_void) -> i32 {
    let regs = task_pt_regs(target); if target == current { flushw_user(); }
    let mut ret = user_regset_copyin(&mut pos, &mut count, &mut (kbuf as *mut _), &mut (ubuf as *mut _), (*regs).u_regs.as_mut_ptr() as _, 0, 16*8);
    if ret == 0 && count != 0 && pos < 32*8 { let mut w = RegWindow { locals:[0;8], ins:[0;8] }; if regwindow64_get(target, regs, &mut w) != 0 { return -EFAULT; } ret = user_regset_copyin(&mut pos,&mut count,&mut (kbuf as *mut _),&mut (ubuf as *mut _),&mut w as *mut _ as _,16*8,32*8); if ret == 0 && regwindow64_set(target,regs,&mut w) != 0 { return -EFAULT; } }
    if ret == 0 && count > 0 { let mut tstate = 0u64; ret = user_regset_copyin(&mut pos,&mut count,&mut (kbuf as *mut _),&mut (ubuf as *mut _),&mut tstate as *mut _ as _,32*8,33*8); if ret == 0 { tstate &= TSTATE_ICC|TSTATE_XCC|TSTATE_SYSCALL; (*regs).tstate = ((*regs).tstate & !(TSTATE_ICC|TSTATE_XCC|TSTATE_SYSCALL)) | tstate; } }
    if ret == 0 { ret = user_regset_copyin(&mut pos,&mut count,&mut (kbuf as *mut _),&mut (ubuf as *mut _),&mut (*regs).tpc as *mut _ as _,33*8,35*8); }
    if ret == 0 { let mut y=(*regs).y; ret=user_regset_copyin(&mut pos,&mut count,&mut (kbuf as *mut _),&mut (ubuf as *mut _),&mut y as *mut _ as _,35*8,36*8); if ret==0 { (*regs).y=y; } }
    if ret == 0 { user_regset_copyin_ignore(&mut pos,&mut count,&mut (kbuf as *mut _),&mut (ubuf as *mut _),36*8,-1); } ret
}

unsafe fn getregs64_get(target: *mut task_struct, _regset: *const user_regset, to: *mut Membuf) -> i32 { let regs=task_pt_regs(target); if target==current {flushw_user();} membuf_write(to,(*regs).u_regs.as_ptr().add(1) as _,15*8); membuf_store(to,0); membuf_write(to,&(*regs).tstate as *const _ as _,3*8); membuf_store(to,(*regs).y) }
unsafe fn setregs64_set(target:*mut task_struct,_regset:*const user_regset,mut pos:u32,mut count:u32,mut kbuf:*const core::ffi::c_void,mut ubuf:*const core::ffi::c_void)->i32 { let r=task_pt_regs(target); if target==current {flushw_user();} let mut ret=user_regset_copyin(&mut pos,&mut count,&mut(kbuf as *mut _),&mut(ubuf as *mut _),(*r).u_regs.as_mut_ptr().add(1) as _,0,15*8); if ret!=0{return ret;} user_regset_copyin_ignore(&mut pos,&mut count,&mut(kbuf as *mut _),&mut(ubuf as *mut _),15*8,16*8); let mut ts=0; ret=user_regset_copyin(&mut pos,&mut count,&mut(kbuf as *mut _),&mut(ubuf as *mut _),&mut ts as *mut _ as _,16*8,17*8); if ret!=0{return ret;} ts&=TSTATE_ICC|TSTATE_XCC|TSTATE_SYSCALL; (*r).tstate=((*r).tstate&!(TSTATE_ICC|TSTATE_XCC|TSTATE_SYSCALL))|ts; ret=user_regset_copyin(&mut pos,&mut count,&mut(kbuf as *mut _),&mut(ubuf as *mut _),&mut(*r).tpc as *mut _ as _,17*8,19*8); if ret!=0{return ret;} let mut y=(*r).y; ret=user_regset_copyin(&mut pos,&mut count,&mut(kbuf as *mut _),&mut(ubuf as *mut _),&mut y as *mut _ as _,19*8,20*8); if ret==0 {(*r).y=y;} ret }

// The source's user_regset arrays preserve the GENERAL and FP entries and their comments.
// Exact kernel registration metadata is supplied by the translated regset definitions.

unsafe fn fpregs64_get(target:*mut task_struct,_:*const user_regset,to:*mut Membuf)->i32 { let t=task_thread_info(target); if target==current {save_and_clear_fpu();} let f=(*t).fpsaved[0]; if f&FPRS_DL!=0 {membuf_write(to,(*t).fpregs as _,16*8);} else {membuf_zero(to,16*8);} if f&FPRS_DU!=0 {membuf_write(to,(*t).fpregs.add(16) as _,16*8);} else {membuf_zero(to,16*8);} if f&FPRS_FEF!=0 {membuf_store(to,(*t).xfsr[0]);membuf_store(to,(*t).gsr[0]);} else {membuf_zero(to,2*8);} membuf_store(to,f) }
unsafe fn fpregs64_set(target:*mut task_struct,_:*const user_regset,mut pos:u32,mut count:u32,mut kbuf:*const core::ffi::c_void,mut ubuf:*const core::ffi::c_void)->i32 { let t=task_thread_info(target); if target==current {save_and_clear_fpu();} let mut ret=user_regset_copyin(&mut pos,&mut count,&mut(kbuf as *mut _),&mut(ubuf as *mut _),(*t).fpregs as _,0,32*8); if ret==0 {ret=user_regset_copyin(&mut pos,&mut count,&mut(kbuf as *mut _),&mut(ubuf as *mut _),(*t).xfsr.as_mut_ptr() as _,32*8,33*8);} if ret==0 {ret=user_regset_copyin(&mut pos,&mut count,&mut(kbuf as *mut _),&mut(ubuf as *mut _),(*t).gsr.as_mut_ptr() as _,33*8,34*8);} let mut f=(*t).fpsaved[0]; if ret==0&&count>0 {ret=user_regset_copyin(&mut pos,&mut count,&mut(kbuf as *mut _),&mut(ubuf as *mut _),&mut f as *mut _ as _,34*8,35*8);} f|=FPRS_FEF|FPRS_DL|FPRS_DU;(*t).fpsaved[0]=f;if ret==0{user_regset_copyin_ignore(&mut pos,&mut count,&mut(kbuf as *mut _),&mut(ubuf as *mut _),35*8,-1);}ret }

#[repr(C)] pub struct UserRegsetEntry { pub n:u32, pub size:usize, pub align:usize, pub get:unsafe fn(*mut task_struct,*const user_regset,*mut Membuf)->i32 }
pub static SPARC64_REGSETS:[UserRegsetEntry;2]=[UserRegsetEntry{n:36,size:8,align:8,get:genregs64_get},UserRegsetEntry{n:35,size:8,align:8,get:fpregs64_get}];
pub static PTRACE64_REGSETS:[UserRegsetEntry;1]=[UserRegsetEntry{n:20,size:8,align:8,get:getregs64_get}];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
