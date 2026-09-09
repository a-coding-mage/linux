// SPDX-License-Identifier: GPL-2.0

// C headers and project headers provide the external types and functions used below.

static mut PROGS: [*mut bpf_program; 2] = [core::ptr::null_mut(); 2];
static mut LINKS: [*mut bpf_link; 2] = [core::ptr::null_mut(); 2];

const PMU_TYPE_FILE: &str = "/sys/bus/event_source/devices/%s/type";
const PMU_RETPROBE_FILE: &str = "/sys/bus/event_source/devices/%s/format/retprobe";

unsafe fn ptr_to_u64(ptr: *mut core::ffi::c_void) -> u64 { ptr as usize as u64 }

unsafe fn bpf_find_probe_type(event_type: *const i8) -> i32 {
    let mut buf = [0i8; 256];
    let ret = snprintf(buf.as_mut_ptr(), buf.len(), PMU_TYPE_FILE.as_ptr() as *const i8, event_type);
    if ret < 0 || ret >= buf.len() as i32 { perror(b"    \0".as_ptr() as *const i8); return -1; }
    let fd = open(buf.as_ptr(), O_RDONLY);
    if fd < 0 { perror(b"    \0".as_ptr() as *const i8); return -1; }
    let ret = read(fd, buf.as_mut_ptr() as *mut _, buf.len()); close(fd);
    if ret < 0 || ret >= buf.len() as isize { perror(b"    \0".as_ptr() as *const i8); return -1; }
    *__errno_location() = 0;
    let result = strtol(buf.as_ptr(), core::ptr::null_mut(), 10) as i32;
    if *__errno_location() != 0 { perror(b"    \0".as_ptr() as *const i8); return -1; }
    result
}

unsafe fn bpf_get_retprobe_bit(event_type: *const i8) -> i32 {
    let mut buf = [0i8; 256];
    let ret = snprintf(buf.as_mut_ptr(), buf.len(), PMU_RETPROBE_FILE.as_ptr() as *const i8, event_type);
    if ret < 0 || ret >= buf.len() as i32 { perror(b"    \0".as_ptr() as *const i8); return -1; }
    let fd = open(buf.as_ptr(), O_RDONLY);
    if fd < 0 { perror(b"    \0".as_ptr() as *const i8); return -1; }
    let ret = read(fd, buf.as_mut_ptr() as *mut _, buf.len()); close(fd);
    if ret < 0 || ret >= buf.len() as isize { perror(b"    \0".as_ptr() as *const i8); return -1; }
    if strlen(buf.as_ptr()) < strlen(b"config:\0".as_ptr() as *const i8) { perror(b"    \0".as_ptr() as *const i8); return -1; }
    *__errno_location() = 0;
    let result = strtol(buf.as_ptr().add(strlen(b"config:\0".as_ptr() as *const i8)), core::ptr::null_mut(), 10) as i32;
    if *__errno_location() != 0 { perror(b"    \0".as_ptr() as *const i8); return -1; }
    result
}

unsafe fn test_debug_fs_kprobe(link_idx: i32, fn_name: *const i8, expected_fd_type: u32) -> i32 {
    let mut buf = [0i8; 256]; let mut len = buf.len() as u32; let mut prog_id = 0; let mut fd_type = 0; let mut probe_offset = 0u64; let mut probe_addr = 0u64;
    let event_fd = bpf_link__fd(LINKS[link_idx as usize]);
    let err = bpf_task_fd_query(getpid(), event_fd, 0, buf.as_mut_ptr(), &mut len, &mut prog_id, &mut fd_type, &mut probe_offset, &mut probe_addr);
    if err < 0 { perror(b"    :\0".as_ptr() as *const i8); return -1; }
    if strcmp(buf.as_ptr(), fn_name) != 0 || fd_type != expected_fd_type || probe_offset != 0 || probe_addr != 0 { return -1; }
    0
}

unsafe fn test_nondebug_fs_kuprobe_common(event_type: *const i8, name: *const i8, offset: u64, addr: u64, is_return: bool, buf: *mut i8, buf_len: *mut u32, prog_id: *mut u32, fd_type: *mut u32, probe_offset: *mut u64, probe_addr: *mut u64) -> i32 {
    let is_return_bit = bpf_get_retprobe_bit(event_type); let typ = bpf_find_probe_type(event_type); let mut attr: perf_event_attr = core::mem::zeroed(); let mut link: *mut bpf_link; let mut err = -1;
    if typ < 0 || is_return_bit < 0 { return err; }
    attr.sample_period = 1; attr.wakeup_events = 1; if is_return { attr.config |= 1u64 << is_return_bit; }
    if !name.is_null() { attr.config1 = ptr_to_u64(name as *mut _); attr.config2 = offset; } else { attr.config1 = 0; attr.config2 = addr; }
    attr.size = core::mem::size_of::<perf_event_attr>() as u32; attr.type_ = typ as u32;
    let fd = sys_perf_event_open(&attr, -1, 0, -1, 0); link = bpf_program__attach_perf_event(PROGS[0], fd);
    if libbpf_get_error(link) != 0 { close(fd); link = core::ptr::null_mut(); } else { if bpf_task_fd_query(getpid(), fd, 0, buf, buf_len, prog_id, fd_type, probe_offset, probe_addr) < 0 { bpf_link__destroy(link); return -1; } err = 0; }
    bpf_link__destroy(link); err
}

