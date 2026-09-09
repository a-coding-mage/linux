// SPDX-License-Identifier: GPL-2.0
/* Copyright 2020-2021 Amazon.com, Inc. or its affiliates. All Rights Reserved. */
// Translation of ne_ioctl_sample.c. Linux ioctl/kernel ABI names are supplied externally.

use std::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use std::mem::{size_of, zeroed};
use std::ptr::{null, null_mut};

const NE_DEV_NAME: &[u8] = b"/dev/nitro_enclaves\0";
const NE_POLL_WAIT_TIME: c_int = 60;
const NE_POLL_WAIT_TIME_MS: c_int = NE_POLL_WAIT_TIME * 1000;
const NE_SLEEP_TIME: c_uint = 300;
const NE_DEFAULT_NR_VCPUS: usize = 2;
const NE_MIN_MEM_REGION_SIZE: usize = 2 * 1024 * 1024;
const NE_DEFAULT_NR_MEM_REGIONS: usize = 256;
const NE_IMAGE_LOAD_HEARTBEAT_CID: u32 = 3;
const NE_IMAGE_LOAD_HEARTBEAT_PORT: u32 = 9000;
const NE_IMAGE_LOAD_HEARTBEAT_VALUE: u8 = 0xb7;

#[repr(C)]
pub struct ne_user_mem_region { pub userspace_addr: *mut c_void, pub memory_size: usize }

// Supplied by linux/nitro_enclaves.h and linux/vm_sockets.h.
#[repr(C)] pub struct ne_image_load_info { pub flags: u64, pub memory_offset: u64 }
#[repr(C)] pub struct ne_user_memory_region { pub flags: u64, pub memory_size: u64, pub userspace_addr: u64 }
#[repr(C)] pub struct ne_enclave_start_info { pub flags: u64, pub enclave_cid: u64 }
#[repr(C)] pub struct sockaddr_vm { pub svm_family: u16, pub svm_reserved1: u16, pub svm_port: u32, pub svm_cid: u32, pub svm_zero: [u8; 4] }
#[repr(C)] pub struct pollfd { pub fd: c_int, pub events: i16, pub revents: i16 }

extern "C" {
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn mmap(addr: *mut c_void, len: usize, prot: c_int, flags: c_int, fd: c_int, off: isize) -> *mut c_void;
    fn munmap(addr: *mut c_void, len: usize) -> c_int;
    fn stat(path: *const c_char, buf: *mut stat) -> c_int;
    fn poll(fds: *mut pollfd, n: usize, timeout: c_int) -> c_int;
    fn socket(domain: c_int, typ: c_int, protocol: c_int) -> c_int;
    fn bind(fd: c_int, addr: *const c_void, len: u32) -> c_int;
    fn listen(fd: c_int, backlog: c_int) -> c_int;
    fn accept(fd: c_int, addr: *mut c_void, len: *mut u32) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> isize;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn pthread_create(t: *mut usize, attr: *const c_void, f: unsafe extern "C" fn(*mut c_void) -> *mut c_void, arg: *mut c_void) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn strlen(s: *const c_char) -> usize;
    fn __errno_location() -> *mut c_int;
}
#[repr(C)] struct stat { st_size: i64, _rest: [u8; 128] }

const EINVAL: c_int = 22; const ENOMEM: c_int = 12; const ETIMEDOUT: c_int = 110;
const MAP_FAILED: *mut c_void = (-1isize) as *mut c_void;
const PROT_READ: c_int = 1; const PROT_WRITE: c_int = 2;
const MAP_PRIVATE: c_int = 2; const MAP_ANONYMOUS: c_int = 0x20; const MAP_HUGETLB: c_int = 0x40000; const MAP_HUGE_2MB: c_int = 21 << 26;
const O_RDONLY: c_int = 0; const O_RDWR: c_int = 2; const O_CLOEXEC: c_int = 0o2000000;
const POLLIN: i16 = 1; const POLLERR: i16 = 8; const POLLHUP: i16 = 16; const POLLNVAL: i16 = 32;
const AF_VSOCK: u16 = 40; const SOCK_STREAM: c_int = 1;

// ioctl requests and NE_ERR_* / NE_* ABI constants are intentionally external dependencies.
extern "C" {
    static NE_CREATE_VM: c_ulong; static NE_GET_IMAGE_LOAD_INFO: c_ulong; static NE_SET_USER_MEMORY_REGION: c_ulong;
    static NE_ADD_VCPU: c_ulong; static NE_START_ENCLAVE: c_ulong;
    static NE_EIF_IMAGE: u64; static NE_DEFAULT_MEMORY_REGION: u64;
}

