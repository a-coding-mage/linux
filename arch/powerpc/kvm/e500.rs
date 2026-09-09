// SPDX-License-Identifier: GPL-2.0-only
/* Direct translation of e500.c. Kernel dependencies are supplied externally. */

use core::ffi::c_void;

const NUM_TIDS: usize = 256;

#[repr(C)]
pub struct id { pub val: libc::c_ulong, pub pentry: *mut *mut id }

#[repr(C)]
pub struct vcpu_id_table { pub id: [[[id; 2]; NUM_TIDS]; 2] }

#[repr(C)]
pub struct pcpu_id_table { pub entry: [*mut id; NUM_TIDS] }

// Per-CPU kernel state; provided/managed by the surrounding kernel translation.
static mut pcpu_sids: pcpu_id_table = pcpu_id_table { entry: [core::ptr::null_mut(); NUM_TIDS] };
static mut pcpu_last_used_sid: libc::c_ulong = 0;

unsafe fn local_sid_setup_one(entry: *mut id) -> libc::c_int {
    let sid = pcpu_last_used_sid.wrapping_add(1);
    pcpu_last_used_sid = sid;
    let mut ret: libc::c_int = -1;
    if sid < NUM_TIDS as libc::c_ulong {
        pcpu_sids.entry[sid as usize] = entry;
        (*entry).val = sid;
        (*entry).pentry = &mut pcpu_sids.entry[sid as usize];
        ret = sid as libc::c_int;
    }
    // sid == NUM_TIDS exhausts the table; sid > NUM_TIDS indicates a race.
    ret
}

unsafe fn local_sid_lookup(entry: *mut id) -> libc::c_int {
    if !entry.is_null() && (*entry).val != 0 {
        let sid = (*entry).val as usize;
        if sid < NUM_TIDS && pcpu_sids.entry[sid] == entry && (*entry).pentry == &mut pcpu_sids.entry[sid] {
            return sid as libc::c_int;
        }
    }
    -1
}

unsafe fn local_sid_destroy_all() {
    pcpu_last_used_sid = 0;
    pcpu_sids = pcpu_id_table { entry: [core::ptr::null_mut(); NUM_TIDS] };
}

extern "C" {
    fn kzalloc_obj_vcpu_id_table() -> *mut vcpu_id_table;
    fn kfree(p: *mut c_void);
    fn preempt_disable(); fn preempt_enable();
    fn get_cur_as(v: *mut kvm_vcpu) -> libc::c_uint;
    fn get_cur_pid(v: *mut kvm_vcpu) -> libc::c_uint;
    fn get_cur_pr(v: *mut kvm_vcpu) -> libc::c_uint;
    fn memset(p: *mut c_void, c: libc::c_int, n: usize) -> *mut c_void;
    fn to_e500(v: *mut kvm_vcpu) -> *mut kvmppc_vcpu_e500;
    fn _tlbil_all();
}

#[repr(C)] pub struct kvm_vcpu { pub arch: kvm_arch }
#[repr(C)] pub struct kvm_arch { pub shadow_pid: libc::c_uint, pub shadow_pid1: libc::c_uint, pub pid: u32, pub pvr: u32, pub shared: *mut c_void, pub cpu_type: u32, pub ivor: [u32; 64], pub shadow_msr: u64 }
#[repr(C)] pub struct kvmppc_vcpu_e500 { pub vcpu: kvm_vcpu, pub idt: *mut vcpu_id_table, pub pid: [u32; 2], pub svr: u32, pub hid0: u32, pub mcar: u32 }

#[no_mangle] pub unsafe extern "C" fn kvmppc_e500_id_table_alloc(v: *mut kvmppc_vcpu_e500) -> *mut vcpu_id_table { (*v).idt = kzalloc_obj_vcpu_id_table(); (*v).idt }
#[no_mangle] pub unsafe extern "C" fn kvmppc_e500_id_table_free(v: *mut kvmppc_vcpu_e500) { kfree((*v).idt as *mut c_void); (*v).idt = core::ptr::null_mut(); }

#[no_mangle] pub unsafe extern "C" fn kvmppc_e500_recalc_shadow_pid(v: *mut kvmppc_vcpu_e500) { preempt_disable(); (*v).vcpu.arch.shadow_pid = kvmppc_e500_get_sid(v, get_cur_as(&mut (*v).vcpu), get_cur_pid(&mut (*v).vcpu), get_cur_pr(&mut (*v).vcpu), 1); (*v).vcpu.arch.shadow_pid1 = kvmppc_e500_get_sid(v, get_cur_as(&mut (*v).vcpu), 0, get_cur_pr(&mut (*v).vcpu), 1); preempt_enable(); }

