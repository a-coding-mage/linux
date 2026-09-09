/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of media-entity.h. */

pub const MEDIA_BITS_PER_TYPE: u32 = 8;
pub const MEDIA_BITS_PER_ID: u32 = 32 - MEDIA_BITS_PER_TYPE;
pub const MEDIA_ID_MASK: u64 = (1u64 << MEDIA_BITS_PER_ID) - 1;
pub const MEDIA_ENTITY_ENUM_MAX_DEPTH: usize = 16;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum media_gobj_type { MEDIA_GRAPH_ENTITY, MEDIA_GRAPH_PAD, MEDIA_GRAPH_LINK, MEDIA_GRAPH_INTF_DEVNODE }

#[repr(C)]
pub struct media_gobj { pub mdev: *mut media_device, pub id: u32, pub list: list_head }

#[repr(C)]
pub struct media_entity_enum { pub bmap: *mut ::std::ffi::c_ulong, pub idx_max: i32 }

#[repr(C)]
pub struct media_graph {
    pub stack: [media_graph_stack; MEDIA_ENTITY_ENUM_MAX_DEPTH],
    pub ent_enum: media_entity_enum,
    pub top: i32,
}
#[repr(C)]
pub struct media_graph_stack { pub entity: *mut media_entity, pub link: *mut list_head }

#[repr(C)]
pub struct media_pipeline { pub allocated: bool, pub mdev: *mut media_device, pub pads: list_head, pub start_count: i32 }
#[repr(C)]
pub struct media_pipeline_pad { pub list: list_head, pub pipe: *mut media_pipeline, pub pad: *mut media_pad }
#[repr(C)]
pub struct media_pipeline_pad_iter { pub cursor: *mut list_head }
#[repr(C)]
pub struct media_pipeline_entity_iter { pub ent_enum: media_entity_enum, pub cursor: *mut list_head }

