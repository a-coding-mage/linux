/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef RUST_X86_PLATFORM_CALLBACKS_BINDINGS_H
#define RUST_X86_PLATFORM_CALLBACKS_BINDINGS_H

/* The original owner's includes, plus explicit dependencies of the Rust code.
 * This header supplies native types/declarations and constant expressions only.
 * No implementation of x86_init.c is compiled through this header.
 */
#include <linux/dmi.h>
#include <linux/init.h>
#include <linux/ioport.h>
#include <linux/export.h>
#include <linux/pci.h>
#include <linux/acpi.h>
#include <linux/sizes.h>
#include <linux/errno.h>
#include <linux/of.h>
#include <linux/time64.h>

#include <asm/acpi.h>
#include <asm/bios_ebda.h>
#include <asm/paravirt.h>
#include <asm/pci_x86.h>
#include <asm/mpspec.h>
#include <asm/setup.h>
#include <asm/apic.h>
#include <asm/e820/api.h>
#include <asm/time.h>
#include <asm/irq.h>
#include <asm/io_apic.h>
#include <asm/hpet.h>
#include <asm/memtype.h>
#include <asm/tsc.h>
#include <asm/iommu.h>
#include <asm/mach_traps.h>
#include <asm/irqdomain.h>
#include <asm/realmode.h>
#include <asm/pgtable.h>
#include <asm/x86_init.h>

#ifdef __BINDGEN__
enum rust_x86_platform_callbacks_constants {
	RUST_X86_PLATFORM_EINVAL = EINVAL,
	RUST_X86_PLATFORM_SZ_1M = SZ_1M,
	RUST_X86_PLATFORM_ISA_START_ADDRESS = ISA_START_ADDRESS,
	RUST_X86_PLATFORM_ISA_END_ADDRESS = ISA_END_ADDRESS,
	RUST_X86_PLATFORM_NMI_REASON_PORT = NMI_REASON_PORT,
};
#endif

#endif /* RUST_X86_PLATFORM_CALLBACKS_BINDINGS_H */
