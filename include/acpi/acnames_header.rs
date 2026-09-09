/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/******************************************************************************
 *
 * Name: acnames.h - Global names and strings
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

/* Method names - these methods can appear anywhere in the namespace */

pub const METHOD_NAME__ADR: &str = "_ADR";
pub const METHOD_NAME__AEI: &str = "_AEI";
pub const METHOD_NAME__BBN: &str = "_BBN";
pub const METHOD_NAME__CBA: &str = "_CBA";
pub const METHOD_NAME__CID: &str = "_CID";
pub const METHOD_NAME__CLS: &str = "_CLS";
pub const METHOD_NAME__CRS: &str = "_CRS";
pub const METHOD_NAME__DDN: &str = "_DDN";
pub const METHOD_NAME__DIS: &str = "_DIS";
pub const METHOD_NAME__DMA: &str = "_DMA";
pub const METHOD_NAME__EVT: &str = "_EVT";
pub const METHOD_NAME__HID: &str = "_HID";
pub const METHOD_NAME__INI: &str = "_INI";
pub const METHOD_NAME__PLD: &str = "_PLD";
pub const METHOD_NAME__DSD: &str = "_DSD";
pub const METHOD_NAME__PRS: &str = "_PRS";
pub const METHOD_NAME__PRT: &str = "_PRT";
pub const METHOD_NAME__PRW: &str = "_PRW";
pub const METHOD_NAME__PS0: &str = "_PS0";
pub const METHOD_NAME__PS1: &str = "_PS1";
pub const METHOD_NAME__PS2: &str = "_PS2";
pub const METHOD_NAME__PS3: &str = "_PS3";
pub const METHOD_NAME__REG: &str = "_REG";
pub const METHOD_NAME__SB_: &str = "_SB_";
pub const METHOD_NAME__SEG: &str = "_SEG";
pub const METHOD_NAME__SRS: &str = "_SRS";
pub const METHOD_NAME__STA: &str = "_STA";
pub const METHOD_NAME__SUB: &str = "_SUB";
pub const METHOD_NAME__UID: &str = "_UID";

/* Method names - these methods must appear at the namespace root */

pub const METHOD_PATHNAME__PTS: &str = "\\_PTS";
pub const METHOD_PATHNAME__SST: &str = "\\_SI._SST";
pub const METHOD_PATHNAME__WAK: &str = "\\_WAK";

/* Definitions of the predefined namespace names  */

pub const ACPI_UNKNOWN_NAME: u32 = 0x3F3F3F3F; /* Unknown name is "????" */
pub const ACPI_PREFIX_MIXED: u32 = 0x69706341; /* "Acpi" */
pub const ACPI_PREFIX_LOWER: u32 = 0x69706361; /* "acpi" */

/* Root name stuff */

pub const ACPI_ROOT_NAME: u32 = 0x5F5F5F5C; /* Root name is    "\\___" */
pub const ACPI_ROOT_PATHNAME: &str = "\\___";
pub const ACPI_NAMESPACE_ROOT: &str = "Namespace Root";
pub const ACPI_NS_ROOT_PATH: &str = "\\";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
