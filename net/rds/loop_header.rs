/* SPDX-License-Identifier: GPL-2.0 */

// External dependency: `struct rds_transport` is defined elsewhere.
extern "C" {
    pub static mut rds_loop_transport: rds_transport;

    pub fn rds_loop_net_init() -> ::std::os::raw::c_int;
    pub fn rds_loop_net_exit();
    pub fn rds_loop_exit();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
