// SPDX-License-Identifier: GPL-2.0
/* Rust translation of linux/fs/fat/cache.c. */

const FAT_MAX_CACHE: i32 = 8;

#[repr(C)]
pub struct FatCache {
    pub cache_list: ListHead,
    pub nr_contig: i32,
    pub fcluster: i32,
    pub dcluster: i32,
}

#[repr(C)]
pub struct FatCacheId {
    pub id: u32,
    pub nr_contig: i32,
    pub fcluster: i32,
    pub dcluster: i32,
}

extern "C" {
    static mut fat_cache_cachep: *mut KmemCache;

    fn INIT_LIST_HEAD(head: *mut ListHead);
    fn kmem_cache_create(name: *const i8, size: usize, align: usize, flags: u32,
                         ctor: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>) -> *mut KmemCache;
    fn kmem_cache_destroy(cache: *mut KmemCache);
    fn kmem_cache_alloc(cache: *mut KmemCache, flags: u32) -> *mut core::ffi::c_void;
    fn kmem_cache_free(cache: *mut KmemCache, object: *mut core::ffi::c_void);
    fn list_empty(head: *const ListHead) -> bool;
    fn list_move(entry: *mut ListHead, head: *mut ListHead);
    fn list_del_init(entry: *mut ListHead);
    fn spin_lock(lock: *mut SpinLock);
    fn spin_unlock(lock: *mut SpinLock);
    fn BUG_ON(condition: bool);
    fn fat_valid_entry(sbi: *mut MsDosSbInfo, cluster: i32) -> bool;
    fn fat_fs_error_ratelimit(sb: *mut SuperBlock, fmt: *const i8, ...);
    fn fat_fs_error(sb: *mut SuperBlock, fmt: *const i8, ...);
    fn fatent_init(ent: *mut FatEntry);
    fn fat_ent_read(inode: *mut Inode, ent: *mut FatEntry, cluster: i32) -> i32;
    fn fatent_brelse(ent: *mut FatEntry);
    fn MSDOS_I(inode: *mut Inode) -> *mut MsDosInodeInfo;
    fn MSDOS_SB(sb: *mut SuperBlock) -> *mut MsDosSbInfo;
    fn is_fat32(sbi: *mut MsDosSbInfo) -> bool;
    fn i_size_read(inode: *mut Inode) -> u64;
    fn fat_clus_to_blknr(sbi: *mut MsDosSbInfo, cluster: i32) -> u64;
}

#[repr(C)] pub struct ListHead { pub next: *mut ListHead, pub prev: *mut ListHead }
#[repr(C)] pub struct KmemCache { _private: [u8; 0] }
#[repr(C)] pub struct SpinLock { _private: [u8; 0] }
#[repr(C)] pub struct FatEntry { _private: [u8; 0] }
#[repr(C)] pub struct SuperBlock { pub s_maxbytes: u64, pub s_blocksize: u32, pub s_blocksize_bits: u8 }
#[repr(C)] pub struct Inode { pub i_sb: *mut SuperBlock, pub i_ino: u64, pub i_blocks: u64 }
#[repr(C)] pub struct MsDosSbInfo { pub cluster_bits: u8, pub sec_per_clus: u32, pub dir_entries: u32, pub dir_per_block_bits: u8, pub dir_start: u64 }
#[repr(C)] pub struct MsDosInodeInfo {
    pub cache_lru: ListHead, pub cache_lru_lock: SpinLock, pub nr_caches: i32,
    pub cache_valid_id: u32, pub i_start: i32, pub i_pos: i64, pub mmu_private: u64,
}
pub type SectorT = u64;
const SLAB_RECLAIM_ACCOUNT: u32 = 0;
const GFP_NOFS: u32 = 0;
const FAT_CACHE_VALID: u32 = 0xffff_ffff;
const FAT_ENT_FREE: i32 = 0;
const FAT_ENT_EOF: i32 = -1;
const MSDOS_ROOT_INO: u64 = 1;

unsafe extern "C" fn init_once(foo: *mut core::ffi::c_void) {
    INIT_LIST_HEAD(&mut (*(foo as *mut FatCache)).cache_list);
}

pub unsafe extern "C" fn fat_cache_init() -> i32 {
    fat_cache_cachep = kmem_cache_create(b"fat_cache\0".as_ptr() as *const i8,
        core::mem::size_of::<FatCache>(), 0, SLAB_RECLAIM_ACCOUNT, Some(init_once));
    if fat_cache_cachep.is_null() { return -12; }
    0
}

pub unsafe extern "C" fn fat_cache_destroy() { kmem_cache_destroy(fat_cache_cachep); }

