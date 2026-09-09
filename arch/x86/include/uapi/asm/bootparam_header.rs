/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Dependency declarations from the C header are supplied by other translated files. */

/* ram_size flags */
pub const RAMDISK_IMAGE_START_MASK: u16 = 0x07ff;
pub const RAMDISK_PROMPT_FLAG: u16 = 0x8000;
pub const RAMDISK_LOAD_FLAG: u16 = 0x4000;

/* loadflags */
pub const LOADED_HIGH: u8 = 1 << 0;
pub const KASLR_FLAG: u8 = 1 << 1;
pub const QUIET_FLAG: u8 = 1 << 5;
pub const KEEP_SEGMENTS: u8 = 1 << 6;
pub const CAN_USE_HEAP: u8 = 1 << 7;

/* xloadflags */
pub const XLF_KERNEL_64: u16 = 1 << 0;
pub const XLF_CAN_BE_LOADED_ABOVE_4G: u16 = 1 << 1;
pub const XLF_EFI_HANDOVER_32: u16 = 1 << 2;
pub const XLF_EFI_HANDOVER_64: u16 = 1 << 3;
pub const XLF_EFI_KEXEC: u16 = 1 << 4;
pub const XLF_5LEVEL: u16 = 1 << 5;
pub const XLF_5LEVEL_ENABLED: u16 = 1 << 6;
pub const XLF_MEM_ENCRYPTION: u16 = 1 << 7;

/* The declarations below are excluded by the C header when compiling as assembler. */

#[repr(C, packed)]
pub struct setup_header {
    pub setup_sects: u8,
    pub root_flags: u16,
    pub syssize: u32,
    pub ram_size: u16,
    pub vid_mode: u16,
    pub root_dev: u16,
    pub boot_flag: u16,
    pub jump: u16,
    pub header: u32,
    pub version: u16,
    pub realmode_swtch: u32,
    pub start_sys_seg: u16,
    pub kernel_version: u16,
    pub type_of_loader: u8,
    pub loadflags: u8,
    pub setup_move_size: u16,
    pub code32_start: u32,
    pub ramdisk_image: u32,
    pub ramdisk_size: u32,
    pub bootsect_kludge: u32,
    pub heap_end_ptr: u16,
    pub ext_loader_ver: u8,
    pub ext_loader_type: u8,
    pub cmd_line_ptr: u32,
    pub initrd_addr_max: u32,
    pub kernel_alignment: u32,
    pub relocatable_kernel: u8,
    pub min_alignment: u8,
    pub xloadflags: u16,
    pub cmdline_size: u32,
    pub hardware_subarch: u32,
    pub hardware_subarch_data: u64,
    pub payload_offset: u32,
    pub payload_length: u32,
    pub setup_data: u64,
    pub pref_address: u64,
    pub init_size: u32,
    pub handover_offset: u32,
    pub kernel_info_offset: u32,
}

#[repr(C)]
pub struct sys_desc_table {
    pub length: u16,
    pub table: [u8; 14],
}

/* Gleaned from OFW's set-parameters in cpu/x86/pc/linux.fth */
#[repr(C, packed)]
pub struct olpc_ofw_header {
    pub ofw_magic: u32, /* OFW signature */
    pub ofw_version: u32,
    pub cif_handler: u32, /* callback into OFW */
    pub irq_desc_table: u32,
}

#[repr(C)]
pub struct efi_info {
    pub efi_loader_signature: u32,
    pub efi_systab: u32,
    pub efi_memdesc_size: u32,
    pub efi_memdesc_version: u32,
    pub efi_memmap: u32,
    pub efi_memmap_size: u32,
    pub efi_systab_hi: u32,
    pub efi_memmap_hi: u32,
}

/* This is the maximum number of entries in struct boot_params::e820_table
 * (the zeropage), which is part of the x86 boot protocol ABI:
 */
pub const E820_MAX_ENTRIES_ZEROPAGE: usize = 128;

