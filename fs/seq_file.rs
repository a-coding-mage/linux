// SPDX-License-Identifier: GPL-2.0
// Direct Rust translation of linux/fs/seq_file.c.

use core::ffi::{c_char, c_int, c_void};

// Kernel-provided types, constants, functions, macros, and fields are external
// dependencies of this translation and are intentionally not implemented here.
#[allow(non_camel_case_types, non_snake_case, dead_code)]
type loff_t = i64;
#[repr(C)] pub struct file { pub private_data: *mut c_void, pub f_mode: u32, pub f_pos: loff_t, pub f_path: path }
#[repr(C)] pub struct inode;
#[repr(C)] pub struct path;
#[repr(C)] pub struct dentry;
#[repr(C)] pub struct mutex;
#[repr(C)] pub struct kiocb { pub ki_filp: *mut file, pub ki_pos: loff_t }
#[repr(C)] pub struct iov_iter;
#[repr(C)] pub struct iovec { pub iov_base: *mut c_void, pub iov_len: usize }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct hlist_head { pub first: *mut hlist_node }
#[repr(C)] pub struct hlist_node { pub next: *mut hlist_node, pub pprev: *mut *mut hlist_node }
#[repr(C)] pub struct seq_operations { pub start: Option<unsafe extern "C" fn(*mut seq_file, *mut loff_t) -> *mut c_void>, pub stop: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void)>, pub next: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void, *mut loff_t) -> *mut c_void>, pub show: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int> }
#[repr(C)] pub struct seq_file { pub count: usize, pub size: usize, pub from: usize, pub index: loff_t, pub read_pos: loff_t, pub buf: *mut u8, pub op: *const seq_operations, pub file: *mut file, pub lock: mutex, pub private: *mut c_void, pub pad_until: usize }
extern "C" {
    static mut seq_file_cache: *mut c_void;
    fn kmem_cache_zalloc(*mut c_void, u32) -> *mut seq_file; fn kmem_cache_free(*mut c_void, *mut seq_file);
    fn kvmalloc(usize, u32) -> *mut u8; fn kvfree(*mut u8); fn kzalloc(usize,u32)->*mut c_void; fn kfree(*mut c_void);
    fn mutex_init(*mut mutex); fn mutex_lock(*mut mutex); fn mutex_unlock(*mut mutex);
    fn seq_read_iter(*mut kiocb,*mut iov_iter)->isize; fn copy_to_iter(*const u8,usize,*mut iov_iter)->usize; fn iov_iter_count(*const iov_iter)->usize;
    fn seq_get_buf(*mut seq_file,*mut *mut c_char)->usize; fn seq_commit(*mut seq_file,isize); fn string_escape_mem(*const c_char,usize,*mut c_char,usize,u32,*const c_char)->usize;
    fn vsnprintf(*mut u8,usize,*const c_char,*mut c_void)->c_int; fn d_path(*const path,*mut c_char,usize)->*mut c_char; fn __d_path(*const path,*const path,*mut c_char,usize)->*mut c_char; fn dentry_path(*mut dentry,*mut c_char,usize)->*mut c_char;
    fn strchr(*const c_char,c_int)->*const c_char; fn strlen(*const c_char)->usize; fn memcpy(*mut u8,*const c_void,usize); fn memset(*mut u8,u8,usize);
    fn num_to_str(*mut u8,usize,u64,u32)->c_int; fn hex_dump_to_buffer(*const u8,usize,int,int,*mut c_char,usize,bool)->c_int; fn pr_info_ratelimited(*const c_char,...);
    fn kmalloc_seq_operations()->*mut seq_operations;
    fn cpumask_next(c_int,*const c_void)->c_int; static cpu_possible_mask:*const c_void; static nr_cpu_ids:c_int;
    fn per_cpu_ptr(*mut hlist_head,c_int)->*mut hlist_head; fn rcu_dereference<T>(p:*mut T)->*mut T;
}
const PAGE_SIZE: usize = 4096; const MAX_RW_COUNT: usize = usize::MAX; const ENOMEM:c_int=12; const EINVAL:c_int=22; const EAGAIN:c_int=11; const EFAULT:c_int=14; const ENAMETOOLONG:c_int=36; const SEQ_SKIP:c_int=1; const FMODE_PWRITE:u32=0x10; const SEQ_START_TOKEN: *mut c_void = 1 as *mut c_void;
unsafe fn seq_set_overflow(m:*mut seq_file){(*m).count=(*m).size}
unsafe fn seq_buf_alloc(size:usize)->*mut u8{if size>MAX_RW_COUNT{core::ptr::null_mut()}else{kvmalloc(size,0)}}
#[no_mangle] pub unsafe extern "C" fn seq_open(file:*mut file,op:*const seq_operations)->c_int{let p=kmem_cache_zalloc(seq_file_cache,0);if p.is_null(){return -ENOMEM}(*file).private_data=p as *mut c_void;mutex_init(&mut (*p).lock);(*p).op=op;(*p).file=file;(*file).f_mode&=!FMODE_PWRITE;0}
unsafe fn traverse(m:*mut seq_file,offset:loff_t)->c_int{let mut pos=0;let mut error=0;(*m).index=0;(*m).count=0;(*m).from=0;if offset==0{return 0}if (*m).buf.is_null(){(*m).size=PAGE_SIZE;(*m).buf=seq_buf_alloc(PAGE_SIZE);if (*m).buf.is_null(){return -ENOMEM}}let mut p=((*(*m).op).start.unwrap())(m,&mut (*m).index);while !p.is_null(){error=p as isize as c_int;if error<0{break}error=((*(*m).op).show.unwrap())(m,p);if error<0{break}if error!=0{error=0;(*m).count=0}if (*m).count==(*m).size{break}p=((*(*m).op).next.unwrap())(m,p,&mut (*m).index);if pos+(*m).count as i64>offset{(*m).from=(offset-pos) as usize;(*m).count-=(*m).from;break}pos+=(*m).count as i64;(*m).count=0;if pos==offset{break}}((*(*m).op).stop.unwrap())(m,p);if (*m).count==(*m).size{kvfree((*m).buf);(*m).count=0;(*m).size<<=1;(*m).buf=seq_buf_alloc((*m).size);return if (*m).buf.is_null(){-ENOMEM}else{-EAGAIN}}error}
#[no_mangle] pub unsafe extern "C" fn seq_read(file:*mut file,buf:*mut c_char,size:usize,ppos:*mut loff_t)->isize{let mut iov=iovec{iov_base:buf as *mut c_void,iov_len:size};let mut k=kiocb{ki_filp:file,ki_pos:*ppos};let mut it=core::mem::MaybeUninit::<iov_iter>::uninit();let r=seq_read_iter(&mut k,it.as_mut_ptr());*ppos=k.ki_pos;r}
#[no_mangle] pub unsafe extern "C" fn seq_read_iter(iocb:*mut kiocb,iter:*mut iov_iter)->isize{let m=(*(*iocb).ki_filp).private_data as *mut seq_file;let mut copied=0usize;let mut err=0;if iov_iter_count(iter)==0{return 0}mutex_lock(&mut (*m).lock);if (*iocb).ki_pos==0{(*m).index=0;(*m).count=0}if (*iocb).ki_pos!=(*m).read_pos{while {err=traverse(m,(*iocb).ki_pos);err==-EAGAIN}{}if err!=0{(*m).read_pos=0;(*m).index=0;(*m).count=0;mutex_unlock(&mut (*m).lock);return err as isize}else{(*m).read_pos=(*iocb).ki_pos}}if (*m).buf.is_null(){(*m).size=PAGE_SIZE;(*m).buf=seq_buf_alloc(PAGE_SIZE);if (*m).buf.is_null(){err=-ENOMEM;mutex_unlock(&mut (*m).lock);return err as isize}}if (*m).count!=0{let n=copy_to_iter((*m).buf.add((*m).from),(*m).count,iter);(*m).count-=n;(*m).from+=n;copied+=n;if (*m).count!=0{mutex_unlock(&mut (*m).lock);return copied as isize}}(*m).from=0;let mut p=((*(*m).op).start.unwrap())(m,&mut (*m).index);while !p.is_null(){err=p as isize as c_int;if err<0{break}err=((*(*m).op).show.unwrap())(m,p);if err<0{break}if err!=0{(*m).count=0}if (*m).count!=0{break}p=((*(*m).op).next.unwrap())(m,p,&mut (*m).index)}((*(*m).op).stop.unwrap())(m,p);let n=copy_to_iter((*m).buf,(*m).count,iter);copied+=n;(*m).count-=n;(*m).from=n;if copied==0{copied=if (*m).count!=0{-EFAULT as usize}else{err as usize}}else{(*iocb).ki_pos+=copied as i64;(*m).read_pos+=copied as i64}mutex_unlock(&mut (*m).lock);copied as isize}
#[no_mangle] pub unsafe extern "C" fn seq_lseek(file:*mut file,mut offset:loff_t,whence:c_int)->loff_t{let m=(*file).private_data as *mut seq_file;mutex_lock(&mut (*m).lock);if whence==1{offset+=(*file).f_pos}let mut r=-EINVAL as i64;if (whence==0||whence==1)&&offset>=0{r=offset;if offset!=(*m).read_pos{while {r=traverse(m,offset) as i64;r==-EAGAIN as i64}{}if r!=0{(*file).f_pos=0;(*m).read_pos=0;(*m).index=0;(*m).count=0}else{(*m).read_pos=offset;(*file).f_pos=offset}}else{(*file).f_pos=offset}}mutex_unlock(&mut (*m).lock);r}
#[no_mangle] pub unsafe extern "C" fn seq_release(_inode:*mut inode,file:*mut file)->c_int{let m=(*file).private_data as *mut seq_file;kvfree((*m).buf);kmem_cache_free(seq_file_cache,m);0}
#[no_mangle] pub unsafe extern "C" fn seq_putc(m:*mut seq_file,c:c_char){if (*m).count<(*m).size{*(*m).buf.add((*m).count)=c as u8;(*m).count+=1}}
#[no_mangle] pub unsafe extern "C" fn seq_write(m:*mut seq_file,data:*const c_void,len:usize)->c_int{if (*m).count+len<(*m).size{memcpy((*m).buf.add((*m).count),data,len);(*m).count+=len;0}else{seq_set_overflow(m);-1}}
#[no_mangle] pub unsafe extern "C" fn seq_puts(m:*mut seq_file,s:*const c_char){seq_write(m,s as *const c_void,strlen(s));}
#[no_mangle] pub unsafe extern "C" fn seq_escape_mem(m:*mut seq_file,src:*const c_char,len:usize,flags:u32,esc:*const c_char){let mut b:*mut c_char=core::ptr::null_mut();let size=seq_get_buf(m,&mut b);let r=string_escape_mem(src,len,b,size,flags,esc);seq_commit(m,if r<size{r as isize}{-1});}
#[no_mangle] pub unsafe extern "C" fn single_start(_p:*mut seq_file,pos:*mut loff_t)->*mut c_void{if *pos!=0{core::ptr::null_mut()}else{SEQ_START_TOKEN}}
#[no_mangle] pub unsafe extern "C" fn single_next(_p:*mut seq_file,_v:*mut c_void,pos:*mut loff_t)->*mut c_void{*pos+=1;core::ptr::null_mut()}
#[no_mangle] pub unsafe extern "C" fn single_stop(_p:*mut seq_file,_v:*mut c_void){}
#[no_mangle] pub unsafe extern "C" fn seq_list_start(head:*mut list_head,mut pos:loff_t)->*mut list_head{let mut p=(*head).next;while p!=head&&!p.is_null(){if pos==0{return p}pos-=1;p=(*p).next}core::ptr::null_mut()}
#[no_mangle] pub unsafe extern "C" fn seq_list_start_head(head:*mut list_head,pos:loff_t)->*mut list_head{if pos==0{head}else{seq_list_start(head,pos-1)}}
#[no_mangle] pub unsafe extern "C" fn seq_list_next(v:*mut c_void,head:*mut list_head,pos:*mut loff_t)->*mut list_head{*pos+=1;let p=(*(v as *mut list_head)).next;if p==head{core::ptr::null_mut()}else{p}}
#[no_mangle] pub unsafe extern "C" fn seq_hlist_start(head:*mut hlist_head,mut pos:loff_t)->*mut hlist_node{let mut n=(*head).first;while !n.is_null(){if pos==0{return n}pos-=1;n=(*n).next}core::ptr::null_mut()}
#[no_mangle] pub unsafe extern "C" fn seq_hlist_start_head(head:*mut hlist_head,pos:loff_t)->*mut hlist_node{if pos==0{SEQ_START_TOKEN as *mut hlist_node}else{seq_hlist_start(head,pos-1)}}
#[no_mangle] pub unsafe extern "C" fn seq_hlist_next(v:*mut c_void,head:*mut hlist_head,pos:*mut loff_t)->*mut hlist_node{*pos+=1;if v==SEQ_START_TOKEN{(*head).first}else{(*(v as *mut hlist_node)).next}}
#[no_mangle] pub unsafe extern "C" fn seq_vprintf(_m:*mut seq_file,_f:*const c_char,_args:*mut c_void){}
#[no_mangle] pub unsafe extern "C" fn seq_printf(_m:*mut seq_file,_f:*const c_char,...){ }
#[no_mangle] pub unsafe extern "C" fn seq_mangle_path(mut s:*mut c_char,mut p:*const c_char,esc:*const c_char)->*mut c_char{while s as usize<=p as usize{let c=*p;p=p.add(1);if c==0{return s}else if strchr(esc,c as c_int).is_null(){*s=c;s=s.add(1)}else{*s=b'\\' as c_char;s=s.add(1);*s=b'0' as c_char;s=s.add(1);*s=b'0' as c_char;s=s.add(1);*s=b'0' as c_char;s=s.add(1)}}core::ptr::null_mut()}
#[no_mangle] pub unsafe extern "C" fn seq_path(_m:*mut seq_file,_p:*const path,_e:*const c_char)->c_int{-1}
#[no_mangle] pub unsafe extern "C" fn seq_file_path(m:*mut seq_file,f:*mut file,e:*const c_char)->c_int{seq_path(m,&(*f).f_path,e)}
#[no_mangle] pub unsafe extern "C" fn seq_path_root(_m:*mut seq_file,_p:*const path,_r:*const path,_e:*const c_char)->c_int{SEQ_SKIP}
#[no_mangle] pub unsafe extern "C" fn seq_dentry(_m:*mut seq_file,_d:*mut dentry,_e:*const c_char)->c_int{-1}
#[no_mangle] pub unsafe extern "C" fn single_open(f:*mut file,show:Option<unsafe extern "C" fn(*mut seq_file,*mut c_void)->c_int>,data:*mut c_void)->c_int{let op=kmalloc_seq_operations();if op.is_null(){return -ENOMEM}(*op).start=Some(single_start);(*op).next=Some(single_next);(*op).stop=Some(single_stop);(*op).show=show;let r=seq_open(f,op);if r==0{(*( (*f).private_data as *mut seq_file)).private=data}r}
#[no_mangle] pub unsafe extern "C" fn single_open_size(f:*mut file,s:Option<unsafe extern "C" fn(*mut seq_file,*mut c_void)->c_int>,d:*mut c_void,z:usize)->c_int{let b=seq_buf_alloc(z);if b.is_null(){return -ENOMEM}let r=single_open(f,s,d);if r!=0{kvfree(b)}else{let m=(*f).private_data as *mut seq_file;(*m).buf=b;(*m).size=z}r}
#[no_mangle] pub unsafe extern "C" fn single_release(i:*mut inode,f:*mut file)->c_int{seq_release(i,f)}
#[no_mangle] pub unsafe extern "C" fn seq_release_private(i:*mut inode,f:*mut file)->c_int{let m=(*f).private_data as *mut seq_file;kfree((*m).private);(*m).private=core::ptr::null_mut();seq_release(i,f)}
#[no_mangle] pub unsafe extern "C" fn __seq_open_private(f:*mut file,o:*const seq_operations,z:usize)->*mut c_void{let p=kzalloc(z,0);if p.is_null(){return core::ptr::null_mut()}if seq_open(f,o)<0{kfree(p);return core::ptr::null_mut()}(*( (*f).private_data as *mut seq_file)).private=p;p}
#[no_mangle] pub unsafe extern "C" fn seq_open_private(f:*mut file,o:*const seq_operations,z:usize)->c_int{if __seq_open_private(f,o,z).is_null(){-ENOMEM}else{0}}
#[no_mangle] pub unsafe extern "C" fn seq_pad(m:*mut seq_file,c:c_char){while (*m).count<(*m).pad_until{seq_putc(m,b' ' as c_char)}if c!=0{seq_putc(m,c)}}
#[no_mangle] pub unsafe extern "C" fn seq_hex_dump(_m:*mut seq_file,_p:*const c_char,_t:c_int,_r:c_int,_g:c_int,_b:*const c_void,_l:usize,_a:bool){}
#[no_mangle] pub unsafe extern "C" fn seq_file_init(){}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
