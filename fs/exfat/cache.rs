// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * linux/fs/fat/cache.c -- source-level Rust translation.
 * Includes and symbols supplied by the surrounding kernel/exFAT sources are
 * intentionally left as external dependencies.
 */

const EXFAT_MAX_CACHE: usize = 16;

#[repr(C)]
pub struct exfat_cache {
    pub cache_list: list_head,
    pub nr_contig: c_uint,
    pub fcluster: c_uint,
    pub dcluster: c_uint,
}

#[repr(C)]
pub struct exfat_cache_id {
    pub id: c_uint,
    pub nr_contig: c_uint,
    pub fcluster: c_uint,
    pub dcluster: c_uint,
}

extern "C" {
    static mut exfat_cachep: *mut kmem_cache;
    fn kmem_cache_create(name: *const c_char, size: usize, align: usize,
        flags: c_uint, ctor: Option<unsafe extern "C" fn(*mut c_void)>)->*mut kmem_cache;
    fn kmem_cache_destroy(cache: *mut kmem_cache);
    fn kmem_cache_alloc(cache: *mut kmem_cache, flags: c_uint) -> *mut c_void;
    fn kmem_cache_free(cache: *mut kmem_cache, obj: *mut c_void);
    fn INIT_LIST_HEAD(head: *mut list_head);
    fn list_empty(head: *const list_head) -> bool;
    fn list_move(entry: *mut list_head, head: *mut list_head);
    fn list_del_init(entry: *mut list_head);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn exfat_ent_get(sb: *mut super_block, cluster: c_uint,
        content: *mut c_uint, bh: *mut *mut buffer_head) -> c_int;
    fn exfat_fs_error(sb: *mut super_block, fmt: *const c_char, ...);
    fn brelse(bh: *mut buffer_head);
}

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct kmem_cache { _private: [u8; 0] }
#[repr(C)] pub struct buffer_head { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct inode { pub i_sb: *mut super_block }
#[repr(C)] pub struct super_block { _private: [u8; 0] }
#[repr(C)] pub struct exfat_inode_info {
    pub cache_lru: list_head, pub cache_lru_lock: spinlock_t,
    pub cache_valid_id: c_uint, pub nr_caches: c_uint, pub start_clu: c_uint,
}

type c_int = i32; type c_uint = u32; type c_char = i8; type c_void = core::ffi::c_void;
const ENOMEM: c_int = 12; const EIO: c_int = 5;
const GFP_NOFS: c_uint = 0; const SLAB_RECLAIM_ACCOUNT: c_uint = 0;
const EXFAT_FREE_CLUSTER: c_uint = 0; const EXFAT_EOF_CLUSTER: c_uint = 0xffff_ffff;
const EXFAT_CACHE_VALID: c_uint = 0xffff_ffff;

#[inline] unsafe fn exfat_i(inode: *mut inode) -> *mut exfat_inode_info { EXFAT_I(inode) }
extern "C" { fn EXFAT_I(inode: *mut inode) -> *mut exfat_inode_info; }

unsafe extern "C" fn exfat_cache_init_once(c: *mut c_void) {
    INIT_LIST_HEAD(&mut (*(c as *mut exfat_cache)).cache_list);
}

#[no_mangle] pub unsafe extern "C" fn exfat_cache_init() -> c_int {
    exfat_cachep = kmem_cache_create(b"exfat_cache\0".as_ptr() as *const c_char,
        core::mem::size_of::<exfat_cache>(), 0, SLAB_RECLAIM_ACCOUNT,
        Some(exfat_cache_init_once));
    if exfat_cachep.is_null() { -ENOMEM } else { 0 }
}

#[no_mangle] pub unsafe extern "C" fn exfat_cache_shutdown() {
    if !exfat_cachep.is_null() { kmem_cache_destroy(exfat_cachep); }
}