#[repr(C)]
pub union media_link_gobj0 { pub gobj0: *mut media_gobj, pub source: *mut media_pad, pub intf: *mut media_interface }
#[repr(C)]
pub union media_link_gobj1 { pub gobj1: *mut media_gobj, pub sink: *mut media_pad, pub entity: *mut media_entity }
#[repr(C)]
pub struct media_link {
    pub graph_obj: media_gobj, pub list: list_head, pub first: media_link_gobj0,
    pub second: media_link_gobj1, pub reverse: *mut media_link, pub flags: ::std::ffi::c_ulong,
    pub is_backlink: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum media_pad_signal_type { PAD_SIGNAL_DEFAULT = 0, PAD_SIGNAL_ANALOG, PAD_SIGNAL_DV, PAD_SIGNAL_AUDIO }

#[repr(C)]
pub struct media_pad {
    pub graph_obj: media_gobj, pub entity: *mut media_entity, pub index: u16, pub num_links: u16,
    pub sig_type: media_pad_signal_type, pub flags: ::std::ffi::c_ulong, pub pipe: *mut media_pipeline,
}

#[repr(C)]
pub struct media_entity_operations {
    pub get_fwnode_pad: Option<unsafe extern "C" fn(*mut media_entity, *mut fwnode_endpoint) -> i32>,
    pub link_setup: Option<unsafe extern "C" fn(*mut media_entity, *const media_pad, *const media_pad, u32) -> i32>,
    pub link_validate: Option<unsafe extern "C" fn(*mut media_link) -> i32>,
    pub has_pad_interdep: Option<unsafe extern "C" fn(*mut media_entity, u32, u32) -> bool>,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum media_entity_type { MEDIA_ENTITY_TYPE_BASE, MEDIA_ENTITY_TYPE_VIDEO_DEVICE, MEDIA_ENTITY_TYPE_V4L2_SUBDEV }

#[repr(C)]
pub union media_entity_info { pub dev: media_entity_dev }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct media_entity_dev { pub major: u32, pub minor: u32 }
#[repr(C)]
pub struct media_entity {
    pub graph_obj: media_gobj, pub name: *const ::std::ffi::c_char, pub obj_type: media_entity_type,
    pub function: u32, pub flags: ::std::ffi::c_ulong, pub num_pads: u16, pub num_links: u16,
    pub num_backlinks: u16, pub internal_idx: i32, pub pads: *mut media_pad, pub links: list_head,
    pub ops: *const media_entity_operations, pub use_count: i32, pub info: media_entity_info,
}
#[repr(C)]
pub struct media_interface { pub graph_obj: media_gobj, pub links: list_head, pub type_: u32, pub flags: u32 }
#[repr(C)]
pub struct media_intf_devnode { pub intf: media_interface, pub major: u32, pub minor: u32 }

pub unsafe fn media_entity_id(entity: *mut media_entity) -> u32 { (*entity).graph_obj.id }
pub unsafe fn media_type(gobj: *mut media_gobj) -> media_gobj_type { ::std::mem::transmute((*gobj).id >> MEDIA_BITS_PER_ID) }
pub unsafe fn media_id(gobj: *mut media_gobj) -> u32 { (*gobj).id & MEDIA_ID_MASK as u32 }
pub fn media_gobj_gen_id(kind: media_gobj_type, local_id: u64) -> u32 { (kind as u32) << MEDIA_BITS_PER_ID | (local_id & MEDIA_ID_MASK) as u32 }
pub unsafe fn is_media_entity_v4l2_video_device(e: *mut media_entity) -> bool { !e.is_null() && (*e).obj_type == media_entity_type::MEDIA_ENTITY_TYPE_VIDEO_DEVICE }
pub unsafe fn is_media_entity_v4l2_subdev(e: *mut media_entity) -> bool { !e.is_null() && (*e).obj_type == media_entity_type::MEDIA_ENTITY_TYPE_V4L2_SUBDEV }

pub unsafe fn media_entity_enum_zero(e: *mut media_entity_enum) { bitmap_zero((*e).bmap, (*e).idx_max); }
pub unsafe fn media_entity_enum_set(e: *mut media_entity_enum, entity: *mut media_entity) { if (*entity).internal_idx < (*e).idx_max { __set_bit((*entity).internal_idx, (*e).bmap); } }
pub unsafe fn media_entity_enum_clear(e: *mut media_entity_enum, entity: *mut media_entity) { if (*entity).internal_idx < (*e).idx_max { __clear_bit((*entity).internal_idx, (*e).bmap); } }
pub unsafe fn media_entity_enum_test(e: *mut media_entity_enum, entity: *mut media_entity) -> bool { if (*entity).internal_idx >= (*e).idx_max { true } else { test_bit((*entity).internal_idx, (*e).bmap) } }
pub unsafe fn media_entity_enum_test_and_set(e: *mut media_entity_enum, entity: *mut media_entity) -> bool { if (*entity).internal_idx >= (*e).idx_max { true } else { __test_and_set_bit((*entity).internal_idx, (*e).bmap) } }
pub unsafe fn media_entity_enum_empty(e: *mut media_entity_enum) -> bool { bitmap_empty((*e).bmap, (*e).idx_max) }
pub unsafe fn media_entity_enum_intersects(a: *mut media_entity_enum, b: *mut media_entity_enum) -> bool { bitmap_intersects((*a).bmap, (*b).bmap, ::std::cmp::min((*a).idx_max, (*b).idx_max)) }

pub unsafe fn media_entity_cleanup(_entity: *mut media_entity) {}
pub unsafe fn media_entity_remote_source_pad_unique(entity: *const media_entity) -> *mut media_pad { media_entity_remote_pad_unique(entity, MEDIA_PAD_FL_SOURCE) }
pub unsafe fn media_pad_is_streaming(pad: *const media_pad) -> bool { !(*pad).pipe.is_null() }
pub unsafe fn media_entity_is_streaming(entity: *const media_entity) -> bool { for i in 0..(*entity).num_pads { if media_pad_is_streaming((*entity).pads.add(i as usize)) { return true; } } false }

extern "C" {
    pub fn media_entity_enum_init(*mut media_entity_enum, *mut media_device) -> i32;
    pub fn media_entity_enum_cleanup(*mut media_entity_enum);
    pub fn media_gobj_create(*mut media_device, media_gobj_type, *mut media_gobj);
    pub fn media_gobj_destroy(*mut media_gobj);
    pub fn media_entity_pads_init(*mut media_entity, u16, *mut media_pad) -> i32;
    pub fn media_get_pad_index(*mut media_entity, u32, media_pad_signal_type) -> i32;
    pub fn media_create_pad_link(*mut media_entity, u16, *mut media_entity, u16, u32) -> i32;
    pub fn media_create_pad_links(*const media_device, u32, *mut media_entity, u16, u32, *mut media_entity, u16, u32, bool) -> i32;
    pub fn media_entity_remove_links(*mut media_entity);
    pub fn __media_entity_remove_links(*mut media_entity);
    pub fn __media_entity_setup_link(*mut media_link, u32) -> i32;
    pub fn media_entity_setup_link(*mut media_link, u32) -> i32;
    pub fn media_entity_find_link(*mut media_pad, *mut media_pad) -> *mut media_link;
    pub fn media_pad_remote_pad_first(*const media_pad) -> *mut media_pad;
    pub fn media_pad_remote_pad_unique(*const media_pad) -> *mut media_pad;
    pub fn media_entity_remote_pad_unique(*const media_entity, u32) -> *mut media_pad;
    pub fn media_entity_pipeline(*mut media_entity) -> *mut media_pipeline;
    pub fn media_pad_pipeline(*mut media_pad) -> *mut media_pipeline;
    pub fn media_entity_get_fwnode_pad(*mut media_entity, *const fwnode_handle, ::std::ffi::c_ulong) -> i32;
    pub fn media_graph_walk_init(*mut media_graph, *mut media_device) -> i32;
    pub fn media_graph_walk_cleanup(*mut media_graph);
    pub fn media_graph_walk_start(*mut media_graph, *mut media_entity);
    pub fn media_graph_walk_next(*mut media_graph) -> *mut media_entity;
    pub fn media_pipeline_start(*mut media_pad, *mut media_pipeline) -> i32;
    pub fn __media_pipeline_start(*mut media_pad, *mut media_pipeline) -> i32;
    pub fn media_pipeline_stop(*mut media_pad);
    pub fn __media_pipeline_stop(*mut media_pad);
    pub fn __media_pipeline_pad_iter_next(*mut media_pipeline, *mut media_pipeline_pad_iter, *mut media_pad) -> *mut media_pad;
    pub fn media_pipeline_entity_iter_init(*mut media_pipeline, *mut media_pipeline_entity_iter) -> i32;
    pub fn media_pipeline_entity_iter_cleanup(*mut media_pipeline_entity_iter);
    pub fn __media_pipeline_entity_iter_next(*mut media_pipeline, *mut media_pipeline_entity_iter, *mut media_entity) -> *mut media_entity;
    pub fn media_pipeline_alloc_start(*mut media_pad) -> i32;
    pub fn media_devnode_create(*mut media_device, u32, u32, u32, u32) -> *mut media_intf_devnode;
    pub fn media_devnode_remove(*mut media_intf_devnode);
    pub fn media_create_intf_link(*mut media_entity, *mut media_interface, u32) -> *mut media_link;
    pub fn __media_remove_intf_link(*mut media_link);
    pub fn media_remove_intf_link(*mut media_link);
    pub fn __media_remove_intf_links(*mut media_interface);
    pub fn media_remove_intf_links(*mut media_interface);
    pub fn media_create_ancillary_link(*mut media_entity, *mut media_entity) -> *mut media_link;
    pub fn __media_entity_next_link(*mut media_entity, *mut media_link, ::std::ffi::c_ulong) -> *mut media_link;
}

/* External types and Linux bitmap/list helpers are supplied by other translated headers. */
extern "C" {
    type media_device; type list_head; type fwnode_endpoint; type fwnode_handle;
    fn bitmap_zero(*mut ::std::ffi::c_ulong, i32); fn __set_bit(i32, *mut ::std::ffi::c_ulong);
    fn __clear_bit(i32, *mut ::std::ffi::c_ulong); fn test_bit(i32, *mut ::std::ffi::c_ulong) -> bool;
    fn __test_and_set_bit(i32, *mut ::std::ffi::c_ulong) -> bool; fn bitmap_empty(*mut ::std::ffi::c_ulong, i32) -> bool;
    fn bitmap_intersects(*mut ::std::ffi::c_ulong, *mut ::std::ffi::c_ulong, i32) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
