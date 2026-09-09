/* SPDX-License-Identifier: GPL-2.0 */

// __LINUX_MIPS_DB1XXX__

extern "C" {
    pub fn get_system_type() -> *const core::ffi::c_char;
    pub fn db1000_board_setup() -> core::ffi::c_int;
    pub fn db1000_dev_setup() -> core::ffi::c_int;
    pub fn db1500_pci_setup() -> core::ffi::c_int;
    pub fn db1200_board_setup() -> core::ffi::c_int;
    pub fn db1200_dev_setup() -> core::ffi::c_int;
    pub fn db1300_board_setup() -> core::ffi::c_int;
    pub fn db1300_dev_setup() -> core::ffi::c_int;
    pub fn db1550_board_setup() -> core::ffi::c_int;
    pub fn db1550_dev_setup() -> core::ffi::c_int;
    pub fn db1550_pci_setup(id: core::ffi::c_int) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
