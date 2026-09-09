// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Nomadik RNG support
 *  Copyright 2009 Alessandro Rubini
 */

// Kernel dependencies supplied by the surrounding translation environment.

unsafe extern "C" {
    fn __raw_readl(addr: *const core::ffi::c_void) -> u32;
}

unsafe fn nmk_rng_read(
    rng: *mut hwrng,
    data: *mut core::ffi::c_void,
    _max: usize,
    _wait: bool,
) -> i32 {
    let base = (*rng).priv_ as *mut core::ffi::c_void;

    /*
     * The register is 32 bits and gives 16 random bits (low half).
     * A subsequent read will delay the core for 400ns, so we just read
     * once and accept the very unlikely very small delay, even if wait==0.
     */
    *(data as *mut u16) = (__raw_readl(base.add(8)) & 0xffff) as u16;
    2
}

/* we have at most one RNG per machine, granted */
static mut nmk_rng: hwrng = hwrng {
    name: "nomadik",
    read: Some(nmk_rng_read),
    priv_: 0,
};

unsafe fn nmk_rng_probe(dev: *mut amba_device, _id: *const amba_id) -> i32 {
    let rng_clk: *mut clk;
    let base: *mut core::ffi::c_void;
    let mut ret: i32;

    rng_clk = devm_clk_get_enabled(&mut (*dev).dev, core::ptr::null());
    if is_err(rng_clk) {
        return dev_err_probe(
            &mut (*dev).dev,
            ptr_err(rng_clk),
            "could not get rng clock\n",
        );
    }

    ret = amba_request_regions(dev, (*dev).dev.init_name);
    if ret != 0 {
        return ret;
    }
    ret = -12; // -ENOMEM
    base = devm_ioremap(
        &mut (*dev).dev,
        (*dev).res.start,
        resource_size(&(*dev).res),
    );
    if base.is_null() {
        goto_out_release(dev, ret);
    }
    nmk_rng.priv_ = base as usize;
    ret = devm_hwrng_register(&mut (*dev).dev, &mut nmk_rng);
    if ret != 0 {
        goto_out_release(dev, ret);
    }
    0
}

unsafe fn goto_out_release(dev: *mut amba_device, ret: i32) -> i32 {
    amba_release_regions(dev);
    ret
}

unsafe fn nmk_rng_remove(dev: *mut amba_device) {
    amba_release_regions(dev);
}

static nmk_rng_ids: [amba_id; 2] = [
    amba_id {
        id: 0x0008_05e1,
        mask: 0x000f_ffff, // top bits are rev and cfg: accept all
    },
    amba_id { id: 0, mask: 0 },
];

// MODULE_DEVICE_TABLE(amba, nmk_rng_ids);

static mut nmk_rng_driver: amba_driver = amba_driver {
    drv: driver { name: "rng" },
    probe: Some(nmk_rng_probe),
    remove: Some(nmk_rng_remove),
    id_table: nmk_rng_ids.as_ptr(),
};

// module_amba_driver(nmk_rng_driver);

// MODULE_DESCRIPTION("ST-Ericsson Nomadik Random Number Generator");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
