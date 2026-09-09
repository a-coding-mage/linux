/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2009 Wind River Systems,
 *   written by Ralf Baechle <ralf@linux-mips.org>
 *
 * Copyright (c) 2013 by Cisco Systems, Inc.
 * All rights reserved.
 */

// C includes and symbols supplied by the kernel/OCTEON headers are external dependencies.

const OCTEON_MAX_MC: usize = 4;

#[repr(C)]
struct octeon_lmc_pvt {
    inject: libc::c_ulong,
    error_type: libc::c_ulong,
    dimm: libc::c_ulong,
    rank: libc::c_ulong,
    bank: libc::c_ulong,
    row: libc::c_ulong,
    col: libc::c_ulong,
}

unsafe fn octeon_lmc_edac_poll(mci: *mut mem_ctl_info) {
    let mut cfg0: cvmx_lmcx_mem_cfg0 = core::mem::zeroed();
    let mut do_clear = false;
    let mut msg = [0i8; 64];

    cfg0.u64 = cvmx_read_csr(CVMX_LMCX_MEM_CFG0((*mci).mc_idx));
    if cfg0.s.sec_err || cfg0.s.ded_err {
        let mut fadr: cvmx_lmcx_fadr = core::mem::zeroed();
        fadr.u64 = cvmx_read_csr(CVMX_LMCX_FADR((*mci).mc_idx));
        snprintf(msg.as_mut_ptr(), msg.len(), b"DIMM %d rank %d bank %d row %d col %d\0".as_ptr() as *const i8,
            fadr.cn30xx.fdimm, fadr.cn30xx.fbunk, fadr.cn30xx.fbank,
            fadr.cn30xx.frow, fadr.cn30xx.fcol);
    }

    if cfg0.s.sec_err {
        edac_mc_handle_error(HW_EVENT_ERR_CORRECTED, mci, 1, 0, 0, 0, -1, -1, -1, msg.as_ptr(), b"\0".as_ptr() as *const i8);
        cfg0.s.sec_err = -1;
        do_clear = true;
    }
    if cfg0.s.ded_err {
        edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED, mci, 1, 0, 0, 0, -1, -1, -1, msg.as_ptr(), b"\0".as_ptr() as *const i8);
        cfg0.s.ded_err = -1;
        do_clear = true;
    }
    if do_clear { cvmx_write_csr(CVMX_LMCX_MEM_CFG0((*mci).mc_idx), cfg0.u64); }
}

unsafe fn octeon_lmc_edac_poll_o2(mci: *mut mem_ctl_info) {
    let pvt = (*mci).pvt_info as *mut octeon_lmc_pvt;
    let mut int_reg: cvmx_lmcx_int = core::mem::zeroed();
    let mut do_clear = false;
    let mut msg = [0i8; 64];
    if (*pvt).inject == 0 { int_reg.u64 = cvmx_read_csr(CVMX_LMCX_INT((*mci).mc_idx)); }
    else {
        int_reg.u64 = 0;
        if (*pvt).error_type == 1 { int_reg.s.sec_err = 1; }
        if (*pvt).error_type == 2 { int_reg.s.ded_err = 1; }
    }
    if int_reg.s.sec_err || int_reg.s.ded_err {
        let mut fadr: cvmx_lmcx_fadr = core::mem::zeroed();
        if (*pvt).inject == 0 { fadr.u64 = cvmx_read_csr(CVMX_LMCX_FADR((*mci).mc_idx)); }
        else {
            fadr.cn61xx.fdimm = (*pvt).dimm; fadr.cn61xx.fbunk = (*pvt).rank;
            fadr.cn61xx.fbank = (*pvt).bank; fadr.cn61xx.frow = (*pvt).row; fadr.cn61xx.fcol = (*pvt).col;
        }
        snprintf(msg.as_mut_ptr(), msg.len(), b"DIMM %d rank %d bank %d row %d col %d\0".as_ptr() as *const i8,
            fadr.cn61xx.fdimm, fadr.cn61xx.fbunk, fadr.cn61xx.fbank, fadr.cn61xx.frow, fadr.cn61xx.fcol);
    }
    if int_reg.s.sec_err {
        edac_mc_handle_error(HW_EVENT_ERR_CORRECTED, mci, 1, 0, 0, 0, -1, -1, -1, msg.as_ptr(), b"\0".as_ptr() as *const i8);
        int_reg.s.sec_err = -1; do_clear = true;
    }
    if int_reg.s.ded_err {
        edac_mc_handle_error(HW_EVENT_ERR_UNCORRECTED, mci, 1, 0, 0, 0, -1, -1, -1, msg.as_ptr(), b"\0".as_ptr() as *const i8);
        int_reg.s.ded_err = -1; do_clear = true;
    }
    if do_clear {
        if (*pvt).inject == 0 { cvmx_write_csr(CVMX_LMCX_INT((*mci).mc_idx), int_reg.u64); }
        else { (*pvt).inject = 0; }
    }
}

