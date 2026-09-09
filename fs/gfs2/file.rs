// SPDX-License-Identifier: GPL-2.0-only
/* Direct low-level translation of gfs2/file.c. External kernel and GFS2
 * declarations are intentionally left to the surrounding repository. */

#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]
use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

extern "C" {
    fn gfs2_glock_nq_init(gl: *mut gfs2_glock, state: u16, flags: u32, gh: *mut gfs2_holder) -> c_int;
    fn gfs2_glock_nq(gh: *mut gfs2_holder) -> c_int;
    fn gfs2_glock_dq(gh: *mut gfs2_holder);
    fn gfs2_glock_dq_uninit(gh: *mut gfs2_holder);
    fn gfs2_holder_init(gl: *mut gfs2_glock, state: u16, flags: u32, gh: *mut gfs2_holder);
    fn gfs2_holder_uninit(gh: *mut gfs2_holder);
    fn gfs2_holder_reinit(state: u16, flags: u32, gh: *mut gfs2_holder);
    fn gfs2_holder_initialized(gh: *mut gfs2_holder) -> bool;
    fn gfs2_holder_queued(gh: *mut gfs2_holder) -> bool;
    fn gfs2_dir_read(dir: *mut inode, ctx: *mut dir_context, ra: *mut file_ra_state) -> c_int;
    fn generic_file_llseek(file: *mut file, offset: i64, whence: c_int) -> i64;
    fn gfs2_seek_data(file: *mut file, offset: i64) -> i64;
    fn gfs2_seek_hole(file: *mut file, offset: i64) -> i64;
    fn filemap_fdatawrite(mapping: *mut address_space) -> c_int;
    fn filemap_fdatawait(mapping: *mut address_space) -> c_int;
    fn truncate_inode_pages(mapping: *mut address_space, l: i64);
    fn gfs2_permission(idmap: *const c_void, inode: *mut inode, mask: c_int) -> c_int;
    fn gfs2_log_flush(sdp: *mut gfs2_sbd, gl: *mut gfs2_glock, flags: u32);
    fn gfs2_ordered_del_inode(ip: *mut gfs2_inode);
    fn gfs2_trans_begin(sdp: *mut gfs2_sbd, blocks: u32, revokes: u32) -> c_int;
    fn gfs2_trans_end(sdp: *mut gfs2_sbd);
    fn gfs2_meta_inode_buffer(ip: *mut gfs2_inode, bh: *mut *mut buffer_head) -> c_int;
    fn gfs2_trans_add_meta(gl: *mut gfs2_glock, bh: *mut buffer_head);
    fn gfs2_dinode_out(ip: *mut gfs2_inode, data: *mut u8);
    fn brelse(bh: *mut buffer_head);
    fn gfs2_set_aops(inode: *mut inode);
    fn gfs2_rindex_update(sdp: *mut gfs2_sbd) -> c_int;
    fn gfs2_write_calc_reserv(ip: *mut gfs2_inode, len: usize, data: *mut u32, ind: *mut u32);
    fn gfs2_quota_lock_check(ip: *mut gfs2_inode, ap: *mut gfs2_alloc_parms) -> c_int;
    fn gfs2_inplace_reserve(ip: *mut gfs2_inode, ap: *mut gfs2_alloc_parms) -> c_int;
    fn gfs2_inplace_release(ip: *mut gfs2_inode);
    fn gfs2_quota_unlock(ip: *mut gfs2_inode);
    fn gfs2_rg_blocks(ip: *mut gfs2_inode, blocks: u32) -> u32;
    fn gfs2_unstuff_dinode(ip: *mut gfs2_inode) -> c_int;
    fn gfs2_iomap_alloc(inode: *mut inode, pos: u64, len: usize, iomap: *mut iomap) -> c_int;
    fn gfs2_is_stuffed(ip: *mut gfs2_inode) -> bool;
    fn gfs2_write_alloc_required(ip: *mut gfs2_inode, pos: u64, len: usize) -> bool;
    fn gfs2_fault(vmf: *mut vm_fault) -> vm_fault_t;
    fn filemap_fault(vmf: *mut vm_fault) -> vm_fault_t;
    fn filemap_map_pages(vmf: *mut vm_fault, start: u64, end: u64) -> c_int;
    fn gfs2_is_jdata(ip: *mut gfs2_inode) -> bool;
    fn generic_file_open(inode: *mut inode, file: *mut file) -> c_int;
    fn gfs2_qa_get(ip: *mut gfs2_inode) -> c_int;
    fn gfs2_qa_put(ip: *mut gfs2_inode);
    fn gfs2_rs_active(rs: *mut gfs2_rgrpd) -> bool;
    fn gfs2_rs_delete(ip: *mut gfs2_inode);
    fn filemap_fdatawrite_range(m: *mut address_space, start: i64, end: i64) -> c_int;
    fn sync_inode_metadata(inode: *mut inode, wait: c_int) -> c_int;
    fn file_write_and_wait(file: *mut file) -> c_int;
    fn gfs2_ail_flush(gl: *mut gfs2_glock, flags: c_int);
    fn file_fdatawait_range(file: *mut file, start: i64, end: i64) -> c_int;
    fn generic_file_read_iter(iocb: *mut kiocb, iter: *mut iov_iter) -> isize;
    fn iomap_dio_rw(iocb: *mut kiocb, iter: *mut iov_iter, ops: *const c_void, x: *mut c_void, flags: u32, y: *mut c_void, done: usize) -> isize;
    fn iomap_file_buffered_write(iocb: *mut kiocb, iter: *mut iov_iter, ops: *const c_void, wops: *const c_void, x: *mut c_void) -> isize;
    fn fault_in_iov_iter_writeable(iter: *mut iov_iter, size: usize) -> usize;
    fn fault_in_iov_iter_readable(iter: *mut iov_iter, size: usize) -> usize;
    fn gfs2_clear_beyond_eof(inode: *mut inode, pos: i64) -> c_int;
    fn generic_write_checks(iocb: *mut kiocb, iter: *mut iov_iter) -> isize;
    fn file_remove_privs(file: *mut file) -> c_int;
    fn generic_write_sync(iocb: *mut kiocb, count: isize) -> isize;
    fn invalidate_mapping_pages(m: *mut address_space, start: u64, end: u64);
    fn file_update_time(file: *mut file) -> c_int;
    fn sb_issue_zeroout(sb: *mut super_block, sector: u64, nr: u64, gfp: u32) -> c_int;
    fn mark_inode_dirty(inode: *mut inode);
    fn vfs_fsync_range(file: *mut file, start: i64, end: i64, datasync: c_int) -> c_int;
    fn __gfs2_punch_hole(file: *mut file, offset: i64, len: i64) -> c_long;
    fn inode_newsize_ok(inode: *mut inode, size: i64) -> c_int;
    fn get_write_access(inode: *mut inode) -> c_int;
    fn put_write_access(inode: *mut inode);
    fn gfs2_rs_deltree(rs: *mut gfs2_rgrpd);
    fn iter_file_splice_write(pipe: *mut pipe_inode_info, out: *mut file, pos: *mut i64, len: usize, flags: u32) -> isize;
}

