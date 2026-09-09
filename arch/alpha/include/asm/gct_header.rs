/* SPDX-License-Identifier: GPL-2.0 */

pub type gct_id = u64;
pub type gct6_handle = u64;

#[repr(C)]
pub struct gct6_node {
    pub type_: u8,
    pub subtype: u8,
    pub size: u16,
    pub hd_extension: u32,
    pub owner: gct6_handle,
    pub active_user: gct6_handle,
    pub id: gct_id,
    pub flags: u64,
    pub rev: u16,
    pub change_counter: u16,
    pub max_child: u16,
    pub reserved1: u16,
    pub saved_owner: gct6_handle,
    pub affinity: gct6_handle,
    pub parent: gct6_handle,
    pub next: gct6_handle,
    pub prev: gct6_handle,
    pub child: gct6_handle,
    pub fw_flags: u64,
    pub os_usage: u64,
    pub fru_id: u64,
    pub checksum: u32,
    pub magic: u32, /* 'GLXY' */
}

#[repr(C)]
pub struct gct6_search_struct {
    pub type_: u8,
    pub subtype: u8,
    pub callout: Option<unsafe extern "C" fn(*mut gct6_node)>,
}

pub const GCT_NODE_MAGIC: u32 = 0x59584c47; /* 'GLXY' */

/* node types */
pub const GCT_TYPE_HOSE: u8 = 0x0E;

/* node subtypes */
pub const GCT_SUBTYPE_IO_PORT_MODULE: u8 = 0x2C;

/*
 * Original C macro:
 * ((gct6_node *)((char *)hwrpb + hwrpb->frut_offset + (gct6_handle)(off)))
 * The hwrpb type and its layout are supplied by the including environment.
 */
#[macro_export]
macro_rules! GCT_NODE_PTR {
    ($hwrpb:expr, $off:expr) => {
        (($hwrpb as *mut u8)
            .wrapping_add((*($hwrpb as *mut _)).frut_offset as usize)
            .wrapping_add(($off as gct6_handle) as usize)) as *mut gct6_node
    };
}

unsafe extern "C" {
    pub fn gct6_find_nodes(node: *mut gct6_node, search: *mut gct6_search_struct) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
