/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/* Do not edit directly, auto-generated from: */
/*	Documentation/netlink/specs/drm_ras.yaml */
/* YNL-GEN uapi header */
/* To regenerate run: tools/net/ynl/ynl-regen.sh */

pub const DRM_RAS_FAMILY_NAME: &str = "drm-ras";
pub const DRM_RAS_FAMILY_VERSION: i32 = 1;

/*
 * Type of the node. Currently, only error-counter nodes are supported, which
 * expose reliability counters for a hardware/software component.
 */
#[repr(i32)]
pub enum drm_ras_node_type {
    DRM_RAS_NODE_TYPE_ERROR_COUNTER = 1,
}

pub const DRM_RAS_A_NODE_ATTRS_NODE_ID: i32 = 1;
pub const DRM_RAS_A_NODE_ATTRS_DEVICE_NAME: i32 =
    DRM_RAS_A_NODE_ATTRS_NODE_ID + 1;
pub const DRM_RAS_A_NODE_ATTRS_NODE_NAME: i32 =
    DRM_RAS_A_NODE_ATTRS_DEVICE_NAME + 1;
pub const DRM_RAS_A_NODE_ATTRS_NODE_TYPE: i32 =
    DRM_RAS_A_NODE_ATTRS_NODE_NAME + 1;

pub const __DRM_RAS_A_NODE_ATTRS_MAX: i32 = DRM_RAS_A_NODE_ATTRS_NODE_TYPE + 1;
pub const DRM_RAS_A_NODE_ATTRS_MAX: i32 = __DRM_RAS_A_NODE_ATTRS_MAX - 1;

pub const DRM_RAS_A_ERROR_COUNTER_ATTRS_NODE_ID: i32 = 1;
pub const DRM_RAS_A_ERROR_COUNTER_ATTRS_ERROR_ID: i32 =
    DRM_RAS_A_ERROR_COUNTER_ATTRS_NODE_ID + 1;
pub const DRM_RAS_A_ERROR_COUNTER_ATTRS_ERROR_NAME: i32 =
    DRM_RAS_A_ERROR_COUNTER_ATTRS_ERROR_ID + 1;
pub const DRM_RAS_A_ERROR_COUNTER_ATTRS_ERROR_VALUE: i32 =
    DRM_RAS_A_ERROR_COUNTER_ATTRS_ERROR_NAME + 1;

pub const __DRM_RAS_A_ERROR_COUNTER_ATTRS_MAX: i32 =
    DRM_RAS_A_ERROR_COUNTER_ATTRS_ERROR_VALUE + 1;
pub const DRM_RAS_A_ERROR_COUNTER_ATTRS_MAX: i32 =
    __DRM_RAS_A_ERROR_COUNTER_ATTRS_MAX - 1;

pub const DRM_RAS_CMD_LIST_NODES: i32 = 1;
pub const DRM_RAS_CMD_GET_ERROR_COUNTER: i32 = DRM_RAS_CMD_LIST_NODES + 1;
pub const DRM_RAS_CMD_CLEAR_ERROR_COUNTER: i32 = DRM_RAS_CMD_GET_ERROR_COUNTER + 1;

pub const __DRM_RAS_CMD_MAX: i32 = DRM_RAS_CMD_CLEAR_ERROR_COUNTER + 1;
pub const DRM_RAS_CMD_MAX: i32 = __DRM_RAS_CMD_MAX - 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
