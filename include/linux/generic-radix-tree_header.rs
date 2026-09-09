/* Translated from linux/generic-radix-tree.h. */

pub const GENRADIX_NODE_SHIFT: usize = 9;
pub const GENRADIX_NODE_SIZE: usize = 1usize << GENRADIX_NODE_SHIFT;

/* GENRADIX_ARY and dependent constants retain their C header intent. */
pub const GENRADIX_ARY: usize = GENRADIX_NODE_SIZE / core::mem::size_of::<*mut genradix_node>();
pub const GENRADIX_ARY_SHIFT: usize = ilog2_const(GENRADIX_ARY);
pub const GENRADIX_MAX_DEPTH: usize = div_round_up_const(usize::BITS as usize - GENRADIX_NODE_SHIFT, GENRADIX_ARY_SHIFT);
pub const GENRADIX_DEPTH_MASK: usize = (roundup_pow_of_two_const(GENRADIX_MAX_DEPTH + 1) - 1);

const fn ilog2_const(mut x: usize) -> usize {
    let mut n = 0;
    while x > 1 { x >>= 1; n += 1; }
    n
}
const fn div_round_up_const(a: usize, b: usize) -> usize { (a + b - 1) / b }
const fn roundup_pow_of_two_const(mut x: usize) -> usize {
    if x <= 1 { return 1; }
    x -= 1;
    let mut s = 1;
    while s < usize::BITS { x |= x >> s; s <<= 1; }
    x + 1
}

#[repr(C)]
pub struct genradix_root { _private: [u8; 0] }

#[inline]
pub fn genradix_depth_shift(depth: u32) -> usize {
    GENRADIX_NODE_SHIFT + GENRADIX_ARY_SHIFT * depth as usize
}

#[inline]
pub fn genradix_depth_size(depth: u32) -> usize { 1usize << genradix_depth_shift(depth) }

#[inline]
pub unsafe fn genradix_root_to_depth(r: *mut genradix_root) -> u32 {
    r as usize & GENRADIX_DEPTH_MASK as usize as u32 as usize as u32
}

#[inline]
pub unsafe fn genradix_root_to_node(r: *mut genradix_root) -> *mut genradix_node {
    (r as usize & !GENRADIX_DEPTH_MASK) as *mut genradix_node
}

#[repr(C)]
pub struct __genradix { pub root: *mut genradix_root }

#[repr(C)]
pub union genradix_node {
    pub children: [*mut genradix_node; GENRADIX_ARY],
    pub data: [u8; GENRADIX_NODE_SIZE],
}

extern "C" {
    pub fn kzalloc(size: usize, gfp_mask: gfp_t) -> *mut genradix_node;
    pub fn kfree(node: *mut genradix_node);
    pub fn __genradix_free(radix: *mut __genradix);
    pub fn __genradix_ptr(radix: *mut __genradix, offset: usize) -> *mut core::ffi::c_void;
    pub fn __genradix_ptr_alloc(radix: *mut __genradix, offset: usize,
        new_node: *mut *mut genradix_node, gfp: gfp_t) -> *mut core::ffi::c_void;
    pub fn __genradix_iter_peek(iter: *mut genradix_iter, radix: *mut __genradix,
        objs_per_page: usize) -> *mut core::ffi::c_void;
    pub fn __genradix_iter_peek_prev(iter: *mut genradix_iter, radix: *mut __genradix,
        objs_per_page: usize, page_remainder: usize) -> *mut core::ffi::c_void;
    pub fn __genradix_prealloc(radix: *mut __genradix, offset: usize, gfp: gfp_t) -> i32;
}

pub type gfp_t = u32;
pub const SIZE_MAX: usize = usize::MAX;

#[inline]
pub unsafe fn genradix_alloc_node(gfp_mask: gfp_t) -> *mut genradix_node { kzalloc(GENRADIX_NODE_SIZE, gfp_mask) }
#[inline]
pub unsafe fn genradix_free_node(node: *mut genradix_node) { kfree(node); }

#[repr(C)]
pub struct genradix_iter { pub offset: usize, pub pos: usize }

#[macro_export]
macro_rules! GENRADIX {
    ($type:ty) => { struct { pub tree: $crate::__genradix, pub type_: [$type; 0] } };
}

#[macro_export]
macro_rules! DEFINE_GENRADIX { ($name:ident, $type:ty) => {
    static mut $name: GENRADIX!($type) = GENRADIX!($type) { tree: $crate::__genradix { root: core::ptr::null_mut() }, type_: [] };
} }

