// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *      names.c  --  USB name database manipulation routines
 *
 *      Copyright (C) 1999, 2000  Thomas Sailer (sailer@ife.ee.ethz.ch)
 *
 *	Copyright (C) 2005 Takahiro Hirofuchi
 *		- names_deinit() is added.
 */

use std::ffi::c_void;
use std::os::raw::{c_char, c_int, c_ulong};
use std::ptr;

type size_t = usize;
type u_int8_t = u8;
type u_int16_t = u16;

#[repr(C)]
pub struct FILE {
	_private: [u8; 0],
}

unsafe extern "C" {
	static mut errno: c_int;

	fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
	fn free(ptr: *mut c_void);
	fn strlen(s: *const c_char) -> size_t;
	fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
	fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
	fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
	fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
	fn fclose(stream: *mut FILE) -> c_int;
	fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
	fn isspace(c: c_int) -> c_int;
	fn isxdigit(c: c_int) -> c_int;

	fn err(fmt: *const c_char, ...);
	fn dbg(fmt: *const c_char, ...);
}

#[repr(C)]
struct vendor {
	next: *mut vendor,
	vendorid: u_int16_t,
	name: [c_char; 1],
}

#[repr(C)]
struct product {
	next: *mut product,
	vendorid: u_int16_t,
	productid: u_int16_t,
	name: [c_char; 1],
}

#[repr(C)]
struct class {
	next: *mut class,
	classid: u_int8_t,
	name: [c_char; 1],
}

#[repr(C)]
struct subclass {
	next: *mut subclass,
	classid: u_int8_t,
	subclassid: u_int8_t,
	name: [c_char; 1],
}

#[repr(C)]
struct protocol {
	next: *mut protocol,
	classid: u_int8_t,
	subclassid: u_int8_t,
	protocolid: u_int8_t,
	name: [c_char; 1],
}

#[repr(C)]
struct genericstrtable {
	next: *mut genericstrtable,
	num: u32,
	name: [c_char; 1],
}

const HASH1: u32 = 0x10;
const HASH2: u32 = 0x02;
const HASHSZ: usize = 16;

unsafe fn hashnum(mut num: u32) -> u32 {
	let mut mask1: u32 = HASH1 << 27;
	let mut mask2: u32 = HASH2 << 27;

	while mask1 >= HASH1 {
		if (num & mask1) != 0 {
			num ^= mask2;
		}
		mask1 >>= 1;
		mask2 >>= 1;
	}
	num & (HASHSZ as u32 - 1)
}

