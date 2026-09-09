// SPDX-License-Identifier: GPL-2.0-only
/*
 * Common parts of the Xilinx Spartan6 and 7 Series FPGA manager drivers.
 *
 * Copyright (C) 2017 DENX Software Engineering
 *
 * Anatolij Gustschin <agust@denx.de>
 */

// Dependency declarations supplied by the surrounding kernel translation.

unsafe fn get_done_gpio(mgr: *mut fpga_manager) -> c_int {
    let core = (*mgr).priv_;
    let ret = gpiod_get_value((*core).done);
    if ret < 0 {
        dev_err(&(*mgr).dev, "Error reading DONE (%d)\n", ret);
    }
    ret
}

unsafe fn xilinx_core_state(mgr: *mut fpga_manager) -> fpga_mgr_states {
    if get_done_gpio(mgr) == 0 {
        return FPGA_MGR_STATE_RESET;
    }
    FPGA_MGR_STATE_UNKNOWN
}

/**
 * wait_for_init_b - wait for the INIT_B pin to have a given state, or wait
 * a given delay if the pin is unavailable
 *
 * @mgr:        The FPGA manager object
 * @value:      Value INIT_B to wait for (1 = asserted = low)
 * @alt_udelay: Delay to wait if the INIT_B GPIO is not available
 *
 * Returns 0 when the INIT_B GPIO reached the given state or -ETIMEDOUT if
 * too much time passed waiting for that. If no INIT_B GPIO is available
 * then always return 0.
 */
unsafe fn wait_for_init_b(
    mgr: *mut fpga_manager,
    value: c_int,
    alt_udelay: c_ulong,
) -> c_int {
    let core = (*mgr).priv_;
    let timeout = jiffies.wrapping_add(msecs_to_jiffies(1000));

    if !(*core).init_b.is_null() {
        while time_before(jiffies, timeout) {
            let ret = gpiod_get_value((*core).init_b);
            if ret == value {
                return 0;
            }
            if ret < 0 {
                dev_err(&(*mgr).dev, "Error reading INIT_B (%d)\n", ret);
                return ret;
            }
            usleep_range(100, 400);
        }
        dev_err(
            &(*mgr).dev,
            "Timeout waiting for INIT_B to %s\n",
            if value != 0 { "assert" } else { "deassert" },
        );
        return -ETIMEDOUT;
    }

    udelay(alt_udelay);
    0
}

unsafe fn xilinx_core_write_init(
    mgr: *mut fpga_manager,
    info: *mut fpga_image_info,
    _buf: *const c_char,
    _count: usize,
) -> c_int {
    let core = (*mgr).priv_;
    if (*info).flags & FPGA_MGR_PARTIAL_RECONFIG != 0 {
        dev_err(&(*mgr).dev, "Partial reconfiguration not supported\n");
        return -EINVAL;
    }

    gpiod_set_value((*core).prog_b, 1);
    let mut err = wait_for_init_b(mgr, 1, 1);
    if err != 0 {
        gpiod_set_value((*core).prog_b, 0);
        return err;
    }
    gpiod_set_value((*core).prog_b, 0);
    err = wait_for_init_b(mgr, 0, 0);
    if err != 0 {
        return err;
    }
    if get_done_gpio(mgr) != 0 {
        dev_err(&(*mgr).dev, "Unexpected DONE pin state...\n");
        return -EIO;
    }
    usleep_range(7500, 7600);
    0
}

unsafe fn xilinx_core_write(
    mgr: *mut fpga_manager,
    buf: *const c_char,
    count: usize,
) -> c_int {
    let core = (*mgr).priv_;
    ((*core).write)(core, buf, count)
}

unsafe fn xilinx_core_write_complete(
    mgr: *mut fpga_manager,
    info: *mut fpga_image_info,
) -> c_int {
    let core = (*mgr).priv_;
    let timeout = jiffies.wrapping_add(usecs_to_jiffies((*info).config_complete_timeout_us));
    let mut expired = false;
    let padding: [c_char; 1] = [0xffu8 as c_char];

    while !expired {
        expired = time_after(jiffies, timeout);
        let done = get_done_gpio(mgr);
        if done < 0 {
            return done;
        }
        let ret = ((*core).write)(core, padding.as_ptr(), padding.len());
        if ret != 0 {
            return ret;
        }
        if done != 0 {
            return 0;
        }
    }

    if !(*core).init_b.is_null() {
        let ret = gpiod_get_value((*core).init_b);
        if ret < 0 {
            dev_err(&(*mgr).dev, "Error reading INIT_B (%d)\n", ret);
            return ret;
        }
        dev_err(
            &(*mgr).dev,
            if ret != 0 {
                "CRC error or invalid device\n"
            } else {
                "Missing sync word or incomplete bitstream\n"
            },
        );
    } else {
        dev_err(&(*mgr).dev, "Timeout after config data transfer\n");
    }
    -ETIMEDOUT
}

unsafe fn xilinx_core_devm_gpiod_get(
    dev: *mut device,
    con_id: *const c_char,
    legacy_con_id: *const c_char,
    flags: gpiod_flags,
) -> *mut gpio_desc {
    let mut desc = devm_gpiod_get(dev, con_id, flags);
    if IS_ERR(desc) && PTR_ERR(desc) == -ENOENT
        && of_device_is_compatible((*dev).of_node, "xlnx,fpga-slave-serial")
    {
        desc = devm_gpiod_get(dev, legacy_con_id, flags);
    }
    desc
}

static xilinx_core_ops: fpga_manager_ops = fpga_manager_ops {
    state: Some(xilinx_core_state),
    write_init: Some(xilinx_core_write_init),
    write: Some(xilinx_core_write),
    write_complete: Some(xilinx_core_write_complete),
};

pub unsafe fn xilinx_core_probe(core: *mut xilinx_fpga_core) -> c_int {
    if core.is_null() || (*core).dev.is_null() || (*core).write.is_none() {
        return -EINVAL;
    }

    (*core).prog_b = xilinx_core_devm_gpiod_get(
        (*core).dev,
        "prog".as_ptr() as *const c_char,
        "prog_b".as_ptr() as *const c_char,
        GPIOD_OUT_LOW,
    );
    if IS_ERR((*core).prog_b) {
        return dev_err_probe((*core).dev, PTR_ERR((*core).prog_b), "Failed to get PROGRAM_B gpio\n");
    }

    (*core).init_b = xilinx_core_devm_gpiod_get(
        (*core).dev,
        "init".as_ptr() as *const c_char,
        "init-b".as_ptr() as *const c_char,
        GPIOD_IN,
    );
    if IS_ERR((*core).init_b) {
        return dev_err_probe((*core).dev, PTR_ERR((*core).init_b), "Failed to get INIT_B gpio\n");
    }

    (*core).done = devm_gpiod_get((*core).dev, "done".as_ptr() as *const c_char, GPIOD_IN);
    if IS_ERR((*core).done) {
        return dev_err_probe((*core).dev, PTR_ERR((*core).done), "Failed to get DONE gpio\n");
    }

    let mgr = devm_fpga_mgr_register(
        (*core).dev,
        "Xilinx Slave Serial FPGA Manager".as_ptr() as *const c_char,
        &xilinx_core_ops,
        core,
    );
    PTR_ERR_OR_ZERO(mgr)
}

// EXPORT_SYMBOL_GPL(xilinx_core_probe);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Anatolij Gustschin <agust@denx.de>");
// MODULE_DESCRIPTION("Xilinx 7 Series FPGA manager core");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
