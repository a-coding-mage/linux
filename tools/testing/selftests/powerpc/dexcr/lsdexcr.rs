// SPDX-License-Identifier: GPL-2.0+

// C dependencies removed:
// <stddef.h>, <stdio.h>, <string.h>, <sys/prctl.h>
// "dexcr.h", "utils.h"

use core::ffi::{c_char, c_int};

#[repr(C)]
pub struct dexcr_aspect {
    pub index: c_int,
    pub name: *const c_char,
    pub desc: *const c_char,
    pub prctl: c_int,
}

unsafe extern "C" {
    static aspects: [dexcr_aspect; 0];
    static DEXCR: c_int;
    static HDEXCR: c_int;
    static DEXCR_PR_NPHIE: u32;
    static PR_PPC_DEXCR_CTRL_SET: c_int;
    static PR_PPC_DEXCR_CTRL_CLEAR: c_int;

    fn printf(format: *const c_char, ...) -> c_int;
    fn dexcr_exists() -> bool;
    fn get_dexcr(which: c_int) -> u32;
    fn pr_get_dexcr(aspect: c_int) -> c_int;
    fn hashchk_triggers() -> bool;
}

static mut dexcr: u32 = 0;
static mut hdexcr: u32 = 0;
static mut effective: u32 = 0;

fn DEXCR_PR_BIT(index: c_int) -> u32 {
    1u32 << index
}

const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

unsafe fn print_list(list: *const *const c_char, len: usize) {
    for i in 0..len {
        unsafe {
            printf(c"%s".as_ptr(), *list.add(i));
        }
        if i + 1 < len {
            unsafe {
                printf(c", ".as_ptr());
            }
        }
    }
}

unsafe fn print_dexcr(name: *mut c_char, mut bits: u32) {
    let mut enabled_aspects: [*const c_char; ARRAY_SIZE(unsafe { &aspects }) + 1] =
        [core::ptr::null(); ARRAY_SIZE(unsafe { &aspects }) + 1];
    let mut j: usize = 0;

    unsafe {
        printf(c"%s: 0x%08x".as_ptr(), name, bits);
    }

    if bits == 0 {
        unsafe {
            printf(c"\n".as_ptr());
        }
        return;
    }

    for i in 0..ARRAY_SIZE(unsafe { &aspects }) {
        let mask: u32 = DEXCR_PR_BIT(unsafe { aspects[i].index });

        if bits & mask != 0 {
            enabled_aspects[j] = unsafe { aspects[i].name };
            j += 1;
            bits &= !mask;
        }
    }

    if bits != 0 {
        enabled_aspects[j] = c"unknown".as_ptr();
        j += 1;
    }

    unsafe {
        printf(c" (".as_ptr());
        print_list(enabled_aspects.as_ptr(), j);
        printf(c")\n".as_ptr());
    }
}

unsafe fn print_aspect(aspect: *const dexcr_aspect) {
    let mut attributes: [*const c_char; 8] = [core::ptr::null(); 8];
    let mut j: usize = 0;
    let mask: u64;

    unsafe {
        mask = DEXCR_PR_BIT((*aspect).index) as u64;
        if dexcr as u64 & mask != 0 {
            attributes[j] = c"set".as_ptr();
            j += 1;
        }
        if hdexcr as u64 & mask != 0 {
            attributes[j] = c"set (hypervisor)".as_ptr();
            j += 1;
        }
        if !(effective as u64 & mask != 0) {
            attributes[j] = c"clear".as_ptr();
            j += 1;
        }

        printf(
            c"%12s %c (%d): ".as_ptr(),
            (*aspect).name,
            if effective as u64 & mask != 0 { '*' as c_int } else { ' ' as c_int },
            (*aspect).index,
        );
        print_list(attributes.as_ptr(), j);
        printf(c"  \t(%s)\n".as_ptr(), (*aspect).desc);
    }
}

unsafe fn print_aspect_config(aspect: *const dexcr_aspect) {
    let mut reason: *const c_char = core::ptr::null();
    let mut reason_hyp: *const c_char = core::ptr::null();
    let mut reason_prctl: *const c_char = c"no prctl".as_ptr();
    let actual: bool = unsafe { effective & DEXCR_PR_BIT((*aspect).index) != 0 };
    let mut expected: bool = actual; /* Assume it's fine if we don't expect a specific set/clear value */

    if actual {
        reason = c"set by unknown".as_ptr();
    } else {
        reason = c"cleared by unknown".as_ptr();
    }

    unsafe {
        if (*aspect).prctl != -1 {
            let ctrl: c_int = pr_get_dexcr((*aspect).prctl);

            if ctrl < 0 {
                reason_prctl = c"failed to read prctl".as_ptr();
            } else {
                if ctrl & PR_PPC_DEXCR_CTRL_SET != 0 {
                    reason_prctl = c"set by prctl".as_ptr();
                    expected = true;
                } else if ctrl & PR_PPC_DEXCR_CTRL_CLEAR != 0 {
                    reason_prctl = c"cleared by prctl".as_ptr();
                    expected = false;
                } else {
                    reason_prctl = c"unknown prctl".as_ptr();
                }

                reason = reason_prctl;
            }
        }

        if hdexcr & DEXCR_PR_BIT((*aspect).index) != 0 {
            reason_hyp = c"set by hypervisor".as_ptr();
            reason = reason_hyp;
            expected = true;
        } else {
            reason_hyp = c"not modified by hypervisor".as_ptr();
        }

        printf(
            c"%12s (%d): %-28s (%s, %s)\n".as_ptr(),
            (*aspect).name,
            (*aspect).index,
            reason,
            reason_hyp,
            reason_prctl,
        );
    }

    /*
     * The checks are not atomic, so this can technically trigger if the
     * hypervisor makes a change while we are checking each source. It's
     * far more likely to be a bug if we see this though.
     */
    if actual != expected {
        unsafe {
            printf(
                c"                : ! actual %s does not match config\n".as_ptr(),
                (*aspect).name,
            );
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    unsafe {
        if !dexcr_exists() {
            printf(c"DEXCR not detected on this hardware\n".as_ptr());
            return 1;
        }

        dexcr = get_dexcr(DEXCR);
        hdexcr = get_dexcr(HDEXCR);
        effective = dexcr | hdexcr;

        printf(c"current status:\n".as_ptr());

        print_dexcr(c"    DEXCR".as_ptr() as *mut c_char, dexcr);
        print_dexcr(c"   HDEXCR".as_ptr() as *mut c_char, hdexcr);
        print_dexcr(c"Effective".as_ptr() as *mut c_char, effective);
        printf(c"\n".as_ptr());

        for i in 0..ARRAY_SIZE(&aspects) {
            print_aspect(&aspects[i]);
        }
        printf(c"\n".as_ptr());

        if effective & DEXCR_PR_NPHIE != 0 {
            printf(c"DEXCR[NPHIE] enabled: hashst/hashchk ".as_ptr());
            if hashchk_triggers() {
                printf(c"working\n".as_ptr());
            } else {
                printf(c"failed to trigger\n".as_ptr());
            }
        } else {
            printf(c"DEXCR[NPHIE] disabled: hashst/hashchk ".as_ptr());
            if hashchk_triggers() {
                printf(c"unexpectedly triggered\n".as_ptr());
            } else {
                printf(c"ignored\n".as_ptr());
            }
        }
        printf(c"\n".as_ptr());

        printf(c"configuration:\n".as_ptr());
        for i in 0..ARRAY_SIZE(&aspects) {
            print_aspect_config(&aspects[i]);
        }
        printf(c"\n".as_ptr());

        0
    }
}
