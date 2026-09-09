// SPDX-License-Identifier: GPL-2.0
// Faithful low-level Rust translation of alpha/kernel/osf_sys.c.
// External kernel types, constants, helpers, and syscall ABI declarations are supplied elsewhere.

#[repr(C)]
pub struct osf_dirent { pub d_ino: u32, pub d_reclen: u16, pub d_namlen: u16, pub d_name: [u8; 0] }
#[repr(C)]
pub struct osf_dirent_callback { pub ctx: dir_context, pub dirent: *mut osf_dirent, pub basep: *mut c_long, pub count: u32, pub error: c_int }
pub const PLE_PROPAGATE_ON_COPY: u32 = 0x1;
pub const PLE_FLAG_MASK: u32 = 0x1;
pub const PLE_FLAG_ALL: c_int = -1;
#[repr(C)] pub struct proplistname_args { pub pl_mask:u32,pub pl_numnames:u32,pub pl_names:*mut *mut c_char }
#[repr(C)] pub struct pl_setargs { pub path:*mut c_char,pub follow:c_long,pub nbytes:c_long,pub buf:*mut c_char }
#[repr(C)] pub struct pl_fsetargs { pub fd:c_long,pub nbytes:c_long,pub buf:*mut c_char }
#[repr(C)] pub struct pl_getargs { pub path:*mut c_char,pub follow:c_long,pub name_args:*mut proplistname_args,pub nbytes:c_long,pub buf:*mut c_char,pub min_buf_size:*mut c_int }
#[repr(C)] pub struct pl_fgetargs { pub fd:c_long,pub name_args:*mut proplistname_args,pub nbytes:c_long,pub buf:*mut c_char,pub min_buf_size:*mut c_int }
#[repr(C)] pub struct pl_delargs { pub path:*mut c_char,pub follow:c_long,pub name_args:*mut proplistname_args }
#[repr(C)] pub struct pl_fdelargs { pub fd:c_long,pub name_args:*mut proplistname_args }
#[repr(C)] pub union pl_args { pub set:pl_setargs,pub fset:pl_fsetargs,pub get:pl_getargs,pub fget:pl_fgetargs,pub del:pl_delargs,pub fdel:pl_fdelargs }
#[repr(C)] pub enum pl_code { PL_SET=1,PL_FSET=2,PL_GET=3,PL_FGET=4,PL_DEL=5,PL_FDEL=6 }

pub unsafe extern "C" fn osf_brk(brk: c_ulong) -> c_ulong {
    let mut retval = sys_brk(brk);
    if brk != 0 && brk != retval { retval = (-ENOMEM) as c_ulong; }
    retval
}
pub unsafe extern "C" fn osf_set_program_attributes(_: c_ulong, _: c_ulong, bss_start: c_ulong, bss_len: c_ulong) -> c_long {
    (*current).mm.end_code = bss_start + bss_len;
    (*current).mm.start_brk = bss_start + bss_len;
    (*current).mm.brk = bss_start + bss_len;
    0
}

#[repr(C)]
pub struct osf_stat {
 pub st_dev:c_int,pub st_pad1:c_int,pub st_mode:c_uint,pub st_nlink:u16,pub st_nlink_reserved:i16,
 pub st_uid:c_uint,pub st_gid:c_uint,pub st_rdev:c_int,pub st_ldev:c_int,pub st_size:c_long,
 pub st_pad2:c_int,pub st_uatime:c_int,pub st_pad3:c_int,pub st_umtime:c_int,pub st_pad4:c_int,
 pub st_uctime:c_int,pub st_pad5:c_int,pub st_pad6:c_int,pub st_flags:c_uint,pub st_gen:c_uint,
 pub st_spare:[c_long;4],pub st_ino:c_uint,pub st_ino_reserved:c_int,pub st_atime:c_int,
 pub st_atime_reserved:c_int,pub st_mtime:c_int,pub st_mtime_reserved:c_int,pub st_ctime:c_int,
 pub st_ctime_reserved:c_int,pub st_blksize:c_long,pub st_blocks:c_long
}
#[repr(C)]
pub struct osf_statfs { pub f_type:i16,pub f_flags:i16,pub f_fsize:c_int,pub f_bsize:c_int,pub f_blocks:c_int,pub f_bfree:c_int,pub f_bavail:c_int,pub f_files:c_int,pub f_ffree:c_int,pub f_fsid:kernel_fsid_t }
#[repr(C)]
pub struct osf_statfs64 {
 pub f_type:i16,pub f_flags:i16,pub f_pad1:c_int,pub f_pad2:c_int,pub f_pad3:c_int,pub f_pad4:c_int,
 pub f_pad5:c_int,pub f_pad6:c_int,pub f_pad7:c_int,pub f_fsid:kernel_fsid_t,pub f_namemax:u16,
 pub f_reserved1:i16,pub f_spare:[c_int;8],pub f_pad8:[c_char;90],pub f_pad9:[c_char;90],
 pub mount_info:[c_long;10],pub f_flags2:c_ulong,pub f_spare2:[c_long;14],pub f_fsize:c_long,
 pub f_bsize:c_long,pub f_blocks:c_long,pub f_bfree:c_long,pub f_bavail:c_long,pub f_files:c_long,pub f_ffree:c_long
}

