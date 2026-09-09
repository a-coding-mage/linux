/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1994, 1995, 1996, 1999, 2000 by Ralf Baechle
 * Copyright (C) 1999, 2000 by Silicon Graphics
 * Copyright (C) 2002  Maciej W. Rozycki
 */

// Linux and architecture dependencies supplied by the surrounding translation.

unsafe fn dump_hub_information(errst0: libc::c_ulong, errst1: libc::c_ulong) {
    static mut ERR_TYPE: [[*const libc::c_char; 8]; 2] = [
        [
            core::ptr::null(),
            b"Uncached Partial Read PRERR\0".as_ptr() as *const libc::c_char,
            b"DERR\0".as_ptr() as *const libc::c_char,
            b"Read Timeout\0".as_ptr() as *const libc::c_char,
            core::ptr::null(),
            core::ptr::null(),
            core::ptr::null(),
            core::ptr::null(),
        ],
        [
            b"WERR\0".as_ptr() as *const libc::c_char,
            b"Uncached Partial Write\0".as_ptr() as *const libc::c_char,
            b"PWERR\0".as_ptr() as *const libc::c_char,
            b"Write Timeout\0".as_ptr() as *const libc::c_char,
            core::ptr::null(),
            core::ptr::null(),
            core::ptr::null(),
            core::ptr::null(),
        ],
    ];
    let mut st0: pi_err_stat0 = core::mem::zeroed();
    let mut st1: pi_err_stat1 = core::mem::zeroed();

    st0.pi_stat0_word = errst0;
    st1.pi_stat1_word = errst1;

    if st0.pi_stat0_fmt.s0_valid == 0 {
        pr_info!("Hub does not contain valid error information\n");
        return;
    }

    pr_info!("Hub has valid error information:\n");
    if st0.pi_stat0_fmt.s0_ovr_run != 0 {
        pr_info!("Overrun is set. Error stack may contain additional information.\n");
    }
    pr_info!("Hub error address is %08lx\n", st0.pi_stat0_fmt.s0_addr as libc::c_ulong);
    pr_info!("Incoming message command 0x%lx\n", st0.pi_stat0_fmt.s0_cmd as libc::c_ulong);
    pr_info!("Supplemental field of incoming message is 0x%lx\n", st0.pi_stat0_fmt.s0_supl as libc::c_ulong);
    pr_info!("T5 Rn (for RRB only) is 0x%lx\n", st0.pi_stat0_fmt.s0_t5_req as libc::c_ulong);

    let error_type = ERR_TYPE[st1.pi_stat1_fmt.s1_rw_rb as usize]
        [st0.pi_stat0_fmt.s0_err_type as usize];
    pr_info!(
        "Error type is %s\n",
        if error_type.is_null() { b"invalid\0".as_ptr() } else { error_type }
    );
}

unsafe fn ip27_be_handler(regs: *mut pt_regs, is_fixup: libc::c_int) -> libc::c_int {
    let regs = &mut *regs;
    let data: libc::c_int = (regs.cp0_cause & 4) as libc::c_int;
    let cpu: libc::c_int = LOCAL_HUB_L(PI_CPU_NUM) as libc::c_int;

    if is_fixup != 0 {
        return MIPS_BE_FIXUP;
    }

    printk!("Slice %c got %cbe at 0x%lx\n", b'A' + cpu as u8, if data != 0 { b'd' } else { b'i' }, regs.cp0_epc);
    printk!("Hub information:\n");
    printk!("ERR_INT_PEND = 0x%06llx\n", LOCAL_HUB_L(PI_ERR_INT_PEND));
    let errst0 = LOCAL_HUB_L(if cpu != 0 { PI_ERR_STATUS0_B } else { PI_ERR_STATUS0_A });
    let errst1 = LOCAL_HUB_L(if cpu != 0 { PI_ERR_STATUS1_B } else { PI_ERR_STATUS1_A });
    dump_hub_information(errst0, errst1);
    show_regs(regs);
    dump_tlb_all();
    loop {}
    force_sig(SIGBUS);
}

#[inline]
pub unsafe fn ip27_be_init() {
    // XXX Initialize all the Hub & Bridge error handling here.
    let cpu = LOCAL_HUB_L(PI_CPU_NUM) as libc::c_int;
    let cpuoff = cpu << 8;

    mips_set_be_handler(ip27_be_handler);

    LOCAL_HUB_S(PI_ERR_INT_PEND, if cpu != 0 { PI_ERR_CLEAR_ALL_B } else { PI_ERR_CLEAR_ALL_A });
    LOCAL_HUB_S(PI_ERR_INT_MASK_A + cpuoff, 0);
    LOCAL_HUB_S(PI_ERR_STACK_ADDR_A + cpuoff, 0);
    LOCAL_HUB_S(PI_ERR_STACK_SIZE, 0); // Disable error stack
    LOCAL_HUB_S(PI_SYSAD_ERRCHK_EN, PI_SYSAD_CHECK_ALL);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
