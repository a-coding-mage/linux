/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/******************************************************************************
 *
 * Name: acuuid.h - ACPI-related UUID/GUID definitions
 *
 * Copyright (C) 2000 - 2026, Intel Corp.
 *
 *****************************************************************************/

/*
 * Note1: UUIDs and GUIDs are defined to be identical in ACPI.
 *
 * Note2: This file is standalone and should remain that way.
 */

/* Controllers */

pub const UUID_GPIO_CONTROLLER: &str = "4f248f40-d5e2-499f-834c-27758ea1cd3f";
pub const UUID_USB_CONTROLLER: &str = "ce2ee385-00e6-48cb-9f05-2edb927c4899";
pub const UUID_SATA_CONTROLLER: &str = "e4db149b-fcfe-425b-a6d8-92357d78fc7f";

/* Devices */

pub const UUID_PCI_HOST_BRIDGE: &str = "33db4d5b-1ff7-401c-9657-7441c03dd766";
pub const UUID_I2C_DEVICE: &str = "3cdff6f7-4267-4555-ad05-b30a3d8938de";
pub const UUID_POWER_BUTTON: &str = "dfbcf3c5-e7a5-44e6-9c1f-29c76f6e059c";
pub const UUID_MEMORY_DEVICE: &str = "03b19910-f473-11dd-87af-0800200c9a66";
pub const UUID_GENERIC_BUTTONS_DEVICE: &str = "fa6bd625-9ce8-470d-a2c7-b3ca36c4282e";
pub const UUID_NVDIMM_ROOT_DEVICE: &str = "2f10e7a4-9e91-11e4-89d3-123b93f75cba";
pub const UUID_CONTROL_METHOD_BATTERY: &str = "f18fc78b-0f15-4978-b793-53f833a1d35b";

/* Interfaces */

pub const UUID_DEVICE_LABELING: &str = "e5c937d0-3553-4d7a-9117-ea4d19c3434d";
pub const UUID_PHYSICAL_PRESENCE: &str = "3dddfaa6-361b-4eb4-a424-8d10089d1653";

/* TPM */
pub const UUID_HARDWARE_INFORMATION: &str = "cf8e16a5-c1e8-4e25-b712-4f54a96702c8";
pub const UUID_START_METHOD: &str = "6bbf6cab-5463-4714-b7cd-f0203c0368d4";
pub const UUID_MEMORY_CLEAR: &str = "376054ed-cc13-4675-901c-4756d7f2d45d";

/* NVDIMM - NFIT table */

pub const UUID_NFIT_DIMM: &str = "4309ac30-0d11-11e4-9191-0800200c9a66";
pub const UUID_VOLATILE_MEMORY: &str = "7305944f-fdda-44e3-b16c-3f22d252e5d0";
pub const UUID_PERSISTENT_MEMORY: &str = "66f0d379-b4f3-4074-ac43-0d3318b78cdb";
pub const UUID_CONTROL_REGION: &str = "92f701f6-13b4-405d-910b-299367e8234c";
pub const UUID_DATA_REGION: &str = "91af0530-5d86-470e-a6b0-0a2db9408249";
pub const UUID_VOLATILE_VIRTUAL_DISK: &str = "77ab535a-45fc-624b-5560-f7b281d1f96e";
pub const UUID_VOLATILE_VIRTUAL_CD: &str = "3d5abd30-4175-87ce-6d64-d2ade523c4bb";
pub const UUID_PERSISTENT_VIRTUAL_DISK: &str = "5cea02c9-4d07-69d3-269f-4496fbe096f9";
pub const UUID_PERSISTENT_VIRTUAL_CD: &str = "08018188-42cd-bb48-100f-5387d53ded3d";
pub const UUID_NFIT_DIMM_N_MSFT: &str = "1ee68b36-d4bd-4a1a-9a16-4f8e53d46e05";
pub const UUID_NFIT_DIMM_N_HPE1: &str = "9002c334-acf3-4c0e-9642-a235f0d53bc6";
pub const UUID_NFIT_DIMM_N_HPE2: &str = "5008664b-b758-41a0-a03c-27c2f2d04f7e";
pub const UUID_NFIT_DIMM_N_HYPERV: &str = "5746c5f2-a9a2-4264-ad0e-e4ddc9e09e80";

/* Processor Properties (ACPI 6.2) */

pub const UUID_CACHE_PROPERTIES: &str = "6DC63E77-257E-4E78-A973-A21F2796898D";
pub const UUID_PHYSICAL_PROPERTY: &str = "DDE4D59A-AA42-4349-B407-EA40F57D9FB7";

/* Modern Standby */
pub const UUID_LPS0_MICROSOFT: &str = "11E00D56-CE64-47CE-837B-1F898F9AA461";
pub const UUID_LPS0_INTEL: &str = "C4EB40A0-6CD2-11E2-BCFD-0800200C9A66";
pub const UUID_LPS0_AMD: &str = "E3F32452-FEBC-43CE-9039-932122D37721";

/* Miscellaneous */

pub const UUID_PLATFORM_CAPABILITIES: &str = "0811b06e-4a27-44f9-8d60-3cbbc22e7b48";
pub const UUID_DYNAMIC_ENUMERATION: &str = "d8c1a3a6-be9b-4c9b-91bf-c3cb81fc5daf";
pub const UUID_BATTERY_THERMAL_LIMIT: &str = "4c2067e3-887d-475c-9720-4af1d3ed602e";
pub const UUID_THERMAL_EXTENSIONS: &str = "14d399cd-7a27-4b18-8fb4-7cb7b9f4e500";
pub const UUID_DEVICE_PROPERTIES: &str = "daffd814-6eba-4d8c-8a91-bc9bbf4aa301";
pub const UUID_DEVICE_GRAPHS: &str = "ab02a46b-74c7-45a2-bd68-f7d344ef2153";
pub const UUID_HIERARCHICAL_DATA_EXTENSION: &str = "dbb8e3e6-5886-4ba6-8795-1319f52a966b";
pub const UUID_CORESIGHT_GRAPH: &str = "3ecbc8b6-1d0e-4fb3-8107-e627f805c6cd";
pub const UUID_USB4_CAPABILITIES: &str = "23a0d13a-26ab-486c-9c5f-0ffa525a575a";
pub const UUID_1ST_FUNCTION_ID: &str = "893f00a6-660c-494e-bcfd-3043f4fb67c0";
pub const UUID_2ND_FUNCTION_ID: &str = "107ededd-d381-4fd7-8da9-08e9a6c79644";
pub const UUID_FAN_TRIP_POINTS: &str = "a7611840-99fe-41ae-a488-35c75926c8eb";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
