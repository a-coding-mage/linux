/* SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0 */
/*
 * Rust translation of acoutput.h -- debug output.
 * C preprocessor conditionals are preserved with cfg where practical.
 */

/* Component IDs */
pub const ACPI_UTILITIES: u32 = 0x00000001;
pub const ACPI_HARDWARE: u32 = 0x00000002;
pub const ACPI_EVENTS: u32 = 0x00000004;
pub const ACPI_TABLES: u32 = 0x00000008;
pub const ACPI_NAMESPACE: u32 = 0x00000010;
pub const ACPI_PARSER: u32 = 0x00000020;
pub const ACPI_DISPATCHER: u32 = 0x00000040;
pub const ACPI_EXECUTER: u32 = 0x00000080;
pub const ACPI_RESOURCES: u32 = 0x00000100;
pub const ACPI_CA_DEBUGGER: u32 = 0x00000200;
pub const ACPI_OS_SERVICES: u32 = 0x00000400;
pub const ACPI_CA_DISASSEMBLER: u32 = 0x00000800;
pub const ACPI_COMPILER: u32 = 0x00001000;
pub const ACPI_TOOLS: u32 = 0x00002000;
pub const ACPI_EXAMPLE: u32 = 0x00004000;
pub const ACPI_DRIVER: u32 = 0x00008000;
pub const DT_COMPILER: u32 = 0x00010000;
pub const ASL_PREPROCESSOR: u32 = 0x00020000;
pub const ACPI_ALL_COMPONENTS: u32 = 0x0001FFFF;
pub const ACPI_COMPONENT_DEFAULT: u32 = ACPI_ALL_COMPONENTS;
pub const ACPI_ALL_DRIVERS: u32 = 0xFFFF0000;

/* Raw debug output levels */
pub const ACPI_LV_INIT: u32 = 0x00000001;
pub const ACPI_LV_DEBUG_OBJECT: u32 = 0x00000002;
pub const ACPI_LV_INFO: u32 = 0x00000004;
pub const ACPI_LV_REPAIR: u32 = 0x00000008;
pub const ACPI_LV_TRACE_POINT: u32 = 0x00000010;
pub const ACPI_LV_ALL_EXCEPTIONS: u32 = 0x0000001F;
pub const ACPI_LV_INIT_NAMES: u32 = 0x00000020;
pub const ACPI_LV_PARSE: u32 = 0x00000040;
pub const ACPI_LV_LOAD: u32 = 0x00000080;
pub const ACPI_LV_DISPATCH: u32 = 0x00000100;
pub const ACPI_LV_EXEC: u32 = 0x00000200;
pub const ACPI_LV_NAMES: u32 = 0x00000400;
pub const ACPI_LV_OPREGION: u32 = 0x00000800;
pub const ACPI_LV_BFIELD: u32 = 0x00001000;
pub const ACPI_LV_TABLES: u32 = 0x00002000;
pub const ACPI_LV_VALUES: u32 = 0x00004000;
pub const ACPI_LV_OBJECTS: u32 = 0x00008000;
pub const ACPI_LV_RESOURCES: u32 = 0x00010000;
pub const ACPI_LV_USER_REQUESTS: u32 = 0x00020000;
pub const ACPI_LV_PACKAGE: u32 = 0x00040000;
pub const ACPI_LV_EVALUATION: u32 = 0x00080000;
pub const ACPI_LV_VERBOSITY1: u32 = 0x000FFF40 | ACPI_LV_ALL_EXCEPTIONS;
pub const ACPI_LV_ALLOCATIONS: u32 = 0x00100000;
pub const ACPI_LV_FUNCTIONS: u32 = 0x00200000;
pub const ACPI_LV_OPTIMIZATIONS: u32 = 0x00400000;
pub const ACPI_LV_PARSE_TREES: u32 = 0x00800000;
pub const ACPI_LV_VERBOSITY2: u32 = 0x00F00000 | ACPI_LV_VERBOSITY1;
pub const ACPI_LV_ALL: u32 = ACPI_LV_VERBOSITY2;
pub const ACPI_LV_MUTEX: u32 = 0x01000000;
pub const ACPI_LV_THREADS: u32 = 0x02000000;
pub const ACPI_LV_IO: u32 = 0x04000000;
pub const ACPI_LV_INTERRUPTS: u32 = 0x08000000;
pub const ACPI_LV_VERBOSITY3: u32 = 0x0F000000 | ACPI_LV_VERBOSITY2;
pub const ACPI_LV_AML_DISASSEMBLE: u32 = 0x10000000;
pub const ACPI_LV_VERBOSE_INFO: u32 = 0x20000000;
pub const ACPI_LV_FULL_TABLES: u32 = 0x40000000;
pub const ACPI_LV_EVENTS: u32 = 0x80000000;
pub const ACPI_LV_VERBOSE: u32 = 0xF0000000;

