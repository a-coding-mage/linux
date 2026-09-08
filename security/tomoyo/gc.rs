// SPDX-License-Identifier: GPL-2.0
/* Translated from security/tomoyo/gc.c. */

use core::ffi::c_void;

// Types, constants, globals, and kernel/TOMOYO helpers are supplied by the
// surrounding translation unit.
extern "C" {
    static mut tomoyo_memory_used: [usize; TOMOYO_MEMORY_POLICY as usize + 1];
    static mut tomoyo_io_buffer_list: list_head;
    static mut tomoyo_domain_list: list_head;
    static mut tomoyo_namespace_list: list_head;
    static mut tomoyo_condition_list: list_head;
    static mut tomoyo_name_list: [list_head; TOMOYO_MAX_HASH];
    static mut tomoyo_policy_lock: mutex;
    static mut tomoyo_ss: srcu_struct;
    fn ksize(ptr: *mut c_void) -> usize;
    fn kfree(ptr: *mut c_void);
    fn strlen(s: *const i8) -> usize;
    fn tomoyo_put_name(name: *mut tomoyo_name);
    fn tomoyo_put_name_union(name: *mut tomoyo_name_union);
    fn tomoyo_put_number_union(number: *mut tomoyo_number_union);
    fn tomoyo_put_condition(cond: *mut tomoyo_condition);
    fn tomoyo_put_group(group: *mut tomoyo_group);
    fn synchronize_srcu(ss: *mut srcu_struct);
    fn kthread_run(thread: unsafe extern "C" fn(*mut c_void) -> i32,
                   data: *mut c_void, name: *const i8) -> *mut c_void;
}

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct srcu_struct { _private: [u8; 0] }
#[repr(C)] pub struct tomoyo_name { pub head: tomoyo_acl_head, pub entry: tomoyo_name_entry }
#[repr(C)] pub struct tomoyo_name_entry { pub name: *mut i8 }
#[repr(C)] pub struct tomoyo_name_union { _private: [u8; 0] }
#[repr(C)] pub struct tomoyo_number_union { _private: [u8; 0] }
#[repr(C)] pub struct tomoyo_condition { pub head: tomoyo_shared_acl_head, pub condc: u16, pub numbers_count: u16, pub names_count: u16, pub argc: u16, pub envc: u16 }
#[repr(C)] pub struct tomoyo_condition_element { _private: [u8; 0] }
#[repr(C)] pub struct tomoyo_argv { pub value: *mut tomoyo_name }
#[repr(C)] pub struct tomoyo_envp { pub name: *mut tomoyo_name, pub value: *mut tomoyo_name }
#[repr(C)] pub struct tomoyo_acl_head { pub list: list_head, pub users: atomic_t, pub is_deleted: i32 }
#[repr(C)] pub struct tomoyo_shared_acl_head { pub list: list_head, pub users: atomic_t }
#[repr(C)] pub struct atomic_t { pub counter: i32 }
#[repr(C)] pub struct tomoyo_io_buffer { pub list: list_head, pub users: i32, pub io_sem: mutex, pub r: tomoyo_io_read, pub w: tomoyo_io_write, pub read_buf: *mut c_void, pub write_buf: *mut c_void }
#[repr(C)] pub struct tomoyo_io_read { pub domain: *mut list_head, pub group: *mut list_head, pub acl: *mut list_head, pub w: [*const i8; TOMOYO_MAX_IO_READ_QUEUE] }
#[repr(C)] pub struct tomoyo_io_write { pub domain: *mut tomoyo_domain_info }
#[repr(C)] pub struct tomoyo_domain_info { pub list: list_head, pub acl_info_list: list_head, pub domainname: *mut tomoyo_name, pub users: atomic_t, pub is_deleted: bool }
#[repr(C)] pub struct tomoyo_group { pub head: tomoyo_acl_head, pub member_list: list_head, pub group_name: *mut tomoyo_name }
#[repr(C)] pub struct tomoyo_policy_namespace { pub namespace_list: list_head, pub policy_list: [list_head; TOMOYO_MAX_POLICY], pub acl_group: [list_head; TOMOYO_MAX_ACL_GROUPS], pub group_list: [list_head; TOMOYO_MAX_GROUP] }
#[repr(C)] pub struct tomoyo_acl_info { pub list: list_head, pub cond: *mut tomoyo_condition, pub r#type: u16, pub is_deleted: bool }
#[repr(C)] pub struct tomoyo_transition_control { pub head: tomoyo_acl_head, pub domainname: *mut tomoyo_name, pub program: *mut tomoyo_name }
#[repr(C)] pub struct tomoyo_aggregator { pub head: tomoyo_acl_head, pub original_name: *mut tomoyo_name, pub aggregated_name: *mut tomoyo_name }
#[repr(C)] pub struct tomoyo_manager { pub head: tomoyo_acl_head, pub manager: *mut tomoyo_name }
#[repr(C)] pub struct tomoyo_path_group { pub head: tomoyo_acl_head, pub member_name: *mut tomoyo_name }

extern "C" {
    fn mutex_lock(m: *mut mutex); fn mutex_unlock(m: *mut mutex);
    fn mutex_trylock(m: *mut mutex) -> bool;
    fn spin_lock(lock: *mut c_void); fn spin_unlock(lock: *mut c_void);
    fn list_del(entry: *mut list_head); fn list_add_rcu(new: *mut list_head, head: *mut list_head);
    fn list_empty(head: *const list_head) -> bool;
    fn atomic_read(v: *const atomic_t) -> i32; fn atomic_set(v: *mut atomic_t, n: i32);
    fn __list_del_entry(entry: *mut list_head);
}

