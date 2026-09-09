// SPDX-License-Identifier: BSD-3-Clause
/* Simple Landlock sandbox manager translated from sandboxer.c. */

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

// These constants and declarations are supplied by the corresponding system bindings.
#[repr(C)]
#[derive(Clone, Copy)]
struct landlock_ruleset_attr {
    handled_access_fs: u64, handled_access_net: u64, scoped: u64,
    quiet_access_fs: u64, quiet_access_net: u64, quiet_scoped: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
struct landlock_path_beneath_attr { allowed_access: u64, parent_fd: c_int }
#[repr(C)]
#[derive(Clone, Copy)]
struct landlock_net_port_attr { allowed_access: u64, port: u64 }
extern "C" {
    fn getenv(_: *const c_char) -> *mut c_char; fn unsetenv(_: *const c_char) -> c_int;
    fn strdup(_: *const c_char) -> *mut c_char; fn free(_: *mut c_void);
    fn strsep(_: *mut *mut c_char, _: *const c_char) -> *mut c_char;
    fn strtoull(_: *const c_char, _: *mut *mut c_char, _: c_int) -> u64;
    fn strcmp(_: *const c_char, _: *const c_char) -> c_int;
    fn fprintf(_: *mut c_void, _: *const c_char, ...) -> c_int;
    fn strerror(_: c_int) -> *const c_char; fn perror(_: *const c_char);
    fn open(_: *const c_char, _: c_int, ...) -> c_int; fn close(_: c_int) -> c_int;
    fn fstat(_: c_int, _: *mut stat) -> c_int; fn prctl(_: c_int, ...) -> c_int;
    fn execvpe(_: *const c_char, _: *const *mut c_char, _: *const *mut c_char) -> c_int;
    fn __errno_location() -> *mut c_int;
}
#[repr(C)] struct stat { st_mode: u32, _rest: [u8; 0] }
const STDERR: *mut c_void = 2 as *mut c_void;
const EINVAL: c_int = 22; const ENOSYS: c_int = 38; const EOPNOTSUPP: c_int = 95;
const O_PATH: c_int = 0o10000000; const O_CLOEXEC: c_int = 0o2000000;
const LANDLOCK_RULE_PATH_BENEATH: u32 = 1; const LANDLOCK_RULE_NET_PORT: u32 = 2;
const LANDLOCK_CREATE_RULESET_VERSION: u32 = 1; const LANDLOCK_ADD_RULE_QUIET: u32 = 1;
const LANDLOCK_ACCESS_FS_EXECUTE:u64=1<<0; const LANDLOCK_ACCESS_FS_WRITE_FILE:u64=1<<1;
const LANDLOCK_ACCESS_FS_READ_FILE:u64=1<<2; const LANDLOCK_ACCESS_FS_READ_DIR:u64=1<<3;
const LANDLOCK_ACCESS_FS_REMOVE_DIR:u64=1<<4; const LANDLOCK_ACCESS_FS_REMOVE_FILE:u64=1<<5;
const LANDLOCK_ACCESS_FS_MAKE_CHAR:u64=1<<6; const LANDLOCK_ACCESS_FS_MAKE_DIR:u64=1<<7;
const LANDLOCK_ACCESS_FS_MAKE_REG:u64=1<<8; const LANDLOCK_ACCESS_FS_MAKE_SOCK:u64=1<<9;
const LANDLOCK_ACCESS_FS_MAKE_FIFO:u64=1<<10; const LANDLOCK_ACCESS_FS_MAKE_BLOCK:u64=1<<11;
const LANDLOCK_ACCESS_FS_MAKE_SYM:u64=1<<12; const LANDLOCK_ACCESS_FS_REFER:u64=1<<13;
const LANDLOCK_ACCESS_FS_TRUNCATE:u64=1<<14; const LANDLOCK_ACCESS_FS_IOCTL_DEV:u64=1<<15;
const LANDLOCK_ACCESS_FS_RESOLVE_UNIX:u64=1<<16; const LANDLOCK_ACCESS_NET_BIND_TCP:u64=1<<0;
const LANDLOCK_ACCESS_NET_CONNECT_TCP:u64=1<<1; const LANDLOCK_ACCESS_NET_BIND_UDP:u64=1<<2;
const LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP:u64=1<<3; const LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET:u64=1<<0;
const LANDLOCK_SCOPE_SIGNAL:u64=1<<1; const LANDLOCK_RESTRICT_SELF_LOG_NEW_EXEC_ON:i32=1;
const LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS:i32=2; const PR_SET_NO_NEW_PRIVS:c_int=38;
const ACCESS_FILE:u64=LANDLOCK_ACCESS_FS_EXECUTE|LANDLOCK_ACCESS_FS_WRITE_FILE|LANDLOCK_ACCESS_FS_READ_FILE|LANDLOCK_ACCESS_FS_TRUNCATE|LANDLOCK_ACCESS_FS_IOCTL_DEV|LANDLOCK_ACCESS_FS_RESOLVE_UNIX;
const ACCESS_FS_ROUGHLY_READ:u64=LANDLOCK_ACCESS_FS_EXECUTE|LANDLOCK_ACCESS_FS_READ_FILE|LANDLOCK_ACCESS_FS_READ_DIR;
const ACCESS_FS_ROUGHLY_WRITE:u64=LANDLOCK_ACCESS_FS_WRITE_FILE|LANDLOCK_ACCESS_FS_REMOVE_DIR|LANDLOCK_ACCESS_FS_REMOVE_FILE|LANDLOCK_ACCESS_FS_MAKE_CHAR|LANDLOCK_ACCESS_FS_MAKE_DIR|LANDLOCK_ACCESS_FS_MAKE_REG|LANDLOCK_ACCESS_FS_MAKE_SOCK|LANDLOCK_ACCESS_FS_MAKE_FIFO|LANDLOCK_ACCESS_FS_MAKE_BLOCK|LANDLOCK_ACCESS_FS_MAKE_SYM|LANDLOCK_ACCESS_FS_REFER|LANDLOCK_ACCESS_FS_TRUNCATE|LANDLOCK_ACCESS_FS_IOCTL_DEV|LANDLOCK_ACCESS_FS_RESOLVE_UNIX;
const ENV_DELIMITER: &[u8] = b":";
macro_rules! cs { ($s:literal) => { concat!($s, "\0").as_ptr() as *const c_char }; }
unsafe fn env(name: *const c_char) -> *mut c_char { getenv(name) }
unsafe fn str2num(s:*const c_char, dst:*mut u64)->c_int { let mut e=ptr::null_mut(); let n=strtoull(s,&mut e,10); if e==s || *e!=0 {EINVAL} else {*dst=n;0} }
unsafe fn parse_path(mut p:*mut c_char, out:*mut *mut *const c_char)->c_int { let mut n=0; if !p.is_null(){n=1;let mut q=p;while *q!=0{if *q==b':' as c_char{n+=1;}q=q.add(1);}} let a=libc_malloc((n as usize)*std::mem::size_of::<*const c_char>()) as *mut *const c_char;if a.is_null(){return -1;}for i in 0..n{*a.add(i as usize)=strsep(&mut p,ENV_DELIMITER.as_ptr() as *const c_char);}*out=a;n }
unsafe fn libc_malloc(n:usize)->*mut c_void { extern "C"{fn malloc(_:usize)->*mut c_void} malloc(n) }
unsafe fn add_rule(fd:c_int,ty:u32,attr:*const c_void,flags:u32)->c_int { extern "C"{fn landlock_add_rule(c_int,u32,*const c_void,u32)->c_int} landlock_add_rule(fd,ty,attr,flags) }
unsafe fn create(attr:*const landlock_ruleset_attr,size:usize,flags:u32)->c_int { extern "C"{fn landlock_create_ruleset(*const landlock_ruleset_attr,usize,u32)->c_int} landlock_create_ruleset(attr,size,flags) }
unsafe fn restrict(fd:c_int,flags:i32)->c_int { extern "C"{fn landlock_restrict_self(c_int,i32)->c_int} landlock_restrict_self(fd,flags) }

unsafe fn populate_fs(name:*const c_char,fd:c_int,access:u64,flags:u32)->c_int { let p=getenv(name);if p.is_null(){fprintf(STDERR,cs!("Missing environment variable %s\n"),name);return 1;}let copy=strdup(p);unsetenv(name);let mut list=ptr::null_mut();let n=parse_path(copy,&mut list);if n<0{return 1;}if n==1&&**list==0{free(list as *mut c_void);free(copy as *mut c_void);return 0;}for i in 0..n{let mut a=landlock_path_beneath_attr{allowed_access:access,parent_fd:open(*list.add(i as usize),O_PATH|O_CLOEXEC)};if a.parent_fd<0{continue;}let mut st=stat{st_mode:0,_rest:[]};if fstat(a.parent_fd,&mut st)!=0{close(a.parent_fd);free(list as *mut c_void);free(copy as *mut c_void);return 1;}if st.st_mode&0o170000 != 0o040000{a.allowed_access&=ACCESS_FILE;}if add_rule(fd,LANDLOCK_RULE_PATH_BENEATH,&a as *const _ as *const c_void,flags)!=0{close(a.parent_fd);free(list as *mut c_void);free(copy as *mut c_void);return 1;}close(a.parent_fd);}free(list as *mut c_void);free(copy as *mut c_void);0 }
unsafe fn populate_net(name:*const c_char,fd:c_int,access:u64,flags:u32)->c_int {let p=getenv(name);if p.is_null(){return 0;}let copy=strdup(p);unsetenv(name);let mut next=copy;while{let s=strsep(&mut next,ENV_DELIMITER.as_ptr() as *const c_char);if s.is_null(){false}else{if *s!=0{let mut port=0;if str2num(s,&mut port)!=0{free(copy as *mut c_void);return 1;}let a=landlock_net_port_attr{allowed_access:access,port};if add_rule(fd,LANDLOCK_RULE_NET_PORT,&a as *const _ as *const c_void,flags)!=0{free(copy as *mut c_void);return 1;}}true}}{}free(copy as *mut c_void);0}

unsafe fn check_scope(name:*const c_char, a:*mut landlock_ruleset_attr)->bool { let mut aa=false;let mut ss=false;let p=getenv(name);if p.is_null(){unsetenv(name);return false;}let cp=strdup(p);let mut next=cp;while{let s=strsep(&mut next,ENV_DELIMITER.as_ptr() as *const c_char);if s.is_null(){false}else{if strcmp(s,cs!("a"))==0&&!aa{aa=true}else if strcmp(s,cs!("s"))==0&&!ss{ss=true}else{free(cp as *mut c_void);return true;}true}}{}free(cp as *mut c_void);if !aa{(*a).scoped&=!LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET}if !ss{(*a).scoped&=!LANDLOCK_SCOPE_SIGNAL}unsetenv(name);false }

pub unsafe fn main_impl(argc:c_int,argv:*mut *mut c_char,envp:*mut *mut c_char)->c_int {
    if argc<2 { fprintf(STDERR,cs!("usage: LL_FS_RO=\"...\" LL_FS_RW=\"...\" %s <cmd> [args]...\n"),*argv);return 1; }
    let mut attr=landlock_ruleset_attr{handled_access_fs:ACCESS_FS_ROUGHLY_READ|ACCESS_FS_ROUGHLY_WRITE,handled_access_net:LANDLOCK_ACCESS_NET_BIND_TCP|LANDLOCK_ACCESS_NET_CONNECT_TCP|LANDLOCK_ACCESS_NET_BIND_UDP|LANDLOCK_ACCESS_NET_CONNECT_SEND_UDP,scoped:LANDLOCK_SCOPE_ABSTRACT_UNIX_SOCKET|LANDLOCK_SCOPE_SIGNAL,quiet_access_fs:0,quiet_access_net:0,quiet_scoped:0};
    let abi=create(ptr::null(),0,LANDLOCK_CREATE_RULESET_VERSION);if abi<0{perror(cs!("Failed to check Landlock compatibility"));return 1;}
    if check_scope(cs!("LL_SCOPED"),&mut attr){return 1;}
    let fd=create(&attr,std::mem::size_of::<landlock_ruleset_attr>(),0);if fd<0{perror(cs!("Failed to create a ruleset"));return 1;}
    if populate_fs(cs!("LL_FS_RO"),fd,ACCESS_FS_ROUGHLY_READ,0)!=0||populate_fs(cs!("LL_FS_RW"),fd,ACCESS_FS_ROUGHLY_READ|ACCESS_FS_ROUGHLY_WRITE,0)!=0{close(fd);return 1;}
    if restrict(fd,LANDLOCK_RESTRICT_SELF_NO_NEW_PRIVS)!=0{perror(cs!("Failed to enforce ruleset"));close(fd);return 1;}close(fd);
    let cmd=*argv.add(1);execvpe(cmd,argv.add(1),envp);fprintf(STDERR,cs!("Failed to execute \"%s\"\n"),cmd);1
}

fn main() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
