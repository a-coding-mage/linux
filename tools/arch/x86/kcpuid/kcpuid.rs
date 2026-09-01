// SPDX-License-Identifier: GPL-2.0

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(static_mut_refs)]

use core::arch::x86_64::__cpuid_count;
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type u32 = c_uint;
type u64 = u64;
type size_t = usize;
type ssize_t = isize;

const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;

const no_argument: c_int = 0;
const required_argument: c_int = 1;

const R_EAX: usize = 0;
const R_EBX: usize = 1;
const R_ECX: usize = 2;
const R_EDX: usize = 3;
const NR_REGS: usize = 4;

const RANGE_STD: u32 = 0;
const RANGE_EXT: u32 = 0x80000000;
const RANGE_TSM: u32 = 0x80860000;
const RANGE_CTR: u32 = 0xc0000000;

const CPUID_INDEX_MASK: u32 = 0xffff0000;
const CPUID_FUNCTION_MASK: u32 = !CPUID_INDEX_MASK;

const MAX_SUBLEAF_NUM: u32 = 64;
const MAX_RANGE_INDEX_OFFSET: u32 = 0xff;

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
struct option {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct bits_desc {
    /* start and end bits */
    start: c_int,
    end: c_int,
    /* 0 or 1 for 1-bit flag */
    value: c_int,
    simp: [c_char; 32],
    detail: [c_char; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct reg_desc {
    /* number of valid entries */
    nr: c_int,
    descs: [bits_desc; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
struct subleaf {
    index: u32,
    sub: u32,
    output: [u32; NR_REGS],
    info: [reg_desc; NR_REGS],
}

#[repr(C)]
struct cpuid_func {
    /*
     * Array of subleafs for this func, if there is no subleafs
     * then the leafs[0] is the main leaf
     */
    leafs: *mut subleaf,
    nr: c_int,
}

#[repr(C)]
struct cpuid_range {
    /* array of main leafs */
    funcs: *mut cpuid_func,
    /* number of valid leafs */
    nr: c_int,
    index: u32,
}

unsafe extern "C" {
    static mut optarg: *mut c_char;

    fn err(eval: c_int, fmt: *const c_char, ...) -> !;
    fn errx(eval: c_int, fmt: *const c_char, ...) -> !;
    fn warnx(fmt: *const c_char, ...);
    fn getopt_long(
        argc: c_int,
        argv: *mut *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strtok(str: *mut c_char, delim: *const c_char) -> *mut c_char;
    fn strcasestr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> u64;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t;
    fn feof(stream: *mut FILE) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
}

static mut def_csv: *mut c_char = c"/usr/share/misc/cpuid.csv".as_ptr() as *mut c_char;
static mut user_csv: *mut c_char = ptr::null_mut();

static reg_names: [*const c_char; NR_REGS] = [
    c"EAX".as_ptr(),
    c"EBX".as_ptr(),
    c"ECX".as_ptr(),
    c"EDX".as_ptr(),
];

static mut ranges: [cpuid_range; 4] = [
    cpuid_range {
        funcs: ptr::null_mut(),
        nr: 0,
        index: RANGE_STD,
    },
    cpuid_range {
        funcs: ptr::null_mut(),
        nr: 0,
        index: RANGE_EXT,
    },
    cpuid_range {
        funcs: ptr::null_mut(),
        nr: 0,
        index: RANGE_TSM,
    },
    cpuid_range {
        funcs: ptr::null_mut(),
        nr: 0,
        index: RANGE_CTR,
    },
];

static mut show_details: bool = false;
static mut show_raw: bool = false;
static mut show_flags_only: bool = true;
static mut user_index: u32 = 0xFFFFFFFF;
static mut user_sub: u32 = 0xFFFFFFFF;
static mut flines: c_int = 0;

unsafe fn range_to_str(range: *mut cpuid_range) -> *const c_char {
    match (*range).index {
        RANGE_STD => c"Standard".as_ptr(),
        RANGE_EXT => c"Extended".as_ptr(),
        RANGE_TSM => c"Transmeta".as_ptr(),
        RANGE_CTR => c"Centaur".as_ptr(),
        _ => ptr::null(),
    }
}

unsafe fn index_to_cpuid_range(index: u32) -> *mut cpuid_range {
    let func_idx = index & CPUID_FUNCTION_MASK;
    let range_idx = index & CPUID_INDEX_MASK;

    let mut i = 0usize;
    while i < ranges.len() {
        let range = ranges.as_mut_ptr().add(i);
        if (*range).nr != 0 && (*range).index == range_idx && ((*range).nr as u32) > func_idx {
            return range;
        }
        i += 1;
    }

    ptr::null_mut()
}

unsafe fn cpuid(leaf: u32, a: *mut u32, b: *mut u32, c: *mut u32, d: *mut u32) {
    let r = __cpuid_count(leaf, 0);
    *a = r.eax;
    *b = r.ebx;
    *c = r.ecx;
    *d = r.edx;
}

unsafe fn cpuid_count(leaf: u32, subleaf: u32, a: *mut u32, b: *mut u32, c: *mut u32, d: *mut u32) {
    let r = __cpuid_count(leaf, subleaf);
    *a = r.eax;
    *b = r.ebx;
    *c = r.ecx;
    *d = r.edx;
}

unsafe fn has_subleafs(f: u32) -> bool {
    let with_subleaves: [u32; 17] = [
        0x4, 0x7, 0xb, 0xd, 0xf, 0x10, 0x12, 0x14, 0x17, 0x18, 0x1b, 0x1d, 0x1f, 0x23,
        0x8000001d, 0x80000020, 0x80000026,
    ];

    let mut i = 0usize;
    while i < with_subleaves.len() {
        if f == with_subleaves[i] {
            return true;
        }
        i += 1;
    }

    false
}

unsafe fn leaf_print_raw(leaf: *mut subleaf) {
    if has_subleafs((*leaf).index) {
        if (*leaf).sub == 0 {
            printf(c"0x%08x: subleafs:\n".as_ptr(), (*leaf).index);
        }

        printf(
            c" %2d: EAX=0x%08x, EBX=0x%08x, ECX=0x%08x, EDX=0x%08x\n".as_ptr(),
            (*leaf).sub,
            (*leaf).output[0],
            (*leaf).output[1],
            (*leaf).output[2],
            (*leaf).output[3],
        );
    } else {
        printf(
            c"0x%08x: EAX=0x%08x, EBX=0x%08x, ECX=0x%08x, EDX=0x%08x\n".as_ptr(),
            (*leaf).index,
            (*leaf).output[0],
            (*leaf).output[1],
            (*leaf).output[2],
            (*leaf).output[3],
        );
    }
}

/* Return true is the input eax/ebx/ecx/edx are all zero */
unsafe fn cpuid_store(range: *mut cpuid_range, f: u32, subleaf_: c_int, a: u32, b: u32, c: u32, d: u32) -> bool {
    let mut s: c_int = 0;

    if a == 0 && b == 0 && c == 0 && d == 0 {
        return true;
    }

    /*
     * Cut off vendor-prefix from CPUID function as we're using it as an
     * index into ->funcs.
     */
    let func = (*range).funcs.add((f & CPUID_FUNCTION_MASK) as usize);

    if (*func).leafs.is_null() {
        (*func).leafs = malloc(size_of::<subleaf>()) as *mut subleaf;
        if (*func).leafs.is_null() {
            err(EXIT_FAILURE, ptr::null());
        }

        (*func).nr = 1;
    } else {
        s = (*func).nr;
        (*func).leafs = realloc(
            (*func).leafs as *mut c_void,
            ((s + 1) as usize) * size_of::<subleaf>(),
        ) as *mut subleaf;
        if (*func).leafs.is_null() {
            err(EXIT_FAILURE, ptr::null());
        }

        (*func).nr += 1;
    }

    let leaf = (*func).leafs.add(s as usize);
    memset(leaf as *mut c_void, 0, size_of::<subleaf>());

    (*leaf).index = f;
    (*leaf).sub = subleaf_ as u32;
    (*leaf).output[R_EAX] = a;
    (*leaf).output[R_EBX] = b;
    (*leaf).output[R_ECX] = c;
    (*leaf).output[R_EDX] = d;

    false
}

unsafe fn raw_dump_range(range: *mut cpuid_range) {
    printf(c"%s Leafs :\n".as_ptr(), range_to_str(range));
    printf(c"================\n".as_ptr());

    let mut f: u32 = 0;
    while (f as c_int) < (*range).nr {
        let func = (*range).funcs.add(f as usize);

        /* Skip leaf without valid items */
        if (*func).nr == 0 {
            f += 1;
            continue;
        }

        /* First item is the main leaf, followed by all subleafs */
        let mut i: c_int = 0;
        while i < (*func).nr {
            leaf_print_raw((*func).leafs.add(i as usize));
            i += 1;
        }
        f += 1;
    }
}

unsafe fn setup_cpuid_range(range: *mut cpuid_range) {
    let mut max_func: u32 = 0;
    let mut range_funcs_sz: u32;
    let mut eax: u32 = 0;
    let mut ebx: u32 = 0;
    let mut ecx: u32 = 0;
    let mut edx: u32 = 0;

    cpuid((*range).index, &mut max_func, &mut ebx, &mut ecx, &mut edx);

    /*
     * If the CPUID range's maximum function value is garbage, then it
     * is not recognized by this CPU.  Set the range's number of valid
     * leaves to zero so that for_each_valid_cpu_range() can ignore it.
     */
    if max_func < (*range).index || max_func > (*range).index.wrapping_add(MAX_RANGE_INDEX_OFFSET) {
        (*range).nr = 0;
        return;
    }

    (*range).nr = ((max_func & CPUID_FUNCTION_MASK) + 1) as c_int;
    range_funcs_sz = ((*range).nr as u32).wrapping_mul(size_of::<cpuid_func>() as u32);

    (*range).funcs = malloc(range_funcs_sz as size_t) as *mut cpuid_func;
    if (*range).funcs.is_null() {
        err(EXIT_FAILURE, ptr::null());
    }

    memset((*range).funcs as *mut c_void, 0, range_funcs_sz as size_t);

    let mut f = (*range).index;
    while f <= max_func {
        let mut max_subleaf: u32 = MAX_SUBLEAF_NUM;

        cpuid(f, &mut eax, &mut ebx, &mut ecx, &mut edx);

        let mut allzero = cpuid_store(range, f, 0, eax, ebx, ecx, edx);
        if allzero {
            f = f.wrapping_add(1);
            continue;
        }

        if !has_subleafs(f) {
            f = f.wrapping_add(1);
            continue;
        }

        /*
         * Some can provide the exact number of subleafs,
         * others have to be tried (0xf)
         */
        if f == 0x7 || f == 0x14 || f == 0x17 || f == 0x18 || f == 0x1d {
            max_subleaf = core::cmp::min((eax & 0xff) + 1, max_subleaf);
        }
        if f == 0xb {
            max_subleaf = 2;
        }
        if f == 0x1f {
            max_subleaf = 6;
        }
        if f == 0x23 {
            max_subleaf = 4;
        }
        if f == 0x80000020 {
            max_subleaf = 4;
        }
        if f == 0x80000026 {
            max_subleaf = 5;
        }

        let mut subleaf_: u32 = 1;
        while subleaf_ < max_subleaf {
            cpuid_count(f, subleaf_, &mut eax, &mut ebx, &mut ecx, &mut edx);

            allzero = cpuid_store(range, f, subleaf_ as c_int, eax, ebx, ecx, edx);
            if allzero {
                subleaf_ += 1;
                continue;
            }
            subleaf_ += 1;
        }

        f = f.wrapping_add(1);
    }
}

/*
 * The basic row format for cpuid.csv  is
 *	LEAF,SUBLEAF,register_name,bits,short name,long description
 *
 * like:
 *	0,    0,  EAX,   31:0, max_basic_leafs,  Max input value for supported subleafs
 *	1,    0,  ECX,      0, sse3,  Streaming SIMD Extensions 3(SSE3)
 */
unsafe fn parse_line(line: *mut c_char) {
    let mut str_: *mut c_char;
    let mut buffer: [c_char; 512] = [0; 512];
    let mut tokens: [*mut c_char; 6] = [ptr::null_mut(); 6];
    let mut start: *mut c_char;
    let mut end: *mut c_char;

    /* Skip comments and NULL line */
    if *line == b'#' as c_char || *line == b'\n' as c_char {
        return;
    }

    strncpy(buffer.as_mut_ptr(), line, 511);
    buffer[511] = 0;
    str_ = buffer.as_mut_ptr();
    let mut i: c_int = 0;
    while i < 5 {
        tokens[i as usize] = strtok(str_, c",".as_ptr());
        if tokens[i as usize].is_null() {
            warnx(c"Wrong line format:\n\tline[%d]: %s".as_ptr(), flines, line);
            return;
        }
        str_ = ptr::null_mut();
        i += 1;
    }
    tokens[5] = strtok(str_, c"\n".as_ptr());
    if tokens[5].is_null() {
        warnx(c"Wrong line format:\n\tline[%d]: %s".as_ptr(), flines, line);
        return;
    }

    /* index/main-leaf */
    let mut index = strtoull(tokens[0], ptr::null_mut(), 0) as u32;

    /*
     * Skip line parsing if the index is not covered by known-valid
     * CPUID ranges on this CPU.
     */
    let range = index_to_cpuid_range(index);
    if range.is_null() {
        return;
    }

    /* Skip line parsing if the index CPUID output is all zero */
    index &= CPUID_FUNCTION_MASK;
    let func = (*range).funcs.add(index as usize);
    if (*func).nr == 0 {
        return;
    }

    /* subleaf */
    let mut buf = tokens[1];
    end = strtok(buf, c":".as_ptr());
    start = strtok(ptr::null_mut(), c":".as_ptr());
    let mut subleaf_end = strtoul(end, ptr::null_mut(), 0) as u32;

    let subleaf_start: u32;
    /* A subleaf range is given? */
    if !start.is_null() {
        subleaf_start = strtoul(start, ptr::null_mut(), 0) as u32;
        subleaf_end = core::cmp::min(subleaf_end, ((*func).nr - 1) as u32);
        if subleaf_start > subleaf_end {
            return;
        }
    } else {
        subleaf_start = subleaf_end;
        if subleaf_start > ((*func).nr - 1) as u32 {
            return;
        }
    }

    /* register */
    buf = tokens[2];
    let reg_index: usize;
    if !strcasestr(buf, c"EAX".as_ptr()).is_null() {
        reg_index = R_EAX;
    } else if !strcasestr(buf, c"EBX".as_ptr()).is_null() {
        reg_index = R_EBX;
    } else if !strcasestr(buf, c"ECX".as_ptr()).is_null() {
        reg_index = R_ECX;
    } else if !strcasestr(buf, c"EDX".as_ptr()).is_null() {
        reg_index = R_EDX;
    } else {
        warnx(c"Wrong line format:\n\tline[%d]: %s".as_ptr(), flines, line);
        return;
    }

    /* bit flag or bits field */
    buf = tokens[3];
    end = strtok(buf, c":".as_ptr());
    start = strtok(ptr::null_mut(), c":".as_ptr());
    let bit_end = strtoul(end, ptr::null_mut(), 0) as c_uint;
    let bit_start = if !start.is_null() {
        strtoul(start, ptr::null_mut(), 0) as c_uint
    } else {
        bit_end
    };

    let mut sub = subleaf_start;
    while sub <= subleaf_end {
        let leaf = (*func).leafs.add(sub as usize);
        let reg = &mut (*leaf).info[reg_index] as *mut reg_desc;
        let bdesc = &mut (*reg).descs[(*reg).nr as usize] as *mut bits_desc;
        (*reg).nr += 1;

        (*bdesc).end = bit_end as c_int;
        (*bdesc).start = bit_start as c_int;
        strcpy((*bdesc).simp.as_mut_ptr(), strtok(tokens[4], c" \t".as_ptr()));
        strcpy((*bdesc).detail.as_mut_ptr(), tokens[5]);
        sub += 1;
    }
}

/* Parse csv file, and construct the array of all leafs and subleafs */
unsafe fn parse_text() {
    let mut line: *mut c_char = ptr::null_mut();
    let mut len: size_t = 0;

    if show_raw {
        return;
    }

    let filename = if !user_csv.is_null() { user_csv } else { def_csv };
    let mut file = fopen(filename, c"r".as_ptr());
    if file.is_null() {
        /* Fallback to a csv in the same dir */
        file = fopen(c"./cpuid.csv".as_ptr(), c"r".as_ptr());
    }

    if file.is_null() {
        err(EXIT_FAILURE, c"%s".as_ptr(), filename);
    }

    loop {
        let ret = getline(&mut line, &mut len, file);
        flines += 1;
        if ret > 0 {
            parse_line(line);
        }

        if feof(file) != 0 {
            break;
        }
    }

    fclose(file);
}

unsafe fn show_reg(rdesc: *const reg_desc, value: u32) {
    let mut i: c_int = 0;
    while i < (*rdesc).nr {
        let bdesc = &(*rdesc).descs[i as usize] as *const bits_desc;

        let start = (*bdesc).start;
        let end = (*bdesc).end;
        if start == end {
            /* single bit flag */
            if (value & (1u32 << start)) != 0 {
                printf(
                    c"\t%-20s %s%s%s\n".as_ptr(),
                    (*bdesc).simp.as_ptr(),
                    if show_flags_only { c"".as_ptr() } else { c"\t\t\t".as_ptr() },
                    if show_details { c"-".as_ptr() } else { c"".as_ptr() },
                    if show_details { (*bdesc).detail.as_ptr() } else { c"".as_ptr() },
                );
            }
        } else {
            /* bit fields */
            if show_flags_only {
                i += 1;
                continue;
            }

            let mask = ((1u64 << (end - start + 1)) - 1) as u32;
            printf(
                c"\t%-20s\t: 0x%-8x\t%s%s\n".as_ptr(),
                (*bdesc).simp.as_ptr(),
                (value >> start) & mask,
                if show_details { c"-".as_ptr() } else { c"".as_ptr() },
                if show_details { (*bdesc).detail.as_ptr() } else { c"".as_ptr() },
            );
        }
        i += 1;
    }
}

unsafe fn show_reg_header(has_entries: bool, leaf: u32, subleaf_: u32, reg_name: *const c_char) {
    if show_details && has_entries {
        printf(c"CPUID_0x%x_%s[0x%x]:\n".as_ptr(), leaf, reg_name, subleaf_);
    }
}

unsafe fn show_leaf(leaf: *mut subleaf) {
    if show_raw {
        leaf_print_raw(leaf);
    }

    let mut i = R_EAX;
    while i < NR_REGS {
        show_reg_header((*leaf).info[i].nr > 0, (*leaf).index, (*leaf).sub, reg_names[i]);
        show_reg(&(*leaf).info[i] as *const reg_desc, (*leaf).output[i]);
        i += 1;
    }

    if !show_raw && show_details {
        printf(c"\n".as_ptr());
    }
}

unsafe fn show_func(func: *mut cpuid_func) {
    let mut i: c_int = 0;
    while i < (*func).nr {
        show_leaf((*func).leafs.add(i as usize));
        i += 1;
    }
}

unsafe fn show_range(range: *mut cpuid_range) {
    let mut i: c_int = 0;
    while i < (*range).nr {
        show_func((*range).funcs.add(i as usize));
        i += 1;
    }
}

unsafe fn index_to_func(index: u32) -> *mut cpuid_func {
    let func_idx = index & CPUID_FUNCTION_MASK;
    let range = index_to_cpuid_range(index);
    if range.is_null() {
        return ptr::null_mut();
    }

    (*range).funcs.add(func_idx as usize)
}

unsafe fn show_info() {
    if show_raw {
        /* Show all of the raw output of 'cpuid' instr */
        let mut i = 0usize;
        while i < ranges.len() {
            let range = ranges.as_mut_ptr().add(i);
            if (*range).nr != 0 {
                raw_dump_range(range);
            }
            i += 1;
        }
        return;
    }

    if user_index != 0xFFFFFFFF {
        /* Only show specific leaf/subleaf info */
        let func = index_to_func(user_index);
        if func.is_null() {
            errx(EXIT_FAILURE, c"Invalid input leaf (0x%x)".as_ptr(), user_index);
        }

        /* Dump the raw data also */
        show_raw = true;

        if user_sub != 0xFFFFFFFF {
            if user_sub + 1 > (*func).nr as u32 {
                errx(
                    EXIT_FAILURE,
                    c"Leaf 0x%x has no valid subleaf = 0x%x".as_ptr(),
                    user_index,
                    user_sub,
                );
            }

            show_leaf((*func).leafs.add(user_sub as usize));
            return;
        }

        show_func(func);
        return;
    }

    printf(c"CPU features:\n=============\n\n".as_ptr());
    let mut i = 0usize;
    while i < ranges.len() {
        let range = ranges.as_mut_ptr().add(i);
        if (*range).nr != 0 {
            show_range(range);
        }
        i += 1;
    }
}

unsafe fn usage(exit_code: c_int) -> ! {
    errx(
        exit_code,
        c"kcpuid [-abdfhr] [-l leaf] [-s subleaf]\n\t-a|--all             Show both bit flags and complex bit fields info\n\t-b|--bitflags        Show boolean flags only\n\t-d|--detail          Show details of the flag/fields (default)\n\t-f|--flags           Specify the CPUID CSV file\n\t-h|--help            Show usage info\n\t-l|--leaf=index      Specify the leaf you want to check\n\t-r|--raw             Show raw CPUID data\n\t-s|--subleaf=sub     Specify the subleaf you want to check".as_ptr(),
    );
}

static opts: [option; 9] = [
    option {
        name: c"all".as_ptr(),
        has_arg: no_argument,
        flag: ptr::null_mut(),
        val: b'a' as c_int,
    }, /* show both bit flags and fields */
    option {
        name: c"bitflags".as_ptr(),
        has_arg: no_argument,
        flag: ptr::null_mut(),
        val: b'b' as c_int,
    }, /* only show bit flags, default on */
    option {
        name: c"detail".as_ptr(),
        has_arg: no_argument,
        flag: ptr::null_mut(),
        val: b'd' as c_int,
    }, /* show detail descriptions */
    option {
        name: c"file".as_ptr(),
        has_arg: required_argument,
        flag: ptr::null_mut(),
        val: b'f' as c_int,
    }, /* use user's cpuid file */
    option {
        name: c"help".as_ptr(),
        has_arg: no_argument,
        flag: ptr::null_mut(),
        val: b'h' as c_int,
    }, /* show usage */
    option {
        name: c"leaf".as_ptr(),
        has_arg: required_argument,
        flag: ptr::null_mut(),
        val: b'l' as c_int,
    }, /* only check a specific leaf */
    option {
        name: c"raw".as_ptr(),
        has_arg: no_argument,
        flag: ptr::null_mut(),
        val: b'r' as c_int,
    }, /* show raw CPUID leaf data */
    option {
        name: c"subleaf".as_ptr(),
        has_arg: required_argument,
        flag: ptr::null_mut(),
        val: b's' as c_int,
    }, /* check a specific subleaf */
    option {
        name: ptr::null(),
        has_arg: 0,
        flag: ptr::null_mut(),
        val: 0,
    },
];

unsafe fn parse_options(argc: c_int, argv: *mut *mut c_char) {
    loop {
        let c = getopt_long(argc, argv, c"abdf:hl:rs:".as_ptr(), opts.as_ptr(), ptr::null_mut());
        if c == -1 {
            break;
        }
        match c {
            x if x == b'a' as c_int => {
                show_flags_only = false;
            }
            x if x == b'b' as c_int => {
                show_flags_only = true;
            }
            x if x == b'd' as c_int => {
                show_details = true;
            }
            x if x == b'f' as c_int => {
                user_csv = optarg;
            }
            x if x == b'h' as c_int => {
                usage(EXIT_SUCCESS);
            }
            x if x == b'l' as c_int => {
                /* main leaf */
                user_index = strtoul(optarg, ptr::null_mut(), 0) as u32;
            }
            x if x == b'r' as c_int => {
                show_raw = true;
            }
            x if x == b's' as c_int => {
                /* subleaf */
                user_sub = strtoul(optarg, ptr::null_mut(), 0) as u32;
            }
            _ => {
                usage(EXIT_FAILURE);
            }
        }
    }
}

/*
 * Do 4 things in turn:
 * 1. Parse user options
 * 2. Parse and store all the CPUID leaf data supported on this platform
 * 2. Parse the csv file, while skipping leafs which are not available
 *    on this platform
 * 3. Print leafs info based on user options
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    parse_options(argc, argv);

    /* Setup the cpuid leafs of current platform */
    let mut i = 0usize;
    while i < ranges.len() {
        setup_cpuid_range(ranges.as_mut_ptr().add(i));
        i += 1;
    }

    /* Read and parse the 'cpuid.csv' */
    parse_text();

    show_info();
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