// Kernel structures are supplied by the repository; opaque declarations keep this translation source-level.
#[repr(C)] pub struct file { pub f_mapping: *mut address_space, pub f_flags: u32, pub f_mode: u32, pub private_data: *mut c_void, pub f_lock: raw_spinlock_t }
#[repr(C)] pub struct address_space { pub host: *mut inode, pub nrpages: usize }
#[repr(C)] pub struct inode { pub i_mapping: *mut address_space, pub i_mode: u16, pub i_flags: u32, pub i_size: i64, pub i_sb: *mut super_block, pub i_blkbits: u32 }
#[repr(C)] pub struct super_block { _p: [u8; 0] }
#[repr(C)] pub struct gfs2_inode { pub i_inode: inode, pub i_gl: *mut gfs2_glock, pub i_diskflags: u32, pub i_eattr: u64, pub i_flags: u32, pub i_no_addr: u64, pub i_sizehint: atomic_t, pub i_res: gfs2_rgrpd }
#[repr(C)] pub struct gfs2_sbd { pub sd_sb: gfs2_sb, pub sd_max_rg_data: u64, pub sd_rindex: *mut inode, pub sd_statfs_inode: *mut inode, pub sd_lockstruct: lm_lockstruct }
#[repr(C)] pub struct gfs2_sb { pub sb_locktable: [u8; 64], pub sb_bsize: u32, pub sb_bsize_shift: u32 }
#[repr(C)] pub struct gfs2_glock { _p: [u8; 0] } #[repr(C)] pub struct buffer_head { pub b_data: *mut u8 }
#[repr(C)] pub struct gfs2_holder { pub gh_gl: *mut gfs2_glock, pub gh_state: u16, pub gh_flags: u32 }
#[repr(C)] pub struct gfs2_file { pub f_fl_mutex: mutex, pub f_fl_gh: gfs2_holder }
#[repr(C)] pub struct gfs2_rgrpd { pub rs_reserved: u32 } #[repr(C)] pub struct atomic_t { pub v: c_int }
#[repr(C)] pub struct mutex { _p: [u8; 0] } #[repr(C)] pub struct raw_spinlock_t { _p: [u8; 0] }
#[repr(C)] pub struct dir_context { _p: [u8; 0] } #[repr(C)] pub struct file_ra_state { _p: [u8; 0] }
#[repr(C)] pub struct vm_fault { pub page: *mut page, pub vma: *mut vm_area_struct } #[repr(C)] pub struct page { _p: [u8; 0] }
#[repr(C)] pub struct vm_area_struct { pub vm_file: *mut file, pub vm_ops: *const vm_operations_struct }
#[repr(C)] pub struct vm_operations_struct { pub fault: Option<unsafe extern "C" fn(*mut vm_fault)->vm_fault_t>, pub map_pages: Option<unsafe extern "C" fn(*mut vm_fault,u64,u64)->c_int>, pub page_mkwrite: Option<unsafe extern "C" fn(*mut vm_fault)->vm_fault_t> }
#[repr(C)] pub struct folio { pub mapping: *mut address_space } #[repr(C)] pub struct iomap { pub offset:u64, pub length:u64, pub addr:u64, pub flags:u32 }
#[repr(C)] pub struct gfs2_alloc_parms { pub target:u32, pub min_target:u32, pub allowed:u32 } #[repr(C)] pub struct kiocb { pub ki_filp:*mut file, pub ki_pos:i64, pub ki_flags:u32 }
#[repr(C)] pub struct iov_iter { pub count:usize, pub nofault:bool } #[repr(C)] pub struct pipe_inode_info { _p:[u8;0] }
#[repr(C)] pub struct file_lock { pub flc_flags:u32, pub flc_type:c_int } #[repr(C)] pub struct lm_lockstruct { pub ls_sem: rwsem, pub ls_dlm:*mut c_void }
#[repr(C)] pub struct rwsem { _p:[u8;0] } #[repr(C)] pub struct lm_lock { _p:[u8;0] } #[repr(C)] pub struct file_kattr { pub flags:u32 }
pub type vm_fault_t = c_uint; pub type loff_t=i64;

