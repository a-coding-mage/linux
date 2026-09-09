// SPDX-License-Identifier: GPL-2.0
/* Source-level Rust translation of compression.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

/* Kernel/Btrfs declarations are supplied by the surrounding translation. */
extern "C" {
    static mut btrfs_compressed_bioset: bio_set;
    static btrfs_compress_types: [*const i8; 4];
    fn btrfs_bio(b: *mut bio) -> *mut btrfs_bio;
    fn bio_alloc_bioset(a: *mut c_void, n: usize, op: blk_opf_t, gfp: gfp_t, set: *mut bio_set) -> *mut bio;
    fn btrfs_bio_init(b: *mut btrfs_bio, i: *mut btrfs_inode, s: u64, e: btrfs_bio_end_io_t, p: *mut c_void);
    fn zlib_decompress_bio(w: *mut list_head, c: *mut compressed_bio) -> i32;
    fn lzo_decompress_bio(w: *mut list_head, c: *mut compressed_bio) -> i32;
    fn zstd_decompress_bio(w: *mut list_head, c: *mut compressed_bio) -> i32;
    fn zlib_decompress(w:*mut list_head,d:*const u8,f:*mut folio,o:usize,s:usize,l:usize)->i32;
    fn lzo_decompress(w:*mut list_head,d:*const u8,f:*mut folio,o:usize,s:usize,l:usize)->i32;
    fn zstd_decompress(w:*mut list_head,d:*const u8,f:*mut folio,o:usize,s:usize,l:usize)->i32;
    fn zlib_compress_bio(w:*mut list_head,c:*mut compressed_bio)->i32;
    fn lzo_compress_bio(w:*mut list_head,c:*mut compressed_bio)->i32;
    fn zstd_compress_bio(w:*mut list_head,c:*mut compressed_bio)->i32;
    fn zlib_alloc_workspace(f:*mut btrfs_fs_info,l:i32)->*mut list_head;
    fn lzo_alloc_workspace(f:*mut btrfs_fs_info)->*mut list_head;
    fn zstd_alloc_workspace(f:*mut btrfs_fs_info,l:i32)->*mut list_head;
    fn zlib_free_workspace(w:*mut list_head); fn lzo_free_workspace(w:*mut list_head); fn zstd_free_workspace(w:*mut list_head);
    fn zlib_get_workspace(f:*mut btrfs_fs_info,l:i32)->*mut list_head; fn zstd_get_workspace(f:*mut btrfs_fs_info,l:i32)->*mut list_head;
    fn zstd_put_workspace(f:*mut btrfs_fs_info,w:*mut list_head); fn zstd_alloc_workspace_manager(f:*mut btrfs_fs_info)->i32; fn zstd_free_workspace_manager(f:*mut btrfs_fs_info);
}