unsafe fn ne_create_vm(ne_dev_fd: c_int, slot_uid: *mut c_ulong, enclave_fd: *mut c_int) -> c_int {
    let mut rc = -EINVAL; *enclave_fd = ioctl(ne_dev_fd, NE_CREATE_VM, slot_uid);
    if *enclave_fd < 0 { rc = *enclave_fd; if *__errno_location() == NE_ERR_NO_CPUS_AVAIL_IN_POOL { println!("Error in create VM, no CPUs available in the NE CPU pool"); } else { println!("Error in create VM"); } return rc; } 0
}
unsafe extern "C" fn ne_poll_enclave_fd(data: *mut c_void) -> *mut c_void {
    let enclave_fd = *(data as *const c_int); let mut fds = [pollfd { fd: 0, events: 0, revents: 0 }]; let mut i = 0; let mut rc;
    println!("Running from poll thread, enclave fd {}", enclave_fd); fds[0].fd = enclave_fd; fds[0].events = POLLIN | POLLERR | POLLHUP;
    loop { println!("[iter {}] Polling ...", i); rc = poll(fds.as_mut_ptr(), 1, NE_POLL_WAIT_TIME_MS); if rc < 0 { println!("Error in poll"); return null_mut(); } i += 1;
        if rc == 0 { println!("Poll: {} seconds elapsed", i * NE_POLL_WAIT_TIME); continue; } println!("Poll received value 0x{:x}", fds[0].revents);
        if fds[0].revents & POLLHUP != 0 { println!("Received POLLHUP"); return null_mut(); } if fds[0].revents & POLLNVAL != 0 { println!("Received POLLNVAL"); return null_mut(); }
    }
}
unsafe fn ne_alloc_user_mem_region(r: *mut ne_user_mem_region) -> c_int { (*r).userspace_addr = mmap(null_mut(), (*r).memory_size, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS|MAP_HUGETLB|MAP_HUGE_2MB, -1, 0); if (*r).userspace_addr == MAP_FAILED { println!("Error in mmap memory"); return -1; } 0 }
unsafe fn ne_load_enclave_image(fd: c_int, rs: *mut ne_user_mem_region, path: *mut c_char) -> c_int {
    let mut st: stat = zeroed(); let mut info = ne_image_load_info { flags: NE_EIF_IMAGE, memory_offset: 0 }; let mut total=0usize; for i in 0..NE_DEFAULT_NR_MEM_REGIONS { total += (*rs.add(i)).memory_size; }
    let mut rc=stat(path,&mut st); if rc<0 { println!("Error in get image stat info"); return rc; } let image_size=st.st_size as usize; if total<image_size { println!("The enclave memory is smaller than the enclave image size"); return -ENOMEM; }
    rc=ioctl(fd,NE_GET_IMAGE_LOAD_INFO,&mut info); if rc<0 { println!("Error in get image load info"); return rc; } println!("Enclave image offset in enclave memory is {}",info.memory_offset);
    let image_fd=open(path,O_RDONLY); if image_fd<0 { println!("Error in open enclave image file"); return image_fd; } let image=mmap(null_mut(),image_size,PROT_READ,MAP_PRIVATE,image_fd,0); if image==MAP_FAILED { println!("Error in mmap enclave image"); return -1; }
    let mut off=info.memory_offset as usize; let mut written=0; for i in 0..NE_DEFAULT_NR_MEM_REGIONS { let r=&*rs.add(i); if off>=r.memory_size { off-=r.memory_size; continue; } let memoff=off; let avail=r.memory_size-off; off=0; let n=avail.min(image_size-written); memcpy((r.userspace_addr as *mut u8).add(memoff) as *mut c_void,(image as *const u8).add(written) as *const c_void,n); written+=n; if written==image_size { break; } } munmap(image,image_size); close(image_fd); 0
}
unsafe fn ne_set_user_mem_region(fd:c_int,r:ne_user_mem_region)->c_int { let mut m=ne_user_memory_region{flags:NE_DEFAULT_MEMORY_REGION,memory_size:r.memory_size as u64,userspace_addr:r.userspace_addr as u64}; let rc=ioctl(fd,NE_SET_USER_MEMORY_REGION,&mut m); if rc<0 { println!("Error in set user memory region"); return rc; } 0 }
unsafe fn ne_free_mem_regions(rs:*mut ne_user_mem_region){for i in 0..NE_DEFAULT_NR_MEM_REGIONS{munmap((*rs.add(i)).userspace_addr,(*rs.add(i)).memory_size);}}
unsafe fn ne_add_vcpu(fd:c_int,id:*mut c_uint)->c_int{let rc=ioctl(fd,NE_ADD_VCPU,id);if rc<0{println!("Error in add vcpu");return rc;}0}
unsafe fn ne_start_enclave(fd:c_int,info:*mut ne_enclave_start_info)->c_int{let rc=ioctl(fd,NE_START_ENCLAVE,info);if rc<0{println!("Error in start enclave");return rc;}0}
unsafe fn ne_start_enclave_check_booted(fd:c_int)->c_int { let mut server=sockaddr_vm{svm_family:AF_VSOCK,svm_reserved1:0,svm_port:NE_IMAGE_LOAD_HEARTBEAT_PORT,svm_cid:NE_IMAGE_LOAD_HEARTBEAT_CID,svm_zero:[0;4]}; let s=socket(AF_VSOCK as c_int,SOCK_STREAM,0); if s<0{return s;} let mut rc=bind(s,&server as *const _ as *const c_void,size_of::<sockaddr_vm>() as u32); if rc<0{close(s);return rc;} rc=listen(s,1);if rc<0{close(s);return rc;} let mut info:ne_enclave_start_info=zeroed();rc=ne_start_enclave(fd,&mut info);if rc<0{close(s);return rc;} println!("Enclave started, CID {}",info.enclave_cid);let mut p=pollfd{fd:s,events:POLLIN,revents:0};rc=poll(&mut p,1,NE_POLL_WAIT_TIME_MS);if rc<=0{close(s);return if rc==0{-ETIMEDOUT}else{rc};}if p.revents&POLLIN==0{close(s);return -EINVAL;}let mut ca: sockaddr_vm=zeroed();let mut len=size_of::<sockaddr_vm>() as u32;let c=accept(s,&mut ca as *mut _ as *mut c_void,&mut len);if c<0{close(s);return c;}let mut b=0u8;rc=read(c,&mut b as *mut _ as *mut c_void,1) as c_int;if rc<0{close(s);return rc;}if rc!=1||b!=NE_IMAGE_LOAD_HEARTBEAT_VALUE{close(s);return -EINVAL;}rc=write(c,&b as *const _ as *const c_void,1) as c_int;close(c);close(s);if rc<0{rc}else{0} }

