/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/* Exception codes returned by the ACPI subsystem. */

/* The C header depends on the externally supplied `acpi_status` type. */

pub const AE_CODE_ENVIRONMENTAL: u32 = 0x0000;
pub const AE_CODE_PROGRAMMER: u32 = 0x1000;
pub const AE_CODE_ACPI_TABLES: u32 = 0x2000;
pub const AE_CODE_AML: u32 = 0x3000;
pub const AE_CODE_CONTROL: u32 = 0x4000;
pub const AE_CODE_MAX: u32 = 0x4000;
pub const AE_CODE_MASK: u32 = 0xF000;

macro_rules! EXCEP_ENV { ($code:expr) => { (($code | AE_CODE_ENVIRONMENTAL) as acpi_status) }; }
macro_rules! EXCEP_PGM { ($code:expr) => { (($code | AE_CODE_PROGRAMMER) as acpi_status) }; }
macro_rules! EXCEP_TBL { ($code:expr) => { (($code | AE_CODE_ACPI_TABLES) as acpi_status) }; }
macro_rules! EXCEP_AML { ($code:expr) => { (($code | AE_CODE_AML) as acpi_status) }; }
macro_rules! EXCEP_CTL { ($code:expr) => { (($code | AE_CODE_CONTROL) as acpi_status) }; }

#[repr(C)]
pub struct acpi_exception_info {
    pub name: *mut core::ffi::c_char,
    #[cfg(any(feature = "ACPI_HELP_APP", feature = "ACPI_ASL_COMPILER"))]
    pub description: *mut core::ffi::c_char,
}

#[cfg(any(feature = "ACPI_HELP_APP", feature = "ACPI_ASL_COMPILER"))]
macro_rules! EXCEP_TXT { ($name:expr, $description:expr) => { acpi_exception_info { name: $name.as_ptr() as *mut _, description: $description.as_ptr() as *mut _ } }; }
#[cfg(not(any(feature = "ACPI_HELP_APP", feature = "ACPI_ASL_COMPILER")))]
macro_rules! EXCEP_TXT { ($name:expr, $description:expr) => { acpi_exception_info { name: $name.as_ptr() as *mut _ } }; }

pub const AE_OK: acpi_status = 0x0000 as acpi_status;
pub const AE_ERROR: acpi_status = EXCEP_ENV!(0x0001);
pub const AE_NO_ACPI_TABLES: acpi_status = EXCEP_ENV!(0x0002);
pub const AE_NO_NAMESPACE: acpi_status = EXCEP_ENV!(0x0003);
pub const AE_NO_MEMORY: acpi_status = EXCEP_ENV!(0x0004);
pub const AE_NOT_FOUND: acpi_status = EXCEP_ENV!(0x0005);
pub const AE_NOT_EXIST: acpi_status = EXCEP_ENV!(0x0006);
pub const AE_ALREADY_EXISTS: acpi_status = EXCEP_ENV!(0x0007);
pub const AE_TYPE: acpi_status = EXCEP_ENV!(0x0008);
pub const AE_NULL_OBJECT: acpi_status = EXCEP_ENV!(0x0009);
pub const AE_NULL_ENTRY: acpi_status = EXCEP_ENV!(0x000A);
pub const AE_BUFFER_OVERFLOW: acpi_status = EXCEP_ENV!(0x000B);
pub const AE_STACK_OVERFLOW: acpi_status = EXCEP_ENV!(0x000C);
pub const AE_STACK_UNDERFLOW: acpi_status = EXCEP_ENV!(0x000D);
pub const AE_NOT_IMPLEMENTED: acpi_status = EXCEP_ENV!(0x000E);
pub const AE_SUPPORT: acpi_status = EXCEP_ENV!(0x000F);
pub const AE_LIMIT: acpi_status = EXCEP_ENV!(0x0010);
pub const AE_TIME: acpi_status = EXCEP_ENV!(0x0011);
pub const AE_ACQUIRE_DEADLOCK: acpi_status = EXCEP_ENV!(0x0012);
pub const AE_RELEASE_DEADLOCK: acpi_status = EXCEP_ENV!(0x0013);
pub const AE_NOT_ACQUIRED: acpi_status = EXCEP_ENV!(0x0014);
pub const AE_ALREADY_ACQUIRED: acpi_status = EXCEP_ENV!(0x0015);
pub const AE_NO_HARDWARE_RESPONSE: acpi_status = EXCEP_ENV!(0x0016);
pub const AE_NO_GLOBAL_LOCK: acpi_status = EXCEP_ENV!(0x0017);
pub const AE_ABORT_METHOD: acpi_status = EXCEP_ENV!(0x0018);
pub const AE_SAME_HANDLER: acpi_status = EXCEP_ENV!(0x0019);
pub const AE_NO_HANDLER: acpi_status = EXCEP_ENV!(0x001A);
pub const AE_OWNER_ID_LIMIT: acpi_status = EXCEP_ENV!(0x001B);
pub const AE_NOT_CONFIGURED: acpi_status = EXCEP_ENV!(0x001C);
pub const AE_ACCESS: acpi_status = EXCEP_ENV!(0x001D);
pub const AE_IO_ERROR: acpi_status = EXCEP_ENV!(0x001E);
pub const AE_NUMERIC_OVERFLOW: acpi_status = EXCEP_ENV!(0x001F);
pub const AE_HEX_OVERFLOW: acpi_status = EXCEP_ENV!(0x0020);
pub const AE_DECIMAL_OVERFLOW: acpi_status = EXCEP_ENV!(0x0021);
pub const AE_OCTAL_OVERFLOW: acpi_status = EXCEP_ENV!(0x0022);
pub const AE_END_OF_TABLE: acpi_status = EXCEP_ENV!(0x0023);
pub const AE_CODE_ENV_MAX: u32 = 0x0023;

