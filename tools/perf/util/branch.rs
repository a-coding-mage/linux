// Translated from perf/util/branch.c.
// Dependencies originally included:
// - "util/map_symbol.h"
// - "util/branch.h"
// - <linux/kernel.h>

use core::ffi::{c_char, c_double, c_int};

pub type u64 = u64;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct branch_type_stat {
    pub counts: [u64; PERF_BR_MAX as usize],
    pub new_counts: [u64; PERF_BR_NEW_MAX as usize],
    pub cond_fwd: u64,
    pub cond_bwd: u64,
    pub cross_4k: u64,
    pub cross_2m: u64,
}

#[repr(C)]
pub struct branch_flags {
    pub type_: c_int,
    pub new_type: c_int,
}

#[repr(C)]
pub struct branch_entry {
    pub flags: branch_flags,
}

pub const PERF_BR_UNKNOWN: c_int = 0;
pub const PERF_BR_COND: c_int = 1;
pub const PERF_BR_EXTEND_ABI: c_int = 15;
pub const PERF_BR_MAX: c_int = 16;
pub const PERF_BR_NEW_MAX: c_int = 8;
pub const PERF_BR_SPEC_MAX: c_int = 4;

unsafe extern "C" {
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
}

unsafe fn cross_area(addr1: u64, addr2: u64, size: c_int) -> bool {
    let align1: u64;
    let align2: u64;

    align1 = addr1 & !((size - 1) as u64);
    align2 = addr2 & !((size - 1) as u64);

    if align1 != align2 { true } else { false }
}