unsafe fn fat_cache_alloc() -> *mut FatCache {
    kmem_cache_alloc(fat_cache_cachep, GFP_NOFS) as *mut FatCache
}
unsafe fn fat_cache_free(cache: *mut FatCache) {
    BUG_ON(!list_empty(&(*cache).cache_list));
    kmem_cache_free(fat_cache_cachep, cache as *mut core::ffi::c_void);
}
unsafe fn fat_cache_update_lru(inode: *mut Inode, cache: *mut FatCache) {
    let i = MSDOS_I(inode);
    if (*i).cache_lru.next != &mut (*cache).cache_list { list_move(&mut (*cache).cache_list, &mut (*i).cache_lru); }
}

unsafe fn fat_cache_lookup(inode: *mut Inode, fclus: i32, cid: *mut FatCacheId,
                           cached_fclus: *mut i32, cached_dclus: *mut i32) -> i32 {
    let mut nohit = FatCache { cache_list: ListHead { next: core::ptr::null_mut(), prev: core::ptr::null_mut() }, nr_contig: 0, fcluster: 0, dcluster: 0 };
    let mut hit: *mut FatCache = &mut nohit;
    let mut offset = -1;
    let i = MSDOS_I(inode); spin_lock(&mut (*i).cache_lru_lock);
    let mut p = (*i).cache_lru.next;
    while p != &mut (*i).cache_lru {
        let c = (p as *mut u8).sub(core::mem::offset_of!(FatCache, cache_list)) as *mut FatCache;
        if (*c).fcluster <= fclus && (*hit).fcluster < (*c).fcluster {
            hit = c;
            if (*hit).fcluster + (*hit).nr_contig < fclus { offset = (*hit).nr_contig; }
            else { offset = fclus - (*hit).fcluster; break; }
        }
        p = (*p).next;
    }
    if hit != &mut nohit { fat_cache_update_lru(inode, hit); (*cid).id=(*i).cache_valid_id; (*cid).nr_contig=(*hit).nr_contig; (*cid).fcluster=(*hit).fcluster; (*cid).dcluster=(*hit).dcluster; *cached_fclus=(*cid).fcluster+offset; *cached_dclus=(*cid).dcluster+offset; }
    spin_unlock(&mut (*i).cache_lru_lock); offset
}

unsafe fn fat_cache_merge(inode: *mut Inode, new: *mut FatCacheId) -> *mut FatCache {
    let i=MSDOS_I(inode); let mut p=(*i).cache_lru.next;
    while p != &mut (*i).cache_lru { let c=(p as *mut u8).sub(core::mem::offset_of!(FatCache,cache_list)) as *mut FatCache; if (*c).fcluster==(*new).fcluster { BUG_ON((*c).dcluster!=(*new).dcluster); if (*new).nr_contig>(*c).nr_contig {(*c).nr_contig=(*new).nr_contig;} return c;} p=(*p).next; } core::ptr::null_mut()
}

unsafe fn fat_cache_add(inode: *mut Inode, new: *mut FatCacheId) {
    if (*new).fcluster == -1 { return; } let i=MSDOS_I(inode); spin_lock(&mut (*i).cache_lru_lock);
    if (*new).id != FAT_CACHE_VALID && (*new).id != (*i).cache_valid_id { spin_unlock(&mut (*i).cache_lru_lock); return; }
    let mut cache=fat_cache_merge(inode,new);
    if cache.is_null() { if (*i).nr_caches < FAT_MAX_CACHE { (*i).nr_caches+=1; spin_unlock(&mut (*i).cache_lru_lock); let tmp=fat_cache_alloc(); if tmp.is_null(){spin_lock(&mut (*i).cache_lru_lock);(*i).nr_caches-=1;spin_unlock(&mut (*i).cache_lru_lock);return;} spin_lock(&mut (*i).cache_lru_lock); cache=fat_cache_merge(inode,new); if !cache.is_null(){(*i).nr_caches-=1;fat_cache_free(tmp);} else {cache=tmp;} } else { cache=((*i).cache_lru.prev as *mut u8).sub(core::mem::offset_of!(FatCache,cache_list)) as *mut FatCache; } if !cache.is_null() {(*cache).fcluster=(*new).fcluster;(*cache).dcluster=(*new).dcluster;(*cache).nr_contig=(*new).nr_contig;} }
    if !cache.is_null(){fat_cache_update_lru(inode,cache);} spin_unlock(&mut (*i).cache_lru_lock);
}

unsafe fn __fat_cache_inval_inode(inode:*mut Inode){let i=MSDOS_I(inode);while !list_empty(&(*i).cache_lru){let c=((*i).cache_lru.next as *mut u8).sub(core::mem::offset_of!(FatCache,cache_list)) as *mut FatCache;list_del_init(&mut (*c).cache_list);(*i).nr_caches-=1;fat_cache_free(c);}(*i).cache_valid_id+=1;if (*i).cache_valid_id==FAT_CACHE_VALID{(*i).cache_valid_id+=1;}}
pub unsafe extern "C" fn fat_cache_inval_inode(inode:*mut Inode){let i=MSDOS_I(inode);spin_lock(&mut (*i).cache_lru_lock);__fat_cache_inval_inode(inode);spin_unlock(&mut (*i).cache_lru_lock);}

