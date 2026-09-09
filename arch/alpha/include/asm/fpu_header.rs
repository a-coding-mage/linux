/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// asm/special_insns.h and uapi/asm/fpu.h

/* The following two functions don't need trapb/excb instructions
   around the mf_fpcr/mt_fpcr instructions because (a) the kernel
   never generates arithmetic faults and (b) call_pal instructions
   are implied trap barriers. */

pub unsafe fn rdfpcr() -> ::core::ffi::c_ulong {
    let mut tmp: ::core::ffi::c_ulong;
    let ret: ::core::ffi::c_ulong;

    preempt_disable();
    if (*current_thread_info()).status & TS_SAVED_FP != 0 {
        ret = (*current_thread_info()).fp[31];
    } else {
        #[cfg(any(CONFIG_ALPHA_EV6, CONFIG_ALPHA_EV67))]
        {
            // Alpha EV6/EV67 implementation of the original inline assembly.
            ::core::arch::asm!(
                "ftoit $f0,{0}\n\t",
                "mf_fpcr $f0\n\t",
                "ftoit $f0,{1}\n\t",
                "itoft {0},$f0",
                out(reg) tmp,
                out(reg) ret,
            );
        }
        #[cfg(not(any(CONFIG_ALPHA_EV6, CONFIG_ALPHA_EV67)))]
        {
            // Non-EV6 implementation of the original inline assembly.
            ::core::arch::asm!(
                "stt $f0,{0}\n\t",
                "mf_fpcr $f0\n\t",
                "stt $f0,{1}\n\t",
                "ldt $f0,{0}",
                out(reg) tmp,
                out(reg) ret,
            );
        }
    }
    preempt_enable();

    ret
}

pub unsafe fn wrfpcr(val: ::core::ffi::c_ulong) {
    let mut tmp: ::core::ffi::c_ulong;

    preempt_disable();
    if (*current_thread_info()).status & TS_SAVED_FP != 0 {
        (*current_thread_info()).status |= TS_RESTORE_FP;
        (*current_thread_info()).fp[31] = val;
    } else {
        #[cfg(any(CONFIG_ALPHA_EV6, CONFIG_ALPHA_EV67))]
        {
            // Alpha EV6/EV67 implementation of the original inline assembly.
            ::core::arch::asm!(
                "ftoit $f0,{0}\n\t",
                "itoft {1},$f0\n\t",
                "mt_fpcr $f0\n\t",
                "itoft {0},$f0",
                out(reg) tmp,
                in(reg) val,
            );
        }
        #[cfg(not(any(CONFIG_ALPHA_EV6, CONFIG_ALPHA_EV67)))]
        {
            // Non-EV6 implementation of the original inline assembly.
            ::core::arch::asm!(
                "stt $f0,{0}\n\t",
                "ldt $f0,{1}\n\t",
                "mt_fpcr $f0\n\t",
                "ldt $f0,{0}",
                out(reg) tmp,
                in(reg) val,
            );
        }
    }
    preempt_enable();
}

pub unsafe fn swcr_update_status(
    mut swcr: ::core::ffi::c_ulong,
    fpcr: ::core::ffi::c_ulong,
) -> ::core::ffi::c_ulong {
    /* EV6 implements most of the bits in hardware.  Collect
       the acrued exception bits from the real fpcr. */
    if implver() == IMPLVER_EV6 {
        swcr &= !IEEE_STATUS_MASK;
        swcr |= (fpcr >> 35) & IEEE_STATUS_MASK;
    }
    swcr
}

extern "C" {
    pub fn alpha_read_fp_reg(reg: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    pub fn alpha_write_fp_reg(reg: ::core::ffi::c_ulong, val: ::core::ffi::c_ulong);
    pub fn alpha_read_fp_reg_s(reg: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong;
    pub fn alpha_write_fp_reg_s(reg: ::core::ffi::c_ulong, val: ::core::ffi::c_ulong);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
