/* SPDX-License-Identifier: MIT */
/******************************************************************************
 * elfnote.h
 *
 * Definitions used for the Xen ELF notes.
 *
 * Copyright (c) 2006, Ian Campbell, XenSource Ltd.
 */

/*
 * `incontents 200 elfnotes ELF notes
 *
 * The notes should live in a PT_NOTE segment and have "Xen" in the
 * name field.
 *
 * Numeric types are either 4 or 8 bytes depending on the content of
 * the desc field.
 *
 * LEGACY indicated the fields in the legacy __xen_guest string which
 * this a note type replaces.
 *
 * String values (for non-legacy) are NULL terminated ASCII, also known
 * as ASCIZ type.
 *
 * Xen only uses ELF Notes contained in x86 binaries.
 */

/* NAME=VALUE pair (string). */
pub const XEN_ELFNOTE_INFO: i32 = 0;

/* The virtual address of the entry point (numeric).
 *
 * LEGACY: VIRT_ENTRY
 */
pub const XEN_ELFNOTE_ENTRY: i32 = 1;

/* The virtual address of the hypercall transfer page (numeric).
 *
 * LEGACY: HYPERCALL_PAGE. (n.b. legacy value is a physical page
 * number not a virtual address)
 */
pub const XEN_ELFNOTE_HYPERCALL_PAGE: i32 = 2;

/* The virtual address where the kernel image should be mapped (numeric).
 *
 * Defaults to 0.
 *
 * LEGACY: VIRT_BASE
 */
pub const XEN_ELFNOTE_VIRT_BASE: i32 = 3;

/* The offset of the ELF paddr field from the actual required
 * pseudo-physical address (numeric).
 *
 * This is used to maintain backwards compatibility with older kernels
 * which wrote __PAGE_OFFSET into that field. This field defaults to 0
 * if not present.
 *
 * LEGACY: ELF_PADDR_OFFSET. (n.b. legacy default is VIRT_BASE)
 */
pub const XEN_ELFNOTE_PADDR_OFFSET: i32 = 4;

/* The version of Xen that we work with (string).
 *
 * LEGACY: XEN_VER
 */
pub const XEN_ELFNOTE_XEN_VERSION: i32 = 5;

/* The name of the guest operating system (string).
 *
 * LEGACY: GUEST_OS
 */
pub const XEN_ELFNOTE_GUEST_OS: i32 = 6;

/* The version of the guest operating system (string).
 *
 * LEGACY: GUEST_VER
 */
pub const XEN_ELFNOTE_GUEST_VERSION: i32 = 7;

/* The loader type (string).
 *
 * LEGACY: LOADER
 */
pub const XEN_ELFNOTE_LOADER: i32 = 8;

/* The kernel supports PAE (x86/32 only, string = "yes", "no" or
 * "bimodal").
 *
 * For compatibility with Xen 3.0.3 and earlier the "bimodal" setting
 * may be given as "yes,bimodal" which will cause older Xen to treat
 * this kernel as PAE.
 *
 * LEGACY: PAE (n.b. The legacy interface included a provision to
 * indicate 'extended-cr3' support allowing L3 page tables to be
 * placed above 4G. It is assumed that any kernel new enough to use
 * these ELF notes will include this and therefore "yes" here is
 * equivalent to "yes[entended-cr3]" in the __xen_guest interface.
 */
pub const XEN_ELFNOTE_PAE_MODE: i32 = 9;

/* The features supported/required by this kernel (string).
 *
 * The string must consist of a list of feature names (as given in
 * features.h, without the "XENFEAT_" prefix) separated by '|'
 * characters. If a feature is required for the kernel to function
 * then the feature name must be preceded by a '!' character.
 *
 * LEGACY: FEATURES
 */
pub const XEN_ELFNOTE_FEATURES: i32 = 10;

/* The kernel requires the symbol table to be loaded (string = "yes" or "no")
 * LEGACY: BSD_SYMTAB (n.b. The legacy treated the presence or absence
 * of this string as a boolean flag rather than requiring "yes" or
 * "no".
 */
