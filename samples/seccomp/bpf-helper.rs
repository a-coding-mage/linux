// SPDX-License-Identifier: GPL-2.0
/*
 * Seccomp BPF helper functions
 *
 * Copyright (c) 2012 The Chromium OS Authors <chromium-os-dev@chromium.org>
 * Author: Will Drewry <wad@chromium.org>
 *
 * The code may be used by anyone for any purpose,
 * and can serve as a starting point for developing
 * applications using prctl(PR_ATTACH_SECCOMP_FILTER).
 */

use core::ffi::{c_char, c_int, c_void};
use core::mem;

extern "C" {
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn strcmp(lhs: *const c_char, rhs: *const c_char) -> c_int;
}

// These types and constants are supplied by the corresponding BPF headers.
extern "C" {
    static mut stderr: *mut c_void;
}

pub unsafe fn bpf_resolve_jumps(
    labels: *mut bpf_labels,
    filter: *mut sock_filter,
    count: usize,
) -> c_int {
    if count < 1 || count > BPF_MAXINSNS as usize {
        return -1;
    }

    /*
     * Walk it once, backwards, to build the label table and do fixups.
     * Since backward jumps are disallowed by BPF, this is easy.
     */
    for i in 0..count {
        let offset = count - i - 1;
        let instr = &mut *filter.add(offset);
        if instr.code != (BPF_JMP + BPF_JA) {
            continue;
        }
        match ((instr.jt as u16) << 8) | instr.jf as u16 {
            ((JUMP_JT as u16) << 8) | JUMP_JF as u16 => {
                let label = &mut (*labels).labels[instr.k as usize];
                if label.location == 0xffff_ffff {
                    fprintf(stderr, b"Unresolved label: '%s'\n\0".as_ptr() as *const c_char,
                            label.label);
                    return 1;
                }
                instr.k = label.location - (offset as u32 + 1);
                instr.jt = 0;
                instr.jf = 0;
                continue;
            }
            ((LABEL_JT as u16) << 8) | LABEL_JF as u16 => {
                let label = &mut (*labels).labels[instr.k as usize];
                if label.location != 0xffff_ffff {
                    fprintf(stderr, b"Duplicate label use: '%s'\n\0".as_ptr() as *const c_char,
                            label.label);
                    return 1;
                }
                label.location = offset as u32;
                instr.k = 0; /* fall through */
                instr.jt = 0;
                instr.jf = 0;
                continue;
            }
            _ => {}
        }
    }
    0
}

/* Simple lookup table for labels. */
pub unsafe fn seccomp_bpf_label(labels: *mut bpf_labels, label: *const c_char) -> u32 {
    let mut begin = (*labels).labels.as_mut_ptr();
    let mut end;
    let mut id: c_int;

    if (*labels).count == BPF_LABELS_MAX {
        fprintf(stderr, b"Too many labels\n\0".as_ptr() as *const c_char);
        exit(1);
    }
    if (*labels).count == 0 {
        (*begin).label = label;
        (*begin).location = 0xffff_ffff;
        (*labels).count += 1;
        return 0;
    }
    end = begin.add((*labels).count as usize);
    id = 0;
    while begin < end {
        if strcmp(label, (*begin).label) == 0 {
            return id as u32;
        }
        begin = begin.add(1);
        id += 1;
    }
    (*begin).label = label;
    (*begin).location = 0xffff_ffff;
    (*labels).count += 1;
    id as u32
}

pub unsafe fn seccomp_bpf_print(filter: *mut sock_filter, count: usize) {
    let end = filter.add(count);
    while filter < end {
        let instr = &*filter;
        printf(b"{ code=%u,jt=%u,jf=%u,k=%u },\n\0".as_ptr() as *const c_char,
               instr.code, instr.jt, instr.jf, instr.k);
        filter = filter.add(1);
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