pub const AREA_4K: c_int = 4096;
pub const AREA_2M: c_int = 2 * 1024 * 1024;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn branch_type_count(
    st: *mut branch_type_stat,
    flags: *mut branch_flags,
    from: u64,
    to: u64,
) {
    if (*flags).type_ == PERF_BR_UNKNOWN || from == 0 {
        return;
    }

    if (*flags).type_ == PERF_BR_EXTEND_ABI {
        (*st).new_counts[(*flags).new_type as usize] += 1;
    } else {
        (*st).counts[(*flags).type_ as usize] += 1;
    }

    if (*flags).type_ == PERF_BR_COND {
        if to > from {
            (*st).cond_fwd += 1;
        } else {
            (*st).cond_bwd += 1;
        }
    }

    if cross_area(from, to, AREA_2M) {
        (*st).cross_2m += 1;
    } else if cross_area(from, to, AREA_4K) {
        (*st).cross_4k += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn branch_new_type_name(new_type: c_int) -> *const c_char {
    /*
     * TODO: This switch should happen on 'perf_session__env(session)->arch'
     * instead, because an arm64 platform perf recording could be
     * opened for analysis on other platforms as well.
     */
    // C selected ARM64 names under #ifdef __aarch64__, otherwise ARCH_*.
    #[cfg(target_arch = "aarch64")]
    let branch_new_names: [*const c_char; PERF_BR_NEW_MAX as usize] = [
        c"FAULT_ALGN".as_ptr(),
        c"FAULT_DATA".as_ptr(),
        c"FAULT_INST".as_ptr(),
        c"ARM64_FIQ".as_ptr(),
        c"ARM64_DEBUG_HALT".as_ptr(),
        c"ARM64_DEBUG_EXIT".as_ptr(),
        c"ARM64_DEBUG_INST".as_ptr(),
        c"ARM64_DEBUG_DATA".as_ptr(),
    ];
    #[cfg(not(target_arch = "aarch64"))]
    let branch_new_names: [*const c_char; PERF_BR_NEW_MAX as usize] = [
        c"FAULT_ALGN".as_ptr(),
        c"FAULT_DATA".as_ptr(),
        c"FAULT_INST".as_ptr(),
        c"ARCH_1".as_ptr(),
        c"ARCH_2".as_ptr(),
        c"ARCH_3".as_ptr(),
        c"ARCH_4".as_ptr(),
        c"ARCH_5".as_ptr(),
    ];

    if new_type >= 0 && new_type < PERF_BR_NEW_MAX {
        return branch_new_names[new_type as usize];
    }

    core::ptr::null()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn branch_type_name(type_: c_int) -> *const c_char {
    let branch_names: [*const c_char; PERF_BR_MAX as usize] = [
        c"N/A".as_ptr(),
        c"COND".as_ptr(),
        c"UNCOND".as_ptr(),
        c"IND".as_ptr(),
        c"CALL".as_ptr(),
        c"IND_CALL".as_ptr(),
        c"RET".as_ptr(),
        c"SYSCALL".as_ptr(),
        c"SYSRET".as_ptr(),
        c"COND_CALL".as_ptr(),
        c"COND_RET".as_ptr(),
        c"ERET".as_ptr(),
        c"IRQ".as_ptr(),
        c"SERROR".as_ptr(),
        c"NO_TX".as_ptr(),
        c"".as_ptr(), // Needed for PERF_BR_EXTEND_ABI that ends up triggering some compiler warnings about NULL deref
    ];

    if type_ >= 0 && type_ < PERF_BR_MAX {
        return branch_names[type_ as usize];
    }

    core::ptr::null()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_branch_type(e: *mut branch_entry) -> *const c_char {
    if (*e).flags.type_ == PERF_BR_UNKNOWN {
        return c"".as_ptr();
    }

    if (*e).flags.type_ == PERF_BR_EXTEND_ABI {
        return branch_new_type_name((*e).flags.new_type);
    }

    branch_type_name((*e).flags.type_)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn branch_type_stat_display(fp: *mut FILE, st: *const branch_type_stat) {
    let mut total: u64 = 0;
    let mut i: c_int;

    i = 0;
    while i < PERF_BR_MAX {
        total += (*st).counts[i as usize];
        i += 1;
    }

    if total == 0 {
        return;
    }

    fprintf(fp, c"\n#".as_ptr());
    fprintf(fp, c"\n# Branch Statistics:".as_ptr());
    fprintf(fp, c"\n#".as_ptr());

    if (*st).cond_fwd > 0 {
        fprintf(
            fp,
            c"\n%8s: %5.1f%%".as_ptr(),
            c"COND_FWD".as_ptr(),
            100.0 as c_double * (*st).cond_fwd as c_double / total as c_double,
        );
    }

    if (*st).cond_bwd > 0 {
        fprintf(
            fp,
            c"\n%8s: %5.1f%%".as_ptr(),
            c"COND_BWD".as_ptr(),
            100.0 as c_double * (*st).cond_bwd as c_double / total as c_double,
        );
    }

    if (*st).cross_4k > 0 {
        fprintf(
            fp,
            c"\n%8s: %5.1f%%".as_ptr(),
            c"CROSS_4K".as_ptr(),
            100.0 as c_double * (*st).cross_4k as c_double / total as c_double,
        );
    }

    if (*st).cross_2m > 0 {
        fprintf(
            fp,
            c"\n%8s: %5.1f%%".as_ptr(),
            c"CROSS_2M".as_ptr(),
            100.0 as c_double * (*st).cross_2m as c_double / total as c_double,
        );
    }

    i = 0;
    while i < PERF_BR_MAX {
        if (*st).counts[i as usize] > 0 {
            fprintf(
                fp,
                c"\n%8s: %5.1f%%".as_ptr(),
                branch_type_name(i),
                100.0 as c_double * (*st).counts[i as usize] as c_double / total as c_double,
            );
        }
        i += 1;
    }

    i = 0;
    while i < PERF_BR_NEW_MAX {
        if (*st).new_counts[i as usize] > 0 {
            fprintf(
                fp,
                c"\n%8s: %5.1f%%".as_ptr(),
                branch_new_type_name(i),
                100.0 as c_double * (*st).new_counts[i as usize] as c_double / total as c_double,
            );
        }
        i += 1;
    }
}

unsafe fn count_str_scnprintf(
    idx: c_int,
    str_: *const c_char,
    bf: *mut c_char,
    size: c_int,
) -> c_int {
    scnprintf(
        bf,
        size as usize,
        c"%s%s".as_ptr(),
        if idx != 0 { c" ".as_ptr() } else { c" (".as_ptr() },
        str_,
    )
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn branch_type_str(
    st: *const branch_type_stat,
    bf: *mut c_char,
    size: c_int,
) -> c_int {
    let mut i: c_int;
    let mut j: c_int = 0;
    let mut printed: c_int = 0;
    let mut total: u64 = 0;

    i = 0;
    while i < PERF_BR_MAX {
        total += (*st).counts[i as usize];
        i += 1;
    }

    i = 0;
    while i < PERF_BR_NEW_MAX {
        total += (*st).new_counts[i as usize];
        i += 1;
    }

    if total == 0 {
        return 0;
    }

    if (*st).cond_fwd > 0 {
        printed += count_str_scnprintf(j, c"COND_FWD".as_ptr(), bf.offset(printed as isize), size - printed);
        j += 1;
    }

    if (*st).cond_bwd > 0 {
        printed += count_str_scnprintf(j, c"COND_BWD".as_ptr(), bf.offset(printed as isize), size - printed);
        j += 1;
    }

    i = 0;
    while i < PERF_BR_MAX {
        if i == PERF_BR_COND {
            i += 1;
            continue;
        }

        if (*st).counts[i as usize] > 0 {
            printed += count_str_scnprintf(j, branch_type_name(i), bf.offset(printed as isize), size - printed);
            j += 1;
        }
        i += 1;
    }

    i = 0;
    while i < PERF_BR_NEW_MAX {
        if (*st).new_counts[i as usize] > 0 {
            printed += count_str_scnprintf(j, branch_new_type_name(i), bf.offset(printed as isize), size - printed);
            j += 1;
        }
        i += 1;
    }

    if (*st).cross_4k > 0 {
        printed += count_str_scnprintf(j, c"CROSS_4K".as_ptr(), bf.offset(printed as isize), size - printed);
        j += 1;
    }

    if (*st).cross_2m > 0 {
        printed += count_str_scnprintf(j, c"CROSS_2M".as_ptr(), bf.offset(printed as isize), size - printed);
        j += 1;
    }

    printed
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn branch_spec_desc(spec: c_int) -> *const c_char {
    let branch_spec_outcomes: [*const c_char; PERF_BR_SPEC_MAX as usize] = [
        c"N/A".as_ptr(),
        c"SPEC_WRONG_PATH".as_ptr(),
        c"NON_SPEC_CORRECT_PATH".as_ptr(),
        c"SPEC_CORRECT_PATH".as_ptr(),
    ];

    if spec >= 0 && spec < PERF_BR_SPEC_MAX {
        return branch_spec_outcomes[spec as usize];
    }

    core::ptr::null()
}
