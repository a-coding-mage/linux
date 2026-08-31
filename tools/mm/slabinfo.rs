// SPDX-License-Identifier: GPL-2.0
/*
 * Slabinfo: Tool to get reports about slabs
 *
 * (C) 2007 sgi, Christoph Lameter
 * (C) 2011 Linux Foundation, Christoph Lameter
 *
 * Compile with:
 *
 * gcc -o slabinfo slabinfo.c
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(static_mut_refs)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_ulonglong, c_void};

const MAX_SLABS: usize = 2000;
const MAX_ALIASES: usize = 500;
const MAX_NODES: usize = 1024;
const EXIT_FAILURE: c_int = 1;
const DT_LNK: c_int = 10;
const DT_DIR: c_int = 4;
const REG_ICASE: c_int = 1;
const REG_NOSUB: c_int = 8;
const no_argument: c_int = 0;
const required_argument: c_int = 1;
const optional_argument: c_int = 2;
const EACCES: c_int = 13;

#[repr(C)]
pub struct FILE {
	_private: [u8; 0],
}

#[repr(C)]
pub struct DIR {
	_private: [u8; 0],
}

#[repr(C)]
pub struct regex_t {
	_private: [u8; 64],
}

#[repr(C)]
pub struct dirent {
	pub d_ino: c_ulong,
	pub d_off: c_long,
	pub d_reclen: u16,
	pub d_type: u8,
	pub d_name: [c_char; 256],
}

#[repr(C)]
pub struct option {
	pub name: *const c_char,
	pub has_arg: c_int,
	pub flag: *mut c_int,
	pub val: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slabinfo {
	pub name: *mut c_char,
	pub alias: c_int,
	pub refs: c_int,
	pub aliases: c_int,
	pub align: c_int,
	pub cache_dma: c_int,
	pub cpu_slabs: c_int,
	pub destroy_by_rcu: c_int,
	pub hwcache_align: c_uint,
	pub object_size: c_uint,
	pub objs_per_slab: c_uint,
	pub sanity_checks: c_uint,
	pub slab_size: c_uint,
	pub store_user: c_uint,
	pub trace: c_uint,
	pub order: c_int,
	pub poison: c_int,
	pub reclaim_account: c_int,
	pub red_zone: c_int,
	pub partial: c_ulong,
	pub objects: c_ulong,
	pub slabs: c_ulong,
	pub objects_partial: c_ulong,
	pub total_objects: c_ulong,
	pub alloc_fastpath: c_ulong,
	pub alloc_slowpath: c_ulong,
	pub free_fastpath: c_ulong,
	pub free_slowpath: c_ulong,
	pub free_frozen: c_ulong,
	pub free_add_partial: c_ulong,
	pub free_remove_partial: c_ulong,
	pub alloc_from_partial: c_ulong,
	pub alloc_slab: c_ulong,
	pub free_slab: c_ulong,
	pub alloc_refill: c_ulong,
	pub cpuslab_flush: c_ulong,
	pub deactivate_full: c_ulong,
	pub deactivate_empty: c_ulong,
	pub deactivate_to_head: c_ulong,
	pub deactivate_to_tail: c_ulong,
	pub deactivate_remote_frees: c_ulong,
	pub order_fallback: c_ulong,
	pub cmpxchg_double_cpu_fail: c_ulong,
	pub cmpxchg_double_fail: c_ulong,
	pub alloc_node_mismatch: c_ulong,
	pub deactivate_bypass: c_ulong,
	pub cpu_partial_alloc: c_ulong,
	pub cpu_partial_free: c_ulong,
	pub numa: [c_int; MAX_NODES],
	pub numa_partial: [c_int; MAX_NODES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aliasinfo {
	pub name: *mut c_char,
	pub ref_: *mut c_char,
	pub slab: *mut slabinfo,
}

const ZERO_SLABINFO: slabinfo = slabinfo {
	name: core::ptr::null_mut(), alias: 0, refs: 0, aliases: 0, align: 0, cache_dma: 0,
	cpu_slabs: 0, destroy_by_rcu: 0, hwcache_align: 0, object_size: 0, objs_per_slab: 0,
	sanity_checks: 0, slab_size: 0, store_user: 0, trace: 0, order: 0, poison: 0,
	reclaim_account: 0, red_zone: 0, partial: 0, objects: 0, slabs: 0, objects_partial: 0,
	total_objects: 0, alloc_fastpath: 0, alloc_slowpath: 0, free_fastpath: 0,
	free_slowpath: 0, free_frozen: 0, free_add_partial: 0, free_remove_partial: 0,
	alloc_from_partial: 0, alloc_slab: 0, free_slab: 0, alloc_refill: 0, cpuslab_flush: 0,
	deactivate_full: 0, deactivate_empty: 0, deactivate_to_head: 0, deactivate_to_tail: 0,
	deactivate_remote_frees: 0, order_fallback: 0, cmpxchg_double_cpu_fail: 0,
	cmpxchg_double_fail: 0, alloc_node_mismatch: 0, deactivate_bypass: 0,
	cpu_partial_alloc: 0, cpu_partial_free: 0, numa: [0; MAX_NODES], numa_partial: [0; MAX_NODES],
};
const ZERO_ALIASINFO: aliasinfo = aliasinfo { name: core::ptr::null_mut(), ref_: core::ptr::null_mut(), slab: core::ptr::null_mut() };

static mut slabinfo: [slabinfo; MAX_SLABS] = [ZERO_SLABINFO; MAX_SLABS];
static mut aliasinfo: [aliasinfo; MAX_ALIASES] = [ZERO_ALIASINFO; MAX_ALIASES];

static mut slabs: c_int = 0;
static mut actual_slabs: c_int = 0;
static mut aliases: c_int = 0;
static mut alias_targets: c_int = 0;
static mut highest_node: c_int = 0;
static mut buffer: [c_char; 4096] = [0; 4096];

static mut show_empty: c_int = 0;
static mut show_report: c_int = 0;
static mut show_alias: c_int = 0;
static mut show_slab: c_int = 0;
static mut skip_zero: c_int = 1;
static mut show_numa: c_int = 0;
static mut show_track: c_int = 0;
static mut show_first_alias: c_int = 0;
static mut validate: c_int = 0;
static mut shrink: c_int = 0;
static mut show_inverted: c_int = 0;
static mut show_single_ref: c_int = 0;
static mut show_totals: c_int = 0;
static mut sort_size: c_int = 0;
static mut sort_active: c_int = 0;
static mut set_debug: c_int = 0;
static mut show_ops: c_int = 0;
static mut sort_partial: c_int = 0;
static mut show_activity: c_int = 0;
static mut output_lines: c_int = -1;
static mut sort_loss: c_int = 0;
static mut extended_totals: c_int = 0;
static mut show_bytes: c_int = 0;
static mut unreclaim_only: c_int = 0;

/* Debug options */
static mut sanity: c_int = 0;
static mut redzone: c_int = 0;
static mut poison: c_int = 0;
static mut tracking: c_int = 0;
static mut tracing: c_int = 0;

static mut page_size: c_int = 0;
static mut pattern: regex_t = regex_t { _private: [0; 64] };

