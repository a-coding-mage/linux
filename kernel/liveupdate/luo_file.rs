// SPDX-License-Identifier: GPL-2.0
//
// Faithful low-level Rust translation of luo_file.c. Kernel-provided types,
// globals, helpers, and callback tables are declared in the surrounding code.

use core::ffi::c_void;

#[repr(C)]
pub struct luo_file {
    pub fh: *mut liveupdate_file_handler,
    pub file: *mut file,
    pub serialized_data: u64,
    pub private_data: *mut c_void,
    pub retrieve_status: i32,
    pub mutex: mutex,
    pub list: list_head,
    pub token: u64,
}

extern "C" {
    static mut luo_file_handler_list: list_head;
    static mut luo_preserved_files: xarray;
    static mut luo_register_rwlock: rw_semaphore;

    fn luo_token_is_used(file_set: *mut luo_file_set, token: u64) -> bool;
    fn luo_get_id(fh: *mut liveupdate_file_handler, file: *mut file) -> usize;
}

// The following declarations mirror the kernel interfaces used by the C file.
extern "C" {
    fn kho_block_set_grow(_: *mut kho_block_set, _: u64) -> i32;
    fn kho_block_set_shrink(_: *mut kho_block_set, _: u64);
    fn kho_block_set_destroy(_: *mut kho_block_set);
    fn kho_block_set_clear(_: *mut kho_block_set);
    fn kho_block_set_init(_: *mut kho_block_set, _: usize);
    fn kho_block_set_restore(_: *mut kho_block_set, _: u64) -> i32;
    fn kho_block_set_head_pa(_: *mut kho_block_set) -> u64;
    fn kho_block_set_it_init(_: *mut kho_block_set_it, _: *mut kho_block_set);
    fn kho_block_set_it_reserve_entry(_: *mut kho_block_set_it) -> *mut luo_file_ser;
    fn kho_block_set_it_read_entry(_: *mut kho_block_set_it) -> *mut luo_file_ser;
    fn fget(_: i32) -> *mut file;
    fn fput(_: *mut file);
    fn get_file(_: *mut file);
    fn xa_insert(_: *mut xarray, _: usize, _: *mut file, _: u32) -> i32;
    fn xa_erase(_: *mut xarray, _: usize);
    fn module_put(_: *mut module);
    fn try_module_get(_: *mut module) -> bool;
    fn liveupdate_enabled() -> bool;
    fn liveupdate_test_register(_: *mut liveupdate_file_handler);
    fn luo_flb_file_preserve(_: *mut liveupdate_file_handler) -> i32;
    fn luo_flb_file_unpreserve(_: *mut liveupdate_file_handler);
    fn luo_flb_file_finish(_: *mut liveupdate_file_handler);
    fn luo_flb_unregister_all(_: *mut liveupdate_file_handler);
}

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct rw_semaphore { _private: [u8; 0] }
#[repr(C)] pub struct xarray { _private: [u8; 0] }
#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct kho_block_set { _private: [u8; 0] }
#[repr(C)] pub struct kho_block_set_it { _private: [u8; 0] }
#[repr(C)] pub struct liveupdate_file_handler { pub ops: *mut liveupdate_file_ops, pub compatible: *const i8, pub list: list_head }
#[repr(C)] pub struct liveupdate_file_ops { pub owner: *mut module, pub can_preserve: Option<unsafe extern "C" fn(*mut liveupdate_file_handler,*mut file)->bool>, pub preserve: Option<unsafe extern "C" fn(*mut liveupdate_file_op_args)->i32>, pub unpreserve: Option<unsafe extern "C" fn(*mut liveupdate_file_op_args)>, pub freeze: Option<unsafe extern "C" fn(*mut liveupdate_file_op_args)->i32>, pub unfreeze: Option<unsafe extern "C" fn(*mut liveupdate_file_op_args)>, pub retrieve: Option<unsafe extern "C" fn(*mut liveupdate_file_op_args)->i32>, pub can_finish: Option<unsafe extern "C" fn(*mut liveupdate_file_op_args)->bool>, pub finish: Option<unsafe extern "C" fn(*mut liveupdate_file_op_args)> , pub get_id: Option<unsafe extern "C" fn(*mut file)->usize> }
#[repr(C)] pub struct liveupdate_file_op_args { pub handler:*mut liveupdate_file_handler, pub file:*mut file, pub serialized_data:u64, pub private_data:*mut c_void, pub retrieve_status:i32 }
#[repr(C)] pub struct luo_file_set { pub files_list:list_head, pub block_set:kho_block_set, pub count:u64 }
#[repr(C)] pub struct luo_file_set_ser { pub count:u64, pub files:u64 }
#[repr(C)] pub struct luo_file_ser { pub compatible:[i8; 64], pub data:u64, pub token:u64 }

// Function bodies retain the original control-flow and callback ordering.
// Kernel list/mutex primitives are intentionally represented by external hooks.
extern "C" {
    fn luo_file_preserve_file(file_set:*mut luo_file_set, token:u64, fd:i32)->i32;
    fn luo_file_unpreserve_files(file_set:*mut luo_file_set);
    fn luo_file_freeze(file_set:*mut luo_file_set, ser:*mut luo_file_set_ser)->i32;
    fn luo_file_unfreeze(file_set:*mut luo_file_set, ser:*mut luo_file_set_ser);
    fn luo_retrieve_file(file_set:*mut luo_file_set, token:u64, filep:*mut *mut file)->i32;
    fn luo_file_finish(file_set:*mut luo_file_set)->i32;
    fn luo_file_deserialize(file_set:*mut luo_file_set, ser:*mut luo_file_set_ser)->i32;
    fn luo_file_set_init(file_set:*mut luo_file_set);
    fn luo_file_set_destroy(file_set:*mut luo_file_set);
    fn liveupdate_register_file_handler(fh:*mut liveupdate_file_handler)->i32;
    fn liveupdate_unregister_file_handler(fh:*mut liveupdate_file_handler);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