pub const AE_BAD_PARAMETER: acpi_status = EXCEP_PGM!(0x0001);
pub const AE_BAD_CHARACTER: acpi_status = EXCEP_PGM!(0x0002);
pub const AE_BAD_PATHNAME: acpi_status = EXCEP_PGM!(0x0003);
pub const AE_BAD_DATA: acpi_status = EXCEP_PGM!(0x0004);
pub const AE_BAD_HEX_CONSTANT: acpi_status = EXCEP_PGM!(0x0005);
pub const AE_BAD_OCTAL_CONSTANT: acpi_status = EXCEP_PGM!(0x0006);
pub const AE_BAD_DECIMAL_CONSTANT: acpi_status = EXCEP_PGM!(0x0007);
pub const AE_MISSING_ARGUMENTS: acpi_status = EXCEP_PGM!(0x0008);
pub const AE_BAD_ADDRESS: acpi_status = EXCEP_PGM!(0x0009);
pub const AE_CODE_PGM_MAX: u32 = 0x0009;

pub const AE_BAD_SIGNATURE: acpi_status = EXCEP_TBL!(0x0001);
pub const AE_BAD_HEADER: acpi_status = EXCEP_TBL!(0x0002);
pub const AE_BAD_CHECKSUM: acpi_status = EXCEP_TBL!(0x0003);
pub const AE_BAD_VALUE: acpi_status = EXCEP_TBL!(0x0004);
pub const AE_INVALID_TABLE_LENGTH: acpi_status = EXCEP_TBL!(0x0005);
pub const AE_CODE_TBL_MAX: u32 = 0x0005;

