// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Module Name: utresdecode - Resource descriptor keyword strings
//
// The following declarations are enabled when ACPI_DEBUG_OUTPUT,
// ACPI_DISASSEMBLER, or ACPI_DEBUGGER is enabled in the C build.

use core::ffi::c_char;

pub static mut acpi_gbl_bm_decode: [*const c_char; 2] = [
    b"NotBusMaster\0".as_ptr() as *const c_char,
    b"BusMaster\0".as_ptr() as *const c_char,
];
pub static mut acpi_gbl_config_decode: [*const c_char; 4] = [
    b"0 - Good Configuration\0".as_ptr() as *const c_char,
    b"1 - Acceptable Configuration\0".as_ptr() as *const c_char,
    b"2 - Suboptimal Configuration\0".as_ptr() as *const c_char,
    b"3 - ***Invalid Configuration***\0".as_ptr() as *const c_char,
];
pub static mut acpi_gbl_consume_decode: [*const c_char; 2] = [
    b"ResourceProducer\0".as_ptr() as *const c_char,
    b"ResourceConsumer\0".as_ptr() as *const c_char,
];
pub static mut acpi_gbl_dec_decode: [*const c_char; 2] = [
    b"PosDecode\0".as_ptr() as *const c_char,
    b"SubDecode\0".as_ptr() as *const c_char,
];
pub static mut acpi_gbl_he_decode: [*const c_char; 2] = [b"Level\0".as_ptr() as *const c_char, b"Edge\0".as_ptr() as *const c_char];
pub static mut acpi_gbl_io_decode: [*const c_char; 2] = [b"Decode10\0".as_ptr() as *const c_char, b"Decode16\0".as_ptr() as *const c_char];
pub static mut acpi_gbl_ll_decode: [*const c_char; 4] = [b"ActiveHigh\0".as_ptr() as *const c_char, b"ActiveLow\0".as_ptr() as *const c_char, b"ActiveBoth\0".as_ptr() as *const c_char, b"Reserved\0".as_ptr() as *const c_char];
pub static mut acpi_gbl_max_decode: [*const c_char; 2] = [b"MaxNotFixed\0".as_ptr() as *const c_char, b"MaxFixed\0".as_ptr() as *const c_char];
pub static mut acpi_gbl_mem_decode: [*const c_char; 4] = [b"NonCacheable\0".as_ptr() as *const c_char, b"Cacheable\0".as_ptr() as *const c_char, b"WriteCombining\0".as_ptr() as *const c_char, b"Prefetchable\0".as_ptr() as *const c_char];
pub static mut acpi_gbl_min_decode: [*const c_char; 2] = [b"MinNotFixed\0".as_ptr() as *const c_char, b"MinFixed\0".as_ptr() as *const c_char];
pub static mut acpi_gbl_mtp_decode: [*const c_char; 4] = [b"AddressRangeMemory\0".as_ptr() as *const c_char, b"AddressRangeReserved\0".as_ptr() as *const c_char, b"AddressRangeACPI\0".as_ptr() as *const c_char, b"AddressRangeNVS\0".as_ptr() as *const c_char];
pub static mut acpi_gbl_phy_decode: [*const c_char; 4] = [b"Type C\0".as_ptr() as *const c_char, b"Type D\0".as_ptr() as *const c_char, b"Unknown Type\0".as_ptr() as *const c_char, b"Unknown Type\0".as_ptr() as *const c_char];
pub static mut acpi_gbl_rng_decode: [*const c_char; 4] = [b"InvalidRanges\0".as_ptr() as *const c_char, b"NonISAOnlyRanges\0".as_ptr() as *const c_char, b"ISAOnlyRanges\0".as_ptr() as *const c_char, b"EntireRange\0".as_ptr() as *const c_char];
pub static mut acpi_gbl_rw_decode: [*const c_char; 2] = [b"ReadOnly\0".as_ptr() as *const c_char, b"ReadWrite\0".as_ptr() as *const c_char];
pub static mut acpi_gbl_shr_decode: [*const c_char; 4] = [b"Exclusive\0".as_ptr() as *const c_char, b"Shared\0".as_ptr() as *const c_char, b"ExclusiveAndWake\0".as_ptr() as *const c_char, b"SharedAndWake\0".as_ptr() as *const c_char];
pub static mut acpi_gbl_siz_decode: [*const c_char; 4] = [b"Transfer8\0".as_ptr() as *const c_char, b"Transfer8_16\0".as_ptr() as *const c_char, b"Transfer16\0".as_ptr() as *const c_char, b"InvalidSize\0".as_ptr() as *const c_char];
pub static mut acpi_gbl_trs_decode: [*const c_char; 2] = [b"DenseTranslation\0".as_ptr() as *const c_char, b"SparseTranslation\0".as_ptr() as *const c_char];
pub static mut acpi_gbl_ttp_decode: [*const c_char; 2] = [b"TypeStatic\0".as_ptr() as *const c_char, b"TypeTranslation\0".as_ptr() as *const c_char];
pub static mut acpi_gbl_typ_decode: [*const c_char; 4] = [b"Compatibility\0".as_ptr() as *const c_char, b"TypeA\0".as_ptr() as *const c_char, b"TypeB\0".as_ptr() as *const c_char, b"TypeF\0".as_ptr() as *const c_char];
pub static mut acpi_gbl_ppc_decode: [*const c_char; 4] = [b"PullDefault\0".as_ptr() as *const c_char, b"PullUp\0".as_ptr() as *const c_char, b"PullDown\0".as_ptr() as *const c_char, b"PullNone\0".as_ptr() as *const c_char];
pub static mut acpi_gbl_ior_decode: [*const c_char; 4] = [b"IoRestrictionNone\0".as_ptr() as *const c_char, b"IoRestrictionInputOnly\0".as_ptr() as *const c_char, b"IoRestrictionOutputOnly\0".as_ptr() as *const c_char, b"IoRestrictionNoneAndPreserve\0".as_ptr() as *const c_char];
pub static mut acpi_gbl_dts_decode: [*const c_char; 6] = [b"Width8bit\0".as_ptr() as *const c_char, b"Width16bit\0".as_ptr() as *const c_char, b"Width32bit\0".as_ptr() as *const c_char, b"Width64bit\0".as_ptr() as *const c_char, b"Width128bit\0".as_ptr() as *const c_char, b"Width256bit\0".as_ptr() as *const c_char];
pub static mut acpi_gbl_ct_decode: [*const c_char; 2] = [b"Interrupt\0".as_ptr() as *const c_char, b"I/O\0".as_ptr() as *const c_char];
pub static mut acpi_gbl_sbt_decode: [*const c_char; 5] = [b"/* UNKNOWN serial bus type */\0".as_ptr() as *const c_char, b"I2C\0".as_ptr() as *const c_char, b"SPI\0".as_ptr() as *const c_char, b"UART\0".as_ptr() as *const c_char, b"CSI2\0".as_ptr() as *const c_char];
pub static mut acpi_gbl_am_decode: [*const c_char; 2] = [b"AddressingMode7Bit\0".as_ptr() as *const c_char, b"AddressingMode10Bit\0".as_ptr() as *const c_char];
pub static mut acpi_gbl_sm_decode: [*const c_char; 2] = [b"ControllerInitiated\0".as_ptr() as *const c_char, b"DeviceInitiated\0".as_ptr() as *const c_char];
pub static mut acpi_gbl_wm_decode: [*const c_char; 2] = [b"FourWireMode\0".as_ptr() as *const c_char, b"ThreeWireMode\0".as_ptr() as *const c_char];
pub static mut acpi_gbl_cph_decode: [*const c_char; 2] = [b"ClockPhaseFirst\0".as_ptr() as *const c_char, b"ClockPhaseSecond\0".as_ptr() as *const c_char];
pub static mut acpi_gbl_cpo_decode: [*const c_char; 2] = [b"ClockPolarityLow\0".as_ptr() as *const c_char, b"ClockPolarityHigh\0".as_ptr() as *const c_char];
pub static mut acpi_gbl_dp_decode: [*const c_char; 2] = [b"PolarityLow\0".as_ptr() as *const c_char, b"PolarityHigh\0".as_ptr() as *const c_char];
pub static mut acpi_gbl_ed_decode: [*const c_char; 2] = [b"LittleEndian\0".as_ptr() as *const c_char, b"BigEndian\0".as_ptr() as *const c_char];
pub static mut acpi_gbl_bpb_decode: [*const c_char; 8] = [b"DataBitsFive\0".as_ptr() as *const c_char, b"DataBitsSix\0".as_ptr() as *const c_char, b"DataBitsSeven\0".as_ptr() as *const c_char, b"DataBitsEight\0".as_ptr() as *const c_char, b"DataBitsNine\0".as_ptr() as *const c_char, b"/* UNKNOWN Bits per byte */\0".as_ptr() as *const c_char, b"/* UNKNOWN Bits per byte */\0".as_ptr() as *const c_char, b"/* UNKNOWN Bits per byte */\0".as_ptr() as *const c_char];
pub static mut acpi_gbl_sb_decode: [*const c_char; 4] = [b"StopBitsZero\0".as_ptr() as *const c_char, b"StopBitsOne\0".as_ptr() as *const c_char, b"StopBitsOnePlusHalf\0".as_ptr() as *const c_char, b"StopBitsTwo\0".as_ptr() as *const c_char];
pub static mut acpi_gbl_fc_decode: [*const c_char; 4] = [b"FlowControlNone\0".as_ptr() as *const c_char, b"FlowControlHardware\0".as_ptr() as *const c_char, b"FlowControlXON\0".as_ptr() as *const c_char, b"/* UNKNOWN flow control keyword */\0".as_ptr() as *const c_char];
pub static mut acpi_gbl_pt_decode: [*const c_char; 8] = [b"ParityTypeNone\0".as_ptr() as *const c_char, b"ParityTypeEven\0".as_ptr() as *const c_char, b"ParityTypeOdd\0".as_ptr() as *const c_char, b"ParityTypeMark\0".as_ptr() as *const c_char, b"ParityTypeSpace\0".as_ptr() as *const c_char, b"/* UNKNOWN parity keyword */\0".as_ptr() as *const c_char, b"/* UNKNOWN parity keyword */\0".as_ptr() as *const c_char, b"/* UNKNOWN parity keyword */\0".as_ptr() as *const c_char];
pub static mut acpi_gbl_ptyp_decode: [*const c_char; 14] = [b"Default\0".as_ptr() as *const c_char, b"Bias Pull-up\0".as_ptr() as *const c_char, b"Bias Pull-down\0".as_ptr() as *const c_char, b"Bias Default\0".as_ptr() as *const c_char, b"Bias Disable\0".as_ptr() as *const c_char, b"Bias High Impedance\0".as_ptr() as *const c_char, b"Bias Bus Hold\0".as_ptr() as *const c_char, b"Drive Open Drain\0".as_ptr() as *const c_char, b"Drive Open Source\0".as_ptr() as *const c_char, b"Drive Push Pull\0".as_ptr() as *const c_char, b"Drive Strength\0".as_ptr() as *const c_char, b"Slew Rate\0".as_ptr() as *const c_char, b"Input Debounce\0".as_ptr() as *const c_char, b"Input Schmitt Trigger\0".as_ptr() as *const c_char];
pub static mut acpi_gbl_clock_input_mode: [*const c_char; 2] = [b"Fixed\0".as_ptr() as *const c_char, b"Variable\0".as_ptr() as *const c_char];
pub static mut acpi_gbl_clock_input_scale: [*const c_char; 3] = [b"Hz\0".as_ptr() as *const c_char, b"KHz\0".as_ptr() as *const c_char, b"MHz\0".as_ptr() as *const c_char];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
