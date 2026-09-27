/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from include/linux/generic-radix-tree.h. */

/**
 * DOC: Generic radix trees/sparse arrays
 *
 * Very simple and minimalistic, supporting arbitrary size entries up to
 * GENRADIX_NODE_SIZE.
 *
 * A genradix is defined with the type it will store, like so:
 *
 * static GENRADIX(struct foo) foo_genradix;
 *
 * The main operations are:
 *
 * - genradix_init(radix) - initialize an empty genradix
 *
 * - genradix_free(radix) - free all memory owned by the genradix and
 *   reinitialize it
 *
 * - genradix_ptr(radix, idx) - gets a pointer to the entry at idx, returning
 *   NULL if that entry does not exist
 *
 * - genradix_ptr_alloc(radix, idx, gfp) - gets a pointer to an entry,
 *   allocating it if necessary
 *
 * - genradix_for_each(radix, iter, p) - iterate over each entry in a genradix
 *
 * The radix tree allocates one page of entries at a time, so entries may exist
 * that were never explicitly allocated - they will be initialized to all
 * zeroes.
 *
 * Internally, a genradix is just a radix tree of pages, and indexing works in
 * terms of byte offsets. The wrappers in this header file use sizeof on the
 * type the radix contains to calculate a byte offset from the index - see
 * __idx_to_offset.
 */

/* Depends on: asm/page.h, linux/bug.h, linux/limits.h, linux/log2.h,
 * linux/math.h, linux/slab.h, linux/types.h */

/* Opaque: the root pointer carries the tree depth in its low bits. */
#[repr(C)]
pub struct genradix_root {
    _opaque: [u8; 0],
}

pub const GENRADIX_NODE_SHIFT: u32 = 9;
pub const GENRADIX_NODE_SIZE: usize = 1 << GENRADIX_NODE_SHIFT;

pub const GENRADIX_ARY: usize = GENRADIX_NODE_SIZE / core::mem::size_of::<*mut genradix_node>();
pub const GENRADIX_ARY_SHIFT: u32 = ilog2!(GENRADIX_ARY) as u32;

/* depth that's needed for a genradix that can address up to ULONG_MAX: */
pub const GENRADIX_MAX_DEPTH: u32 =
    DIV_ROUND_UP!(BITS_PER_LONG as u32 - GENRADIX_NODE_SHIFT, GENRADIX_ARY_SHIFT);

pub const GENRADIX_DEPTH_MASK: kernel::ffi::c_ulong =
    roundup_pow_of_two!(GENRADIX_MAX_DEPTH + 1) - 1;

#[inline]
pub const fn genradix_depth_shift(depth: kernel::ffi::c_uint) -> kernel::ffi::c_int {
    (GENRADIX_NODE_SHIFT + GENRADIX_ARY_SHIFT * depth) as kernel::ffi::c_int
}

/*
 * Returns size (of data, in bytes) that a tree of a given depth holds:
 */
#[inline]
pub const fn genradix_depth_size(depth: kernel::ffi::c_uint) -> usize {
    1usize << genradix_depth_shift(depth)
}

#[inline]
pub fn genradix_root_to_depth(r: *mut genradix_root) -> kernel::ffi::c_uint {
    (r as kernel::ffi::c_ulong & GENRADIX_DEPTH_MASK) as kernel::ffi::c_uint
}

#[inline]
pub fn genradix_root_to_node(r: *mut genradix_root) -> *mut genradix_node {
    (r as kernel::ffi::c_ulong & !GENRADIX_DEPTH_MASK) as *mut genradix_node
}

#[repr(C)]
pub struct __genradix {
    pub root: *mut genradix_root,
}

#[repr(C)]
pub union genradix_node {
    /* Interior node: */
    pub children: [*mut genradix_node; GENRADIX_ARY],

    /* Leaf: */
    pub data: [u8; GENRADIX_NODE_SIZE],
}

#[inline]
pub unsafe fn genradix_alloc_node(gfp_mask: gfp_t) -> *mut genradix_node {
    unsafe { kzalloc(GENRADIX_NODE_SIZE, gfp_mask).cast() }
}

#[inline]
pub unsafe fn genradix_free_node(node: *mut genradix_node) {
    unsafe { kfree(node.cast()) }
}

/*
 * NOTE: currently, sizeof(_type) must not be larger than GENRADIX_NODE_SIZE:
 */

#[macro_export]
macro_rules! __GENRADIX_INITIALIZER {
    () => {
        GENRADIX {
            tree: __genradix {
                root: core::ptr::null_mut(),
            },
            r#type: core::marker::PhantomData,
        }
    };
}