pub const AE_AML_BAD_OPCODE: acpi_status = EXCEP_AML!(0x0001);
pub const AE_AML_NO_OPERAND: acpi_status = EXCEP_AML!(0x0002);
pub const AE_AML_OPERAND_TYPE: acpi_status = EXCEP_AML!(0x0003);
pub const AE_AML_OPERAND_VALUE: acpi_status = EXCEP_AML!(0x0004);
pub const AE_AML_UNINITIALIZED_LOCAL: acpi_status = EXCEP_AML!(0x0005);
pub const AE_AML_UNINITIALIZED_ARG: acpi_status = EXCEP_AML!(0x0006);
pub const AE_AML_UNINITIALIZED_ELEMENT: acpi_status = EXCEP_AML!(0x0007);
pub const AE_AML_NUMERIC_OVERFLOW: acpi_status = EXCEP_AML!(0x0008);
pub const AE_AML_REGION_LIMIT: acpi_status = EXCEP_AML!(0x0009);
pub const AE_AML_BUFFER_LIMIT: acpi_status = EXCEP_AML!(0x000A);
pub const AE_AML_PACKAGE_LIMIT: acpi_status = EXCEP_AML!(0x000B);
pub const AE_AML_DIVIDE_BY_ZERO: acpi_status = EXCEP_AML!(0x000C);
pub const AE_AML_BAD_NAME: acpi_status = EXCEP_AML!(0x000D);
pub const AE_AML_NAME_NOT_FOUND: acpi_status = EXCEP_AML!(0x000E);
pub const AE_AML_INTERNAL: acpi_status = EXCEP_AML!(0x000F);
pub const AE_AML_INVALID_SPACE_ID: acpi_status = EXCEP_AML!(0x0010);
pub const AE_AML_STRING_LIMIT: acpi_status = EXCEP_AML!(0x0011);
pub const AE_AML_NO_RETURN_VALUE: acpi_status = EXCEP_AML!(0x0012);
pub const AE_AML_METHOD_LIMIT: acpi_status = EXCEP_AML!(0x0013);
pub const AE_AML_NOT_OWNER: acpi_status = EXCEP_AML!(0x0014);
pub const AE_AML_MUTEX_ORDER: acpi_status = EXCEP_AML!(0x0015);
pub const AE_AML_MUTEX_NOT_ACQUIRED: acpi_status = EXCEP_AML!(0x0016);
pub const AE_AML_INVALID_RESOURCE_TYPE: acpi_status = EXCEP_AML!(0x0017);
pub const AE_AML_INVALID_INDEX: acpi_status = EXCEP_AML!(0x0018);
pub const AE_AML_REGISTER_LIMIT: acpi_status = EXCEP_AML!(0x0019);
pub const AE_AML_NO_WHILE: acpi_status = EXCEP_AML!(0x001A);
pub const AE_AML_ALIGNMENT: acpi_status = EXCEP_AML!(0x001B);
pub const AE_AML_NO_RESOURCE_END_TAG: acpi_status = EXCEP_AML!(0x001C);
pub const AE_AML_BAD_RESOURCE_VALUE: acpi_status = EXCEP_AML!(0x001D);
pub const AE_AML_CIRCULAR_REFERENCE: acpi_status = EXCEP_AML!(0x001E);
pub const AE_AML_BAD_RESOURCE_LENGTH: acpi_status = EXCEP_AML!(0x001F);
pub const AE_AML_ILLEGAL_ADDRESS: acpi_status = EXCEP_AML!(0x0020);
pub const AE_AML_LOOP_TIMEOUT: acpi_status = EXCEP_AML!(0x0021);
pub const AE_AML_UNINITIALIZED_NODE: acpi_status = EXCEP_AML!(0x0022);
pub const AE_AML_TARGET_TYPE: acpi_status = EXCEP_AML!(0x0023);
pub const AE_AML_PROTOCOL: acpi_status = EXCEP_AML!(0x0024);
pub const AE_AML_BUFFER_LENGTH: acpi_status = EXCEP_AML!(0x0025);
pub const AE_AML_TOO_FEW_ARGUMENTS: acpi_status = EXCEP_AML!(0x0026);
pub const AE_AML_TOO_MANY_ARGUMENTS: acpi_status = EXCEP_AML!(0x0027);
pub const AE_CODE_AML_MAX: u32 = 0x0027;

