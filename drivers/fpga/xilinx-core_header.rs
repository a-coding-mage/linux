/* SPDX-License-Identifier: GPL-2.0-only */

// Dependency equivalent of: #include <linux/device.h>

// External types supplied by the Linux device and GPIO dependencies.
pub use linux_device::{device, gpio_desc};

/**
 * struct xilinx_fpga_core - interface between the driver and the core manager
 *                           of Xilinx 7 Series FPGA manager
 * @dev:       device node
 * @write:     write callback of the driver
 */
#[repr(C)]
pub struct xilinx_fpga_core {
    /* public: */
    pub dev: *mut device,
    pub write: Option<unsafe extern "C" fn(
        core: *mut xilinx_fpga_core,
        buf: *const core::ffi::c_char,
        count: usize,
    ) -> core::ffi::c_int>,
    /* private: handled by xilinx-core */
    prog_b: *mut gpio_desc,
    init_b: *mut gpio_desc,
    done: *mut gpio_desc,
}

unsafe extern "C" {
    pub fn xilinx_core_probe(core: *mut xilinx_fpga_core) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
