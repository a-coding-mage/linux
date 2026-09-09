/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2008 David Daney
 */

/* External declarations supplied by the scheduler and MIPS watch-register headers. */

pub unsafe fn mips_install_watch_registers(t: *mut task_struct) {
    let watches = &mut (*t).thread.watch.mips3264;
    let watchhi: u32 = MIPS_WATCHHI_G | MIPS_WATCHHI_IRW;

    match current_cpu_data.watch_reg_use_cnt {
        4 => {
            write_c0_watchlo3(watches.watchlo[3]);
            write_c0_watchhi3(watchhi | watches.watchhi[3]);
            write_c0_watchlo2(watches.watchlo[2]);
            write_c0_watchhi2(watchhi | watches.watchhi[2]);
            write_c0_watchlo1(watches.watchlo[1]);
            write_c0_watchhi1(watchhi | watches.watchhi[1]);
            write_c0_watchlo0(watches.watchlo[0]);
            write_c0_watchhi0(watchhi | watches.watchhi[0]);
        }
        3 => {
            write_c0_watchlo2(watches.watchlo[2]);
            write_c0_watchhi2(watchhi | watches.watchhi[2]);
            write_c0_watchlo1(watches.watchlo[1]);
            write_c0_watchhi1(watchhi | watches.watchhi[1]);
            write_c0_watchlo0(watches.watchlo[0]);
            write_c0_watchhi0(watchhi | watches.watchhi[0]);
        }
        2 => {
            write_c0_watchlo1(watches.watchlo[1]);
            write_c0_watchhi1(watchhi | watches.watchhi[1]);
            write_c0_watchlo0(watches.watchlo[0]);
            write_c0_watchhi0(watchhi | watches.watchhi[0]);
        }
        1 => {
            write_c0_watchlo0(watches.watchlo[0]);
            write_c0_watchhi0(watchhi | watches.watchhi[0]);
        }
        _ => BUG(),
    }
}

pub unsafe fn mips_read_watch_registers() {
    let watches = &mut (*current).thread.watch.mips3264;
    let watchhi_mask: u32 = MIPS_WATCHHI_MASK | MIPS_WATCHHI_IRW;

    match current_cpu_data.watch_reg_use_cnt {
        4 => {
            watches.watchhi[3] = read_c0_watchhi3() & watchhi_mask;
            watches.watchhi[2] = read_c0_watchhi2() & watchhi_mask;
            watches.watchhi[1] = read_c0_watchhi1() & watchhi_mask;
            watches.watchhi[0] = read_c0_watchhi0() & watchhi_mask;
        }
        3 => {
            watches.watchhi[2] = read_c0_watchhi2() & watchhi_mask;
            watches.watchhi[1] = read_c0_watchhi1() & watchhi_mask;
            watches.watchhi[0] = read_c0_watchhi0() & watchhi_mask;
        }
        2 => {
            watches.watchhi[1] = read_c0_watchhi1() & watchhi_mask;
            watches.watchhi[0] = read_c0_watchhi0() & watchhi_mask;
        }
        1 => {
            watches.watchhi[0] = read_c0_watchhi0() & watchhi_mask;
        }
        _ => BUG(),
    }

    if current_cpu_data.watch_reg_use_cnt == 1
        && (watches.watchhi[0] & MIPS_WATCHHI_IRW) == 0
    {
        /* Pathological case of release 1 architecture that
         * doesn't set the condition bits.  We assume that
         * since we got here, the watch condition was met and
         * signal that the conditions requested in watchlo
         * were met.  */
        watches.watchhi[0] |= watches.watchlo[0] & MIPS_WATCHHI_IRW;
    }
}

pub unsafe fn mips_clear_watch_registers() {
    match current_cpu_data.watch_reg_count {
        8 => {
            write_c0_watchlo7(0);
            write_c0_watchlo6(0);
            write_c0_watchlo5(0);
            write_c0_watchlo4(0);
            write_c0_watchlo3(0);
            write_c0_watchlo2(0);
            write_c0_watchlo1(0);
            write_c0_watchlo0(0);
        }
        7 => {
            write_c0_watchlo6(0);
            write_c0_watchlo5(0);
            write_c0_watchlo4(0);
            write_c0_watchlo3(0);
            write_c0_watchlo2(0);
            write_c0_watchlo1(0);
            write_c0_watchlo0(0);
        }
        6 => {
            write_c0_watchlo5(0);
            write_c0_watchlo4(0);
            write_c0_watchlo3(0);
            write_c0_watchlo2(0);
            write_c0_watchlo1(0);
            write_c0_watchlo0(0);
        }
        5 => {
            write_c0_watchlo4(0);
            write_c0_watchlo3(0);
            write_c0_watchlo2(0);
            write_c0_watchlo1(0);
            write_c0_watchlo0(0);
        }
        4 => {
            write_c0_watchlo3(0);
            write_c0_watchlo2(0);
            write_c0_watchlo1(0);
            write_c0_watchlo0(0);
        }
        3 => {
            write_c0_watchlo2(0);
            write_c0_watchlo1(0);
            write_c0_watchlo0(0);
        }
        2 => {
            write_c0_watchlo1(0);
            write_c0_watchlo0(0);
        }
        1 => write_c0_watchlo0(0),
        _ => BUG(),
    }
}