pub const AE_CTRL_RETURN_VALUE: acpi_status = EXCEP_CTL!(0x0001);
pub const AE_CTRL_PENDING: acpi_status = EXCEP_CTL!(0x0002);
pub const AE_CTRL_TERMINATE: acpi_status = EXCEP_CTL!(0x0003);
pub const AE_CTRL_TRUE: acpi_status = EXCEP_CTL!(0x0004);
pub const AE_CTRL_FALSE: acpi_status = EXCEP_CTL!(0x0005);
pub const AE_CTRL_DEPTH: acpi_status = EXCEP_CTL!(0x0006);
pub const AE_CTRL_END: acpi_status = EXCEP_CTL!(0x0007);
pub const AE_CTRL_TRANSFER: acpi_status = EXCEP_CTL!(0x0008);
pub const AE_CTRL_BREAK: acpi_status = EXCEP_CTL!(0x0009);
pub const AE_CTRL_CONTINUE: acpi_status = EXCEP_CTL!(0x000A);
pub const AE_CTRL_PARSE_CONTINUE: acpi_status = EXCEP_CTL!(0x000B);
pub const AE_CTRL_PARSE_PENDING: acpi_status = EXCEP_CTL!(0x000C);
pub const AE_CODE_CTRL_MAX: u32 = 0x000C;

/* Success is always zero, failure is non-zero. */
#[inline] pub const fn ACPI_SUCCESS(a: acpi_status) -> bool { a == 0 }
#[inline] pub const fn ACPI_FAILURE(a: acpi_status) -> bool { a != 0 }
#[inline] pub const fn ACPI_ENV_EXCEPTION(status: acpi_status) -> bool { (status & AE_CODE_MASK as acpi_status) == AE_CODE_ENVIRONMENTAL as acpi_status }
#[inline] pub const fn ACPI_AML_EXCEPTION(status: acpi_status) -> bool { (status & AE_CODE_MASK as acpi_status) == AE_CODE_AML as acpi_status }
#[inline] pub const fn ACPI_PROG_EXCEPTION(status: acpi_status) -> bool { (status & AE_CODE_MASK as acpi_status) == AE_CODE_PROGRAMMER as acpi_status }
#[inline] pub const fn ACPI_TABLE_EXCEPTION(status: acpi_status) -> bool { (status & AE_CODE_MASK as acpi_status) == AE_CODE_ACPI_TABLES as acpi_status }
#[inline] pub const fn ACPI_CNTL_EXCEPTION(status: acpi_status) -> bool { (status & AE_CODE_MASK as acpi_status) == AE_CODE_CONTROL as acpi_status }

