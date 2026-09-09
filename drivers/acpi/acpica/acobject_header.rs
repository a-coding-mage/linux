/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/* Translation of acobject.h. External ACPICA types are supplied by other units. */

/* C uses pragma pack(8) on 64-bit targets and pack(4) otherwise. */

#[repr(C)]
pub struct AcpiObjectCommon {
    pub next_object: *mut AcpiOperandObject,
    pub descriptor_type: u8,
    pub type_: u8,
    pub reference_count: u16,
    pub flags: u8,
}

pub const AOPOBJ_AML_CONSTANT: u8 = 0x01;
pub const AOPOBJ_STATIC_POINTER: u8 = 0x02;
pub const AOPOBJ_DATA_VALID: u8 = 0x04;
pub const AOPOBJ_OBJECT_INITIALIZED: u8 = 0x08;
pub const AOPOBJ_REG_CONNECTED: u8 = 0x10;
pub const AOPOBJ_SETUP_COMPLETE: u8 = 0x20;
pub const AOPOBJ_INVALID: u8 = 0x40;

#[repr(C)]
pub struct AcpiObjectInteger {
    pub next_object: *mut AcpiOperandObject, pub descriptor_type: u8, pub type_: u8,
    pub reference_count: u16, pub flags: u8, pub fill: [u8; 3], pub value: u64,
}

#[repr(C)]
pub struct AcpiObjectString {
    pub next_object: *mut AcpiOperandObject, pub descriptor_type: u8, pub type_: u8,
    pub reference_count: u16, pub flags: u8, pub pointer: *mut i8, pub length: u32,
}

#[repr(C)]
pub struct AcpiObjectBuffer {
    pub next_object: *mut AcpiOperandObject, pub descriptor_type: u8, pub type_: u8,
    pub reference_count: u16, pub flags: u8, pub pointer: *mut u8, pub length: u32,
    pub aml_length: u32, pub aml_start: *mut u8, pub node: *mut AcpiNamespaceNode,
}

#[repr(C)]
pub struct AcpiObjectPackage {
    pub next_object: *mut AcpiOperandObject, pub descriptor_type: u8, pub type_: u8,
    pub reference_count: u16, pub flags: u8, pub node: *mut AcpiNamespaceNode,
    pub elements: *mut *mut AcpiOperandObject, pub aml_start: *mut u8,
    pub aml_length: u32, pub count: u32,
}

#[repr(C)] pub struct AcpiObjectEvent { pub next_object:*mut AcpiOperandObject, pub descriptor_type:u8, pub type_:u8, pub reference_count:u16, pub flags:u8, pub os_semaphore: AcpiSemaphore }
#[repr(C)] pub struct AcpiObjectMutex { pub next_object:*mut AcpiOperandObject, pub descriptor_type:u8, pub type_:u8, pub reference_count:u16, pub flags:u8, pub sync_level:u8, pub acquisition_depth:u16, pub os_mutex:AcpiMutex, pub thread_id:AcpiThreadId, pub owner_thread:*mut AcpiThreadState, pub prev:*mut AcpiOperandObject, pub next:*mut AcpiOperandObject, pub node:*mut AcpiNamespaceNode, pub original_sync_level:u8 }
#[repr(C)] pub struct AcpiObjectRegion { pub next_object:*mut AcpiOperandObject, pub descriptor_type:u8, pub type_:u8, pub reference_count:u16, pub flags:u8, pub space_id:u8, pub node:*mut AcpiNamespaceNode, pub handler:*mut AcpiOperandObject, pub next:*mut AcpiOperandObject, pub address:AcpiPhysicalAddress, pub length:u32, pub pointer:*mut core::ffi::c_void }

#[repr(C)]
pub union AcpiObjectMethodDispatch { pub implementation: AcpiInternalMethod, pub handler: *mut AcpiOperandObject }
#[repr(C)] pub struct AcpiObjectMethod { pub next_object:*mut AcpiOperandObject, pub descriptor_type:u8, pub type_:u8, pub reference_count:u16, pub flags:u8, pub info_flags:u8, pub param_count:u8, pub sync_level:u8, pub mutex:*mut AcpiOperandObject, pub node:*mut AcpiOperandObject, pub aml_start:*mut u8, pub dispatch:AcpiObjectMethodDispatch, pub aml_length:u32, pub owner_id:AcpiOwnerId, pub thread_count:u8 }

pub const ACPI_METHOD_MODULE_LEVEL:u8=0x01; pub const ACPI_METHOD_INTERNAL_ONLY:u8=0x02; pub const ACPI_METHOD_SERIALIZED:u8=0x04; pub const ACPI_METHOD_SERIALIZED_PENDING:u8=0x08; pub const ACPI_METHOD_IGNORE_SYNC_LEVEL:u8=0x10; pub const ACPI_METHOD_MODIFIED_NAMESPACE:u8=0x20;

