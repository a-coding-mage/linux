/* SPDX-License-Identifier: GPL-2.0 */
#ifndef RUST_X86_RESOURCE_BINDINGS_H
#define RUST_X86_RESOURCE_BINDINGS_H
#include <linux/ioport.h>
#include <linux/printk.h>
#include <asm/e820/api.h>
#include <asm/pci_x86.h>

#ifndef CONFIG_X86_64
#error "Rust resource currently requires x86-64"
#endif
#ifdef __BINDGEN__
static const resource_size_t RUST_RESOURCE_BIOS_ROM_BASE = BIOS_ROM_BASE;
static const resource_size_t RUST_RESOURCE_BIOS_ROM_END = BIOS_ROM_END;
static const unsigned long RUST_RESOURCE_IORESOURCE_MEM = IORESOURCE_MEM;
#endif
#endif
