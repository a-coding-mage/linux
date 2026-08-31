// SPDX-License-Identifier: GPL-2.0-only
// Translated from lib/perf/cpumap.c. C includes are intentionally omitted;
// external types, functions, and constants come from the surrounding project.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const MAX_NR_CPUS: c_ulong = 4096;
const INT16_MAX: c_ulong = i16::MAX as c_ulong;
const ENOMEM: c_int = 12;
const _SC_NPROCESSORS_ONLN: c_int = 84;
const _SC_NPROCESSORS_CONF: c_int = 83;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_cpu {
    pub cpu: i16,
}

#[repr(C)]
pub struct refcount_t {
    pub refs: c_int,
}

#[repr(C)]
pub struct perf_cpu_map {
    pub refcnt: refcount_t,
    pub nr: c_uint,
    pub map: [perf_cpu; 0],
}

unsafe extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn qsort(
        base: *mut c_void,
        nmemb: usize,
        size: usize,
        compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int,
    );
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn sysconf(name: c_int) -> c_long;
    fn sysfs__read_str(path: *const c_char, buf: *mut *mut c_char, len: *mut usize) -> c_int;

    fn refcount_set(r: *mut refcount_t, n: c_uint);
    fn refcount_read(r: *const refcount_t) -> c_uint;
    fn refcount_inc(r: *mut refcount_t);
    fn refcount_dec_and_test(r: *mut refcount_t) -> bool;

    fn pr_warning(fmt: *const c_char, ...);
    fn WARN_ONCE(condition: bool, fmt: *const c_char, ...);
}

#[inline]
unsafe fn perf_cpu_map__refcnt(map: *const perf_cpu_map) -> *mut refcount_t {
    unsafe { &mut (*(map as *mut perf_cpu_map)).refcnt }
}

#[inline]
unsafe fn RC_CHK_ACCESS(map: *const perf_cpu_map) -> *mut perf_cpu_map {
    map as *mut perf_cpu_map
}

#[inline]
unsafe fn RC_CHK_FREE(map: *mut perf_cpu_map) {
    unsafe { free(map as *mut c_void) };
}

#[inline]
unsafe fn RC_CHK_PUT(_map: *mut perf_cpu_map) {}

#[inline]
unsafe fn map_ptr(map: *const perf_cpu_map) -> *mut perf_cpu {
    unsafe { (*(map as *mut perf_cpu_map)).map.as_mut_ptr() }
}

#[inline]
unsafe fn isdigit_c(ch: c_char) -> bool {
    ch >= b'0' as c_char && ch <= b'9' as c_char
}