#[repr(C)] pub struct AcpiObjectNotifyCommon { pub next_object:*mut AcpiOperandObject,pub descriptor_type:u8,pub type_:u8,pub reference_count:u16,pub flags:u8,pub notify_list:[*mut AcpiOperandObject;2],pub handler:*mut AcpiOperandObject }
#[repr(C)] pub struct AcpiObjectDevice { pub next_object:*mut AcpiOperandObject,pub descriptor_type:u8,pub type_:u8,pub reference_count:u16,pub flags:u8,pub notify_list:[*mut AcpiOperandObject;2],pub handler:*mut AcpiOperandObject,pub gpe_block:*mut AcpiGpeBlockInfo }
#[repr(C)] pub struct AcpiObjectPowerResource { pub next_object:*mut AcpiOperandObject,pub descriptor_type:u8,pub type_:u8,pub reference_count:u16,pub flags:u8,pub notify_list:[*mut AcpiOperandObject;2],pub handler:*mut AcpiOperandObject,pub system_level:u32,pub resource_order:u32 }
#[repr(C)] pub struct AcpiObjectProcessor { pub next_object:*mut AcpiOperandObject,pub descriptor_type:u8,pub type_:u8,pub reference_count:u16,pub flags:u8,pub proc_id:u8,pub length:u8,pub notify_list:[*mut AcpiOperandObject;2],pub handler:*mut AcpiOperandObject,pub address:AcpiIoAddress }
#[repr(C)] pub struct AcpiObjectThermalZone { pub next_object:*mut AcpiOperandObject,pub descriptor_type:u8,pub type_:u8,pub reference_count:u16,pub flags:u8,pub notify_list:[*mut AcpiOperandObject;2],pub handler:*mut AcpiOperandObject }

#[repr(C)] pub struct AcpiObjectFieldCommon { pub next_object:*mut AcpiOperandObject,pub descriptor_type:u8,pub type_:u8,pub reference_count:u16,pub flags:u8,pub field_flags:u8,pub attribute:u8,pub access_byte_width:u8,pub node:*mut AcpiNamespaceNode,pub bit_length:u32,pub base_byte_offset:u32,pub value:u32,pub start_field_bit_offset:u8,pub access_length:u8,pub region_obj:*mut AcpiOperandObject }
#[repr(C)] pub struct AcpiObjectRegionField { pub next_object:*mut AcpiOperandObject,pub descriptor_type:u8,pub type_:u8,pub reference_count:u16,pub flags:u8,pub field_flags:u8,pub attribute:u8,pub access_byte_width:u8,pub node:*mut AcpiNamespaceNode,pub bit_length:u32,pub base_byte_offset:u32,pub value:u32,pub start_field_bit_offset:u8,pub access_length:u8,pub resource_length:u16,pub region_obj:*mut AcpiOperandObject,pub resource_buffer:*mut u8,pub pin_number_index:u16,pub internal_pcc_buffer:*mut u8 }
#[repr(C)] pub struct AcpiObjectBankField { pub common:AcpiObjectFieldCommon,pub bank_obj:*mut AcpiOperandObject }
#[repr(C)] pub struct AcpiObjectIndexField { pub common:AcpiObjectFieldCommon,pub index_obj:*mut AcpiOperandObject,pub data_obj:*mut AcpiOperandObject }
#[repr(C)] pub struct AcpiObjectBufferField { pub common:AcpiObjectFieldCommon,pub is_create_field:u8,pub buffer_obj:*mut AcpiOperandObject }

#[repr(C)] pub struct AcpiObjectNotifyHandler { pub next_object:*mut AcpiOperandObject,pub descriptor_type:u8,pub type_:u8,pub reference_count:u16,pub flags:u8,pub node:*mut AcpiNamespaceNode,pub handler_type:u32,pub handler:AcpiNotifyHandler,pub context:*mut core::ffi::c_void,pub next:[*mut AcpiOperandObject;2] }
#[repr(C)] pub struct AcpiObjectAddrHandler { pub next_object:*mut AcpiOperandObject,pub descriptor_type:u8,pub type_:u8,pub reference_count:u16,pub flags:u8,pub space_id:u8,pub handler_flags:u8,pub handler:AcpiAdrSpaceHandler,pub node:*mut AcpiNamespaceNode,pub context:*mut core::ffi::c_void,pub context_mutex:AcpiMutex,pub setup:AcpiAdrSpaceSetup,pub region_list:*mut AcpiOperandObject,pub next:*mut AcpiOperandObject }
pub const ACPI_ADDR_HANDLER_DEFAULT_INSTALLED:u8=0x01;