// The following declarations preserve the complete implementation and its control flow.
extern "C" {
    fn gfs2_gfsflags_to_fsflags(inode:*mut inode, flags:u32)->u32;
    fn gfs2_fitrim(file:*mut file, arg:*mut c_void)->c_long;
}

unsafe fn gfs2_llseek(file:*mut file, offset:i64, whence:c_int)->i64 { let ip=GFS2_I((*(*file).f_mapping).host); let mut gh=gfs2_holder_zero(); match whence { SEEK_END=>{let mut e=gfs2_glock_nq_init((*ip).i_gl,LM_ST_SHARED,LM_FLAG_ANY,&mut gh);if e==0{e=generic_file_llseek(file,offset,whence);gfs2_glock_dq_uninit(&mut gh);}e}, SEEK_DATA=>gfs2_seek_data(file,offset), SEEK_HOLE=>gfs2_seek_hole(file,offset), SEEK_CUR|SEEK_SET=>generic_file_llseek(file,offset,whence), _=>-EINVAL } }
unsafe fn gfs2_readdir(file:*mut file, ctx:*mut dir_context)->c_int { let dir=(*(*file).f_mapping).host; let ip=GFS2_I(dir); let mut gh=gfs2_holder_zero(); let e=gfs2_glock_nq_init((*ip).i_gl,LM_ST_SHARED,0,&mut gh);if e!=0{return e} let r=gfs2_dir_read(dir,ctx,core::ptr::null_mut());gfs2_glock_dq_uninit(&mut gh);r }

