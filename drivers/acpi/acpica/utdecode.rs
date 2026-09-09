// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Utility decoding routines (value-to-string)

/* Dependencies are supplied by the surrounding ACPI translation unit. */

pub static acpi_gbl_ns_properties: [u8; ACPI_NUM_NS_TYPES] = [
    ACPI_NS_NORMAL, ACPI_NS_NORMAL, ACPI_NS_NORMAL, ACPI_NS_NORMAL,
    ACPI_NS_NORMAL, ACPI_NS_NORMAL, ACPI_NS_NEWSCOPE, ACPI_NS_NORMAL,
    ACPI_NS_NEWSCOPE, ACPI_NS_NORMAL, ACPI_NS_NORMAL, ACPI_NS_NEWSCOPE,
    ACPI_NS_NEWSCOPE, ACPI_NS_NEWSCOPE, ACPI_NS_NORMAL, ACPI_NS_NORMAL,
    ACPI_NS_NORMAL, ACPI_NS_NORMAL, ACPI_NS_NORMAL, ACPI_NS_NORMAL,
    ACPI_NS_NORMAL, ACPI_NS_NORMAL, ACPI_NS_NORMAL, ACPI_NS_NORMAL,
    ACPI_NS_NORMAL, ACPI_NS_NEWSCOPE | ACPI_NS_LOCAL,
    ACPI_NS_NEWSCOPE | ACPI_NS_LOCAL, ACPI_NS_NEWSCOPE, ACPI_NS_NORMAL,
    ACPI_NS_NORMAL, ACPI_NS_NORMAL,
];

pub static acpi_gbl_region_types: [&'static str; ACPI_NUM_PREDEFINED_REGIONS] = [
    "SystemMemory", "SystemIO", "PCI_Config", "EmbeddedControl", "SMBus",
    "SystemCMOS", "PCIBARTarget", "IPMI", "GeneralPurposeIo",
    "GenericSerialBus", "PCC", "PlatformRtMechanism",
];

pub unsafe fn acpi_ut_get_region_name(space_id: u8) -> &'static str {
    if space_id >= ACPI_USER_REGION_BEGIN { "UserDefinedRegion" }
    else if space_id == ACPI_ADR_SPACE_DATA_TABLE { "DataTable" }
    else if space_id == ACPI_ADR_SPACE_FIXED_HARDWARE { "FunctionalFixedHW" }
    else if space_id >= ACPI_NUM_PREDEFINED_REGIONS { "InvalidSpaceId" }
    else { acpi_gbl_region_types[space_id as usize] }
}

static acpi_gbl_event_types: [&'static str; ACPI_NUM_FIXED_EVENTS] =
    ["PM_Timer", "GlobalLock", "PowerButton", "SleepButton", "RealTimeClock"];

pub unsafe fn acpi_ut_get_event_name(event_id: u32) -> &'static str {
    if event_id > ACPI_EVENT_MAX { "InvalidEventID" }
    else { acpi_gbl_event_types[event_id as usize] }
}

static acpi_gbl_bad_type: &str = "UNDEFINED";
static acpi_gbl_ns_type_names: [&'static str; 31] = [
    "Untyped", "Integer", "String", "Buffer", "Package", "FieldUnit", "Device",
    "Event", "Method", "Mutex", "Region", "Power", "Processor", "Thermal",
    "BufferField", "DdbHandle", "DebugObject", "RegionField", "BankField",
    "IndexField", "Reference", "Alias", "MethodAlias", "Notify", "AddrHandler",
    "ResourceDesc", "ResourceFld", "Scope", "Extra", "Data", "Invalid",
];

pub unsafe fn acpi_ut_get_type_name(type_: acpi_object_type) -> &'static str {
    if type_ > ACPI_TYPE_INVALID { acpi_gbl_bad_type }
    else { acpi_gbl_ns_type_names[type_ as usize] }
}

pub unsafe fn acpi_ut_get_object_type_name(
    obj_desc: *mut acpi_operand_object,
) -> &'static str {
    if obj_desc.is_null() { return "[NULL Object Descriptor]"; }
    let descriptor_type = ACPI_GET_DESCRIPTOR_TYPE(obj_desc);
    if descriptor_type != ACPI_DESC_TYPE_OPERAND && descriptor_type != ACPI_DESC_TYPE_NAMED {
        return "Invalid object";
    }
    acpi_ut_get_type_name((*obj_desc).common.type_)
}

pub unsafe fn acpi_ut_get_node_name(object: *mut core::ffi::c_void) -> &'static str {
    let node = object as *mut acpi_namespace_node;
    if object.is_null() { return "NULL"; }
    if object == ACPI_ROOT_OBJECT || object == acpi_gbl_root_node {
        return "\"\\\" ";
    }
    if ACPI_GET_DESCRIPTOR_TYPE(node) != ACPI_DESC_TYPE_NAMED { return "####"; }
    acpi_ut_repair_name((*node).name.ascii.as_mut_ptr());
    core::ffi::CStr::from_ptr((*node).name.ascii.as_ptr()).to_str().unwrap_or("")
}