static mut vendors: [*mut vendor; HASHSZ] = [ptr::null_mut(); HASHSZ];
static mut products: [*mut product; HASHSZ] = [ptr::null_mut(); HASHSZ];
static mut classes: [*mut class; HASHSZ] = [ptr::null_mut(); HASHSZ];
static mut subclasses: [*mut subclass; HASHSZ] = [ptr::null_mut(); HASHSZ];
static mut protocols: [*mut protocol; HASHSZ] = [ptr::null_mut(); HASHSZ];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn names_vendor(vendorid: u_int16_t) -> *const c_char {
	let mut v: *mut vendor;

	v = vendors[hashnum(vendorid as u32) as usize];
	while !v.is_null() {
		if (*v).vendorid == vendorid {
			return (*v).name.as_ptr();
		}
		v = (*v).next;
	}
	ptr::null()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn names_product(vendorid: u_int16_t, productid: u_int16_t) -> *const c_char {
	let mut p: *mut product;

	p = products[hashnum(((vendorid as u32) << 16) | productid as u32) as usize];
	while !p.is_null() {
		if (*p).vendorid == vendorid && (*p).productid == productid {
			return (*p).name.as_ptr();
		}
		p = (*p).next;
	}
	ptr::null()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn names_class(classid: u_int8_t) -> *const c_char {
	let mut c: *mut class;

	c = classes[hashnum(classid as u32) as usize];
	while !c.is_null() {
		if (*c).classid == classid {
			return (*c).name.as_ptr();
		}
		c = (*c).next;
	}
	ptr::null()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn names_subclass(classid: u_int8_t, subclassid: u_int8_t) -> *const c_char {
	let mut s: *mut subclass;

	s = subclasses[hashnum(((classid as u32) << 8) | subclassid as u32) as usize];
	while !s.is_null() {
		if (*s).classid == classid && (*s).subclassid == subclassid {
			return (*s).name.as_ptr();
		}
		s = (*s).next;
	}
	ptr::null()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn names_protocol(
	classid: u_int8_t,
	subclassid: u_int8_t,
	protocolid: u_int8_t,
) -> *const c_char {
	let mut p: *mut protocol;

	p = protocols[hashnum(((classid as u32) << 16) | ((subclassid as u32) << 8) | protocolid as u32)
		as usize];
	while !p.is_null() {
		if (*p).classid == classid && (*p).subclassid == subclassid && (*p).protocolid == protocolid {
			return (*p).name.as_ptr();
		}
		p = (*p).next;
	}
	ptr::null()
}

/* add a cleanup function by takahiro */
#[repr(C)]
struct pool {
	next: *mut pool,
	mem: *mut c_void,
}

static mut pool_head: *mut pool = ptr::null_mut();

unsafe fn my_malloc(size: size_t) -> *mut c_void {
	let mut p: *mut pool;

	p = calloc(1, std::mem::size_of::<pool>()) as *mut pool;
	if p.is_null() {
		return ptr::null_mut();
	}

	(*p).mem = calloc(1, size);
	if (*p).mem.is_null() {
		free(p as *mut c_void);
		return ptr::null_mut();
	}

	(*p).next = pool_head;
	pool_head = p;

	(*p).mem
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn names_free() {
	let mut pool: *mut pool;

	if pool_head.is_null() {
		return;
	}

	pool = pool_head;
	while !pool.is_null() {
		let tmp: *mut pool;

		if !(*pool).mem.is_null() {
			free((*pool).mem);
		}

		tmp = pool;
		pool = (*pool).next;
		free(tmp as *mut c_void);
	}
}

unsafe fn new_vendor(name: *const c_char, vendorid: u_int16_t) -> c_int {
	let mut v: *mut vendor;
	let h: u32 = hashnum(vendorid as u32);

	v = vendors[h as usize];
	while !v.is_null() {
		if (*v).vendorid == vendorid {
			return -1;
		}
		v = (*v).next;
	}
	v = my_malloc(std::mem::size_of::<vendor>() + strlen(name)) as *mut vendor;
	if v.is_null() {
		return -1;
	}
	strcpy((*v).name.as_mut_ptr(), name);
	(*v).vendorid = vendorid;
	(*v).next = vendors[h as usize];
	vendors[h as usize] = v;
	0
}

unsafe fn new_product(name: *const c_char, vendorid: u_int16_t, productid: u_int16_t) -> c_int {
	let mut p: *mut product;
	let h: u32 = hashnum(((vendorid as u32) << 16) | productid as u32);

	p = products[h as usize];
	while !p.is_null() {
		if (*p).vendorid == vendorid && (*p).productid == productid {
			return -1;
		}
		p = (*p).next;
	}
	p = my_malloc(std::mem::size_of::<product>() + strlen(name)) as *mut product;
	if p.is_null() {
		return -1;
	}
	strcpy((*p).name.as_mut_ptr(), name);
	(*p).vendorid = vendorid;
	(*p).productid = productid;
	(*p).next = products[h as usize];
	products[h as usize] = p;
	0
}

unsafe fn new_class(name: *const c_char, classid: u_int8_t) -> c_int {
	let mut c: *mut class;
	let h: u32 = hashnum(classid as u32);

	c = classes[h as usize];
	while !c.is_null() {
		if (*c).classid == classid {
			return -1;
		}
		c = (*c).next;
	}
	c = my_malloc(std::mem::size_of::<class>() + strlen(name)) as *mut class;
	if c.is_null() {
		return -1;
	}
	strcpy((*c).name.as_mut_ptr(), name);
	(*c).classid = classid;
	(*c).next = classes[h as usize];
	classes[h as usize] = c;
	0
}

unsafe fn new_subclass(name: *const c_char, classid: u_int8_t, subclassid: u_int8_t) -> c_int {
	let mut s: *mut subclass;
	let h: u32 = hashnum(((classid as u32) << 8) | subclassid as u32);

	s = subclasses[h as usize];
	while !s.is_null() {
		if (*s).classid == classid && (*s).subclassid == subclassid {
			return -1;
		}
		s = (*s).next;
	}
	s = my_malloc(std::mem::size_of::<subclass>() + strlen(name)) as *mut subclass;
	if s.is_null() {
		return -1;
	}
	strcpy((*s).name.as_mut_ptr(), name);
	(*s).classid = classid;
	(*s).subclassid = subclassid;
	(*s).next = subclasses[h as usize];
	subclasses[h as usize] = s;
	0
}

unsafe fn new_protocol(
	name: *const c_char,
	classid: u_int8_t,
	subclassid: u_int8_t,
	protocolid: u_int8_t,
) -> c_int {
	let mut p: *mut protocol;
	let h: u32 = hashnum(((classid as u32) << 16) | ((subclassid as u32) << 8) | protocolid as u32);

	p = protocols[h as usize];
	while !p.is_null() {
		if (*p).classid == classid && (*p).subclassid == subclassid && (*p).protocolid == protocolid {
			return -1;
		}
		p = (*p).next;
	}
	p = my_malloc(std::mem::size_of::<protocol>() + strlen(name)) as *mut protocol;
	if p.is_null() {
		return -1;
	}
	strcpy((*p).name.as_mut_ptr(), name);
	(*p).classid = classid;
	(*p).subclassid = subclassid;
	(*p).protocolid = protocolid;
	(*p).next = protocols[h as usize];
	protocols[h as usize] = p;
	0
}

unsafe fn parse(f: *mut FILE) {
	let mut buf: [c_char; 512] = [0; 512];
	let mut cp: *mut c_char;
	let mut linectr: u32 = 0;
	let mut lastvendor: c_int = -1;
	let mut lastclass: c_int = -1;
	let mut lastsubclass: c_int = -1;
	let mut lasthut: c_int = -1;
	let mut lastlang: c_int = -1;
	let mut u: u32;

	while !fgets(buf.as_mut_ptr(), std::mem::size_of_val(&buf) as c_int, f).is_null() {
		linectr += 1;
		/* remove line ends */
		cp = strchr(buf.as_ptr(), '\r' as c_int);
		if !cp.is_null() {
			*cp = 0;
		}
		cp = strchr(buf.as_ptr(), '\n' as c_int);
		if !cp.is_null() {
			*cp = 0;
		}
		if buf[0] == '#' as c_char || buf[0] == 0 {
			continue;
		}
		cp = buf.as_mut_ptr();
		if buf[0] == 'P' as c_char
			&& buf[1] == 'H' as c_char
			&& buf[2] == 'Y' as c_char
			&& buf[3] == 'S' as c_char
			&& buf[4] == 'D' as c_char
			&& buf[5] == 'E' as c_char
			&& buf[6] == 'S' as c_char
			&& /*isspace(buf[7])*/ buf[7] == ' ' as c_char
		{
			continue;
		}
		if buf[0] == 'P' as c_char
			&& buf[1] == 'H' as c_char
			&& buf[2] == 'Y' as c_char
			&& /*isspace(buf[3])*/ buf[3] == ' ' as c_char
		{
			continue;
		}
		if buf[0] == 'B' as c_char
			&& buf[1] == 'I' as c_char
			&& buf[2] == 'A' as c_char
			&& buf[3] == 'S' as c_char
			&& /*isspace(buf[4])*/ buf[4] == ' ' as c_char
		{
			continue;
		}
		if buf[0] == 'L' as c_char && /*isspace(buf[1])*/ buf[1] == ' ' as c_char {
			lasthut = -1;
			lastclass = -1;
			lastvendor = -1;
			lastsubclass = -1;
			/*
			 * set 1 as pseudo-id to indicate that the parser is
			 * in a `L' section.
			 */
			lastlang = 1;
			continue;
		}
		if buf[0] == 'C' as c_char && /*isspace(buf[1])*/ buf[1] == ' ' as c_char {
			/* class spec */
			cp = buf.as_mut_ptr().add(2);
			while isspace(*cp as c_int) != 0 {
				cp = cp.add(1);
			}
			if isxdigit(*cp as c_int) == 0 {
				err(b"Invalid class spec at line %u\0".as_ptr() as *const c_char, linectr);
				continue;
			}
			u = strtoul(cp, &mut cp, 16) as u32;
			while isspace(*cp as c_int) != 0 {
				cp = cp.add(1);
			}
			if *cp == 0 {
				err(b"Invalid class spec at line %u\0".as_ptr() as *const c_char, linectr);
				continue;
			}
			if new_class(cp, u as u_int8_t) != 0 {
				err(
					b"Duplicate class spec at line %u class %04x %s\0".as_ptr() as *const c_char,
					linectr,
					u,
					cp,
				);
			}
			dbg(b"line %5u class %02x %s\0".as_ptr() as *const c_char, linectr, u, cp);
			lasthut = -1;
			lastlang = -1;
			lastvendor = -1;
			lastsubclass = -1;
			lastclass = u as c_int;
			continue;
		}
		if buf[0] == 'A' as c_char && buf[1] == 'T' as c_char && isspace(buf[2] as c_int) != 0 {
			/* audio terminal type spec */
			continue;
		}
		if buf[0] == 'H' as c_char
			&& buf[1] == 'C' as c_char
			&& buf[2] == 'C' as c_char
			&& isspace(buf[3] as c_int) != 0
		{
			/* HID Descriptor bCountryCode */
			continue;
		}
		if isxdigit(*cp as c_int) != 0 {
			/* vendor */
			u = strtoul(cp, &mut cp, 16) as u32;
			while isspace(*cp as c_int) != 0 {
				cp = cp.add(1);
			}
			if *cp == 0 {
				err(b"Invalid vendor spec at line %u\0".as_ptr() as *const c_char, linectr);
				continue;
			}
			if new_vendor(cp, u as u_int16_t) != 0 {
				err(
					b"Duplicate vendor spec at line %u vendor %04x %s\0".as_ptr() as *const c_char,
					linectr,
					u,
					cp,
				);
			}
			dbg(b"line %5u vendor %04x %s\0".as_ptr() as *const c_char, linectr, u, cp);
			lastvendor = u as c_int;
			lasthut = -1;
			lastlang = -1;
			lastclass = -1;
			lastsubclass = -1;
			continue;
		}
		if buf[0] == '\t' as c_char && isxdigit(buf[1] as c_int) != 0 {
			/* product or subclass spec */
			u = strtoul(buf.as_mut_ptr().add(1), &mut cp, 16) as u32;
			while isspace(*cp as c_int) != 0 {
				cp = cp.add(1);
			}
			if *cp == 0 {
				err(b"Invalid product/subclass spec at line %u\0".as_ptr() as *const c_char, linectr);
				continue;
			}
			if lastvendor != -1 {
				if new_product(cp, lastvendor as u_int16_t, u as u_int16_t) != 0 {
					err(
						b"Duplicate product spec at line %u product %04x:%04x %s\0".as_ptr()
							as *const c_char,
						linectr,
						lastvendor,
						u,
						cp,
					);
				}
				dbg(
					b"line %5u product %04x:%04x %s\0".as_ptr() as *const c_char,
					linectr,
					lastvendor,
					u,
					cp,
				);
				continue;
			}
			if lastclass != -1 {
				if new_subclass(cp, lastclass as u_int8_t, u as u_int8_t) != 0 {
					err(
						b"Duplicate subclass spec at line %u class %02x:%02x %s\0".as_ptr()
							as *const c_char,
						linectr,
						lastclass,
						u,
						cp,
					);
				}
				dbg(
					b"line %5u subclass %02x:%02x %s\0".as_ptr() as *const c_char,
					linectr,
					lastclass,
					u,
					cp,
				);
				lastsubclass = u as c_int;
				continue;
			}
			if lasthut != -1 {
				/* do not store hut */
				continue;
			}
			if lastlang != -1 {
				/* do not store langid */
				continue;
			}
			err(
				b"Product/Subclass spec without prior Vendor/Class spec at line %u\0".as_ptr()
					as *const c_char,
				linectr,
			);
			continue;
		}
		if buf[0] == '\t' as c_char && buf[1] == '\t' as c_char && isxdigit(buf[2] as c_int) != 0 {
			/* protocol spec */
			u = strtoul(buf.as_mut_ptr().add(2), &mut cp, 16) as u32;
			while isspace(*cp as c_int) != 0 {
				cp = cp.add(1);
			}
			if *cp == 0 {
				err(b"Invalid protocol spec at line %u\0".as_ptr() as *const c_char, linectr);
				continue;
			}
			if lastclass != -1 && lastsubclass != -1 {
				if new_protocol(cp, lastclass as u_int8_t, lastsubclass as u_int8_t, u as u_int8_t) != 0 {
					err(
						b"Duplicate protocol spec at line %u class %02x:%02x:%02x %s\0".as_ptr()
							as *const c_char,
						linectr,
						lastclass,
						lastsubclass,
						u,
						cp,
					);
				}
				dbg(
					b"line %5u protocol %02x:%02x:%02x %s\0".as_ptr() as *const c_char,
					linectr,
					lastclass,
					lastsubclass,
					u,
					cp,
				);
				continue;
			}
			err(
				b"Protocol spec without prior Class and Subclass spec at line %u\0".as_ptr()
					as *const c_char,
				linectr,
			);
			continue;
		}
		if buf[0] == 'H' as c_char
			&& buf[1] == 'I' as c_char
			&& buf[2] == 'D' as c_char
			&& /*isspace(buf[3])*/ buf[3] == ' ' as c_char
		{
			continue;
		}
		if buf[0] == 'H' as c_char
			&& buf[1] == 'U' as c_char
			&& buf[2] == 'T' as c_char
			&& /*isspace(buf[3])*/ buf[3] == ' ' as c_char
		{
			lastlang = -1;
			lastclass = -1;
			lastvendor = -1;
			lastsubclass = -1;
			/*
			 * set 1 as pseudo-id to indicate that the parser is
			 * in a `HUT' section.
			 */
			lasthut = 1;
			continue;
		}
		if buf[0] == 'R' as c_char && buf[1] == ' ' as c_char {
			continue;
		}

		if buf[0] == 'V' as c_char && buf[1] == 'T' as c_char {
			continue;
		}

		err(b"Unknown line at line %u\0".as_ptr() as *const c_char, linectr);
	}
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn names_init(n: *mut c_char) -> c_int {
	let f: *mut FILE;

	f = fopen(n, b"r\0".as_ptr() as *const c_char);
	if f.is_null() {
		return errno;
	}

	parse(f);
	fclose(f);
	0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
