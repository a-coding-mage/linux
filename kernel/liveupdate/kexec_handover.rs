// SPDX-License-Identifier: GPL-2.0-only
// Faithful low-level Rust translation of kexec_handover.c.  Kernel-provided
// types, constants, macros, and functions remain external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{ffi::{c_char, c_int, c_void}, mem::MaybeUninit, ptr};

extern "C" {
    fn kstrtobool(p: *mut c_char, v: *mut bool) -> c_int;
    fn phys_to_virt(p: u64) -> *mut c_void;
    fn virt_to_phys(p: *const c_void) -> u64;
    fn fls64(v: u64) -> u32;
    fn memblock_alloc(size: usize, align: usize) -> *mut c_void;
    fn get_zeroed_page(gfp: u64) -> *mut c_void;
    fn free_page(p: usize);
    fn memblock_free(p: *mut c_void, size: usize);
    fn mutex_init(lock: *mut c_void);
    fn set_bit(n: usize, p: *mut usize);
    fn clear_bit(n: usize, p: *mut usize);
}

#[repr(C)]
pub union kho_page_info {
    pub page_private: usize,
    pub fields: kho_page_info_fields,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kho_page_info_fields { pub order: u32, pub magic: u32 }

#[repr(C)]
pub struct kho_radix_node { pub table: [u64; 512] }
#[repr(C)]
pub struct kho_radix_leaf { pub bitmap: [usize; 512 / core::mem::size_of::<usize>()] }
#[repr(C)]
pub struct kho_radix_tree { pub root: *mut kho_radix_node, pub lock: [usize; 8] }
#[repr(C)] pub struct kho_debugfs { _private: [u8; 0] }
#[repr(C)] pub struct page { pub private: usize, _private: [u8; 0] }
#[repr(C)] pub struct folio { _private: [u8; 0] }
#[repr(C)] pub struct kho_scratch { pub addr: u64, pub size: u64 }
#[repr(C)] pub struct kho_vmalloc { pub first: *mut kho_vmalloc_chunk, pub total_pages: usize, pub flags: u16, pub order: u32 }
#[repr(C)] pub struct kho_vmalloc_chunk { pub hdr: kho_vmalloc_chunk_hdr, pub phys: [u64; 64] }
#[repr(C)] pub struct kho_vmalloc_chunk_hdr { pub next: *mut kho_vmalloc_chunk }
#[repr(C)] pub struct kho_radix_walk_cb { pub leaf: Option<unsafe extern "C" fn(usize,*mut c_void)->c_int>, pub node: Option<unsafe extern "C" fn(u64,*mut c_void)->c_int> }

const KHO_PAGE_MAGIC: u32 = 0x4b484f50;
const PAGE_SHIFT: u32 = 12;
const KHO_BITMAP_SIZE_LOG2: u32 = 12;
const KHO_TABLE_SIZE_LOG2: u32 = 9;
const KHO_TREE_MAX_DEPTH: u32 = 5;
const KHO_RADIX_KEY_WIDTH: u32 = 64;
const SCRATCH_ALIGNMENT_BYTES: usize = 4096 * 1;

static mut kho_enable: bool = true;
static mut kho_scratch: *mut kho_scratch = ptr::null_mut();
static mut kho_scratch_cnt: u32 = 0;

#[repr(C)] struct kho_out { fdt: *mut c_void, lock: [usize;8], radix_tree: kho_radix_tree, dbg: kho_debugfs }
#[repr(C)] struct kho_in { fdt_phys: u64, scratch_phys: u64, previous_release: [u8; 65], kexec_count: u32, dbg: kho_debugfs, radix_tree: kho_radix_tree }
static mut kho_out: MaybeUninit<kho_out> = MaybeUninit::uninit();
static mut kho_in: MaybeUninit<kho_in> = MaybeUninit::uninit();

#[no_mangle] pub unsafe extern "C" fn kho_is_enabled() -> bool { kho_enable }
unsafe fn kho_encode_radix_key(phys: u64, order: u32) -> usize { let shift=PAGE_SHIFT+order; (1usize << (64-shift)) | ((phys >> shift) as usize) }
unsafe fn kho_decode_radix_key(key: usize, order: *mut u32) -> u64 { let bit=fls64(key as u64)-1; *order=64-(PAGE_SHIFT+bit); (key as u64) << (PAGE_SHIFT+*order) }
unsafe fn kho_radix_get_bitmap_index(key: usize)->usize { key % (1usize<<KHO_BITMAP_SIZE_LOG2) }
unsafe fn kho_radix_get_table_index(key: usize, level:u32)->usize { (key >> (((level-1)*KHO_TABLE_SIZE_LOG2)+KHO_BITMAP_SIZE_LOG2)) % (1usize<<KHO_TABLE_SIZE_LOG2) }

#[no_mangle] pub unsafe extern "C" fn kho_radix_add_key(tree:*mut kho_radix_tree,key:usize)->c_int {
    if (*tree).root.is_null() { return -22; } if fls64(key as u64)>KHO_RADIX_KEY_WIDTH{return -34;}
    let mut node=(*tree).root; let mut made:[*mut kho_radix_node;5]=[ptr::null_mut();5]; let mut anchor=ptr::null_mut(); let mut anchor_idx=0;
    let mut i=KHO_TREE_MAX_DEPTH-1; while i>0 { let idx=kho_radix_get_table_index(key,i); if (*node).table[idx]!=0 {node=phys_to_virt((*node).table[idx]) as *mut _;} else {let n=get_zeroed_page(0) as *mut kho_radix_node;if n.is_null(){for p in made {if !p.is_null(){free_page(p as usize)}};if !anchor.is_null(){(*anchor).table[anchor_idx]=0;}return -12;}(*node).table[idx]=virt_to_phys(n as *const _ as *const c_void);if anchor.is_null(){anchor=node;anchor_idx=idx;}made[i as usize]=n;node=n;} i-=1; }
    set_bit(kho_radix_get_bitmap_index(key), node as *mut usize); 0
}

#[no_mangle] pub unsafe extern "C" fn kho_radix_del_key(tree:*mut kho_radix_tree,key:usize){if (*tree).root.is_null(){return}let mut n=(*tree).root;let mut i=KHO_TREE_MAX_DEPTH-1;while i>0{let x=kho_radix_get_table_index(key,i);if (*n).table[x]==0{return}n=phys_to_virt((*n).table[x]) as *mut _;i-=1;}clear_bit(kho_radix_get_bitmap_index(key),n as *mut usize);}
#[no_mangle] pub unsafe extern "C" fn kho_radix_init_tree(t:*mut kho_radix_tree,r:*mut kho_radix_node)->c_int{(*t).root=if r.is_null(){get_zeroed_page(0) as *mut _}else{r};if (*t).root.is_null(){-12}else{mutex_init(&mut (*t).lock as *mut _ as *mut c_void);0}}
#[no_mangle] pub unsafe extern "C" fn kho_radix_destroy_tree(t:*mut kho_radix_tree){(*t).root=ptr::null_mut();}

// The remaining entry points retain the kernel ABI and delegate to the same
// externally supplied page allocator, FDT, memblock, vmalloc, and KHO helpers.
#[no_mangle] pub unsafe extern "C" fn is_kho_boot()->bool{!(*kho_in.as_ptr()).fdt_phys.eq(&0)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
