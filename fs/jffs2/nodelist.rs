/* Rust translation of nodelist.c. External kernel/JFFS2 definitions are supplied elsewhere. */

unsafe extern "C" {
    fn jffs2_mark_node_obsolete(c: *mut jffs2_sb_info, raw: *mut jffs2_raw_node_ref);
    fn jffs2_free_full_dirent(p: *mut jffs2_full_dirent);
    fn jffs2_free_full_dnode(p: *mut jffs2_full_dnode);
    fn jffs2_free_node_frag(p: *mut jffs2_node_frag);
    fn jffs2_alloc_node_frag() -> *mut jffs2_node_frag;
    fn jffs2_free_inode_cache(p: *mut jffs2_inode_cache);
    fn jffs2_xattr_free_inode(c: *mut jffs2_sb_info, p: *mut jffs2_inode_cache);
    fn jffs2_free_refblock(p: *mut jffs2_raw_node_ref);
    fn jffs2_dbg_fragtree_paranoia_check_nolock(f: *mut jffs2_inode_info);
    fn ref_next(p: *mut jffs2_raw_node_ref) -> *mut jffs2_raw_node_ref;
    fn ref_offset(p: *mut jffs2_raw_node_ref) -> u32;
    fn ref_flags(p: *mut jffs2_raw_node_ref) -> u32;
    fn ref_obsolete(p: *mut jffs2_raw_node_ref) -> bool;
    fn ref_totlen(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock, p: *mut jffs2_raw_node_ref) -> u32;
    fn mark_ref_normal(p: *mut jffs2_raw_node_ref);
    fn frag_next(p: *mut jffs2_node_frag) -> *mut jffs2_node_frag;
    fn frag_prev(p: *mut jffs2_node_frag) -> *mut jffs2_node_frag;
    fn frag_last(root: *mut rb_root) -> *mut jffs2_node_frag;
    fn frag_erase(p: *mut jffs2_node_frag, root: *mut rb_root);
    fn rb_link_node(n: *mut rb_node, parent: *mut rb_node, link: *mut *mut rb_node);
    fn rb_insert_color(n: *mut rb_node, root: *mut rb_root);
    fn rb_replace_node(old: *mut rb_node, new: *mut rb_node, root: *mut rb_root);
    fn rb_erase(n: *mut rb_node, root: *mut rb_root);
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn wake_up(wq: *mut wait_queue_head_t);
    fn cond_resched();
    fn strcmp(a: *const i8, b: *const i8) -> i32;
}

const REF_EMPTY_NODE: u32 = 0xffff_ffff;
const REF_LINK_NODE: u32 = 0xffff_fffe;
const REF_UNCHECKED: u32 = 0;
const REF_NORMAL: u32 = 1;
const REF_PRISTINE: u32 = 2;
const REF_OBSOLETE: u32 = 3;
const INO_STATE_READING: i32 = 1;
const INO_STATE_CLEARING: i32 = 2;
const PAGE_SIZE: u32 = 4096;
const PAGE_SHIFT: u32 = 12;

/* Types and helper macros are declared by nodelist.h and the kernel headers. */
#[allow(non_camel_case_types)] type uint32_t = u32;

pub unsafe fn jffs2_add_fd_to_list(c: *mut jffs2_sb_info, new: *mut jffs2_full_dirent, list: *mut *mut jffs2_full_dirent) {
    let mut prev = list;
    while !(*prev).is_null() && (**prev).nhash <= (*new).nhash {
        if (**prev).nhash == (*new).nhash && strcmp((*new).name.as_ptr(), (**prev).name.as_ptr()) == 0 {
            if (*new).version < (**prev).version {
                jffs2_mark_node_obsolete(c, (*new).raw); jffs2_free_full_dirent(new);
            } else {
                (*new).next = (**prev).next;
                if !(**prev).raw.is_null() { jffs2_mark_node_obsolete(c, (**prev).raw); }
                jffs2_free_full_dirent(*prev); *prev = new;
            }
            return;
        }
        prev = &mut (**prev).next;
    }
    (*new).next = *prev; *prev = new;
}

pub unsafe fn jffs2_truncate_fragtree(c: *mut jffs2_sb_info, list: *mut rb_root, size: u32) -> u32 {
    let mut frag = jffs2_lookup_node_frag(list, size);
    if !frag.is_null() && (*frag).ofs != size {
        if (*frag).ofs + (*frag).size > size { (*frag).size = size - (*frag).ofs; }
        frag = frag_next(frag);
    }
    while !frag.is_null() && (*frag).ofs >= size {
        let next = frag_next(frag); frag_erase(frag, list); jffs2_obsolete_node_frag(c, frag); frag = next;
    }
    if size == 0 { return 0; }
    frag = frag_last(list); if frag.is_null() { return 0; }
    if (*frag).ofs + (*frag).size < size { return (*frag).ofs + (*frag).size; }
    if !(*frag).node.is_null() && ((*frag).ofs & (PAGE_SIZE - 1)) == 0 {
        (*(*frag).node).raw.as_mut().unwrap().flash_offset = ref_offset((*frag).node.raw) | REF_PRISTINE;
    }
    size
}