/* Smallest compatible version of jailhouse_setup_data required by this kernel. */
pub const JAILHOUSE_SETUP_REQUIRED_VERSION: u32 = 1;

/* The so-called "zeropage" */
#[repr(C, packed)]
pub struct boot_params {
    pub screen_info: screen_info,
    pub apm_bios_info: apm_bios_info,
    pub _pad2: [u8; 4],
    pub tboot_addr: u64,
    pub ist_info: ist_info,
    pub acpi_rsdp_addr: u64,
    pub _pad3: [u8; 8],
    pub hd0_info: [u8; 16], /* obsolete! */
    pub hd1_info: [u8; 16], /* obsolete! */
    pub sys_desc_table: sys_desc_table, /* obsolete! */
    pub olpc_ofw_header: olpc_ofw_header,
    pub ext_ramdisk_image: u32,
    pub ext_ramdisk_size: u32,
    pub ext_cmd_line_ptr: u32,
    pub _pad4: [u8; 112],
    pub cc_blob_address: u32,
    pub edid_info: edid_info,
    pub efi_info: efi_info,
    pub alt_mem_k: u32,
    pub scratch: u32, /* Scratch field! */
    pub e820_entries: u8,
    pub eddbuf_entries: u8,
    pub edd_mbr_sig_buf_entries: u8,
    pub kbd_status: u8,
    pub secure_boot: u8,
    pub _pad5: [u8; 2],
    /*
     * The sentinel is set to a nonzero value (0xff) in header.S.
     *
     * A bootloader is supposed to only take setup_header and put
     * it into a clean boot_params buffer. If it turns out that
     * it is clumsy or too generous with the buffer, it most
     * probably will pick up the sentinel variable too. The fact
     * that this variable then is still 0xff will let kernel
     * know that some variables in boot_params are invalid and
     * kernel should zero out certain portions of boot_params.
     */
    pub sentinel: u8,
    pub _pad6: [u8; 1],
    pub hdr: setup_header, /* setup header */
    pub _pad7: [u8; 0x290 - 0x1f1 - core::mem::size_of::<setup_header>()],
    pub edd_mbr_sig_buffer: [u32; EDD_MBR_SIG_MAX],
    pub e820_table: [boot_e820_entry; E820_MAX_ENTRIES_ZEROPAGE],
    pub _pad8: [u8; 48],
    pub eddbuf: [edd_info; EDDMAXNR],
    pub _pad9: [u8; 276],
}

/**
 * enum x86_hardware_subarch - x86 hardware subarchitecture
 *
 * The x86 hardware_subarch and hardware_subarch_data were added as of the x86
 * boot protocol 2.07 to help distinguish and support custom x86 boot
 * sequences. This enum represents accepted values for the x86
 * hardware_subarch. Custom x86 boot sequences (not X86_SUBARCH_PC) do not
 * have or simply *cannot* make use of natural stubs like BIOS or EFI, the
 * hardware_subarch can be used on the Linux entry path to revector to a
 * subarchitecture stub when needed. This subarchitecture stub can be used to
 * set up Linux boot parameters or for special care to account for nonstandard
 * handling of page tables.
 *
 * These enums should only ever be used by x86 code, and the code that uses
 * it should be well contained and compartmentalized.
 *
 * KVM and Xen HVM do not have a subarch as these are expected to follow
 * standard x86 boot entries. If there is a genuine need for "hypervisor" type
 * that should be considered separately in the future. Future guest types
 * should seriously consider working with standard x86 boot stubs such as
 * the BIOS or EFI boot stubs.
 *
 * WARNING: this enum is only used for legacy hacks, for platform features that
 * are not easily enumerated or discoverable. You should not ever use this
 * for new features.
 */
#[repr(C)]
pub enum x86_hardware_subarch {
    X86_SUBARCH_PC = 0,
    X86_SUBARCH_LGUEST,
    X86_SUBARCH_XEN,
    X86_SUBARCH_INTEL_MID,
    X86_SUBARCH_CE4100,
    X86_NR_SUBARCHS,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