pub const XEN_ELFNOTE_BSD_SYMTAB: i32 = 11;

/* The lowest address the hypervisor hole can begin at (numeric).
 *
 * This must not be set higher than HYPERVISOR_VIRT_START. Its presence
 * also indicates to the hypervisor that the kernel can deal with the
 * hole starting at a higher address.
 */
pub const XEN_ELFNOTE_HV_START_LOW: i32 = 12;

/* List of maddr_t-sized mask/value pairs describing how to recognize
 * (non-present) L1 page table entries carrying valid MFNs (numeric).
 */
pub const XEN_ELFNOTE_L1_MFN_VALID: i32 = 13;

/* Whether or not the guest supports cooperative suspend cancellation.
 * This is a numeric value.
 *
 * Default is 0
 */
pub const XEN_ELFNOTE_SUSPEND_CANCEL: i32 = 14;

/* The (non-default) location the initial phys-to-machine map should be
 * placed at by the hypervisor (Dom0) or the tools (DomU).
 * The kernel must be prepared for this mapping to be established using
 * large pages, despite such otherwise not being available to guests. Note
 * that these large pages may be misaligned in PFN space (they'll obviously
 * be aligned in MFN and virtual address spaces).
 * The kernel must also be able to handle the page table pages used for
 * this mapping not being accessible through the initial mapping.
 * (Only x86-64 supports this at present.)
 */
pub const XEN_ELFNOTE_INIT_P2M: i32 = 15;

/* Whether or not the guest can deal with being passed an initrd not
 * mapped through its initial page tables.
 */
pub const XEN_ELFNOTE_MOD_START_PFN: i32 = 16;

/* The features supported by this kernel (numeric).
 *
 * Other than XEN_ELFNOTE_FEATURES on pre-4.2 Xen, this note allows a
 * kernel to specify support for features that older hypervisors don't
 * know about. The set of features 4.2 and newer hypervisors will
 * consider supported by the kernel is the combination of the sets
 * specified through this and the string note.
 *
 * LEGACY: FEATURES
 */
pub const XEN_ELFNOTE_SUPPORTED_FEATURES: i32 = 17;

/* Physical entry point into the kernel.
 *
 * 32bit entry point into the kernel. When requested to launch the
 * guest kernel in a HVM container, Xen will use this entry point to
 * launch the guest in 32bit protected mode with paging disabled.
 * Ignored otherwise.
 */
pub const XEN_ELFNOTE_PHYS32_ENTRY: i32 = 18;

/* Physical loading constraints for PVH kernels
 *
 * The presence of this note indicates the kernel supports relocating itself.
 *
 * The note may include up to three 32bit values to place constraints on the
 * guest physical loading addresses and alignment for a PVH kernel. Values
 * are read in the following order:
 *  - a required start alignment (default 0x200000)
 *  - a minimum address for the start of the image (default 0; see below)
 *  - a maximum address for the last byte of the image (default 0xffffffff)
 *
 * When this note specifies an alignment value, it is used. Otherwise, the
 * maximum p_align value from loadable ELF Program Headers is used, if it is
 * greater than or equal to 4k (0x1000). Otherwise, the default is used.
 */
pub const XEN_ELFNOTE_PHYS32_RELOC: i32 = 19;

/* The number of the highest elfnote defined. */
pub const XEN_ELFNOTE_MAX: i32 = XEN_ELFNOTE_PHYS32_RELOC;

/* System information exported through crash notes.
 *
 * The kexec / kdump code will create one XEN_ELFNOTE_CRASH_INFO
 * note in case of a system crash. This note will contain various
 * information about the system, see xen/include/xen/elfcore.h.
 */
pub const XEN_ELFNOTE_CRASH_INFO: u32 = 0x1000001;

/* System registers exported through crash notes.
 *
 * The kexec / kdump code will create one XEN_ELFNOTE_CRASH_REGS
 * note per cpu in case of a system crash. This note is architecture
 * specific and will contain registers not saved in the "CORE" note.
 * See xen/include/xen/elfcore.h for more information.
 */
pub const XEN_ELFNOTE_CRASH_REGS: u32 = 0x1000002;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
