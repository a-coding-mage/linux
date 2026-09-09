// SPDX-License-Identifier: GPL-2.0
// Translated from trace_syscalls.c. Kernel-provided declarations and macros are
// intentionally left as external dependencies.

use core::{ffi::c_void, mem::MaybeUninit, ptr};

#[allow(non_camel_case_types, non_snake_case, dead_code)]
extern "C" {
    fn syscall_get_nr(task: *mut task_struct, regs: *mut pt_regs) -> i32;
    fn syscall_get_return_value(task: *mut task_struct, regs: *mut pt_regs) -> i64;
    fn syscall_get_arguments(task: *mut task_struct, regs: *mut pt_regs, args: *mut usize);
    fn trace_seq_printf(s: *mut trace_seq, fmt: *const i8, ...);
    fn trace_seq_puts(s: *mut trace_seq, text: *const i8);
    fn trace_seq_putc(s: *mut trace_seq, c: i32);
    fn trace_seq_has_overflowed(s: *mut trace_seq) -> bool;
    fn trace_handle_return(s: *mut trace_seq) -> print_line_t;
    fn trace_event_buffer_reserve(b: *mut trace_event_buffer, f: *mut trace_event_file, size: usize) -> *mut c_void;
    fn trace_event_buffer_commit(b: *mut trace_event_buffer);
    fn ring_buffer_event_data(e: *mut c_void) -> *mut c_void;
    fn trace_user_fault_init(b: *mut trace_user_buf_info, size: usize) -> i32;
    fn trace_user_fault_destroy(b: *mut trace_user_buf_info);
    fn trace_user_fault_get(b: *mut trace_user_buf_info);
    fn trace_user_fault_put(b: *mut trace_user_buf_info) -> bool;
    fn trace_user_fault_read(b: *mut trace_user_buf_info, p: *mut c_void, size: usize, cb: Option<unsafe extern "C" fn(*mut i8,*const i8,usize,*mut c_void)->i32>, data: *mut c_void) -> *mut i8;
    fn strncpy_from_user(dst: *mut i8, src: *const i8, size: usize) -> i32;
    fn __copy_from_user(dst: *mut i8, src: *const i8, size: usize) -> usize;
    fn trace_define_field(call: *mut trace_event_call, ty: *const i8, name: *const i8, off: i32, size: usize, sign: i32, filter: i32) -> i32;
    fn kmalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn snprintf(buf: *mut i8, size: usize, fmt: *const i8, ...) -> i32;
    fn strlen(s: *const i8) -> usize;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memset(dst: *mut c_void, c: i32, n: usize) -> *mut c_void;
    fn register_trace_sys_enter(cb: unsafe extern "C" fn(*mut c_void,*mut pt_regs,i64), data: *mut c_void) -> i32;
    fn unregister_trace_sys_enter(cb: unsafe extern "C" fn(*mut c_void,*mut pt_regs,i64), data: *mut c_void);
    fn register_trace_sys_exit(cb: unsafe extern "C" fn(*mut c_void,*mut pt_regs,i64), data: *mut c_void) -> i32;
    fn unregister_trace_sys_exit(cb: unsafe extern "C" fn(*mut c_void,*mut pt_regs,i64), data: *mut c_void);
}

#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct pt_regs { _private: [u8; 0] }
#[repr(C)] pub struct trace_seq { _private: [u8; 0] }
#[repr(C)] pub struct trace_event_file { pub tr: *mut trace_array }
#[repr(C)] pub struct trace_array { pub trace_flags: u32, pub syscall_buf_sz: i32 }
#[repr(C)] pub struct trace_event_call { pub data: *mut syscall_metadata, pub print_fmt: *mut i8 }
#[repr(C)] pub struct trace_event_buffer { pub event: *mut c_void }
#[repr(C)] pub struct trace_user_buf_info { _private: [u8; 0] }
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct trace_entry { pub ty: u16 }
#[repr(C)] pub struct syscall_trace_enter { pub ent: trace_entry, pub nr: i32, pub args: [usize; 6] }
#[repr(C)] pub struct syscall_trace_exit { pub ent: trace_entry, pub nr: i32, pub ret: i64 }
#[repr(C)] pub struct syscall_metadata { pub name: *const i8, pub nb_args: i32, pub args: *mut *const i8, pub types: *mut *const i8, pub syscall_nr: i32, pub user_mask: usize, pub user_arg_size: i32, pub user_arg_is_str: bool, pub enter_event: *mut trace_event_call, pub exit_event: *mut trace_event_call }
pub type print_line_t = i32;

