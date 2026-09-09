// SPDX-License-Identifier: GPL-2.0
// Linux dependencies supplied by other translation units are intentionally
// referenced here and are not implemented in this file.

extern "C" {
    fn kstrtobool(s: *mut core::ffi::c_char, result: *mut bool) -> i32;
    fn test_facility(facility: i32) -> bool;
    fn nospec_uses_trampoline() -> bool;
    fn nobp_enabled() -> bool;
    fn cpu_mitigations_off() -> bool;
    fn s390_kernel_write(addr: *mut u8, data: *const u8, size: usize);
    fn pr_info(fmt: *const core::ffi::c_char, ...);
}

// IS_ENABLED(CONFIG_KERNEL_NOBP)
#[no_mangle]
pub static mut nobp: i32 = 0;

#[no_mangle]
unsafe extern "C" fn nobp_setup_early(str_: *mut core::ffi::c_char) -> i32 {
    let mut enabled: bool = false;
    let rc = kstrtobool(str_, &mut enabled);
    if rc != 0 {
        return rc;
    }
    if enabled && test_facility(82) {
        /*
         * The user explicitly requested nobp=1, enable it and
         * disable the expoline support.
         */
        nobp = 1;
        // IS_ENABLED(CONFIG_EXPOLINE)
        #[cfg(feature = "CONFIG_EXPOLINE")]
        {
            nospec_disable = 1;
        }
    } else {
        nobp = 0;
    }
    0
}

// early_param("nobp", nobp_setup_early);

#[no_mangle]
unsafe extern "C" fn nospec_setup_early(_str: *mut core::ffi::c_char) -> i32 {
    nobp = 0;
    0
}

// early_param("nospec", nospec_setup_early);

#[no_mangle]
unsafe extern "C" fn nospec_report() -> i32 {
    if test_facility(156) {
        pr_info(b"Spectre V2 mitigation: etokens\n\0".as_ptr() as *const core::ffi::c_char);
    }
    if nospec_uses_trampoline() {
        pr_info(b"Spectre V2 mitigation: execute trampolines\n\0".as_ptr() as *const core::ffi::c_char);
    }
    if nobp_enabled() {
        pr_info(b"Spectre V2 mitigation: limited branch prediction\n\0".as_ptr() as *const core::ffi::c_char);
    }
    0
}

// arch_initcall(nospec_report);

// CONFIG_EXPOLINE conditional section from the original source.
#[cfg(feature = "CONFIG_EXPOLINE")]
mod expoline {
    use super::*;

    // IS_ENABLED(CONFIG_EXPOLINE_OFF)
    #[no_mangle]
    pub static mut nospec_disable: i32 = 0;

    #[no_mangle]
    unsafe extern "C" fn nospectre_v2_setup_early(
        _str: *mut core::ffi::c_char,
    ) -> i32 {
        nospec_disable = 1;
        0
    }

    // early_param("nospectre_v2", nospectre_v2_setup_early);

    #[no_mangle]
    pub unsafe extern "C" fn nospec_auto_detect() {
        if test_facility(156) || cpu_mitigations_off() {
            /*
             * The machine supports etokens.
             * Disable expolines and disable nobp.
             */
            // __is_defined(CC_USING_EXPOLINE)
            nospec_disable = 1;
            nobp = 0;
        } else {
            // __is_defined(CC_USING_EXPOLINE)
            /*
             * The kernel has been compiled with expolines.
             * Keep expolines enabled and disable nobp.
             */
            nospec_disable = 0;
            nobp = 0;
        }
        /*
         * If the kernel has not been compiled with expolines the
         * nobp setting decides what is done, this depends on the
         * CONFIG_KERNEL_NP option and the nobp/nospec parameters.
         */
    }

    #[no_mangle]
    unsafe extern "C" fn spectre_v2_setup_early(str_: *mut core::ffi::c_char) -> i32 {
        if !str_.is_null()
            && core::slice::from_raw_parts(str_ as *const u8, 2) == b"on"
        {
            nospec_disable = 0;
            nobp = 0;
        }
        if !str_.is_null()
            && core::slice::from_raw_parts(str_ as *const u8, 3) == b"off"
        {
            nospec_disable = 1;
        }
        if !str_.is_null()
            && core::slice::from_raw_parts(str_ as *const u8, 4) == b"auto"
        {
            nospec_auto_detect();
        }
        0
    }

    // early_param("spectre_v2", spectre_v2_setup_early);

    unsafe fn __nospec_revert(start: *mut i32, end: *mut i32) {
        const BRCL_EXPOLINE: i32 = 0;
        const BRASL_EXPOLINE: i32 = 1;
        let branch: [u8; 4] = [0x47, 0x00, 0x07, 0x00];
        let mut instr: *mut u8;
        let mut thunk: *mut u8;
        let mut br: *mut u8;
        let mut insnbuf = [0u8; 6];
        let mut epo = start;

        insnbuf[2..6].copy_from_slice(&branch);
        while epo < end {
            instr = (epo as *mut u8).offset((*epo) as isize);
            let type_: i32;
            if *instr == 0xc0 && (*instr.add(1) & 0x0f) == 0x04 {
                type_ = BRCL_EXPOLINE; // brcl instruction
            } else if *instr == 0xc0 && (*instr.add(1) & 0x0f) == 0x05 {
                type_ = BRASL_EXPOLINE; // brasl instruction
            } else {
                epo = epo.add(1);
                continue;
            }
            thunk = instr.offset((*(instr.add(2) as *const i32) as isize) * 2);
            if *thunk == 0xc6 && *thunk.add(1) == 0x00 {
                // exrl %r0,<target-br>
                br = thunk.offset((*(thunk.add(2) as *const i32) as isize) * 2);
            } else {
                epo = epo.add(1);
                continue;
            }
            if *br != 0x07 || (*br.add(1) & 0xf0) != 0xf0 {
                epo = epo.add(1);
                continue;
            }
            match type_ {
                BRCL_EXPOLINE => {
                    // brcl to thunk, replace with br + nop
                    insnbuf[0] = *br;
                    insnbuf[1] = (*instr.add(1) & 0xf0) | (*br.add(1) & 0x0f);
                }
                BRASL_EXPOLINE => {
                    // brasl to thunk, replace with basr + nop
                    insnbuf[0] = 0x0d;
                    insnbuf[1] = (*instr.add(1) & 0xf0) | (*br.add(1) & 0x0f);
                }
                _ => {}
            }
            s390_kernel_write(instr, insnbuf.as_ptr(), 6);
            epo = epo.add(1);
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn nospec_revert(start: *mut i32, end: *mut i32) {
        if nospec_disable != 0 {
            __nospec_revert(start, end);
        }
    }

    extern "C" {
        static mut __nospec_call_start: i32;
        static mut __nospec_call_end: i32;
        static mut __nospec_return_start: i32;
        static mut __nospec_return_end: i32;
    }

    #[no_mangle]
    pub unsafe extern "C" fn nospec_init_branches() {
        nospec_revert(&mut __nospec_call_start, &mut __nospec_call_end);
        nospec_revert(&mut __nospec_return_start, &mut __nospec_return_end);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