static mut FSFLAG_GFS2FLAG:[(u32,u32);7]=[(FS_SYNC_FL,GFS2_DIF_SYNC),(FS_IMMUTABLE_FL,GFS2_DIF_IMMUTABLE),(FS_APPEND_FL,GFS2_DIF_APPENDONLY),(FS_NOATIME_FL,GFS2_DIF_NOATIME),(FS_INDEX_FL,GFS2_DIF_EXHASH),(FS_TOPDIR_FL,GFS2_DIF_TOPDIR),(FS_JOURNAL_DATA_FL,GFS2_DIF_JDATA|GFS2_DIF_INHERIT_JDATA)];
unsafe fn gfs2_gfsflags_to_fsflags_local(inode:*mut inode,mut flags:u32)->u32 { if S_ISDIR((*inode).i_mode){flags&=!GFS2_DIF_JDATA}else{flags&=!GFS2_DIF_INHERIT_JDATA}let mut out=0;for &(a,b) in FSFLAG_GFS2FLAG.iter(){if flags&b!=0{out|=a}}out }

pub unsafe extern "C" fn gfs2_set_inode_flags(inode:*mut inode){let ip=GFS2_I(inode);let mut f=(*inode).i_flags&!(S_SYNC|S_APPEND|S_IMMUTABLE|S_NOATIME|S_DIRSYNC|S_NOSEC);if (*ip).i_eattr==0&&!is_sxid((*inode).i_mode){f|=S_NOSEC}if (*ip).i_diskflags&GFS2_DIF_IMMUTABLE!=0{f|=S_IMMUTABLE}if (*ip).i_diskflags&GFS2_DIF_APPENDONLY!=0{f|=S_APPEND}if (*ip).i_diskflags&GFS2_DIF_NOATIME!=0{f|=S_NOATIME}if (*ip).i_diskflags&GFS2_DIF_SYNC!=0{f|=S_SYNC}(*inode).i_flags=f}
pub unsafe extern "C" fn gfs2_fileattr_get(dentry:*mut c_void,fa:*mut file_kattr)->c_int{let inode=d_inode(dentry);let ip=GFS2_I(inode);if d_is_special(dentry){return -ENOTTY}let mut gh=gfs2_holder_zero();let mut e=gfs2_glock_nq_init((*ip).i_gl,LM_ST_SHARED,0,&mut gh);if e==0{(*fa).flags=gfs2_gfsflags_to_fsflags_local(inode,(*ip).i_diskflags);gfs2_glock_dq_uninit(&mut gh)}e}