unsafe fn exfat_cache_alloc() -> *mut exfat_cache { kmem_cache_alloc(exfat_cachep, GFP_NOFS) as *mut exfat_cache }
unsafe fn exfat_cache_free(cache: *mut exfat_cache) {
    debug_assert!(!list_empty(&(*cache).cache_list)); kmem_cache_free(exfat_cachep, cache as *mut c_void);
}

unsafe fn exfat_cache_update_lru(inode: *mut inode, cache: *mut exfat_cache) {
    let ei = exfat_i(inode); if (*ei).cache_lru.next != &mut (*cache).cache_list { list_move(&mut (*cache).cache_list, &mut (*ei).cache_lru); }
}

unsafe fn exfat_cache_lookup(inode: *mut inode, cid: *mut exfat_cache_id, fclus: c_uint,
    mut end: c_uint, cached_fclus: *mut c_uint, cached_dclus: *mut c_uint) -> c_uint {
    let ei = exfat_i(inode); let mut hit: *mut exfat_cache = core::ptr::null_mut(); let mut tail = 0;
    spin_lock(&mut (*ei).cache_lru_lock);
    let mut p = (*ei).cache_lru.next;
    while p != &mut (*ei).cache_lru as *mut list_head {
        let x = (p as *mut u8).sub(core::mem::offset_of!(exfat_cache, cache_list)) as *mut exfat_cache;
        if (*x).fcluster <= fclus { if !hit.is_null() && (*x).fcluster < (*hit).fcluster { p=(*p).next; continue; } hit=x; tail=(*x).fcluster+(*x).nr_contig; if tail>=end { break; } }
        else if (*x).fcluster <= end { end=(*x).fcluster-1; if tail != 0 && tail>=end { break; } }
        p=(*p).next;
    }
    if !hit.is_null() { exfat_cache_update_lru(inode, hit); (*cid).id=(*ei).cache_valid_id; (*cid).nr_contig=(*hit).nr_contig; (*cid).fcluster=(*hit).fcluster; (*cid).dcluster=(*hit).dcluster; let offset=core::cmp::min((*cid).nr_contig, fclus-(*cid).fcluster); *cached_fclus=(*cid).fcluster+offset; *cached_dclus=(*cid).dcluster+offset; }
    spin_unlock(&mut (*ei).cache_lru_lock); end
}

unsafe fn cache_contiguous(cid: *mut exfat_cache_id, dclus: c_uint) -> bool { (*cid).nr_contig+=1; (*cid).dcluster+(*cid).nr_contig==dclus }
unsafe fn cache_init(cid: *mut exfat_cache_id, fclus: c_uint, dclus: c_uint) { (*cid).id=EXFAT_CACHE_VALID; (*cid).fcluster=fclus; (*cid).dcluster=dclus; (*cid).nr_contig=0; }

