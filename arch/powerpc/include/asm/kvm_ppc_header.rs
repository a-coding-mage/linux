/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of powerpc/include/asm/kvm_ppc.h. */

pub const KVMPPC_INST_SW_BREAKPOINT: u32 = 0x00dddd00;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum emulation_result { EMULATE_DONE, EMULATE_DO_MMIO, EMULATE_FAIL, EMULATE_AGAIN, EMULATE_EXIT_USER }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum instruction_fetch_type { INST_GENERIC, INST_SC }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum xlate_instdata { XLATE_INST, XLATE_DATA }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum xlate_readwrite { XLATE_READ, XLATE_WRITE }

extern "C" {
    pub fn kvmppc_vcpu_run(vcpu: *mut kvm_vcpu) -> i32;
    pub fn __kvmppc_vcpu_run(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvmppc_handler_highmem();
    pub fn kvmppc_dump_vcpu(vcpu: *mut kvm_vcpu);
    pub fn kvmppc_handle_load(vcpu:*mut kvm_vcpu, rt:u32, bytes:u32, is_default_endian:i32)->i32;
    pub fn kvmppc_handle_loads(vcpu:*mut kvm_vcpu, rt:u32, bytes:u32, is_default_endian:i32)->i32;
    pub fn kvmppc_handle_vsx_load(vcpu:*mut kvm_vcpu, rt:u32, bytes:u32, is_default_endian:i32, mmio_sign_extend:i32)->i32;
    pub fn kvmppc_handle_vmx_load(vcpu:*mut kvm_vcpu, rt:u32, bytes:u32, is_default_endian:i32)->i32;
    pub fn kvmppc_handle_vmx_store(vcpu:*mut kvm_vcpu, rs:u32, bytes:u32, is_default_endian:i32)->i32;
    pub fn kvmppc_handle_store(vcpu:*mut kvm_vcpu, val:u64, bytes:u32, is_default_endian:i32)->i32;
    pub fn kvmppc_handle_vsx_store(vcpu:*mut kvm_vcpu, rs:i32, bytes:u32, is_default_endian:i32)->i32;
    pub fn kvmppc_load_last_inst(vcpu:*mut kvm_vcpu, ty:instruction_fetch_type, inst:*mut usize)->i32;
    pub fn kvmppc_ld(vcpu:*mut kvm_vcpu, eaddr:*mut usize, size:i32, ptr:*mut core::ffi::c_void, data:bool)->i32;
    pub fn kvmppc_st(vcpu:*mut kvm_vcpu, eaddr:*mut usize, size:i32, ptr:*mut core::ffi::c_void, data:bool)->i32;
    pub fn kvmppc_emulate_instruction(vcpu:*mut kvm_vcpu)->i32;
    pub fn kvmppc_emulate_loadstore(vcpu:*mut kvm_vcpu)->i32;
    pub fn kvmppc_emulate_mmio(vcpu:*mut kvm_vcpu)->i32;
    pub fn kvmppc_emulate_dec(vcpu:*mut kvm_vcpu);
    pub fn kvmppc_get_dec(vcpu:*mut kvm_vcpu, tb:u64)->u32;
    pub fn kvmppc_decrementer_func(vcpu:*mut kvm_vcpu);
    pub fn kvmppc_sanity_check(vcpu:*mut kvm_vcpu)->i32;
    pub fn kvmppc_subarch_vcpu_init(vcpu:*mut kvm_vcpu)->i32;
    pub fn kvmppc_subarch_vcpu_uninit(vcpu:*mut kvm_vcpu);
}

/* The remaining declarations retain the C ABI and depend on kernel types supplied by other headers. */
extern "C" {
    pub fn kvmppc_mmu_map(vcpu:*mut kvm_vcpu, gvaddr:u64, gpaddr:gpa_t, gtlb_idx:u32);
    pub fn kvmppc_mmu_switch_pid(vcpu:*mut kvm_vcpu, pid:u32);
    pub fn kvmppc_mmu_dtlb_index(vcpu:*mut kvm_vcpu, eaddr:gva_t)->i32;
    pub fn kvmppc_mmu_itlb_index(vcpu:*mut kvm_vcpu, eaddr:gva_t)->i32;
    pub fn kvmppc_mmu_xlate(vcpu:*mut kvm_vcpu, gtlb_index:u32, eaddr:gva_t)->gpa_t;
    pub fn kvmppc_mmu_dtlb_miss(vcpu:*mut kvm_vcpu); pub fn kvmppc_mmu_itlb_miss(vcpu:*mut kvm_vcpu);
    pub fn kvmppc_xlate(vcpu:*mut kvm_vcpu,eaddr:usize,xlid:xlate_instdata,xlrw:xlate_readwrite,pte:*mut kvmppc_pte)->i32;
    pub fn kvmppc_core_vcpu_create(vcpu:*mut kvm_vcpu)->i32; pub fn kvmppc_core_vcpu_free(vcpu:*mut kvm_vcpu);
    pub fn kvmppc_core_vcpu_setup(vcpu:*mut kvm_vcpu)->i32; pub fn kvmppc_core_vcpu_load(vcpu:*mut kvm_vcpu,cpu:i32); pub fn kvmppc_core_vcpu_put(vcpu:*mut kvm_vcpu);
    pub fn kvmppc_core_prepare_to_enter(vcpu:*mut kvm_vcpu)->i32; pub fn kvmppc_core_pending_dec(vcpu:*mut kvm_vcpu)->i32;
    pub fn kvmppc_core_flush_tlb(vcpu:*mut kvm_vcpu); pub fn kvmppc_core_check_requests(vcpu:*mut kvm_vcpu)->i32;
    pub fn kvmppc_booke_init()->i32; pub fn kvmppc_booke_exit(); pub fn kvmppc_kvm_pv(vcpu:*mut kvm_vcpu)->i32; pub fn kvmppc_map_magic(vcpu:*mut kvm_vcpu);
    pub fn kvmppc_core_init_vm(kvm:*mut kvm)->i32; pub fn kvmppc_core_destroy_vm(kvm:*mut kvm);
    pub fn kvmppc_prepare_to_enter(vcpu:*mut kvm_vcpu)->i32;
    pub fn kvmppc_core_get_sregs(vcpu:*mut kvm_vcpu,sregs:*mut kvm_sregs)->i32; pub fn kvmppc_core_set_sregs(vcpu:*mut kvm_vcpu,sregs:*mut kvm_sregs)->i32;
    pub fn kvmppc_get_one_reg(vcpu:*mut kvm_vcpu,id:u64,val:*mut kvmppc_one_reg)->i32; pub fn kvmppc_set_one_reg(vcpu:*mut kvm_vcpu,id:u64,val:*mut kvmppc_one_reg)->i32;
}

#[repr(C)]
pub union kvmppc_one_reg { pub wval:u32, pub dval:u64, pub vval:vector128, pub vsxval:[u64;2], pub vsx32val:[u32;4], pub vsx16val:[u16;8], pub vsx8val:[u8;16], pub vpaval: kvmppc_vpa, pub xive_timaval:[u64;2] }
#[repr(C)] pub struct kvmppc_vpa { pub addr:u64, pub length:u64 }

#[repr(C)]
pub struct kvmppc_ops {
    pub owner:*mut module,
    pub get_sregs:Option<unsafe extern "C" fn(*mut kvm_vcpu,*mut kvm_sregs)->i32>, pub set_sregs:Option<unsafe extern "C" fn(*mut kvm_vcpu,*mut kvm_sregs)->i32>,
    pub get_one_reg:Option<unsafe extern "C" fn(*mut kvm_vcpu,u64,*mut kvmppc_one_reg)->i32>, pub set_one_reg:Option<unsafe extern "C" fn(*mut kvm_vcpu,u64,*mut kvmppc_one_reg)->i32>,
    pub vcpu_load:Option<unsafe extern "C" fn(*mut kvm_vcpu,i32)>, pub vcpu_put:Option<unsafe extern "C" fn(*mut kvm_vcpu)>, pub inject_interrupt:Option<unsafe extern "C" fn(*mut kvm_vcpu,i32,u64)>, pub set_msr:Option<unsafe extern "C" fn(*mut kvm_vcpu,u64)>, pub vcpu_run:Option<unsafe extern "C" fn(*mut kvm_vcpu)->i32>, pub vcpu_create:Option<unsafe extern "C" fn(*mut kvm_vcpu)->i32>, pub vcpu_free:Option<unsafe extern "C" fn(*mut kvm_vcpu)>, pub check_requests:Option<unsafe extern "C" fn(*mut kvm_vcpu)->i32>,
    pub get_dirty_log:Option<unsafe extern "C" fn(*mut kvm,*mut kvm_dirty_log)->i32>, pub flush_memslot:Option<unsafe extern "C" fn(*mut kvm,*mut kvm_memory_slot)>, pub free_memslot:Option<unsafe extern "C" fn(*mut kvm_memory_slot)>, pub init_vm:Option<unsafe extern "C" fn(*mut kvm)->i32>, pub destroy_vm:Option<unsafe extern "C" fn(*mut kvm)>,
}
extern "C" { pub static mut kvmppc_hv_ops:*mut kvmppc_ops; pub static mut kvmppc_pr_ops:*mut kvmppc_ops; }

#[inline] pub unsafe fn kvmppc_get_field(inst:u64,msb:i32,lsb:i32)->u32 { assert!(msb<=lsb); let mask=(1u64<<((lsb-msb+1) as u32))-1; ((inst>>(63-lsb))&mask) as u32 }
#[inline] pub unsafe fn kvmppc_set_field(inst:u64,msb:i32,lsb:i32,value:i32)->u32 { assert!(msb<=lsb); let mask=((1u64<<((lsb-msb+1) as u32))-1)<<(63-lsb); ((inst&!mask)|(((value as u64)<<(63-lsb))&mask)) as u32 }
#[inline] pub unsafe fn is_kvmppc_hv_enabled(kvm:*mut kvm)->bool { (*kvm).arch.kvm_ops==kvmppc_hv_ops }

/* Configuration-dependent inline helpers and macro-generated accessors are preserved as declarations. */
extern "C" { pub fn kvmppc_hwrng_present()->i32; pub fn kvm_vcpu_ioctl_get_one_reg(vcpu:*mut kvm_vcpu,reg:*mut kvm_one_reg)->i32; pub fn kvm_vcpu_ioctl_set_one_reg(vcpu:*mut kvm_vcpu,reg:*mut kvm_one_reg)->i32; pub fn kvmppc_set_pid(vcpu:*mut kvm_vcpu,pid:u32); pub fn xics_wake_cpu(cpu:i32); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