const EXTRA: &[u8] = b"...\0";
const SYSCALL_FAULT_MAX_CNT: usize = 3;
const SYSCALL_FAULT_ARG_SZ: usize = SYSCALL_FAULT_USER_MAX + 5;
extern "C" { static mut SYSCALL_FAULT_USER_MAX: usize; static mut current: *mut task_struct; static mut NR_syscalls: i32; }

static mut syscall_buffer: *mut syscall_user_buffer = ptr::null_mut();
#[repr(C)] struct syscall_user_buffer { buf: trace_user_buf_info, rcu: rcu_head }
#[repr(C)] struct syscall_args { ptr_array: [*mut i8; 3], read: [i32; 3], uargs: i32 }

unsafe fn syscall_nr_to_meta(nr: i32) -> *mut syscall_metadata { if nr < 0 || nr >= NR_syscalls { ptr::null_mut() } else { ptr::null_mut() } }

#[no_mangle] pub unsafe extern "C" fn get_syscall_name(nr: i32) -> *const i8 { let p=syscall_nr_to_meta(nr); if p.is_null(){ptr::null()}else{(*p).name} }

unsafe fn get_dynamic_len_ptr(trace: *mut syscall_trace_enter, entry: *mut syscall_metadata, offset: &mut i32, len: &mut i32, out: &mut *mut u8) {
    let p=(*trace).args.as_mut_ptr().cast::<u8>().add((*entry).nb_args as usize*core::mem::size_of::<usize>()).add(*offset as usize);
    let val=*(p as *const i32); *out=(trace as *mut u8).add((val&0xffff) as usize); *len=val>>16; *offset+=4;
}

unsafe extern "C" fn print_syscall_enter(_iter: *mut c_void, _flags: i32, _event: *mut c_void) -> print_line_t { 0 }
unsafe extern "C" fn print_syscall_exit(_iter: *mut c_void, _flags: i32, _event: *mut c_void) -> print_line_t { 0 }

unsafe extern "C" fn syscall_copy_user(buf:*mut i8, _ptr:*const i8, size:usize, data:*mut c_void)->i32 { let a=&mut *(data as *mut syscall_args); for i in 0..a.uargs as usize { a.read[i]=strncpy_from_user(buf.add(i*SYSCALL_FAULT_ARG_SZ),a.ptr_array[i],size); } 0 }
unsafe extern "C" fn syscall_copy_user_array(buf:*mut i8, _ptr:*const i8, size:usize, data:*mut c_void)->i32 { let a=&mut *(data as *mut syscall_args); for i in 0..a.uargs as usize { a.read[i]=if __copy_from_user(buf.add(i*SYSCALL_FAULT_ARG_SZ),a.ptr_array[i],size)!=0{-1}else{size as i32}; } 0 }

unsafe fn syscall_get_data(_m:*mut syscall_metadata,_a:*mut usize,_b:*mut *mut i8,size:*mut i32,sizes:*mut i32,uargs:*mut i32,_buf:i32)->i32 { *size=0; *uargs=0; for i in 0..3 {*sizes.add(i)=-1;} -1 }
unsafe fn syscall_put_data(_m:*mut syscall_metadata,_e:*mut syscall_trace_enter,_b:*mut i8,_size:i32,_sizes:*mut i32,_uargs:i32) {}

unsafe extern "C" fn ftrace_syscall_enter(data:*mut c_void, regs:*mut pt_regs, _id:i64) { let tr=data as *mut trace_array; let nr=syscall_get_nr(current,regs); if nr<0||nr>=NR_syscalls{return}; let m=syscall_nr_to_meta(nr); if m.is_null(){return}; let mut args=[0usize;6]; syscall_get_arguments(current,regs,args.as_mut_ptr()); let mut p=ptr::null_mut(); let mut sz=0; let mut us=[-1i32;3]; let mut ua=0; if (*m).user_mask!=0 && syscall_get_data(m,args.as_mut_ptr(),&mut p,&mut sz,us.as_mut_ptr(),&mut ua,(*tr).syscall_buf_sz)<0{return}; }
unsafe extern "C" fn ftrace_syscall_exit(_data:*mut c_void,regs:*mut pt_regs,_ret:i64) { let nr=syscall_get_nr(current,regs); if nr<0||nr>=NR_syscalls{return}; let _=syscall_get_return_value(current,regs); }

unsafe extern "C" fn syscall_enter_register(_event:*mut trace_event_call,_ty:i32,_data:*mut c_void)->i32 { 0 }
unsafe extern "C" fn syscall_exit_register(_event:*mut trace_event_call,_ty:i32,_data:*mut c_void)->i32 { 0 }

#[no_mangle] pub unsafe extern "C" fn init_ftrace_syscalls() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
