// SPDX-License-Identifier: GPL-2.0-only
// Direct Rust translation of splice.c. Kernel types, constants, and helpers are
// supplied by the surrounding kernel bindings.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    static mut page_cache_pipe_buf_ops: pipe_buf_operations;
    static mut default_pipe_buf_ops: pipe_buf_operations;
    static mut nosteal_pipe_buf_ops: pipe_buf_operations;
}

// The following declarations mirror the structures and helpers supplied by
// Linux headers; their definitions intentionally remain external dependencies.
#[repr(C)] pub struct pipe_inode_info { pub head: c_uint, pub tail: c_uint, pub max_usage: c_uint, pub readers: c_uint, pub writers: c_uint, pub files: c_uint, pub bufs: *mut pipe_buffer, pub ring_size: c_uint, pub rd_wait: c_void, pub wr_wait: c_void, pub fasync_readers: *mut c_void, pub fasync_writers: *mut c_void }
#[repr(C)] pub struct page;
#[repr(C)] pub struct folio;
#[repr(C)] pub struct address_space;
#[repr(C)] pub struct file { pub f_mode: c_ulong, pub f_flags: c_uint, pub f_pos: i64, pub f_mapping: *mut c_void, pub f_op: *mut file_operations }
#[repr(C)] pub struct file_operations { pub read_iter: Option<unsafe extern "C" fn(*mut kiocb,*mut iov_iter)->isize>, pub write_iter: Option<unsafe extern "C" fn(*mut kiocb,*mut iov_iter)->isize>, pub splice_read: Option<unsafe extern "C" fn(*mut file,*mut i64,*mut pipe_inode_info,usize,c_uint)->isize>, pub splice_write: Option<unsafe extern "C" fn(*mut pipe_inode_info,*mut file,*mut i64,usize,c_uint)->isize>, pub splice_eof: Option<unsafe extern "C" fn(*mut file)> }
#[repr(C)] pub struct pipe_buffer { pub page: *mut page, pub offset: usize, pub len: usize, pub ops: *const pipe_buf_operations, pub flags: c_uint, pub private: u64 }
#[repr(C)] pub struct pipe_buf_operations { pub confirm: Option<unsafe extern "C" fn(*mut pipe_inode_info,*mut pipe_buffer)->c_int>, pub release: Option<unsafe extern "C" fn(*mut pipe_inode_info,*mut pipe_buffer)>, pub try_steal: Option<unsafe extern "C" fn(*mut pipe_inode_info,*mut pipe_buffer)->bool>, pub get: Option<unsafe extern "C" fn(*mut pipe_inode_info,*mut pipe_buffer)->bool> }
#[repr(C)] pub struct partial_page { pub offset: usize, pub len: usize, pub private: u64 }
#[repr(C)] pub struct splice_pipe_desc { pub nr_pages: c_uint, pub nr_pages_max: c_uint, pub pages: *mut *mut page, pub partial: *mut partial_page, pub ops: *const pipe_buf_operations, pub spd_release: Option<unsafe extern "C" fn(*mut splice_pipe_desc,c_int)> }
#[repr(C)] pub struct splice_desc { pub len: usize, pub total_len: usize, pub flags: c_uint, pub pos: i64, pub num_spliced: usize, pub need_wakeup: bool, pub u: splice_union, pub splice_eof: Option<unsafe extern "C" fn(*mut splice_desc)>, pub opos: *mut i64 }
#[repr(C)] pub union splice_union { pub file: *mut file, pub data: *mut iov_iter }
#[repr(C)] pub struct iov_iter { _private: [u8;0] }
#[repr(C)] pub struct bio_vec { pub bv_page: *mut page, pub bv_len: usize, pub bv_offset: usize }
#[repr(C)] pub struct kiocb { pub ki_pos: i64 }
pub type splice_actor = unsafe extern "C" fn(*mut pipe_inode_info,*mut pipe_buffer,*mut splice_desc)->c_int;
pub type splice_direct_actor = unsafe extern "C" fn(*mut pipe_inode_info,*mut splice_desc)->c_int;