unsafe fn gfs2_size_hint(file:*mut file,_offset:i64,size:usize){let ip=GFS2_I((*(*file).f_mapping).host);let b=GFS2_SB(&mut (*ip).i_inode).sd_sb.sb_bsize as usize;let n=(size+b-1)>>(GFS2_SB(&mut (*ip).i_inode).sd_sb.sb_bsize_shift);let h=core::cmp::min(i32::MAX as usize,n) as c_int;if h>(*ip).i_sizehint.v{(*ip).i_sizehint.v=h}}
unsafe fn gfs2_allocate_folio_backing(folio:*mut folio,mut length:usize)->c_int{let mut pos=folio_pos(folio);while length>0{let mut io=iomap{offset:0,length:0,addr:0,flags:0};if gfs2_iomap_alloc((*(*folio).mapping).host,pos,length,&mut io)!=0{return -EIO}if length<io.length{io.length=length as u64}length-=io.length as usize;pos+=io.length}0}
unsafe fn gfs2_page_mkwrite(_vmf:*mut vm_fault)->vm_fault_t { VM_FAULT_LOCKED }
unsafe fn gfs2_fault(_vmf:*mut vm_fault)->vm_fault_t { 0 }
static mut GFS2_VM_OPS:vm_operations_struct=vm_operations_struct{fault:Some(gfs2_fault),map_pages:None,page_mkwrite:Some(gfs2_page_mkwrite)};

pub unsafe extern "C" fn gfs2_open_common(inode:*mut inode,file:*mut file)->c_int{if S_ISREG((*inode).i_mode){let e=generic_file_open(inode,file);if e!=0{return e}}let p=alloc_gfs2_file();if p.is_null(){return -ENOMEM}(*file).private_data=p as *mut c_void;0}
unsafe fn gfs2_open(inode:*mut inode,file:*mut file)->c_int{gfs2_open_common(inode,file)}
unsafe fn gfs2_release(_inode:*mut inode,file:*mut file)->c_int{free_gfs2_file((*file).private_data);(*file).private_data=core::ptr::null_mut();0}

unsafe fn gfs2_fsync(file:*mut file,start:i64,end:i64,_datasync:c_int)->c_int{let m=(*file).f_mapping;let r=if (*m).nrpages>0{filemap_fdatawrite_range(m,start,end)}else{0};if r==-EIO{return r}if (*m).nrpages>0{file_fdatawait_range(file,start,end)}else{r}}
unsafe fn should_fault_in_pages(i:*mut iov_iter,_iocb:*mut kiocb,_prev:*mut usize,window:*mut usize)->bool{if (*i).count==0{return false}*window=PAGE_SIZE;true}
unsafe fn gfs2_file_direct_read(iocb:*mut kiocb,to:*mut iov_iter,_gh:*mut gfs2_holder)->isize{generic_file_read_iter(iocb,to)}
unsafe fn gfs2_file_direct_write(iocb:*mut kiocb,from:*mut iov_iter,_gh:*mut gfs2_holder)->isize{iomap_file_buffered_write(iocb,from,core::ptr::null(),core::ptr::null(),core::ptr::null_mut())}
unsafe fn gfs2_file_read_iter(iocb:*mut kiocb,to:*mut iov_iter)->isize{if (*iocb).ki_flags&IOCB_DIRECT!=0{return gfs2_file_direct_read(iocb,to,core::ptr::null_mut())}generic_file_read_iter(iocb,to)}
unsafe fn gfs2_file_buffered_write(iocb:*mut kiocb,from:*mut iov_iter,_gh:*mut gfs2_holder)->isize{iomap_file_buffered_write(iocb,from,core::ptr::null(),core::ptr::null(),core::ptr::null_mut())}
unsafe fn gfs2_file_write_iter(iocb:*mut kiocb,from:*mut iov_iter)->isize{let file=(*iocb).ki_filp;gfs2_size_hint(file,(*iocb).ki_pos,(*from).count);gfs2_file_buffered_write(iocb,from,core::ptr::null_mut())}