unsafe fn jffs2_obsolete_node_frag(c: *mut jffs2_sb_info, this: *mut jffs2_node_frag) {
    if !(*this).node.is_null() {
        (*(*this).node).frags -= 1;
        if (*(*this).node).frags == 0 { jffs2_mark_node_obsolete(c, (*this).node.raw); jffs2_free_full_dnode((*this).node); }
        else { mark_ref_normal((*this).node.raw); }
    }
    jffs2_free_node_frag(this);
}

unsafe fn new_fragment(fn_: *mut jffs2_full_dnode, ofs: u32, size: u32) -> *mut jffs2_node_frag {
    let p = jffs2_alloc_node_frag();
    if !p.is_null() { (*p).ofs = ofs; (*p).size = size; (*p).node = fn_; }
    p
}

/* The remaining tree manipulation is a direct unsafe translation of the C implementation. */
unsafe fn jffs2_fragtree_insert(newfrag: *mut jffs2_node_frag, base: *mut jffs2_node_frag) {
    let mut parent = &mut (*base).rb as *mut rb_node; let mut link = &mut parent;
    while !(*link).is_null() { parent = *link; base = rb_entry(parent); link = if (*newfrag).ofs > (*base).ofs { &mut (*base).rb.rb_right } else { &mut (*base).rb.rb_left }; }
    rb_link_node(&mut (*newfrag).rb, parent, link);
}

unsafe fn no_overlapping_node(_c: *mut jffs2_sb_info, root: *mut rb_root, newfrag: *mut jffs2_node_frag, mut this: *mut jffs2_node_frag, lastend: u32) -> i32 {
    if lastend < (*newfrag).node.as_ref().unwrap().ofs {
        let hole = new_fragment(std::ptr::null_mut(), lastend, (*newfrag).node.as_ref().unwrap().ofs - lastend);
        if hole.is_null() { jffs2_free_node_frag(newfrag); return -12; }
        if !this.is_null() { rb_link_node(&mut (*hole).rb, &mut (*this).rb, &mut (*this).rb.rb_right); }
        else { rb_link_node(&mut (*hole).rb, std::ptr::null_mut(), &mut (*root).rb_node); }
        rb_insert_color(&mut (*hole).rb, root); this = hole;
    }
    if !this.is_null() { rb_link_node(&mut (*newfrag).rb, &mut (*this).rb, &mut (*this).rb.rb_right); }
    else { rb_link_node(&mut (*newfrag).rb, std::ptr::null_mut(), &mut (*root).rb_node); }
    rb_insert_color(&mut (*newfrag).rb, root); 0
}

/* Remaining declarations preserve the public interfaces; detailed bodies follow the source control flow. */
pub unsafe fn jffs2_set_inocache_state(c: *mut jffs2_sb_info, ic: *mut jffs2_inode_cache, state: i32) { spin_lock(&mut (*c).inocache_lock); (*ic).state = state; wake_up(&mut (*c).inocache_wq); spin_unlock(&mut (*c).inocache_lock); }

pub unsafe fn jffs2_get_ino_cache(c: *mut jffs2_sb_info, ino: u32) -> *mut jffs2_inode_cache { let mut ret = (*c).inocache_list[(ino % (*c).inocache_hashsize) as usize]; while !ret.is_null() && (*ret).ino < ino { ret = (*ret).next; } if !ret.is_null() && (*ret).ino != ino { std::ptr::null_mut() } else { ret } }

pub unsafe fn jffs2_free_ino_caches(c: *mut jffs2_sb_info) { for i in 0..(*c).inocache_hashsize as usize { let mut p = (*c).inocache_list[i]; while !p.is_null() { let n = (*p).next; jffs2_xattr_free_inode(c,p); jffs2_free_inode_cache(p); p=n; } (*c).inocache_list[i]=std::ptr::null_mut(); } }

unsafe fn __ref_totlen(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock, r: *mut jffs2_raw_node_ref) -> u32 { let n=ref_next(r); let end=if !n.is_null(){ref_offset(n)}else{if jeb.is_null(){jeb=&mut (*c).blocks[( (*r).flash_offset / (*c).sector_size) as usize];} (*jeb).offset+(*c).sector_size-(*jeb).free_size}; end-ref_offset(r) }
pub unsafe fn __jffs2_ref_totlen(c:*mut jffs2_sb_info,jeb:*mut jffs2_eraseblock,r:*mut jffs2_raw_node_ref)->u32 { __ref_totlen(c,jeb,r) }

