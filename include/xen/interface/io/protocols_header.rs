/* SPDX-License-Identifier: MIT */

pub const XEN_IO_PROTO_ABI_X86_32: &str = "x86_32-abi";
pub const XEN_IO_PROTO_ABI_X86_64: &str = "x86_64-abi";
pub const XEN_IO_PROTO_ABI_POWERPC64: &str = "powerpc64-abi";
pub const XEN_IO_PROTO_ABI_ARM: &str = "arm-abi";

#[cfg(target_arch = "x86")]
pub const XEN_IO_PROTO_ABI_NATIVE: &str = XEN_IO_PROTO_ABI_X86_32;

#[cfg(target_arch = "x86_64")]
pub const XEN_IO_PROTO_ABI_NATIVE: &str = XEN_IO_PROTO_ABI_X86_64;

#[cfg(target_arch = "powerpc64")]
pub const XEN_IO_PROTO_ABI_NATIVE: &str = XEN_IO_PROTO_ABI_POWERPC64;

#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
pub const XEN_IO_PROTO_ABI_NATIVE: &str = XEN_IO_PROTO_ABI_ARM;

#[cfg(not(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "powerpc64",
    target_arch = "arm",
    target_arch = "aarch64",
)))]
compile_error!("arch fixup needed here");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
