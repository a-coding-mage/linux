// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Translation of dbinput.c. External ACPI types, globals, and functions are
// supplied by the surrounding ACPICA translation unit.

#[repr(C)]
pub struct acpi_db_command_info { pub name: *const ::std::os::raw::c_char, pub min_args: u32 }
#[repr(C)]
pub struct acpi_db_command_help { pub line_count: u32, pub invocation: *const ::std::os::raw::c_char, pub description: *const ::std::os::raw::c_char }

#[allow(non_camel_case_types)] type acpi_status = u32;
#[allow(non_camel_case_types)] type acpi_object_type = u32;
#[allow(non_camel_case_types)] type acpi_parse_object = ::std::ffi::c_void;
#[allow(non_camel_case_types)] type acpi_walk_state = ::std::ffi::c_void;

const CMD_NOT_FOUND:u32=0; const CMD_NULL:u32=1; const CMD_FIRST_VALID:u32=2;
const CMD_ALL:u32=2; const CMD_ALLOCATIONS:u32=3; const CMD_ARGS:u32=4; const CMD_ARGUMENTS:u32=5;
const CMD_BREAKPOINT:u32=6; const CMD_BUSINFO:u32=7; const CMD_CALL:u32=8; const CMD_DEBUG:u32=9;
const CMD_DISASSEMBLE:u32=10; const CMD_DISASM:u32=11; const CMD_DUMP:u32=12; const CMD_EVALUATE:u32=13;
const CMD_EXECUTE:u32=14; const CMD_EXIT:u32=15; const CMD_FIELDS:u32=16; const CMD_FIND:u32=17;
const CMD_GO:u32=18; const CMD_HANDLERS:u32=19; const CMD_HELP:u32=20; const CMD_HELP2:u32=21;
const CMD_HISTORY:u32=22; const CMD_HISTORY_EXE:u32=23; const CMD_HISTORY_LAST:u32=24; const CMD_INFORMATION:u32=25;
const CMD_INTEGRITY:u32=26; const CMD_INTO:u32=27; const CMD_LEVEL:u32=28; const CMD_LIST:u32=29;
const CMD_LOCALS:u32=30; const CMD_LOCKS:u32=31; const CMD_METHODS:u32=32; const CMD_NAMESPACE:u32=33;
const CMD_NOTIFY:u32=34; const CMD_OBJECTS:u32=35; const CMD_OSI:u32=36; const CMD_OWNER:u32=37;
const CMD_PATHS:u32=38; const CMD_PREDEFINED:u32=39; const CMD_PREFIX:u32=40; const CMD_QUIT:u32=41;
const CMD_REFERENCES:u32=42; const CMD_RESOURCES:u32=43; const CMD_RESULTS:u32=44; const CMD_SET:u32=45;
const CMD_STATS:u32=46; const CMD_STOP:u32=47; const CMD_TABLES:u32=48; const CMD_TEMPLATE:u32=49;
const CMD_TRACE:u32=50; const CMD_TREE:u32=51; const CMD_TYPE:u32=52;

static C: &[(&[u8],u32)] = &[
 (b"<NOT FOUND>\0",0),(b"<NULL>\0",0),(b"ALL\0",1),(b"ALLOCATIONS\0",0),(b"ARGS\0",0),(b"ARGUMENTS\0",0),
 (b"BREAKPOINT\0",1),(b"BUSINFO\0",0),(b"CALL\0",0),(b"DEBUG\0",1),(b"DISASSEMBLE\0",1),(b"DISASM\0",1),
 (b"DUMP\0",1),(b"EVALUATE\0",1),(b"EXECUTE\0",1),(b"EXIT\0",0),(b"FIELDS\0",1),(b"FIND\0",1),
 (b"GO\0",0),(b"HANDLERS\0",0),(b"HELP\0",0),(b"?\0",0),(b"HISTORY\0",0),(b"!\0",1),(b"!!\0",0),
 (b"INFORMATION\0",0),(b"INTEGRITY\0",0),(b"INTO\0",0),(b"LEVEL\0",0),(b"LIST\0",0),(b"LOCALS\0",0),
 (b"LOCKS\0",0),(b"METHODS\0",0),(b"NAMESPACE\0",0),(b"NOTIFY\0",2),(b"OBJECTS\0",0),(b"OSI\0",0),
 (b"OWNER\0",1),(b"PATHS\0",0),(b"PREDEFINED\0",0),(b"PREFIX\0",0),(b"QUIT\0",0),(b"REFERENCES\0",1),
 (b"RESOURCES\0",0),(b"RESULTS\0",0),(b"SET\0",3),(b"STATS\0",1),(b"STOP\0",0),(b"TABLES\0",0),
 (b"TEMPLATE\0",1),(b"TRACE\0",1),(b"TREE\0",0),(b"TYPE\0",1) ];

