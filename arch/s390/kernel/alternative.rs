// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the corresponding Linux headers:
// linux/hex.h, linux/uaccess.h, linux/printk.h,
// asm/nospec-branch.h, asm/abs_lowcore.h, asm/alternative.h,
// asm/facility.h, asm/sections.h, asm/machine.h

// Build-time macro defaults from the C source:
// pr_fmt(fmt) => "alt: " fmt
// a_debug => pr_debug
// __kernel_va(x) => (void *)(x)

pub static mut machine_features: [c_ulong; 1] = [0; 1];

#[repr(C)]
pub struct alt_debug {
    pub facilities: [c_ulong; MAX_FACILITY_BIT / BITS_PER_LONG],
    pub mfeatures: [c_ulong; MAX_MFEATURE_BIT / BITS_PER_LONG],
    pub spec: c_int,
}

static mut alt_debug: alt_debug = alt_debug {
    facilities: [0; MAX_FACILITY_BIT / BITS_PER_LONG],
    mfeatures: [0; MAX_MFEATURE_BIT / BITS_PER_LONG],
    spec: 0,
};

unsafe fn alternative_dump(
    old: *mut u8,
    new: *mut u8,
    len: c_uint,
    type_: c_uint,
    data: c_uint,
) {
    let mut oinsn = [0 as c_char; 33];
    let mut ninsn = [0 as c_char; 33];
    let kptr: c_ulong;
    let mut pos: c_uint;

    pos = 0;
    while pos < len && 2 * pos < (oinsn.len() as c_uint) - 3 {
        hex_byte_pack(oinsn.as_mut_ptr().add((2 * pos) as usize), *old.add(pos as usize));
        pos += 1;
    }
    *oinsn.as_mut_ptr().add((2 * pos) as usize) = 0;
    pos = 0;
    while pos < len && 2 * pos < (ninsn.len() as c_uint) - 3 {
        hex_byte_pack(ninsn.as_mut_ptr().add((2 * pos) as usize), *new.add(pos as usize));
        pos += 1;
    }
    *ninsn.as_mut_ptr().add((2 * pos) as usize) = 0;
    kptr = __kernel_va(old) as c_ulong;
    a_debug(
        "[%d/%3d] %016lx: %s -> %s\n",
        type_,
        data,
        kptr,
        oinsn.as_ptr(),
        ninsn.as_ptr(),
    );
}

pub unsafe fn __apply_alternatives(
    mut start: *mut alt_instr,
    end: *mut alt_instr,
    ctx: c_uint,
) {
    let d: *mut alt_debug = &raw mut alt_debug;
    let mut a = start;
    let mut debug: bool;
    let mut replace: bool;
    let old: *mut u8;
    let new: *mut u8;

    /*
     * The scan order should be from start to end. A later scanned
     * alternative code can overwrite previously scanned alternative code.
     */
    while a < end {
        if (*a).ctx & ctx == 0 {
            a = a.add(1);
            continue;
        }
        match (*a).type_ {
            ALT_TYPE_FACILITY => {
                replace = test_facility((*a).data);
                debug = __test_facility((*a).data, (*d).facilities.as_mut_ptr());
            }
            ALT_TYPE_FEATURE => {
                replace = test_machine_feature((*a).data);
                debug = __test_machine_feature((*a).data, (*d).mfeatures.as_mut_ptr());
            }
            ALT_TYPE_SPEC => {
                replace = nobp_enabled();
                debug = (*d).spec != 0;
            }
            _ => {
                replace = false;
                debug = false;
            }
        }
        if replace {
            old = (&raw mut (*a).instr_offset as *mut u8).offset((*a).instr_offset as isize);
            new = (&raw mut (*a).repl_offset as *mut u8).offset((*a).repl_offset as isize);
            if debug {
                alternative_dump(old, new, (*a).instrlen, (*a).type_, (*a).data);
            }
            s390_kernel_write(old, new, (*a).instrlen);
        }
        a = a.add(1);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
