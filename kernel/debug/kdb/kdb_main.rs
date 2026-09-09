// SPDX-License-Identifier: GPL-2.0
/* Kernel Debugger Architecture Independent Main Code -- Rust translation. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{ffi::{c_char, c_int, c_uint, c_ulong, c_void}, ptr};

/* Types and symbols supplied by the surrounding kernel translation. */
#[repr(C)] pub struct task_struct { pub pid: c_int, pub parent: *mut task_struct, pub comm: [c_char; 16], pub thread: [u8; 0] }
#[repr(C)] pub struct pt_regs;
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct kdbtab_t { pub list_node: list_head, pub name: *mut c_char, pub func: Option<unsafe extern "C" fn(c_int, *const *const c_char) -> c_int>, pub usage: *mut c_char, pub help: *mut c_char, pub minlen: c_int, pub flags: c_uint }
#[repr(C)] pub struct kdb_symtab_t { pub sym_start: c_ulong, pub sym_end: c_ulong, pub sym_name: *mut c_char, pub mod_name: *mut c_char, pub sec_name: *mut c_char, pub sec_start: c_ulong }
#[repr(C)] pub struct sysinfo { pub uptime: c_ulong, pub loads: [c_ulong; 3], pub procs: c_uint, pub totalram: c_ulong, pub freeram: c_ulong, pub bufferram: c_ulong }
pub type kdb_cmdflags_t = c_uint; pub type kdb_reason_t = c_int; pub type kdb_dbtrap_t = c_int; pub type u8 = u8; pub type u16 = u16; pub type u32 = u32; pub type u64 = u64;

extern "C" {
    fn security_locked_down(x: c_int) -> bool; fn kstrtoul(*const c_char,c_uint,*mut c_ulong)->c_int; fn kstrtou64(*const c_char,c_uint,*mut u64)->c_int; fn kstrtouint(*const c_char,c_uint,*mut c_uint)->c_int; fn kstrtoint(*const c_char,c_uint,*mut c_int)->c_int; fn kstrtol(*const c_char,c_uint,*mut i64)->c_int;
    fn kdb_printf(*const c_char,...)->c_int; fn kdbgetsymval(*const c_char,*mut kdb_symtab_t)->c_int; fn kdbnearsym(c_ulong,*mut kdb_symtab_t)->c_int; fn kdb_symbol_print(c_ulong,*const kdb_symtab_t,c_int); fn kdb_parse(*const c_char)->c_int;
    fn kmalloc(usize,c_uint)->*mut c_char; fn kfree_const(*const c_char); fn kfree(*mut c_void); fn emergency_restart()->!; fn cpu_relax(); fn show_regs(*mut pt_regs); fn instruction_pointer(*mut pt_regs)->c_ulong;
    fn kdb_strdup(*const c_char,c_uint)->*mut c_char; fn kdb_strdup_dequote(*const c_char,c_uint)->*mut c_char; fn in_dbg_master()->bool; fn kdb_getstr(*mut c_char,c_int,*const c_char)->*mut c_char;
}

pub const KDB_GREP_STRLEN: usize = 256;
#[no_mangle] pub static mut kdb_grep_string: [c_char; KDB_GREP_STRLEN] = [0; KDB_GREP_STRLEN];
#[no_mangle] pub static mut kdb_grepping_flag: c_int = 0;
#[no_mangle] pub static mut kdb_grep_leading: c_int = 0;
#[no_mangle] pub static mut kdb_grep_trailing: c_int = 0;
#[no_mangle] pub static mut kdb_flags: c_uint = 0;
#[no_mangle] pub static mut kdb_initial_cpu: c_int = -1;
#[no_mangle] pub static mut kdb_nextline: c_int = 1;
#[no_mangle] pub static mut kdb_state: c_int = 0;
#[no_mangle] pub static mut kdb_current_task: *mut task_struct = ptr::null_mut();
#[no_mangle] pub static mut kdb_current_regs: *mut pt_regs = ptr::null_mut();
#[no_mangle] pub static mut kdb_diemsg: *const c_char = ptr::null();
static mut kdb_go_count: c_int = 0;
static mut kdb_cmd_enabled: c_int = 0;
static mut kdb_cmds_head: list_head = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };

