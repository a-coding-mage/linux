// SPDX-License-Identifier: GPL-2.0-only
/*
 * ePAPR para-virtualization support.
 *
 * Copyright (C) 2012 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by the surrounding kernel translation.

#[cfg(any(not(feature = "config_64bit"), feature = "config_ppc_book3e_64"))]
extern "C" {
    fn epapr_ev_idle();
    static mut epapr_ev_idle_start: u32;
}

pub static mut epapr_paravirt_enabled: bool = false;
static mut epapr_has_idle: bool = false;

unsafe fn early_init_dt_scan_epapr(
    node: c_ulong,
    uname: *const c_char,
    depth: c_int,
    data: *mut c_void,
) -> c_int {
    let mut len: c_int = 0;
    let mut i: c_int;

    let insts = of_get_flat_dt_prop(node, b"hcall-instructions\0".as_ptr() as *const c_char, &mut len);
    if insts.is_null() {
        return 0;
    }

    if len % 4 != 0 || len > (4 * 4) {
        return -1;
    }

    i = 0;
    while i < (len / 4) {
        let inst = ppc_inst(be32_to_cpu(*insts.add(i as usize)));
        patch_instruction(epapr_hypercall_start.add(i as usize), inst);
        #[cfg(any(not(feature = "config_64bit"), feature = "config_ppc_book3e_64"))]
        patch_instruction((&raw mut epapr_ev_idle_start).add(i as usize), inst);
        i += 1;
    }

    #[cfg(any(not(feature = "config_64bit"), feature = "config_ppc_book3e_64"))]
    {
        if !of_get_flat_dt_prop(node, b"has-idle\0".as_ptr() as *const c_char, core::ptr::null_mut()).is_null() {
            epapr_has_idle = true;
        }
    }

    epapr_paravirt_enabled = true;

    1
}

pub unsafe fn epapr_paravirt_early_init() -> c_int {
    of_scan_flat_dt(Some(early_init_dt_scan_epapr), core::ptr::null_mut());

    0
}

unsafe fn epapr_idle_init() -> c_int {
    #[cfg(any(not(feature = "config_64bit"), feature = "config_ppc_book3e_64"))]
    {
        if epapr_has_idle {
            ppc_md.power_save = Some(epapr_ev_idle);
        }
    }

    0
}

// Equivalent of postcore_initcall(epapr_idle_init).
postcore_initcall!(epapr_idle_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