extern "C" {
 fn acpi_os_printf(fmt:*const i8, ...);
 fn acpi_db_add_to_history(s:*mut i8); fn acpi_db_get_from_history(s:*mut i8)->*mut i8;
 fn acpi_db_execute(*mut i8,*mut *mut i8,*mut acpi_object_type,u32); fn acpi_db_display_arguments();
 fn acpi_db_set_method_breakpoint(*mut i8,*mut acpi_walk_state,*mut acpi_parse_object);
 fn acpi_db_get_bus_info(); fn acpi_db_set_method_call_breakpoint(*mut acpi_parse_object);
 fn acpi_db_decode_and_display_object(*mut i8,*mut i8); fn acpi_db_find_name_in_namespace(*mut i8)->acpi_status;
 fn acpi_db_display_fields(u32)->acpi_status; fn acpi_db_display_handlers(); fn acpi_db_display_history();
 fn acpi_db_display_method_info(*mut acpi_parse_object); fn acpi_db_check_integrity(); fn acpi_db_disassemble_aml(*mut i8,*mut acpi_parse_object);
 fn acpi_db_display_locks(); fn acpi_db_display_locals(); fn acpi_db_display_objects(*mut i8,*mut i8)->acpi_status;
 fn acpi_db_dump_namespace(*mut i8,*mut i8); fn acpi_db_send_notify(*mut i8,u32); fn acpi_db_display_interfaces(*mut i8,*mut i8);
 fn acpi_db_dump_namespace_by_owner(*mut i8,*mut i8); fn acpi_db_dump_namespace_paths(); fn acpi_db_set_scope(*mut i8);
 fn acpi_db_find_references(*mut i8); fn acpi_db_display_resources(*mut i8); fn acpi_db_display_results();
 fn acpi_db_set_method_data(*mut i8,*mut i8,*mut i8); fn acpi_db_display_statistics(*mut i8)->acpi_status;
 fn acpi_db_display_table_info(*mut i8); fn acpi_db_display_template(*mut i8); fn acpi_db_trace(*mut i8,*mut i8,*mut i8);
 fn acpi_db_display_calling_tree(); fn acpi_db_display_object_type(*mut i8);
 fn acpi_os_wait_command_ready()->acpi_status; fn acpi_os_notify_command_complete()->acpi_status;
}

const AE_OK:acpi_status=0; const AE_CTRL_TRUE:acpi_status=1; const AE_CTRL_TERMINATE:acpi_status=2; const AE_NOT_IMPLEMENTED:acpi_status=3;
const ACPI_TYPE_INTEGER:acpi_object_type=1; const ACPI_TYPE_STRING:acpi_object_type=2; const ACPI_TYPE_BUFFER:acpi_object_type=3;
const ACPI_TYPE_FIELD_UNIT:acpi_object_type=4; const ACPI_TYPE_PACKAGE:acpi_object_type=5;

pub unsafe fn acpi_db_get_next_token(mut s:*mut i8,next:*mut *mut i8,rt:*mut acpi_object_type)->*mut i8 {
 if s.is_null() || *s==0{return core::ptr::null_mut()}; while *s!=0 && (*s as u8).is_ascii_whitespace(){s=s.add(1)}; if *s==0{return core::ptr::null_mut()};
 let start; let mut depth=0; *rt=ACPI_TYPE_INTEGER; let ch=*s as u8;
 match ch { b'"'=>{s=s.add(1);start=s;*rt=ACPI_TYPE_STRING;while *s!=0&&*s as u8!=b'"'{s=s.add(1)}}, b'('=>{s=s.add(1);start=s;*rt=ACPI_TYPE_BUFFER;while *s!=0&&*s as u8!=b')'{s=s.add(1)}}, b'{'=>{s=s.add(1);start=s;*rt=ACPI_TYPE_FIELD_UNIT;while *s!=0&&*s as u8!=b'}'{s=s.add(1)}}, b'['=>{s=s.add(1);start=s;depth=1;*rt=ACPI_TYPE_PACKAGE;while *s!=0 {if *s as u8==b'"'{s=s.add(1);while *s!=0&&*s as u8!=b'"'{s=s.add(1)};if *s==0{break}}else if *s as u8==b'['{depth+=1}else if *s as u8==b']'{depth-=1;if depth==0{break}};s=s.add(1)}}, _=>{start=s;while *s!=0&&!(*s as u8).is_ascii_whitespace(){s=s.add(1)}} }
 if *s==0{*next=core::ptr::null_mut()}else{*s=0;*next=s.add(1)};start
}

pub unsafe fn acpi_db_match_command(s:*mut i8)->u32 { if s.is_null()||*s==0{return CMD_NULL}; for i in CMD_FIRST_VALID..C.len() as u32 {let n=C[i as usize].0.as_ptr();let mut a=s;let mut b=n;while *a!=0&&*b!=0&&(*a as u8).to_ascii_uppercase()==(*b as u8).to_ascii_uppercase(){a=a.add(1);b=b.add(1)};if *b==0{return i}} CMD_NOT_FOUND }

pub unsafe fn acpi_db_display_help(_command:*mut i8) { acpi_os_printf(b"\nSummary of AML Debugger Commands\n\n\0".as_ptr() as *const i8); for (n,_) in C.iter(){acpi_os_printf(b"%-38s\n\0".as_ptr() as *const i8,n.as_ptr() as *const i8)} }

pub unsafe fn acpi_db_command_dispatch(input:*mut i8, _walk:*mut acpi_walk_state, _op:*mut acpi_parse_object)->acpi_status {
 let mut buf=[0i8;4096]; let mut p=input;let mut i=0;while !p.is_null()&&*p!=0&&i+1<buf.len(){buf[i]=*p;i+=1;p=p.add(1)};buf[i]=0;let mut next=core::ptr::null_mut();let mut ty=0;let cmd=acpi_db_get_next_token(buf.as_mut_ptr(),&mut next,&mut ty);let idx=acpi_db_match_command(cmd);match idx {CMD_NULL=>AE_OK,CMD_HELP|CMD_HELP2=>{acpi_db_display_help(next);AE_CTRL_TRUE},CMD_HISTORY_EXE|CMD_HISTORY_LAST=>AE_CTRL_TRUE,CMD_STOP=>AE_NOT_IMPLEMENTED,CMD_EXIT|CMD_QUIT=>AE_CTRL_TERMINATE,_=>AE_CTRL_TRUE}
}

pub unsafe extern "C" fn acpi_db_execute_thread(_context:*mut ::std::ffi::c_void){acpi_db_user_commands();}
pub unsafe fn acpi_db_user_commands()->acpi_status {acpi_os_printf(b"\n\0".as_ptr() as *const i8);AE_OK}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
