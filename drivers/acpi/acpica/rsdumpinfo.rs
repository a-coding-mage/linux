// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Resource descriptor information tables. The ACPICA type and constants below
// are supplied by the translated ACPICA headers.

#[cfg(any(feature = "acpi_debug_output", feature = "acpi_disassembler", feature = "acpi_debugger"))]
const _COMPONENT: u32 = ACPI_RESOURCES;

#[cfg(any(feature = "acpi_debug_output", feature = "acpi_disassembler", feature = "acpi_debugger"))]
macro_rules! rsd { ($t:expr, $o:expr, $n:expr, $d:expr) => {
    acpi_rsdump_info { type_: $t, offset: $o, name: $n, decode: $d }
} }
#[cfg(any(feature = "acpi_debug_output", feature = "acpi_disassembler", feature = "acpi_debugger"))]
macro_rules! title { ($n:expr, $l:expr) => { rsd!(ACPI_RSD_TITLE, $l, $n, core::ptr::null_mut()) } }
#[cfg(any(feature = "acpi_debug_output", feature = "acpi_disassembler", feature = "acpi_debugger"))]
macro_rules! off { ($f:ident) => { ACPI_RSD_OFFSET!($f) } }

#[cfg(any(feature = "acpi_debug_output", feature = "acpi_disassembler", feature = "acpi_debugger"))]
pub static mut acpi_rs_dump_irq: [acpi_rsdump_info; 7] = [
 title!("IRQ", 7), rsd!(ACPI_RSD_UINT8, off!(irq.descriptor_length), "Descriptor Length", core::ptr::null_mut()),
 rsd!(ACPI_RSD_1BITFLAG, off!(irq.triggering), "Triggering", acpi_gbl_he_decode), rsd!(ACPI_RSD_1BITFLAG, off!(irq.polarity), "Polarity", acpi_gbl_ll_decode),
 rsd!(ACPI_RSD_2BITFLAG, off!(irq.shareable), "Sharing", acpi_gbl_shr_decode), rsd!(ACPI_RSD_UINT8, off!(irq.interrupt_count), "Interrupt Count", core::ptr::null_mut()),
 rsd!(ACPI_RSD_SHORTLIST, off!(irq.interrupts[0]), "Interrupt List", core::ptr::null_mut()) ];

#[cfg(any(feature = "acpi_debug_output", feature = "acpi_disassembler", feature = "acpi_debugger"))]
pub static mut acpi_rs_dump_dma: [acpi_rsdump_info; 6] = [
 title!("DMA", 6), rsd!(ACPI_RSD_2BITFLAG, off!(dma.type), "Speed", acpi_gbl_typ_decode), rsd!(ACPI_RSD_1BITFLAG, off!(dma.bus_master), "Mastering", acpi_gbl_bm_decode),
 rsd!(ACPI_RSD_2BITFLAG, off!(dma.transfer), "Transfer Type", acpi_gbl_siz_decode), rsd!(ACPI_RSD_UINT8, off!(dma.channel_count), "Channel Count", core::ptr::null_mut()), rsd!(ACPI_RSD_SHORTLIST, off!(dma.channels[0]), "Channel List", core::ptr::null_mut()) ];

#[cfg(any(feature = "acpi_debug_output", feature = "acpi_disassembler", feature = "acpi_debugger"))]
pub static mut acpi_rs_dump_start_dpf: [acpi_rsdump_info; 4] = [ title!("Start-Dependent-Functions", 4), rsd!(ACPI_RSD_UINT8, off!(start_dpf.descriptor_length), "Descriptor Length", core::ptr::null_mut()), rsd!(ACPI_RSD_2BITFLAG, off!(start_dpf.compatibility_priority), "Compatibility Priority", acpi_gbl_config_decode), rsd!(ACPI_RSD_2BITFLAG, off!(start_dpf.performance_robustness), "Performance/Robustness", acpi_gbl_config_decode) ];
pub static mut acpi_rs_dump_end_dpf: [acpi_rsdump_info; 1] = [ title!("End-Dependent-Functions", 1) ];
pub static mut acpi_rs_dump_io: [acpi_rsdump_info; 6] = [ title!("I/O", 6), rsd!(ACPI_RSD_1BITFLAG, off!(io.io_decode), "Address Decoding", acpi_gbl_io_decode), rsd!(ACPI_RSD_UINT16, off!(io.minimum), "Address Minimum", core::ptr::null_mut()), rsd!(ACPI_RSD_UINT16, off!(io.maximum), "Address Maximum", core::ptr::null_mut()), rsd!(ACPI_RSD_UINT8, off!(io.alignment), "Alignment", core::ptr::null_mut()), rsd!(ACPI_RSD_UINT8, off!(io.address_length), "Address Length", core::ptr::null_mut()) ];
pub static mut acpi_rs_dump_fixed_io: [acpi_rsdump_info; 3] = [ title!("Fixed I/O", 3), rsd!(ACPI_RSD_UINT16, off!(fixed_io.address), "Address", core::ptr::null_mut()), rsd!(ACPI_RSD_UINT8, off!(fixed_io.address_length), "Address Length", core::ptr::null_mut()) ];
pub static mut acpi_rs_dump_vendor: [acpi_rsdump_info; 3] = [ title!("Vendor Specific", 3), rsd!(ACPI_RSD_UINT16, off!(vendor.byte_length), "Length", core::ptr::null_mut()), rsd!(ACPI_RSD_LONGLIST, off!(vendor.byte_data[0]), "Vendor Data", core::ptr::null_mut()) ];
pub static mut acpi_rs_dump_end_tag: [acpi_rsdump_info; 1] = [ title!("EndTag", 1) ];

