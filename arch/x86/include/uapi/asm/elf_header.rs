/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency: `__u32` from <linux/types.h> is represented by Rust `u32`.
#[repr(C, packed)]
pub struct x86_xfeat_component {
    pub r#type: u32,
    pub size: u32,
    pub offset: u32,
    pub flags: u32,
}

const _: () = {
    assert!(core::mem::size_of::<x86_xfeat_component>() % 4 == 0);
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