static acpi_gbl_desc_type_names: [&'static str; 16] = [
    "Not a Descriptor", "Cached Object", "State-Generic", "State-Update",
    "State-Package", "State-Control", "State-RootParseScope", "State-ParseScope",
    "State-WalkScope", "State-Result", "State-Notify", "State-Thread",
    "Tree Walk State", "Parse Tree Op", "Operand Object", "Namespace Node",
];

pub unsafe fn acpi_ut_get_descriptor_name(object: *mut core::ffi::c_void) -> &'static str {
    if object.is_null() { return "NULL OBJECT"; }
    let descriptor_type = ACPI_GET_DESCRIPTOR_TYPE(object);
    if descriptor_type > ACPI_DESC_TYPE_MAX { return "Not a Descriptor"; }
    acpi_gbl_desc_type_names[descriptor_type as usize]
}

static acpi_gbl_ref_class_names: [&'static str; 7] =
    ["Local", "Argument", "RefOf", "Index", "DdbHandle", "Named Object", "Debug"];

pub unsafe fn acpi_ut_get_reference_name(object: *mut acpi_operand_object) -> &'static str {
    if object.is_null() { return "NULL Object"; }
    if ACPI_GET_DESCRIPTOR_TYPE(object) != ACPI_DESC_TYPE_OPERAND { return "Not an Operand object"; }
    if (*object).common.type_ != ACPI_TYPE_LOCAL_REFERENCE { return "Not a Reference object"; }
    if (*object).reference.class_ > ACPI_REFCLASS_MAX { return "Unknown Reference class"; }
    acpi_gbl_ref_class_names[(*object).reference.class_ as usize]
}

static acpi_gbl_mutex_names: [&'static str; ACPI_NUM_MUTEX] = [
    "ACPI_MTX_Interpreter", "ACPI_MTX_Namespace", "ACPI_MTX_Tables",
    "ACPI_MTX_Events", "ACPI_MTX_Caches", "ACPI_MTX_Memory",
];

pub unsafe fn acpi_ut_get_mutex_name(mutex_id: u32) -> &'static str {
    if mutex_id > ACPI_MAX_MUTEX { "Invalid Mutex ID" }
    else { acpi_gbl_mutex_names[mutex_id as usize] }
}

#[cfg(any(ACPI_DEBUG_OUTPUT, ACPI_DEBUGGER))]
pub unsafe fn acpi_ut_get_notify_name(notify_value: u32, type_: acpi_object_type) -> &'static str {
    const GENERIC: [&str; 16] = ["Bus Check", "Device Check", "Device Wake", "Eject Request", "Device Check Light", "Frequency Mismatch", "Bus Mode Mismatch", "Power Fault", "Capabilities Check", "Device PLD Check", "Reserved", "System Locality Update", "Reserved (was previously Shutdown Request)", "System Resource Affinity Update", "Heterogeneous Memory Attributes Update", "Error Disconnect Recover"];
    const DEVICE: [&str; 5] = ["Status Change", "Information Change", "Device-Specific Change", "Device-Specific Change", "Reserved"];
    const PROCESSOR: [&str; 5] = ["Performance Capability Change", "C-State Change", "Throttling Capability Change", "Guaranteed Change", "Minimum Excursion"];
    const THERMAL: [&str; 5] = ["Thermal Status Change", "Thermal Trip Point Change", "Thermal Device List Change", "Thermal Relationship Change", "Reserved"];
    if notify_value <= ACPI_GENERIC_NOTIFY_MAX { return GENERIC[notify_value as usize]; }
    if notify_value <= ACPI_MAX_SYS_NOTIFY { return "Reserved"; }
    if notify_value <= ACPI_SPECIFIC_NOTIFY_MAX {
        let i = (notify_value - 0x80) as usize;
        return match type_ { ACPI_TYPE_ANY | ACPI_TYPE_DEVICE => DEVICE[i], ACPI_TYPE_PROCESSOR => PROCESSOR[i], ACPI_TYPE_THERMAL => THERMAL[i], _ => "Target object type does not support notifies" };
    }
    if notify_value <= ACPI_MAX_DEVICE_SPECIFIC_NOTIFY { "Device-Specific" } else { "Hardware-Specific" }
}

#[cfg(any(ACPI_DEBUG_OUTPUT, ACPI_DEBUGGER))]
pub unsafe fn acpi_ut_get_argument_type_name(arg_type: u32) -> &'static str {
    const ARGUMENT: [&str; 20] = ["Unknown ARGP", "ByteData", "ByteList", "CharList", "DataObject", "DataObjectList", "DWordData", "FieldList", "Name", "NameString", "ObjectList", "PackageLength", "SuperName", "Target", "TermArg", "TermList", "WordData", "QWordData", "SimpleName", "NameOrRef"];
    if arg_type > ARGP_MAX { "Unknown ARGP" } else { ARGUMENT[arg_type as usize] }
}

pub unsafe fn acpi_ut_valid_object_type(type_: acpi_object_type) -> u8 {
    if type_ > ACPI_TYPE_LOCAL_MAX { FALSE } else { TRUE }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
