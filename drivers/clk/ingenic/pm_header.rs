/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2019 Paul Cercueil <paul@crapouillou.net>
 */

#[repr(C)]
pub struct ingenic_cgu {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn ingenic_cgu_register_syscore(cgu: *mut ingenic_cgu);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
