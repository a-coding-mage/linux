// SPDX-License-Identifier: GPL-2.0-only
/*
 * ST Random Number Generator Driver ST's Platforms
 *
 * Author: Pankaj Dev: <pankaj.dev@st.com>
 *         Lee Jones <lee.jones@linaro.org>
 *
 * Copyright (C) 2015 STMicroelectronics (R&D) Limited
 */

// Linux kernel dependencies supplied by other translation units.

/* Registers */
const ST_RNG_STATUS_REG: usize = 0x20;
const ST_RNG_DATA_REG: usize = 0x24;

/* Registers fields */
const ST_RNG_STATUS_BAD_SEQUENCE: u32 = 1 << 0;
const ST_RNG_STATUS_BAD_ALTERNANCE: u32 = 1 << 1;
const ST_RNG_STATUS_FIFO_FULL: u32 = 1 << 5;

const ST_RNG_SAMPLE_SIZE: usize = 2; // 2 Byte (16bit) samples
const ST_RNG_FIFO_DEPTH: usize = 4;
const ST_RNG_FIFO_SIZE: usize = ST_RNG_FIFO_DEPTH * ST_RNG_SAMPLE_SIZE;

/*
 * Samples are documented to be available every 0.667us, so in theory the
 * 4 sample deep FIFO should take 2.668us to fill.  However, during
 * thorough testing, it became apparent that filling the FIFO actually
 * takes closer to 12us.  We then multiply by 2 in order to account for
 * the lack of udelay()'s reliability, suggested by Russell King.
 */
const ST_RNG_FILL_FIFO_TIMEOUT: i32 = 12 * 2;

#[repr(C)]
pub struct st_rng_data {
    pub base: *mut core::ffi::c_void,
    pub ops: hwrng,
}

unsafe fn st_rng_read(
    rng: *mut hwrng,
    data: *mut core::ffi::c_void,
    max: usize,
    _wait: bool,
) -> i32 {
    let ddata = (*rng).r#priv as *mut st_rng_data;
    let mut status: u32;
    let mut i: usize;

    /* Wait until FIFO is full - max 4uS*/
    i = 0;
    while i < ST_RNG_FILL_FIFO_TIMEOUT as usize {
        status = readl_relaxed((*ddata).base.add(ST_RNG_STATUS_REG));
        if status & ST_RNG_STATUS_FIFO_FULL != 0 {
            break;
        }
        udelay(1);
        i += 1;
    }

    if i == ST_RNG_FILL_FIFO_TIMEOUT as usize {
        return 0;
    }

    while i < ST_RNG_FIFO_SIZE && i < max {
        *(data.add(i) as *mut u16) =
            readl_relaxed((*ddata).base.add(ST_RNG_DATA_REG)) as u16;
        i += 2;
    }

    i as i32 // No of bytes read
}

unsafe fn st_rng_probe(pdev: *mut platform_device) -> i32 {
    let mut ddata: *mut st_rng_data;
    let mut clk: *mut clk;
    let mut base: *mut core::ffi::c_void;
    let mut ret: i32;

    ddata = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<st_rng_data>(), GFP_KERNEL);
    if ddata.is_null() {
        return -ENOMEM;
    }

    base = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(base) {
        return PTR_ERR(base);
    }

    clk = devm_clk_get_enabled(&mut (*pdev).dev, core::ptr::null());
    if IS_ERR(clk) {
        return PTR_ERR(clk);
    }

    (*ddata).ops.r#priv = ddata as usize;
    (*ddata).ops.read = Some(st_rng_read);
    (*ddata).ops.name = (*pdev).name;
    (*ddata).base = base;

    ret = devm_hwrng_register(&mut (*pdev).dev, &mut (*ddata).ops);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, "Failed to register HW RNG\n");
        return ret;
    }

    dev_info(&mut (*pdev).dev, "Successfully registered HW RNG\n");

    0
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const core::ffi::c_char,
}

static ST_RNG_MATCH: [of_device_id; 2] = [
    of_device_id { compatible: c"st,rng".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

// MODULE_DEVICE_TABLE(of, st_rng_match);

static mut ST_RNG_DRIVER: platform_driver = platform_driver {
    driver: driver {
        name: c"st-hwrandom".as_ptr(),
        of_match_table: of_match_ptr(&ST_RNG_MATCH),
    },
    probe: Some(st_rng_probe),
};

// module_platform_driver(st_rng_driver);

// MODULE_AUTHOR("Pankaj Dev <pankaj.dev@st.com>");
// MODULE_DESCRIPTION("ST Microelectronics HW Random Number Generator");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