/*
 * We use a 0 size array to stash the type we're storing without taking any
 * space at runtime - then the various accessor macros can use typeof() to get
 * to it for casts/sizeof - we also force the alignment so that storing a type
 * with a ridiculous alignment doesn't blow up the alignment or size of the
 * genradix.
 *
 * In Rust the stored type is the generic parameter; PhantomData<T> is the
 * zero-sized, align(1) equivalent of `_type type[0] __aligned(1)`.
 */
#[repr(C)]
pub struct GENRADIX<T> {
    pub tree: __genradix,
    pub r#type: core::marker::PhantomData<T>,
}

#[macro_export]
macro_rules! DEFINE_GENRADIX {
    ($name:ident, $type:ty) => {
        static mut $name: GENRADIX<$type> = __GENRADIX_INITIALIZER!();
    };
}

/**
 * genradix_init - initialize a genradix
 * @_radix:	genradix to initialize
 *
 * Does not fail
 */
#[inline]
pub unsafe fn genradix_init<T>(radix: *mut GENRADIX<T>) {
    unsafe { *radix = __GENRADIX_INITIALIZER!() };
}

extern "C" {
    pub fn __genradix_free(radix: *mut __genradix);
}

/**
 * genradix_free: free all memory owned by a genradix
 * @_radix: the genradix to free
 *
 * After freeing, @_radix will be reinitialized and empty
 */
#[inline]
pub unsafe fn genradix_free<T>(radix: *mut GENRADIX<T>) {
    unsafe { __genradix_free(&raw mut (*radix).tree) }
}

#[inline]
pub fn __idx_to_offset(idx: usize, obj_size: usize) -> usize {
    BUG_ON!(obj_size > GENRADIX_NODE_SIZE);

    if !is_power_of_2(obj_size as kernel::ffi::c_ulong) {
        let objs_per_page = GENRADIX_NODE_SIZE / obj_size;

        (idx / objs_per_page) * GENRADIX_NODE_SIZE + (idx % objs_per_page) * obj_size
    } else {
        idx.wrapping_mul(obj_size)
    }
}

/* __genradix_cast() is the `.cast::<T>()` in each typed wrapper below. */

#[inline]
pub const fn __genradix_obj_size<T>(_radix: *const GENRADIX<T>) -> usize {
    core::mem::size_of::<T>()
}

#[inline]
pub const fn __genradix_objs_per_page<T>(_radix: *const GENRADIX<T>) -> usize {
    GENRADIX_NODE_SIZE / core::mem::size_of::<T>()
}

#[inline]
pub const fn __genradix_page_remainder<T>(_radix: *const GENRADIX<T>) -> usize {
    GENRADIX_NODE_SIZE % core::mem::size_of::<T>()
}

#[inline]
pub fn __genradix_idx_to_offset<T>(_radix: *const GENRADIX<T>, idx: usize) -> usize {
    __idx_to_offset(idx, core::mem::size_of::<T>())
}

#[inline]
pub unsafe fn __genradix_ptr_inlined(
    radix: *mut __genradix,
    mut offset: usize,
) -> *mut kernel::ffi::c_void {
    /* READ_ONCE(radix->root) */
    let r = unsafe { core::ptr::read_volatile(&raw const (*radix).root) };
    let mut n = genradix_root_to_node(r);
    let level = genradix_root_to_depth(r);
    let mut shift = genradix_depth_shift(level);

    if unlikely!(ilog2!(offset) >= genradix_depth_shift(level)) {
        return core::ptr::null_mut();
    }

    while !n.is_null() && shift > GENRADIX_NODE_SHIFT as kernel::ffi::c_int {
        shift -= GENRADIX_ARY_SHIFT as kernel::ffi::c_int;
        n = unsafe { (*n).children[offset >> shift] };
        offset &= (1usize << shift) - 1;
    }

    if n.is_null() {
        core::ptr::null_mut()
    } else {
        unsafe { (&raw mut (*n).data).cast::<u8>().add(offset).cast() }
    }
}

#[inline]
pub unsafe fn genradix_ptr_inlined<T>(radix: *mut GENRADIX<T>, idx: usize) -> *mut T {
    unsafe {
        __genradix_ptr_inlined(&raw mut (*radix).tree, __genradix_idx_to_offset(radix, idx))
            .cast()
    }
}

extern "C" {
    pub fn __genradix_ptr(radix: *mut __genradix, offset: usize) -> *mut kernel::ffi::c_void;
}

/**
 * genradix_ptr - get a pointer to a genradix entry
 * @_radix:	genradix to access
 * @_idx:	index to fetch
 *
 * Returns a pointer to entry at @_idx, or NULL if that entry does not exist.
 */
