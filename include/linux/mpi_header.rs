/* SPDX-License-Identifier: GPL-2.0-or-later */
/* mpi.h  -  Multi Precision Integers
 *	Copyright (C) 1994, 1996, 1998, 1999,
 *                    2000, 2001 Free Software Foundation, Inc.
 *
 * This file is part of GNUPG.
 *
 * Note: This code is heavily based on the GNU MP Library.
 *	 Actually it's the same code with only minor changes in the
 *	 way the data is stored; this is to support the abstraction
 *	 of an optional secure memory allocation which may be used
 *	 to avoid revealing of sensitive data due to paging etc.
 *	 The GNU MP Library itself is published under the LGPL;
 *	 however I decided to publish this code under the plain GPL.
 */

// Dependencies supplied by the surrounding Linux code are intentionally not
// implemented here: linux/types.h and linux/scatterlist.h.

pub const BYTES_PER_MPI_LIMB: usize = BITS_PER_LONG / 8;
pub const BITS_PER_MPI_LIMB: usize = BITS_PER_LONG;

pub type mpi_limb_t = ::kernel::ffi::c_ulong;
pub type mpi_limb_signed_t = ::kernel::ffi::c_long;

#[repr(C)]
pub struct gcry_mpi {
	pub alloced: ::kernel::ffi::c_int,
	pub nlimbs: ::kernel::ffi::c_int,
	pub nbits: ::kernel::ffi::c_int,
	pub sign: ::kernel::ffi::c_int,
	pub flags: ::kernel::ffi::c_uint,
	pub d: *mut mpi_limb_t,
}

pub type MPI = *mut gcry_mpi;

#[inline]
pub unsafe fn mpi_get_nlimbs(a: MPI) -> ::kernel::ffi::c_int {
	(*a).nlimbs
}

extern "C" {
	pub fn mpi_alloc(nlimbs: ::kernel::ffi::c_uint) -> MPI;
	pub fn mpi_free(a: MPI);
	pub fn mpi_resize(a: MPI, nlimbs: ::kernel::ffi::c_uint) -> ::kernel::ffi::c_int;

	pub fn mpi_copy(a: MPI) -> MPI;

	pub fn mpi_read_raw_data(
		xbuffer: *const ::kernel::ffi::c_void,
		nbytes: usize,
	) -> MPI;
	pub fn mpi_read_from_buffer(
		buffer: *const ::kernel::ffi::c_void,
		ret_nread: *mut ::kernel::ffi::c_uint,
	) -> MPI;
	pub fn mpi_read_raw_from_sgl(sgl: *mut scatterlist, len: ::kernel::ffi::c_uint) -> MPI;
	pub fn mpi_get_buffer(
		a: MPI,
		nbytes: *mut ::kernel::ffi::c_uint,
		sign: *mut ::kernel::ffi::c_int,
	) -> *mut ::kernel::ffi::c_void;
	pub fn mpi_read_buffer(
		a: MPI,
		buf: *mut u8,
		buf_len: ::kernel::ffi::c_uint,
		nbytes: *mut ::kernel::ffi::c_uint,
		sign: *mut ::kernel::ffi::c_int,
	) -> ::kernel::ffi::c_int;
	pub fn mpi_write_to_sgl(
		a: MPI,
		sg: *mut scatterlist,
		nbytes: ::kernel::ffi::c_uint,
		sign: *mut ::kernel::ffi::c_int,
	) -> ::kernel::ffi::c_int;

	pub fn mpi_mod(rem: MPI, dividend: MPI, divisor: MPI) -> ::kernel::ffi::c_int;
	pub fn mpi_powm(res: MPI, base: MPI, exp: MPI, r#mod: MPI) -> ::kernel::ffi::c_int;

	pub fn mpi_cmp_ui(u: MPI, v: ::kernel::ffi::c_ulong) -> ::kernel::ffi::c_int;
	pub fn mpi_cmp(u: MPI, v: MPI) -> ::kernel::ffi::c_int;

	pub fn mpi_sub_ui(
		w: MPI,
		u: MPI,
		vval: ::kernel::ffi::c_ulong,
	) -> ::kernel::ffi::c_int;

	pub fn mpi_normalize(a: MPI);
	pub fn mpi_get_nbits(a: MPI) -> ::kernel::ffi::c_uint;
	pub fn mpi_test_bit(a: MPI, n: ::kernel::ffi::c_uint) -> ::kernel::ffi::c_int;
	pub fn mpi_set_bit(a: MPI, n: ::kernel::ffi::c_uint) -> ::kernel::ffi::c_int;
	pub fn mpi_rshift(x: MPI, a: MPI, n: ::kernel::ffi::c_uint) -> ::kernel::ffi::c_int;

	pub fn mpi_add(w: MPI, u: MPI, v: MPI) -> ::kernel::ffi::c_int;
	pub fn mpi_sub(w: MPI, u: MPI, v: MPI) -> ::kernel::ffi::c_int;
	pub fn mpi_addm(w: MPI, u: MPI, v: MPI, m: MPI) -> ::kernel::ffi::c_int;
	pub fn mpi_subm(w: MPI, u: MPI, v: MPI, m: MPI) -> ::kernel::ffi::c_int;

	pub fn mpi_mul(w: MPI, u: MPI, v: MPI) -> ::kernel::ffi::c_int;
	pub fn mpi_mulm(w: MPI, u: MPI, v: MPI, m: MPI) -> ::kernel::ffi::c_int;

	pub fn mpi_tdiv_r(rem: MPI, num: MPI, den: MPI) -> ::kernel::ffi::c_int;
	pub fn mpi_fdiv_r(rem: MPI, dividend: MPI, divisor: MPI) -> ::kernel::ffi::c_int;
}

/// mpi_get_size() - returns max size required to store the number
///
/// @a: A multi precision integer for which we want to allocate a buffer
///
/// Return: size required to store the number
#[inline]
pub unsafe fn mpi_get_size(a: MPI) -> ::kernel::ffi::c_uint {
	((*a).nlimbs as ::kernel::ffi::c_uint) * (BYTES_PER_MPI_LIMB as ::kernel::ffi::c_uint)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
