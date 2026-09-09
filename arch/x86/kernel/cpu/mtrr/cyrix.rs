// SPDX-License-Identifier: GPL-2.0

unsafe fn cyrix_get_arr(
    reg: ::core::ffi::c_uint,
    base: *mut ::core::ffi::c_ulong,
    size: *mut ::core::ffi::c_ulong,
    typ: *mut mtrr_type,
) {
    let arr: u8;
    let ccr3: u8;
    let rcr: u8;
    let shift: u8;
    let mut flags: ::core::ffi::c_ulong = 0;

    arr = CX86_ARR_BASE.wrapping_add((reg << 1) as u8).wrapping_add(reg as u8);

    local_irq_save(&mut flags);

    ccr3 = getCx86(CX86_CCR3);
    setCx86(CX86_CCR3, (ccr3 & 0x0f) | 0x10);
    *((base as *mut u8).add(3)) = getCx86(arr);
    *((base as *mut u8).add(2)) = getCx86(arr.wrapping_add(1));
    *((base as *mut u8).add(1)) = getCx86(arr.wrapping_add(2));
    rcr = getCx86(CX86_RCR_BASE.wrapping_add(reg as u8));
    setCx86(CX86_CCR3, ccr3);

    local_irq_restore(flags);

    shift = *((base as *mut u8).add(1)) & 0x0f;
    *base >>= PAGE_SHIFT;

    /*
     * Power of two, at least 4K on ARR0-ARR6, 256K on ARR7
     * Note: shift==0xf means 4G, this is unsupported.
     */
    if shift != 0 {
        *size = (if reg < 7 { 0x1 } else { 0x40 }) << (shift - 1);
    } else {
        *size = 0;
    }

    /* Bit 0 is Cache Enable on ARR7, Cache Disable on ARR0-ARR6 */
    if reg < 7 {
        *typ = match rcr {
            1 => MTRR_TYPE_UNCACHABLE,
            8 => MTRR_TYPE_WRBACK,
            9 => MTRR_TYPE_WRCOMB,
            24 => MTRR_TYPE_WRTHROUGH,
            _ => MTRR_TYPE_WRTHROUGH,
        };
    } else {
        *typ = match rcr {
            0 => MTRR_TYPE_UNCACHABLE,
            8 => MTRR_TYPE_WRCOMB,
            9 => MTRR_TYPE_WRBACK,
            25 => MTRR_TYPE_WRTHROUGH,
            _ => MTRR_TYPE_WRTHROUGH,
        };
    }
}

/*
 * cyrix_get_free_region - get a free ARR.
 *
 * @base: the starting (base) address of the region.
 * @size: the size (in bytes) of the region.
 *
 * Returns: the index of the region on success, else -1 on error.
 */
unsafe fn cyrix_get_free_region(
    _base: ::core::ffi::c_ulong,
    size: ::core::ffi::c_ulong,
    replace_reg: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut lbase: ::core::ffi::c_ulong = 0;
    let mut lsize: ::core::ffi::c_ulong = 0;
    let mut ltype: mtrr_type = MTRR_TYPE_UNCACHABLE;
    let mut i: ::core::ffi::c_int;

    match replace_reg {
        7 if size < 0x40 => {}
        7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 => return replace_reg,
        _ => {}
    }

    /* If we are to set up a region >32M then look at ARR7 immediately */
    if size > 0x2000 {
        cyrix_get_arr(7, &mut lbase, &mut lsize, &mut ltype);
        if lsize == 0 {
            return 7;
        }
    } else {
        i = 0;
        while i < 7 {
            cyrix_get_arr(i as ::core::ffi::c_uint, &mut lbase, &mut lsize, &mut ltype);
            if lsize == 0 {
                return i;
            }
            i += 1;
        }
        /*
         * ARR0-ARR6 isn't free
         * try ARR7 but its size must be at least 256K
         */
        cyrix_get_arr(i as ::core::ffi::c_uint, &mut lbase, &mut lsize, &mut ltype);
        if lsize == 0 && size >= 0x40 {
            return i;
        }
    }
    -ENOSPC
}

static mut cr4: u32 = 0;
static mut ccr3: u8 = 0;

unsafe fn prepare_set() {
    let mut cr0: u32;

    /* Save value of CR4 and clear Page Global Enable (bit 7) */
    if boot_cpu_has(X86_FEATURE_PGE) {
        cr4 = __read_cr4();
        __write_cr4(cr4 & !X86_CR4_PGE);
    }

    /*
     * Disable and flush caches.
     * Note that wbinvd flushes the TLBs as a side-effect
     */
    cr0 = read_cr0() | X86_CR0_CD;
    wbinvd();
    write_cr0(cr0);
    wbinvd();

    /* Cyrix ARRs - everything else was excluded at the top */
    ccr3 = getCx86(CX86_CCR3);
    setCx86(CX86_CCR3, (ccr3 & 0x0f) | 0x10);
}

unsafe fn post_set() {
    /* Flush caches and TLBs */
    wbinvd();
    /* Cyrix ARRs - everything else was excluded at the top */
    setCx86(CX86_CCR3, ccr3);
    /* Enable caches */
    write_cr0(read_cr0() & !X86_CR0_CD);
    /* Restore value of CR4 */
    if boot_cpu_has(X86_FEATURE_PGE) {
        __write_cr4(cr4);
    }
}

unsafe fn cyrix_set_arr(
    reg: ::core::ffi::c_uint,
    mut base: ::core::ffi::c_ulong,
    mut size: ::core::ffi::c_ulong,
    typ: mtrr_type,
) {
    let arr = CX86_ARR_BASE.wrapping_add((reg << 1) as u8).wrapping_add(reg as u8);
    let arr_type: u8;

    /* count down from 32M (ARR0-ARR6) or from 2G (ARR7) */
    if reg >= 7 { size >>= 6; }
    size &= 0x7fff;
    let mut arr_size: u8 = 0;
    while size != 0 { arr_size = arr_size.wrapping_add(1); size >>= 1; }

    arr_type = if reg < 7 {
        match typ { MTRR_TYPE_UNCACHABLE => 1, MTRR_TYPE_WRCOMB => 9, MTRR_TYPE_WRTHROUGH => 24, _ => 8 }
    } else {
        match typ { MTRR_TYPE_UNCACHABLE => 0, MTRR_TYPE_WRCOMB => 8, MTRR_TYPE_WRTHROUGH => 25, _ => 9 }
    };

    prepare_set();
    base <<= PAGE_SHIFT;
    setCx86(arr, *((&base as *const _ as *const u8).add(3)));
    setCx86(arr.wrapping_add(1), *((&base as *const _ as *const u8).add(2)));
    setCx86(arr.wrapping_add(2), *((&base as *const _ as *const u8).add(1)) | arr_size);
    setCx86(CX86_RCR_BASE.wrapping_add(reg as u8), arr_type);
    post_set();
}

const cyrix_mtrr_ops: mtrr_ops = mtrr_ops {
    var_regs: 8,
    set: cyrix_set_arr,
    get: cyrix_get_arr,
    get_free_region: cyrix_get_free_region,
    validate_add_page: generic_validate_add_page,
    have_wrcomb: positive_have_wrcomb,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
