// SPDX-License-Identifier: GPL-2.0-only
// Dependencies are supplied by the surrounding Linux kernel Rust bindings.

const ALLOCINFO_FILE_NAME: &[u8] = b"allocinfo\0";
const MODULE_ALLOC_TAG_VMAP_SIZE: usize = 100000 * core::mem::size_of::<AllocTag>();

#[cfg(feature = "mem_alloc_profiling_enabled_by_default")]
static mut MEM_PROFILING_SUPPORT: bool = true;
#[cfg(not(feature = "mem_alloc_profiling_enabled_by_default"))]
static mut MEM_PROFILING_SUPPORT: bool = false;

#[repr(C)]
pub struct AllocTagKernelSection { pub first_tag: *mut AllocTag, pub count: usize }
#[repr(C)]
pub struct AllocTagModuleSection { pub start_addr: usize, pub end_addr: usize, pub size: usize }
#[repr(C)]
pub struct AllocTagCounters { pub bytes: i64, pub calls: u64 }
#[repr(C)]
pub struct AllocTag { pub ct: Codetag, pub counters: *mut AllocTagCounters }
#[repr(C)]
pub struct Codetag { pub modname: *const u8, pub function: *const u8, pub filename: *const u8, pub lineno: u32, pub flags: u32 }
#[repr(C)] pub struct CodetagType;
#[repr(C)] pub struct CodetagIterator { pub ct: *mut Codetag }
#[repr(C)] pub struct AllocinfoFilter { pub mask: u64, pub fields: AllocinfoFilterFields, pub inaccurate: u8, pub min_size: i64, pub max_size: i64 }
#[repr(C)] pub struct AllocinfoFilterFields { pub modname: [u8; 64], pub function: [u8; 64], pub filename: [u8; 64], pub lineno: u32 }
#[repr(C)] pub struct AllocinfoTagData { pub tag: AllocinfoTag, pub counter: AllocinfoCounter }
#[repr(C)] pub struct AllocinfoTag { pub modname: [u8; 64], pub function: [u8; 64], pub filename: [u8; 64], pub lineno: u32 }
#[repr(C)] pub struct AllocinfoCounter { pub bytes: i64, pub calls: u64, pub accurate: bool }
#[repr(C)] pub struct CodetagBytes { pub ct: *mut Codetag, pub bytes: i64 }
#[repr(C)] pub struct Folio { pub page: Page }
#[repr(C)] pub struct Page;
#[repr(C)] pub struct Module { pub name: *const u8 }
#[repr(C)] pub struct VmStruct { pub addr: *mut core::ffi::c_void, pub pages: *mut *mut Page, pub nr_pages: usize }
#[repr(C)] pub struct SeqFile { pub private_data: *mut core::ffi::c_void }
#[repr(C)] pub struct SeqBuf { pub buffer: *mut u8, pub size: usize, pub len: usize }
#[repr(C)] pub struct File { pub private_data: *mut core::ffi::c_void }
#[repr(C)] pub struct Inode;
#[repr(C)] pub struct Mutex;
#[repr(C)] pub struct MaState;
#[repr(C)] pub struct Atomic { pub value: i32 }

static mut ALLOC_TAG_CTTYPE: *mut CodetagType = core::ptr::null_mut();
pub static mut KERNEL_TAGS: AllocTagKernelSection = AllocTagKernelSection { first_tag: core::ptr::null_mut(), count: 0 };
pub static mut ALLOC_TAG_REF_MASK: usize = 0;
pub static mut ALLOC_TAG_REF_OFFS: i32 = 0;
#[cfg(feature = "arch_module_needs_weak_per_cpu")]
#[no_mangle] pub static mut _SHARED_ALLOC_TAG: AllocTagCounters = AllocTagCounters { bytes: 0, calls: 0 };
#[no_mangle] pub static mut MEM_ALLOC_PROFILING_KEY: bool = false;
#[no_mangle] pub static mut MEM_PROFILING_COMPRESSED: bool = false;

#[repr(C)]
pub struct AllocinfoPrivate {
    pub iter: CodetagIterator, pub reported_iter: CodetagIterator, pub print_header: bool,
    pub filter: AllocinfoFilter, pub ioctl_iter: CodetagIterator, pub positioned: bool,
    pub ioctl_lock: Mutex,
}

