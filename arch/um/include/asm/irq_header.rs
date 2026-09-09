/* SPDX-License-Identifier: GPL-2.0 */

pub const TIMER_IRQ: i32 = 0;
pub const UMN_IRQ: i32 = 1;
pub const UBD_IRQ: i32 = 2;
pub const UM_ETH_IRQ: i32 = 3;
pub const ACCEPT_IRQ: i32 = 4;
pub const MCONSOLE_IRQ: i32 = 5;
pub const WINCH_IRQ: i32 = 6;
pub const SIGIO_WRITE_IRQ: i32 = 7;
pub const TELNETD_IRQ: i32 = 8;
pub const XTERM_IRQ: i32 = 9;
pub const RANDOM_IRQ: i32 = 10;
pub const SIGCHLD_IRQ: i32 = 11;

/* CONFIG_UML_NET_VECTOR is a build-time configuration condition. */
#[cfg(feature = "CONFIG_UML_NET_VECTOR")]
pub const VECTOR_BASE_IRQ: i32 = SIGCHLD_IRQ + 1;
#[cfg(feature = "CONFIG_UML_NET_VECTOR")]
pub const VECTOR_IRQ_SPACE: i32 = 8;
#[cfg(feature = "CONFIG_UML_NET_VECTOR")]
pub const UM_FIRST_DYN_IRQ: i32 = VECTOR_IRQ_SPACE + VECTOR_BASE_IRQ;

#[cfg(not(feature = "CONFIG_UML_NET_VECTOR"))]
pub const UM_FIRST_DYN_IRQ: i32 = SIGCHLD_IRQ + 1;

pub const UM_LAST_SIGNAL_IRQ: i32 = 64;
/* If we have (simulated) PCI MSI, allow 64 more interrupt numbers for it. */
/* CONFIG_PCI_MSI is a build-time configuration condition. */
#[cfg(feature = "CONFIG_PCI_MSI")]
pub const NR_IRQS: i32 = UM_LAST_SIGNAL_IRQ + 64;
#[cfg(not(feature = "CONFIG_PCI_MSI"))]
pub const NR_IRQS: i32 = UM_LAST_SIGNAL_IRQ;

/* Declarations from <asm-generic/irq.h> are supplied by another dependency. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