unsafe extern "C" {
	static mut stderr: *mut FILE;
	static mut errno: c_int;
	static mut optarg: *mut c_char;
	static mut optind: c_int;
	static mut optopt: c_int;
	fn printf(fmt: *const c_char, ...) -> c_int;
	fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
	fn vfprintf(stream: *mut FILE, fmt: *const c_char, ap: *mut c_void) -> c_int;
	fn sprintf(s: *mut c_char, fmt: *const c_char, ...) -> c_int;
	fn snprintf(s: *mut c_char, n: usize, fmt: *const c_char, ...) -> c_int;
	fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
	fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
	fn fread(ptr: *mut c_void, size: usize, nmemb: usize, stream: *mut FILE) -> usize;
	fn fclose(stream: *mut FILE) -> c_int;
	fn strlen(s: *const c_char) -> usize;
	fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
	fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
	fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
	fn strerror(errnum: c_int) -> *mut c_char;
	fn atol(nptr: *const c_char) -> c_long;
	fn atoi(nptr: *const c_char) -> c_int;
	fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
	fn strdup(s: *const c_char) -> *mut c_char;
	fn free(ptr: *mut c_void);
	fn exit(status: c_int) -> !;
	fn memmove(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
	fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
	fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
	fn chdir(path: *const c_char) -> c_int;
	fn opendir(name: *const c_char) -> *mut DIR;
	fn readdir(dirp: *mut DIR) -> *mut dirent;
	fn closedir(dirp: *mut DIR) -> c_int;
	fn readlink(path: *const c_char, buf: *mut c_char, bufsiz: usize) -> isize;
	fn getpagesize() -> c_int;
	fn getopt_long(argc: c_int, argv: *const *mut c_char, optstring: *const c_char, longopts: *const option, longindex: *mut c_int) -> c_int;
	fn regcomp(preg: *mut regex_t, regex: *const c_char, cflags: c_int) -> c_int;
	fn regexec(preg: *const regex_t, string: *const c_char, nmatch: usize, pmatch: *mut c_void, eflags: c_int) -> c_int;
}

macro_rules! c {
	($s:literal) => {
		concat!($s, "\0").as_ptr() as *const c_char
	};
}

unsafe fn fatal(x: *const c_char) -> ! {
	fprintf(stderr, c!("%s"), x);
	exit(EXIT_FAILURE);
}

unsafe fn fatal1(fmt: *const c_char, a: *const c_char) -> ! {
	fprintf(stderr, fmt, a);
	exit(EXIT_FAILURE);
}

unsafe fn fatal2(fmt: *const c_char, a: *const c_char, b: *const c_char) -> ! {
	fprintf(stderr, fmt, a, b);
	exit(EXIT_FAILURE);
}

unsafe fn fatal3(fmt: *const c_char, a: *const c_char, b: *const c_char, c_: c_int) -> ! {
	fprintf(stderr, fmt, a, b, c_);
	exit(EXIT_FAILURE);
}

unsafe fn usage() {
	printf(c!("slabinfo 4/15/2011. (c) 2007 sgi/(c) 2011 Linux Foundation.\n\nslabinfo [-aABDefhilLnoPrsStTUvXz1] [N=K] [-dafzput] [slab-regexp]\n-a|--aliases           Show aliases\n-A|--activity          Most active slabs first\n-B|--Bytes             Show size in bytes\n-D|--display-active    Switch line format to activity\n-e|--empty             Show empty slabs\n-f|--first-alias       Show first alias\n-h|--help              Show usage information\n-i|--inverted          Inverted list\n-l|--slabs             Show slabs\n-L|--Loss              Sort by loss\n-n|--numa              Show NUMA information\n-N|--lines=K           Show the first K slabs\n-o|--ops               Show kmem_cache_ops\n-P|--partial           Sort by number of partial slabs\n-r|--report            Detailed report on single slabs\n-s|--shrink            Shrink slabs\n-S|--Size              Sort by size\n-t|--tracking          Show alloc/free information\n-T|--Totals            Show summary information\n-U|--Unreclaim         Show unreclaimable slabs only\n-v|--validate          Validate slabs\n-X|--Xtotals           Show extended summary information\n-z|--zero              Include empty slabs\n-1|--1ref              Single reference\n\n-d  | --debug          Switch off all debug options\n-da | --debug=a        Switch on all debug options (--debug=FZPU)\n\n-d[afzput] | --debug=[afzput]\n    f | F              Sanity Checks (SLAB_CONSISTENCY_CHECKS)\n    z | Z              Redzoning\n    p | P              Poisoning\n    u | U              Tracking\n    t | T              Tracing\n\nSorting options (--Loss, --Size, --Partial) are mutually exclusive\n"));
}

unsafe fn read_obj(name: *const c_char) -> c_ulong {
	let mut len: usize;
	let f = fopen(name, c!("r"));
	if f.is_null() {
		buffer[0] = 0;
		if errno == EACCES {
			fatal1(c!("%s, Try using superuser\n"), strerror(errno));
		}
	} else {
		if fgets(buffer.as_mut_ptr(), buffer.len() as c_int, f).is_null() {
			buffer[0] = 0;
		}
		fclose(f);
		len = strlen(buffer.as_ptr());
		if len > 0 && buffer[len - 1] == b'\n' as c_char {
			buffer[len - 1] = 0;
		}
	}
	strlen(buffer.as_ptr()) as c_ulong
}

/*
 * Get the contents of an attribute
 */
unsafe fn get_obj(name: *const c_char) -> c_ulong {
	if read_obj(name) == 0 { return 0; }
	atol(buffer.as_ptr()) as c_ulong
}

unsafe fn get_obj_and_str(name: *const c_char, x: *mut *mut c_char) -> c_ulong {
	let mut p: *mut c_char = core::ptr::null_mut();
	*x = core::ptr::null_mut();
	if read_obj(name) == 0 { return 0; }
	let result = strtoul(buffer.as_ptr(), &mut p, 10);
	while *p == b' ' as c_char { p = p.add(1); }
	if *p != 0 { *x = strdup(p); }
	result
}

unsafe fn set_obj(s: *mut slabinfo, name: *const c_char, n: c_int) {
	let mut x = [0 as c_char; 100];
	snprintf(x.as_mut_ptr(), 100, c!("%s/%s"), (*s).name, name);
	let f = fopen(x.as_ptr(), c!("w"));
	if f.is_null() { fatal1(c!("Cannot write to %s\n"), x.as_ptr()); }
	fprintf(f, c!("%d\n"), n);
	fclose(f);
}

unsafe fn read_slab_obj(s: *mut slabinfo, name: *const c_char) -> c_ulong {
	let mut x = [0 as c_char; 100];
	snprintf(x.as_mut_ptr(), 100, c!("%s/%s"), (*s).name, name);
	let f = fopen(x.as_ptr(), c!("r"));
	if f.is_null() {
		buffer[0] = 0;
		0
	} else {
		let l = fread(buffer.as_mut_ptr() as *mut c_void, 1, buffer.len(), f);
		buffer[l] = 0;
		fclose(f);
		l as c_ulong
	}
}

unsafe fn read_debug_slab_obj(s: *mut slabinfo, name: *const c_char) -> c_ulong {
	let mut x = [0 as c_char; 128];
	snprintf(x.as_mut_ptr(), 128, c!("/sys/kernel/debug/slab/%s/%s"), (*s).name, name);
	let f = fopen(x.as_ptr(), c!("r"));
	if f.is_null() {
		buffer[0] = 0;
		0
	} else {
		let l = fread(buffer.as_mut_ptr() as *mut c_void, 1, buffer.len(), f);
		buffer[l] = 0;
		fclose(f);
		l as c_ulong
	}
}

/*
 * Put a size string together
 */
unsafe fn store_size(buf: *mut c_char, mut value: c_ulong) -> c_int {
	let mut divisor: c_ulong = 1;
	let mut trailer: c_char = 0;
	if show_bytes == 0 {
		if value > 1000000000 { divisor = 100000000; trailer = b'G' as c_char; }
		else if value > 1000000 { divisor = 100000; trailer = b'M' as c_char; }
		else if value > 1000 { divisor = 100; trailer = b'K' as c_char; }
	}
	value /= divisor;
	let mut n = sprintf(buf, c!("%ld"), value as c_long);
	if trailer != 0 {
		*buf.add(n as usize) = trailer;
		n += 1;
		*buf.add(n as usize) = 0;
	}
	if divisor != 1 {
		memmove(buf.add((n - 2) as usize) as *mut c_void, buf.add((n - 3) as usize) as *const c_void, 4);
		*buf.add((n - 2) as usize) = b'.' as c_char;
		n += 1;
	}
	n
}

unsafe fn decode_numa_list(numa: *mut c_int, mut t: *mut c_char) {
	memset(numa as *mut c_void, 0, MAX_NODES * core::mem::size_of::<c_int>());
	if t.is_null() { return; }
	while *t == b'N' as c_char {
		t = t.add(1);
		let node = strtoul(t, &mut t, 10) as c_int;
		if *t == b'=' as c_char {
			t = t.add(1);
			let nr = strtoul(t, &mut t, 10) as c_int;
			*numa.add(node as usize) = nr;
			if node > highest_node { highest_node = node; }
		}
		while *t == b' ' as c_char { t = t.add(1); }
	}
}

unsafe fn slab_validate(s: *mut slabinfo) {
	if strcmp((*s).name, c!("*")) == 0 { return; }
	set_obj(s, c!("validate"), 1);
}

unsafe fn slab_shrink(s: *mut slabinfo) {
	if strcmp((*s).name, c!("*")) == 0 { return; }
	set_obj(s, c!("shrink"), 1);
}

static mut line: c_int = 0;

unsafe fn first_line() {
	if show_activity != 0 {
		printf(c!("Name                   Objects      Alloc       Free   %%Fast Fallb O CmpX   UL\n"));
	} else {
		printf(c!("Name                   Objects Objsize           %s Slabs/Part/Cpu  O/S O %%Fr %%Ef Flg\n"), if sort_loss != 0 { c!(" Loss") } else { c!("Space") });
	}
}

/*
 * Find the shortest alias of a slab
 */
unsafe fn find_one_alias(find: *mut slabinfo) -> *mut aliasinfo {
	let mut a = aliasinfo.as_mut_ptr();
	let end = a.add(aliases as usize);
	let mut best: *mut aliasinfo = core::ptr::null_mut();
	while a < end {
		if (*a).slab == find && (best.is_null() || strlen((*best).name) < strlen((*a).name)) {
			best = a;
			if strncmp((*a).name, c!("kmall"), 5) == 0 { return best; }
		}
		a = a.add(1);
	}
	best
}

unsafe fn slab_size(s: *mut slabinfo) -> c_ulong {
	(*s).slabs * ((page_size as c_ulong) << (*s).order)
}

unsafe fn slab_activity(s: *mut slabinfo) -> c_ulong {
	(*s).alloc_fastpath + (*s).free_fastpath + (*s).alloc_slowpath + (*s).free_slowpath
}

unsafe fn slab_waste(s: *mut slabinfo) -> c_ulong {
	slab_size(s) - (*s).objects * (*s).object_size as c_ulong
}

unsafe fn slab_numa(s: *mut slabinfo, mode: c_int) {
	if strcmp((*s).name, c!("*")) == 0 { return; }
	if highest_node == 0 {
		printf(c!("\n%s: No NUMA information available.\n"), (*s).name);
		return;
	}
	if skip_zero != 0 && (*s).slabs == 0 { return; }
	if line == 0 {
		printf(c!("\n%-21s:"), if mode != 0 { c!("NUMA nodes") } else { c!("Slab") });
		for node in 0..=highest_node { printf(c!(" %4d"), node); }
		printf(c!("\n----------------------"));
		for _node in 0..=highest_node { printf(c!("-----")); }
		printf(c!("\n"));
	}
	printf(c!("%-21s "), if mode != 0 { c!("All slabs") } else { (*s).name });
	for node in 0..=highest_node {
		let mut b = [0 as c_char; 20];
		store_size(b.as_mut_ptr(), (*s).numa[node as usize] as c_ulong);
		printf(c!(" %4s"), b.as_ptr());
	}
	printf(c!("\n"));
	if mode != 0 {
		printf(c!("%-21s "), c!("Partial slabs"));
		for node in 0..=highest_node {
			let mut b = [0 as c_char; 20];
			store_size(b.as_mut_ptr(), (*s).numa_partial[node as usize] as c_ulong);
			printf(c!(" %4s"), b.as_ptr());
		}
		printf(c!("\n"));
	}
	line += 1;
}

unsafe fn show_tracking(s: *mut slabinfo) {
	printf(c!("\n%s: Kernel object allocation\n"), (*s).name);
	printf(c!("-----------------------------------------------------------------------\n"));
	if read_debug_slab_obj(s, c!("alloc_traces")) != 0 { printf(c!("%s"), buffer.as_ptr()); }
	else if read_slab_obj(s, c!("alloc_calls")) != 0 { printf(c!("%s"), buffer.as_ptr()); }
	else { printf(c!("No Data\n")); }
	printf(c!("\n%s: Kernel object freeing\n"), (*s).name);
	printf(c!("------------------------------------------------------------------------\n"));
	if read_debug_slab_obj(s, c!("free_traces")) != 0 { printf(c!("%s"), buffer.as_ptr()); }
	else if read_slab_obj(s, c!("free_calls")) != 0 { printf(c!("%s"), buffer.as_ptr()); }
	else { printf(c!("No Data\n")); }
}

unsafe fn ops(s: *mut slabinfo) {
	if strcmp((*s).name, c!("*")) == 0 { return; }
	if read_slab_obj(s, c!("ops")) != 0 {
		printf(c!("\n%s: kmem_cache operations\n"), (*s).name);
		printf(c!("--------------------------------------------\n"));
		printf(c!("%s"), buffer.as_ptr());
	} else {
		printf(c!("\n%s has no kmem_cache operations\n"), (*s).name);
	}
}

unsafe fn onoff(x: c_int) -> *const c_char {
	if x != 0 { c!("On ") } else { c!("Off") }
}

unsafe fn slab_stats(s: *mut slabinfo) {
	if (*s).alloc_slab == 0 { return; }
	let total_alloc = (*s).alloc_fastpath + (*s).alloc_slowpath;
	let total_free = (*s).free_fastpath + (*s).free_slowpath;
	if total_alloc == 0 { return; }
	printf(c!("\nSlab Perf Counter       Alloc     Free %%Al %%Fr\n"));
	printf(c!("--------------------------------------------------\n"));
	printf(c!("Fastpath             %8lu %8lu %3lu %3lu\n"), (*s).alloc_fastpath, (*s).free_fastpath, (*s).alloc_fastpath * 100 / total_alloc, if total_free != 0 { (*s).free_fastpath * 100 / total_free } else { 0 });
	printf(c!("Slowpath             %8lu %8lu %3lu %3lu\n"), total_alloc - (*s).alloc_fastpath, (*s).free_slowpath, (total_alloc - (*s).alloc_fastpath) * 100 / total_alloc, if total_free != 0 { (*s).free_slowpath * 100 / total_free } else { 0 });
	printf(c!("Page Alloc           %8lu %8lu %3lu %3lu\n"), (*s).alloc_slab, (*s).free_slab, (*s).alloc_slab * 100 / total_alloc, if total_free != 0 { (*s).free_slab * 100 / total_free } else { 0 });
	printf(c!("Add partial          %8lu %8lu %3lu %3lu\n"), (*s).deactivate_to_head + (*s).deactivate_to_tail, (*s).free_add_partial, ((*s).deactivate_to_head + (*s).deactivate_to_tail) * 100 / total_alloc, if total_free != 0 { (*s).free_add_partial * 100 / total_free } else { 0 });
	printf(c!("Remove partial       %8lu %8lu %3lu %3lu\n"), (*s).alloc_from_partial, (*s).free_remove_partial, (*s).alloc_from_partial * 100 / total_alloc, if total_free != 0 { (*s).free_remove_partial * 100 / total_free } else { 0 });
	printf(c!("Cpu partial list     %8lu %8lu %3lu %3lu\n"), (*s).cpu_partial_alloc, (*s).cpu_partial_free, (*s).cpu_partial_alloc * 100 / total_alloc, if total_free != 0 { (*s).cpu_partial_free * 100 / total_free } else { 0 });
	printf(c!("RemoteObj/SlabFrozen %8lu %8lu %3lu %3lu\n"), (*s).deactivate_remote_frees, (*s).free_frozen, (*s).deactivate_remote_frees * 100 / total_alloc, if total_free != 0 { (*s).free_frozen * 100 / total_free } else { 0 });
	printf(c!("Total                %8lu %8lu\n\n"), total_alloc, total_free);
	if (*s).cpuslab_flush != 0 { printf(c!("Flushes %8lu\n"), (*s).cpuslab_flush); }
	let total = (*s).deactivate_full + (*s).deactivate_empty + (*s).deactivate_to_head + (*s).deactivate_to_tail + (*s).deactivate_bypass;
	if total != 0 {
		printf(c!("\nSlab Deactivation             Occurrences %%\n"));
		printf(c!("-------------------------------------------------\n"));
		printf(c!("Slab full                     %7lu  %3lu%%\n"), (*s).deactivate_full, ((*s).deactivate_full * 100) / total);
		printf(c!("Slab empty                    %7lu  %3lu%%\n"), (*s).deactivate_empty, ((*s).deactivate_empty * 100) / total);
		printf(c!("Moved to head of partial list %7lu  %3lu%%\n"), (*s).deactivate_to_head, ((*s).deactivate_to_head * 100) / total);
		printf(c!("Moved to tail of partial list %7lu  %3lu%%\n"), (*s).deactivate_to_tail, ((*s).deactivate_to_tail * 100) / total);
		printf(c!("Deactivation bypass           %7lu  %3lu%%\n"), (*s).deactivate_bypass, ((*s).deactivate_bypass * 100) / total);
		printf(c!("Refilled from foreign frees   %7lu  %3lu%%\n"), (*s).alloc_refill, ((*s).alloc_refill * 100) / total);
		printf(c!("Node mismatch                 %7lu  %3lu%%\n"), (*s).alloc_node_mismatch, ((*s).alloc_node_mismatch * 100) / total);
	}
	if (*s).cmpxchg_double_fail != 0 || (*s).cmpxchg_double_cpu_fail != 0 {
		printf(c!("\nCmpxchg_double Looping\n------------------------\n"));
		printf(c!("Locked Cmpxchg Double redos   %lu\nUnlocked Cmpxchg Double redos %lu\n"), (*s).cmpxchg_double_fail, (*s).cmpxchg_double_cpu_fail);
	}
}

unsafe fn report(s: *mut slabinfo) {
	if strcmp((*s).name, c!("*")) == 0 { return; }
	printf(c!("\nSlabcache: %-15s  Aliases: %2d Order : %2d Objects: %lu\n"), (*s).name, (*s).aliases, (*s).order, (*s).objects);
	if (*s).hwcache_align != 0 { printf(c!("** Hardware cacheline aligned\n")); }
	if (*s).cache_dma != 0 { printf(c!("** Memory is allocated in a special DMA zone\n")); }
	if (*s).destroy_by_rcu != 0 { printf(c!("** Slabs are destroyed via RCU\n")); }
	if (*s).reclaim_account != 0 { printf(c!("** Reclaim accounting active\n")); }
	printf(c!("\nSizes (bytes)     Slabs              Debug                Memory\n"));
	printf(c!("------------------------------------------------------------------------\n"));
	printf(c!("Object : %7d  Total  : %7ld   Sanity Checks : %s  Total: %7ld\n"), (*s).object_size, (*s).slabs, onoff((*s).sanity_checks as c_int), (*s).slabs * ((page_size as c_ulong) << (*s).order));
	printf(c!("SlabObj: %7d  Full   : %7ld   Redzoning     : %s  Used : %7ld\n"), (*s).slab_size, (*s).slabs - (*s).partial - (*s).cpu_slabs as c_ulong, onoff((*s).red_zone), (*s).objects * (*s).object_size as c_ulong);
	printf(c!("SlabSiz: %7d  Partial: %7ld   Poisoning     : %s  Loss : %7ld\n"), (page_size << (*s).order), (*s).partial, onoff((*s).poison), (*s).slabs * ((page_size as c_ulong) << (*s).order) - (*s).objects * (*s).object_size as c_ulong);
	printf(c!("Loss   : %7d  CpuSlab: %7d   Tracking      : %s  Lalig: %7ld\n"), (*s).slab_size - (*s).object_size, (*s).cpu_slabs, onoff((*s).store_user as c_int), ((*s).slab_size - (*s).object_size) as c_ulong * (*s).objects);
	printf(c!("Align  : %7d  Objects: %7d   Tracing       : %s  Lpadd: %7ld\n"), (*s).align, (*s).objs_per_slab, onoff((*s).trace as c_int), (((page_size as c_uint) << (*s).order) - (*s).objs_per_slab * (*s).slab_size) as c_ulong * (*s).slabs);
	ops(s);
	show_tracking(s);
	slab_numa(s, 1);
	slab_stats(s);
}

unsafe fn slabcache(s: *mut slabinfo) {
	let mut size_str = [0 as c_char; 20];
	let mut dist_str = [0 as c_char; 40];
	let mut flags = [0 as c_char; 20];
	let mut p = flags.as_mut_ptr();
	if strcmp((*s).name, c!("*")) == 0 { return; }
	if unreclaim_only != 0 && (*s).reclaim_account != 0 { return; }
	if actual_slabs == 1 { report(s); return; }
	if skip_zero != 0 && show_empty == 0 && (*s).slabs == 0 { return; }
	if show_empty != 0 && (*s).slabs != 0 { return; }
	if sort_loss == 0 { store_size(size_str.as_mut_ptr(), slab_size(s)); } else { store_size(size_str.as_mut_ptr(), slab_waste(s)); }
	snprintf(dist_str.as_mut_ptr(), 40, c!("%lu/%lu/%d"), (*s).slabs - (*s).cpu_slabs as c_ulong, (*s).partial, (*s).cpu_slabs);
	if line == 0 { first_line(); }
	line += 1;
	if (*s).aliases != 0 { *p = b'*' as c_char; p = p.add(1); }
	if (*s).cache_dma != 0 { *p = b'd' as c_char; p = p.add(1); }
	if (*s).hwcache_align != 0 { *p = b'A' as c_char; p = p.add(1); }
	if (*s).poison != 0 { *p = b'P' as c_char; p = p.add(1); }
	if (*s).reclaim_account != 0 { *p = b'a' as c_char; p = p.add(1); }
	if (*s).red_zone != 0 { *p = b'Z' as c_char; p = p.add(1); }
	if (*s).sanity_checks != 0 { *p = b'F' as c_char; p = p.add(1); }
	if (*s).store_user != 0 { *p = b'U' as c_char; p = p.add(1); }
	if (*s).trace != 0 { *p = b'T' as c_char; p = p.add(1); }
	*p = 0;
	if show_activity != 0 {
		let total_alloc = (*s).alloc_fastpath + (*s).alloc_slowpath;
		let total_free = (*s).free_fastpath + (*s).free_slowpath;
		printf(c!("%-21s %8ld %10ld %10ld %3ld %3ld %5ld %1d %4ld %4ld\n"), (*s).name, (*s).objects, total_alloc, total_free, if total_alloc != 0 { (*s).alloc_fastpath * 100 / total_alloc } else { 0 }, if total_free != 0 { (*s).free_fastpath * 100 / total_free } else { 0 }, (*s).order_fallback, (*s).order, (*s).cmpxchg_double_fail, (*s).cmpxchg_double_cpu_fail);
	} else {
		printf(c!("%-21s %8ld %7d %15s %14s %4d %1d %3ld %3ld %s\n"), (*s).name, (*s).objects, (*s).object_size, size_str.as_ptr(), dist_str.as_ptr(), (*s).objs_per_slab, (*s).order, if (*s).slabs != 0 { (*s).partial * 100 / (*s).slabs } else { 100 }, if (*s).slabs != 0 { (*s).objects * (*s).object_size as c_ulong * 100 / ((*s).slabs * ((page_size as c_ulong) << (*s).order)) } else { 100 }, flags.as_ptr());
	}
}

/*
 * Analyze debug options. Return false if something is amiss.
 */
unsafe fn debug_opt_scan(mut opt: *mut c_char) -> c_int {
	if opt.is_null() || *opt == 0 || strcmp(opt, c!("-")) == 0 { return 1; }
	if strcasecmp(opt, c!("a")) == 0 {
		sanity = 1; poison = 1; redzone = 1; tracking = 1;
		return 1;
	}
	while *opt != 0 {
		match *opt as u8 as char {
			'F' | 'f' => { if sanity != 0 { return 0; } sanity = 1; }
			'P' | 'p' => { if poison != 0 { return 0; } poison = 1; }
			'Z' | 'z' => { if redzone != 0 { return 0; } redzone = 1; }
			'U' | 'u' => { if tracking != 0 { return 0; } tracking = 1; }
			'T' | 't' => { if tracing != 0 { return 0; } tracing = 1; }
			_ => return 0,
		}
		opt = opt.add(1);
	}
	1
}

unsafe fn slab_empty(s: *mut slabinfo) -> c_int {
	if (*s).objects > 0 { return 0; }
	/*
	 * We may still have slabs even if there are no objects. Shrinking will
	 * remove them.
	 */
	if (*s).slabs != 0 { set_obj(s, c!("shrink"), 1); }
	1
}

unsafe fn slab_debug(s: *mut slabinfo) {
	if strcmp((*s).name, c!("*")) == 0 { return; }
	if sanity != 0 && (*s).sanity_checks == 0 { set_obj(s, c!("sanity_checks"), 1); }
	if sanity == 0 && (*s).sanity_checks != 0 {
		if slab_empty(s) != 0 { set_obj(s, c!("sanity_checks"), 0); } else { fprintf(stderr, c!("%s not empty cannot disable sanity checks\n"), (*s).name); }
	}
	if redzone != 0 && (*s).red_zone == 0 {
		if slab_empty(s) != 0 { set_obj(s, c!("red_zone"), 1); } else { fprintf(stderr, c!("%s not empty cannot enable redzoning\n"), (*s).name); }
	}
	if redzone == 0 && (*s).red_zone != 0 {
		if slab_empty(s) != 0 { set_obj(s, c!("red_zone"), 0); } else { fprintf(stderr, c!("%s not empty cannot disable redzoning\n"), (*s).name); }
	}
	if poison != 0 && (*s).poison == 0 {
		if slab_empty(s) != 0 { set_obj(s, c!("poison"), 1); } else { fprintf(stderr, c!("%s not empty cannot enable poisoning\n"), (*s).name); }
	}
	if poison == 0 && (*s).poison != 0 {
		if slab_empty(s) != 0 { set_obj(s, c!("poison"), 0); } else { fprintf(stderr, c!("%s not empty cannot disable poisoning\n"), (*s).name); }
	}
	if tracking != 0 && (*s).store_user == 0 {
		if slab_empty(s) != 0 { set_obj(s, c!("store_user"), 1); } else { fprintf(stderr, c!("%s not empty cannot enable tracking\n"), (*s).name); }
	}
	if tracking == 0 && (*s).store_user != 0 {
		if slab_empty(s) != 0 { set_obj(s, c!("store_user"), 0); } else { fprintf(stderr, c!("%s not empty cannot disable tracking\n"), (*s).name); }
	}
	if tracing != 0 && (*s).trace == 0 {
		if slabs == 1 { set_obj(s, c!("trace"), 1); } else { fprintf(stderr, c!("%s can only enable trace for one slab at a time\n"), (*s).name); }
	}
	if tracing == 0 && (*s).trace != 0 { set_obj(s, c!("trace"), 0); }
}

unsafe fn totals() {
	let mut used_slabs: c_int = 0;
	let (mut b1, mut b2, mut b3, mut b4) = ([0 as c_char; 20], [0 as c_char; 20], [0 as c_char; 20], [0 as c_char; 20]);
	let max: c_ulonglong = 1u64 << 63;
	let (mut min_objsize, mut max_objsize, mut avg_objsize) = (max, 0u64, 0u64);
	let (mut min_partial, mut max_partial, mut avg_partial, mut total_partial) = (max, 0u64, 0u64, 0u64);
	let (mut min_slabs, mut max_slabs, mut avg_slabs, mut total_slabs) = (max, 0u64, 0u64, 0u64);
	let (mut min_size, mut max_size, mut avg_size, mut total_size) = (max, 0u64, 0u64, 0u64);
	let (mut min_used, mut max_used, mut avg_used, mut total_used) = (max, 0u64, 0u64, 0u64);
	let (mut min_waste, mut max_waste, mut avg_waste, mut total_waste) = (max, 0u64, 0u64, 0u64);
	let (mut min_objects, mut max_objects, mut avg_objects, mut total_objects) = (max, 0u64, 0u64, 0u64);
	let (mut min_objwaste, mut max_objwaste, mut avg_objwaste, mut total_objwaste) = (max, 0u64, 0u64, 0u64);
	let (mut min_memobj, mut max_memobj, mut avg_memobj, mut total_objsize) = (max, 0u64, 0u64, 0u64);
	let (mut min_ppart, mut max_ppart, mut avg_ppart, mut total_ppart): (c_ulong, c_ulong, c_ulong, c_ulong) = (100, 0, 0, 0);
	let (mut min_partobj, mut max_partobj, mut avg_partobj, mut total_partobj): (c_ulong, c_ulong, c_ulong, c_ulong) = (max as c_ulong, 0, 0, 0);
	let (mut min_ppartobj, mut max_ppartobj, mut avg_ppartobj, mut total_ppartobj): (c_ulong, c_ulong, c_ulong, c_ulong) = (100, 0, 0, 0);
	let mut s = slabinfo.as_mut_ptr();
	let end = s.add(slabs as usize);
	while s < end {
		if (*s).slabs == 0 || (*s).objects == 0 { s = s.add(1); continue; }
		used_slabs += 1;
		let size = slab_size(s) as c_ulonglong;
		let used = (*s).objects * (*s).object_size as c_ulong;
		let wasted = size - used as c_ulonglong;
		let objwaste = ((*s).slab_size - (*s).object_size) as c_ulonglong;
		let mut percentage_partial_slabs = (*s).partial * 100 / (*s).slabs;
		if percentage_partial_slabs > 100 { percentage_partial_slabs = 100; }
		let mut percentage_partial_objs = (*s).objects_partial * 100 / (*s).objects;
		if percentage_partial_objs > 100 { percentage_partial_objs = 100; }
		if ((*s).object_size as c_ulonglong) < min_objsize { min_objsize = (*s).object_size as c_ulonglong; }
		if ((*s).partial as c_ulonglong) < min_partial { min_partial = (*s).partial as c_ulonglong; }
		if ((*s).slabs as c_ulonglong) < min_slabs { min_slabs = (*s).slabs as c_ulonglong; }
		if size < min_size { min_size = size; }
		if wasted < min_waste { min_waste = wasted; }
		if objwaste < min_objwaste { min_objwaste = objwaste; }
		if ((*s).objects as c_ulonglong) < min_objects { min_objects = (*s).objects as c_ulonglong; }
		if used < min_used as c_ulong { min_used = used as c_ulonglong; }
		if (*s).objects_partial < min_partobj { min_partobj = (*s).objects_partial; }
		if percentage_partial_slabs < min_ppart { min_ppart = percentage_partial_slabs; }
		if percentage_partial_objs < min_ppartobj { min_ppartobj = percentage_partial_objs; }
		if ((*s).slab_size as c_ulonglong) < min_memobj { min_memobj = (*s).slab_size as c_ulonglong; }
		if ((*s).object_size as c_ulonglong) > max_objsize { max_objsize = (*s).object_size as c_ulonglong; }
		if ((*s).partial as c_ulonglong) > max_partial { max_partial = (*s).partial as c_ulonglong; }
		if ((*s).slabs as c_ulonglong) > max_slabs { max_slabs = (*s).slabs as c_ulonglong; }
		if size > max_size { max_size = size; }
		if wasted > max_waste { max_waste = wasted; }
		if objwaste > max_objwaste { max_objwaste = objwaste; }
		if ((*s).objects as c_ulonglong) > max_objects { max_objects = (*s).objects as c_ulonglong; }
		if used > max_used as c_ulong { max_used = used as c_ulonglong; }
		if (*s).objects_partial > max_partobj { max_partobj = (*s).objects_partial; }
		if percentage_partial_slabs > max_ppart { max_ppart = percentage_partial_slabs; }
		if percentage_partial_objs > max_ppartobj { max_ppartobj = percentage_partial_objs; }
		if ((*s).slab_size as c_ulonglong) > max_memobj { max_memobj = (*s).slab_size as c_ulonglong; }
		total_partial += (*s).partial as c_ulonglong;
		total_slabs += (*s).slabs as c_ulonglong;
		total_size += size;
		total_waste += wasted;
		total_objects += (*s).objects as c_ulonglong;
		total_used += used as c_ulonglong;
		total_partobj += (*s).objects_partial;
		total_ppart += percentage_partial_slabs;
		total_ppartobj += percentage_partial_objs;
		total_objwaste += (*s).objects as c_ulonglong * objwaste;
		total_objsize += (*s).objects as c_ulonglong * (*s).slab_size as c_ulonglong;
		s = s.add(1);
	}
	if total_objects == 0 { printf(c!("No objects\n")); return; }
	if used_slabs == 0 { printf(c!("No slabs\n")); return; }
	avg_partial = total_partial / used_slabs as c_ulonglong;
	avg_slabs = total_slabs / used_slabs as c_ulonglong;
	avg_size = total_size / used_slabs as c_ulonglong;
	avg_waste = total_waste / used_slabs as c_ulonglong;
	avg_objects = total_objects / used_slabs as c_ulonglong;
	avg_used = total_used / used_slabs as c_ulonglong;
	avg_partobj = (total_partobj / used_slabs as c_ulong) as c_ulong;
	avg_ppart = total_ppart / used_slabs as c_ulong;
	avg_ppartobj = total_ppartobj / used_slabs as c_ulong;
	avg_objsize = total_used / total_objects;
	avg_objwaste = total_objwaste / total_objects;
	avg_partobj = total_partobj * 100 / total_objects as c_ulong;
	avg_memobj = total_objsize / total_objects;
	printf(c!("Slabcache Totals\n"));
	printf(c!("----------------\n"));
	printf(c!("Slabcaches : %15d   Aliases  : %11d->%-3d  Active:    %3d\n"), slabs, aliases, alias_targets, used_slabs);
	store_size(b1.as_mut_ptr(), total_size as c_ulong); store_size(b2.as_mut_ptr(), total_waste as c_ulong);
	store_size(b3.as_mut_ptr(), (total_waste * 100 / total_used) as c_ulong);
	printf(c!("Memory used: %15s   # Loss   : %15s   MRatio:%6s%%\n"), b1.as_ptr(), b2.as_ptr(), b3.as_ptr());
	store_size(b1.as_mut_ptr(), total_objects as c_ulong); store_size(b2.as_mut_ptr(), total_partobj);
	store_size(b3.as_mut_ptr(), (total_partobj as c_ulonglong * 100 / total_objects) as c_ulong);
	printf(c!("# Objects  : %15s   # PartObj: %15s   ORatio:%6s%%\n"), b1.as_ptr(), b2.as_ptr(), b3.as_ptr());
	printf(c!("\n"));
	printf(c!("Per Cache         Average              Min              Max            Total\n"));
	printf(c!("----------------------------------------------------------------------------\n"));
	store_size(b1.as_mut_ptr(), avg_objects as c_ulong); store_size(b2.as_mut_ptr(), min_objects as c_ulong); store_size(b3.as_mut_ptr(), max_objects as c_ulong); store_size(b4.as_mut_ptr(), total_objects as c_ulong);
	printf(c!("#Objects  %15s  %15s  %15s  %15s\n"), b1.as_ptr(), b2.as_ptr(), b3.as_ptr(), b4.as_ptr());
	store_size(b1.as_mut_ptr(), avg_slabs as c_ulong); store_size(b2.as_mut_ptr(), min_slabs as c_ulong); store_size(b3.as_mut_ptr(), max_slabs as c_ulong); store_size(b4.as_mut_ptr(), total_slabs as c_ulong);
	printf(c!("#Slabs    %15s  %15s  %15s  %15s\n"), b1.as_ptr(), b2.as_ptr(), b3.as_ptr(), b4.as_ptr());
	store_size(b1.as_mut_ptr(), avg_partial as c_ulong); store_size(b2.as_mut_ptr(), min_partial as c_ulong); store_size(b3.as_mut_ptr(), max_partial as c_ulong); store_size(b4.as_mut_ptr(), total_partial as c_ulong);
	printf(c!("#PartSlab %15s  %15s  %15s  %15s\n"), b1.as_ptr(), b2.as_ptr(), b3.as_ptr(), b4.as_ptr());
	store_size(b1.as_mut_ptr(), avg_ppart); store_size(b2.as_mut_ptr(), min_ppart); store_size(b3.as_mut_ptr(), max_ppart); store_size(b4.as_mut_ptr(), (total_partial * 100 / total_slabs) as c_ulong);
	printf(c!("%%PartSlab%15s%% %15s%% %15s%% %15s%%\n"), b1.as_ptr(), b2.as_ptr(), b3.as_ptr(), b4.as_ptr());
	store_size(b1.as_mut_ptr(), avg_partobj); store_size(b2.as_mut_ptr(), min_partobj); store_size(b3.as_mut_ptr(), max_partobj); store_size(b4.as_mut_ptr(), total_partobj);
	printf(c!("PartObjs  %15s  %15s  %15s  %15s\n"), b1.as_ptr(), b2.as_ptr(), b3.as_ptr(), b4.as_ptr());
	store_size(b1.as_mut_ptr(), avg_ppartobj); store_size(b2.as_mut_ptr(), min_ppartobj); store_size(b3.as_mut_ptr(), max_ppartobj); store_size(b4.as_mut_ptr(), (total_partobj as c_ulonglong * 100 / total_objects) as c_ulong);
	printf(c!("%% PartObj%15s%% %15s%% %15s%% %15s%%\n"), b1.as_ptr(), b2.as_ptr(), b3.as_ptr(), b4.as_ptr());
	store_size(b1.as_mut_ptr(), avg_size as c_ulong); store_size(b2.as_mut_ptr(), min_size as c_ulong); store_size(b3.as_mut_ptr(), max_size as c_ulong); store_size(b4.as_mut_ptr(), total_size as c_ulong);
	printf(c!("Memory    %15s  %15s  %15s  %15s\n"), b1.as_ptr(), b2.as_ptr(), b3.as_ptr(), b4.as_ptr());
	store_size(b1.as_mut_ptr(), avg_used as c_ulong); store_size(b2.as_mut_ptr(), min_used as c_ulong); store_size(b3.as_mut_ptr(), max_used as c_ulong); store_size(b4.as_mut_ptr(), total_used as c_ulong);
	printf(c!("Used      %15s  %15s  %15s  %15s\n"), b1.as_ptr(), b2.as_ptr(), b3.as_ptr(), b4.as_ptr());
	store_size(b1.as_mut_ptr(), avg_waste as c_ulong); store_size(b2.as_mut_ptr(), min_waste as c_ulong); store_size(b3.as_mut_ptr(), max_waste as c_ulong); store_size(b4.as_mut_ptr(), total_waste as c_ulong);
	printf(c!("Loss      %15s  %15s  %15s  %15s\n"), b1.as_ptr(), b2.as_ptr(), b3.as_ptr(), b4.as_ptr());
	printf(c!("\n"));
	printf(c!("Per Object        Average              Min              Max\n"));
	printf(c!("-----------------------------------------------------------\n"));
	store_size(b1.as_mut_ptr(), avg_memobj as c_ulong); store_size(b2.as_mut_ptr(), min_memobj as c_ulong); store_size(b3.as_mut_ptr(), max_memobj as c_ulong);
	printf(c!("Memory    %15s  %15s  %15s\n"), b1.as_ptr(), b2.as_ptr(), b3.as_ptr());
	store_size(b1.as_mut_ptr(), avg_objsize as c_ulong); store_size(b2.as_mut_ptr(), min_objsize as c_ulong); store_size(b3.as_mut_ptr(), max_objsize as c_ulong);
	printf(c!("User      %15s  %15s  %15s\n"), b1.as_ptr(), b2.as_ptr(), b3.as_ptr());
	store_size(b1.as_mut_ptr(), avg_objwaste as c_ulong); store_size(b2.as_mut_ptr(), min_objwaste as c_ulong); store_size(b3.as_mut_ptr(), max_objwaste as c_ulong);
	printf(c!("Loss      %15s  %15s  %15s\n"), b1.as_ptr(), b2.as_ptr(), b3.as_ptr());
}

unsafe fn sort_slabs() {
	let mut s1 = slabinfo.as_mut_ptr();
	let end = s1.add(slabs as usize);
	while s1 < end {
		let mut s2 = s1.add(1);
		while s2 < end {
			let mut result: c_int;
			if sort_size != 0 {
				result = if slab_size(s1) == slab_size(s2) { strcasecmp((*s1).name, (*s2).name) } else { (slab_size(s1) < slab_size(s2)) as c_int };
			} else if sort_active != 0 {
				result = if slab_activity(s1) == slab_activity(s2) { strcasecmp((*s1).name, (*s2).name) } else { (slab_activity(s1) < slab_activity(s2)) as c_int };
			} else if sort_loss != 0 {
				result = if slab_waste(s1) == slab_waste(s2) { strcasecmp((*s1).name, (*s2).name) } else { (slab_waste(s1) < slab_waste(s2)) as c_int };
			} else if sort_partial != 0 {
				result = if (*s1).partial == (*s2).partial { strcasecmp((*s1).name, (*s2).name) } else { ((*s1).partial < (*s2).partial) as c_int };
			} else {
				result = strcasecmp((*s1).name, (*s2).name);
			}
			if show_inverted != 0 { result = -result; }
			if result > 0 {
				let mut t = ZERO_SLABINFO;
				memcpy(&mut t as *mut _ as *mut c_void, s1 as *const c_void, core::mem::size_of::<slabinfo>());
				memcpy(s1 as *mut c_void, s2 as *const c_void, core::mem::size_of::<slabinfo>());
				memcpy(s2 as *mut c_void, &t as *const _ as *const c_void, core::mem::size_of::<slabinfo>());
			}
			s2 = s2.add(1);
		}
		s1 = s1.add(1);
	}
}

unsafe fn sort_aliases() {
	let mut a1 = aliasinfo.as_mut_ptr();
	let end = a1.add(aliases as usize);
	while a1 < end {
		let mut a2 = a1.add(1);
		while a2 < end {
			let mut n1 = (*a1).name;
			let mut n2 = (*a2).name;
			if show_alias != 0 && show_inverted == 0 {
				n1 = (*a1).ref_;
				n2 = (*a2).ref_;
			}
			if strcasecmp(n1, n2) > 0 {
				let mut t = ZERO_ALIASINFO;
				memcpy(&mut t as *mut _ as *mut c_void, a1 as *const c_void, core::mem::size_of::<aliasinfo>());
				memcpy(a1 as *mut c_void, a2 as *const c_void, core::mem::size_of::<aliasinfo>());
				memcpy(a2 as *mut c_void, &t as *const _ as *const c_void, core::mem::size_of::<aliasinfo>());
			}
			a2 = a2.add(1);
		}
		a1 = a1.add(1);
	}
}

unsafe fn link_slabs() {
	let mut a = aliasinfo.as_mut_ptr();
	let aend = a.add(aliases as usize);
	while a < aend {
		let mut s = slabinfo.as_mut_ptr();
		let send = s.add(slabs as usize);
		while s < send {
			if strcmp((*a).ref_, (*s).name) == 0 {
				(*a).slab = s;
				(*s).refs += 1;
				break;
			}
			s = s.add(1);
		}
		if s == send { fatal1(c!("Unresolved alias %s\n"), (*a).ref_); }
		a = a.add(1);
	}
}

unsafe fn alias() {
	let mut active: *mut c_char = core::ptr::null_mut();
	sort_aliases();
	link_slabs();
	let mut a = aliasinfo.as_mut_ptr();
	let end = a.add(aliases as usize);
	while a < end {
		if show_single_ref == 0 && (*(*a).slab).refs == 1 { a = a.add(1); continue; }
		if show_inverted == 0 {
			if !active.is_null() && strcmp((*(*a).slab).name, active) == 0 {
				printf(c!(" %s"), (*a).name);
				a = a.add(1);
				continue;
			}
			printf(c!("\n%-12s <- %s"), (*(*a).slab).name, (*a).name);
			active = (*(*a).slab).name;
		} else {
			printf(c!("%-15s -> %s\n"), (*a).name, (*(*a).slab).name);
		}
		a = a.add(1);
	}
	if !active.is_null() { printf(c!("\n")); }
}

unsafe fn rename_slabs() {
	let mut s = slabinfo.as_mut_ptr();
	let end = s.add(slabs as usize);
	while s < end {
		if *(*s).name != b':' as c_char { s = s.add(1); continue; }
		if (*s).refs > 1 && show_first_alias == 0 { s = s.add(1); continue; }
		let a = find_one_alias(s);
		if !a.is_null() { (*s).name = (*a).name; }
		else {
			(*s).name = c!("*") as *mut c_char;
			actual_slabs -= 1;
		}
		s = s.add(1);
	}
}

unsafe fn slab_mismatch(slab: *mut c_char) -> c_int {
	regexec(&pattern, slab, 0, core::ptr::null_mut(), 0)
}

unsafe fn read_slab_dir() {
	let mut slab = slabinfo.as_mut_ptr();
	let mut aliasp = aliasinfo.as_mut_ptr();
	if chdir(c!("/sys/kernel/slab")) != 0 && chdir(c!("/sys/slab")) != 0 {
		fatal(c!("SYSFS support for SLUB not active\n"));
	}
	let dir = opendir(c!("."));
	loop {
		let de = readdir(dir);
		if de.is_null() { break; }
		let dname = (*de).d_name.as_mut_ptr();
		if *dname == b'.' as c_char || (*dname != b':' as c_char && slab_mismatch(dname) != 0) { continue; }
		match (*de).d_type as c_int {
			DT_LNK => {
				if aliasp.offset_from(aliasinfo.as_mut_ptr()) == MAX_ALIASES as isize { fatal(c!("Too many aliases\n")); }
				(*aliasp).name = strdup(dname);
				let count = readlink(dname, buffer.as_mut_ptr(), buffer.len() - 1);
				if count < 0 { fatal1(c!("Cannot read symlink %s\n"), dname); }
				buffer[count as usize] = 0;
				let mut p = buffer.as_mut_ptr().add(count as usize);
				while p > buffer.as_mut_ptr() && *p.sub(1) != b'/' as c_char { p = p.sub(1); }
				(*aliasp).ref_ = strdup(p);
				aliasp = aliasp.add(1);
			}
			DT_DIR => {
				if slab.offset_from(slabinfo.as_mut_ptr()) == MAX_SLABS as isize { fatal(c!("Too many slabs\n")); }
				if chdir(dname) != 0 { fatal1(c!("Unable to access slab %s\n"), (*slab).name); }
				(*slab).name = strdup(dname);
				(*slab).alias = 0;
				(*slab).refs = 0;
				(*slab).aliases = get_obj(c!("aliases")) as c_int;
				(*slab).align = get_obj(c!("align")) as c_int;
				(*slab).cache_dma = get_obj(c!("cache_dma")) as c_int;
				(*slab).cpu_slabs = get_obj(c!("cpu_slabs")) as c_int;
				(*slab).destroy_by_rcu = get_obj(c!("destroy_by_rcu")) as c_int;
				(*slab).hwcache_align = get_obj(c!("hwcache_align")) as c_uint;
				(*slab).object_size = get_obj(c!("object_size")) as c_uint;
				(*slab).objects = get_obj(c!("objects"));
				(*slab).objects_partial = get_obj(c!("objects_partial"));
				(*slab).total_objects = get_obj(c!("total_objects"));
				(*slab).objs_per_slab = get_obj(c!("objs_per_slab")) as c_uint;
				(*slab).order = get_obj(c!("order")) as c_int;
				let mut t: *mut c_char = core::ptr::null_mut();
				(*slab).partial = get_obj_and_str(c!("partial"), &mut t);
				decode_numa_list((*slab).numa_partial.as_mut_ptr(), t);
				free(t as *mut c_void);
				(*slab).poison = get_obj(c!("poison")) as c_int;
				(*slab).reclaim_account = get_obj(c!("reclaim_account")) as c_int;
				(*slab).red_zone = get_obj(c!("red_zone")) as c_int;
				(*slab).sanity_checks = get_obj(c!("sanity_checks")) as c_uint;
				(*slab).slab_size = get_obj(c!("slab_size")) as c_uint;
				(*slab).slabs = get_obj_and_str(c!("slabs"), &mut t);
				decode_numa_list((*slab).numa.as_mut_ptr(), t);
				free(t as *mut c_void);
				(*slab).store_user = get_obj(c!("store_user")) as c_uint;
				(*slab).trace = get_obj(c!("trace")) as c_uint;
				(*slab).alloc_fastpath = get_obj(c!("alloc_fastpath"));
				(*slab).alloc_slowpath = get_obj(c!("alloc_slowpath"));
				(*slab).free_fastpath = get_obj(c!("free_fastpath"));
				(*slab).free_slowpath = get_obj(c!("free_slowpath"));
				(*slab).free_frozen = get_obj(c!("free_frozen"));
				(*slab).free_add_partial = get_obj(c!("free_add_partial"));
				(*slab).free_remove_partial = get_obj(c!("free_remove_partial"));
				(*slab).alloc_from_partial = get_obj(c!("alloc_from_partial"));
				(*slab).alloc_slab = get_obj(c!("alloc_slab"));
				(*slab).alloc_refill = get_obj(c!("alloc_refill"));
				(*slab).free_slab = get_obj(c!("free_slab"));
				(*slab).cpuslab_flush = get_obj(c!("cpuslab_flush"));
				(*slab).deactivate_full = get_obj(c!("deactivate_full"));
				(*slab).deactivate_empty = get_obj(c!("deactivate_empty"));
				(*slab).deactivate_to_head = get_obj(c!("deactivate_to_head"));
				(*slab).deactivate_to_tail = get_obj(c!("deactivate_to_tail"));
				(*slab).deactivate_remote_frees = get_obj(c!("deactivate_remote_frees"));
				(*slab).order_fallback = get_obj(c!("order_fallback"));
				(*slab).cmpxchg_double_cpu_fail = get_obj(c!("cmpxchg_double_cpu_fail"));
				(*slab).cmpxchg_double_fail = get_obj(c!("cmpxchg_double_fail"));
				(*slab).cpu_partial_alloc = get_obj(c!("cpu_partial_alloc"));
				(*slab).cpu_partial_free = get_obj(c!("cpu_partial_free"));
				(*slab).alloc_node_mismatch = get_obj(c!("alloc_node_mismatch"));
				(*slab).deactivate_bypass = get_obj(c!("deactivate_bypass"));
				if chdir(c!("..")) != 0 { fatal1(c!("Unable to chdir from slab ../%s\n"), (*slab).name); }
				if *(*slab).name == b':' as c_char { alias_targets += 1; }
				slab = slab.add(1);
			}
			_ => fatal1(c!("Unknown file type %lx\n"), (*de).d_type as *const c_char),
		}
	}
	closedir(dir);
	slabs = slab.offset_from(slabinfo.as_mut_ptr()) as c_int;
	actual_slabs = slabs;
	aliases = aliasp.offset_from(aliasinfo.as_mut_ptr()) as c_int;
}

unsafe fn output_slabs() {
	let mut slab = slabinfo.as_mut_ptr();
	let end = slab.add(slabs as usize);
	let mut lines = output_lines;
	while slab < end && lines != 0 {
		if (*slab).alias != 0 { slab = slab.add(1); continue; }
		if lines != -1 { lines -= 1; }
		if show_numa != 0 { slab_numa(slab, 0); }
		else if show_track != 0 { show_tracking(slab); }
		else if validate != 0 { slab_validate(slab); }
		else if shrink != 0 { slab_shrink(slab); }
		else if set_debug != 0 { slab_debug(slab); }
		else if show_ops != 0 { ops(slab); }
		else if show_slab != 0 { slabcache(slab); }
		else if show_report != 0 { report(slab); }
		slab = slab.add(1);
	}
}

unsafe fn _xtotals(heading: *mut c_char, underline: *mut c_char, loss: c_int, size: c_int, partial: c_int) {
	printf(c!("%s%s"), heading, underline);
	line = 0;
	sort_loss = loss;
	sort_size = size;
	sort_partial = partial;
	sort_slabs();
	output_slabs();
}

unsafe fn xtotals() {
	totals();
	link_slabs();
	rename_slabs();
	_xtotals(c!("\nSlabs sorted by size\n") as *mut c_char, c!("--------------------\n") as *mut c_char, 0, 1, 0);
	_xtotals(c!("\nSlabs sorted by loss\n") as *mut c_char, c!("--------------------\n") as *mut c_char, 1, 0, 0);
	_xtotals(c!("\nSlabs sorted by number of partial slabs\n") as *mut c_char, c!("---------------------------------------\n") as *mut c_char, 0, 0, 1);
	printf(c!("\n"));
}

static mut opts: [option; 28] = [
	option { name: c!("aliases"), has_arg: no_argument, flag: core::ptr::null_mut(), val: 'a' as c_int },
	option { name: c!("activity"), has_arg: no_argument, flag: core::ptr::null_mut(), val: 'A' as c_int },
	option { name: c!("Bytes"), has_arg: no_argument, flag: core::ptr::null_mut(), val: 'B' as c_int },
	option { name: c!("debug"), has_arg: optional_argument, flag: core::ptr::null_mut(), val: 'd' as c_int },
	option { name: c!("display-activity"), has_arg: no_argument, flag: core::ptr::null_mut(), val: 'D' as c_int },
	option { name: c!("empty"), has_arg: no_argument, flag: core::ptr::null_mut(), val: 'e' as c_int },
	option { name: c!("first-alias"), has_arg: no_argument, flag: core::ptr::null_mut(), val: 'f' as c_int },
	option { name: c!("help"), has_arg: no_argument, flag: core::ptr::null_mut(), val: 'h' as c_int },
	option { name: c!("inverted"), has_arg: no_argument, flag: core::ptr::null_mut(), val: 'i' as c_int },
	option { name: c!("slabs"), has_arg: no_argument, flag: core::ptr::null_mut(), val: 'l' as c_int },
	option { name: c!("Loss"), has_arg: no_argument, flag: core::ptr::null_mut(), val: 'L' as c_int },
	option { name: c!("numa"), has_arg: no_argument, flag: core::ptr::null_mut(), val: 'n' as c_int },
	option { name: c!("lines"), has_arg: required_argument, flag: core::ptr::null_mut(), val: 'N' as c_int },
	option { name: c!("ops"), has_arg: no_argument, flag: core::ptr::null_mut(), val: 'o' as c_int },
	option { name: c!("partial"), has_arg: no_argument, flag: core::ptr::null_mut(), val: 'P' as c_int },
	option { name: c!("report"), has_arg: no_argument, flag: core::ptr::null_mut(), val: 'r' as c_int },
	option { name: c!("shrink"), has_arg: no_argument, flag: core::ptr::null_mut(), val: 's' as c_int },
	option { name: c!("Size"), has_arg: no_argument, flag: core::ptr::null_mut(), val: 'S' as c_int },
	option { name: c!("tracking"), has_arg: no_argument, flag: core::ptr::null_mut(), val: 't' as c_int },
	option { name: c!("Totals"), has_arg: no_argument, flag: core::ptr::null_mut(), val: 'T' as c_int },
	option { name: c!("Unreclaim"), has_arg: no_argument, flag: core::ptr::null_mut(), val: 'U' as c_int },
	option { name: c!("validate"), has_arg: no_argument, flag: core::ptr::null_mut(), val: 'v' as c_int },
	option { name: c!("Xtotals"), has_arg: no_argument, flag: core::ptr::null_mut(), val: 'X' as c_int },
	option { name: c!("zero"), has_arg: no_argument, flag: core::ptr::null_mut(), val: 'z' as c_int },
	option { name: c!("1ref"), has_arg: no_argument, flag: core::ptr::null_mut(), val: '1' as c_int },
	option { name: core::ptr::null(), has_arg: 0, flag: core::ptr::null_mut(), val: 0 },
	option { name: core::ptr::null(), has_arg: 0, flag: core::ptr::null_mut(), val: 0 },
	option { name: core::ptr::null(), has_arg: 0, flag: core::ptr::null_mut(), val: 0 },
];

unsafe fn real_main(argc: c_int, argv: *mut *mut c_char) -> c_int {
	let mut pattern_source: *mut c_char;
	page_size = getpagesize();
	loop {
		let c_ = getopt_long(argc, argv, c!("aABd::DefhilLnN:oPrsStTUvXz1"), opts.as_ptr(), core::ptr::null_mut());
		if c_ == -1 { break; }
		match c_ as u8 as char {
			'a' => show_alias = 1,
			'A' => sort_active = 1,
			'B' => show_bytes = 1,
			'd' => { set_debug = 1; if debug_opt_scan(optarg) == 0 { fatal1(c!("Invalid debug option '%s'\n"), optarg); } }
			'D' => show_activity = 1,
			'e' => show_empty = 1,
			'f' => show_first_alias = 1,
			'h' => { usage(); return 0; }
			'i' => show_inverted = 1,
			'l' => show_slab = 1,
			'L' => sort_loss = 1,
			'n' => show_numa = 1,
			'N' => if !optarg.is_null() { output_lines = atoi(optarg); if output_lines < 1 { output_lines = 1; } },
			'o' => show_ops = 1,
			'r' => show_report = 1,
			'P' => sort_partial = 1,
			's' => shrink = 1,
			'S' => sort_size = 1,
			't' => show_track = 1,
			'T' => show_totals = 1,
			'U' => unreclaim_only = 1,
			'v' => validate = 1,
			'X' => { if output_lines == -1 { output_lines = 1; } extended_totals = 1; show_bytes = 1; }
			'z' => skip_zero = 0,
			'1' => show_single_ref = 1,
			_ => fatal2(c!("%s: Invalid option '%c'\n"), *argv, optopt as usize as *const c_char),
		}
	}
	if show_slab == 0 && show_alias == 0 && show_track == 0 && show_report == 0 && validate == 0 && shrink == 0 && set_debug == 0 && show_ops == 0 {
		show_slab = 1;
	}
	if argc > optind { pattern_source = *argv.add(optind as usize); } else { pattern_source = c!(".*") as *mut c_char; }
	let err = regcomp(&mut pattern, pattern_source, REG_ICASE | REG_NOSUB);
	if err != 0 { fatal3(c!("%s: Invalid pattern '%s' code %d\n"), *argv, pattern_source, err); }
	read_slab_dir();
	if show_alias != 0 {
		alias();
	} else if extended_totals != 0 {
		xtotals();
	} else if show_totals != 0 {
		totals();
	} else {
		link_slabs();
		rename_slabs();
		sort_slabs();
		output_slabs();
	}
	0
}

pub unsafe fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
	real_main(argc, argv)
}