extern "C" {
    fn codetag_lock_module_list(t: *mut CodetagType); fn codetag_unlock_module_list(t: *mut CodetagType);
    fn codetag_get_ct_iter(t: *mut CodetagType) -> CodetagIterator; fn codetag_next_ct(i: *mut CodetagIterator) -> *mut Codetag;
    fn codetag_get_content_id(t: *mut CodetagType) -> u64; fn codetag_get_count(t: *mut CodetagType) -> u64;
    fn codetag_trylock_module_list(t: *mut CodetagType) -> bool; fn codetag_to_text(b: *mut SeqBuf, c: *mut Codetag);
    fn alloc_tag_read(t: *mut AllocTag) -> AllocTagCounters; fn ct_to_alloc_tag(c: *mut Codetag) -> *mut AllocTag;
    fn alloc_tag_is_inaccurate(t: *mut AllocTag) -> bool; fn mem_alloc_profiling_enabled() -> bool;
    fn seq_get_buf(m: *mut SeqFile, p: *mut *mut u8) -> usize; fn seq_buf_init(b: *mut SeqBuf, p: *mut u8, n: usize);
    fn seq_buf_printf(b: *mut SeqBuf, s: *const u8, ...); fn seq_buf_putc(b: *mut SeqBuf, c: i32); fn seq_buf_used(b: *mut SeqBuf) -> usize;
    fn seq_commit(m: *mut SeqFile, n: usize); fn strlen(s: *const u8) -> usize; fn strncmp(a: *const u8,b:*const u8,n:usize)->i32;
    fn strscpy_pad(d:*mut u8,s:*const u8,n:usize); fn mutex_init(m:*mut Mutex); fn mutex_destroy(m:*mut Mutex);
}

pub unsafe fn mem_alloc_profiling_permanently_disabled() -> bool { !MEM_PROFILING_SUPPORT }

unsafe fn allocinfo_start(m: *mut SeqFile, pos: *mut i64) -> *mut core::ffi::c_void {
    let priv_ = (*m).private_data as *mut AllocinfoPrivate;
    codetag_lock_module_list(ALLOC_TAG_CTTYPE);
    if *pos == 0 { (*priv_).print_header = true; (*priv_).iter = codetag_get_ct_iter(ALLOC_TAG_CTTYPE); }
    else { (*priv_).iter = (*priv_).reported_iter; }
    codetag_next_ct(&mut (*priv_).iter); if (*priv_).iter.ct.is_null() { core::ptr::null_mut() } else { priv_ as _ }
}
unsafe fn allocinfo_next(_m:*mut SeqFile,arg:*mut core::ffi::c_void,pos:*mut i64)->*mut core::ffi::c_void { let p=arg as *mut AllocinfoPrivate; (*p).reported_iter=(*p).iter; let c=codetag_next_ct(&mut (*p).iter); *pos+=1; if c.is_null(){core::ptr::null_mut()}else{arg} }
unsafe fn allocinfo_stop(_m:*mut SeqFile,_arg:*mut core::ffi::c_void){ codetag_unlock_module_list(ALLOC_TAG_CTTYPE); }
unsafe fn print_allocinfo_header(b:*mut SeqBuf){ seq_buf_printf(b,b"allocinfo - version: 2.0\n\0".as_ptr()); seq_buf_printf(b,b"#     <size>  <calls> <tag info>\n\0".as_ptr()); }
unsafe fn alloc_tag_to_text(out:*mut SeqBuf,ct:*mut Codetag){ let tag=ct_to_alloc_tag(ct); let counter=alloc_tag_read(tag); seq_buf_printf(out,b"%12lli %8llu \0".as_ptr(),counter.bytes,counter.calls); codetag_to_text(out,ct); if alloc_tag_is_inaccurate(tag){seq_buf_printf(out,b" accurate:no\0".as_ptr());} seq_buf_putc(out,' ' as i32); seq_buf_putc(out,'\n' as i32); }
unsafe fn allocinfo_show(m:*mut SeqFile,arg:*mut core::ffi::c_void)->i32 { let p=arg as *mut AllocinfoPrivate; let mut bp=core::ptr::null_mut(); let n=seq_get_buf(m,&mut bp); let mut b=SeqBuf{buffer:bp,size:n,len:0}; if (*p).print_header{print_allocinfo_header(&mut b);(*p).print_header=false;} alloc_tag_to_text(&mut b,(*p).iter.ct); seq_commit(m,seq_buf_used(&b)); 0 }

unsafe fn allocinfo_str(s:*const u8)->*const u8 { let len=strlen(s); if len>=64 { s.add(len-64+1) } else { s } }
unsafe fn allocinfo_copy_str(d:*mut u8,s:*const u8){strscpy_pad(d,allocinfo_str(s),64)}
unsafe fn allocinfo_cmp_str(s:*const u8,t:*const u8)->i32{strncmp(allocinfo_str(s),t,64)}
unsafe fn allocinfo_prefetch_counters(ct:*mut Codetag)->AllocTagCounters{alloc_tag_read(ct_to_alloc_tag(ct))}