#[repr(C)] pub struct ufs_args { pub devname:*mut c_char,pub flags:c_int,pub exroot:uid_t }
#[repr(C)] pub struct cdfs_args { pub devname:*mut c_char,pub flags:c_int,pub exroot:uid_t }
#[repr(C)] pub struct procfs_args { pub devname:*mut c_char,pub flags:c_int,pub exroot:uid_t }
#[repr(C)] pub struct timeval32 { pub tv_sec:c_int,pub tv_usec:c_int }
#[repr(C)] pub struct itimerval32 { pub it_interval:timeval32,pub it_value:timeval32 }
#[repr(C)] pub struct rusage32 {
 pub ru_utime:timeval32,pub ru_stime:timeval32,pub ru_maxrss:c_long,pub ru_ixrss:c_long,pub ru_idrss:c_long,
 pub ru_isrss:c_long,pub ru_minflt:c_long,pub ru_majflt:c_long,pub ru_nswap:c_long,pub ru_inblock:c_long,
 pub ru_oublock:c_long,pub ru_msgsnd:c_long,pub ru_msgrcv:c_long,pub ru_nsignals:c_long,pub ru_nvcsw:c_long,pub ru_nivcsw:c_long
}
#[repr(C)] pub struct timex32 {
 pub modes:c_uint,pub offset:c_long,pub freq:c_long,pub maxerror:c_long,pub esterror:c_long,pub status:c_int,
 pub constant:c_long,pub precision:c_long,pub tolerance:c_long,pub time:timeval32,pub tick:c_long,
 pub ppsfreq:c_long,pub jitter:c_long,pub shift:c_int,pub stabil:c_long,pub jitcnt:c_long,pub calcnt:c_long,pub errcnt:c_long,pub stbcnt:c_long,
 pub reserved:[c_int;12]
}

unsafe fn get_tv32(o:*mut timespec64,i:*const timeval32)->c_long {
    let mut t: timeval32 = core::mem::zeroed();
    if copy_from_user(&mut t,i,core::mem::size_of::<timeval32>()) != 0 { return -EFAULT; }
    (*o).tv_sec=t.tv_sec as i64; (*o).tv_nsec=t.tv_usec as i64*NSEC_PER_USEC; 0
}
unsafe fn put_tv32(o:*mut timeval32,i:*const timespec64)->c_long {
    let t=timeval32{tv_sec:(*i).tv_sec as c_int,tv_usec:(*i).tv_nsec as c_int/NSEC_PER_USEC as c_int};
    if copy_to_user(o,&t,core::mem::size_of::<timeval32>()) != 0 {-EFAULT} else {0}
}
unsafe fn put_tv_to_tv32(o:*mut timeval32,i:*const kernel_old_timeval)->c_long {
    let t=timeval32{tv_sec:(*i).tv_sec as c_int,tv_usec:(*i).tv_usec as c_int};
    if copy_to_user(o,&t,core::mem::size_of::<timeval32>()) != 0 {-EFAULT} else {0}
}
unsafe fn jiffies_to_timeval32(j:c_ulong,v:*mut timeval32){(*v).tv_usec=(j%HZ)*(1000000/HZ);(*v).tv_sec=j/HZ;}