pub unsafe fn main_c(argc:c_int,argv:*mut *mut c_char)->c_int { if argc!=2{return 1;}let path=*argv.add(1);if strlen(path)>=4096{return 1;}let dev=open(NE_DEV_NAME.as_ptr() as *const c_char,O_RDWR|O_CLOEXEC);if dev<0{return 1;}let mut fd=-1;let mut uid=0;let rc=ne_create_vm(dev,&mut uid,&mut fd);close(dev);if rc<0{return 1;}let mut thread=0;pthread_create(&mut thread,null(),ne_poll_enclave_fd,&mut fd as *mut _ as *mut c_void);let mut rs=[ne_user_mem_region{userspace_addr:null_mut(),memory_size:0};NE_DEFAULT_NR_MEM_REGIONS];for r in &mut rs{r.memory_size=NE_MIN_MEM_REGION_SIZE;if ne_alloc_user_mem_region(r)<0{ne_free_mem_regions(rs.as_mut_ptr());close(fd);return 1;}}if ne_load_enclave_image(fd,rs.as_mut_ptr(),path)<0{ne_free_mem_regions(rs.as_mut_ptr());close(fd);return 1;}for r in rs{if ne_set_user_mem_region(fd,r)<0{ne_free_mem_regions(rs.as_mut_ptr());close(fd);return 1;}}for _ in 0..NE_DEFAULT_NR_VCPUS{let mut id=0; if ne_add_vcpu(fd,&mut id)<0{ne_free_mem_regions(rs.as_mut_ptr());close(fd);return 1;}}if ne_start_enclave_check_booted(fd)<0{ne_free_mem_regions(rs.as_mut_ptr());close(fd);return 1;}sleep(NE_SLEEP_TIME);close(fd);ne_free_mem_regions(rs.as_mut_ptr());0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
