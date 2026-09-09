// SPDX-License-Identifier: GPL-2.0-only
/*
 * Kernel module for testing static keys.
 *
 * Copyright 2015 Akamai Technologies Inc. All Rights Reserved
 *
 * Authors:
 *      Jason Baron       <jbaron@akamai.com>
 */

// The Linux kernel module and jump-label interfaces are supplied externally.

#[repr(C)]
pub struct StaticKey {
    _private: [u8; 0],
}

#[repr(C)]
pub struct StaticKeyTrue {
    pub key: StaticKey,
}

#[repr(C)]
pub struct StaticKeyFalse {
    pub key: StaticKey,
}

extern "C" {
    fn static_key_enabled(key: *const StaticKey) -> bool;
    fn static_key_disable(key: *mut StaticKey);
    fn static_key_enable(key: *mut StaticKey);
}

/* old keys */
#[no_mangle]
pub static mut base_old_true_key: StaticKey = StaticKey { _private: [] };
// EXPORT_SYMBOL_GPL(base_old_true_key);
#[no_mangle]
pub static mut base_inv_old_true_key: StaticKey = StaticKey { _private: [] };
// EXPORT_SYMBOL_GPL(base_inv_old_true_key);
#[no_mangle]
pub static mut base_old_false_key: StaticKey = StaticKey { _private: [] };
// EXPORT_SYMBOL_GPL(base_old_false_key);
#[no_mangle]
pub static mut base_inv_old_false_key: StaticKey = StaticKey { _private: [] };
// EXPORT_SYMBOL_GPL(base_inv_old_false_key);

/* new keys */
#[no_mangle]
pub static mut base_true_key: StaticKeyTrue = StaticKeyTrue {
    key: StaticKey { _private: [] },
};
// EXPORT_SYMBOL_GPL(base_true_key);
#[no_mangle]
pub static mut base_inv_true_key: StaticKeyTrue = StaticKeyTrue {
    key: StaticKey { _private: [] },
};
// EXPORT_SYMBOL_GPL(base_inv_true_key);
#[no_mangle]
pub static mut base_false_key: StaticKeyFalse = StaticKeyFalse {
    key: StaticKey { _private: [] },
};
// EXPORT_SYMBOL_GPL(base_false_key);
#[no_mangle]
pub static mut base_inv_false_key: StaticKeyFalse = StaticKeyFalse {
    key: StaticKey { _private: [] },
};
// EXPORT_SYMBOL_GPL(base_inv_false_key);

unsafe fn invert_key(key: *mut StaticKey) {
    if static_key_enabled(key as *const StaticKey) {
        static_key_disable(key);
    } else {
        static_key_enable(key);
    }
}

unsafe fn test_static_key_base_init() -> i32 {
    invert_key(&raw mut base_inv_old_true_key);
    invert_key(&raw mut base_inv_old_false_key);
    invert_key(&raw mut base_inv_true_key.key);
    invert_key(&raw mut base_inv_false_key.key);

    0
}

unsafe fn test_static_key_base_exit() {}

// module_init(test_static_key_base_init);
// module_exit(test_static_key_base_exit);

// MODULE_AUTHOR("Jason Baron <jbaron@akamai.com>");
// MODULE_DESCRIPTION("Kernel module to support testing static keys");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