unsafe fn exfat_cache_merge(inode: *mut inode, new: *mut exfat_cache_id) -> *mut exfat_cache {
    let ei=exfat_i(inode); let mut p=(*ei).cache_lru.next;
    while p != &mut (*ei).cache_lru as *mut list_head { let x=(p as *mut u8).sub(core::mem::offset_of!(exfat_cache,cache_list)) as *mut exfat_cache; if (*x).fcluster==(*new).fcluster { if (*new).nr_contig>(*x).nr_contig {(*x).nr_contig=(*new).nr_contig;} return x; } p=(*p).next; } core::ptr::null_mut()
}
unsafe fn exfat_cache_add(inode:*mut inode,new:*mut exfat_cache_id) {
    if (*new).fcluster==EXFAT_EOF_CLUSTER{return} let ei=exfat_i(inode); spin_lock(&mut (*ei).cache_lru_lock);
    if (*new).id!=EXFAT_CACHE_VALID && (*new).id!=(*ei).cache_valid_id {spin_unlock(&mut (*ei).cache_lru_lock);return}
    let mut cache=exfat_cache_merge(inode,new);
    if cache.is_null() { if (*ei).nr_caches<EXFAT_MAX_CACHE as u32 {(*ei).nr_caches+=1;spin_unlock(&mut (*ei).cache_lru_lock);let tmp=exfat_cache_alloc();if tmp.is_null(){spin_lock(&mut (*ei).cache_lru_lock);(*ei).nr_caches-=1;spin_unlock(&mut (*ei).cache_lru_lock);return}spin_lock(&mut (*ei).cache_lru_lock);cache=exfat_cache_merge(inode,new);if !cache.is_null(){(*ei).nr_caches-=1;exfat_cache_free(tmp);}else{cache=tmp;}} else {let p=(*ei).cache_lru.prev;cache=(p as *mut u8).sub(core::mem::offset_of!(exfat_cache,cache_list)) as *mut exfat_cache} if !cache.is_null(){(*cache).fcluster=(*new).fcluster;(*cache).dcluster=(*new).dcluster;(*cache).nr_contig=(*new).nr_contig;}}
    if !cache.is_null(){exfat_cache_update_lru(inode,cache)} spin_unlock(&mut (*ei).cache_lru_lock);
}
unsafe fn __exfat_cache_inval_inode(inode:*mut inode){let ei=exfat_i(inode);while !list_empty(&(*ei).cache_lru){let p=(*ei).cache_lru.next;let c=(p as *mut u8).sub(core::mem::offset_of!(exfat_cache,cache_list)) as *mut exfat_cache;list_del_init(&mut (*c).cache_list);(*ei).nr_caches-=1;exfat_cache_free(c)}(*ei).cache_valid_id+=1;if (*ei).cache_valid_id==EXFAT_CACHE_VALID{(*ei).cache_valid_id+=1}}
#[no_mangle] pub unsafe extern "C" fn exfat_cache_inval_inode(inode:*mut inode){let ei=exfat_i(inode);spin_lock(&mut (*ei).cache_lru_lock);__exfat_cache_inval_inode(inode);spin_unlock(&mut (*ei).cache_lru_lock)}

#[no_mangle] pub unsafe extern "C" fn exfat_get_cluster(inode:*mut inode,cluster:c_uint,dclus:*mut c_uint,count:*mut c_uint,last_dclus:*mut c_uint)->c_int{
 let sb=(*inode).i_sb;let ei=exfat_i(inode);let mut bh: *mut buffer_head=core::ptr::null_mut();let mut cid=exfat_cache_id{id:0,nr_contig:0,fcluster:0,dcluster:0};let mut content=0;let mut fclus=0;let end=cluster+*count-1;
 if (*ei).start_clu==EXFAT_FREE_CLUSTER{return -EIO}*dclus=(*ei).start_clu;*last_dclus=*dclus;if *dclus==EXFAT_EOF_CLUSTER{*count=0;return 0}if cluster==0&&*count==1{return 0}cache_init(&mut cid,0,*dclus);let end=exfat_cache_lookup(inode,&mut cid,cluster,end,&mut fclus,dclus);if cid.fcluster+cid.nr_contig>=end{*count=end-cluster+1;return 0}
 while fclus<cluster{if exfat_ent_get(sb,*dclus,&mut content,&mut bh)!=0{return -EIO}*last_dclus=*dclus;*dclus=content;fclus+=1;if content==EXFAT_EOF_CLUSTER{break}if !cache_contiguous(&mut cid,*dclus){cache_init(&mut cid,fclus,*dclus)}}
 if *dclus!=EXFAT_EOF_CLUSTER{let mut clu=*dclus;while fclus<end{if exfat_ent_get(sb,clu,&mut content,&mut bh)!=0{return -EIO}clu+=1;if clu!=content{break}fclus+=1}cid.nr_contig=fclus-cid.fcluster;*count=fclus-cluster+1;if fclus<end&&content!=EXFAT_EOF_CLUSTER{exfat_cache_add(inode,&mut cid);cache_init(&mut cid,fclus+1,content)}}else{*count=0}brelse(bh);exfat_cache_add(inode,&mut cid);0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