unsafe fn cache_contiguous(cid:*mut FatCacheId,dclus:i32)->bool{(*cid).nr_contig+=1;(*cid).dcluster+(*cid).nr_contig==dclus}
unsafe fn cache_init(cid:*mut FatCacheId,fclus:i32,dclus:i32){(*cid).id=FAT_CACHE_VALID;(*cid).fcluster=fclus;(*cid).dcluster=dclus;(*cid).nr_contig=0;}

pub unsafe extern "C" fn fat_get_cluster(inode:*mut Inode,cluster:i32,fclus:*mut i32,dclus:*mut i32)->i32{
 let sb=(*inode).i_sb;let sbi=MSDOS_SB(sb);let limit=((*sb).s_maxbytes>>(*sbi).cluster_bits) as i32;let mut fatent=core::mem::MaybeUninit::<FatEntry>::uninit();let mut cid=core::mem::MaybeUninit::<FatCacheId>::uninit();let mut nr;
 BUG_ON((*MSDOS_I(inode)).i_start==0);*fclus=0;*dclus=(*MSDOS_I(inode)).i_start;if !fat_valid_entry(sbi,*dclus){return -5;}if cluster==0{return 0;}
 if fat_cache_lookup(inode,cluster,cid.as_mut_ptr(),fclus,dclus)<0{cache_init(cid.as_mut_ptr(),-1,-1);}fatent_init(fatent.as_mut_ptr());
 while *fclus<cluster {if *fclus>limit{nr=-5;break;}nr=fat_ent_read(inode,fatent.as_mut_ptr(),*dclus);if nr<0{break;}if nr==FAT_ENT_FREE{nr=-5;break;}if nr==FAT_ENT_EOF{fat_cache_add(inode,cid.as_mut_ptr());break;}*fclus+=1;*dclus=nr;if !cache_contiguous(cid.as_mut_ptr(),*dclus){cache_init(cid.as_mut_ptr(),*fclus,*dclus);}}
 if *fclus>=cluster {fat_cache_add(inode,cid.as_mut_ptr());nr=0;}fatent_brelse(fatent.as_mut_ptr());nr
}

unsafe fn fat_bmap_cluster(inode:*mut Inode,cluster:i32)->i32{if (*MSDOS_I(inode)).i_start==0{return 0;}let mut f=0;let mut d=0;let r=fat_get_cluster(inode,cluster,&mut f,&mut d);if r<0{return r;}if r==FAT_ENT_EOF{return -5;}d}

pub unsafe extern "C" fn fat_get_mapped_cluster(inode:*mut Inode,sector:SectorT,last_block:SectorT,mapped_blocks:*mut usize,bmap:*mut SectorT)->i32{let sb=(*inode).i_sb;let sbi=MSDOS_SB(sb);let mut cluster=(sector>>((*sbi).cluster_bits-(*sb).s_blocksize_bits)) as i32;let offset=sector&((*sbi).sec_per_clus as u64-1);cluster=fat_bmap_cluster(inode,cluster);if cluster<0{return cluster;}if cluster!=0{*bmap=fat_clus_to_blknr(sbi,cluster)+offset;*mapped_blocks=(*sbi).sec_per_clus as usize-offset as usize;if *mapped_blocks> (last_block-sector) as usize{*mapped_blocks=(last_block-sector) as usize;}}0}

unsafe fn is_exceed_eof(inode:*mut Inode,sector:SectorT,last_block:*mut SectorT,create:bool)->bool{let sb=(*inode).i_sb;let bs=(*sb).s_blocksize as u64;*last_block=(i_size_read(inode)+bs-1)>>(*sb).s_blocksize_bits;if sector>=*last_block{if !create{return true;}*last_block=((*MSDOS_I(inode)).mmu_private+bs-1)>>(*sb).s_blocksize_bits;if sector>=*last_block{return true;}}false}

pub unsafe extern "C" fn fat_bmap(inode:*mut Inode,sector:SectorT,phys:*mut SectorT,mapped_blocks:*mut usize,create:bool,from_bmap:bool)->i32{let sbi=MSDOS_SB((*inode).i_sb);let mut last=0;*phys=0;*mapped_blocks=0;if !is_fat32(sbi)&&(*inode).i_ino==MSDOS_ROOT_INO{if sector<((*sbi).dir_entries>>(*sbi).dir_per_block_bits) as u64{*phys=sector+(*sbi).dir_start;*mapped_blocks=1;}return 0;}if !from_bmap{if is_exceed_eof(inode,sector,&mut last,create){return 0;}}else{last=(*inode).i_blocks>>((*inode).i_sb).s_blocksize_bits-9;if sector>=last{return 0;}}fat_get_mapped_cluster(inode,sector,last,mapped_blocks,phys)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
