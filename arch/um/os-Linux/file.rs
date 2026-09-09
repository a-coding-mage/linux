// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com) */

use std::ffi::c_void;
use std::os::raw::{c_char, c_int, c_long, c_uint, c_ulong, c_ulonglong};

#[repr(C)] pub struct uml_stat { pub ust_dev:u64, pub ust_ino:u64, pub ust_mode:u32, pub ust_nlink:u64, pub ust_uid:u32, pub ust_gid:u32, pub ust_size:i64, pub ust_blksize:i64, pub ust_blocks:i64, pub ust_atime:i64, pub ust_mtime:i64, pub ust_ctime:i64 }
#[repr(C)] pub struct openflags { pub r:bool, pub w:bool, pub s:bool, pub c:bool, pub t:bool, pub e:bool, pub a:bool, pub cl:bool }
#[repr(C)] struct stat64 { st_dev:u64, st_ino:u64, st_mode:u32, st_nlink:u64, st_uid:u32, st_gid:u32, st_size:i64, st_blksize:i64, st_blocks:i64, st_atime:i64, st_mtime:i64, st_ctime:i64 }
#[repr(C)] struct sockaddr_un { sun_family:u16, sun_path:[c_char;108] }
#[repr(C)] struct iovec { iov_base:*mut c_void, iov_len:usize }
#[repr(C)] struct msghdr { msg_name:*mut c_void, msg_namelen:u32, msg_iov:*mut iovec, msg_iovlen:usize, msg_control:*mut c_void, msg_controllen:usize, msg_flags:c_int }
#[repr(C)] struct cmsghdr { cmsg_len:usize, cmsg_level:c_int, cmsg_type:c_int }
#[repr(C)] struct flock { l_type:c_short, l_whence:c_short, l_start:i64, l_len:i64, l_pid:c_int }
#[repr(C)] struct pollfd { fd:c_int, events:c_short, revents:c_short }
type c_short=i16;