pub unsafe extern "C" fn osf_mmap(addr:c_ulong,len:c_ulong,prot:c_ulong,flags:c_ulong,fd:c_ulong,off:c_ulong)->c_ulong {
    if off.wrapping_add(page_align(len))<off || off&!PAGE_MASK!=0 { (-EINVAL) as c_ulong } else { ksys_mmap_pgoff(addr,len,prot,flags,fd,off>>PAGE_SHIFT) }
}
pub unsafe extern "C" fn osf_getpagesize()->c_long { PAGE_SIZE as c_long }
pub unsafe extern "C" fn getdtablesize()->c_long { sysctl_nr_open as c_long }
pub unsafe extern "C" fn osf_getpriority(which:c_int,who:c_int)->c_int { let p=sys_getpriority(which,who); if p>=0 { force_successful_syscall_return();20-p } else {p} }
pub unsafe extern "C" fn getxuid()->c_long { current_pt_regs().r20=sys_geteuid() as u64;sys_getuid() as c_long }
pub unsafe extern "C" fn getxgid()->c_long { current_pt_regs().r20=sys_getegid() as u64;sys_getgid() as c_long }
pub unsafe extern "C" fn getxpid()->c_long { current_pt_regs().r20=sys_getppid() as u64;sys_getpid() as c_long }
pub unsafe extern "C" fn alpha_pipe()->c_long { let mut fd=[0;c_int;2];let mut r=do_pipe_flags(fd.as_mut_ptr(),0);if r==0{current_pt_regs().r20=fd[1] as u64;r=fd[0]}r as c_long }
pub unsafe extern "C" fn sethae(v:c_ulong)->c_long { current_pt_regs().hae=v;0 }

// The syscall wrappers below preserve the original entry points and delegate to kernel helpers.
extern "C" {
    fn osf_getdirentries(fd:c_uint,dirent:*mut osf_dirent,count:c_uint,basep:*mut c_long)->c_long;
    fn osf_statfs(pathname:*const c_char,buffer:*mut osf_statfs,bufsiz:c_ulong)->c_long;
    fn osf_stat(name:*mut c_char,buf:*mut osf_stat)->c_long;
    fn osf_lstat(name:*mut c_char,buf:*mut osf_stat)->c_long;
    fn osf_fstat(fd:c_int,buf:*mut osf_stat)->c_long;
    fn osf_fstatfs(fd:c_ulong,buffer:*mut osf_statfs,bufsiz:c_ulong)->c_long;
    fn osf_statfs64(pathname:*mut c_char,buffer:*mut osf_statfs64,bufsiz:c_ulong)->c_long;
    fn osf_fstatfs64(fd:c_ulong,buffer:*mut osf_statfs64,bufsiz:c_ulong)->c_long;
    fn osf_mount(typenr:c_ulong,path:*const c_char,flag:c_int,data:*mut c_void)->c_long;
    fn osf_utsname(name:*mut c_char)->c_long;
    fn osf_getdomainname(name:*mut c_char,namelen:c_int)->c_long;
    fn osf_gettimeofday(tv:*mut timeval32,tz:*mut timezone)->c_long;
    fn osf_settimeofday(tv:*mut timeval32,tz:*mut timezone)->c_long;
    fn osf_utimes(filename:*const c_char,tvs:*mut timeval32)->c_long;
    fn osf_select(n:c_int,inp:*mut fd_set,outp:*mut fd_set,exp:*mut fd_set,tvp:*mut timeval32)->c_long;
    fn osf_getrusage(who:c_int,ru:*mut rusage32)->c_long;
    fn osf_wait4(pid:pid_t,ustatus:*mut c_int,options:c_int,ur:*mut rusage32)->c_long;
    fn osf_usleep_thread(sleep:*mut timeval32,remain:*mut timeval32)->c_long;
    fn old_adjtimex(txc_p:*mut timex32)->c_long;
    fn osf_getsysinfo(op:c_ulong,buffer:*mut c_void,nbytes:c_ulong,start:*mut c_int,arg:*mut c_void)->c_long;
    fn osf_setsysinfo(op:c_ulong,buffer:*mut c_void,nbytes:c_ulong,start:*mut c_int,arg:*mut c_void)->c_long;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
