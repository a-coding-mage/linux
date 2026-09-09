/* SPDX-License-Identifier: GPL-2.0-or-later */
/* A generic kernel FIFO implementation; direct Rust translation of kfifo.h. */

/* C includes are supplied by the surrounding kernel translation unit. */

#[repr(C)]
pub struct Scatterlist { _private: [u8; 0] }

#[repr(C)]
pub struct __kfifo {
    pub in_: ::core::ffi::c_uint,
    pub out: ::core::ffi::c_uint,
    pub mask: ::core::ffi::c_uint,
    pub esize: ::core::ffi::c_uint,
    pub data: *mut ::core::ffi::c_void,
}

/* __STRUCT_KFIFO_COMMON and the related C type-generating macros are represented
 * by this layout. The flexible storage is intentionally left caller-defined. */
#[repr(C)]
pub struct kfifo {
    pub kfifo: __kfifo,
    pub buf: [u8; 0],
}
#[repr(C)]
pub struct kfifo_rec_ptr_1 { pub kfifo: __kfifo, pub buf: [u8; 0] }
#[repr(C)]
pub struct kfifo_rec_ptr_2 { pub kfifo: __kfifo, pub buf: [u8; 0] }

#[inline]
pub fn __kfifo_uint_must_check_helper(val: ::core::ffi::c_uint) -> ::core::ffi::c_uint { val }
#[inline]
pub fn __kfifo_int_must_check_helper(val: ::core::ffi::c_int) -> ::core::ffi::c_int { val }

#[macro_export]
macro_rules! STRUCT_KFIFO_PTR { ($type:ty) => { $crate::kfifo }; }
#[macro_export]
macro_rules! STRUCT_KFIFO { ($type:ty, $size:expr) => { $crate::kfifo }; }
#[macro_export]
macro_rules! STRUCT_KFIFO_REC_1 { ($size:expr) => { $crate::kfifo_rec_ptr_1 }; }
#[macro_export]
macro_rules! STRUCT_KFIFO_REC_2 { ($size:expr) => { $crate::kfifo_rec_ptr_2 }; }
#[macro_export]
macro_rules! DECLARE_KFIFO_PTR { ($fifo:ident, $type:ty) => { let mut $fifo: $crate::kfifo = $crate::kfifo { kfifo: unsafe { ::core::mem::zeroed() }, buf: [] }; }; }
#[macro_export]
macro_rules! DECLARE_KFIFO { ($fifo:ident, $type:ty, $size:expr) => { let mut $fifo: $crate::kfifo = $crate::kfifo { kfifo: unsafe { ::core::mem::zeroed() }, buf: [] }; }; }

#[macro_export]
macro_rules! kfifo_initialized { ($fifo:expr) => { unsafe { (*($fifo)).kfifo.mask } }; }
#[macro_export]
macro_rules! kfifo_esize { ($fifo:expr) => { unsafe { (*($fifo)).kfifo.esize } }; }
#[macro_export]
macro_rules! kfifo_size { ($fifo:expr) => { unsafe { (*($fifo)).kfifo.mask.wrapping_add(1) } }; }
#[macro_export]
macro_rules! kfifo_len { ($fifo:expr) => { unsafe { (*($fifo)).kfifo.in_.wrapping_sub((*($fifo)).kfifo.out) } }; }
#[macro_export]
macro_rules! kfifo_is_empty { ($fifo:expr) => { unsafe { (*($fifo)).kfifo.in_ == (*($fifo)).kfifo.out } }; }
#[macro_export]
macro_rules! kfifo_is_full { ($fifo:expr) => { $crate::kfifo_len!($fifo) > unsafe { (*($fifo)).kfifo.mask } }; }
#[macro_export]
macro_rules! kfifo_avail { ($fifo:expr) => { $crate::kfifo_size!($fifo).wrapping_sub($crate::kfifo_len!($fifo)) }; }
#[macro_export]
macro_rules! kfifo_reset { ($fifo:expr) => {{ unsafe { (*($fifo)).kfifo.in_ = 0; (*($fifo)).kfifo.out = 0; } }}; }
#[macro_export]
macro_rules! kfifo_reset_out { ($fifo:expr) => {{ unsafe { (*($fifo)).kfifo.out = (*($fifo)).kfifo.in_; } }}; }
#[macro_export]
macro_rules! kfifo_skip_count { ($fifo:expr, $count:expr) => {{ unsafe { (*($fifo)).kfifo.out = (*($fifo)).kfifo.out.wrapping_add($count as _); } }}; }
#[macro_export]
macro_rules! kfifo_skip { ($fifo:expr) => { $crate::kfifo_skip_count!($fifo, 1) }; }

