/* SPDX-License-Identifier: GPL-2.0 */

use std::os::raw::{c_char, c_int};

unsafe extern "C" {
    pub fn pager_init(pager_env: *const c_char);

    pub fn setup_pager();
    pub fn pager_in_use() -> c_int;
    pub fn pager_get_columns() -> c_int;
    pub fn force_pager(arg1: *const c_char);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