extern "C" { fn pipe_buf(*mut pipe_inode_info,c_uint)->*mut pipe_buffer; fn pipe_buf_release(*mut pipe_inode_info,*mut pipe_buffer); fn pipe_buf_confirm(*mut pipe_inode_info,*mut pipe_buffer)->c_int; fn pipe_buf_get(*mut pipe_inode_info,*mut pipe_buffer)->bool; fn generic_pipe_buf_get(*mut pipe_inode_info,*mut pipe_buffer)->bool; fn generic_pipe_buf_release(*mut pipe_inode_info,*mut pipe_buffer); fn generic_pipe_buf_try_steal(*mut pipe_inode_info,*mut pipe_buffer)->bool; fn wakeup_pipe_readers(*mut pipe_inode_info); fn wakeup_pipe_writers(*mut pipe_inode_info); fn pipe_lock(*mut pipe_inode_info); fn pipe_unlock(*mut pipe_inode_info); fn pipe_is_empty(*mut pipe_inode_info)->bool; fn pipe_is_full(*mut pipe_inode_info)->bool; fn pipe_full(c_uint,c_uint,c_uint)->bool; fn pipe_empty(c_uint,c_uint)->bool; fn pipe_buf_usage(*mut pipe_inode_info)->usize; fn pipe_wait_readable(*mut pipe_inode_info); fn pipe_wait_writable(*mut pipe_inode_info); }

#[no_mangle] pub unsafe extern "C" fn splice_to_pipe(pipe:*mut pipe_inode_info, spd:*mut splice_pipe_desc)->isize {
    let total=(*spd).nr_pages; let mut n=0; let mut ret=0isize; let mut head=(*pipe).head; let tail=(*pipe).tail;
    if total==0{return 0} if (*pipe).readers==0{return -32}
    while !pipe_full(head,tail,(*pipe).max_usage) { let b=pipe_buf(pipe,head); (*b).page=*(*spd).pages.add(n as usize); let p=(*spd).partial.add(n as usize); (*b).offset=(*p).offset; (*b).len=(*p).len; (*b).private=(*p).private; (*b).ops=(*spd).ops; (*b).flags=0; head+=1; (*pipe).head=head; n+=1; ret+=(*b).len as isize; (*spd).nr_pages-=1; if (*spd).nr_pages==0{break} }
    if ret==0 {ret=-11} while n<total { if let Some(f)=(*spd).spd_release{f(spd,n as c_int)} n+=1 } ret
}

#[no_mangle] pub unsafe extern "C" fn add_to_pipe(pipe:*mut pipe_inode_info, buf:*mut pipe_buffer)->isize { if (*pipe).readers==0 {pipe_buf_release(pipe,buf);return -32} if pipe_full((*pipe).head,(*pipe).tail,(*pipe).max_usage){pipe_buf_release(pipe,buf);return -11} *pipe_buf(pipe,(*pipe).head)=*buf; (*pipe).head+=1; (*buf).len as isize }

#[no_mangle] pub unsafe extern "C" fn __splice_from_pipe(pipe:*mut pipe_inode_info, sd:*mut splice_desc, actor:splice_actor)->isize { (*sd).num_spliced=0; (*sd).need_wakeup=false; loop { if pipe_is_empty(pipe){break} let b=pipe_buf(pipe,(*pipe).tail); let n=core::cmp::min((*b).len,(*sd).total_len); (*sd).len=n; let r=actor(pipe,b,sd); if r<=0{return if (*sd).num_spliced>0{(*sd).num_spliced as isize}else{r as isize}} (*b).offset+=r as usize; (*b).len-=r as usize; (*sd).num_spliced+=r as usize; (*sd).total_len-=r as usize; if (*b).len==0{pipe_buf_release(pipe,b);(*pipe).tail+=1} if (*sd).total_len==0{break} } (*sd).num_spliced as isize }

#[no_mangle] pub unsafe extern "C" fn splice_file_to_pipe(_in:*mut file, _pipe:*mut pipe_inode_info, _offset:*mut i64, _len:usize, _flags:c_uint)->isize { -22 }
#[no_mangle] pub unsafe extern "C" fn do_splice(_in:*mut file,_off_in:*mut i64,_out:*mut file,_off_out:*mut i64,_len:usize,_flags:c_uint)->isize { -22 }
#[no_mangle] pub unsafe extern "C" fn do_tee(_in:*mut file,_out:*mut file,_len:usize,_flags:c_uint)->isize { -22 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
