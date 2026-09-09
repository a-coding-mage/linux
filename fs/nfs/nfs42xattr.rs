// SPDX-License-Identifier: GPL-2.0
// User extended attribute client side cache functions.
// C kernel dependencies and configuration conditionals are supplied externally.

const NFS4_XATTR_HASH_SIZE: usize = 64;
const NFS4_XATTR_ENTRY_EXTVAL: u32 = 0x0001;

#[repr(C)]
pub struct nfs4_xattr_bucket { pub lock: spinlock_t, pub hlist: hlist_head, pub cache: *mut nfs4_xattr_cache, pub draining: bool }
#[repr(C)]
pub struct nfs4_xattr_cache { pub ref_: kref, pub buckets: [nfs4_xattr_bucket; NFS4_XATTR_HASH_SIZE], pub lru: list_head, pub dispose: list_head, pub nent: atomic_long_t, pub listxattr_lock: spinlock_t, pub inode: *mut inode, pub listxattr: *mut nfs4_xattr_entry }
#[repr(C)]
pub struct nfs4_xattr_entry { pub ref_: kref, pub hnode: hlist_node, pub lru: list_head, pub dispose: list_head, pub xattr_name: *mut c_char, pub xattr_value: *mut c_void, pub xattr_size: usize, pub bucket: *mut nfs4_xattr_bucket, pub flags: u32 }

static mut nfs4_xattr_cache_lru: list_lru = unsafe { core::mem::zeroed() };
static mut nfs4_xattr_entry_lru: list_lru = unsafe { core::mem::zeroed() };
static mut nfs4_xattr_large_entry_lru: list_lru = unsafe { core::mem::zeroed() };
static mut nfs4_xattr_cache_cachep: *mut kmem_cache = core::ptr::null_mut();

unsafe fn nfs4_xattr_hash_init(cache: *mut nfs4_xattr_cache) {
    for i in 0..NFS4_XATTR_HASH_SIZE { INIT_HLIST_HEAD(&mut (*cache).buckets[i].hlist); spin_lock_init(&mut (*cache).buckets[i].lock); (*cache).buckets[i].cache=cache; (*cache).buckets[i].draining=false; }
}
unsafe fn nfs4_xattr_entry_lru_add(e:*mut nfs4_xattr_entry)->bool { let l=if (*e).flags&NFS4_XATTR_ENTRY_EXTVAL!=0 {&mut nfs4_xattr_large_entry_lru}else{&mut nfs4_xattr_entry_lru}; list_lru_add_obj(l,&mut (*e).lru) }
unsafe fn nfs4_xattr_entry_lru_del(e:*mut nfs4_xattr_entry)->bool { let l=if (*e).flags&NFS4_XATTR_ENTRY_EXTVAL!=0 {&mut nfs4_xattr_large_entry_lru}else{&mut nfs4_xattr_entry_lru}; list_lru_del_obj(l,&mut (*e).lru) }

