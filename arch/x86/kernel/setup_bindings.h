/* SPDX-License-Identifier: GPL-2.0-only */
#ifndef RUST_X86_SETUP_BINDINGS_H
#define RUST_X86_SETUP_BINDINGS_H
#include <linux/acpi.h>
#include <linux/bootconfig.h>
#include <linux/console.h>
#include <linux/cpu.h>
#include <linux/crash_dump.h>
#include <linux/dma-map-ops.h>
#include <linux/efi.h>
#include <linux/hugetlb.h>
#include <linux/ima.h>
#include <linux/init_ohci1394_dma.h>
#include <linux/initrd.h>
#include <linux/iscsi_ibft.h>
#include <linux/kexec_handover.h>
#include <linux/memblock.h>
#include <linux/panic_notifier.h>
#include <linux/pci.h>
#include <linux/random.h>
#include <linux/root_dev.h>
#include <linux/static_call.h>
#include <linux/sysfb.h>
#include <linux/swiotlb.h>
#include <linux/tboot.h>
#include <linux/usb/xhci-dbgp.h>
#include <linux/vmalloc.h>
#include <uapi/linux/mount.h>
#include <xen/xen.h>
#include <asm/apic.h>
#include <asm/bios_ebda.h>
#include <asm/bugs.h>
#include <asm/cacheinfo.h>
#include <asm/coco.h>
#include <asm/cpu.h>
#include <asm/efi.h>
#include <asm/gart.h>
#include <asm/hypervisor.h>
#include <asm/io_apic.h>
#include <asm/kasan.h>
#include <asm/kaslr.h>
#include <asm/mce.h>
#include <asm/memtype.h>
#include <asm/mtrr.h>
#include <asm/nmi.h>
#include <asm/numa.h>
#include <asm/olpc_ofw.h>
#include <asm/pci-direct.h>
#include <asm/prom.h>
#include <asm/proto.h>
#include <asm/realmode.h>
#include <asm/thermal.h>
#include <asm/unwind.h>
#include <asm/vsyscall.h>

/* SPDX-License-Identifier: GPL-2.0-only */
/* Canonical configured headers: no stand-in data layouts or prototypes. */
#include <linux/kdev_t.h>
#include <linux/sysctl.h>
#include <asm/sections.h>
extern int root_mountflags;
extern char boot_command_line[COMMAND_LINE_SIZE];
extern unsigned long _brk_start, _brk_end;
#ifdef __BINDGEN__
#ifdef CONFIG_CMDLINE_BOOL
#define RUST_SETUP_BUILTIN_CMDLINE CONFIG_CMDLINE
#endif
enum rust_setup_constants {
#define RUST_SETUP_CONST(name, value) RUST_SETUP_##name = (value),
RUST_SETUP_CONST(COMMAND_LINE_SIZE, COMMAND_LINE_SIZE)
RUST_SETUP_CONST(EFI_SECURE_DISABLED, efi_secureboot_mode_disabled)
RUST_SETUP_CONST(EFI_SECURE_ENABLED, efi_secureboot_mode_enabled)
RUST_SETUP_CONST(RESOURCE_RAM_FLAGS, IORESOURCE_BUSY | IORESOURCE_SYSTEM_RAM)
RUST_SETUP_CONST(RESOURCE_IO_FLAGS, IORESOURCE_BUSY | IORESOURCE_IO)
RUST_SETUP_CONST(PAGE_SIZE, PAGE_SIZE)
RUST_SETUP_CONST(PAGE_SHIFT, PAGE_SHIFT)
RUST_SETUP_CONST(PAGE_NX, _PAGE_NX)
RUST_SETUP_CONST(CR4_PCIDE, X86_CR4_PCIDE)
RUST_SETUP_CONST(MINORBITS, MINORBITS)
RUST_SETUP_CONST(BIOS_BEGIN, BIOS_BEGIN)
RUST_SETUP_CONST(BIOS_END, BIOS_END)
RUST_SETUP_CONST(START_KERNEL, __START_KERNEL)
RUST_SETUP_CONST(START_KERNEL_MAP, __START_KERNEL_map)
RUST_SETUP_CONST(MODULES_VADDR, MODULES_VADDR)
#undef RUST_SETUP_CONST
};
static const int RUST_SETUP_NUMA_NO_NODE = NUMA_NO_NODE;
#endif
unsigned long rust_setup_pa_symbol(unsigned long address);
void __noreturn rust_setup_bug(void);
void rust_setup_memzero_explicit(void *address, size_t size);
bool rust_setup_boot_cpu_has_nx(void);
unsigned long rust_setup_read_cr4(void);
unsigned int rust_setup_max_physmem_bits(void);
bool rust_setup_efi_enabled(unsigned int bit);
void rust_setup_set_efi_flag(unsigned int bit);
bool rust_setup_xen_pv_domain(void);

#endif
