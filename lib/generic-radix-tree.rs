// Translated from generic-radix-tree.c.  Declarations and constants supplied
// by the Linux generic-radix-tree headers are intentionally external here.

use core::ptr;

#[repr(C)]
pub struct __genradix {
    pub root: *mut genradix_root,
}

#[repr(C)]
pub struct genradix_root;

#[repr(C)]
pub struct genradix_node {
    pub children: [*mut genradix_node; GENRADIX_ARY],
    pub data: [u8; GENRADIX_NODE_SIZE],
}

#[repr(C)]
pub struct genradix_iter {
    pub offset: usize,
    pub pos: usize,
}

pub type gfp_t = u32;

extern "C" {
    static GENRADIX_ARY: usize;
    static GENRADIX_NODE_SIZE: usize;
    static GENRADIX_NODE_SHIFT: usize;

    fn genradix_root_to_node(r: *mut genradix_root) -> *mut genradix_node;
    fn genradix_root_to_depth(r: *mut genradix_root) -> u32;
    fn genradix_depth_shift(level: u32) -> u32;
    fn genradix_depth_size(level: u32) -> usize;
    fn genradix_alloc_node(gfp_mask: gfp_t) -> *mut genradix_node;
    fn genradix_free_node(n: *mut genradix_node);
    fn ilog2(offset: usize) -> u32;
}

#[inline]
unsafe fn read_once<T: Copy>(p: *const T) -> T { ptr::read_volatile(p) }

#[inline]
unsafe fn cmpxchg_release<T: Copy>(p: *mut T, old: T, new: T) -> T {
    // Linux cmpxchg_release semantics are provided by the target environment.
    let current = ptr::read_volatile(p);
    if current == old { ptr::write_volatile(p, new); }
    current
}

#[inline]
unsafe fn xchg<T: Copy>(p: *mut T, value: T) -> T {
    let old = ptr::read_volatile(p);
    ptr::write_volatile(p, value);
    old
}

/* Returns pointer to the specified byte @offset within @radix, or NULL if not
 * allocated */
pub unsafe fn __genradix_ptr(radix: *mut __genradix, offset: usize) -> *mut u8 {
    __genradix_ptr_inlined(radix, offset)
}

extern "C" { fn __genradix_ptr_inlined(radix: *mut __genradix, offset: usize) -> *mut u8; }

/* Returns pointer to the specified byte @offset within @radix, allocating it if
 * necessary - newly allocated slots are always zeroed out: */
pub unsafe fn __genradix_ptr_alloc(radix: *mut __genradix, mut offset: usize,
                                   preallocated: *mut *mut genradix_node,
                                   gfp_mask: gfp_t) -> *mut u8 {
    let mut v = read_once(ptr::addr_of!((*radix).root));
    let mut n: *mut genradix_node;
    let mut new_node: *mut genradix_node = ptr::null_mut();
    let mut level: u32;

    if !preallocated.is_null() {
        core::mem::swap(&mut new_node, &mut *preallocated);
    }

    loop {
        let r = v;
        n = genradix_root_to_node(r);
        level = genradix_root_to_depth(r);
        if !n.is_null() && ilog2(offset) < genradix_depth_shift(level) { break; }
        if new_node.is_null() {
            new_node = genradix_alloc_node(gfp_mask);
            if new_node.is_null() { return ptr::null_mut(); }
        }
        (*new_node).children[0] = n;
        let new_root = ((new_node as usize) | (if !n.is_null() { (level + 1) as usize } else { 0 })) as *mut genradix_root;
        if cmpxchg_release(ptr::addr_of_mut!((*radix).root), r, new_root) == r {
            v = new_root;
            new_node = ptr::null_mut();
        } else { (*new_node).children[0] = ptr::null_mut(); }
    }

    while level != 0 {
        level -= 1;
        let index = offset >> genradix_depth_shift(level);
        let p = &mut (*n).children[index];
        offset &= genradix_depth_size(level) - 1;
        n = read_once(p);
        if n.is_null() {
            if new_node.is_null() {
                new_node = genradix_alloc_node(gfp_mask);
                if new_node.is_null() { return ptr::null_mut(); }
            }
            let old = cmpxchg_release(p, ptr::null_mut(), new_node);
            if old.is_null() { core::mem::swap(&mut n, &mut new_node); } else { n = old; }
        }
    }
    if !new_node.is_null() { genradix_free_node(new_node); }
    (*n).data.as_mut_ptr().add(offset)
}

