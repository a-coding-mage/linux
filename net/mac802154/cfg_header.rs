/* SPDX-License-Identifier: GPL-2.0 */
/* mac802154 configuration hooks for cfg802154
 */

// C dependency: `struct cfg802154_ops` is supplied by another translation unit.
#[repr(C)]
pub struct cfg802154_ops {
    _private: [u8; 0],
}

extern "C" {
    pub static mac802154_config_ops: cfg802154_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
