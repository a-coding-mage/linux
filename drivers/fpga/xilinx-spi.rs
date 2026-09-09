// SPDX-License-Identifier: GPL-2.0-only
/*
 * Xilinx Spartan6 and 7 Series Slave Serial SPI Driver
 *
 * Copyright (C) 2017 DENX Software Engineering
 *
 * Anatolij Gustschin <agust@denx.de>
 *
 * Manage Xilinx FPGA firmware that is loaded over SPI using
 * the slave serial configuration interface.
 */

// The declarations supplied by xilinx-core.h and the Linux kernel headers
// remain external dependencies of this translation.

const SZ_4K: usize = 4096;

unsafe fn xilinx_spi_write(
    core: *mut xilinx_fpga_core,
    buf: *const u8,
    count: usize,
) -> i32 {
    let spi: *mut spi_device = to_spi_device((*core).dev);
    let mut fw_data = buf;
    let fw_data_end = fw_data.add(count);

    while fw_data < fw_data_end {
        let remaining: usize = fw_data_end.offset_from(fw_data) as usize;
        let stride = if remaining < SZ_4K { remaining } else { SZ_4K };

        let ret = spi_write(spi, fw_data, stride);
        if ret != 0 {
            dev_err((*core).dev, "SPI error in firmware write: %d\\n", ret);
            return ret;
        }
        fw_data = fw_data.add(stride);
    }

    0
}

unsafe fn xilinx_spi_probe(spi: *mut spi_device) -> i32 {
    let mut core: *mut xilinx_fpga_core =
        devm_kzalloc(
            &mut (*spi).dev,
            std::mem::size_of::<xilinx_fpga_core>(),
            GFP_KERNEL,
        );
    if core.is_null() {
        return -12; // -ENOMEM
    }

    (*core).dev = &mut (*spi).dev;
    (*core).write = Some(xilinx_spi_write);

    xilinx_core_probe(core)
}

static xilinx_spi_ids: [spi_device_id; 2] = [
    spi_device_id { name: "fpga-slave-serial" },
    spi_device_id { name: "" },
];

// MODULE_DEVICE_TABLE(spi, xilinx_spi_ids);

#[cfg(CONFIG_OF)]
static xlnx_spi_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: "xlnx,fpga-slave-serial",
    },
    of_device_id {
        compatible: "",
    },
];

// MODULE_DEVICE_TABLE(of, xlnx_spi_of_match);

static mut xilinx_slave_spi_driver: spi_driver = spi_driver {
    driver: driver {
        name: "xlnx-slave-spi",
        of_match_table: of_match_ptr(xlnx_spi_of_match),
    },
    probe: Some(xilinx_spi_probe),
    id_table: xilinx_spi_ids.as_ptr(),
};

// module_spi_driver(xilinx_slave_spi_driver)
// MODULE_LICENSE("GPL v2");
// MODULE_AUTHOR("Anatolij Gustschin <agust@denx.de>");
// MODULE_DESCRIPTION("Load Xilinx FPGA firmware over SPI");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