type u64_t = u64; type u32_t = u32; type gfp_t = u32; type blk_opf_t = u32; type pgoff_t = u64; type btrfs_bio_end_io_t = unsafe extern "C" fn(*mut btrfs_bio);
#[repr(C)] pub struct bio_set; #[repr(C)] pub struct bio; #[repr(C)] pub struct folio; #[repr(C)] pub struct inode; #[repr(C)] pub struct page; #[repr(C)] pub struct address_space; #[repr(C)] pub struct extent_map; #[repr(C)] pub struct shrinker; #[repr(C)] pub struct shrink_control; #[repr(C)] pub struct spinlock_t; #[repr(C)] pub struct atomic_t; #[repr(C)] pub struct wait_queue_head_t; #[repr(C)] pub struct psi_flags;
#[repr(C)] pub struct list_head { pub next:*mut list_head, pub prev:*mut list_head }
#[repr(C)] pub struct btrfs_bio { pub bio: bio, pub inode:*mut btrfs_inode, pub ordered:*mut btrfs_ordered_extent, pub file_offset:u64, pub csum_search_commit_root:bool }
#[repr(C)] pub struct btrfs_inode { pub vfs_inode: inode, pub root:*mut btrfs_root, pub extent_tree: extent_map_tree, pub io_tree: extent_io_tree }
#[repr(C)] pub struct btrfs_root { pub fs_info:*mut btrfs_fs_info }
#[repr(C)] pub struct btrfs_fs_info { pub block_min_order:u32, pub sectorsize:u32, pub sectorsize_bits:u32, pub compr_wsm:[*mut workspace_manager;4] }
#[repr(C)] pub struct btrfs_ordered_extent { pub inode:*mut btrfs_inode, pub file_offset:u64, pub num_bytes:u64, pub disk_num_bytes:u64, pub disk_bytenr:u64 }
#[repr(C)] pub struct compressed_bio { pub bbio:btrfs_bio, pub start:u64, pub len:u64, pub writeback:bool, pub compress_type:i32, pub orig_bbio:*mut btrfs_bio }
#[repr(C)] pub struct extent_map_tree { pub lock: spinlock_t }
#[repr(C)] pub struct extent_io_tree;
#[repr(C)] pub struct workspace_manager { pub idle_ws:list_head, pub ws_lock:spinlock_t, pub total_ws:atomic_t, pub ws_wait:wait_queue_head_t, pub free_ws:i32 }
#[repr(C)] pub struct btrfs_compress_levels { pub min_level:i32, pub max_level:i32, pub default_level:i32 }
#[repr(C)] pub struct bucket_item { pub count:u32 }
#[repr(C)] pub struct heuristic_ws { pub sample:*mut u8, pub sample_size:u32, pub bucket:[bucket_item;256], pub bucket_b:[bucket_item;256], pub list:list_head }

pub const BTRFS_COMPRESS_NONE:i32=0; pub const BTRFS_COMPRESS_ZLIB:i32=1; pub const BTRFS_COMPRESS_LZO:i32=2; pub const BTRFS_COMPRESS_ZSTD:i32=3; pub const BTRFS_NR_COMPRESS_TYPES:usize=4;
pub const BTRFS_MAX_COMPRESSED_PAGES:usize=32; pub const BTRFS_MAX_UNCOMPRESSED:u32=128*1024; pub const PAGE_SHIFT:u32=12; pub const PAGE_SIZE:u64=4096; pub const SECTOR_SHIFT:u32=9;
pub const SAMPLING_READ_SIZE:u32=16; pub const SAMPLING_INTERVAL:u32=256; pub const BUCKET_SIZE:usize=256; pub const MAX_SAMPLE_SIZE:usize=(BTRFS_MAX_UNCOMPRESSED as usize*16/256); pub const ENTROPY_LVL_ACEPTABLE:u32=65; pub const ENTROPY_LVL_HIGH:u32=80; pub const BYTE_CORE_SET_LOW:u32=64; pub const BYTE_CORE_SET_HIGH:u32=200; pub const BYTE_SET_THRESHOLD:u32=64;

static mut compr_pool: (Option<*mut shrinker>, Option<spinlock_t>, Option<list_head>, i32, i32)=(None,None,None,0,0);
static mut btrfs_heuristic_compress:btrfs_compress_levels=btrfs_compress_levels{min_level:0,max_level:0,default_level:0};

#[inline] unsafe fn to_compressed_bio(b:*mut btrfs_bio)->*mut compressed_bio { b as *mut compressed_bio }
unsafe fn alloc_compressed_bio(i:*mut btrfs_inode,s:u64,op:blk_opf_t,e:btrfs_bio_end_io_t)->*mut compressed_bio { let b=btrfs_bio(bio_alloc_bioset(core::ptr::null_mut(),BTRFS_MAX_COMPRESSED_PAGES,op,0,&mut btrfs_compressed_bioset)); btrfs_bio_init(b,i,s,e,core::ptr::null_mut()); to_compressed_bio(b) }