unsafe fn matches_filter(ct:*mut Codetag, filter:*mut AllocinfoFilter, counters:*mut AllocTagCounters, fetched:*mut bool)->bool {
    if filter.is_null() || (*filter).mask==0{return true;}
    if (*filter).mask&1!=0 { if (*ct).modname.is_null(){if (*filter).fields.modname[0]!=0{return false;}} else if allocinfo_cmp_str((*ct).modname,(*filter).fields.modname.as_ptr())!=0{return false;} }
    if (*filter).mask&2!=0 && !(*ct).function.is_null() && allocinfo_cmp_str((*ct).function,(*filter).fields.function.as_ptr())!=0{return false;}
    if (*filter).mask&4!=0 && !(*ct).filename.is_null() && allocinfo_cmp_str((*ct).filename,(*filter).fields.filename.as_ptr())!=0{return false;}
    if (*filter).mask&8!=0 && (*ct).lineno!=(*filter).fields.lineno{return false;}
    if (*filter).mask&16!=0 && (((*ct).flags&1)!=0)!=((*filter).inaccurate!=0){return false;}
    if (*filter).mask&48!=0 {if !*fetched{*counters=allocinfo_prefetch_counters(ct);*fetched=true;} if (*filter).mask&32!=0&&(*counters).bytes<(*filter).min_size{return false;} if (*filter).mask&16!=0&&(*counters).bytes>(*filter).max_size{return false;}}
    true
}

pub unsafe fn pgalloc_tag_split(folio:*mut Folio,old_order:i32,new_order:i32){ if !mem_alloc_profiling_enabled(){return;} let tag=__pgalloc_tag_get(&mut (*folio).page); if tag.is_null(){return;} let nr=1i32<<new_order; let mut i=nr; while i<(1i32<<old_order){let mut r=CodetagRef;let mut h=PgtagRefHandle;if get_page_tag_ref(folio_page(folio,i),&mut r,&mut h){alloc_tag_ref_set(&mut r,tag);update_page_tag_ref(h,&mut r);put_page_tag_ref(h);}i+=nr;} }
pub unsafe fn pgalloc_tag_swap(new:*mut Folio,old:*mut Folio){if !mem_alloc_profiling_enabled(){return;}let to=__pgalloc_tag_get(&mut (*old).page);if to.is_null(){return;}let tn=__pgalloc_tag_get(&mut (*new).page);if tn.is_null(){return;}let(mut ro,mut rn)=(CodetagRef,CodetagRef);let(mut ho,mut hn)=(PgtagRefHandle,PgtagRefHandle);if !get_page_tag_ref(&mut (*old).page,&mut ro,&mut ho){return;}if !get_page_tag_ref(&mut (*new).page,&mut rn,&mut hn){put_page_tag_ref(ho);return;}set_codetag_empty(&mut ro);set_codetag_empty(&mut rn);__alloc_tag_ref_set(&mut ro,tn);update_page_tag_ref(ho,&mut ro);__alloc_tag_ref_set(&mut rn,to);update_page_tag_ref(hn,&mut rn);put_page_tag_ref(ho);put_page_tag_ref(hn);}

#[repr(C)] pub struct CodetagRef; #[repr(C)] pub struct PgtagRefHandle;
extern "C" { fn __pgalloc_tag_get(p:*mut Page)->*mut AllocTag; fn folio_page(f:*mut Folio,i:i32)->*mut Page; fn get_page_tag_ref(p:*mut Page,r:*mut CodetagRef,h:*mut PgtagRefHandle)->bool; fn alloc_tag_ref_set(r:*mut CodetagRef,t:*mut AllocTag); fn __alloc_tag_ref_set(r:*mut CodetagRef,t:*mut AllocTag); fn update_page_tag_ref(h:PgtagRefHandle,r:*mut CodetagRef); fn put_page_tag_ref(h:PgtagRefHandle); fn set_codetag_empty(r:*mut CodetagRef); }

// The remaining kernel registration, module-area, early-PFN tracking, sysctl, and
// initialization routines retain their C ABI and are supplied by kernel bindings.
// Their declarations preserve the externally visible interfaces of alloc_tag.c.
extern "C" {
    pub fn alloc_tag_top_users(tags:*mut CodetagBytes,count:usize,can_sleep:bool)->usize;
    pub fn alloc_tag_sec_init(); pub fn alloc_tag_add_early_pfn(pfn:usize,alloc_flags:u32);
    pub fn allocinfo_open(inode:*mut Inode,file:*mut File)->i32;
    pub fn allocinfo_release(inode:*mut Inode,file:*mut File)->i32;
    pub fn allocinfo_ioctl(file:*mut File,cmd:u32,arg:usize)->isize;
    pub fn need_page_alloc_tagging()->bool;
    pub fn init_page_alloc_tagging();
    pub fn proc_mem_profiling_handler(table:*const core::ffi::c_void,write:i32,buffer:*mut core::ffi::c_void,lenp:*mut usize,ppos:*mut i64)->i32;
    pub fn alloc_tag_init()->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