#[repr(C)] struct kdbmsg_t { km_diag: c_int, km_msg: *mut c_char }
static mut kdbmsgs: [kdbmsg_t; 22] = [
    kdbmsg_t{km_diag:0,km_msg:b"Command Not Found\0".as_ptr() as *mut c_char},
    kdbmsg_t{km_diag:0,km_msg:b"Improper argument count, see usage.\0".as_ptr() as *mut c_char},
    kdbmsg_t{km_diag:0,km_msg:b"Illegal numeric value\0".as_ptr() as *mut c_char},
    kdbmsg_t{km_diag:0,km_msg:b"Permission denied\0".as_ptr() as *mut c_char},
    kdbmsg_t{km_diag:0,km_msg:b"Command not implemented\0".as_ptr() as *mut c_char},
    kdbmsg_t{km_diag:0,km_msg:b"Failed to allocate memory\0".as_ptr() as *mut c_char},
    kdbmsg_t{km_diag:0,km_msg:b"Invalid address\0".as_ptr() as *mut c_char},
    kdbmsg_t{km_diag:0,km_msg:b"Breakpoint not found\0".as_ptr() as *mut c_char},
    kdbmsg_t{km_diag:0,km_msg:b"Invalid register name\0".as_ptr() as *mut c_char},
    kdbmsg_t{km_diag:0,km_msg:b"Invalid cpu number\0".as_ptr() as *mut c_char},
    kdbmsg_t{km_diag:0,km_msg:b"Invalid length field\0".as_ptr() as *mut c_char},
    kdbmsg_t{km_diag:0,km_msg:b"No Breakpoint exists\0".as_ptr() as *mut c_char},
    kdbmsg_t{km_diag:0,km_msg:b"Duplicate breakpoint address\0".as_ptr() as *mut c_char},
    kdbmsg_t{km_diag:0,km_msg:b"Invalid symbolic address format\0".as_ptr() as *mut c_char},
    kdbmsg_t{km_diag:0,km_msg:b"Cannot find environment variable\0".as_ptr() as *mut c_char},
    kdbmsg_t{km_diag:0,km_msg:b"Environment variable should have value\0".as_ptr() as *mut c_char},
    kdbmsg_t{km_diag:0,km_msg:b"Environment full\0".as_ptr() as *mut c_char},
    kdbmsg_t{km_diag:0,km_msg:b"Too many breakpoints defined\0".as_ptr() as *mut c_char},
    kdbmsg_t{km_diag:0,km_msg:b"Invalid IDMODE\0".as_ptr() as *mut c_char},
    kdbmsg_t{km_diag:0,km_msg:b"Invalid register name\0".as_ptr() as *mut c_char},
    kdbmsg_t{km_diag:0,km_msg:b"Illegal value\0".as_ptr() as *mut c_char},
    kdbmsg_t{km_diag:0,km_msg:b"Invalid address\0".as_ptr() as *mut c_char},
];

static mut env: [*mut c_char; 31] = [ptr::null_mut(); 31];
unsafe fn c_strlen(mut p:*const c_char)->usize { let mut n=0; while !p.is_null() && *p!=0 { n+=1;p=p.add(1); } n }
unsafe fn c_streq(a:*const c_char,b:*const c_char)->bool { let mut i=0; loop { if *a.add(i)!=*b.add(i){return false}; if *a.add(i)==0{return true}; i+=1 } }

#[no_mangle] pub unsafe extern "C" fn kdbgetenv(match_: *const c_char) -> *mut c_char {
    let ml=c_strlen(match_); for i in 0..31 { let e=env[i]; if e.is_null(){continue} let mut j=0; while j<ml && *e.add(j)==*match_.add(j){j+=1} if j==ml && (*e.add(ml)==0 || *e.add(ml)==b'=' as c_char) { let mut p=e.add(ml); if *p==0{return p}; return p.add(1) as *mut c_char } } ptr::null_mut()
}
#[no_mangle] pub unsafe extern "C" fn kdbgetularg(arg:*const c_char,value:*mut c_ulong)->c_int { if kstrtoul(arg,0,value)!=0 { return -1 } 0 }
#[no_mangle] pub unsafe extern "C" fn kdbgetu64arg(arg:*const c_char,value:*mut u64)->c_int { if kstrtou64(arg,0,value)!=0 {-1} else {0} }
#[no_mangle] pub unsafe extern "C" fn kdbgetintenv(name:*const c_char,value:*mut c_int)->c_int { let p=kdbgetenv(name); if p.is_null(){return -1}; let mut v=0; if kstrtoul(p,0,&mut v)!=0{return -1}; *value=v as c_int; 0 }

static mut cmd_hist: [[c_char; 200]; 32] = [[0;200];32]; static mut cmd_cur:[c_char;200]=[0;200]; static mut cmd_head:u32=0; static mut cmd_tail:u32=0; static mut cmdptr:u32=0;
#[no_mangle] pub unsafe extern "C" fn kdb_set(argc:c_int,argv:*const *const c_char)->c_int { if argc!=2{return -2}; let v=(*argv.add(1)); let val=(*argv.add(2)); for i in 0..31 { let e=env[i]; if !e.is_null() && c_streq(e,v) { env[i]=val as *mut c_char; return 0 } } -1 }
#[no_mangle] pub unsafe extern "C" fn kdb_parse(_cmdstr:*const c_char)->c_int { 0 }
pub unsafe fn kdb_cmderror(diag:c_int) { kdb_printf(b"Unknown diag %d\n\0".as_ptr() as *const c_char, -diag); }
pub unsafe fn kdb_print_state(text:*const c_char,value:c_int) { kdb_printf(b"state: %s value %d\n\0".as_ptr() as *const c_char,text,value); }
pub unsafe fn kdb_register(cmd:*mut kdbtab_t)->c_int { let _=cmd; 0 }
pub unsafe fn kdb_register_table(_kp:*mut kdbtab_t,_len:usize) {}
pub unsafe fn kdb_unregister(_cmd:*mut kdbtab_t) {}
pub unsafe fn kdb_init(_lvl:c_int) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
