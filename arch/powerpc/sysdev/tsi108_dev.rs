// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * tsi108/109 device setup code
 *
 * Maintained by Roy Zang < tie-fei.zang@freescale.com >
 */

// Linux and architecture headers supplied by the surrounding translation.
// The original DEBUG configuration is undefined in this source file.

static mut tsi108_csr_base: phys_addr_t = !0 as phys_addr_t;

pub unsafe fn get_csrbase() -> phys_addr_t {
    let mut tsi: *mut device_node;

    if tsi108_csr_base != !0 as phys_addr_t {
        return tsi108_csr_base;
    }

    tsi = of_find_node_by_type(core::ptr::null_mut(), c"tsi-bridge".as_ptr());
    if !tsi.is_null() {
        let mut res: resource = core::mem::zeroed();
        of_address_to_resource(tsi, 0, &mut res);
        tsi108_csr_base = res.start;
        of_node_put(tsi);
    }
    tsi108_csr_base
}

// EXPORT_SYMBOL(get_csrbase);

pub unsafe fn get_vir_csrbase() -> u32 {
    ioremap(get_csrbase(), 0x10000) as u32
}

// EXPORT_SYMBOL(get_vir_csrbase);

unsafe fn tsi108_eth_of_init() -> c_int {
    let mut np: *mut device_node = core::ptr::null_mut();
    let mut i: c_uint = 0;
    let mut tsi_eth_dev: *mut platform_device = core::ptr::null_mut();
    let mut res: resource = core::mem::zeroed();
    let mut ret: c_int;

    // for_each_compatible_node(np, "network", "tsi108-ethernet")
    while {
        np = of_find_compatible_node(np, c"network".as_ptr(), c"tsi108-ethernet".as_ptr());
        !np.is_null()
    } {
        let mut r: [resource; 2] = [core::mem::zeroed(), core::mem::zeroed()];
        let mut phy: *mut device_node;
        let mut mdio: *mut device_node;
        let mut tsi_eth_data: hw_info = core::mem::zeroed();
        let phy_id: *const c_uint;
        let ph: *const phandle;

        core::ptr::write_bytes(r.as_mut_ptr(), 0, 1);
        core::ptr::write_bytes(&mut tsi_eth_data as *mut hw_info, 0, 1);

        ret = of_address_to_resource(np, 0, &mut r[0]);
        if ret != 0 {
            break;
        }

        r[1].name = c"tx".as_ptr();
        r[1].start = irq_of_parse_and_map(np, 0) as _;
        r[1].end = irq_of_parse_and_map(np, 0) as _;
        r[1].flags = IORESOURCE_IRQ;

        tsi_eth_dev = platform_device_register_simple(c"tsi-ethernet".as_ptr(), i, &r[0], 1);
        i = i.wrapping_add(1);

        if IS_ERR(tsi_eth_dev) {
            ret = PTR_ERR(tsi_eth_dev);
            break;
        }

        of_get_mac_address(np, tsi_eth_data.mac_addr.as_mut_ptr());

        ph = of_get_property(np, c"mdio-handle".as_ptr(), core::ptr::null_mut()) as *const phandle;
        mdio = of_find_node_by_phandle(*ph);
        ret = of_address_to_resource(mdio, 0, &mut res);
        of_node_put(mdio);
        if ret != 0 {
            platform_device_unregister(tsi_eth_dev);
            break;
        }

        ph = of_get_property(np, c"phy-handle".as_ptr(), core::ptr::null_mut()) as *const phandle;
        phy = of_find_node_by_phandle(*ph);

        if phy.is_null() {
            ret = -ENODEV;
            platform_device_unregister(tsi_eth_dev);
            break;
        }

        phy_id = of_get_property(phy, c"reg".as_ptr(), core::ptr::null_mut()) as *const c_uint;

        tsi_eth_data.regs = r[0].start;
        tsi_eth_data.phyregs = res.start;
        tsi_eth_data.phy = *phy_id;
        tsi_eth_data.irq_num = irq_of_parse_and_map(np, 0);

        /* Some boards with the TSI108 bridge (e.g. Holly)
         * have a miswiring of the ethernet PHYs which
         * requires a workaround.  The special
         * "txc-rxc-delay-disable" property enables this
         * workaround.  FIXME: Need to port the tsi108_eth
         * driver itself to phylib and use a non-misleading
         * name for the workaround flag - it's not actually to
         * do with the model of PHY in use */
        if of_property_read_bool(phy, c"txc-rxc-delay-disable".as_ptr()) {
            tsi_eth_data.phy_type = TSI108_PHY_BCM54XX;
        }
        of_node_put(phy);

        ret = platform_device_add_data(
            tsi_eth_dev,
            &tsi_eth_data as *const hw_info as *const c_void,
            core::mem::size_of::<hw_info>(),
        );
        if ret != 0 {
            platform_device_unregister(tsi_eth_dev);
            break;
        }
    }

    if np.is_null() {
        return 0;
    }
    of_node_put(np);
    ret
}

// arch_initcall(tsi108_eth_of_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
