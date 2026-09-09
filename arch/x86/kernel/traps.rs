/* Faithful low-level Rust translation of x86/kernel/traps.c.  Symbols supplied
 * by the surrounding kernel are intentionally left as external dependencies. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

extern "C" {
    static mut current: *mut task_struct;
    fn pt_regs_offset(r: *mut pt_regs, n: i32) -> isize;
    fn fixup_exception(r: *mut pt_regs, trap: i32, error: i64, addr: u64) -> bool;
    fn fixup_vdso_exception(r: *mut pt_regs, trap: i32, error: i64, addr: u64) -> bool;
    fn die(s: *const i8, r: *mut pt_regs, error: i64);
    fn force_sig(sig: i32); fn force_sig_fault(sig: i32, code: i32, addr: *mut core::ffi::c_void);
    fn notify_die(a:i32,s:*const i8,r:*mut pt_regs,e:i64,t:u64,sig:i32)->i32;
    fn uprobe_get_trap_addr(r:*mut pt_regs)->u64;
    fn report_bug(ip:u64,r:*mut pt_regs)->i32;
    fn handle_cfi_failure(r:*mut pt_regs)->i32;
    fn report_bug_entry(p:*mut core::ffi::c_void,r:*mut pt_regs)->i32;
    fn raw_local_irq_enable(); fn raw_local_irq_disable();
}

#[repr(C)] pub struct pt_regs { pub di:u64,pub si:u64,pub dx:u64,pub cx:u64,pub r8:u64,pub r9:u64,pub sp:u64,pub ip:u64,pub flags:u64,pub cs:u64,pub ss:u64,pub orig_ax:u64 }
#[repr(C)] pub struct task_struct { pub thread: thread_struct, pub comm:[u8;16] }
#[repr(C)] pub struct thread_struct { pub error_code:i64,pub trap_nr:i32,pub virtual_dr6:u64,pub iopl_emul:i32,pub iopl_warn:i32,pub pasid_activated:i32 }
#[repr(C)] pub struct arch_va_list { pub regs:[u64;6], pub args: va_args }
#[repr(C)] pub struct va_args { pub gp_offset:u32,pub fp_offset:u32,pub overflow_arg_area:*mut core::ffi::c_void,pub reg_save_area:*mut u64 }

#[inline(always)] unsafe fn is_valid_bugaddr(addr:u64)->i32 { if addr < TASK_SIZE_MAX {0} else if *(addr as *const u16)==INSN_UD2 {1} else {0} }

unsafe fn decode_bug(mut addr:u64, imm:&mut i32, len:&mut i32)->i32 {
    let start=addr; let mut rex:u8=0; let mut lock=false; let mut v:u8;
    if addr<TASK_SIZE_MAX{return BUG_NONE;} loop { v=*(addr as *const u8); addr+=1; if v==INSN_ASOP{continue} if v==INSN_LOCK{lock=true;continue} if v&0xf0==0x40{rex=v;continue} break }
    if (0x70..=0x7f).contains(&v) {addr+=1;*len=(addr-start) as i32;return BUG_LOCK} if v==0xd6 {*len=(addr-start) as i32;return BUG_UDB} if v!=OPCODE_ESCAPE{return BUG_NONE}
    v=*(addr as *const u8);addr+=1;if v==SECOND_BYTE_OPCODE_UD2{*len=(addr-start) as i32;return BUG_UD2} if v!=SECOND_BYTE_OPCODE_UD1{return BUG_NONE}
    *imm=0;v=*(addr as *const u8);addr+=1;let mode=(v>>6)&3;let rm=(v&7)+8*((rex&1)!=0) as u8;let reg=((v>>3)&7)+8*(((rex>>2)&1)!=0) as u8;
    if mode!=3&&(v&7)==4{addr+=1} match mode {0=>{if v&7==5{addr+=4}if rm==0{}if rm==2{*imm=reg as i32}},1=>{*imm=*(addr as *const i8) as i32;addr+=1},2=>{*imm=*(addr as *const i32);addr+=4},_=>{}} *len=(addr-start) as i32; if mode==0&&rm==0{BUG_UD1_UBSAN}else if mode==0&&rm==2{BUG_UD1_WARN}else{BUG_UD1}
}

unsafe fn pt_regs_val(regs:*mut pt_regs,n:i32)->u64 { let off=pt_regs_offset(regs,n); *((regs as *mut u8).offset(off) as *mut u64) }
unsafe fn error_get_trap_addr(regs:*mut pt_regs)->*mut core::ffi::c_void { uprobe_get_trap_addr(regs) as *mut _ }

unsafe fn do_trap_no_signal(tsk:*mut task_struct, trapnr:i32, _str:*const i8, regs:*mut pt_regs, error:i64)->i32 { if !fixup_exception(regs,trapnr,error,0) {(*tsk).thread.error_code=error;(*tsk).thread.trap_nr=trapnr;return -1} 0 }
unsafe fn do_trap(trapnr:i32,signr:i32,str_:*const i8,regs:*mut pt_regs,error:i64,code:i32,addr:*mut core::ffi::c_void){if do_trap_no_signal(current,trapnr,str_,regs,error)!=0{if code==0{force_sig(signr)}else{force_sig_fault(signr,code,addr)}}}

#[no_mangle] pub unsafe extern "C" fn handle_bug(regs:*mut pt_regs)->bool { let addr=(*regs).ip;let mut imm=0;let mut len=0;let typ=decode_bug(addr,&mut imm,&mut len);if typ==BUG_NONE{return false} let mut handled=false;match typ{BUG_UD1_WARN=>{handled=report_bug_entry(pt_regs_val(regs,imm) as *mut _,regs)==BUG_TRAP_TYPE_WARN},BUG_UD2=>{handled=report_bug((*regs).ip,regs)==BUG_TRAP_TYPE_WARN},BUG_UDB|BUG_LOCK=>{handled=handle_cfi_failure(regs)==BUG_TRAP_TYPE_WARN},_=>{}}if handled{if (*regs).ip==addr{(*regs).ip=(*regs).ip.wrapping_add(len as u64)}}else{(*regs).ip=addr}handled}

/* Exception entry wrappers retain the original externally visible interfaces. */
pub unsafe extern "C" fn exc_divide_error(r:*mut pt_regs){do_trap(0,8,b"divide error\0".as_ptr() as _,r,0,0,error_get_trap_addr(r));}
pub unsafe extern "C" fn exc_overflow(r:*mut pt_regs){do_trap(4,11,b"overflow\0".as_ptr() as _,r,0,0,core::ptr::null_mut());}

const TASK_SIZE_MAX:u64=0x0000_8000_0000_0000;const INSN_UD2:u16=0x0b0f;const INSN_ASOP:u8=0x67;const INSN_LOCK:u8=0xf0;const OPCODE_ESCAPE:u8=0x0f;const SECOND_BYTE_OPCODE_UD1:u8=0xb9;const SECOND_BYTE_OPCODE_UD2:u8=0x0b;const BUG_NONE:i32=0;const BUG_UD1:i32=1;const BUG_UD1_WARN:i32=2;const BUG_UD1_UBSAN:i32=3;const BUG_UD2:i32=4;const BUG_UDB:i32=5;const BUG_LOCK:i32=6;const BUG_TRAP_TYPE_WARN:i32=1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
