/*
 *  vectors.c
 *
 *  Copyright (C) 1993, 1994 by Hamish Macdonald
 *
 *  68040 fixes by Michael Rausch
 *  68040 fixes by Martin Apel
 *  68040 fixes and writeback by Richard Zidlicky
 *  68060 fixes by Roman Hodek
 *  68060 fixes by Jesper Skov
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

/* Sets up all exception vectors.  C header dependencies are supplied elsewhere. */

extern "C" {
    fn system_call();
    fn buserr();
    fn trap();
    fn bad_inthandler();
    static mut vectors: [e_vector; 256];
}

#[cfg(CONFIG_M68KFPU_EMU)]
extern "C" {
    fn fpu_emu();
}

/* nmi handler for the Amiga */
#[no_mangle]
unsafe extern "C" fn nmihandler() {
    core::arch::asm!("rte");
}

/*
 * this must be called very early as the kernel might
 * use some instruction that is emulated on the 060
 * and so we're prepared for early probe attempts (e.g. nf_init).
 */
unsafe fn base_trap_init() {
    if MACH_IS_SUN3X {
        extern "C" {
            static mut sun3x_prom_vbr: *mut e_vector;
        }

        core::arch::asm!("movec vbr, {0}", out(reg) sun3x_prom_vbr);
    }

    /* setup the exception vector table */
    core::arch::asm!("movec {0},vbr", in(reg) vectors.as_mut_ptr());

    if CPU_IS_060 {
        /* set up ISP entry points */
        extern "C" {
            #[link_name = "_060_isp_unimp"]
            fn unimp_vec();
        }

        vectors[VEC_UNIMPII] = unimp_vec;
    }

    vectors[VEC_BUSERR] = buserr;
    vectors[VEC_ILLEGAL] = trap;
    vectors[VEC_SYS] = system_call;
}

unsafe fn trap_init() {
    let mut i: i32;

    i = VEC_SPUR;
    while i <= VEC_INT7 {
        vectors[i as usize] = bad_inthandler;
        i += 1;
    }

    i = 0;
    while i < VEC_USER {
        if vectors[i as usize] as usize == 0 {
            vectors[i as usize] = trap;
        }
        i += 1;
    }

    i = VEC_USER;
    while i < 256 {
        vectors[i as usize] = bad_inthandler;
        i += 1;
    }

    #[cfg(CONFIG_M68KFPU_EMU)]
    if FPU_IS_EMU {
        vectors[VEC_LINE11] = fpu_emu;
    }

    if CPU_IS_040 && !FPU_IS_EMU {
        /* set up FPSP entry points */
        extern "C" {
            #[link_name = "dz"] fn dz_vec();
            #[link_name = "inex"] fn inex_vec();
            #[link_name = "ovfl"] fn ovfl_vec();
            #[link_name = "unfl"] fn unfl_vec();
            #[link_name = "snan"] fn snan_vec();
            #[link_name = "operr"] fn operr_vec();
            #[link_name = "bsun"] fn bsun_vec();
            #[link_name = "fline"] fn fline_vec();
            #[link_name = "unsupp"] fn unsupp_vec();
        }

        vectors[VEC_FPDIVZ] = dz_vec;
        vectors[VEC_FPIR] = inex_vec;
        vectors[VEC_FPOVER] = ovfl_vec;
        vectors[VEC_FPUNDER] = unfl_vec;
        vectors[VEC_FPNAN] = snan_vec;
        vectors[VEC_FPOE] = operr_vec;
        vectors[VEC_FPBRUC] = bsun_vec;
        vectors[VEC_LINE11] = fline_vec;
        vectors[VEC_FPUNSUP] = unsupp_vec;
    }

    if CPU_IS_060 && !FPU_IS_EMU {
        /* set up IFPSP entry points */
        extern "C" {
            #[link_name = "_060_fpsp_snan"] fn snan_vec6();
            #[link_name = "_060_fpsp_operr"] fn operr_vec6();
            #[link_name = "_060_fpsp_ovfl"] fn ovfl_vec6();
            #[link_name = "_060_fpsp_unfl"] fn unfl_vec6();
            #[link_name = "_060_fpsp_dz"] fn dz_vec6();
            #[link_name = "_060_fpsp_inex"] fn inex_vec6();
            #[link_name = "_060_fpsp_fline"] fn fline_vec6();
            #[link_name = "_060_fpsp_unsupp"] fn unsupp_vec6();
            #[link_name = "_060_fpsp_effadd"] fn effadd_vec6();
        }

        vectors[VEC_FPNAN] = snan_vec6;
        vectors[VEC_FPOE] = operr_vec6;
        vectors[VEC_FPOVER] = ovfl_vec6;
        vectors[VEC_FPUNDER] = unfl_vec6;
        vectors[VEC_FPDIVZ] = dz_vec6;
        vectors[VEC_FPIR] = inex_vec6;
        vectors[VEC_LINE11] = fline_vec6;
        vectors[VEC_FPUNSUP] = unsupp_vec6;
        vectors[VEC_UNIMPEA] = effadd_vec6;
    }

    /* if running on an amiga, make the NMI interrupt do nothing */
    if MACH_IS_AMIGA {
        vectors[VEC_INT7] = nmihandler;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
