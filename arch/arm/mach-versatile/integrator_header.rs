/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding translation unit:
// #include <linux/reboot.h>
// #include <linux/amba/serial.h>

extern "C" {
    pub static mut ap_uart_data: amba_pl010_data;

    pub fn integrator_init_early();
    pub fn integrator_init(is_cp: bool) -> i32;
    pub fn integrator_reserve();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
