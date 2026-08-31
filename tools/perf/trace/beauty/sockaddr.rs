// SPDX-License-Identifier: LGPL-2.1
// Copyright (C) 2018, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>

// C dependencies:
// #include "trace/beauty/beauty.h"
// #include <sys/socket.h>
// #include <sys/types.h>
// #include <sys/un.h>
// #include <arpa/inet.h>
//
// #include "trace/beauty/generated/sockaddr.c"
// DEFINE_STRARRAY(socket_families, "PF_");

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type size_t = usize;
type socklen_t = c_uint;
type sa_family_t = u16;
type in_port_t = u16;
type u32 = u32;

const AF_LOCAL: usize = 1;
const AF_INET: usize = 2;
const AF_INET6: usize = 10;
const AF_SCNPRINTFS_SIZE: usize = AF_INET6 + 1;

#[repr(C)]
pub struct sockaddr {
	pub sa_family: sa_family_t,
	pub sa_data: [c_char; 14],
}

#[repr(C)]
pub struct in_addr {
	pub s_addr: u32,
}

#[repr(C)]
pub struct in6_addr {
	pub s6_addr: [u8; 16],
}

#[repr(C)]
pub struct sockaddr_in {
	pub sin_family: sa_family_t,
	pub sin_port: in_port_t,
	pub sin_addr: in_addr,
	pub sin_zero: [u8; 8],
}

#[repr(C)]
pub struct sockaddr_in6 {
	pub sin6_family: sa_family_t,
	pub sin6_port: in_port_t,
	pub sin6_flowinfo: u32,
	pub sin6_addr: in6_addr,
	pub sin6_scope_id: u32,
}

#[repr(C)]
pub struct sockaddr_un {
	pub sun_family: sa_family_t,
	pub sun_path: [c_char; 108],
}

#[repr(C)]
pub struct augmented_args_payload {
	pub value: c_ulong,
}

#[repr(C)]
pub struct augmented_arg {
	pub args: *mut augmented_args_payload,
}

#[repr(C)]
pub struct syscall_arg {
	pub val: c_ulong,
	pub augmented: augmented_arg,
	pub show_string_prefix: bool,
}

#[repr(C)]
pub struct strarray {
	_private: [u8; 0],
}

unsafe extern "C" {
	static strarray__socket_families: strarray;

	fn scnprintf(bf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> size_t;
	fn strarray__scnprintf(
		sra: *const strarray,
		bf: *mut c_char,
		size: size_t,
		fmt: *const c_char,
		show_prefix: bool,
		idx: c_int,
	) -> size_t;
	fn inet_ntop(
		af: c_int,
		src: *const c_void,
		dst: *mut c_char,
		size: socklen_t,
	) -> *const c_char;
	fn ntohs(netshort: u16) -> u16;
	fn ntohl(netlong: u32) -> u32;
}

type AfScnprintf = unsafe extern "C" fn(sa: *mut sockaddr, bf: *mut c_char, size: size_t) -> size_t;

unsafe extern "C" fn af_inet__scnprintf(
	sa: *mut sockaddr,
	bf: *mut c_char,
	size: size_t,
) -> size_t {
	let sin = sa as *mut sockaddr_in;
	let mut tmp: [c_char; 16] = [0; 16];

	unsafe {
		scnprintf(
			bf,
			size,
			c", port: %d, addr: %s".as_ptr(),
			ntohs((*sin).sin_port) as c_int,
			inet_ntop(
				(*sin).sin_family as c_int,
				&raw const (*sin).sin_addr as *const c_void,
				tmp.as_mut_ptr(),
				core::mem::size_of_val(&tmp) as socklen_t,
			),
		)
	}
}

unsafe extern "C" fn af_inet6__scnprintf(
	sa: *mut sockaddr,
	bf: *mut c_char,
	size: size_t,
) -> size_t {
	let sin6 = sa as *mut sockaddr_in6;
	let flowinfo: u32 = unsafe { ntohl((*sin6).sin6_flowinfo) };
	let mut tmp: [c_char; 512] = [0; 512];
	let mut printed: size_t = unsafe {
		scnprintf(
			bf,
			size,
			c", port: %d, addr: %s".as_ptr(),
			ntohs((*sin6).sin6_port) as c_int,
			inet_ntop(
				(*sin6).sin6_family as c_int,
				&raw const (*sin6).sin6_addr as *const c_void,
				tmp.as_mut_ptr(),
				core::mem::size_of_val(&tmp) as socklen_t,
			),
		)
	};

	if flowinfo != 0 {
		printed += unsafe {
			scnprintf(
				bf.add(printed),
				size.wrapping_sub(printed),
				c", flowinfo: %lu".as_ptr(),
				flowinfo as c_ulong,
			)
		};
	}
	if unsafe { (*sin6).sin6_scope_id } != 0 {
		printed += unsafe {
			scnprintf(
				bf.add(printed),
				size.wrapping_sub(printed),
				c", scope_id: %lu".as_ptr(),
				(*sin6).sin6_scope_id as c_ulong,
			)
		};
	}

	printed
}

unsafe extern "C" fn af_local__scnprintf(
	sa: *mut sockaddr,
	bf: *mut c_char,
	size: size_t,
) -> size_t {
	let sun = sa as *mut sockaddr_un;

	unsafe { scnprintf(bf, size, c", path: %s".as_ptr(), (*sun).sun_path.as_ptr()) }
}

static af_scnprintfs: [Option<AfScnprintf>; AF_SCNPRINTFS_SIZE] = {
	let mut a: [Option<AfScnprintf>; AF_SCNPRINTFS_SIZE] = [None; AF_SCNPRINTFS_SIZE];
	a[AF_LOCAL] = Some(af_local__scnprintf);
	a[AF_INET] = Some(af_inet__scnprintf);
	a[AF_INET6] = Some(af_inet6__scnprintf);
	a
};

unsafe extern "C" fn syscall_arg__scnprintf_augmented_sockaddr(
	arg: *mut syscall_arg,
	bf: *mut c_char,
	size: size_t,
) -> size_t {
	let sa = unsafe { &raw mut (*(*arg).augmented.args).value as *mut sockaddr };
	let mut family: [c_char; 32] = [0; 32];
	let mut printed: size_t;

	unsafe {
		strarray__scnprintf(
			&raw const strarray__socket_families,
			family.as_mut_ptr(),
			core::mem::size_of_val(&family),
			c"%d".as_ptr(),
			(*arg).show_string_prefix,
			(*sa).sa_family as c_int,
		);
		printed = scnprintf(bf, size, c"{ .family: %s".as_ptr(), family.as_ptr());
	}

	if unsafe { (*sa).sa_family as usize } < af_scnprintfs.len() {
		if let Some(af_scnprintf) = af_scnprintfs[unsafe { (*sa).sa_family as usize }] {
			printed += unsafe { af_scnprintf(sa, bf.add(printed), size.wrapping_sub(printed)) };
		}
	}

	printed
		+ unsafe {
			scnprintf(
				bf.add(printed),
				size.wrapping_sub(printed),
				c" }".as_ptr(),
			)
		}
}

#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_sockaddr(
	bf: *mut c_char,
	size: size_t,
	arg: *mut syscall_arg,
) -> size_t {
	if unsafe { !(*arg).augmented.args.is_null() } {
		return unsafe { syscall_arg__scnprintf_augmented_sockaddr(arg, bf, size) };
	}

	unsafe { scnprintf(bf, size, c"%#lx".as_ptr(), (*arg).val) }
}
