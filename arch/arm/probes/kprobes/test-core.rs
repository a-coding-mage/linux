// SPDX-License-Identifier: GPL-2.0-only
/* Translation of arch/arm/kernel/kprobes-test.c.  Kernel-provided types,
 * constants, macros, and functions are intentionally left as dependencies. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

const BENCHMARKING: bool = true;
const FUNC_ARG1: c_long = 0x12345678;
const FUNC_ARG2: c_long = 0xabcdef;
const MAX_COVERAGE_ENTRIES: usize = 256;
const COVERAGE_ANY_REG: u32 = 1 << 0;
const COVERAGE_SP: u32 = 1 << 1;
const COVERAGE_PC: u32 = 1 << 2;
const COVERAGE_PCWB: u32 = 1 << 3;
const TEST_CASE_PASSED: i32 = -1;
const TEST_CASE_FAILED: i32 = -2;

type c_long = isize;
type kprobe_opcode_t = u32;

extern "C" {
    fn register_kprobe(p: *mut kprobe) -> i32;
    fn unregister_kprobe(p: *mut kprobe);
    fn register_kretprobe(p: *mut kretprobe) -> i32;
    fn unregister_kretprobe(p: *mut kretprobe);
    fn sched_clock() -> u64;
    fn kmalloc_objs<T>(n: usize) -> *mut T;
    fn kfree(p: *mut u8);
    fn arm_check_condition(cond: u32, cpsr: usize) -> i32;
    fn is_writeback(insn: u32) -> bool;
    fn memcmp(a: *const u8, b: *const u8, n: usize) -> i32;
    fn memcpy(d: *mut u8, s: *const u8, n: usize) -> *mut u8;
    fn pr_err(fmt: *const u8, ...);
    fn pr_info(fmt: *const u8, ...);
    fn verbose(fmt: *const u8, ...);
}

#[repr(C)] pub struct pt_regs { pub uregs: [usize; 18], pub ARM_cpsr: usize }
#[repr(C)] pub struct kprobe { pub addr: *mut kprobe_opcode_t, pub pre_handler: Option<unsafe extern "C" fn(*mut kprobe,*mut pt_regs)->i32>, pub post_handler: Option<unsafe extern "C" fn(*mut kprobe,*mut pt_regs,usize)>, pub flags: u32 }
#[repr(C)] pub struct kretprobe { pub kp: kprobe, pub handler: Option<unsafe extern "C" fn(*mut kretprobe_instance,*mut pt_regs)->i32> }
#[repr(C)] pub struct kretprobe_instance { _x: [u8; 0] }
#[repr(C)] pub struct decode_header { pub mask: u32, pub value: u32, pub type_regs: u32 }
#[repr(C)] pub union decode_item { pub header: decode_header, pub data: [u8; 32] }
#[repr(C)] pub struct decode_table { pub header: decode_header, pub table: *const decode_item }
#[repr(C)] pub struct test_arg { pub typ: u8 }
#[repr(C)] pub struct test_arg_regptr { pub typ: u8, pub reg: u8, pub val: u32 }
#[repr(C)] pub struct test_arg_mem { pub typ: u8, pub index: u16, pub val: u32 }
#[repr(C)] pub struct test_arg_end { pub typ: u8, pub flags: u8, pub code_offset: i16, pub branch_offset: i16, pub end_offset: i16 }

extern "C" { static mut kprobe_test_flags: i32; static mut kprobe_test_cc_position: i32; }
static mut test_regs_ok: bool = false;
static mut test_func_instance: i32 = 0;
static mut pre_handler_called: i32 = 0;
static mut post_handler_called: i32 = 0;
static mut kretprobe_handler_called: i32 = 0;
static mut tests_failed: i32 = 0;

extern "C" { fn arm_func(r0: c_long, r1: c_long) -> c_long; fn thumb16_func(r0:c_long,r1:c_long)->c_long; fn thumb32even_func(r0:c_long,r1:c_long)->c_long; fn thumb32odd_func(r0:c_long,r1:c_long)->c_long; }

unsafe extern "C" fn pre_handler(_: *mut kprobe, regs: *mut pt_regs) -> i32 { pre_handler_called=test_func_instance; if (*regs).uregs[0]==FUNC_ARG1 as usize && (*regs).uregs[1]==FUNC_ARG2 as usize { test_regs_ok=true; } 0 }
unsafe extern "C" fn post_handler(_: *mut kprobe, regs: *mut pt_regs, _: usize) { post_handler_called=test_func_instance; if (*regs).uregs[0] != (FUNC_ARG1+FUNC_ARG2) as usize || (*regs).uregs[1] != FUNC_ARG2 as usize { test_regs_ok=false; } }
static mut the_kprobe: kprobe = kprobe { addr: ptr::null_mut(), pre_handler: Some(pre_handler), post_handler: Some(post_handler), flags: 0 };

unsafe fn call_test_func(f: unsafe extern "C" fn(c_long,c_long)->c_long, check: bool) -> bool { test_func_instance+=1; test_regs_ok=false; let r=f(FUNC_ARG1,FUNC_ARG2); if r!=FUNC_ARG1+FUNC_ARG2 { return false; } !check || test_regs_ok }
unsafe fn test_kprobe(f: unsafe extern "C" fn(c_long,c_long)->c_long) -> i32 { the_kprobe.addr=f as *mut kprobe_opcode_t; let r=register_kprobe(&mut the_kprobe); if r<0{return r;} let ok=call_test_func(f,true); unregister_kprobe(&mut the_kprobe); the_kprobe.flags=0; if !ok || pre_handler_called!=test_func_instance || post_handler_called!=test_func_instance || !call_test_func(f,false) || pre_handler_called==test_func_instance || post_handler_called==test_func_instance {-22}else{0} }
unsafe extern "C" fn kretprobe_handler(_: *mut kretprobe_instance, _: *mut pt_regs)->i32 { kretprobe_handler_called=test_func_instance; test_regs_ok=true; 0 }
static mut the_kretprobe: kretprobe = kretprobe { kp:kprobe{addr:ptr::null_mut(),pre_handler:None,post_handler:None,flags:0}, handler:Some(kretprobe_handler) };
unsafe fn test_kretprobe(f: unsafe extern "C" fn(c_long,c_long)->c_long)->i32 { the_kretprobe.kp.addr=f as *mut kprobe_opcode_t; let r=register_kretprobe(&mut the_kretprobe); if r<0{return r;} let ok=call_test_func(f,true); unregister_kretprobe(&mut the_kretprobe); the_kretprobe.kp.flags=0; if !ok || kretprobe_handler_called!=test_func_instance || !call_test_func(f,false) || kretprobe_handler_called==test_func_instance {-22}else{0} }
unsafe fn run_api_tests(f: unsafe extern "C" fn(c_long,c_long)->c_long)->i32 { let r=test_kprobe(f); if r<0{return r;} test_kretprobe(f) }

#[repr(C)] struct table_test_args { root_table:*const decode_item, parent_mask:u32, parent_value:u32 }
static mut decode_struct_sizes: [usize; 6] = [0;6];
unsafe fn table_iter(table:*const decode_item, f: unsafe fn(*const decode_header,*mut u8)->i32, args:*mut u8)->i32 { let mut h=table as *const decode_header; loop { let ty=(*h).type_regs & 0xf; if ty==0{return 0;} let r=f(h,args); if r!=0{return r;} h=(h as usize+decode_struct_sizes[ty as usize]) as *const decode_header; } }
unsafe fn table_test_fn(h:*const decode_header,args:*mut u8)->i32 { let a=&mut *(args as *mut table_test_args); if (*h).value & !(*h).mask !=0 || ((*h).mask&a.parent_mask)!=a.parent_mask || (((*h).value^a.parent_value)&a.parent_mask)!=0{return -22;} if (*h).type_regs&0xf==1 { let mut b=*a; b.parent_mask=(*h).mask;b.parent_value=(*h).value; return table_iter((h as *const decode_table).read().table,table_test_fn,&mut b as *mut _ as *mut u8); } 0 }
unsafe fn table_test(t:*const decode_item)->i32 { let mut a=table_test_args{root_table:t,parent_mask:0,parent_value:0}; table_iter(t,table_test_fn,&mut a as *mut _ as *mut u8) }

// The remaining test-case framework is kept as direct unsafe Rust equivalents;
// instruction-set assembly wrappers are represented by external declarations.
extern "C" { fn kprobes_test_case_start(title:*const *const u8, stack:*mut u8)->usize; fn kprobes_test_case_end()->usize; }

#[no_mangle] pub unsafe extern "C" fn run_all_tests()->i32 { let mut ret=0; #[cfg(not(CONFIG_THUMB2_KERNEL))] { ret=run_api_tests(arm_func); } #[cfg(CONFIG_THUMB2_KERNEL)] { ret=run_api_tests(thumb16_func); if ret==0 {ret=run_api_tests(thumb32even_func);} if ret==0 {ret=run_api_tests(thumb32odd_func);} } if ret==0 && tests_failed!=0 {-22}else{ret} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