unsafe fn test_nondebug_fs_probe(event_type: *const i8, name: *const i8, offset: u64, addr: u64, is_return: bool, expected_fd_type: u32, expected_ret_fd_type: u32, buf: *mut i8, buf_len: u32) -> i32 {
    let mut probe_offset = 0; let mut probe_addr = 0; let mut prog_id = 0; let mut fd_type = 0; let mut len = buf_len;
    if test_nondebug_fs_kuprobe_common(event_type, name, offset, addr, is_return, buf, &mut len, &mut prog_id, &mut fd_type, &mut probe_offset, &mut probe_addr) < 0 { return -1; }
    if (is_return && fd_type != expected_ret_fd_type) || (!is_return && fd_type != expected_fd_type) { return -1; }
    if !name.is_null() { if strcmp(name, buf) != 0 || probe_offset != offset { return -1; } } else if len != 0 || probe_addr != addr { return -1; }
    0
}

unsafe fn test_debug_fs_uprobe(binary_path: *mut i8, offset: i64, is_return: bool) -> i32 {
    let mut buf = [0i8; 256]; let mut alias = [0i8; 16]; let mut attr: perf_event_attr = core::mem::zeroed(); let mut probe_offset=0; let mut probe_addr=0; let mut prog_id=0; let mut fd_type=0; let mut len=buf.len() as u32;
    snprintf(buf.as_mut_ptr(), buf.len(), b"/sys/kernel/tracing/%s_events\0".as_ptr() as *const i8, b"uprobe\0".as_ptr()); let kfd=open(buf.as_ptr(), O_WRONLY|O_TRUNC); if kfd<0{return -1;}
    snprintf(alias.as_mut_ptr(), alias.len(), b"test_%d\0".as_ptr() as *const i8, getpid()); snprintf(buf.as_mut_ptr(), buf.len(), b"%c:%ss/%s %s:0x%lx\0".as_ptr() as *const i8, if is_return {'r'} else {'p'} as i32, b"uprobe\0".as_ptr(), alias.as_ptr(), binary_path, offset); if write(kfd,buf.as_ptr() as *const _,strlen(buf.as_ptr()))<0{return -1;} close(kfd);
    snprintf(buf.as_mut_ptr(),buf.len(),b"/sys/kernel/tracing/events/%ss/%s/id\0".as_ptr() as *const i8,b"uprobe\0".as_ptr(),alias.as_ptr()); let efd=open(buf.as_ptr(),O_RDONLY); if efd<0{return -1;} let bytes=read(efd,buf.as_mut_ptr() as *mut _,buf.len()); close(efd); if bytes<=0{return -1;} *buf.as_mut_ptr().add(bytes as usize)=0; attr.config=strtol(buf.as_ptr(),core::ptr::null_mut(),0) as u64; attr.type_=PERF_TYPE_TRACEPOINT; attr.sample_period=1; attr.wakeup_events=1;
    let fd=sys_perf_event_open(&attr,-1,0,-1,PERF_FLAG_FD_CLOEXEC); let link=bpf_program__attach_perf_event(PROGS[0],fd); if libbpf_get_error(link)!=0{close(fd);return -1;} let r=bpf_task_fd_query(getpid(),fd,0,buf.as_mut_ptr(),&mut len,&mut prog_id,&mut fd_type,&mut probe_offset,&mut probe_addr); bpf_link__destroy(link); if r<0{return -1;} if (is_return&&fd_type!=BPF_FD_TYPE_URETPROBE)||(!is_return&&fd_type!=BPF_FD_TYPE_UPROBE)||strcmp(binary_path,buf.as_ptr())!=0||probe_offset as i64!=offset{return -1;} 0
}