extern "C" {
    fn fstat64(c_int,*mut stat64)->c_int; fn stat64(*const c_char,*mut stat64)->c_int;
    fn access(*const c_char,c_int)->c_int; fn ioctl(c_int,c_ulong,...)->c_int;
    fn fchmod(c_int,c_uint)->c_int; fn open64(*const c_char,c_int,...)->c_int; fn close(c_int)->c_int;
    fn fcntl(c_int,c_int,...)->c_int; fn socket(c_int,c_int,c_int)->c_int; fn connect(c_int,*const c_void,u32)->c_int;
    fn dup(c_int)->c_int; fn lseek64(c_int,u64,c_int)->u64; fn read(c_int,*mut c_void,usize)->isize; fn write(c_int,*const c_void,usize)->isize;
    fn pread(c_int,*mut c_void,usize,u64)->isize; fn pwrite(c_int,*const c_void,usize,u64)->isize; fn fdatasync(c_int)->c_int;
    fn shutdown(c_int,c_int)->c_int; fn recvmsg(c_int,*mut msghdr,c_int)->isize; fn sendmsg(c_int,*const msghdr,c_int)->c_int;
    fn socketpair(c_int,c_int,c_int,*mut c_int)->c_int; fn accept(c_int,*mut c_void,*mut u32)->c_int; fn bind(c_int,*const c_void,u32)->c_int;
    fn fflush(*mut c_void)->c_int; fn fallocate(c_int,c_int,u64,i64)->c_int; fn eventfd(c_uint,c_int)->c_int;
    fn poll(*mut pollfd,usize,c_int)->c_int; fn mmap(*mut c_void,usize,c_int,c_int,c_int,i64)->*mut c_void; fn mremap(*mut c_void,usize,usize,c_int,...)->*mut c_void;
    fn __errno_location()->*mut c_int; fn os_getpid()->c_int; fn printk(*const c_char,...)->c_int;
    fn of_read(openflags)->openflags; fn of_write(openflags)->openflags;
}
unsafe fn errno()->c_int { *__errno_location() }
unsafe fn copy_stat(d:&mut uml_stat,s:&stat64){ *d=uml_stat{ust_dev:s.st_dev,ust_ino:s.st_ino,ust_mode:s.st_mode,ust_nlink:s.st_nlink,ust_uid:s.st_uid,ust_gid:s.st_gid,ust_size:s.st_size,ust_blksize:s.st_blksize,ust_blocks:s.st_blocks,ust_atime:s.st_atime,ust_mtime:s.st_mtime,ust_ctime:s.st_ctime}; }
pub unsafe fn os_stat_fd(fd:c_int,ubuf:*mut uml_stat)->c_int { let mut s=std::mem::zeroed(); let e=fstat64(fd,&mut s); if e<0{-errno()}else{if !ubuf.is_null(){copy_stat(&mut*ubuf,&s)} e} }
pub unsafe fn os_stat_file(f:*const c_char,u:*mut uml_stat)->c_int { let mut s=std::mem::zeroed(); let e=stat64(f,&mut s); if e<0{-errno()}else{if !u.is_null(){copy_stat(&mut*u,&s)} e} }
pub unsafe fn os_access(f:*const c_char,m:c_int)->c_int { let e=access(f,m);if e<0{-errno()}else{0} }
pub unsafe fn os_ioctl_generic(fd:c_int,cmd:c_uint,arg:c_ulong)->c_int {let e=ioctl(fd,cmd as c_ulong,arg);if e<0{-errno()}else{e}}
pub unsafe fn os_get_ifname(fd:c_int,n:*mut c_char)->c_int {if ioctl(fd,0x8910,n)<0{-errno()}else{0}}
pub unsafe fn os_mode_fd(fd:c_int,m:c_int)->c_int {let e=fchmod(fd,m as c_uint);if e<0{-errno()}else{0}}
pub unsafe fn os_file_type(f:*mut c_char)->c_int {let mut b=std::mem::zeroed();let e=os_stat_file(f,&mut b);if e<0{return e} let m=b.ust_mode;if m&0xf000==0x4000{1}else if m&0xf000==0xa000{2}else if m&0xf000==0x2000{3}else if m&0xf000==0x6000{4}else if m&0xf000==0x1000{5}else if m&0xc000==0xc000{6}else{0}}
pub unsafe fn os_file_mode(f:*const c_char,o:*mut openflags)->c_int {let mut e=access(f,2);if e!=0&&errno()!=13{return -errno()}if e==0{*o=of_write(*o)}e=access(f,4);if e!=0&&errno()!=13{return -errno()}if e==0{*o=of_read(*o)}e}
pub unsafe fn os_open_file(f:*const c_char,fl:openflags,m:c_int)->c_int {let mut x=if fl.r&&fl.w{2}else if fl.r{0}else if fl.w{1}else{0};if fl.s{x|=0x101000}else{ };if fl.c{x|=64};if fl.t{x|=512};if fl.e{x|=128};if fl.a{x|=1024};let fd=open64(f,x,m);if fd<0{return -errno()}if fl.cl&&fcntl(fd,2,1)!=0{let e=-errno();close(fd);return e}fd}
pub unsafe fn os_connect_socket(n:*const c_char)->c_int {let mut s=sockaddr_un{sun_family:1,sun_path:[0;108]};let mut fd=socket(1,1,0);if fd<0{return -errno()}if connect(fd,&s as*const _ as*const c_void,110)!=0{let e=-errno();close(fd);return e}fd}
pub unsafe fn os_dup_file(fd:c_int)->c_int {let n=dup(fd);if n<0{-errno()}else{n}} pub unsafe fn os_close_file(fd:c_int){close(fd)}
pub unsafe fn os_seek_file(fd:c_int,o:u64)->c_int {if lseek64(fd,o,0)!=o{-errno()}else{0}}
pub unsafe fn os_read_file(fd:c_int,b:*mut c_void,l:c_int)->c_int {let n=read(fd,b,l as usize);if n<0{-errno()}else{n as c_int}} pub unsafe fn os_pread_file(fd:c_int,b:*mut c_void,l:c_int,o:u64)->c_int{let n=pread(fd,b,l as usize,o);if n<0{-errno()}else{n as c_int}}
pub unsafe fn os_write_file(fd:c_int,b:*const c_void,l:c_int)->c_int{let n=write(fd,b,l as usize);if n<0{-errno()}else{n as c_int}} pub unsafe fn os_sync_file(fd:c_int)->c_int{let n=fdatasync(fd);if n<0{-errno()}else{n}}
pub unsafe fn os_pwrite_file(fd:c_int,b:*const c_void,l:c_int,o:u64)->c_int{let n=pwrite(fd,b,l as usize,o);if n<0{-errno()}else{n as c_int}}
pub unsafe fn os_set_exec_close(fd:c_int)->c_int{let e=fcntl(fd,2,1);if e<0{-errno()}else{e}}
pub unsafe fn os_pipe(f:*mut c_int,stream:c_int,cl:c_int)->c_int{let e=socketpair(1,if stream!=0{1}else{2},0,f);if e<0{return -errno()}if cl==0{return 0}let e=os_set_exec_close(*f);if e<0{close(*f.add(1));close(*f);return e}let e=os_set_exec_close(*f.add(1));if e<0{close(*f.add(1));close(*f)}e}
pub unsafe fn os_set_fd_block(fd:c_int,b:c_int)->c_int{let mut x=fcntl(fd,3);if x<0{return -errno()}if b!=0{x&=!2048}else{x|=2048}if fcntl(fd,4,x)<0{-errno()}else{0}}
pub unsafe fn os_accept_connection(fd:c_int)->c_int{let n=accept(fd,std::ptr::null_mut(),std::ptr::null_mut());if n<0{-errno()}else{n}}
pub unsafe fn os_shutdown_socket(fd:c_int,r:c_int,w:c_int)->c_int{let x=if r!=0&&w!=0{2}else if r!=0{0}else if w!=0{1}else{return -22};if shutdown(fd,x)<0{-errno()}else{0}}
pub unsafe fn os_flush_stdout(){fflush(std::ptr::null_mut());}
pub unsafe fn os_eventfd(v:c_uint,f:c_int)->c_int{let n=eventfd(v,f);if n<0{-errno()}else{n}}
pub unsafe fn os_file_size(f:*const c_char,o:*mut u64)->c_int{let mut b=std::mem::zeroed();let e=os_stat_file(f,&mut b);if e<0{return e}*o=b.ust_size as u64;0}
pub unsafe fn os_file_modtime(f:*const c_char,o:*mut i64)->c_int{let mut b=std::mem::zeroed();let e=os_stat_file(f,&mut b);if e<0{return e}*o=b.ust_mtime;0}
pub unsafe fn os_major(d:u64)->c_uint{(d>>8) as c_uint} pub unsafe fn os_minor(d:u64)->c_uint{(d&255) as c_uint} pub unsafe fn os_makedev(a:c_uint,b:c_uint)->u64{((a as u64)<<8)|(b as u64)}
pub unsafe fn os_falloc_punch(fd:c_int,o:u64,l:c_int)->c_int{let n=fallocate(fd,2|1,o,l as i64);if n<0{-errno()}else{n}} pub unsafe fn os_falloc_zeroes(fd:c_int,o:u64,l:c_int)->c_int{let n=fallocate(fd,16|1,o,l as i64);if n<0{-errno()}else{n}}
pub unsafe fn os_mmap_rw_shared(fd:c_int,s:usize)->*mut c_void{let p=mmap(std::ptr::null_mut(),s,3,1,fd,0);if p as isize==-1{std::ptr::null_mut()}else{p}}
pub unsafe fn os_mremap_rw_shared(a:*mut c_void,o:usize,n:usize)->*mut c_void{let p=mremap(a,o,n,1);if p as isize==-1{std::ptr::null_mut()}else{p}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
