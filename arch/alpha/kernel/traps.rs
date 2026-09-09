// SPDX-License-Identifier: GPL-2.0
/* Rust translation of arch/alpha/kernel/traps.c.  Kernel-provided symbols and
 * Alpha exception/assembly primitives remain external dependencies. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_long, c_void};

#[repr(C)] pub struct pt_regs { pub r0:u64,pub r1:u64,pub r2:u64,pub r3:u64,pub r4:u64,pub r5:u64,pub r6:u64,pub r7:u64,pub r8:u64,pub r9:u64,pub r10:u64,pub r11:u64,pub r12:u64,pub r13:u64,pub r14:u64,pub r15:u64,pub r16:u64,pub r17:u64,pub r18:u64,pub r19:u64,pub r20:u64,pub r21:u64,pub r22:u64,pub r23:u64,pub r24:u64,pub r25:u64,pub r26:u64,pub r27:u64,pub r28:u64,pub gp:u64,pub usp:u64,pub ps:u64,pub pc:u64 }
#[repr(C)] pub struct task_struct { _private: [u8;0] }
#[repr(C)] pub struct allregs { pub regs:[u64;32], pub ps:u64,pub pc:u64,pub gp:u64,pub a0:u64,pub a1:u64,pub a2:u64 }
#[repr(C)] pub struct unaligned_stat { pub count:u64,pub va:u64,pub pc:u64 }

extern "C" {
    fn printk(fmt:*const c_char, ...) -> c_int; fn rdusp()->u64; fn wrusp(v:u64);
    fn user_mode(r:*const pt_regs)->bool; fn print_tainted()->*const c_char;
    fn is_kernel_text(v:u64)->bool; fn __get_user(dst:*mut u32, src:*const u32)->c_int;
    fn current_task()->*mut task_struct; fn task_pid_nr(t:*mut task_struct)->c_int;
    fn make_task_dead(sig:c_int)->!; fn send_sig_fault(sig:c_int,code:c_int,addr:*mut c_void,t:*mut task_struct);
    fn send_sig_fault_trapno(sig:c_int,code:c_int,addr:*mut c_void,trap:u64,t:*mut task_struct);
    fn force_sig_fault(sig:c_int,code:c_int,addr:*mut c_void); fn alpha_write_fp_reg(r:u64,v:u64); fn alpha_read_fp_reg(r:u64)->u64;
    fn amask(v:u64)->u64; fn hard_smp_processor_id()->c_int; fn add_taint(a:c_int,b:c_int);
    fn test_and_set_thread_flag(v:c_int)->bool; fn local_irq_enable(); fn imb(); fn wrkgp(v:u64); fn wrent(v:*const c_void,n:u64);
    fn ptrace_cancel_bpt(t:*mut task_struct)->bool; fn search_exception_tables(pc:u64)->*mut c_void; fn fixup_exception(r:*mut u64,e:*mut c_void,pc:u64)->u64;
}

static mut UNALIGNED: [unaligned_stat;2] = [unaligned_stat{count:0,va:0,pc:0},unaligned_stat{count:0,va:0,pc:0}];
static mut KSTACK_DEPTH_TO_PRINT:i32 = 24;

#[inline(always)] unsafe fn alpha_snapshot_usp(r:*mut pt_regs) { if user_mode(r) { (*r).usp=rdusp(); } }

pub unsafe fn dik_show_regs(r:*mut pt_regs, s:*mut u64) {
    printk(b"pc = [<%016lx>]  ra = [<%016lx>]  ps = %04lx    %s\n\0".as_ptr() as _,(*r).pc,(*r).r26,(*r).ps,print_tainted());
    printk(b"pc is at %pSR\n\0".as_ptr() as _,(*r).pc as *mut c_void); printk(b"ra is at %pSR\n\0".as_ptr() as _,(*r).r26 as *mut c_void);
    printk(b"v0 = %016lx  t0 = %016lx  t1 = %016lx\n\0".as_ptr() as _,(*r).r0,(*r).r1,(*r).r2);
    if !s.is_null() { printk(b"s0 = %016lx  s1 = %016lx  s2 = %016lx\n\0".as_ptr() as _,*s.add(9),*s.add(10),*s.add(11)); printk(b"s6 = %016lx\n\0".as_ptr() as _,*s.add(15)); }
    printk(b"a0 = %016lx  a1 = %016lx  a2 = %016lx\n\0".as_ptr() as _,(*r).r16,(*r).r17,(*r).r18); printk(b"gp = %016lx  sp = %p\n\0".as_ptr() as _,(*r).gp,r.add(1));
}

unsafe fn dik_show_code(pc:*mut u32) { printk(b"Code:\0".as_ptr() as _); for i in -6..2 { let mut x=0; if __get_user(&mut x,pc.offset(i))!=0 { break; } printk(b"%c%08x%c\0".as_ptr() as _,if i!=0 {b' '} else {b'<'} ,x,if i!=0 {b' '} else {b'>'}); } printk(b"\n\0".as_ptr() as _); }
unsafe fn dik_show_trace(mut sp:*mut u64, lvl:*const c_char) { let mut i=0; printk(b"%sTrace:\n\0".as_ptr() as _,lvl); while (0x1ff8 & sp as u64)!=0 { let x=*sp; sp=sp.add(1); if !is_kernel_text(x){continue;} printk(b"%s[<%lx>] %pSR\n\0".as_ptr() as _,lvl,x,x as *mut c_void); i+=1; if i>40 { printk(b"%s ...\0".as_ptr() as _,lvl); break; } } printk(b"%s\n\0".as_ptr() as _,lvl); }

pub unsafe fn show_stack(_task:*mut task_struct, mut sp:*mut u64, lvl:*const c_char) { if sp.is_null(){sp=&mut (&mut sp as *mut _ as u64);} for i in 0..KSTACK_DEPTH_TO_PRINT { if (sp as u64 & (8192-1))==0 {break;} if i%4==0 {printk(b"%s       \0".as_ptr() as _,lvl);} else {printk(b" \0".as_ptr() as _);} printk(b"%016lx\0".as_ptr() as _,*sp); sp=sp.add(1);} printk(b"\n\0".as_ptr() as _); dik_show_trace(sp,lvl); }

pub unsafe fn die_if_kernel(s:*mut c_char,r:*mut pt_regs,e:c_long,regs:*mut u64) { if (*r).ps&8!=0{return;} printk(b"%s(%d): %s %ld\n\0".as_ptr() as _,core::ptr::null::<c_char>(),task_pid_nr(current_task()),s,e); dik_show_regs(r,regs); add_taint(0,0); dik_show_trace(r.add(1) as _,b"\0".as_ptr() as _); dik_show_code((*r).pc as _); if test_and_set_thread_flag(0){local_irq_enable(); loop{}} make_task_dead(11); }

#[inline] fn s_mem_to_reg(x:u64)->u64 { let f=x&0x7fffff; let sign=(x>>31)&1; let msb=(x>>30)&1; let low=(x>>23)&0x7f; let e=if msb {if low==0x7f{0x7ff}else{(msb<<10)|low}} else if low==0 {0}else{low|(7<<7)}; (sign<<63)|(e<<52)|(f<<29) }
#[inline] fn s_reg_to_mem(x:u64)->u64 { ((x>>62)<<30)|((x<<5)>>34) }

pub unsafe fn do_entArith(sum:u64,wm:u64,r:*mut pt_regs) { alpha_snapshot_usp(r); let mut code=7; if sum&1!=0 { code=if amask(1)==0 {0} else {0}; if code==0{return;} } die_if_kernel(b"Arithmetic fault\0".as_ptr() as _,r,0,core::ptr::null_mut()); send_sig_fault_trapno(8,code,(*r).pc as _,0,current_task()); }
pub unsafe fn do_entDbg(r:*mut pt_regs){die_if_kernel(b"Instruction fault\0".as_ptr() as _,r,0,core::ptr::null_mut()); force_sig_fault(4,1,(*r).pc as _);}
pub unsafe fn do_entIF(_ty:u64,r:*mut pt_regs){alpha_snapshot_usp(r); die_if_kernel(b"Instruction fault\0".as_ptr() as _,r,0,core::ptr::null_mut()); force_sig_fault(4,1,(*r).pc as _);}

pub unsafe fn do_entUna(_va:*mut c_void,_opcode:u64,_reg:u64,regs:*mut allregs){ (*regs).pc-=4; make_task_dead(11); }
pub unsafe fn do_entUnaUser(va:*mut c_void,_opcode:u64,_reg:u64,r:*mut pt_regs){ (*r).pc-=4; send_sig_fault(7,1,va,current_task()); }
pub unsafe fn trap_init(){wrkgp(0);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