unsafe fn main_impl(argc: i32, argv: *mut *mut i8) -> i32 {
    let mut filename=[0i8;256]; let mut buf=[0i8;256]; let mut i=0; let mut err=-1; if load_kallsyms()!=0{return err;} snprintf(filename.as_mut_ptr(),filename.len(),b"%s_kern.o\0".as_ptr() as *const i8,*argv); let obj=bpf_object__open_file(filename.as_ptr(),core::ptr::null()); if libbpf_get_error(obj)!=0{return err;} if bpf_object__load(obj)!=0{bpf_object__close(obj);return err;}
    let mut prog=core::ptr::null_mut(); while bpf_object__next_program(obj,&mut prog)==0 { PROGS[i]=prog; LINKS[i]=bpf_program__attach(PROGS[i]); if libbpf_get_error(LINKS[i])!=0{LINKS[i]=core::ptr::null_mut();break;} i+=1; }
    let p=b"bpf_check\0".as_ptr() as *const i8; let k=b"kprobe\0".as_ptr() as *const i8; let u=b"uprobe\0".as_ptr() as *const i8; if test_debug_fs_kprobe(0,b"blk_mq_start_request\0".as_ptr() as _,BPF_FD_TYPE_KPROBE)<0{return -1;} if test_debug_fs_kprobe(1,b"__blk_account_io_done\0".as_ptr() as _,BPF_FD_TYPE_KRETPROBE)<0{return -1;} if test_nondebug_fs_probe(k,p,0,0,false,BPF_FD_TYPE_KPROBE,BPF_FD_TYPE_KRETPROBE,buf.as_mut_ptr(),256)<0{return -1;}
    let off=(main_impl as usize-(&__executable_start as *const _ as usize)) as u64; if test_nondebug_fs_probe(u,*argv,off,0,false,BPF_FD_TYPE_UPROBE,BPF_FD_TYPE_URETPROBE,buf.as_mut_ptr(),256)<0{return -1;} if test_nondebug_fs_probe(u,*argv,off,0,true,BPF_FD_TYPE_UPROBE,BPF_FD_TYPE_URETPROBE,buf.as_mut_ptr(),256)<0{return -1;} if test_debug_fs_uprobe(*argv,off as i64,false)<0{return -1;} if test_debug_fs_uprobe(*argv,off as i64,true)<0{return -1;} err=0; while i>0{i-=1;bpf_link__destroy(LINKS[i]);} bpf_object__close(obj); err
}

// External declarations supplied by the surrounding build.
extern "C" {
    static __executable_start: u8;
    fn snprintf(*mut i8, usize, *const i8, ...) -> i32; fn perror(*const i8); fn open(*const i8, i32, ...) -> i32; fn read(i32,*mut core::ffi::c_void,usize)->isize; fn close(i32)->i32; fn strtol(*const i8,*mut *mut i8,i32)->i64; fn strlen(*const i8)->usize; fn strcmp(*const i8,*const i8)->i32; fn write(i32,*const core::ffi::c_void,usize)->isize; fn getpid()->i32; fn __errno_location()->*mut i32;
    fn load_kallsyms()->i32; fn sys_perf_event_open(*const perf_event_attr,i32,i32,i32,u64)->i32; fn bpf_task_fd_query(i32,i32,u32,*mut i8,*mut u32,*mut u32,*mut u32,*mut u64,*mut u64)->i32; fn bpf_link__fd(*mut bpf_link)->i32; fn bpf_program__attach_perf_event(*mut bpf_program,i32)->*mut bpf_link; fn bpf_program__attach(*mut bpf_program)->*mut bpf_link; fn libbpf_get_error(*const core::ffi::c_void)->i64; fn bpf_link__destroy(*mut bpf_link); fn bpf_object__open_file(*const i8,*const core::ffi::c_void)->*mut bpf_object; fn bpf_object__load(*mut bpf_object)->i32; fn bpf_object__next_program(*mut bpf_object,*mut *mut bpf_program)->i32; fn bpf_object__close(*mut bpf_object);
}

#[repr(C)] struct bpf_program; #[repr(C)] struct bpf_link; #[repr(C)] struct bpf_object; #[repr(C)] struct perf_event_attr { sample_period:u64, wakeup_events:u32, config:u64, config1:u64, config2:u64, size:u32, type_:u32 }
const O_RDONLY:i32=0; const O_WRONLY:i32=1; const O_TRUNC:i32=512; const PERF_TYPE_TRACEPOINT:u32=2; const PERF_FLAG_FD_CLOEXEC:u64=8; const BPF_FD_TYPE_KPROBE:u32=1; const BPF_FD_TYPE_KRETPROBE:u32=2; const BPF_FD_TYPE_UPROBE:u32=3; const BPF_FD_TYPE_URETPROBE:u32=4;

fn main() { unsafe { std::process::exit(main_impl(std::env::args().count() as i32, std::env::args().collect::<Vec<_>>().as_mut_ptr() as *mut *mut i8)); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