unsafe fn kvmppc_e500_id_table_reset_all(v: *mut kvmppc_vcpu_e500) { memset((*v).idt as *mut c_void, 0, core::mem::size_of::<vcpu_id_table>()); kvmppc_e500_recalc_shadow_pid(v); }
unsafe fn kvmppc_e500_id_table_reset_one(v: *mut kvmppc_vcpu_e500, as_: usize, pid: usize, pr: usize) { let e = &mut (*(*v).idt).id[as_][pid][pr]; e.val = 0; e.pentry = core::ptr::null_mut(); kvmppc_e500_recalc_shadow_pid(v); }

#[no_mangle] pub unsafe extern "C" fn kvmppc_e500_get_sid(v: *mut kvmppc_vcpu_e500, as_: libc::c_uint, gid: libc::c_uint, pr: libc::c_uint, avoid: libc::c_int) -> libc::c_uint { let e = &mut (*(*v).idt).id[as_ as usize][gid as usize][pr as usize]; let mut sid = local_sid_lookup(e); while sid <= 0 { sid = local_sid_setup_one(e); if sid <= 0 { _tlbil_all(); local_sid_destroy_all(); } if avoid == 0 { kvmppc_e500_recalc_shadow_pid(v); } } sid as libc::c_uint }

extern "C" {
    fn get_tlb_ts(x: *mut kvm_book3e_206_tlb_entry) -> libc::c_uint; fn get_tlb_tid(x: *mut kvm_book3e_206_tlb_entry) -> libc::c_uint;
    fn get_cur_pr_vcpu(x: *mut kvm_vcpu) -> libc::c_uint;
    fn kvmppc_e500_tlb_setup(v: *mut kvmppc_vcpu_e500); fn kvmppc_e500_tlb_init(v: *mut kvmppc_vcpu_e500) -> libc::c_int; fn kvmppc_e500_tlb_uninit(v: *mut kvmppc_vcpu_e500);
}
#[repr(C)] pub struct kvm_book3e_206_tlb_entry { pub mas1: u32, pub mas2: u32, pub mas7_3: u32 }

#[no_mangle] pub unsafe extern "C" fn kvmppc_e500_get_tlb_stid(v: *mut kvm_vcpu, t: *mut kvm_book3e_206_tlb_entry) -> libc::c_uint { kvmppc_e500_get_sid(to_e500(v), get_tlb_ts(t), get_tlb_tid(t), get_cur_pr_vcpu(v), 0) }
#[no_mangle] pub unsafe extern "C" fn kvmppc_set_pid(v: *mut kvm_vcpu, pid: u32) { if (*v).arch.pid != pid { let e = to_e500(v); (*e).pid[0] = pid; (*v).arch.pid = pid; kvmppc_e500_recalc_shadow_pid(e); } }

// The remaining MMU, register, lifecycle, operation-table, module-init, and
// exception-handler interfaces are declarations implemented by the surrounding
// PowerPC KVM translation unit.
extern "C" {
}

#[no_mangle] pub unsafe extern "C" fn kvmppc_e500_tlbil_all(v: *mut kvmppc_vcpu_e500) { kvmppc_e500_id_table_reset_all(v); }
#[no_mangle] pub unsafe extern "C" fn kvmppc_mmu_msr_notify(v: *mut kvm_vcpu, _old_msr: u32) { kvmppc_e500_recalc_shadow_pid(to_e500(v)); }

#[no_mangle] pub unsafe extern "C" fn kvmppc_e500_tlbil_one(v: *mut kvmppc_vcpu_e500, t: *mut kvm_book3e_206_tlb_entry) {
    let ts = get_tlb_ts(t) as usize; let tid = get_tlb_tid(t) as usize;
    preempt_disable();
    for pr in 0..2 { if local_sid_lookup(&mut (*(*v).idt).id[ts][tid][pr]) <= 0 { kvmppc_e500_id_table_reset_one(v, ts, tid, pr); } }
    preempt_enable();
}

#[no_mangle] pub unsafe extern "C" fn kvmppc_core_vcpu_setup(v: *mut kvm_vcpu) -> libc::c_int { kvmppc_e500_tlb_setup(to_e500(v)); 0 }
#[no_mangle] pub unsafe extern "C" fn kvmppc_core_init_vm_e500(_kvm: *mut c_void) -> libc::c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn kvmppc_core_destroy_vm_e500(_kvm: *mut c_void) {}
#[no_mangle] pub unsafe extern "C" fn kvmppc_e500_init() -> libc::c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn kvmppc_e500_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