pub const ACPI_NORMAL_DEFAULT: u32 = ACPI_LV_INIT | ACPI_LV_DEBUG_OBJECT | ACPI_LV_REPAIR;
pub const ACPI_DEBUG_ALL: u32 = ACPI_LV_AML_DISASSEMBLE | ACPI_LV_ALL_EXCEPTIONS | ACPI_LV_ALL;
pub const ACPI_TRACE_ENABLED: u32 = 4;
pub const ACPI_TRACE_ONESHOT: u32 = 2;
pub const ACPI_TRACE_OPCODE: u32 = 1;
pub const ACPI_TRACE_LEVEL_ALL: u32 = ACPI_LV_ALL;
pub const ACPI_TRACE_LAYER_ALL: u32 = 0x000001FF;
pub const ACPI_TRACE_LEVEL_DEFAULT: u32 = ACPI_LV_TRACE_POINT;
pub const ACPI_TRACE_LAYER_DEFAULT: u32 = ACPI_EXECUTER;

/* C macro interfaces retained as Rust macros. External symbols are supplied elsewhere. */
#[macro_export]
macro_rules! ACPI_DEBUG_LEVEL { ($dl:expr) => { (($dl) as u32) }; }
#[macro_export]
macro_rules! ACPI_DEBUG_PRINT { ($($arg:tt)*) => { acpi_debug_print!($($arg)*) }; }
#[macro_export]
macro_rules! ACPI_DEBUG_PRINT_RAW { ($($arg:tt)*) => { acpi_debug_print_raw!($($arg)*) }; }
#[macro_export]
macro_rules! ACPI_DEBUG_EXEC { ($($arg:tt)*) => { $($arg)* }; }
#[macro_export]
macro_rules! ACPI_DEBUG_ONLY_MEMBERS { ($($arg:tt)*) => { $($arg)* }; }
#[macro_export]
macro_rules! ACPI_FUNCTION_NAME { ($name:ident) => {}; }
#[macro_export]
macro_rules! ACPI_FUNCTION_TRACE { ($name:ident) => { ACPI_FUNCTION_NAME!($name); acpi_ut_trace!(); }; }
#[macro_export]
macro_rules! ACPI_FUNCTION_TRACE_PTR { ($name:ident, $pointer:expr) => { ACPI_FUNCTION_NAME!($name); acpi_ut_trace_ptr!($pointer); }; }
#[macro_export]
macro_rules! ACPI_FUNCTION_TRACE_U32 { ($name:ident, $value:expr) => { ACPI_FUNCTION_NAME!($name); acpi_ut_trace_u32!($value); }; }
#[macro_export]
macro_rules! ACPI_FUNCTION_TRACE_STR { ($name:ident, $string:expr) => { ACPI_FUNCTION_NAME!($name); acpi_ut_trace_str!($string); }; }
#[macro_export]
macro_rules! ACPI_FUNCTION_ENTRY { () => { acpi_ut_track_stack_ptr!(); }; }
#[macro_export]
macro_rules! ACPI_IS_DEBUG_ENABLED { ($level:expr, $component:expr) => { (($level & acpi_dbg_level) != 0 && ($component & acpi_dbg_layer) != 0) }; }
#[macro_export]
macro_rules! ACPI_DUMP_STACK_ENTRY { ($a:expr) => { acpi_ex_dump_operand!($a, 0); }; }
#[macro_export]
macro_rules! ACPI_DUMP_OPERANDS { ($a:expr, $b:expr, $c:expr) => { acpi_ex_dump_operands!($a, $b, $c); }; }
#[macro_export]
macro_rules! ACPI_DUMP_ENTRY { ($a:expr, $b:expr) => { acpi_ns_dump_entry!($a, $b); }; }
#[macro_export]
macro_rules! ACPI_DUMP_PATHNAME { ($a:expr, $b:expr, $c:expr, $d:expr) => { acpi_ns_dump_pathname!($a, $b, $c, $d); }; }
#[macro_export]
macro_rules! ACPI_TRACE_POINT { ($a:expr, $b:expr, $c:expr, $d:expr) => { acpi_trace_point!($a, $b, $c, $d); }; }
#[macro_export]
macro_rules! return_VOID { () => { return; }; }
#[macro_export]
macro_rules! return_ACPI_STATUS { ($s:expr) => { return $s; }; }
#[macro_export]
macro_rules! return_PTR { ($s:expr) => { return $s; }; }
#[macro_export]
macro_rules! return_STR { ($s:expr) => { return $s; }; }
#[macro_export]
macro_rules! return_VALUE { ($s:expr) => { return $s; }; }
#[macro_export]
macro_rules! return_UINT8 { ($s:expr) => { return $s; }; }
#[macro_export]
macro_rules! return_UINT32 { ($s:expr) => { return $s; }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
