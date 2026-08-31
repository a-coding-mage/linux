// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/cpumap.c. Original C includes:
// <api/fs/fs.h>, "cpumap.h", "debug.h", "event.h", <assert.h>,
// <dirent.h>, <stdio.h>, <stdlib.h>, <linux/bitmap.h>, "asm/bug.h",
// <linux/compiler.h>, <linux/ctype.h>, <linux/zalloc.h>,
// <internal/cpumap.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;
use core::ptr;

type size_t = usize;
type u16 = u16;
type __u32 = u32;
type __u64 = u64;
type FILE = c_void;
type DIR = c_void;

const PATH_MAX: usize = 4096;
const BITS_PER_BYTE: c_int = 8;
const INT16_MAX: c_int = 32767;
const DT_DIR: u8 = 4;
const DT_LNK: u8 = 10;
const PERF_CPU_MAP__CPUS: c_int = 0;
const PERF_CPU_MAP__MASK: c_int = 1;
const PERF_CPU_MAP__RANGE_CPUS: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_cpu {
    pub cpu: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aggr_cpu_id {
    pub thread_idx: c_int,
    pub node: c_int,
    pub socket: c_int,
    pub die: c_int,
    pub cluster: c_int,
    pub cache_lvl: c_int,
    pub cache: c_int,
    pub core: c_int,
    pub cpu: perf_cpu,
}

#[repr(C)]
pub struct perf_cpu_map {
    pub nr: c_int,
    pub map: [perf_cpu; 0],
}

#[repr(C)]
pub struct cpu_aggr_map {
    pub nr: c_int,
    pub map: [aggr_cpu_id; 0],
}

#[repr(C)]
pub struct cpus_data {
    pub nr: u16,
    pub cpu: [u16; 0],
}

#[repr(C)]
pub struct mask32_data {
    pub nr: u16,
    pub long_size: u16,
    pub mask: [__u32; 0],
}

#[repr(C)]
pub struct mask64_data {
    pub nr: u16,
    pub long_size: u16,
    pub mask: [__u64; 0],
}

#[repr(C)]
pub struct range_cpu_data {
    pub start_cpu: u16,
    pub end_cpu: u16,
    pub any_cpu: u16,
}

#[repr(C)]
pub union perf_record_cpu_map_data_union {
    pub cpus_data: mem::ManuallyDrop<cpus_data>,
    pub mask32_data: mem::ManuallyDrop<mask32_data>,
    pub mask64_data: mem::ManuallyDrop<mask64_data>,
    pub range_cpu_data: mem::ManuallyDrop<range_cpu_data>,
}

#[repr(C)]
pub struct perf_record_cpu_map_data {
    pub type_: c_int,
    pub data: perf_record_cpu_map_data_union,
}

#[repr(C)]
pub struct dirent {
    pub d_ino: u64,
    pub d_off: i64,
    pub d_reclen: u16,
    pub d_type: u8,
    pub d_name: [c_char; 256],
}

type aggr_cpu_id_get_t = unsafe extern "C" fn(perf_cpu, *mut c_void) -> aggr_cpu_id;

unsafe extern "C" {
    fn perf_cpu_map__alloc(nr: c_int) -> *mut perf_cpu_map;
    fn perf_cpu_map__put(map: *mut perf_cpu_map);
    fn perf_cpu_map__get(map: *mut perf_cpu_map) -> *mut perf_cpu_map;
    fn perf_cpu_map__nr(map: *const perf_cpu_map) -> c_uint;
    fn perf_cpu_map__cpu(map: *const perf_cpu_map, idx: c_int) -> perf_cpu;
    fn perf_cpu_map__max(map: *mut perf_cpu_map) -> perf_cpu;
    fn perf_cpu_map__new_online_cpus() -> *mut perf_cpu_map;
    fn sysfs__read_int(path: *const c_char, value: *mut c_int) -> c_int;
    fn sysfs__mountpoint() -> *const c_char;
    fn filename__read_str(path: *mut c_char, buf: *mut *mut c_char, size: *mut size_t) -> c_int;
    fn bitmap_weight(bitmap: *const c_ulong, bits: c_int) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn scnprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn zalloc(size: size_t) -> *mut c_void;
    fn qsort(base: *mut c_void, nmemb: size_t, size: size_t,
             compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>);
    fn opendir(name: *const c_char) -> *mut DIR;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn closedir(dirp: *mut DIR) -> c_int;
    fn pr_err(format: *const c_char, ...);
    fn pr_warning(format: *const c_char, ...);
    fn pr_debug(format: *const c_char, ...);
    fn pr_debug2(format: *const c_char, ...);
}

type c_ulong = usize;

static mut max_cpu_num: perf_cpu = perf_cpu { cpu: 0 };
static mut max_present_cpu_num: perf_cpu = perf_cpu { cpu: 0 };
static mut max_node_num: c_int = 0;
/**
 * The numa node X as read from /sys/devices/system/node/nodeX indexed by the
 * CPU number.
 */
static mut cpunode_map: *mut c_int = ptr::null_mut();

unsafe fn cpu_map_entry(map: *mut perf_cpu_map, idx: c_int) -> *mut perf_cpu {
    ((*map).map.as_ptr() as *mut perf_cpu).add(idx as usize)
}

unsafe fn aggr_map_entry(map: *mut cpu_aggr_map, idx: c_int) -> *mut aggr_cpu_id {
    ((*map).map.as_ptr() as *mut aggr_cpu_id).add(idx as usize)
}

unsafe fn READ_ONCE<T: Copy>(p: *const T) -> T {
    ptr::read_volatile(p)
}

fn unlikely(v: bool) -> bool {
    v
}

unsafe fn set_bit_iter_next(bitmap: *const c_ulong, nbits: c_int, start: c_int) -> c_int {
    let mut bit = start;
    while bit < nbits {
        let word = *bitmap.add((bit as usize) / (mem::size_of::<c_ulong>() * 8));
        if ((word >> ((bit as usize) % (mem::size_of::<c_ulong>() * 8))) & 1) != 0 {
            return bit;
        }
        bit += 1;
    }
    nbits
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_record_cpu_map_data__test_bit(
    i: c_int,
    data: *const perf_record_cpu_map_data,
) -> bool {
    let bit_word32 = i / 32;
    let bit_mask32: __u32 = 1u32 << (i & 31);
    let bit_word64 = i / 64;
    let bit_mask64: __u64 = (1u64) << (i & 63);

    if READ_ONCE(&(*data).data.mask32_data.long_size as *const u16) == 4 {
        (bit_word32 < READ_ONCE(&(*data).data.mask32_data.nr as *const u16) as c_int)
            && ((*data).data.mask32_data.mask.as_ptr().add(bit_word32 as usize).read() & bit_mask32) != 0
    } else {
        (bit_word64 < READ_ONCE(&(*data).data.mask64_data.nr as *const u16) as c_int)
            && ((*data).data.mask64_data.mask.as_ptr().add(bit_word64 as usize).read() & bit_mask64) != 0
    }
}

/* Read ith mask value from data into the given 64-bit sized bitmap */
unsafe fn perf_record_cpu_map_data__read_one_mask(
    data: *const perf_record_cpu_map_data,
    i: c_int,
    bitmap: *mut c_ulong,
    long_size: u16,
) {
    if mem::size_of::<c_ulong>() == 8 {
        if long_size == 4 {
            *bitmap.add(0) = *(*data).data.mask32_data.mask.as_ptr().add(i as usize) as c_ulong;
        } else {
            *bitmap.add(0) = *(*data).data.mask64_data.mask.as_ptr().add(i as usize) as c_ulong;
        }
    } else if long_size == 4 {
        *bitmap.add(0) = *(*data).data.mask32_data.mask.as_ptr().add(i as usize) as c_ulong;
        *bitmap.add(1) = 0;
    } else {
        #[cfg(target_endian = "big")]
        {
            *bitmap.add(0) = (*(*data).data.mask64_data.mask.as_ptr().add(i as usize) >> 32) as c_ulong;
            *bitmap.add(1) = *(*data).data.mask64_data.mask.as_ptr().add(i as usize) as c_ulong;
        }
        #[cfg(not(target_endian = "big"))]
        {
            *bitmap.add(0) = *(*data).data.mask64_data.mask.as_ptr().add(i as usize) as c_ulong;
            *bitmap.add(1) = (*(*data).data.mask64_data.mask.as_ptr().add(i as usize) >> 32) as c_ulong;
        }
    }
}

unsafe fn cpu_map__from_entries(data: *const perf_record_cpu_map_data) -> *mut perf_cpu_map {
    /* Snapshot nr - data is mmap'd and could change between reads */
    let nr = READ_ONCE(&(*data).data.cpus_data.nr as *const u16);
    let map = perf_cpu_map__empty_new(nr as c_int);

    if map.is_null() {
        return ptr::null_mut();
    }

    for i in 0..nr as c_uint {
        let cpu = READ_ONCE((*data).data.cpus_data.cpu.as_ptr().add(i as usize));
        /*
         * Special treatment for -1, which is not real cpu number,
         * and we need to use (int) -1 to initialize map[i],
         * otherwise it would become 65535.
         */
        if cpu == (-1i16 as u16) {
            (*cpu_map_entry(map, i as c_int)).cpu = -1;
        } else if (cpu as c_int) < INT16_MAX {
            (*cpu_map_entry(map, i as c_int)).cpu = cpu as i16;
        } else {
            pr_err(c"Invalid cpumap entry %u\n".as_ptr(), cpu as c_uint);
            perf_cpu_map__put(map);
            return ptr::null_mut();
        }
    }

    map
}

unsafe fn cpu_map__from_mask(data: *const perf_record_cpu_map_data) -> *mut perf_cpu_map {
    let mut local_copy: [c_ulong; 64 / (mem::size_of::<c_ulong>() * 8)] =
        [0; 64 / (mem::size_of::<c_ulong>() * 8)];
    let mut weight: c_int = 0;
    let mask_nr: c_int;
    /* Snapshot before validation - data is mmap'd and could change */
    let long_size = READ_ONCE(&(*data).data.mask32_data.long_size as *const u16);

    /* long_size must be 4 or 8; other values overflow cpus_per_i below */
    if long_size != 4 && long_size != 8 {
        pr_warning(c"WARNING: cpu_map mask: unsupported long_size %u\n".as_ptr(), long_size as c_uint);
        return ptr::null_mut();
    }

    mask_nr = READ_ONCE(&(*data).data.mask32_data.nr as *const u16) as c_int;

    for i in 0..mask_nr {
        perf_record_cpu_map_data__read_one_mask(data, i, local_copy.as_mut_ptr(), long_size);
        weight += bitmap_weight(local_copy.as_ptr(), 64);
    }

    let map = perf_cpu_map__empty_new(weight);
    if map.is_null() {
        return ptr::null_mut();
    }

    let mut j = 0;
    for i in 0..mask_nr {
        let cpus_per_i = i * long_size as c_int * BITS_PER_BYTE;

        perf_record_cpu_map_data__read_one_mask(data, i, local_copy.as_mut_ptr(), long_size);
        let mut cpu = set_bit_iter_next(local_copy.as_ptr(), 64, 0);
        while cpu < 64 {
            /* Guard against more set bits than the first pass counted */
            if j >= weight {
                break;
            }
            if cpu + cpus_per_i < INT16_MAX {
                (*cpu_map_entry(map, j)).cpu = (cpu + cpus_per_i) as i16;
                j += 1;
            } else {
                pr_err(c"Invalid cpumap entry %d\n".as_ptr(), cpu + cpus_per_i);
                perf_cpu_map__put(map);
                return ptr::null_mut();
            }
            cpu = set_bit_iter_next(local_copy.as_ptr(), 64, cpu + 1);
        }
    }
    map
}

unsafe fn cpu_map__from_range(data: *const perf_record_cpu_map_data) -> *mut perf_cpu_map {
    /* Snapshot fields - data is mmap'd and could change between reads */
    let start_cpu = READ_ONCE(&(*data).data.range_cpu_data.start_cpu as *const u16);
    let end_cpu = READ_ONCE(&(*data).data.range_cpu_data.end_cpu as *const u16);
    let any_cpu = READ_ONCE(&(*data).data.range_cpu_data.any_cpu as *const u16);
    let mut i: c_uint = 0;

    if end_cpu < start_cpu {
        pr_warning(
            c"WARNING: cpu_map range: end_cpu %u < start_cpu %u\n".as_ptr(),
            end_cpu as c_uint,
            start_cpu as c_uint,
        );
        return ptr::null_mut();
    }

    /* any_cpu is boolean (0 or 1), not a count - clamp to avoid inflated nr */
    let map = perf_cpu_map__empty_new(
        (end_cpu - start_cpu + 1 + if any_cpu != 0 { 1 } else { 0 }) as c_int,
    );
    if map.is_null() {
        return ptr::null_mut();
    }

    if any_cpu != 0 {
        (*cpu_map_entry(map, i as c_int)).cpu = -1;
        i += 1;
    }

    let mut cpu = start_cpu as c_int;
    while cpu <= end_cpu as c_int {
        if cpu < INT16_MAX {
            (*cpu_map_entry(map, i as c_int)).cpu = cpu as i16;
        } else {
            pr_err(c"Invalid cpumap entry %d\n".as_ptr(), cpu);
            perf_cpu_map__put(map);
            return ptr::null_mut();
        }
        i += 1;
        cpu += 1;
    }

    map
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cpu_map__new_data(data: *const perf_record_cpu_map_data) -> *mut perf_cpu_map {
    match (*data).type_ {
        PERF_CPU_MAP__CPUS => cpu_map__from_entries(data),
        PERF_CPU_MAP__MASK => cpu_map__from_mask(data),
        PERF_CPU_MAP__RANGE_CPUS => cpu_map__from_range(data),
        _ => {
            pr_err(c"cpu_map__new_data unknown type %d\n".as_ptr(), (*data).type_);
            ptr::null_mut()
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cpu_map__fprintf(map: *mut perf_cpu_map, fp: *mut FILE) -> size_t {
    const BUFSIZE: usize = 1024;
    let mut buf = [0 as c_char; BUFSIZE];

    cpu_map__snprint(map, buf.as_mut_ptr(), mem::size_of_val(&buf));
    fprintf(fp, c"%s\n".as_ptr(), buf.as_ptr()) as size_t
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_cpu_map__empty_new(nr: c_int) -> *mut perf_cpu_map {
    let cpus = perf_cpu_map__alloc(nr);

    if !cpus.is_null() {
        for i in 0..nr {
            (*cpu_map_entry(cpus, i)).cpu = -1;
        }
    }

    cpus
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cpu_aggr_map__empty_new(nr: c_int) -> *mut cpu_aggr_map {
    let cpus = malloc(mem::size_of::<cpu_aggr_map>() + mem::size_of::<aggr_cpu_id>() * nr as usize)
        as *mut cpu_aggr_map;

    if !cpus.is_null() {
        (*cpus).nr = nr;
        for i in 0..nr {
            *aggr_map_entry(cpus, i) = aggr_cpu_id__empty();
        }
    }

    cpus
}

unsafe fn cpu__get_topology_int(cpu: c_int, name: *const c_char, value: *mut c_int) -> c_int {
    let mut path = [0 as c_char; PATH_MAX];

    snprintf(
        path.as_mut_ptr(),
        PATH_MAX,
        c"devices/system/cpu/cpu%d/topology/%s".as_ptr(),
        cpu,
        name,
    );

    sysfs__read_int(path.as_ptr(), value)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cpu__get_socket_id(cpu: perf_cpu) -> c_int {
    let mut value: c_int = 0;
    let ret = cpu__get_topology_int(cpu.cpu as c_int, c"physical_package_id".as_ptr(), &mut value);
    if ret != 0 { ret } else { value }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn aggr_cpu_id__socket(cpu: perf_cpu, _data: *mut c_void) -> aggr_cpu_id {
    let mut id = aggr_cpu_id__empty();

    id.socket = cpu__get_socket_id(cpu);
    id
}

unsafe extern "C" fn aggr_cpu_id__cmp(a_pointer: *const c_void, b_pointer: *const c_void) -> c_int {
    let a = a_pointer as *mut aggr_cpu_id;
    let b = b_pointer as *mut aggr_cpu_id;

    if (*a).node != (*b).node {
        (*a).node - (*b).node
    } else if (*a).socket != (*b).socket {
        (*a).socket - (*b).socket
    } else if (*a).die != (*b).die {
        (*a).die - (*b).die
    } else if (*a).cluster != (*b).cluster {
        (*a).cluster - (*b).cluster
    } else if (*a).cache_lvl != (*b).cache_lvl {
        (*a).cache_lvl - (*b).cache_lvl
    } else if (*a).cache != (*b).cache {
        (*a).cache - (*b).cache
    } else if (*a).core != (*b).core {
        (*a).core - (*b).core
    } else {
        (*a).thread_idx - (*b).thread_idx
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cpu_aggr_map__new(
    cpus: *const perf_cpu_map,
    get_id: aggr_cpu_id_get_t,
    data: *mut c_void,
    needs_sort: bool,
) -> *mut cpu_aggr_map {
    let mut c = cpu_aggr_map__empty_new(perf_cpu_map__nr(cpus) as c_int);

    if c.is_null() {
        return ptr::null_mut();
    }

    /* Reset size as it may only be partially filled */
    (*c).nr = 0;

    let mut idx: c_uint = 0;
    while idx < perf_cpu_map__nr(cpus) {
        let cpu = perf_cpu_map__cpu(cpus, idx as c_int);
        let mut duplicate = false;
        let cpu_id = get_id(cpu, data);

        for j in 0..(*c).nr {
            if aggr_cpu_id__equal(&cpu_id, aggr_map_entry(c, j)) {
                duplicate = true;
                break;
            }
        }
        if !duplicate {
            *aggr_map_entry(c, (*c).nr) = cpu_id;
            (*c).nr += 1;
        }
        idx += 1;
    }
    /* Trim. */
    if (*c).nr != perf_cpu_map__nr(cpus) as c_int {
        let trimmed_c = realloc(
            c as *mut c_void,
            mem::size_of::<cpu_aggr_map>() + mem::size_of::<aggr_cpu_id>() * (*c).nr as usize,
        ) as *mut cpu_aggr_map;

        if !trimmed_c.is_null() {
            c = trimmed_c;
        }
    }

    /* ensure we process id in increasing order */
    if needs_sort {
        qsort(
            (*c).map.as_mut_ptr() as *mut c_void,
            (*c).nr as size_t,
            mem::size_of::<aggr_cpu_id>(),
            Some(aggr_cpu_id__cmp),
        );
    }

    c
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cpu__get_die_id(cpu: perf_cpu) -> c_int {
    let mut value: c_int = 0;
    let ret = cpu__get_topology_int(cpu.cpu as c_int, c"die_id".as_ptr(), &mut value);

    if ret != 0 { ret } else { value }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn aggr_cpu_id__die(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id {
    let mut die = cpu__get_die_id(cpu);

    /* There is no die_id on legacy system. */
    if die < 0 {
        die = 0;
    }

    /*
     * die_id is relative to socket, so start
     * with the socket ID and then add die to
     * make a unique ID.
     */
    let mut id = aggr_cpu_id__socket(cpu, data);
    if aggr_cpu_id__is_empty(&id) {
        return id;
    }

    id.die = die;
    id
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cpu__get_cluster_id(cpu: perf_cpu) -> c_int {
    let mut value: c_int = 0;
    let ret = cpu__get_topology_int(cpu.cpu as c_int, c"cluster_id".as_ptr(), &mut value);

    if ret != 0 { ret } else { value }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn aggr_cpu_id__cluster(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id {
    let mut cluster = cpu__get_cluster_id(cpu);

    /* There is no cluster_id on legacy system. */
    if cluster < 0 {
        cluster = 0;
    }

    let mut id = aggr_cpu_id__die(cpu, data);
    if aggr_cpu_id__is_empty(&id) {
        return id;
    }

    id.cluster = cluster;
    id
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cpu__get_core_id(cpu: perf_cpu) -> c_int {
    let mut value: c_int = 0;
    let ret = cpu__get_topology_int(cpu.cpu as c_int, c"core_id".as_ptr(), &mut value);
    if ret != 0 { ret } else { value }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn aggr_cpu_id__core(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id {
    let core = cpu__get_core_id(cpu);

    /* aggr_cpu_id__die returns a struct with socket die, and cluster set. */
    let mut id = aggr_cpu_id__cluster(cpu, data);
    if aggr_cpu_id__is_empty(&id) {
        return id;
    }

    /*
     * core_id is relative to socket and die, we need a global id.
     * So we combine the result from cpu_map__get_die with the core id
     */
    id.core = core;
    id
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn aggr_cpu_id__cpu(cpu: perf_cpu, data: *mut c_void) -> aggr_cpu_id {
    /* aggr_cpu_id__core returns a struct with socket, die and core set. */
    let mut id = aggr_cpu_id__core(cpu, data);
    if aggr_cpu_id__is_empty(&id) {
        return id;
    }

    id.cpu = cpu;
    id
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn aggr_cpu_id__node(cpu: perf_cpu, _data: *mut c_void) -> aggr_cpu_id {
    let mut id = aggr_cpu_id__empty();

    id.node = cpu__get_node(cpu);
    id
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn aggr_cpu_id__global(mut cpu: perf_cpu, _data: *mut c_void) -> aggr_cpu_id {
    let mut id = aggr_cpu_id__empty();

    /* it always aggregates to the cpu 0 */
    cpu.cpu = 0;
    id.cpu = cpu;
    id
}

/* setup simple routines to easily access node numbers given a cpu number */
unsafe fn get_max_num(path: *mut c_char, max: *mut c_int) -> c_int {
    let mut num: size_t = 0;
    let mut buf: *mut c_char = ptr::null_mut();
    let mut err: c_int = 0;

    if filename__read_str(path, &mut buf, &mut num) != 0 {
        return -1;
    }

    *buf.add(num) = b'\0' as c_char;

    /* empty file - nothing to parse */
    if num == 0 {
        err = -1;
        free(buf as *mut c_void);
        return err;
    }

    /* start on the right, to find highest node num */
    loop {
        num -= 1;
        if num == 0 {
            break;
        }
        if *buf.add(num) == b',' as c_char || *buf.add(num) == b'-' as c_char {
            num += 1;
            break;
        }
    }
    if sscanf(buf.add(num), c"%d".as_ptr(), max) < 1 {
        err = -1;
        free(buf as *mut c_void);
        return err;
    }

    /* convert from 0-based to 1-based */
    *max += 1;

    free(buf as *mut c_void);
    err
}

/* Determine highest possible cpu in the system for sparse allocation */
unsafe fn set_max_cpu_num() {
    let mnt: *const c_char;
    let mut path = [0 as c_char; PATH_MAX];
    let mut max: c_int = 0;
    let mut ret: c_int = -1;

    /* set up default */
    max_cpu_num.cpu = 4096;
    max_present_cpu_num.cpu = 4096;

    mnt = sysfs__mountpoint();
    if mnt.is_null() {
        if ret != 0 {
            pr_err(c"Failed to read max cpus, using default of %d\n".as_ptr(), max_cpu_num.cpu as c_int);
        }
        return;
    }

    /* get the highest possible cpu number for a sparse allocation */
    ret = snprintf(path.as_mut_ptr(), PATH_MAX, c"%s/devices/system/cpu/possible".as_ptr(), mnt);
    if ret >= PATH_MAX as c_int {
        pr_err(c"sysfs path crossed PATH_MAX(%d) size\n".as_ptr(), PATH_MAX as c_int);
        if ret != 0 {
            pr_err(c"Failed to read max cpus, using default of %d\n".as_ptr(), max_cpu_num.cpu as c_int);
        }
        return;
    }

    ret = get_max_num(path.as_mut_ptr(), &mut max);
    if ret != 0 {
        pr_err(c"Failed to read max cpus, using default of %d\n".as_ptr(), max_cpu_num.cpu as c_int);
        return;
    }

    /*
     * struct perf_cpu.cpu is int16_t (libperf ABI) - clamp to avoid
     * truncation to negative.  See tools/lib/perf/TODO for the ABI
     * widening plan.
     */
    if max > INT16_MAX {
        pr_warning(
            c"WARNING: max possible cpus %d exceeds int16_t, clamping to %d\n".as_ptr(),
            max,
            INT16_MAX,
        );
        max = INT16_MAX;
    }
    max_cpu_num.cpu = max as i16;

    /* get the highest present cpu number for a sparse allocation */
    ret = snprintf(path.as_mut_ptr(), PATH_MAX, c"%s/devices/system/cpu/present".as_ptr(), mnt);
    if ret >= PATH_MAX as c_int {
        pr_err(c"sysfs path crossed PATH_MAX(%d) size\n".as_ptr(), PATH_MAX as c_int);
        pr_err(c"Failed to read max cpus, using default of %d\n".as_ptr(), max_cpu_num.cpu as c_int);
        return;
    }

    ret = get_max_num(path.as_mut_ptr(), &mut max);

    if ret == 0 && max > INT16_MAX {
        pr_warning(
            c"WARNING: max present cpus %d exceeds int16_t, clamping to %d\n".as_ptr(),
            max,
            INT16_MAX,
        );
        max = INT16_MAX;
    }
    if ret == 0 {
        max_present_cpu_num.cpu = max as i16;
    }
    if ret != 0 {
        pr_err(c"Failed to read max cpus, using default of %d\n".as_ptr(), max_cpu_num.cpu as c_int);
    }
}

/* Determine highest possible node in the system for sparse allocation */
unsafe fn set_max_node_num() {
    let mnt: *const c_char;
    let mut path = [0 as c_char; PATH_MAX];
    let mut ret: c_int = -1;

    /* set up default */
    max_node_num = 8;

    mnt = sysfs__mountpoint();
    if mnt.is_null() {
        if ret != 0 {
            pr_err(c"Failed to read max nodes, using default of %d\n".as_ptr(), max_node_num);
        }
        return;
    }

    /* get the highest possible cpu number for a sparse allocation */
    ret = snprintf(path.as_mut_ptr(), PATH_MAX, c"%s/devices/system/node/possible".as_ptr(), mnt);
    if ret >= PATH_MAX as c_int {
        pr_err(c"sysfs path crossed PATH_MAX(%d) size\n".as_ptr(), PATH_MAX as c_int);
        if ret != 0 {
            pr_err(c"Failed to read max nodes, using default of %d\n".as_ptr(), max_node_num);
        }
        return;
    }

    ret = get_max_num(path.as_mut_ptr(), &mut max_node_num);

    if ret != 0 {
        pr_err(c"Failed to read max nodes, using default of %d\n".as_ptr(), max_node_num);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cpu__max_node() -> c_int {
    if unlikely(max_node_num == 0) {
        set_max_node_num();
    }

    max_node_num
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cpu__max_cpu() -> perf_cpu {
    if unlikely(max_cpu_num.cpu == 0) {
        set_max_cpu_num();
    }

    max_cpu_num
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cpu__max_present_cpu() -> perf_cpu {
    if unlikely(max_present_cpu_num.cpu == 0) {
        set_max_cpu_num();
    }

    max_present_cpu_num
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cpu__get_node(cpu: perf_cpu) -> c_int {
    if unlikely(cpunode_map.is_null()) {
        pr_debug(c"cpu_map not initialized\n".as_ptr());
        return -1;
    }

    /* cpunode_map allocated for max_cpu_num entries; input may be untrusted */
    if cpu.cpu < 0 || cpu.cpu >= max_cpu_num.cpu {
        return -1;
    }

    *cpunode_map.add(cpu.cpu as usize)
}

unsafe fn init_cpunode_map() -> c_int {
    set_max_cpu_num();
    set_max_node_num();

    cpunode_map = calloc(max_cpu_num.cpu as size_t, mem::size_of::<c_int>()) as *mut c_int;
    if cpunode_map.is_null() {
        pr_err(c"%s: calloc failed\n".as_ptr(), c"init_cpunode_map".as_ptr());
        return -1;
    }

    for i in 0..max_cpu_num.cpu as c_int {
        *cpunode_map.add(i as usize) = -1;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cpu__setup_cpunode_map() -> c_int {
    let mut dent1: *mut dirent;
    let mut dent2: *mut dirent;
    let dir1: *mut DIR;
    let mut dir2: *mut DIR;
    let mut cpu: c_uint = 0;
    let mut mem_: c_uint = 0;
    let mut buf = [0 as c_char; PATH_MAX];
    let mut path = [0 as c_char; PATH_MAX];
    let mnt: *const c_char;
    let mut n: c_int;

    /* initialize globals */
    if init_cpunode_map() != 0 {
        return -1;
    }

    mnt = sysfs__mountpoint();
    if mnt.is_null() {
        return 0;
    }

    n = snprintf(path.as_mut_ptr(), PATH_MAX, c"%s/devices/system/node".as_ptr(), mnt);
    if n >= PATH_MAX as c_int {
        pr_err(c"sysfs path crossed PATH_MAX(%d) size\n".as_ptr(), PATH_MAX as c_int);
        return -1;
    }

    dir1 = opendir(path.as_ptr());
    if dir1.is_null() {
        return 0;
    }

    /* walk tree and setup map */
    loop {
        dent1 = readdir(dir1);
        if dent1.is_null() {
            break;
        }
        if (*dent1).d_type != DT_DIR
            || sscanf((*dent1).d_name.as_ptr(), c"node%u".as_ptr(), &mut mem_) < 1
        {
            continue;
        }

        n = snprintf(buf.as_mut_ptr(), PATH_MAX, c"%s/%s".as_ptr(), path.as_ptr(), (*dent1).d_name.as_ptr());
        if n >= PATH_MAX as c_int {
            pr_err(c"sysfs path crossed PATH_MAX(%d) size\n".as_ptr(), PATH_MAX as c_int);
            continue;
        }

        dir2 = opendir(buf.as_ptr());
        if dir2.is_null() {
            continue;
        }
        loop {
            dent2 = readdir(dir2);
            if dent2.is_null() {
                break;
            }
            if (*dent2).d_type != DT_LNK
                || sscanf((*dent2).d_name.as_ptr(), c"cpu%u".as_ptr(), &mut cpu) < 1
            {
                continue;
            }
            /* cpunode_map allocated for max_cpu_num entries */
            if cpu < max_cpu_num.cpu as c_uint {
                *cpunode_map.add(cpu as usize) = mem_ as c_int;
            }
        }
        closedir(dir2);
    }
    closedir(dir1);
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cpu_map__snprint(
    map: *mut perf_cpu_map,
    buf: *mut c_char,
    size: size_t,
) -> size_t {
    let mut start: c_int = -1;
    let mut first = true;
    let mut ret: size_t = 0;

    let mut i: c_int = 0;
    while i < perf_cpu_map__nr(map) as c_int + 1 {
        let mut cpu = perf_cpu { cpu: INT16_MAX as i16 };
        let last = i == perf_cpu_map__nr(map) as c_int;

        if !last {
            cpu = perf_cpu_map__cpu(map, i);
        }

        if start == -1 {
            start = i;
            if last {
                ret += scnprintf(
                    buf.add(ret),
                    size - ret,
                    c"%s%d".as_ptr(),
                    if first { c"".as_ptr() } else { c",".as_ptr() },
                    perf_cpu_map__cpu(map, i).cpu as c_int,
                ) as size_t;
            }
        } else if (i - start) != (cpu.cpu as c_int - perf_cpu_map__cpu(map, start).cpu as c_int) || last {
            let end = i - 1;

            if start == end {
                ret += scnprintf(
                    buf.add(ret),
                    size - ret,
                    c"%s%d".as_ptr(),
                    if first { c"".as_ptr() } else { c",".as_ptr() },
                    perf_cpu_map__cpu(map, start).cpu as c_int,
                ) as size_t;
            } else {
                ret += scnprintf(
                    buf.add(ret),
                    size - ret,
                    c"%s%d-%d".as_ptr(),
                    if first { c"".as_ptr() } else { c",".as_ptr() },
                    perf_cpu_map__cpu(map, start).cpu as c_int,
                    perf_cpu_map__cpu(map, end).cpu as c_int,
                ) as size_t;
            }
            first = false;
            start = i;
        }
        i += 1;
    }

    pr_debug2(c"cpumask list: %s\n".as_ptr(), buf);
    ret
}

fn hex_char(val: u8) -> c_char {
    if val < 10 {
        (val + b'0') as c_char
    } else if val < 16 {
        (val - 10 + b'a') as c_char
    } else {
        b'?' as c_char
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cpu_map__snprint_mask(
    map: *mut perf_cpu_map,
    buf: *mut c_char,
    size: size_t,
) -> size_t {
    let mut ptr_ = buf;
    let bitmap: *mut u8;
    let last_cpu = perf_cpu_map__max(map);

    if buf.is_null() || size == 0 {
        return 0;
    }

    if last_cpu.cpu < 0 {
        *buf.add(0) = b'\0' as c_char;
        return 0;
    }

    bitmap = zalloc(last_cpu.cpu as size_t / 8 + 1) as *mut u8;
    if bitmap.is_null() {
        *buf.add(0) = b'\0' as c_char;
        return 0;
    }

    let mut idx: c_uint = 0;
    while idx < perf_cpu_map__nr(map) {
        let c = perf_cpu_map__cpu(map, idx as c_int);
        if c.cpu != -1 {
            *bitmap.add(c.cpu as usize / 8) |= (1u8 << (c.cpu as usize % 8)) as u8;
        }
        idx += 1;
    }

    let mut cpu = (last_cpu.cpu as c_int / 4) * 4;
    while cpu >= 0 {
        let mut bits = *bitmap.add(cpu as usize / 8);

        if cpu % 8 != 0 {
            bits >>= 4;
        } else {
            bits &= 0xf;
        }

        *ptr_ = hex_char(bits);
        ptr_ = ptr_.add(1);
        if (cpu % 32) == 0 && cpu > 0 {
            *ptr_ = b',' as c_char;
            ptr_ = ptr_.add(1);
        }
        cpu -= 4;
    }
    *ptr_ = b'\0' as c_char;
    free(bitmap as *mut c_void);

    *buf.add(size - 1) = b'\0' as c_char;
    ptr_.offset_from(buf) as size_t
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cpu_map__online() -> *mut perf_cpu_map {
    /* thread unsafe */
    static mut online: *mut perf_cpu_map = ptr::null_mut();

    if online.is_null() {
        online = perf_cpu_map__new_online_cpus(); /* from /sys/devices/system/cpu/online */
    }

    perf_cpu_map__get(online)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn aggr_cpu_id__equal(a: *const aggr_cpu_id, b: *const aggr_cpu_id) -> bool {
    (*a).thread_idx == (*b).thread_idx
        && (*a).node == (*b).node
        && (*a).socket == (*b).socket
        && (*a).die == (*b).die
        && (*a).cluster == (*b).cluster
        && (*a).cache_lvl == (*b).cache_lvl
        && (*a).cache == (*b).cache
        && (*a).core == (*b).core
        && (*a).cpu.cpu == (*b).cpu.cpu
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn aggr_cpu_id__is_empty(a: *const aggr_cpu_id) -> bool {
    (*a).thread_idx == -1
        && (*a).node == -1
        && (*a).socket == -1
        && (*a).die == -1
        && (*a).cluster == -1
        && (*a).cache_lvl == -1
        && (*a).cache == -1
        && (*a).core == -1
        && (*a).cpu.cpu == -1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn aggr_cpu_id__empty() -> aggr_cpu_id {
    let ret = aggr_cpu_id {
        thread_idx: -1,
        node: -1,
        socket: -1,
        die: -1,
        cluster: -1,
        cache_lvl: -1,
        cache: -1,
        core: -1,
        cpu: perf_cpu { cpu: -1 },
    };
    ret
}