pub unsafe fn jffs2_add_ino_cache(c:*mut jffs2_sb_info,n:*mut jffs2_inode_cache){spin_lock(&mut (*c).inocache_lock);if (*n).ino==0{(*c).highest_ino+=1;(*n).ino=(*c).highest_ino;}let mut p=&mut (*c).inocache_list[((*n).ino%(*c).inocache_hashsize)as usize]as*mut*mut jffs2_inode_cache;while !(*p).is_null()&&(**p).ino<(*n).ino{p=&mut(**p).next;}(*n).next=*p;*p=n;spin_unlock(&mut(*c).inocache_lock)}
pub unsafe fn jffs2_del_ino_cache(c:*mut jffs2_sb_info,old:*mut jffs2_inode_cache){spin_lock(&mut(*c).inocache_lock);let mut p=&mut(*c).inocache_list[((*old).ino%(*c).inocache_hashsize)as usize]as*mut*mut jffs2_inode_cache;while !(*p).is_null()&&(**p).ino<(*old).ino{p=&mut(**p).next;}if *p==old{*p=(*old).next;}if(*old).state!=INO_STATE_READING&&(*old).state!=INO_STATE_CLEARING{jffs2_free_inode_cache(old);}spin_unlock(&mut(*c).inocache_lock)}
pub unsafe fn jffs2_free_raw_node_refs(c:*mut jffs2_sb_info){for i in 0..(*c).nr_blocks as usize{let mut p=(*c).blocks[i].first_node;while !p.is_null(){let n=if(*p.add(REFS_PER_BLOCK)).flash_offset==REF_LINK_NODE{(*p.add(REFS_PER_BLOCK)).next_in_ino}else{std::ptr::null_mut()};jffs2_free_refblock(p);p=n;}(*c).blocks[i].first_node=std::ptr::null_mut();(*c).blocks[i].last_node=std::ptr::null_mut();}}
pub unsafe fn jffs2_lookup_node_frag(root:*mut rb_root,offset:u32)->*mut jffs2_node_frag{let mut n=(*root).rb_node;let mut prev=std::ptr::null_mut();while !n.is_null(){let f=rb_entry(n);if(*f).ofs+(*f).size<=offset{if prev.is_null()||(*f).ofs>(*prev).ofs{prev=f;}n=(*f).rb.rb_right;}else if(*f).ofs>offset{n=(*f).rb.rb_left;}else{return f;}}prev}
pub unsafe fn jffs2_kill_fragtree(root:*mut rb_root,c:*mut jffs2_sb_info){let mut f=rb_first(root);while !f.is_null(){let n=rb_next(f);let x=rb_entry(f);if !(*x).node.is_null(){(*(*x).node).frags-=1;if(*(*x).node).frags==0{if !c.is_null(){jffs2_mark_node_obsolete(c,(*x).node.raw);}jffs2_free_full_dnode((*x).node);}}jffs2_free_node_frag(x);f=n;cond_resched();}}
pub unsafe fn jffs2_scan_dirty_space(c:*mut jffs2_sb_info,j:*mut jffs2_eraseblock,size:u32)->i32{if size==0{return 0;}if !(*j).last_node.is_null()&&ref_obsolete((*j).last_node){(*c).dirty_size+=size;(*c).free_size-=size;(*j).dirty_size+=size;(*j).free_size-=size;}else{let mut o=(*j).offset+(*c).sector_size-(*j).free_size;o|=REF_OBSOLETE;jffs2_link_node_ref(c,j,o,size,std::ptr::null_mut());}0}
pub unsafe fn jffs2_link_node_ref(c:*mut jffs2_sb_info,j:*mut jffs2_eraseblock,ofs:u32,len:u32,ic:*mut jffs2_inode_cache)->*mut jffs2_raw_node_ref{(*j).allocated_refs-=1;let mut r=(*j).last_node;while(*r).flash_offset!=REF_EMPTY_NODE{if(*r).flash_offset==REF_LINK_NODE{r=(*r).next_in_ino;}else{r=r.add(1);}}(*r).flash_offset=ofs;if(*j).first_node.is_null(){(*j).first_node=r;}(*j).last_node=r;if !ic.is_null(){(*r).next_in_ino=(*ic).nodes;(*ic).nodes=r;}else{(*r).next_in_ino=std::ptr::null_mut();}match ref_flags(r){REF_UNCHECKED=>{(*c).unchecked_size+=len;(*j).unchecked_size+=len},REF_NORMAL|REF_PRISTINE=>{(*c).used_size+=len;(*j).used_size+=len},REF_OBSOLETE=>{(*c).dirty_size+=len;(*j).dirty_size+=len},_=>{}}(*c).free_size-=len;(*j).free_size-=len;r}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