pub unsafe fn mips_probe_watch_registers(c: *mut cpuinfo_mips) {
    let mut t: u32;

    if ((*c).options & MIPS_CPU_WATCH) == 0 {
        return;
    }

    /* Check which of the I,R and W bits are supported, then disable the register. */
    write_c0_watchlo0(MIPS_WATCHLO_IRW);
    back_to_back_c0_hazard();
    t = read_c0_watchlo0();
    write_c0_watchlo0(0);
    (*c).watch_reg_masks[0] = t & MIPS_WATCHLO_IRW;

    /* Write the mask bits and read them back to determine which can be used. */
    (*c).watch_reg_count = 1;
    (*c).watch_reg_use_cnt = 1;
    t = read_c0_watchhi0();
    write_c0_watchhi0(t | MIPS_WATCHHI_MASK);
    back_to_back_c0_hazard();
    t = read_c0_watchhi0();
    (*c).watch_reg_masks[0] |= t & MIPS_WATCHHI_MASK;
    if (t & MIPS_WATCHHI_M) == 0 { return; }

    write_c0_watchlo1(MIPS_WATCHLO_IRW); back_to_back_c0_hazard(); t = read_c0_watchlo1(); write_c0_watchlo1(0);
    (*c).watch_reg_masks[1] = t & MIPS_WATCHLO_IRW; (*c).watch_reg_count = 2; (*c).watch_reg_use_cnt = 2;
    t = read_c0_watchhi1(); write_c0_watchhi1(t | MIPS_WATCHHI_MASK); back_to_back_c0_hazard(); t = read_c0_watchhi1();
    (*c).watch_reg_masks[1] |= t & MIPS_WATCHHI_MASK; if (t & MIPS_WATCHHI_M) == 0 { return; }

    write_c0_watchlo2(MIPS_WATCHLO_IRW); back_to_back_c0_hazard(); t = read_c0_watchlo2(); write_c0_watchlo2(0);
    (*c).watch_reg_masks[2] = t & MIPS_WATCHLO_IRW; (*c).watch_reg_count = 3; (*c).watch_reg_use_cnt = 3;
    t = read_c0_watchhi2(); write_c0_watchhi2(t | MIPS_WATCHHI_MASK); back_to_back_c0_hazard(); t = read_c0_watchhi2();
    (*c).watch_reg_masks[2] |= t & MIPS_WATCHHI_MASK; if (t & MIPS_WATCHHI_M) == 0 { return; }

    write_c0_watchlo3(MIPS_WATCHLO_IRW); back_to_back_c0_hazard(); t = read_c0_watchlo3(); write_c0_watchlo3(0);
    (*c).watch_reg_masks[3] = t & MIPS_WATCHLO_IRW; (*c).watch_reg_count = 4; (*c).watch_reg_use_cnt = 4;
    t = read_c0_watchhi3(); write_c0_watchhi3(t | MIPS_WATCHHI_MASK); back_to_back_c0_hazard(); t = read_c0_watchhi3();
    (*c).watch_reg_masks[3] |= t & MIPS_WATCHHI_MASK; if (t & MIPS_WATCHHI_M) == 0 { return; }

    /* We use at most 4, but probe and report up to 8. */
    (*c).watch_reg_count = 5; t = read_c0_watchhi4(); if (t & MIPS_WATCHHI_M) == 0 { return; }
    (*c).watch_reg_count = 6; t = read_c0_watchhi5(); if (t & MIPS_WATCHHI_M) == 0 { return; }
    (*c).watch_reg_count = 7; t = read_c0_watchhi6(); if (t & MIPS_WATCHHI_M) == 0 { return; }
    (*c).watch_reg_count = 8;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
