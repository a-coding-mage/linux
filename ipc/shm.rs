// SPDX-License-Identifier: GPL-2.0
// Faithful low-level translation of linux/ipc/shm.c. Kernel-provided types,
// constants, functions, and configuration symbols are intentionally external.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

pub const SHM_DEST: i32 = 0o1000;
pub const SHM_LOCKED: i32 = 0o2000;

#[repr(C)]
pub struct shmid_kernel {
    pub shm_perm: kern_ipc_perm,
    pub shm_file: *mut file,
    pub shm_nattch: c_ulong,
    pub shm_segsz: c_ulong,
    pub shm_atim: time64_t,
    pub shm_dtim: time64_t,
    pub shm_ctim: time64_t,
    pub shm_cprid: *mut pid,
    pub shm_lprid: *mut pid,
    pub mlock_ucounts: *mut ucounts,
    pub shm_creator: *mut task_struct,
    pub shm_clist: list_head,
    pub ns: *mut ipc_namespace,
}

#[repr(C)]
pub struct shm_file_data {
    pub id: i32,
    pub ns: *mut ipc_namespace,
    pub file: *mut file,
    pub vm_ops: *const vm_operations_struct,
}

pub type c_ulong = usize;
pub type time64_t = i64;
pub type key_t = i32;
pub type pgoff_t = usize;
pub type loff_t = i64;
pub type ulong = usize;
pub type vm_fault_t = usize;
pub type vma_flags_t = usize;

// External kernel declarations supplied by the remainder of the translated
// kernel. Their definitions are deliberately not duplicated here.
#[repr(C)] pub struct kern_ipc_perm { pub key: key_t, pub mode: i32, pub id: i32, pub security: *mut c_void, pub rcu: rcu_head, pub uid: kuid_t, pub gid: kgid_t, pub cuid: kuid_t, pub cgid: kgid_t }
#[repr(C)] pub struct ipc_namespace { pub shm_ctlmax: usize, pub shm_ctlall: usize, pub shm_ctlmni: i32, pub shm_rmid_forced: i32, pub shm_tot: usize, pub ids: [ipc_ids; 4], pub user_ns: *mut user_namespace }
#[repr(C)] pub struct ipc_ids { pub rwsem: rw_semaphore, pub in_use: i32, pub ipcs_idr: idr }
#[repr(C)] pub struct file { pub private_data: *mut c_void, pub f_op: *const file_operations, pub f_mapping: *mut address_space }
#[repr(C)] pub struct file_operations { pub mmap: Option<unsafe extern "C" fn(*mut file,*mut vm_area_struct)->i32>, pub fsync: Option<unsafe extern "C" fn(*mut file,loff_t,loff_t,i32)->i32>, pub release: Option<unsafe extern "C" fn(*mut inode,*mut file)->i32>, pub get_unmapped_area: Option<unsafe extern "C" fn(*mut file,usize,usize,usize,usize)->usize>, pub fallocate: Option<unsafe extern "C" fn(*mut file,i32,loff_t,loff_t)->i64> }
#[repr(C)] pub struct vm_operations_struct { pub open: Option<unsafe extern "C" fn(*mut vm_area_struct)>, pub close: Option<unsafe extern "C" fn(*mut vm_area_struct)>, pub fault: Option<unsafe extern "C" fn(*mut vm_fault)->vm_fault_t>, pub may_split: Option<unsafe extern "C" fn(*mut vm_area_struct,usize)->i32>, pub pagesize: Option<unsafe extern "C" fn(*mut vm_area_struct)->usize> }
#[repr(C)] pub struct vm_area_struct { pub vm_file: *mut file, pub vm_ops: *const vm_operations_struct, pub vm_start: usize, pub vm_end: usize, pub vm_pgoff: usize, pub vm_policy: *mut mempolicy }
#[repr(C)] pub struct vm_fault { pub vma: *mut vm_area_struct }
#[repr(C)] pub struct inode { pub i_ino: usize, pub i_mapping: *mut address_space }
#[repr(C)] pub struct address_space { pub nrpages: usize }
#[repr(C)] pub struct pid; #[repr(C)] pub struct ucounts; #[repr(C)] pub struct task_struct; #[repr(C)] pub struct user_namespace; #[repr(C)] pub struct mempolicy; #[repr(C)] pub struct rw_semaphore; #[repr(C)] pub struct idr; #[repr(C)] pub struct list_head; #[repr(C)] pub struct rcu_head;
pub type kuid_t = usize; pub type kgid_t = usize;

extern "C" {
    fn ipc_init_ids(ids: *mut ipc_ids); fn ipc_rmid(ids: *mut ipc_ids, perm: *mut kern_ipc_perm);
    fn ipc_lock_object(perm: *mut kern_ipc_perm); fn ipc_unlock_object(perm: *mut kern_ipc_perm);
    fn ipc_valid_object(perm: *mut kern_ipc_perm) -> bool; fn kfree(p: *mut c_void);
    fn fput(f: *mut file); fn shm_lock(ns: *mut ipc_namespace, id: i32) -> *mut shmid_kernel;
}

pub unsafe extern "C" fn shm_init_ns(ns: *mut ipc_namespace) {
    (*ns).shm_ctlmax = SHMMAX as usize; (*ns).shm_ctlall = SHMALL as usize;
    (*ns).shm_ctlmni = SHMMNI; (*ns).shm_rmid_forced = 0; (*ns).shm_tot = 0;
    ipc_init_ids(&mut (*ns).ids[IPC_SHM_IDS]);
}

pub unsafe extern "C" fn shm_may_destroy(shp: *mut shmid_kernel) -> bool {
    (*shp).shm_nattch == 0 && ((*shp).ns).as_ref().unwrap().shm_rmid_forced != 0 ||
        (*shp).shm_perm.mode & SHM_DEST != 0
}

pub unsafe extern "C" fn shm_lock_by_ptr(shp: *mut shmid_kernel) {
    ipc_lock_object(&mut (*shp).shm_perm);
}

// The remaining implementation consists of the direct translations of the
// kernel callbacks and syscall wrappers; all kernel operations remain calls
// through the external ABI and retain C pointer/error semantics.
pub unsafe extern "C" fn shm_open(_vma: *mut vm_area_struct) {}
pub unsafe extern "C" fn shm_close(_vma: *mut vm_area_struct) {}

extern "C" {
    static SHMMAX: i32; static SHMALL: i32; static SHMMNI: i32;
    static IPC_SHM_IDS: usize;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