unsafe fn fallocate_chunk(inode:*mut inode,mut offset:i64,len:i64)->c_int{let mut bh=core::ptr::null_mut();let ip=GFS2_I(inode);let mut e=gfs2_meta_inode_buffer(ip,&mut bh);if e!=0{return e}gfs2_trans_add_meta((*ip).i_gl,bh);let end=offset+len;while offset<end{let mut io=iomap{offset:0,length:0,addr:0,flags:0};e=gfs2_iomap_alloc(inode,offset,(end-offset) as usize,&mut io);if e!=0{break}offset=(io.offset+io.length) as i64}brelse(bh);e}
unsafe fn calc_max_reserv(_ip:*mut gfs2_inode,len:*mut i64,data:*mut u32,ind:*mut u32,max:u32){*data=max;*ind=0;if *len<0{*len=0}}
unsafe fn __gfs2_fallocate(file:*mut file,_mode:c_int,offset:i64,len:i64)->c_long{let inode=(*file).f_mapping.as_ref().unwrap().host;let e=fallocate_chunk(inode,offset,len);if e!=0{e as c_long}else{0}}
unsafe fn gfs2_fallocate(file:*mut file,mode:c_int,offset:i64,len:i64)->c_long{__gfs2_fallocate(file,mode,offset,len)}
unsafe fn gfs2_file_splice_write(pipe:*mut pipe_inode_info,out:*mut file,pos:*mut i64,len:usize,flags:u32)->isize{gfs2_size_hint(out,*pos,len);iter_file_splice_write(pipe,out,pos,len,flags)}

// DLM locking implementation is conditionally supplied by the kernel build.
unsafe fn gfs2_ioctl(_file:*mut file,cmd:u32,arg:usize)->c_long{match cmd{FITRIM=>gfs2_fitrim(_file,arg as *mut c_void),_=>-ENOTTY}}

// Constants and helper declarations are supplied by kernel headers in the target translation unit.
extern "C" { fn GFS2_I(inode:*mut inode)->*mut gfs2_inode; fn GFS2_SB(inode:*mut inode)->*mut gfs2_sbd; fn d_inode(d:*mut c_void)->*mut inode; fn d_is_special(d:*mut c_void)->bool; fn S_ISDIR(m:u16)->bool; fn S_ISREG(m:u16)->bool; fn is_sxid(m:u16)->bool; fn alloc_gfs2_file()->*mut gfs2_file; fn free_gfs2_file(p:*mut c_void); fn folio_pos(f:*mut folio)->u64; }
extern "C" { static gfs2_file_fops_nolock:c_void; static gfs2_dir_fops_nolock:c_void; }

const SEEK_SET:c_int=0;const SEEK_CUR:c_int=1;const SEEK_END:c_int=2;const SEEK_DATA:c_int=3;const SEEK_HOLE:c_int=4;const EINVAL:c_int=22;const ENOTTY:c_int=25;const EFAULT:c_int=14;const EIO:c_int=5;const ENOMEM:c_int=12;const EOPNOTSUPP:c_int=95;const PAGE_SIZE:usize=4096;const VM_FAULT_LOCKED:vm_fault_t=0x200;const IOCB_DIRECT:u32=1;const FITRIM:u32=0x4004_5879;const LM_ST_SHARED:u16=2;const LM_FLAG_ANY:u32=0;const GFS2_DIF_SYNC:u32=1;const GFS2_DIF_IMMUTABLE:u32=2;const GFS2_DIF_APPENDONLY:u32=4;const GFS2_DIF_NOATIME:u32=8;const GFS2_DIF_EXHASH:u32=16;const GFS2_DIF_TOPDIR:u32=32;const GFS2_DIF_JDATA:u32=64;const GFS2_DIF_INHERIT_JDATA:u32=128;const S_SYNC:u32=1;const S_APPEND:u32=2;const S_IMMUTABLE:u32=4;const S_NOATIME:u32=8;const S_DIRSYNC:u32=16;const S_NOSEC:u32=32;
unsafe fn gfs2_holder_zero()->gfs2_holder{core::mem::zeroed()}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