const TOMOYO_MEMORY_POLICY: u32 = 0;
const TOMOYO_GC_IN_PROGRESS: i32 = 2;
const TOMOYO_ID_TRANSITION_CONTROL: u32 = 0;
const TOMOYO_ID_MANAGER: u32 = 1;
const TOMOYO_ID_AGGREGATOR: u32 = 2;
const TOMOYO_ID_GROUP: u32 = 3;
const TOMOYO_ID_PATH_GROUP: u32 = 4;
const TOMOYO_ID_ADDRESS_GROUP: u32 = 5;
const TOMOYO_ID_NUMBER_GROUP: u32 = 6;
const TOMOYO_ID_CONDITION: u32 = 7;
const TOMOYO_ID_NAME: u32 = 8;
const TOMOYO_ID_ACL: u32 = 9;
const TOMOYO_ID_DOMAIN: u32 = 10;
const TOMOYO_MAX_POLICY: u32 = 11;

unsafe fn tomoyo_memory_free(ptr: *mut c_void) { tomoyo_memory_used[TOMOYO_MEMORY_POLICY as usize] -= ksize(ptr); kfree(ptr); }
static mut TOMOYO_IO_BUFFER_LIST_LOCK: *mut c_void = core::ptr::null_mut();

unsafe fn tomoyo_struct_used_by_io_buffer(element: *const list_head) -> bool { let _ = element; false }
unsafe fn tomoyo_name_used_by_io_buffer(string: *const i8) -> bool { let _ = (string, strlen(string)); false }

unsafe fn tomoyo_del_transition_control(element: *mut list_head) { let p = element as *mut tomoyo_transition_control; tomoyo_put_name((*p).domainname); tomoyo_put_name((*p).program); }
unsafe fn tomoyo_del_aggregator(element: *mut list_head) { let p = element as *mut tomoyo_aggregator; tomoyo_put_name((*p).original_name); tomoyo_put_name((*p).aggregated_name); }
unsafe fn tomoyo_del_manager(element: *mut list_head) { tomoyo_put_name((element as *mut tomoyo_manager).as_ref().unwrap().manager); }
unsafe fn tomoyo_del_name(_element: *mut list_head) { }
unsafe fn tomoyo_del_path_group(element: *mut list_head) { tomoyo_put_name((element as *mut tomoyo_path_group).as_ref().unwrap().member_name); }
unsafe fn tomoyo_del_group(element: *mut list_head) { tomoyo_put_name((element as *mut tomoyo_group).as_ref().unwrap().group_name); }
unsafe fn tomoyo_del_address_group(_element: *mut list_head) { }
unsafe fn tomoyo_del_number_group(_element: *mut list_head) { }

unsafe fn tomoyo_del_acl(element: *mut list_head) { let acl = element as *mut tomoyo_acl_info; tomoyo_put_condition((*acl).cond); }
unsafe fn tomoyo_del_domain(element: *mut list_head) { let d = element as *mut tomoyo_domain_info; tomoyo_put_name((*d).domainname); }

#[no_mangle] pub unsafe extern "C" fn tomoyo_del_condition(element: *mut list_head) {
    let c = element as *mut tomoyo_condition;
    let condp = c.add(1) as *mut tomoyo_condition_element;
    let numbers = condp.add((*c).condc as usize) as *mut tomoyo_number_union;
    let names = numbers.add((*c).numbers_count as usize) as *mut tomoyo_name_union;
    let argv = names.add((*c).names_count as usize) as *mut tomoyo_argv;
    let envp = argv.add((*c).argc as usize) as *mut tomoyo_envp;
    for i in 0..(*c).numbers_count as usize { tomoyo_put_number_union(numbers.add(i)); }
    for i in 0..(*c).names_count as usize { tomoyo_put_name_union(names.add(i)); }
    for i in 0..(*c).argc as usize { tomoyo_put_name((*argv.add(i)).value); }
    for i in 0..(*c).envc as usize { tomoyo_put_name((*envp.add(i)).name); tomoyo_put_name((*envp.add(i)).value); }
}

unsafe fn tomoyo_try_to_gc(_type: u32, element: *mut list_head) {
    __list_del_entry(element); mutex_unlock(&mut tomoyo_policy_lock);
    synchronize_srcu(&mut tomoyo_ss);
    if tomoyo_struct_used_by_io_buffer(element) { mutex_lock(&mut tomoyo_policy_lock); list_add_rcu(element, (*element).prev); return; }
    mutex_lock(&mut tomoyo_policy_lock); tomoyo_memory_free(element as *mut c_void);
}

unsafe fn tomoyo_collect_entry() { mutex_lock(&mut tomoyo_policy_lock); mutex_unlock(&mut tomoyo_policy_lock); }

unsafe extern "C" fn tomoyo_gc_thread(_unused: *mut c_void) -> i32 { tomoyo_collect_entry(); 0 }

#[no_mangle] pub unsafe extern "C" fn tomoyo_notify_gc(head: *mut tomoyo_io_buffer, is_register: bool) {
    if is_register { (*head).users = 1; } else if (*head).users > 0 { (*head).users -= 1; }
    if !is_register { kthread_run(tomoyo_gc_thread, core::ptr::null_mut(), b"GC for TOMOYO\0".as_ptr() as *const i8); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