/* Exception strings for acpi_format_exception, defined when requested by the build. */
#[cfg(feature = "ACPI_DEFINE_EXCEPTION_TABLE")]
pub static acpi_gbl_exception_names_env: &[acpi_exception_info] = &[
    EXCEP_TXT!(b"AE_OK\0", b"No error\0"), EXCEP_TXT!(b"AE_ERROR\0", b"Unspecified error\0"),
    EXCEP_TXT!(b"AE_NO_ACPI_TABLES\0", b"ACPI tables could not be found\0"), EXCEP_TXT!(b"AE_NO_NAMESPACE\0", b"A namespace has not been loaded\0"),
    EXCEP_TXT!(b"AE_NO_MEMORY\0", b"Insufficient dynamic memory\0"), EXCEP_TXT!(b"AE_NOT_FOUND\0", b"A requested entity is not found\0"),
    EXCEP_TXT!(b"AE_NOT_EXIST\0", b"A required entity does not exist\0"), EXCEP_TXT!(b"AE_ALREADY_EXISTS\0", b"An entity already exists\0"),
    EXCEP_TXT!(b"AE_TYPE\0", b"The object type is incorrect\0"), EXCEP_TXT!(b"AE_NULL_OBJECT\0", b"A required object was missing\0"),
    EXCEP_TXT!(b"AE_NULL_ENTRY\0", b"The requested object does not exist\0"), EXCEP_TXT!(b"AE_BUFFER_OVERFLOW\0", b"The buffer provided is too small\0"),
    EXCEP_TXT!(b"AE_STACK_OVERFLOW\0", b"An internal stack overflowed\0"), EXCEP_TXT!(b"AE_STACK_UNDERFLOW\0", b"An internal stack underflowed\0"),
    EXCEP_TXT!(b"AE_NOT_IMPLEMENTED\0", b"The feature is not implemented\0"), EXCEP_TXT!(b"AE_SUPPORT\0", b"The feature is not supported\0"),
    EXCEP_TXT!(b"AE_LIMIT\0", b"A predefined limit was exceeded\0"), EXCEP_TXT!(b"AE_TIME\0", b"A time limit or timeout expired\0"),
    EXCEP_TXT!(b"AE_ACQUIRE_DEADLOCK\0", b"Internal error, attempt was made to acquire a mutex in improper order\0"),
    EXCEP_TXT!(b"AE_RELEASE_DEADLOCK\0", b"Internal error, attempt was made to release a mutex in improper order\0"),
    EXCEP_TXT!(b"AE_NOT_ACQUIRED\0", b"An attempt to release a mutex or Global Lock without a previous acquire\0"),
    EXCEP_TXT!(b"AE_ALREADY_ACQUIRED\0", b"Internal error, attempt was made to acquire a mutex twice\0"),
    EXCEP_TXT!(b"AE_NO_HARDWARE_RESPONSE\0", b"Hardware did not respond after an I/O operation\0"), EXCEP_TXT!(b"AE_NO_GLOBAL_LOCK\0", b"There is no FACS Global Lock\0"),
    EXCEP_TXT!(b"AE_ABORT_METHOD\0", b"A control method was aborted\0"), EXCEP_TXT!(b"AE_SAME_HANDLER\0", b"Attempt was made to install the same handler that is already installed\0"),
    EXCEP_TXT!(b"AE_NO_HANDLER\0", b"A handler for the operation is not installed\0"), EXCEP_TXT!(b"AE_OWNER_ID_LIMIT\0", b"There are no more Owner IDs available for ACPI tables or control methods\0"),
    EXCEP_TXT!(b"AE_NOT_CONFIGURED\0", b"The interface is not part of the current subsystem configuration\0"), EXCEP_TXT!(b"AE_ACCESS\0", b"Permission denied for the requested operation\0"),
    EXCEP_TXT!(b"AE_IO_ERROR\0", b"An I/O error occurred\0"), EXCEP_TXT!(b"AE_NUMERIC_OVERFLOW\0", b"Overflow during string-to-integer conversion\0"),
    EXCEP_TXT!(b"AE_HEX_OVERFLOW\0", b"Overflow during ASCII hex-to-binary conversion\0"), EXCEP_TXT!(b"AE_DECIMAL_OVERFLOW\0", b"Overflow during ASCII decimal-to-binary conversion\0"),
    EXCEP_TXT!(b"AE_OCTAL_OVERFLOW\0", b"Overflow during ASCII octal-to-binary conversion\0"), EXCEP_TXT!(b"AE_END_OF_TABLE\0", b"Reached the end of table\0"),
];

/* The remaining tables preserve the C table ordering; descriptions are retained in the
 * source-facing names and are supplied by the same conditional table mechanism. */
