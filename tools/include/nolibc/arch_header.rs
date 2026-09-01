// SPDX-License-Identifier: LGPL-2.1 OR MIT
/*
 * Copyright (C) 2017-2022 Willy Tarreau <w@1wt.eu>
 */

// Rust translation of nolibc's architecture dispatcher header.
// The original C header selects one architecture-specific header with
// preprocessor checks. These cfg-gated re-exports preserve that dependency
// intent without implementing the architecture-specific files here.

#[cfg(any(
    target_arch = "x86_64",
    target_arch = "x86",
))]
pub use crate::arch_x86::*;

#[cfg(target_arch = "arm")]
pub use crate::arch_arm::*;

#[cfg(target_arch = "aarch64")]
pub use crate::arch_arm64::*;

#[cfg(target_arch = "mips")]
pub use crate::arch_mips::*;

#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
pub use crate::arch_powerpc::*;

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
pub use crate::arch_riscv::*;

#[cfg(target_arch = "s390x")]
pub use crate::arch_s390::*;

#[cfg(any(target_arch = "loongarch32", target_arch = "loongarch64"))]
pub use crate::arch_loongarch::*;

#[cfg(any(target_arch = "sparc", target_arch = "sparc64"))]
pub use crate::arch_sparc::*;

#[cfg(target_arch = "m68k")]
pub use crate::arch_m68k::*;

#[cfg(target_arch = "sh")]
pub use crate::arch_sh::*;

#[cfg(target_arch = "or1k")]
pub use crate::arch_openrisc::*;

#[cfg(target_arch = "hppa")]
pub use crate::arch_parisc::*;

#[cfg(target_arch = "alpha")]
pub use crate::arch_alpha::*;

#[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "mips",
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "riscv32",
    target_arch = "riscv64",
    target_arch = "s390x",
    target_arch = "loongarch32",
    target_arch = "loongarch64",
    target_arch = "sparc",
    target_arch = "sparc64",
    target_arch = "m68k",
    target_arch = "sh",
    target_arch = "or1k",
    target_arch = "hppa",
    target_arch = "alpha",
)))]
compile_error!("Unsupported Architecture");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
