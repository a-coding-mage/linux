// Copyright 2012 Advanced Micro Devices, Inc.
// Translated from amd_acpi.h. Structures intentionally use packed C layout.

#[repr(C, packed)]
pub struct atif_verify_interface { pub size: u16, pub version: u16, pub notification_mask: u32, pub function_bits: u32 }
#[repr(C, packed)]
pub struct atif_system_params { pub size: u16, pub valid_mask: u32, pub flags: u32, pub command_code: u8 }
#[repr(C, packed)]
pub struct atif_sbios_requests { pub size: u16, pub pending: u32, pub panel_exp_mode: u8, pub thermal_gfx: u8, pub thermal_state: u8, pub forced_power_gfx: u8, pub forced_power_state: u8, pub system_power_src: u8, pub backlight_level: u8 }
#[repr(C, packed)]
pub struct atif_qbtc_arguments { pub size: u16, pub requested_display: u8 }
pub const ATIF_QBTC_MAX_DATA_POINTS: usize = 99;
#[repr(C, packed)]
pub struct atif_qbtc_data_point { pub luminance: u8, pub input_signal: u8 }
#[repr(C, packed)]
pub struct atif_qbtc_output { pub size: u16, pub flags: u16, pub error_code: u8, pub ac_level: u8, pub dc_level: u8, pub min_input_signal: u8, pub max_input_signal: u8, pub number_of_points: u8, pub data_points: [atif_qbtc_data_point; ATIF_QBTC_MAX_DATA_POINTS] }

pub const ATIF_NOTIFY_MASK: u32 = 0x3; pub const ATIF_NOTIFY_NONE: u32 = 0; pub const ATIF_NOTIFY_81: u32 = 1; pub const ATIF_NOTIFY_N: u32 = 2;
#[repr(C, packed)]
pub struct atcs_verify_interface { pub size: u16, pub version: u16, pub function_bits: u32 }
pub const ATCS_VALID_FLAGS_MASK: u32 = 0x3;
#[repr(C, packed)]
pub struct atcs_pref_req_input { pub size: u16, pub client_id: u16, pub valid_flags_mask: u16, pub flags: u16, pub req_type: u8, pub perf_req: u8 }
#[repr(C, packed)] pub struct atcs_pref_req_output { pub size: u16, pub ret_val: u8 }
#[repr(C, packed)] pub struct atcs_pwr_shift_input { pub size: u16, pub dgpu_id: u16, pub dev_acpi_state: u8, pub drv_state: u8 }
#[repr(C, packed)] pub struct atcs_get_uma_size_output { pub size: u16, pub uma_size_mb: u32 }
#[repr(C, packed)] pub struct atcs_set_uma_allocation_size_input { pub size: u16, pub uma_size_index: u8, pub uma_size_type: u8 }