#[inline]
pub unsafe fn genradix_ptr<T>(radix: *mut GENRADIX<T>, idx: usize) -> *mut T {
    unsafe {
        __genradix_ptr(&raw mut (*radix).tree, __genradix_idx_to_offset(radix, idx)).cast()
    }
}

extern "C" {
    pub fn __genradix_ptr_alloc(
        radix: *mut __genradix,
        offset: usize,
        preallocated: *mut *mut genradix_node,
        gfp_mask: gfp_t,
    ) -> *mut kernel::ffi::c_void;
}

#[inline]
pub unsafe fn genradix_ptr_alloc_inlined<T>(
    radix: *mut GENRADIX<T>,
    idx: usize,
    gfp: gfp_t,
) -> *mut T {
    unsafe { genradix_ptr_alloc_preallocated_inlined(radix, idx, core::ptr::null_mut(), gfp) }
}

#[inline]
pub unsafe fn genradix_ptr_alloc_preallocated_inlined<T>(
    radix: *mut GENRADIX<T>,
    idx: usize,
    new_node: *mut *mut genradix_node,
    gfp: gfp_t,
) -> *mut T {
    let offset = __genradix_idx_to_offset(radix, idx);
    unsafe {
        let p = __genradix_ptr_inlined(&raw mut (*radix).tree, offset);
        if !p.is_null() {
            p.cast()
        } else {
            __genradix_ptr_alloc(&raw mut (*radix).tree, offset, new_node, gfp).cast()
        }
    }
}

/**
 * genradix_ptr_alloc - get a pointer to a genradix entry, allocating it
 *			if necessary
 * @_radix:	genradix to access
 * @_idx:	index to fetch
 * @_gfp:	gfp mask
 *
 * Returns a pointer to entry at @_idx, or NULL on allocation failure
 */
#[inline]
pub unsafe fn genradix_ptr_alloc<T>(radix: *mut GENRADIX<T>, idx: usize, gfp: gfp_t) -> *mut T {
    unsafe { genradix_ptr_alloc_preallocated(radix, idx, core::ptr::null_mut(), gfp) }
}