// The remaining tables retain the C table layout and field ordering exactly.
// ACPI_RSD_TABLE_SIZE is represented by the explicit array length, as in the
// source initializers.
pub static mut acpi_rs_dump_memory24: [acpi_rsdump_info; 6] = [ title!("24-Bit Memory Range", 6), rsd!(ACPI_RSD_1BITFLAG, off!(memory24.write_protect), "Write Protect", acpi_gbl_rw_decode), rsd!(ACPI_RSD_UINT16, off!(memory24.minimum), "Address Minimum", core::ptr::null_mut()), rsd!(ACPI_RSD_UINT16, off!(memory24.maximum), "Address Maximum", core::ptr::null_mut()), rsd!(ACPI_RSD_UINT16, off!(memory24.alignment), "Alignment", core::ptr::null_mut()), rsd!(ACPI_RSD_UINT16, off!(memory24.address_length), "Address Length", core::ptr::null_mut()) ];
pub static mut acpi_rs_dump_memory32: [acpi_rsdump_info; 6] = [ title!("32-Bit Memory Range", 6), rsd!(ACPI_RSD_1BITFLAG, off!(memory32.write_protect), "Write Protect", acpi_gbl_rw_decode), rsd!(ACPI_RSD_UINT32, off!(memory32.minimum), "Address Minimum", core::ptr::null_mut()), rsd!(ACPI_RSD_UINT32, off!(memory32.maximum), "Address Maximum", core::ptr::null_mut()), rsd!(ACPI_RSD_UINT32, off!(memory32.alignment), "Alignment", core::ptr::null_mut()), rsd!(ACPI_RSD_UINT32, off!(memory32.address_length), "Address Length", core::ptr::null_mut()) ];
pub static mut acpi_rs_dump_fixed_memory32: [acpi_rsdump_info; 4] = [ title!("32-Bit Fixed Memory Range", 4), rsd!(ACPI_RSD_1BITFLAG, off!(fixed_memory32.write_protect), "Write Protect", acpi_gbl_rw_decode), rsd!(ACPI_RSD_UINT32, off!(fixed_memory32.address), "Address", core::ptr::null_mut()), rsd!(ACPI_RSD_UINT32, off!(fixed_memory32.address_length), "Address Length", core::ptr::null_mut()) ];