#[cfg(feature = "ACPI_DEFINE_EXCEPTION_TABLE")]
pub static acpi_gbl_exception_names_pgm: &[acpi_exception_info] = &[EXCEP_TXT!(b"\0", b"\0"), EXCEP_TXT!(b"AE_BAD_PARAMETER\0", b"A parameter is out of range or invalid\0"), EXCEP_TXT!(b"AE_BAD_CHARACTER\0", b"An invalid character was found in a name\0"), EXCEP_TXT!(b"AE_BAD_PATHNAME\0", b"An invalid character was found in a pathname\0"), EXCEP_TXT!(b"AE_BAD_DATA\0", b"A package or buffer contained incorrect data\0"), EXCEP_TXT!(b"AE_BAD_HEX_CONSTANT\0", b"Invalid character in a Hex constant\0"), EXCEP_TXT!(b"AE_BAD_OCTAL_CONSTANT\0", b"Invalid character in an Octal constant\0"), EXCEP_TXT!(b"AE_BAD_DECIMAL_CONSTANT\0", b"Invalid character in a Decimal constant\0"), EXCEP_TXT!(b"AE_MISSING_ARGUMENTS\0", b"Too few arguments were passed to a control method\0"), EXCEP_TXT!(b"AE_BAD_ADDRESS\0", b"An illegal null I/O address\0")];
#[cfg(feature = "ACPI_DEFINE_EXCEPTION_TABLE")]
pub static acpi_gbl_exception_names_tbl: &[acpi_exception_info] = &[EXCEP_TXT!(b"\0", b"\0"), EXCEP_TXT!(b"AE_BAD_SIGNATURE\0", b"An ACPI table has an invalid signature\0"), EXCEP_TXT!(b"AE_BAD_HEADER\0", b"Invalid field in an ACPI table header\0"), EXCEP_TXT!(b"AE_BAD_CHECKSUM\0", b"An ACPI table checksum is not correct\0"), EXCEP_TXT!(b"AE_BAD_VALUE\0", b"An invalid value was found in a table\0"), EXCEP_TXT!(b"AE_INVALID_TABLE_LENGTH\0", b"The FADT or FACS has improper length\0")];
#[cfg(feature = "ACPI_DEFINE_EXCEPTION_TABLE")]
pub static acpi_gbl_exception_names_aml: &[acpi_exception_info] = &[EXCEP_TXT!(b"\0", b"\0"), EXCEP_TXT!(b"AE_AML_BAD_OPCODE\0", b"Invalid AML opcode encountered\0"), EXCEP_TXT!(b"AE_AML_NO_OPERAND\0", b"A required operand is missing\0"), EXCEP_TXT!(b"AE_AML_OPERAND_TYPE\0", b"An operand of an incorrect type was encountered\0"), EXCEP_TXT!(b"AE_AML_OPERAND_VALUE\0", b"The operand had an inappropriate or invalid value\0"), EXCEP_TXT!(b"AE_AML_UNINITIALIZED_LOCAL\0", b"Method tried to use an uninitialized local variable\0"), EXCEP_TXT!(b"AE_AML_UNINITIALIZED_ARG\0", b"Method tried to use an uninitialized argument\0"), EXCEP_TXT!(b"AE_AML_UNINITIALIZED_ELEMENT\0", b"Method tried to use an empty package element\0"), EXCEP_TXT!(b"AE_AML_NUMERIC_OVERFLOW\0", b"Overflow during BCD conversion or other\0"), EXCEP_TXT!(b"AE_AML_REGION_LIMIT\0", b"Tried to access beyond the end of an Operation Region\0"), EXCEP_TXT!(b"AE_AML_BUFFER_LIMIT\0", b"Tried to access beyond the end of a buffer\0"), EXCEP_TXT!(b"AE_AML_PACKAGE_LIMIT\0", b"Tried to access beyond the end of a package\0"), EXCEP_TXT!(b"AE_AML_DIVIDE_BY_ZERO\0", b"During execution of AML Divide operator\0"), EXCEP_TXT!(b"AE_AML_BAD_NAME\0", b"An ACPI name contains invalid character(s)\0"), EXCEP_TXT!(b"AE_AML_NAME_NOT_FOUND\0", b"Could not resolve a named reference\0"), EXCEP_TXT!(b"AE_AML_INTERNAL\0", b"An internal error within the interpreter\0"), EXCEP_TXT!(b"AE_AML_INVALID_SPACE_ID\0", b"An Operation Region SpaceID is invalid\0"), EXCEP_TXT!(b"AE_AML_STRING_LIMIT\0", b"String is longer than 200 characters\0"), EXCEP_TXT!(b"AE_AML_NO_RETURN_VALUE\0", b"A method did not return a required value\0"), EXCEP_TXT!(b"AE_AML_METHOD_LIMIT\0", b"A control method reached the maximum reentrancy limit of 255\0"), EXCEP_TXT!(b"AE_AML_NOT_OWNER\0", b"A thread tried to release a mutex that it does not own\0"), EXCEP_TXT!(b"AE_AML_MUTEX_ORDER\0", b"Mutex SyncLevel release mismatch\0"), EXCEP_TXT!(b"AE_AML_MUTEX_NOT_ACQUIRED\0", b"Attempt to release a mutex that was not previously acquired\0"), EXCEP_TXT!(b"AE_AML_INVALID_RESOURCE_TYPE\0", b"Invalid resource type in resource list\0"), EXCEP_TXT!(b"AE_AML_INVALID_INDEX\0", b"Invalid Argx or Localx (x too large)\0"), EXCEP_TXT!(b"AE_AML_REGISTER_LIMIT\0", b"Bank value or Index value beyond range of register\0"), EXCEP_TXT!(b"AE_AML_NO_WHILE\0", b"Break or Continue without a While\0"), EXCEP_TXT!(b"AE_AML_ALIGNMENT\0", b"Non-aligned memory transfer on platform that does not support this\0"), EXCEP_TXT!(b"AE_AML_NO_RESOURCE_END_TAG\0", b"No End Tag in a resource list\0"), EXCEP_TXT!(b"AE_AML_BAD_RESOURCE_VALUE\0", b"Invalid value of a resource element\0"), EXCEP_TXT!(b"AE_AML_CIRCULAR_REFERENCE\0", b"Two references refer to each other\0"), EXCEP_TXT!(b"AE_AML_BAD_RESOURCE_LENGTH\0", b"The length of a Resource Descriptor in the AML is incorrect\0"), EXCEP_TXT!(b"AE_AML_ILLEGAL_ADDRESS\0", b"A memory, I/O, or PCI configuration address is invalid\0"), EXCEP_TXT!(b"AE_AML_LOOP_TIMEOUT\0", b"An AML While loop exceeded the maximum execution time\0"), EXCEP_TXT!(b"AE_AML_UNINITIALIZED_NODE\0", b"A namespace node is uninitialized or unresolved\0"), EXCEP_TXT!(b"AE_AML_TARGET_TYPE\0", b"A target operand of an incorrect type was encountered\0"), EXCEP_TXT!(b"AE_AML_PROTOCOL\0", b"Violation of a fixed ACPI protocol\0"), EXCEP_TXT!(b"AE_AML_BUFFER_LENGTH\0", b"The length of the buffer is invalid/incorrect\0"), EXCEP_TXT!(b"AE_AML_TOO_FEW_ARGUMENTS\0", b"There are fewer than expected method arguments\0"), EXCEP_TXT!(b"AE_AML_TOO_MANY_ARGUMENTS\0", b"There are too many arguments for this method\0")];
#[cfg(feature = "ACPI_DEFINE_EXCEPTION_TABLE")]
pub static acpi_gbl_exception_names_ctrl: &[acpi_exception_info] = &[EXCEP_TXT!(b"\0", b"\0"), EXCEP_TXT!(b"AE_CTRL_RETURN_VALUE\0", b"A Method returned a value\0"), EXCEP_TXT!(b"AE_CTRL_PENDING\0", b"Method is calling another method\0"), EXCEP_TXT!(b"AE_CTRL_TERMINATE\0", b"Terminate the executing method\0"), EXCEP_TXT!(b"AE_CTRL_TRUE\0", b"An If or While predicate result\0"), EXCEP_TXT!(b"AE_CTRL_FALSE\0", b"An If or While predicate result\0"), EXCEP_TXT!(b"AE_CTRL_DEPTH\0", b"Maximum search depth has been reached\0"), EXCEP_TXT!(b"AE_CTRL_END\0", b"An If or While predicate is false\0"), EXCEP_TXT!(b"AE_CTRL_TRANSFER\0", b"Transfer control to called method\0"), EXCEP_TXT!(b"AE_CTRL_BREAK\0", b"A Break has been executed\0"), EXCEP_TXT!(b"AE_CTRL_CONTINUE\0", b"A Continue has been executed\0"), EXCEP_TXT!(b"AE_CTRL_PARSE_CONTINUE\0", b"Used to skip over bad opcodes\0"), EXCEP_TXT!(b"AE_CTRL_PARSE_PENDING\0", b"Used to implement AML While loops\0")];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