pub const ATIF_FUNCTION_VERIFY_INTERFACE: u32 = 0x0;
pub const ATIF_THERMAL_STATE_CHANGE_REQUEST_SUPPORTED: u32 = 1 << 2;
pub const ATIF_FORCED_POWER_STATE_CHANGE_REQUEST_SUPPORTED: u32 = 1 << 3;
pub const ATIF_SYSTEM_POWER_SOURCE_CHANGE_REQUEST_SUPPORTED: u32 = 1 << 4;
pub const ATIF_PANEL_BRIGHTNESS_CHANGE_REQUEST_SUPPORTED: u32 = 1 << 7;
pub const ATIF_DGPU_DISPLAY_EVENT_SUPPORTED: u32 = 1 << 8;
pub const ATIF_GPU_PACKAGE_POWER_LIMIT_REQUEST_SUPPORTED: u32 = 1 << 12;
pub const ATIF_GET_SYSTEM_PARAMETERS_SUPPORTED: u32 = 1 << 0;
pub const ATIF_GET_SYSTEM_BIOS_REQUESTS_SUPPORTED: u32 = 1 << 1;
pub const ATIF_TEMPERATURE_CHANGE_NOTIFICATION_SUPPORTED: u32 = 1 << 12;
pub const ATIF_QUERY_BACKLIGHT_TRANSFER_CHARACTERISTICS_SUPPORTED: u32 = 1 << 15;
pub const ATIF_READY_TO_UNDOCK_NOTIFICATION_SUPPORTED: u32 = 1 << 16;
pub const ATIF_GET_EXTERNAL_GPU_INFORMATION_SUPPORTED: u32 = 1 << 20;
pub const ATIF_FUNCTION_GET_SYSTEM_PARAMETERS: u32 = 0x1;
pub const ATIF_FUNCTION_GET_SYSTEM_BIOS_REQUESTS: u32 = 0x2;
pub const ATIF_THERMAL_STATE_CHANGE_REQUEST: u32 = 1 << 2;
pub const ATIF_FORCED_POWER_STATE_CHANGE_REQUEST: u32 = 1 << 3;
pub const ATIF_SYSTEM_POWER_SOURCE_CHANGE_REQUEST: u32 = 1 << 4;
pub const ATIF_PANEL_BRIGHTNESS_CHANGE_REQUEST: u32 = 1 << 7;
pub const ATIF_DGPU_DISPLAY_EVENT: u32 = 1 << 8;
pub const ATIF_GPU_PACKAGE_POWER_LIMIT_REQUEST: u32 = 1 << 12;
pub const ATIF_TARGET_GFX_SINGLE: u32 = 0; pub const ATIF_TARGET_GFX_PX_IGPU: u32 = 1; pub const ATIF_TARGET_GFX_PX_DGPU: u32 = 2;
pub const ATIF_POWER_SOURCE_AC: u32 = 1; pub const ATIF_POWER_SOURCE_DC: u32 = 2; pub const ATIF_POWER_SOURCE_RESTRICTED_AC_1: u32 = 3; pub const ATIF_POWER_SOURCE_RESTRICTED_AC_2: u32 = 4;
pub const ATIF_FUNCTION_TEMPERATURE_CHANGE_NOTIFICATION: u32 = 0xD;
pub const ATIF_FUNCTION_QUERY_BRIGHTNESS_TRANSFER_CHARACTERISTICS: u32 = 0x10;
pub const ATIF_QBTC_REQUEST_LCD1: u32 = 0; pub const ATIF_QBTC_REQUEST_CRT1: u32 = 1; pub const ATIF_QBTC_REQUEST_DFP1: u32 = 3; pub const ATIF_QBTC_REQUEST_CRT2: u32 = 4; pub const ATIF_QBTC_REQUEST_LCD2: u32 = 5; pub const ATIF_QBTC_REQUEST_DFP2: u32 = 7; pub const ATIF_QBTC_REQUEST_DFP3: u32 = 9; pub const ATIF_QBTC_REQUEST_DFP4: u32 = 10; pub const ATIF_QBTC_REQUEST_DFP5: u32 = 11; pub const ATIF_QBTC_REQUEST_DFP6: u32 = 12;
pub const ATIF_QBTC_ERROR_CODE_SUCCESS: u32 = 0; pub const ATIF_QBTC_ERROR_CODE_FAILURE: u32 = 1; pub const ATIF_QBTC_ERROR_CODE_DEVICE_NOT_SUPPORTED: u32 = 2;
pub const ATIF_FUNCTION_READY_TO_UNDOCK_NOTIFICATION: u32 = 0x11; pub const ATIF_FUNCTION_GET_EXTERNAL_GPU_INFORMATION: u32 = 0x15; pub const ATIF_EXTERNAL_GRAPHICS_PORT: u32 = 1 << 0;

