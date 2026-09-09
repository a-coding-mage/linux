/* SPDX-License-Identifier: GPL-2.0 */

// This header is intended to be included through the Linux spinlock types
// header, not directly.

// When CONFIG_PPC_QUEUED_SPINLOCKS is enabled, the corresponding Rust
// translation must provide the PowerPC queued spinlock types and the generic
// queued read/write lock types.
#[cfg(CONFIG_PPC_QUEUED_SPINLOCKS)]
use crate::qspinlock_types::*;
#[cfg(CONFIG_PPC_QUEUED_SPINLOCKS)]
use crate::qrwlock_types::*;

// Otherwise, the corresponding Rust translation must provide the simple
// spinlock types.
#[cfg(not(CONFIG_PPC_QUEUED_SPINLOCKS))]
use crate::simple_spinlock_types::*;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