// The following sysfs callbacks and driver declarations preserve the C interfaces.
// Kernel structures, unions, constants, and registration macros are supplied externally.
unsafe fn octeon_lmc_edac_probe(pdev: *mut platform_device) -> libc::c_int {
    let mut mci: *mut mem_ctl_info;
    let mut layers = [edac_mc_layer { type_: EDAC_MC_LAYER_CHANNEL, size: 1, is_virt_csrow: false }];
    let mc = (*pdev).id;
    opstate_init();
    if OCTEON_IS_OCTEON1PLUS() {
        let mut cfg0: cvmx_lmcx_mem_cfg0 = core::mem::zeroed();
        cfg0.u64 = cvmx_read_csr(CVMX_LMCX_MEM_CFG0(0));
        if !cfg0.s.ecc_ena { dev_info!(&(*pdev).dev, "Disabled (ECC not enabled)\n"); return 0; }
        mci = edac_mc_alloc(mc, layers.len(), layers.as_mut_ptr(), core::mem::size_of::<octeon_lmc_pvt>());
        if mci.is_null() { return -ENXIO; }
        (*mci).pdev = &mut (*pdev).dev; (*mci).dev_name = dev_name(&(*pdev).dev);
        (*mci).mod_name = b"octeon-lmc\0".as_ptr() as *const i8; (*mci).ctl_name = b"octeon-lmc-err\0".as_ptr() as *const i8;
        (*mci).edac_check = Some(octeon_lmc_edac_poll);
        if edac_mc_add_mc_with_groups(mci, octeon_dev_groups) != 0 { dev_err!(&(*pdev).dev, "edac_mc_add_mc() failed\n"); edac_mc_free(mci); return -ENXIO; }
        cfg0.u64 = cvmx_read_csr(CVMX_LMCX_MEM_CFG0(mc)); cfg0.s.intr_ded_ena = 0; cfg0.s.intr_sec_ena = 0; cvmx_write_csr(CVMX_LMCX_MEM_CFG0(mc), cfg0.u64);
    } else { /* OCTEON II */
        let mut en: cvmx_lmcx_int_en = core::mem::zeroed(); let mut config: cvmx_lmcx_config = core::mem::zeroed();
        config.u64 = cvmx_read_csr(CVMX_LMCX_CONFIG(0));
        if !config.s.ecc_ena { dev_info!(&(*pdev).dev, "Disabled (ECC not enabled)\n"); return 0; }
        mci = edac_mc_alloc(mc, layers.len(), layers.as_mut_ptr(), core::mem::size_of::<octeon_lmc_pvt>()); if mci.is_null() { return -ENXIO; }
        (*mci).pdev = &mut (*pdev).dev; (*mci).dev_name = dev_name(&(*pdev).dev); (*mci).mod_name = b"octeon-lmc\0".as_ptr() as *const i8; (*mci).ctl_name = b"co_lmc_err\0".as_ptr() as *const i8; (*mci).edac_check = Some(octeon_lmc_edac_poll_o2);
        if edac_mc_add_mc_with_groups(mci, octeon_dev_groups) != 0 { dev_err!(&(*pdev).dev, "edac_mc_add_mc() failed\n"); edac_mc_free(mci); return -ENXIO; }
        en.u64 = cvmx_read_csr(CVMX_LMCX_MEM_CFG0(mc)); en.s.intr_ded_ena = 0; en.s.intr_sec_ena = 0; cvmx_write_csr(CVMX_LMCX_MEM_CFG0(mc), en.u64);
    }
    platform_set_drvdata(pdev, mci); 0
}

unsafe fn octeon_lmc_edac_remove(pdev: *mut platform_device) {
    let mci = platform_get_drvdata(pdev); edac_mc_del_mc(&mut (*pdev).dev); edac_mc_free(mci);
}

#[repr(C)]
static mut octeon_lmc_edac_driver: platform_driver = platform_driver { probe: Some(octeon_lmc_edac_probe), remove: Some(octeon_lmc_edac_remove), driver: driver { name: b"octeon_lmc_edac\0".as_ptr() as *const i8 } };

// module_platform_driver(octeon_lmc_edac_driver)
// MODULE_DESCRIPTION("Cavium Octeon DRAM Memory Controller (LMC) EDAC driver");
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Ralf Baechle <ralf@linux-mips.org>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
