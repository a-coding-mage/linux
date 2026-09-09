// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2008-2011 DENX Software Engineering GmbH
 * Author: Heiko Schocher <hs@denx.de>
 *
 * Description:
 * Keymile 83xx platform specific routines.
 */

// Linux kernel headers and local dependencies from the C translation unit.

const fn svr_rev(svr: u32) -> u32 {
    (svr >> 0) & 0xffff
}

unsafe fn quirk_mpc8360e_qe_enet10() {
    /*
     * handle mpc8360E Erratum QE_ENET10:
     * RGMII AC values do not meet the specification
     */
    let svid: u32 = mfspr(SPRN_SVR);
    let np_par: *mut device_node;
    let mut res: resource = core::mem::zeroed();
    let mut base: *mut core::ffi::c_void;
    let ret: i32;

    np_par = of_find_node_by_name(core::ptr::null_mut(), c"par_io".as_ptr());
    if np_par.is_null() {
        pr_warn(c"%s couldn't find par_io node\n".as_ptr(), c"quirk_mpc8360e_qe_enet10".as_ptr());
        return;
    }
    /* Map Parallel I/O ports registers */
    ret = of_address_to_resource(np_par, 0, &mut res);
    if ret != 0 {
        pr_warn(c"%s couldn't map par_io registers\n".as_ptr(), c"quirk_mpc8360e_qe_enet10".as_ptr());
        of_node_put(np_par);
        return;
    }

    base = ioremap(res.start, resource_size(&res));
    if base.is_null() {
        of_node_put(np_par);
        return;
    }

    /*
     * set output delay adjustments to default values according
     * table 5 in Errata Rev. 5, 9/2011:
     *
     * write 0b01 to UCC1 bits 18:19
     * write 0b01 to UCC2 option 1 bits 4:5
     * write 0b01 to UCC2 option 2 bits 16:17
     */
    clrsetbits_be32(base.byte_add(0xa8), 0x0c00f000, 0x04005000);

    /*
     * set output delay adjustments to default values according
     * table 3-13 in Reference Manual Rev.3 05/2010:
     *
     * write 0b01 to UCC2 option 2 bits 16:17
     * write 0b0101 to UCC1 bits 20:23
     * write 0b0101 to UCC2 option 1 bits 24:27
     */
    clrsetbits_be32(base.byte_add(0xac), 0x0000cff0, 0x00004550);

    if svr_rev(svid) == 0x0021 {
        /* UCC2 option 1: write 0b1010 to bits 24:27 at IMMRBAR+0x14AC */
        clrsetbits_be32(base.byte_add(0xac), 0x000000f0, 0x000000a0);
    } else if svr_rev(svid) == 0x0020 {
        /* UCC1: write 0b11 to bits 18:19 at IMMRBAR+0x14A8 */
        setbits32(base.byte_add(0xa8), 0x00003000);
        /* UCC2 option 1: write 0b11 to bits 4:5 at IMMRBAR+0x14A8 */
        setbits32(base.byte_add(0xa8), 0x0c000000);
        /* UCC2 option 2: write 0b11 to bits 16:17 at IMMRBAR+0x14AC */
        setbits32(base.byte_add(0xac), 0x0000c000);
    }
    iounmap(base);
    of_node_put(np_par);
}

/* Setup the architecture. */
unsafe fn mpc83xx_km_setup_arch() {
    mpc83xx_setup_arch();

    // CONFIG_QUICC_ENGINE:
    // The following block is compiled when the QUICC Engine is enabled.
    /*
    let mut np: *mut device_node;
    np = of_find_node_by_name(core::ptr::null_mut(), c"par_io".as_ptr());
    if !np.is_null() {
        par_io_init(np);
        of_node_put(np);
        for_each_node_by_name!(np, c"spi", par_io_of_config);
        for_each_node_by_name!(np, c"ucc", par_io_of_config);
        np = of_find_compatible_node(core::ptr::null_mut(), c"network".as_ptr(), c"ucc_geth".as_ptr());
        if !np.is_null() {
            quirk_mpc8360e_qe_enet10();
            of_node_put(np);
        }
    }
    */
}

// machine_device_initcall(mpc83xx_km, mpc83xx_declare_of_platform_devices);

/* list of the supported boards */
static BOARD: [Option<&'static core::ffi::CStr>; 3] = [
    Some(c"keymile,KMETER1"),
    Some(c"keymile,kmpbec8321"),
    None,
];

/* Called very early, MMU is off, device-tree isn't unflattened. */
unsafe fn mpc83xx_km_probe() -> i32 {
    let mut i: usize = 0;
    while BOARD[i].is_some() {
        if of_machine_is_compatible(BOARD[i].unwrap().as_ptr()) != 0 {
            break;
        }
        i += 1;
    }
    (BOARD[i].is_some()) as i32
}

// define_machine(mpc83xx_km)
#[allow(non_upper_case_globals)]
static mpc83xx_km: machine_desc = machine_desc {
    name: c"mpc83xx-km-platform".as_ptr(),
    probe: Some(mpc83xx_km_probe),
    setup_arch: Some(mpc83xx_km_setup_arch),
    discover_phbs: Some(mpc83xx_setup_pci),
    init_IRQ: Some(mpc83xx_ipic_init_IRQ),
    get_irq: Some(ipic_get_irq),
    restart: Some(mpc83xx_restart),
    time_init: Some(mpc83xx_time_init),
    progress: Some(udbg_progress),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