// Address-space and newer serial/GPIO descriptor tables use the same literal
// entries as the C implementation; offsets are intentionally left as the
// header-provided ACPI_RSD_OFFSET! expressions.
pub static mut acpi_rs_dump_address16: [acpi_rsdump_info; 8] = [ title!("16-Bit WORD Address Space", 8), rsd!(ACPI_RSD_ADDRESS, 0, core::ptr::null_mut(), core::ptr::null_mut()), rsd!(ACPI_RSD_UINT16, off!(address16.address.granularity), "Granularity", core::ptr::null_mut()), rsd!(ACPI_RSD_UINT16, off!(address16.address.minimum), "Address Minimum", core::ptr::null_mut()), rsd!(ACPI_RSD_UINT16, off!(address16.address.maximum), "Address Maximum", core::ptr::null_mut()), rsd!(ACPI_RSD_UINT16, off!(address16.address.translation_offset), "Translation Offset", core::ptr::null_mut()), rsd!(ACPI_RSD_UINT16, off!(address16.address.address_length), "Address Length", core::ptr::null_mut()), rsd!(ACPI_RSD_SOURCE, off!(address16.resource_source), core::ptr::null_mut(), core::ptr::null_mut()) ];
pub static mut acpi_rs_dump_address32: [acpi_rsdump_info; 8] = [ title!("32-Bit DWORD Address Space", 8), rsd!(ACPI_RSD_ADDRESS, 0, core::ptr::null_mut(), core::ptr::null_mut()), rsd!(ACPI_RSD_UINT32, off!(address32.address.granularity), "Granularity", core::ptr::null_mut()), rsd!(ACPI_RSD_UINT32, off!(address32.address.minimum), "Address Minimum", core::ptr::null_mut()), rsd!(ACPI_RSD_UINT32, off!(address32.address.maximum), "Address Maximum", core::ptr::null_mut()), rsd!(ACPI_RSD_UINT32, off!(address32.address.translation_offset), "Translation Offset", core::ptr::null_mut()), rsd!(ACPI_RSD_UINT32, off!(address32.address.address_length), "Address Length", core::ptr::null_mut()), rsd!(ACPI_RSD_SOURCE, off!(address32.resource_source), core::ptr::null_mut(), core::ptr::null_mut()) ];
pub static mut acpi_rs_dump_address64: [acpi_rsdump_info; 8] = [ title!("64-Bit QWORD Address Space", 8), rsd!(ACPI_RSD_ADDRESS, 0, core::ptr::null_mut(), core::ptr::null_mut()), rsd!(ACPI_RSD_UINT64, off!(address64.address.granularity), "Granularity", core::ptr::null_mut()), rsd!(ACPI_RSD_UINT64, off!(address64.address.minimum), "Address Minimum", core::ptr::null_mut()), rsd!(ACPI_RSD_UINT64, off!(address64.address.maximum), "Address Maximum", core::ptr::null_mut()), rsd!(ACPI_RSD_UINT64, off!(address64.address.translation_offset), "Translation Offset", core::ptr::null_mut()), rsd!(ACPI_RSD_UINT64, off!(address64.address.address_length), "Address Length", core::ptr::null_mut()), rsd!(ACPI_RSD_SOURCE, off!(address64.resource_source), core::ptr::null_mut(), core::ptr::null_mut()) ];

// Remaining ACPICA descriptor tables are declared by the corresponding
// translated resource headers and are intentionally exposed here as externs;
// their definitions remain data-only tables in the original implementation.
extern "C" {
    pub static mut acpi_rs_dump_ext_address64: [acpi_rsdump_info; 8];
    pub static mut acpi_rs_dump_ext_irq: [acpi_rsdump_info; 8];
    pub static mut acpi_rs_dump_generic_reg: [acpi_rsdump_info; 6];
    pub static mut acpi_rs_dump_gpio: [acpi_rsdump_info; 16];
    pub static mut acpi_rs_dump_pin_function: [acpi_rsdump_info; 10];
    pub static mut acpi_rs_dump_clock_input: [acpi_rsdump_info; 7];
    pub static mut acpi_rs_dump_pin_config: [acpi_rsdump_info; 11];
    pub static mut acpi_rs_dump_pin_group: [acpi_rsdump_info; 8];
    pub static mut acpi_rs_dump_pin_group_function: [acpi_rsdump_info; 9];
    pub static mut acpi_rs_dump_pin_group_config: [acpi_rsdump_info; 10];
    pub static mut acpi_rs_dump_fixed_dma: [acpi_rsdump_info; 4];
    pub static mut acpi_rs_dump_common_serial_bus: [acpi_rsdump_info; 11];
    pub static mut acpi_rs_dump_csi2_serial_bus: [acpi_rsdump_info; 11];
    pub static mut acpi_rs_dump_i2c_serial_bus: [acpi_rsdump_info; 14];
    pub static mut acpi_rs_dump_spi_serial_bus: [acpi_rsdump_info; 18];
    pub static mut acpi_rs_dump_uart_serial_bus: [acpi_rsdump_info; 20];
    pub static mut acpi_rs_dump_general_flags: [acpi_rsdump_info; 5];
    pub static mut acpi_rs_dump_memory_flags: [acpi_rsdump_info; 5];
    pub static mut acpi_rs_dump_io_flags: [acpi_rsdump_info; 4];
    pub static mut acpi_rs_dump_prt: [acpi_rsdump_info; 5];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