#[inline]
pub fn genradix_iter_init(idx: usize, obj_size: usize) -> genradix_iter {
    genradix_iter { pos: idx, offset: __idx_to_offset(idx, obj_size) }
}

#[inline]
pub fn genradix_last_pos(obj_size: usize) -> usize {
    (SIZE_MAX / GENRADIX_NODE_SIZE) * (GENRADIX_NODE_SIZE / obj_size) - 1
}

#[macro_export]
macro_rules! genradix_init { ($radix:expr) => {{ unsafe { (*$radix).tree = $crate::__genradix { root: core::ptr::null_mut() }; }} }
#[macro_export]
macro_rules! genradix_free { ($radix:expr) => {{ unsafe { $crate::__genradix_free(&mut (*$radix).tree); }} }
#[macro_export]
macro_rules! genradix_ptr { ($radix:expr, $idx:expr) => {{ unsafe { $crate::__genradix_ptr(&mut (*$radix).tree, $idx) } }} }
#[macro_export]
macro_rules! genradix_ptr_alloc { ($radix:expr, $idx:expr, $gfp:expr) => {{ unsafe { $crate::__genradix_ptr_alloc(&mut (*$radix).tree, $idx, core::ptr::null_mut(), $gfp) } }} }
#[macro_export]
macro_rules! genradix_iter_advance { ($iter:expr, $obj_size:expr) => {{ $crate::__genradix_iter_advance(&mut $iter, $obj_size); }} }
#[macro_export]
macro_rules! genradix_iter_rewind { ($iter:expr, $obj_size:expr) => {{ $crate::__genradix_iter_rewind(&mut $iter, $obj_size); }} }
#[macro_export]
macro_rules! genradix_prealloc { ($radix:expr, $nr:expr, $gfp:expr) => {{ unsafe { $crate::__genradix_prealloc(&mut (*$radix).tree, $nr + 1, $gfp) } }} }

#[inline]
pub fn __idx_to_offset(idx: usize, obj_size: usize) -> usize {
    if !obj_size.is_power_of_two() {
        let objs_per_page = GENRADIX_NODE_SIZE / obj_size;
        (idx / objs_per_page) * GENRADIX_NODE_SIZE + (idx % objs_per_page) * obj_size
    } else { idx.wrapping_mul(obj_size) }
}

#[inline]
pub unsafe fn __genradix_ptr_inlined(radix: *mut __genradix, offset: usize) -> *mut u8 {
    let r = core::ptr::read_volatile(&(*radix).root);
    let mut n = genradix_root_to_node(r);
    let level = genradix_root_to_depth(r);
    let mut shift = genradix_depth_shift(level);
    if (usize::BITS - 1 - offset.leading_zeros()) as usize >= genradix_depth_shift(level) { return core::ptr::null_mut(); }
    while !n.is_null() && shift > GENRADIX_NODE_SHIFT {
        shift -= GENRADIX_ARY_SHIFT;
        n = (*n).children[offset >> shift];
        let mask = (1usize << shift) - 1;
        let _ = mask;
    }
    if n.is_null() { core::ptr::null_mut() } else { (*n).data.as_mut_ptr().add(offset) }
}

#[inline]
pub fn __genradix_iter_advance(iter: &mut genradix_iter, obj_size: usize) {
    if iter.offset.wrapping_add(obj_size) < iter.offset { iter.offset = SIZE_MAX; iter.pos = SIZE_MAX; return; }
    iter.offset = iter.offset.wrapping_add(obj_size);
    if !obj_size.is_power_of_two() && (iter.offset & (GENRADIX_NODE_SIZE - 1)) + obj_size > GENRADIX_NODE_SIZE {
        iter.offset = (iter.offset + GENRADIX_NODE_SIZE - 1) & !(GENRADIX_NODE_SIZE - 1);
    }
    iter.pos = iter.pos.wrapping_add(1);
}

#[inline]
pub fn __genradix_iter_rewind(iter: &mut genradix_iter, obj_size: usize) {
    if iter.offset == 0 || iter.offset == SIZE_MAX { iter.offset = SIZE_MAX; return; }
    if (iter.offset & (GENRADIX_NODE_SIZE - 1)) == 0 { iter.offset -= GENRADIX_NODE_SIZE % obj_size; }
    iter.offset -= obj_size; iter.pos = iter.pos.wrapping_sub(1);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