extern "C" {
    pub fn __kfifo_alloc_node(fifo: *mut __kfifo, size: ::core::ffi::c_uint, esize: usize, gfp_mask: usize, node: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn __kfifo_free(fifo: *mut __kfifo);
    pub fn __kfifo_init(fifo: *mut __kfifo, buffer: *mut ::core::ffi::c_void, size: ::core::ffi::c_uint, esize: usize) -> ::core::ffi::c_int;
    pub fn __kfifo_in(fifo: *mut __kfifo, buf: *const ::core::ffi::c_void, len: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    pub fn __kfifo_out(fifo: *mut __kfifo, buf: *mut ::core::ffi::c_void, len: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    pub fn __kfifo_from_user(fifo: *mut __kfifo, from: *const ::core::ffi::c_void, len: usize, copied: *mut ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn __kfifo_to_user(fifo: *mut __kfifo, to: *mut ::core::ffi::c_void, len: usize, copied: *mut ::core::ffi::c_uint) -> ::core::ffi::c_int;
    pub fn __kfifo_dma_in_prepare(fifo: *mut __kfifo, sgl: *mut Scatterlist, nents: ::core::ffi::c_int, len: ::core::ffi::c_uint, dma: u64) -> ::core::ffi::c_uint;
    pub fn __kfifo_dma_out_prepare(fifo: *mut __kfifo, sgl: *mut Scatterlist, nents: ::core::ffi::c_int, len: ::core::ffi::c_uint, dma: u64) -> ::core::ffi::c_uint;
    pub fn __kfifo_out_peek(fifo: *mut __kfifo, buf: *mut ::core::ffi::c_void, len: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    pub fn __kfifo_out_linear(fifo: *mut __kfifo, tail: *mut ::core::ffi::c_uint, n: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    pub fn __kfifo_in_r(fifo: *mut __kfifo, buf: *const ::core::ffi::c_void, len: ::core::ffi::c_uint, recsize: usize) -> ::core::ffi::c_uint;
    pub fn __kfifo_out_r(fifo: *mut __kfifo, buf: *mut ::core::ffi::c_void, len: ::core::ffi::c_uint, recsize: usize) -> ::core::ffi::c_uint;
    pub fn __kfifo_from_user_r(fifo: *mut __kfifo, from: *const ::core::ffi::c_void, len: usize, copied: *mut ::core::ffi::c_uint, recsize: usize) -> ::core::ffi::c_int;
    pub fn __kfifo_to_user_r(fifo: *mut __kfifo, to: *mut ::core::ffi::c_void, len: usize, copied: *mut ::core::ffi::c_uint, recsize: usize) -> ::core::ffi::c_int;
    pub fn __kfifo_dma_in_prepare_r(fifo: *mut __kfifo, sgl: *mut Scatterlist, nents: ::core::ffi::c_int, len: ::core::ffi::c_uint, recsize: usize, dma: u64) -> ::core::ffi::c_uint;
    pub fn __kfifo_dma_in_finish_r(fifo: *mut __kfifo, len: ::core::ffi::c_uint, recsize: usize);
    pub fn __kfifo_dma_out_prepare_r(fifo: *mut __kfifo, sgl: *mut Scatterlist, nents: ::core::ffi::c_int, len: ::core::ffi::c_uint, recsize: usize, dma: u64) -> ::core::ffi::c_uint;
    pub fn __kfifo_len_r(fifo: *mut __kfifo, recsize: usize) -> ::core::ffi::c_uint;
    pub fn __kfifo_skip_r(fifo: *mut __kfifo, recsize: usize);
    pub fn __kfifo_out_peek_r(fifo: *mut __kfifo, buf: *mut ::core::ffi::c_void, len: ::core::ffi::c_uint, recsize: usize) -> ::core::ffi::c_uint;
    pub fn __kfifo_out_linear_r(fifo: *mut __kfifo, tail: *mut ::core::ffi::c_uint, n: ::core::ffi::c_uint, recsize: usize) -> ::core::ffi::c_uint;
    pub fn __kfifo_max_r(len: ::core::ffi::c_uint, recsize: usize) -> ::core::ffi::c_uint;
}

#[macro_export] macro_rules! kfifo_in { ($fifo:expr, $buf:expr, $n:expr) => { unsafe { $crate::__kfifo_in(&mut (*($fifo)).kfifo, $buf as *const _, $n as _) } }; }
#[macro_export] macro_rules! kfifo_out { ($fifo:expr, $buf:expr, $n:expr) => { unsafe { $crate::__kfifo_out(&mut (*($fifo)).kfifo, $buf as *mut _, $n as _) } }; }
#[macro_export] macro_rules! kfifo_peek { ($fifo:expr, $buf:expr) => { $crate::kfifo_out!($fifo, $buf, 1) }; }
#[macro_export] macro_rules! kfifo_out_peek { ($fifo:expr, $buf:expr, $n:expr) => { unsafe { $crate::__kfifo_out_peek(&mut (*($fifo)).kfifo, $buf as *mut _, $n as _) } }; }
#[macro_export] macro_rules! kfifo_put { ($fifo:expr, $val:expr) => { $crate::kfifo_in!($fifo, &$val, 1) }; }
#[macro_export] macro_rules! kfifo_get { ($fifo:expr, $val:expr) => { $crate::kfifo_out!($fifo, $val, 1) }; }
#[macro_export] macro_rules! kfifo_alloc { ($fifo:expr, $size:expr, $gfp:expr) => { unsafe { $crate::__kfifo_alloc_node(&mut (*($fifo)).kfifo, $size, ::core::mem::size_of_val(&(*($fifo)).buf), $gfp, -1) } }; }
#[macro_export] macro_rules! kfifo_free { ($fifo:expr) => { unsafe { $crate::__kfifo_free(&mut (*($fifo)).kfifo) } }; }
#[macro_export] macro_rules! kfifo_init { ($fifo:expr, $buffer:expr, $size:expr) => { unsafe { $crate::__kfifo_init(&mut (*($fifo)).kfifo, $buffer, $size, 1) } }; }
#[macro_export] macro_rules! kfifo_in_locked { ($fifo:expr, $buf:expr, $n:expr, $lock:expr) => { $crate::kfifo_in!($fifo, $buf, $n) }; }
#[macro_export] macro_rules! kfifo_out_locked { ($fifo:expr, $buf:expr, $n:expr, $lock:expr) => { $crate::kfifo_out!($fifo, $buf, $n) }; }
#[macro_export] macro_rules! kfifo_dma_in_prepare { ($fifo:expr, $sgl:expr, $nents:expr, $len:expr) => { unsafe { $crate::__kfifo_dma_in_prepare(&mut (*($fifo)).kfifo, $sgl, $nents, $len, u64::MAX) } }; }
#[macro_export] macro_rules! kfifo_dma_out_prepare { ($fifo:expr, $sgl:expr, $nents:expr, $len:expr) => { unsafe { $crate::__kfifo_dma_out_prepare(&mut (*($fifo)).kfifo, $sgl, $nents, $len, u64::MAX) } }; }
#[macro_export] macro_rules! kfifo_dma_in_finish { ($fifo:expr, $len:expr) => { unsafe { (*($fifo)).kfifo.in_ = (*($fifo)).kfifo.in_.wrapping_add($len as _) } }; }
#[macro_export] macro_rules! kfifo_dma_out_finish { ($fifo:expr, $len:expr) => { $crate::kfifo_skip_count!($fifo, $len) }; }
#[macro_export] macro_rules! kfifo_out_linear { ($fifo:expr, $tail:expr, $n:expr) => { unsafe { $crate::__kfifo_out_linear(&mut (*($fifo)).kfifo, $tail, $n) } }; }
#[macro_export] macro_rules! kfifo_out_linear_ptr { ($fifo:expr, $ptr:expr, $n:expr) => { $crate::kfifo_out_linear!($fifo, $ptr, $n) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
