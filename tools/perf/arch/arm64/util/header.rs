use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};

const MIDR: &[u8] = b"/regs/identification/midr_el1\0";
const MIDR_SIZE: usize = 19;
const MIDR_REVISION_MASK: u64 = genmask(3, 0);
const MIDR_VARIANT_MASK: u64 = genmask(23, 20);

const EINVAL: c_int = 22;
const PATH_MAX: usize = 4096;

#[repr(C)]
pub struct perf_cpu {
    pub cpu: c_int,
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

const fn genmask(h: u32, l: u32) -> u64 {
    (!0u64 << l) & (!0u64 >> (63 - h))
}

const fn field_shift(mask: u64) -> u32 {
    mask.trailing_zeros()
}

fn field_get(mask: u64, reg: u64) -> u64 {
    (reg & mask) >> field_shift(mask)
}

unsafe extern "C" {
    fn sysfs__mountpoint() -> *const c_char;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn fclose(stream: *mut FILE) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulonglong;
    fn perf_cpu_map__new_online_cpus() -> *mut perf_cpu_map;
    fn perf_cpu_map__nr(cpus: *const perf_cpu_map) -> c_int;
    fn perf_cpu_map__cpu(cpus: *const perf_cpu_map, idx: c_int) -> perf_cpu;
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
}

unsafe fn _get_cpuid(buf: *mut c_char, sz: usize, cpu: perf_cpu) -> c_int {
    let mut path = [0 as c_char; PATH_MAX];
    let mut file: *mut FILE;
    let sysfs = unsafe { sysfs__mountpoint() };

    assert!(cpu.cpu != -1);
    if sysfs.is_null() || sz < MIDR_SIZE {
        return EINVAL;
    }

    unsafe {
        scnprintf(
            path.as_mut_ptr(),
            PATH_MAX,
            b"%s/devices/system/cpu/cpu%d%s\0".as_ptr() as *const c_char,
            sysfs,
            cpu.cpu,
            MIDR.as_ptr() as *const c_char,
        );
    }

    file = unsafe { fopen(path.as_ptr(), b"r\0".as_ptr() as *const c_char) };
    if file.is_null() {
        unsafe {
            pr_debug(
                b"fopen failed for file %s\n\0".as_ptr() as *const c_char,
                path.as_ptr(),
            );
        }
        return EINVAL;
    }

    if unsafe { fgets(buf, MIDR_SIZE as c_int, file) }.is_null() {
        unsafe {
            pr_debug(
                b"Failed to read file %s\n\0".as_ptr() as *const c_char,
                path.as_ptr(),
            );
            fclose(file);
        }
        return EINVAL;
    }
    unsafe {
        fclose(file);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_cpuid(buf: *mut c_char, sz: usize, mut cpu: perf_cpu) -> c_int {
    let cpus: *mut perf_cpu_map;
    let mut idx: c_uint;

    if cpu.cpu != -1 {
        return unsafe { _get_cpuid(buf, sz, cpu) };
    }

    cpus = unsafe { perf_cpu_map__new_online_cpus() };
    if cpus.is_null() {
        return EINVAL;
    }

    idx = 0;
    while (idx as c_int) < unsafe { perf_cpu_map__nr(cpus) } {
        cpu = unsafe { perf_cpu_map__cpu(cpus, idx as c_int) };
        let ret = unsafe { _get_cpuid(buf, sz, cpu) };

        if ret == 0 {
            return 0;
        }
        idx += 1;
    }
    EINVAL
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_cpuid_str(cpu: perf_cpu) -> *mut c_char {
    let mut buf = unsafe { malloc(MIDR_SIZE) as *mut c_char };
    let res: c_int;

    if buf.is_null() {
        return core::ptr::null_mut();
    }

    /* read midr from list of cpus mapped to this pmu */
    res = unsafe { get_cpuid(buf, MIDR_SIZE, cpu) };
    if res != 0 {
        unsafe {
            pr_err(
                b"failed to get cpuid string for CPU %d\n\0".as_ptr() as *const c_char,
                cpu.cpu,
            );
            free(buf as *mut c_void);
        }
        buf = core::ptr::null_mut();
    }

    buf
}

/*
 * Return 0 if idstr is a higher or equal to version of the same part as
 * mapcpuid. Therefore, if mapcpuid has 0 for revision and variant then any
 * version of idstr will match as long as it's the same CPU type.
 *
 * Return 1 if the CPU type is different or the version of idstr is lower.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn strcmp_cpuid_str(
    mapcpuid: *const c_char,
    idstr: *const c_char,
) -> c_int {
    let map_id = unsafe { strtoull(mapcpuid, core::ptr::null_mut(), 16) as u64 };
    let map_id_variant = field_get(MIDR_VARIANT_MASK, map_id) as c_char;
    let map_id_revision = field_get(MIDR_REVISION_MASK, map_id) as c_char;
    let id = unsafe { strtoull(idstr, core::ptr::null_mut(), 16) as u64 };
    let id_variant = field_get(MIDR_VARIANT_MASK, id) as c_char;
    let id_revision = field_get(MIDR_REVISION_MASK, id) as c_char;
    let id_fields = !(MIDR_VARIANT_MASK | MIDR_REVISION_MASK);

    /* Compare without version first */
    if (map_id & id_fields) != (id & id_fields) {
        return 1;
    }

    /*
     * ID matches, now compare version.
     *
     * Arm revisions (like r0p0) are compared here like two digit semver
     * values eg. 1.3 < 2.0 < 2.1 < 2.2.
     *
     *  r = high value = 'Variant' field in MIDR
     *  p = low value  = 'Revision' field in MIDR
     *
     */
    if id_variant > map_id_variant {
        return 0;
    }

    if id_variant == map_id_variant && id_revision >= map_id_revision {
        return 0;
    }

    /*
     * variant is less than mapfile variant or variants are the same but
     * the revision doesn't match. Return no match.
     */
    1
}