#[inline]
fn max_ulong(a: c_ulong, b: c_ulong) -> c_ulong {
    if a > b { a } else { b }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_cpu_map__set_nr(map: *mut perf_cpu_map, nr_cpus: c_uint) {
    unsafe {
        (*RC_CHK_ACCESS(map)).nr = nr_cpus;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_cpu_map__alloc(nr_cpus: c_uint) -> *mut perf_cpu_map {
    let cpus: *mut perf_cpu_map;
    let result: *mut perf_cpu_map;

    if nr_cpus == 0 {
        return ptr::null_mut();
    }

    unsafe {
        cpus = malloc(size_of::<perf_cpu_map>() + size_of::<perf_cpu>() * nr_cpus as usize)
            as *mut perf_cpu_map;
        result = cpus;
        if !result.is_null() {
            (*cpus).nr = nr_cpus;
            refcount_set(&mut (*cpus).refcnt, 1);
        }
    }
    result
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_cpu_map__new_any_cpu() -> *mut perf_cpu_map {
    let cpus = unsafe { perf_cpu_map__alloc(1) };

    if !cpus.is_null() {
        unsafe {
            (*map_ptr(RC_CHK_ACCESS(cpus))).cpu = -1;
        }
    }

    cpus
}

unsafe fn cpu_map__delete(map: *mut perf_cpu_map) {
    if !map.is_null() {
        unsafe {
            WARN_ONCE(
                refcount_read(perf_cpu_map__refcnt(map)) != 0,
                c"cpu_map refcnt unbalanced\n".as_ptr(),
            );
            RC_CHK_FREE(map);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_cpu_map__get(map: *mut perf_cpu_map) -> *mut perf_cpu_map {
    let result = map;

    if !result.is_null() {
        unsafe {
            refcount_inc(perf_cpu_map__refcnt(map));
        }
    }

    result
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_cpu_map__put(map: *mut perf_cpu_map) {
    if !map.is_null() {
        unsafe {
            if refcount_dec_and_test(perf_cpu_map__refcnt(map)) {
                cpu_map__delete(map);
            } else {
                RC_CHK_PUT(map);
            }
        }
    }
}

unsafe fn cpu_map__new_sysconf() -> *mut perf_cpu_map {
    let cpus: *mut perf_cpu_map;
    let nr_cpus: c_long;
    let nr_cpus_conf: c_long;

    unsafe {
        nr_cpus = sysconf(_SC_NPROCESSORS_ONLN);
    }
    if nr_cpus < 0 {
        return ptr::null_mut();
    }

    unsafe {
        nr_cpus_conf = sysconf(_SC_NPROCESSORS_CONF);
        if nr_cpus != nr_cpus_conf {
            pr_warning(
                c"Number of online CPUs (%ld) differs from the number configured (%ld) the CPU map will only cover the first %ld CPUs.".as_ptr(),
                nr_cpus,
                nr_cpus_conf,
                nr_cpus,
            );
        }

        cpus = perf_cpu_map__alloc(nr_cpus as c_uint);
        if !cpus.is_null() {
            let mut i: c_long = 0;
            while i < nr_cpus {
                (*map_ptr(RC_CHK_ACCESS(cpus)).add(i as usize)).cpu = i as i16;
                i += 1;
            }
        }
    }

    cpus
}

unsafe fn cpu_map__new_sysfs_online() -> *mut perf_cpu_map {
    let mut cpus: *mut perf_cpu_map = ptr::null_mut();
    let mut buf: *mut c_char = ptr::null_mut();
    let mut buf_len: usize = 0;

    unsafe {
        if sysfs__read_str(c"devices/system/cpu/online".as_ptr(), &mut buf, &mut buf_len) >= 0 {
            cpus = perf_cpu_map__new(buf);
            free(buf as *mut c_void);
        }
    }
    cpus
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_cpu_map__new_online_cpus() -> *mut perf_cpu_map {
    let cpus = unsafe { cpu_map__new_sysfs_online() };

    if !cpus.is_null() {
        return cpus;
    }

    unsafe { cpu_map__new_sysconf() }
}

unsafe extern "C" fn cmp_cpu(a: *const c_void, b: *const c_void) -> c_int {
    let cpu_a = a as *const perf_cpu;
    let cpu_b = b as *const perf_cpu;

    unsafe { (*cpu_a).cpu as c_int - (*cpu_b).cpu as c_int }
}

unsafe fn __perf_cpu_map__cpu(cpus: *const perf_cpu_map, idx: c_uint) -> perf_cpu {
    unsafe { *map_ptr(RC_CHK_ACCESS(cpus)).add(idx as usize) }
}

unsafe fn cpu_map__trim_new(nr_cpus: c_uint, tmp_cpus: *const perf_cpu) -> *mut perf_cpu_map {
    let payload_size = nr_cpus as usize * size_of::<perf_cpu>();
    let cpus = unsafe { perf_cpu_map__alloc(nr_cpus) };

    if !cpus.is_null() {
        let mut j: c_uint = 0;

        unsafe {
            memcpy(
                map_ptr(RC_CHK_ACCESS(cpus)) as *mut c_void,
                tmp_cpus as *const c_void,
                payload_size,
            );
            qsort(
                map_ptr(RC_CHK_ACCESS(cpus)) as *mut c_void,
                nr_cpus as usize,
                size_of::<perf_cpu>(),
                cmp_cpu,
            );
            /* Remove dups */
            let mut i: c_uint = 0;
            while i < nr_cpus {
                if i == 0
                    || __perf_cpu_map__cpu(cpus, i).cpu
                        != __perf_cpu_map__cpu(cpus, i - 1).cpu
                {
                    (*map_ptr(RC_CHK_ACCESS(cpus)).add(j as usize)).cpu =
                        __perf_cpu_map__cpu(cpus, i).cpu;
                    j += 1;
                }
                i += 1;
            }
            perf_cpu_map__set_nr(cpus, j);
            assert!(j <= nr_cpus);
        }
    }
    cpus
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_cpu_map__new(cpu_list: *const c_char) -> *mut perf_cpu_map {
    let mut cpus: *mut perf_cpu_map = ptr::null_mut();
    let mut start_cpu: c_ulong;
    let mut end_cpu: c_ulong = 0;
    let mut p: *mut c_char = ptr::null_mut();
    let mut nr_cpus: c_uint = 0;
    let mut max_entries: c_uint = 0;
    let mut tmp_cpus: *mut perf_cpu = ptr::null_mut();
    let mut tmp: *mut perf_cpu;
    let mut cpu_list = cpu_list;
    let mut invalid = false;

    if cpu_list.is_null() {
        return unsafe { perf_cpu_map__new_online_cpus() };
    }

    /*
     * must handle the case of empty cpumap to cover
     * TOPOLOGY header for NUMA nodes with no CPU
     * ( e.g., because of CPU hotplug)
     */
    unsafe {
        if !isdigit_c(*cpu_list) && *cpu_list != 0 {
            return cpus;
        }
    }

    unsafe {
        while isdigit_c(*cpu_list) {
            p = ptr::null_mut();
            start_cpu = strtoul(cpu_list, &mut p, 0);
            if start_cpu >= INT16_MAX
                || (*p != 0 && *p != b',' as c_char && *p != b'-' as c_char && *p != b'\n' as c_char)
            {
                invalid = true;
                break;
            }

            if *p == b'-' as c_char {
                p = p.add(1);
                cpu_list = p;
                p = ptr::null_mut();
                end_cpu = strtoul(cpu_list, &mut p, 0);

                if end_cpu >= INT16_MAX
                    || (*p != 0 && *p != b',' as c_char && *p != b'\n' as c_char)
                {
                    invalid = true;
                    break;
                }

                if end_cpu < start_cpu {
                    invalid = true;
                    break;
                }
            } else {
                end_cpu = start_cpu;
            }

            WARN_ONCE(
                end_cpu >= MAX_NR_CPUS,
                c"Perf can support %d CPUs. Consider raising MAX_NR_CPUS\n".as_ptr(),
                MAX_NR_CPUS as c_int,
            );

            while start_cpu <= end_cpu {
                /* check for duplicates */
                let mut i: c_uint = 0;
                while i < nr_cpus {
                    if (*tmp_cpus.add(i as usize)).cpu == start_cpu as i16 {
                        invalid = true;
                        break;
                    }
                    i += 1;
                }
                if invalid {
                    break;
                }

                if nr_cpus == max_entries {
                    max_entries = max_entries.wrapping_add(
                        max_ulong(end_cpu - start_cpu + 1, 16) as c_uint,
                    );
                    tmp = realloc(
                        tmp_cpus as *mut c_void,
                        max_entries as usize * size_of::<perf_cpu>(),
                    ) as *mut perf_cpu;
                    if tmp.is_null() {
                        invalid = true;
                        break;
                    }
                    tmp_cpus = tmp;
                }
                (*tmp_cpus.add(nr_cpus as usize)).cpu = start_cpu as i16;
                nr_cpus += 1;
                start_cpu += 1;
            }
            if invalid {
                break;
            }
            if *p != 0 {
                p = p.add(1);
            }

            cpu_list = p;
        }

        if !invalid {
            if nr_cpus > 0 {
                cpus = cpu_map__trim_new(nr_cpus, tmp_cpus);
            } else if *cpu_list != 0 {
                pr_warning(
                    c"Unexpected characters at end of cpu list ('%s'), using online CPUs.".as_ptr(),
                    cpu_list,
                );
                cpus = perf_cpu_map__new_online_cpus();
            } else {
                cpus = perf_cpu_map__new_any_cpu();
            }
        }
        free(tmp_cpus as *mut c_void);
    }
    cpus
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_cpu_map__new_int(cpu: c_int) -> *mut perf_cpu_map {
    let cpus = unsafe { perf_cpu_map__alloc(1) };

    if !cpus.is_null() {
        unsafe {
            (*map_ptr(RC_CHK_ACCESS(cpus))).cpu = cpu as i16;
        }
    }

    cpus
}

unsafe fn __perf_cpu_map__nr(cpus: *const perf_cpu_map) -> c_uint {
    unsafe { (*RC_CHK_ACCESS(cpus)).nr }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_cpu_map__cpu(
    cpus: *const perf_cpu_map,
    idx: c_uint,
) -> perf_cpu {
    let result = perf_cpu { cpu: -1 };

    unsafe {
        if !cpus.is_null() && idx < __perf_cpu_map__nr(cpus) {
            return __perf_cpu_map__cpu(cpus, idx);
        }
    }

    result
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_cpu_map__nr(cpus: *const perf_cpu_map) -> c_uint {
    unsafe {
        if !cpus.is_null() {
            __perf_cpu_map__nr(cpus)
        } else {
            1
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_cpu_map__has_any_cpu_or_is_empty(
    map: *const perf_cpu_map,
) -> bool {
    unsafe {
        if !map.is_null() {
            __perf_cpu_map__cpu(map, 0).cpu == -1
        } else {
            true
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_cpu_map__is_any_cpu_or_is_empty(
    map: *const perf_cpu_map,
) -> bool {
    if map.is_null() {
        return true;
    }

    unsafe { __perf_cpu_map__nr(map) == 1 && __perf_cpu_map__cpu(map, 0).cpu == -1 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_cpu_map__is_empty(map: *const perf_cpu_map) -> bool {
    map.is_null()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_cpu_map__idx(
    cpus: *const perf_cpu_map,
    cpu: perf_cpu,
) -> c_int {
    let mut low: c_uint;
    let mut high: c_uint;

    if cpus.is_null() {
        return -1;
    }

    unsafe {
        low = 0;
        high = __perf_cpu_map__nr(cpus);
        while low < high {
            let idx: c_int = ((low + high) / 2) as c_int;
            let cpu_at_idx = __perf_cpu_map__cpu(cpus, idx as c_uint);

            if cpu_at_idx.cpu == cpu.cpu {
                return idx;
            }

            if cpu_at_idx.cpu > cpu.cpu {
                high = idx as c_uint;
            } else {
                low = idx as c_uint + 1;
            }
        }
    }

    -1
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_cpu_map__has(
    cpus: *const perf_cpu_map,
    cpu: perf_cpu,
) -> bool {
    unsafe { perf_cpu_map__idx(cpus, cpu) != -1 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_cpu_map__equal(
    lhs: *const perf_cpu_map,
    rhs: *const perf_cpu_map,
) -> bool {
    let nr: c_uint;

    if lhs == rhs {
        return true;
    }

    if lhs.is_null() || rhs.is_null() {
        return false;
    }

    unsafe {
        nr = __perf_cpu_map__nr(lhs);
        if nr != __perf_cpu_map__nr(rhs) {
            return false;
        }

        let mut idx: c_uint = 0;
        while idx < nr {
            if __perf_cpu_map__cpu(lhs, idx).cpu != __perf_cpu_map__cpu(rhs, idx).cpu {
                return false;
            }
            idx += 1;
        }
    }
    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_cpu_map__has_any_cpu(map: *const perf_cpu_map) -> bool {
    unsafe { !map.is_null() && __perf_cpu_map__cpu(map, 0).cpu == -1 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_cpu_map__min(map: *const perf_cpu_map) -> perf_cpu {
    let mut result = perf_cpu { cpu: -1 };
    let mut idx: c_uint;

    unsafe {
        if !map.is_null() {
            idx = 0;
            while idx < __perf_cpu_map__nr(map) {
                let cpu = __perf_cpu_map__cpu(map, idx);
                if cpu.cpu != -1 {
                    result = cpu;
                    break;
                }
                idx += 1;
            }
        }
    }
    result
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_cpu_map__max(map: *const perf_cpu_map) -> perf_cpu {
    let result = perf_cpu { cpu: -1 };

    if map.is_null() {
        return result;
    }

    // The CPUs are always sorted and nr is always > 0 as 0 length map is
    // encoded as NULL.
    unsafe { __perf_cpu_map__cpu(map, __perf_cpu_map__nr(map) - 1) }
}

/** Is 'b' a subset of 'a'. */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_cpu_map__is_subset(
    a: *const perf_cpu_map,
    b: *const perf_cpu_map,
) -> bool {
    unsafe {
        if a == b || b.is_null() {
            return true;
        }
        if a.is_null() || __perf_cpu_map__nr(b) > __perf_cpu_map__nr(a) {
            return false;
        }

        let mut i: c_uint = 0;
        let mut j: c_uint = 0;
        while i < __perf_cpu_map__nr(a) {
            if __perf_cpu_map__cpu(a, i).cpu > __perf_cpu_map__cpu(b, j).cpu {
                return false;
            }
            if __perf_cpu_map__cpu(a, i).cpu == __perf_cpu_map__cpu(b, j).cpu {
                j += 1;
                if j == __perf_cpu_map__nr(b) {
                    return true;
                }
            }
            i += 1;
        }
    }
    false
}

/*
 * Merge two cpumaps.
 *
 * If 'other' is subset of '*orig', '*orig' keeps itself with no reference count
 * change (similar to "realloc").
 *
 * If '*orig' is subset of 'other', '*orig' reuses 'other' with its reference
 * count increased.
 *
 * Otherwise, '*orig' gets freed and replaced with a new map.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_cpu_map__merge(
    orig: *mut *mut perf_cpu_map,
    other: *mut perf_cpu_map,
) -> c_int {
    let mut tmp_cpus: *mut perf_cpu;
    let tmp_len: c_uint;
    let mut i: c_uint;
    let mut j: c_uint;
    let mut k: c_uint;
    let merged: *mut perf_cpu_map;

    unsafe {
        if perf_cpu_map__is_subset(*orig, other) {
            return 0;
        }
        if perf_cpu_map__is_subset(other, *orig) {
            perf_cpu_map__put(*orig);
            *orig = perf_cpu_map__get(other);
            return 0;
        }

        tmp_len = __perf_cpu_map__nr(*orig) + __perf_cpu_map__nr(other);
        tmp_cpus = malloc(tmp_len as usize * size_of::<perf_cpu>()) as *mut perf_cpu;
        if tmp_cpus.is_null() {
            return -ENOMEM;
        }

        /* Standard merge algorithm from wikipedia */
        i = 0;
        j = 0;
        k = 0;
        while i < __perf_cpu_map__nr(*orig) && j < __perf_cpu_map__nr(other) {
            if __perf_cpu_map__cpu(*orig, i).cpu <= __perf_cpu_map__cpu(other, j).cpu {
                if __perf_cpu_map__cpu(*orig, i).cpu == __perf_cpu_map__cpu(other, j).cpu {
                    j += 1;
                }
                *tmp_cpus.add(k as usize) = __perf_cpu_map__cpu(*orig, i);
                k += 1;
                i += 1;
            } else {
                *tmp_cpus.add(k as usize) = __perf_cpu_map__cpu(other, j);
                k += 1;
                j += 1;
            }
        }

        while i < __perf_cpu_map__nr(*orig) {
            *tmp_cpus.add(k as usize) = __perf_cpu_map__cpu(*orig, i);
            k += 1;
            i += 1;
        }

        while j < __perf_cpu_map__nr(other) {
            *tmp_cpus.add(k as usize) = __perf_cpu_map__cpu(other, j);
            k += 1;
            j += 1;
        }
        assert!(k <= tmp_len);

        merged = cpu_map__trim_new(k, tmp_cpus);
        free(tmp_cpus as *mut c_void);
        perf_cpu_map__put(*orig);
        *orig = merged;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_cpu_map__intersect(
    orig: *mut perf_cpu_map,
    other: *mut perf_cpu_map,
) -> *mut perf_cpu_map {
    let mut i: c_uint;
    let mut j: c_uint;
    let mut k: c_uint;
    let merged: *mut perf_cpu_map;

    unsafe {
        if perf_cpu_map__is_subset(other, orig) {
            return perf_cpu_map__get(orig);
        }
        if perf_cpu_map__is_subset(orig, other) {
            return perf_cpu_map__get(other);
        }

        i = 0;
        j = 0;
        k = 0;
        while i < __perf_cpu_map__nr(orig) && j < __perf_cpu_map__nr(other) {
            if __perf_cpu_map__cpu(orig, i).cpu < __perf_cpu_map__cpu(other, j).cpu {
                i += 1;
            } else if __perf_cpu_map__cpu(orig, i).cpu > __perf_cpu_map__cpu(other, j).cpu {
                j += 1;
            } else {
                /* CPUs match. */
                i += 1;
                j += 1;
                k += 1;
            }
        }
        if k == 0 {
            /* Maps are completely disjoint. */
            return ptr::null_mut();
        }

        merged = perf_cpu_map__alloc(k);
        if merged.is_null() {
            return ptr::null_mut();
        }
        /* Entries are added to merged in sorted order, so no need to sort again. */
        i = 0;
        j = 0;
        k = 0;
        while i < __perf_cpu_map__nr(orig) && j < __perf_cpu_map__nr(other) {
            if __perf_cpu_map__cpu(orig, i).cpu < __perf_cpu_map__cpu(other, j).cpu {
                i += 1;
            } else if __perf_cpu_map__cpu(orig, i).cpu > __perf_cpu_map__cpu(other, j).cpu {
                j += 1;
            } else {
                j += 1;
                *map_ptr(RC_CHK_ACCESS(merged)).add(k as usize) =
                    __perf_cpu_map__cpu(orig, i);
                k += 1;
                i += 1;
            }
        }
    }
    merged
}