pub unsafe fn __genradix_iter_peek(iter: *mut genradix_iter, radix: *mut __genradix,
                                   objs_per_page: usize) -> *mut u8 {
    if (*iter).offset == usize::MAX { return ptr::null_mut(); }
    'restart: loop {
        let r = read_once(ptr::addr_of!((*radix).root)); if r.is_null() { return ptr::null_mut(); }
        let mut n = genradix_root_to_node(r); let mut level = genradix_root_to_depth(r);
        if ilog2((*iter).offset) >= genradix_depth_shift(level) { return ptr::null_mut(); }
        while level != 0 { level -= 1; let mut i = ((*iter).offset >> genradix_depth_shift(level)) & (GENRADIX_ARY - 1);
            while (*n).children[i].is_null() { let s = genradix_depth_size(level); if (*iter).offset + s < (*iter).offset { (*iter).offset=usize::MAX; (*iter).pos=usize::MAX; return ptr::null_mut(); }
                i += 1; (*iter).offset = ((*iter).offset+s) & !(s-1); (*iter).pos = ((*iter).offset >> GENRADIX_NODE_SHIFT) * objs_per_page; if i == GENRADIX_ARY { continue 'restart; } }
            n = (*n).children[i]; }
        return (*n).data.as_mut_ptr().add((*iter).offset & (GENRADIX_NODE_SIZE - 1));
    }
}

pub unsafe fn __genradix_iter_peek_prev(iter: *mut genradix_iter, radix: *mut __genradix,
                                        objs_per_page: usize, rem: usize) -> *mut u8 {
    if (*iter).offset == usize::MAX { return ptr::null_mut(); }
    'restart: loop { let r=read_once(ptr::addr_of!((*radix).root)); if r.is_null(){return ptr::null_mut();} let mut n=genradix_root_to_node(r); let mut level=genradix_root_to_depth(r);
        if ilog2((*iter).offset) >= genradix_depth_shift(level) { (*iter).offset=genradix_depth_size(level); (*iter).pos=((*iter).offset>>GENRADIX_NODE_SHIFT)*objs_per_page; (*iter).offset-=rem; (*iter).pos-=1; }
        while level!=0 { level-=1; let mut i=((*iter).offset>>genradix_depth_shift(level))&(GENRADIX_ARY-1); while (*n).children[i].is_null(){let s=genradix_depth_size(level); (*iter).offset &= !(s-1); (*iter).pos=((*iter).offset>>GENRADIX_NODE_SHIFT)*objs_per_page; if (*iter).offset==0{return ptr::null_mut();} (*iter).offset-=rem; (*iter).pos-=1; if i==0{continue 'restart;} i-=1;} n=(*n).children[i]; }
        return (*n).data.as_mut_ptr().add((*iter).offset&(GENRADIX_NODE_SIZE-1)); }
}

unsafe fn genradix_free_recurse(n: *mut genradix_node, level: u32) { if level!=0 { for i in 0..GENRADIX_ARY { if !(*n).children[i].is_null(){genradix_free_recurse((*n).children[i],level-1);} } } genradix_free_node(n); }

pub unsafe fn __genradix_prealloc(radix:*mut __genradix,size:usize,gfp_mask:gfp_t)->i32 { let mut offset=0; while offset<size { if __genradix_ptr_alloc(radix,offset,ptr::null_mut(),gfp_mask).is_null(){return -12;} offset+=GENRADIX_NODE_SIZE; } 0 }
pub unsafe fn __genradix_free(radix:*mut __genradix) { let r=xchg(ptr::addr_of_mut!((*radix).root),ptr::null_mut()); genradix_free_recurse(genradix_root_to_node(r),genradix_root_to_depth(r)); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