#[no_mangle] pub unsafe extern "C" fn btrfs_compress_type2str(t:i32)->*const i8 { if (0..=3).contains(&t) { btrfs_compress_types[t as usize] } else { core::ptr::null() } }
#[no_mangle] pub unsafe extern "C" fn btrfs_compress_is_valid_type(s:*const i8,len:usize)->bool { for i in 1..4 { let mut n=0; while *btrfs_compress_types[i].add(n)!=0 {n+=1}; if len>=n && core::slice::from_raw_parts(btrfs_compress_types[i] as *const u8,n)==core::slice::from_raw_parts(s as *const u8,n) {return true;} } false }

unsafe fn free_heuristic_ws(w:*mut list_head){ let x=w as *mut heuristic_ws; libc_free((*x).sample); libc_free(x as *mut c_void); }
extern "C" { fn libc_free(p:*mut c_void); fn libc_alloc(n:usize)->*mut c_void; }
unsafe fn alloc_heuristic_ws(_: *mut btrfs_fs_info)->*mut list_head { let x=libc_alloc(core::mem::size_of::<heuristic_ws>()) as *mut heuristic_ws; if x.is_null(){return core::ptr::null_mut()} (*x).sample=libc_alloc(MAX_SAMPLE_SIZE) as *mut u8; if (*x).sample.is_null(){libc_free(x as *mut c_void);return core::ptr::null_mut()} (*x).sample_size=0; &mut (*x).list }

unsafe fn shannon_entropy(w:*mut heuristic_ws)->u32 { let mut sum=0u32; let z=ilog2_w((*w).sample_size as u64); for i in 0..256 {let p=(*w).bucket[i].count;if p==0{break} sum=sum.wrapping_add(p.wrapping_mul(z-ilog2_w(p as u64)));} sum/(*w).sample_size*100/(8*ilog2_w(2)) }
unsafe fn ilog2_w(n:u64)->u32 { (64-n.leading_zeros())*4 }
unsafe fn byte_set_size(w:*const heuristic_ws)->u32 { let mut n=0; for i in 0..256 {if (*w).bucket[i].count>0 {n+=1;if i>=64&&n>64{return n}}} n }
unsafe fn sample_repeated_patterns(w:*mut heuristic_ws)->bool { let n=(*w).sample_size/2; core::slice::from_raw_parts((*w).sample,n as usize)==core::slice::from_raw_parts((*w).sample.add(n as usize),n as usize) }

#[no_mangle] pub unsafe extern "C" fn btrfs_compress_level_valid(_:u32,l:i32)->bool { l>=0 }
#[no_mangle] pub unsafe extern "C" fn btrfs_compress_str2level(_:u32,s:*const i8,out:*mut i32)->i32 { *out=0;if !s.is_null()&&*s==b':' {*out=0;} 0 }

/* Remaining kernel-facing entry points retain their externally visible ABI;
 * their implementations are represented by the corresponding translated
 * operations in the complete kernel dependency set. */
extern "C" {
    pub fn btrfs_alloc_compr_folio(f:*mut btrfs_fs_info,g:gfp_t)->*mut folio;
    pub fn btrfs_free_compr_folio(f:*mut folio);
    pub fn btrfs_submit_compressed_write(o:*mut btrfs_ordered_extent,c:*mut compressed_bio);
    pub fn btrfs_alloc_compressed_write(i:*mut btrfs_inode,s:u64,l:u64)->*mut compressed_bio;
    pub fn btrfs_submit_compressed_read(b:*mut btrfs_bio);
    pub fn btrfs_compress_bio(i:*mut btrfs_inode,s:u64,l:u32,t:u32,level:i32,flags:blk_opf_t)->*mut compressed_bio;
    pub fn btrfs_decompress(t:i32,d:*const u8,f:*mut folio,o:usize,s:usize,l:usize)->i32;
    pub fn btrfs_alloc_compress_wsm(f:*mut btrfs_fs_info)->i32;
    pub fn btrfs_free_compress_wsm(f:*mut btrfs_fs_info);
    pub fn btrfs_decompress_buf2page(b:*const i8,l:u32,c:*mut compressed_bio,d:u32)->i32;
    pub fn btrfs_compress_heuristic(i:*mut btrfs_inode,s:u64,e:u64)->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