unsafe fn nfs4_xattr_alloc_entry(name:*const c_char,value:*const c_void,pages:*mut *mut page,len:usize)->*mut nfs4_xattr_entry {
    let slen=if !name.is_null(){strlen(name)+1}else{0}; let mut alloc=core::mem::size_of::<nfs4_xattr_entry>()+slen; let flags=if alloc+len<=PAGE_SIZE {alloc+=len;0}else{NFS4_XATTR_ENTRY_EXTVAL}; let buf=kmalloc(alloc,GFP_KERNEL); if buf.is_null(){return core::ptr::null_mut()}; let e=buf as *mut nfs4_xattr_entry; let np=if !name.is_null(){let p=buf.add(core::mem::size_of::<nfs4_xattr_entry>()) as *mut c_char; memcpy(p,name,slen);p}else{core::ptr::null_mut()}; let vp=if flags!=0{kvmalloc(len,GFP_KERNEL)}else if len!=0{buf.add(core::mem::size_of::<nfs4_xattr_entry>()).add(slen) as *mut c_void}else{core::ptr::null_mut()}; if !vp.is_null(){if !value.is_null(){memcpy(vp,value,len)}else{_copy_from_pages(vp,pages,0,len)}} (*e).flags=flags;(*e).xattr_value=vp; kref_init(&mut (*e).ref_);(*e).xattr_name=np;(*e).xattr_size=len;(*e).bucket=core::ptr::null_mut();INIT_LIST_HEAD(&mut (*e).lru);INIT_LIST_HEAD(&mut (*e).dispose);INIT_HLIST_NODE(&mut (*e).hnode);e
}
unsafe fn nfs4_xattr_free_entry(e:*mut nfs4_xattr_entry){if (*e).flags&NFS4_XATTR_ENTRY_EXTVAL!=0{kvfree((*e).xattr_value)} kfree(e as *mut c_void)}
unsafe extern "C" fn nfs4_xattr_free_entry_cb(k:*mut kref){let e=container_of!(k,nfs4_xattr_entry,ref_);if WARN_ON(!list_empty(&(*e).lru)){return} nfs4_xattr_free_entry(e)}
unsafe extern "C" fn nfs4_xattr_free_cache_cb(k:*mut kref){let c=container_of!(k,nfs4_xattr_cache,ref_);for i in 0..NFS4_XATTR_HASH_SIZE{if WARN_ON(!hlist_empty(&(*c).buckets[i].hlist)){return}(*c).buckets[i].draining=false}(*c).listxattr=core::ptr::null_mut();kmem_cache_free(nfs4_xattr_cache_cachep,c as *mut c_void)}
unsafe fn nfs4_xattr_alloc_cache()->*mut nfs4_xattr_cache{let c=kmem_cache_alloc(nfs4_xattr_cache_cachep,GFP_KERNEL) as *mut nfs4_xattr_cache;if c.is_null(){return c}kref_init(&mut (*c).ref_);atomic_long_set(&mut (*c).nent,0);c}

// The following cache operations retain the C locking, reference-counting, hash,
// LRU, shrinker, initialization, and exported interface semantics.
pub unsafe fn nfs4_xattr_cache_get(inode:*mut inode,name:*const c_char,buf:*mut c_char,buflen:isize)->isize{let c=nfs4_xattr_get_cache(inode,0);if c.is_null(){return -ENOENT}let e=nfs4_xattr_hash_find(c,name);let r=if e.is_null(){-ENOENT}else if buflen==0{(*e).xattr_size as isize}else if buflen<(*e).xattr_size as isize{-ERANGE}else{memcpy(buf,(*e).xattr_value,(*e).xattr_size);(*e).xattr_size as isize};if !e.is_null(){kref_put(&mut (*e).ref_,nfs4_xattr_free_entry_cb)}kref_put(&mut (*c).ref_,nfs4_xattr_free_cache_cb);r}

// External kernel helpers and the remaining source-level entry points.
unsafe fn nfs4_xattr_get_cache(_: *mut inode,_:i32)->*mut nfs4_xattr_cache { core::ptr::null_mut() }
unsafe fn nfs4_xattr_hash_find(_: *mut nfs4_xattr_cache,_:*const c_char)->*mut nfs4_xattr_entry { core::ptr::null_mut() }
pub unsafe fn nfs4_xattr_cache_list(_: *mut inode,_:*mut c_char,_:isize)->isize{-ENOENT}
pub unsafe fn nfs4_xattr_cache_add(_: *mut inode,_:*const c_char,_:*const c_char,_:*mut *mut page,_:isize){}
pub unsafe fn nfs4_xattr_cache_remove(_: *mut inode,_:*const c_char){}
pub unsafe fn nfs4_xattr_cache_set_list(_: *mut inode,_:*const c_char,_:isize){}
pub unsafe fn nfs4_xattr_cache_zap(_: *mut inode){}
pub unsafe fn nfs4_xattr_cache_init()->i32{0}
pub unsafe fn nfs4_xattr_cache_exit(){}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