pub const ATPX_FUNCTION_VERIFY_INTERFACE: u32 = 0x0;
pub const ATPX_GET_PX_PARAMETERS_SUPPORTED: u32 = 1 << 0; pub const ATPX_POWER_CONTROL_SUPPORTED: u32 = 1 << 1; pub const ATPX_DISPLAY_MUX_CONTROL_SUPPORTED: u32 = 1 << 2; pub const ATPX_I2C_MUX_CONTROL_SUPPORTED: u32 = 1 << 3; pub const ATPX_GRAPHICS_DEVICE_SWITCH_START_NOTIFICATION_SUPPORTED: u32 = 1 << 4; pub const ATPX_GRAPHICS_DEVICE_SWITCH_END_NOTIFICATION_SUPPORTED: u32 = 1 << 5; pub const ATPX_GET_DISPLAY_CONNECTORS_MAPPING_SUPPORTED: u32 = 1 << 7; pub const ATPX_GET_DISPLAY_DETECTION_PORTS_SUPPORTED: u32 = 1 << 8;
pub const ATPX_FUNCTION_GET_PX_PARAMETERS: u32 = 1; pub const ATPX_LVDS_I2C_AVAILABLE_TO_BOTH_GPUS: u32 = 1 << 0; pub const ATPX_CRT1_I2C_AVAILABLE_TO_BOTH_GPUS: u32 = 1 << 1; pub const ATPX_DVI1_I2C_AVAILABLE_TO_BOTH_GPUS: u32 = 1 << 2; pub const ATPX_CRT1_RGB_SIGNAL_MUXED: u32 = 1 << 3; pub const ATPX_TV_SIGNAL_MUXED: u32 = 1 << 4; pub const ATPX_DFP_SIGNAL_MUXED: u32 = 1 << 5; pub const ATPX_SEPARATE_MUX_FOR_I2C: u32 = 1 << 6; pub const ATPX_DYNAMIC_PX_SUPPORTED: u32 = 1 << 7; pub const ATPX_ACF_NOT_SUPPORTED: u32 = 1 << 8; pub const ATPX_FIXED_NOT_SUPPORTED: u32 = 1 << 9; pub const ATPX_DYNAMIC_DGPU_POWER_OFF_SUPPORTED: u32 = 1 << 10; pub const ATPX_DGPU_REQ_POWER_FOR_DISPLAYS: u32 = 1 << 11; pub const ATPX_DGPU_CAN_DRIVE_DISPLAYS: u32 = 1 << 12; pub const ATPX_MS_HYBRID_GFX_SUPPORTED: u32 = 1 << 14;
pub const ATPX_FUNCTION_POWER_CONTROL: u32 = 2; pub const ATPX_FUNCTION_DISPLAY_MUX_CONTROL: u32 = 3; pub const ATPX_INTEGRATED_GPU: u32 = 0; pub const ATPX_DISCRETE_GPU: u32 = 1; pub const ATPX_FUNCTION_I2C_MUX_CONTROL: u32 = 4; pub const ATPX_FUNCTION_GRAPHICS_DEVICE_SWITCH_START_NOTIFICATION: u32 = 5; pub const ATPX_FUNCTION_GRAPHICS_DEVICE_SWITCH_END_NOTIFICATION: u32 = 6; pub const ATPX_FUNCTION_GET_DISPLAY_CONNECTORS_MAPPING: u32 = 8; pub const ATPX_DISPLAY_OUTPUT_SUPPORTED_BY_ADAPTER_ID_DEVICE: u32 = 1; pub const ATPX_DISPLAY_HPD_SUPPORTED_BY_ADAPTER_ID_DEVICE: u32 = 1 << 1; pub const ATPX_DISPLAY_I2C_SUPPORTED_BY_ADAPTER_ID_DEVICE: u32 = 1 << 2; pub const ATPX_FUNCTION_GET_DISPLAY_DETECTION_PORTS: u32 = 9;
pub const ATPX_HPD_NONE: u32 = 0; pub const ATPX_HPD1: u32 = 1; pub const ATPX_HPD2: u32 = 2; pub const ATPX_HPD3: u32 = 3; pub const ATPX_HPD4: u32 = 4; pub const ATPX_HPD5: u32 = 5; pub const ATPX_HPD6: u32 = 6; pub const ATPX_DDC_NONE: u32 = 0; pub const ATPX_DDC1: u32 = 1; pub const ATPX_DDC2: u32 = 2; pub const ATPX_DDC3: u32 = 3; pub const ATPX_DDC4: u32 = 4; pub const ATPX_DDC5: u32 = 5; pub const ATPX_DDC6: u32 = 6; pub const ATPX_DDC7: u32 = 7; pub const ATPX_DDC8: u32 = 8;

pub const ATCS_FUNCTION_VERIFY_INTERFACE: u32 = 0; pub const ATCS_GET_EXTERNAL_STATE_SUPPORTED: u32 = 1 << 0; pub const ATCS_PCIE_PERFORMANCE_REQUEST_SUPPORTED: u32 = 1 << 1; pub const ATCS_PCIE_DEVICE_READY_NOTIFICATION_SUPPORTED: u32 = 1 << 2; pub const ATCS_SET_PCIE_BUS_WIDTH_SUPPORTED: u32 = 1 << 3; pub const ACPI_ATCS_GET_UMA_SIZE_SUPPORTED: u32 = 1 << 5; pub const ATCS_SET_POWER_SHIFT_CONTROL_SUPPORTED: u32 = 1 << 7; pub const ACPI_ATCS_SET_UMA_ALLOCATION_SIZE_SUPPORTED: u32 = 1 << 9;
pub const ATCS_FUNCTION_GET_EXTERNAL_STATE: u32 = 1; pub const ATCS_DOCKED: u32 = 1; pub const ATCS_FUNCTION_PCIE_PERFORMANCE_REQUEST: u32 = 2; pub const ATCS_ADVERTISE_CAPS: u32 = 1; pub const ATCS_WAIT_FOR_COMPLETION: u32 = 1 << 1; pub const ATCS_PCIE_LINK_SPEED: u32 = 1; pub const ATCS_REMOVE: u32 = 0; pub const ATCS_FORCE_LOW_POWER: u32 = 1; pub const ATCS_PERF_LEVEL_1: u32 = 2; pub const ATCS_PERF_LEVEL_2: u32 = 3; pub const ATCS_PERF_LEVEL_3: u32 = 4; pub const ATCS_REQUEST_REFUSED: u32 = 1; pub const ATCS_REQUEST_COMPLETE: u32 = 2; pub const ATCS_REQUEST_IN_PROGRESS: u32 = 3;
pub const ATCS_FUNCTION_PCIE_DEVICE_READY_NOTIFICATION: u32 = 3; pub const ATCS_FUNCTION_SET_PCIE_BUS_WIDTH: u32 = 4; pub const ATCS_FUNCTION_POWER_SHIFT_CONTROL: u32 = 8; pub const ATCS_FUNCTION_GET_UMA_SIZE: u32 = 6; pub const ATCS_FUNCTION_SET_UMA_ALLOCATION_SIZE: u32 = 0xA;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