#[repr(C)] pub struct AcpiObjectReference { pub next_object:*mut AcpiOperandObject,pub descriptor_type:u8,pub type_:u8,pub reference_count:u16,pub flags:u8,pub class:u8,pub target_type:u8,pub resolved:u8,pub object:*mut core::ffi::c_void,pub node:*mut AcpiNamespaceNode,pub where_:*mut *mut AcpiOperandObject,pub index_pointer:*mut u8,pub aml:*mut u8,pub value:u32 }
#[repr(C)] pub enum AcpiReferenceClasses { ACPI_REFCLASS_LOCAL=0,ACPI_REFCLASS_ARG=1,ACPI_REFCLASS_REFOF=2,ACPI_REFCLASS_INDEX=3,ACPI_REFCLASS_TABLE=4,ACPI_REFCLASS_NAME=5,ACPI_REFCLASS_DEBUG=6,ACPI_REFCLASS_MAX=6 }
#[repr(C)] pub struct AcpiObjectExtra { pub next_object:*mut AcpiOperandObject,pub descriptor_type:u8,pub type_:u8,pub reference_count:u16,pub flags:u8,pub method_REG:*mut AcpiNamespaceNode,pub scope_node:*mut AcpiNamespaceNode,pub region_context:*mut core::ffi::c_void,pub aml_start:*mut u8,pub aml_length:u32 }
#[repr(C)] pub struct AcpiObjectData { pub next_object:*mut AcpiOperandObject,pub descriptor_type:u8,pub type_:u8,pub reference_count:u16,pub flags:u8,pub handler:AcpiObjectHandler,pub pointer:*mut core::ffi::c_void }
#[repr(C)] pub struct AcpiObjectCacheList { pub next_object:*mut AcpiOperandObject,pub descriptor_type:u8,pub type_:u8,pub reference_count:u16,pub flags:u8,pub next:*mut AcpiOperandObject }

#[repr(C)] pub union AcpiOperandObject { pub common:AcpiObjectCommon,pub integer:AcpiObjectInteger,pub string:AcpiObjectString,pub buffer:AcpiObjectBuffer,pub package:AcpiObjectPackage,pub event:AcpiObjectEvent,pub method:AcpiObjectMethod,pub mutex:AcpiObjectMutex,pub region:AcpiObjectRegion,pub common_notify:AcpiObjectNotifyCommon,pub device:AcpiObjectDevice,pub power_resource:AcpiObjectPowerResource,pub processor:AcpiObjectProcessor,pub thermal_zone:AcpiObjectThermalZone,pub common_field:AcpiObjectFieldCommon,pub field:AcpiObjectRegionField,pub buffer_field:AcpiObjectBufferField,pub bank_field:AcpiObjectBankField,pub index_field:AcpiObjectIndexField,pub notify:AcpiObjectNotifyHandler,pub address_space:AcpiObjectAddrHandler,pub reference:AcpiObjectReference,pub extra:AcpiObjectExtra,pub data:AcpiObjectData,pub cache:AcpiObjectCacheList,pub node:AcpiNamespaceNode }

pub const ACPI_DESC_TYPE_CACHED:u8=0x01; pub const ACPI_DESC_TYPE_STATE:u8=0x02; pub const ACPI_DESC_TYPE_STATE_UPDATE:u8=0x03; pub const ACPI_DESC_TYPE_STATE_PACKAGE:u8=0x04; pub const ACPI_DESC_TYPE_STATE_CONTROL:u8=0x05; pub const ACPI_DESC_TYPE_STATE_RPSCOPE:u8=0x06; pub const ACPI_DESC_TYPE_STATE_PSCOPE:u8=0x07; pub const ACPI_DESC_TYPE_STATE_WSCOPE:u8=0x08; pub const ACPI_DESC_TYPE_STATE_RESULT:u8=0x09; pub const ACPI_DESC_TYPE_STATE_NOTIFY:u8=0x0A; pub const ACPI_DESC_TYPE_STATE_THREAD:u8=0x0B; pub const ACPI_DESC_TYPE_WALK:u8=0x0C; pub const ACPI_DESC_TYPE_PARSER:u8=0x0D; pub const ACPI_DESC_TYPE_OPERAND:u8=0x0E; pub const ACPI_DESC_TYPE_NAMED:u8=0x0F; pub const ACPI_DESC_TYPE_MAX:u8=0x0F;
#[repr(C)] pub struct AcpiCommonDescriptor { pub common_pointer:*mut core::ffi::c_void,pub descriptor_type:u8 }
#[repr(C)] pub union AcpiDescriptor { pub common:AcpiCommonDescriptor,pub object:AcpiOperandObject,pub node:AcpiNamespaceNode,pub op:AcpiParseObject }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