#[inline]
pub unsafe fn genradix_ptr_alloc_preallocated<T>(
    radix: *mut GENRADIX<T>,
    idx: usize,
    new_node: *mut *mut genradix_node,
    gfp: gfp_t,
) -> *mut T {
    unsafe {
        __genradix_ptr_alloc(
            &raw mut (*radix).tree,
            __genradix_idx_to_offset(radix, idx),
            new_node,
            gfp,
        )
        .cast()
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct genradix_iter {
    pub offset: usize,
    pub pos: usize,
}

/**
 * genradix_iter_init - initialize a genradix_iter
 * @_radix:	genradix that will be iterated over
 * @_idx:	index to start iterating from
 */
#[inline]
pub fn genradix_iter_init<T>(radix: *const GENRADIX<T>, idx: usize) -> genradix_iter {
    genradix_iter {
        pos: idx,
        offset: __genradix_idx_to_offset(radix, idx),
    }
}

extern "C" {
    pub fn __genradix_iter_peek(
        iter: *mut genradix_iter,
        radix: *mut __genradix,
        objs_per_page: usize,
    ) -> *mut kernel::ffi::c_void;
}

/**
 * genradix_iter_peek - get first entry at or above iterator's current
 *			position
 * @_iter:	a genradix_iter
 * @_radix:	genradix being iterated over
 *
 * If no more entries exist at or above @_iter's current position, returns NULL
 */
#[inline]
pub unsafe fn genradix_iter_peek<T>(iter: *mut genradix_iter, radix: *mut GENRADIX<T>) -> *mut T {
    unsafe {
        __genradix_iter_peek(iter, &raw mut (*radix).tree, __genradix_objs_per_page(radix)).cast()
    }
}

extern "C" {
    pub fn __genradix_iter_peek_prev(
        iter: *mut genradix_iter,
        radix: *mut __genradix,
        objs_per_page: usize,
        obj_size_plus_page_remainder: usize,
    ) -> *mut kernel::ffi::c_void;
}

/**
 * genradix_iter_peek_prev - get first entry at or below iterator's current
 *			     position
 * @_iter:	a genradix_iter
 * @_radix:	genradix being iterated over
 *
 * If no more entries exist at or below @_iter's current position, returns NULL
 */
#[inline]
pub unsafe fn genradix_iter_peek_prev<T>(
    iter: *mut genradix_iter,
    radix: *mut GENRADIX<T>,
) -> *mut T {
    unsafe {
        __genradix_iter_peek_prev(
            iter,
            &raw mut (*radix).tree,
            __genradix_objs_per_page(radix),
            __genradix_obj_size(radix) + __genradix_page_remainder(radix),
        )
        .cast()
    }
}

#[inline]
pub fn __genradix_iter_advance(iter: &mut genradix_iter, obj_size: usize) {
    let Some(offset) = iter.offset.checked_add(obj_size) else {
        iter.offset = SIZE_MAX;
        iter.pos = SIZE_MAX;
        return;
    };

    iter.offset = offset;

    if !is_power_of_2(obj_size as kernel::ffi::c_ulong)
        && (iter.offset & (GENRADIX_NODE_SIZE - 1)) + obj_size > GENRADIX_NODE_SIZE
    {
        iter.offset = iter.offset.next_multiple_of(GENRADIX_NODE_SIZE);
    }

    iter.pos = iter.pos.wrapping_add(1);
}

#[inline]
pub fn genradix_iter_advance<T>(iter: &mut genradix_iter, radix: *const GENRADIX<T>) {
    __genradix_iter_advance(iter, __genradix_obj_size(radix))
}

#[inline]
pub fn __genradix_iter_rewind(iter: &mut genradix_iter, obj_size: usize) {
    if iter.offset == 0 || iter.offset == SIZE_MAX {
        iter.offset = SIZE_MAX;
        return;
    }

    if (iter.offset & (GENRADIX_NODE_SIZE - 1)) == 0 {
        iter.offset -= GENRADIX_NODE_SIZE % obj_size;
    }

    iter.offset -= obj_size;
    iter.pos = iter.pos.wrapping_sub(1);
}

#[inline]
pub fn genradix_iter_rewind<T>(iter: &mut genradix_iter, radix: *const GENRADIX<T>) {
    __genradix_iter_rewind(iter, __genradix_obj_size(radix))
}

#[macro_export]
macro_rules! genradix_for_each_from {
    ($radix:expr, $iter:ident, $p:ident, $start:expr, $body:block) => {{
        let __radix = $radix;
        $iter = genradix_iter_init(__radix, $start);
        loop {
            $p = unsafe { genradix_iter_peek(&mut $iter, __radix) };
            if $p.is_null() {
                break;
            }
            $body
            genradix_iter_advance(&mut $iter, __radix);
        }
    }};
}

/**
 * genradix_for_each - iterate over entry in a genradix
 * @_radix:	genradix to iterate over
 * @_iter:	a genradix_iter to track current position
 * @_p:		pointer to genradix entry type
 *
 * On every iteration, @_p will point to the current entry, and @_iter.pos
 * will be the current entry's index.
 */
#[macro_export]
macro_rules! genradix_for_each {
    ($radix:expr, $iter:ident, $p:ident, $body:block) => {
        genradix_for_each_from!($radix, $iter, $p, 0, $body)
    };
}

#[inline]
pub const fn genradix_last_pos<T>(radix: *const GENRADIX<T>) -> usize {
    SIZE_MAX / GENRADIX_NODE_SIZE * __genradix_objs_per_page(radix) - 1
}

/**
 * genradix_for_each_reverse - iterate over entry in a genradix, reverse order
 * @_radix:	genradix to iterate over
 * @_iter:	a genradix_iter to track current position
 * @_p:		pointer to genradix entry type
 *
 * On every iteration, @_p will point to the current entry, and @_iter.pos
 * will be the current entry's index.
 */
#[macro_export]
macro_rules! genradix_for_each_reverse {
    ($radix:expr, $iter:ident, $p:ident, $body:block) => {{
        let __radix = $radix;
        $iter = genradix_iter_init(__radix, genradix_last_pos(__radix));
        loop {
            $p = unsafe { genradix_iter_peek_prev(&mut $iter, __radix) };
            if $p.is_null() {
                break;
            }
            $body
            genradix_iter_rewind(&mut $iter, __radix);
        }
    }};
}

extern "C" {
    pub fn __genradix_prealloc(
        radix: *mut __genradix,
        size: usize,
        gfp_mask: gfp_t,
    ) -> kernel::ffi::c_int;
}

/**
 * genradix_prealloc - preallocate entries in a generic radix tree
 * @_radix:	genradix to preallocate
 * @_nr:	number of entries to preallocate
 * @_gfp:	gfp mask
 *
 * Returns 0 on success, -ENOMEM on failure
 */
#[inline]
pub unsafe fn genradix_prealloc<T>(
    radix: *mut GENRADIX<T>,
    nr: usize,
    gfp: gfp_t,
) -> kernel::ffi::c_int {
    unsafe {
        __genradix_prealloc(
            &raw mut (*radix).tree,
            __genradix_idx_to_offset(radix, nr + 1),
            gfp,
        )
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
