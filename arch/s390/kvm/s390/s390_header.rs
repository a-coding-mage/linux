/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of s390.h; external kernel types and functions are supplied by dependencies. */

#[repr(C)]
pub union kvm_s390_quad {
    pub sixteen: u128,
    pub eight: usize,
    pub four: u32,
    pub two: u16,
    pub one: u8,
}

pub const KVM_S390_UCONTROL_MEMSLOT: usize = KVM_USER_MEM_SLOTS + 0;
pub const TDB_FORMAT1: u8 = 1;
pub const GUEST_PREFIX_SHIFT: u32 = 12;
pub const GUEST_PREFIX_MASK_ZARCH: u32 = 0x7fffe;
pub const GUEST_PREFIX_MASK_ESA: u32 = 0x7ffff;

pub unsafe fn kvm_s390_fpu_store(run: *mut kvm_run) {
    fpu_stfpc(&mut (*run).s.regs.fpc);
    if cpu_has_vx() { save_vx_regs((&mut (*run).s.regs.vrs as *mut _) as *mut __vector128); }
    else { save_fp_regs((&mut (*run).s.regs.fprs as *mut _) as *mut freg_t); }
}
pub unsafe fn kvm_s390_fpu_load(run: *mut kvm_run) {
    fpu_lfpc_safe(&mut (*run).s.regs.fpc);
    if cpu_has_vx() { load_vx_regs((&mut (*run).s.regs.vrs as *mut _) as *mut __vector128); }
    else { load_fp_regs((&mut (*run).s.regs.fprs as *mut _) as *mut freg_t); }
}

#[macro_export] macro_rules! IS_TE_ENABLED { ($vcpu:expr) => { ((*$vcpu).arch.sie_block.ecb & ECB_TE) }; }
#[macro_export] macro_rules! IS_ITDB_VALID { ($vcpu:expr) => { (*(phys_to_virt((*$vcpu).arch.sie_block.itdba) as *const i8) as u8) == TDB_FORMAT1 }; }

extern "C" {
    pub static mut kvm_s390_dbf: *mut debug_info_t;
    pub static mut kvm_s390_dbf_uv: *mut debug_info_t;
}

pub unsafe fn kvm_s390_set_cpuflags(vcpu: *mut kvm_vcpu, flags: u32) { atomic_or(flags, &mut (*(*vcpu).arch.sie_block).cpuflags); }
pub unsafe fn kvm_s390_clear_cpuflags(vcpu: *mut kvm_vcpu, flags: u32) { atomic_andnot(flags, &mut (*(*vcpu).arch.sie_block).cpuflags); }
pub unsafe fn kvm_s390_test_cpuflags(vcpu: *mut kvm_vcpu, flags: u32) -> bool { (atomic_read(&(*(*vcpu).arch.sie_block).cpuflags) & flags) == flags }
pub unsafe fn is_vcpu_stopped(vcpu: *mut kvm_vcpu) -> i32 { kvm_s390_test_cpuflags(vcpu, CPUSTAT_STOPPED) as i32 }
pub unsafe fn is_vcpu_idle(vcpu: *mut kvm_vcpu) -> i32 { test_bit((*vcpu).vcpu_idx, (*(*vcpu).kvm).arch.idle_mask) as i32 }
pub unsafe fn kvm_is_ucontrol(kvm: *mut kvm) -> i32 {
#[cfg(CONFIG_KVM_S390_UCONTROL)] { test_bit(GMAP_FLAG_IS_UCONTROL, &(*(*kvm).arch.gmap).flags) as i32 }
#[cfg(not(CONFIG_KVM_S390_UCONTROL))] { 0 }
}
pub unsafe fn kvm_s390_get_prefix(vcpu: *mut kvm_vcpu) -> u32 { (*(*vcpu).arch.sie_block).prefix << GUEST_PREFIX_SHIFT }
pub unsafe fn kvm_s390_set_prefix(vcpu: *mut kvm_vcpu, prefix: u32) { (*(*vcpu).arch.sie_block).prefix = (prefix >> GUEST_PREFIX_SHIFT) & GUEST_PREFIX_MASK_ZARCH; kvm_make_request(KVM_REQ_TLB_FLUSH, vcpu); kvm_make_request(KVM_REQ_REFRESH_GUEST_PREFIX, vcpu); }

pub unsafe fn kvm_s390_get_base_disp_s(vcpu: *mut kvm_vcpu, ar: *mut u8) -> u64 { let b=(*(*vcpu).arch.sie_block).ipb>>28; let d=((*(*vcpu).arch.sie_block).ipb&0x0fff0000)>>16; if !ar.is_null(){*ar=b as u8;} (if b!=0 {(*vcpu).run.s.regs.gprs[b as usize]} else {0})+d as u64 }
pub unsafe fn kvm_s390_get_base_disp_siy(vcpu: *mut kvm_vcpu, ar: *mut u8) -> u64 { let b=(*(*vcpu).arch.sie_block).ipb>>28; let d=sign_extend64(((((*vcpu).arch.sie_block).ipb&0x0fff0000)>>16)+(((*vcpu).arch.sie_block).ipb&0xff00<<4),19); if !ar.is_null(){*ar=b as u8;} (if b!=0 {(*vcpu).run.s.regs.gprs[b as usize]} else {0}) .wrapping_add(d as u64) }
pub unsafe fn kvm_s390_get_base_disp_sse(vcpu:*mut kvm_vcpu,a1:*mut u64,a2:*mut u64,b1:*mut u8,b2:*mut u8){let x=(*(*vcpu).arch.sie_block).ipb;let r1=(x&0xf0000000)>>28;let d1=(x&0x0fff0000)>>16;let r2=(x&0xf000)>>12;let d2=x&0x0fff;*a1=if r1!=0{(*vcpu).run.s.regs.gprs[r1 as usize]}else{0}+d1 as u64;*a2=if r2!=0{(*vcpu).run.s.regs.gprs[r2 as usize]}else{0}+d2 as u64;if !b1.is_null(){*b1=r1 as u8};if !b2.is_null(){*b2=r2 as u8}}
pub unsafe fn kvm_s390_get_regs_rre(vcpu:*mut kvm_vcpu,r1:*mut i32,r2:*mut i32){let x=(*(*vcpu).arch.sie_block).ipb;if !r1.is_null(){*r1=((x&0x00f00000)>>20)as i32}if !r2.is_null(){*r2=((x&0x000f0000)>>16)as i32}}

pub unsafe fn kvm_s390_set_psw_cc(vcpu:*mut kvm_vcpu,cc:usize){(*(*vcpu).arch.sie_block).gpsw.mask&=!(3usize<<44);(*(*vcpu).arch.sie_block).gpsw.mask|=cc<<44}
pub unsafe fn test_kvm_facility(kvm:*mut kvm,nr:usize)->i32{(__test_facility(nr,(*kvm).arch.model.fac_mask)&&__test_facility(nr,(*kvm).arch.model.fac_list))as i32}
pub unsafe fn set_kvm_facility(fac_list:*mut u64,nr:usize)->i32{if nr>=MAX_FACILITY_BIT{return -EINVAL};let p=(fac_list as *mut u8).add(nr>>3);*p|=0x80u8>>(nr&7);0}
pub unsafe fn test_kvm_cpu_feat(kvm:*mut kvm,nr:usize)->i32{test_bit_inv(nr,(*kvm).arch.cpu_feat)as i32}
pub unsafe fn kvm_s390_user_cpu_state_ctrl(kvm:*mut kvm)->i32{((*kvm).arch.user_cpu_state_ctrl!=0)as i32}
pub unsafe fn kvm_s390_set_user_cpu_state_ctrl(kvm:*mut kvm){if (*kvm).arch.user_cpu_state_ctrl==0{(*kvm).arch.user_cpu_state_ctrl=1}}

pub unsafe fn kvm_s390_pv_get_handle(kvm:*mut kvm)->u64{(*kvm).arch.pv.handle}
pub unsafe fn kvm_s390_pv_cpu_get_handle(vcpu:*mut kvm_vcpu)->u64{(*vcpu).arch.pv.handle}
pub unsafe fn __kvm_s390_pv_destroy_page(page:*mut page)->i32{let f=page_folio(page);if folio_test_large(f)!=0{return -EFAULT};let mut rc=uv_destroy_folio(f);if rc!=0{rc=uv_convert_from_secure_folio(f)}rc}

// External declarations from pv.c, interrupt.c, intercept.c, priv.c, vsie.c, sigp.c, s390.c, diag.c, and guestdbg.c.
extern "C" {
    pub fn kvm_s390_pv_destroy_cpu(*mut kvm_vcpu,*mut u16,*mut u16)->i32;
    pub fn kvm_s390_pv_create_cpu(*mut kvm_vcpu,*mut u16,*mut u16)->i32;
    pub fn kvm_s390_handle_wait(*mut kvm_vcpu)->i32;
    pub fn kvm_s390_get_ilen(*mut kvm_vcpu)->u8;
    pub fn kvm_handle_sie_intercept(*mut kvm_vcpu)->i32;
    pub fn handle_sthyi(*mut kvm_vcpu)->i32;
    pub fn kvm_s390_handle_diag(*mut kvm_vcpu)->i32;
    pub fn kvm_s390_handle_sigp(*mut kvm_vcpu)->i32;
    pub fn kvm_s390_handle_vsie(*mut kvm_vcpu)->i32;
    pub fn kvm_s390_pv_set_aside(*mut kvm,*mut u16,*mut u16)->i32;
    pub fn kvm_s390_pv_deinit_aside_vm(*mut kvm,*mut u16,*mut u16)->i32;
    pub fn kvm_s390_pv_deinit_cleanup_all(*mut kvm,*mut u16,*mut u16)->i32;
    pub fn kvm_s390_pv_deinit_vm(*mut kvm,*mut u16,*mut u16)->i32;
    pub fn kvm_s390_pv_init_vm(*mut kvm,*mut u16,*mut u16)->i32;
    pub fn kvm_s390_pv_set_sec_parms(*mut kvm,*mut core::ffi::c_void,u64,*mut u16,*mut u16)->i32;
    pub fn kvm_s390_pv_unpack(*mut kvm,usize,usize,usize,*mut u16,*mut u16)->i32;
    pub fn kvm_s390_pv_set_cpu_state(*mut kvm_vcpu,u8)->i32;
    pub fn kvm_s390_pv_dump_cpu(*mut kvm_vcpu,*mut core::ffi::c_void,*mut u16,*mut u16)->i32;
    pub fn kvm_s390_pv_destroy_page(*mut kvm,usize)->i32;
    pub fn kvm_s390_pv_convert_to_secure(*mut kvm,usize)->i32;
    pub fn kvm_s390_pv_make_secure(*mut kvm,usize,*mut core::ffi::c_void)->i32;
    pub fn kvm_s390_vcpu_wakeup(*mut kvm_vcpu);
    pub fn kvm_s390_deliver_pending_interrupts(*mut kvm_vcpu)->i32;
    pub fn kvm_s390_clear_local_irqs(*mut kvm_vcpu);
    pub fn kvm_s390_clear_float_irqs(*mut kvm);
    pub fn kvm_s390_vsie_kick(*mut kvm_vcpu);
    pub fn kvm_s390_vsie_init(*mut kvm);
    pub fn kvm_s390_vsie_destroy(*mut kvm);
    pub fn kvm_s390_handle_sigp_pei(*mut kvm_vcpu)->i32;
    pub fn kvm_s390_vcpu_start(*mut kvm_vcpu)->i32;
    pub fn kvm_s390_vcpu_stop(*mut kvm_vcpu)->i32;
    pub fn kvm_s390_vcpu_block(*mut kvm_vcpu);
    pub fn kvm_s390_vcpu_unblock(*mut kvm_vcpu);
    pub fn kvm_s390_sync_request(i32,*mut kvm_vcpu);
    pub fn kvm_s390_handle_aa(*mut kvm_vcpu)->i32;
    pub fn kvm_s390_handle_b2(*mut kvm_vcpu)->i32;
    pub fn kvm_s390_handle_e3(*mut kvm_vcpu)->i32;
    pub fn kvm_s390_handle_e5(*mut kvm_vcpu)->i32;
    pub fn kvm_s390_handle_01(*mut kvm_vcpu)->i32;
    pub fn kvm_s390_handle_b9(*mut kvm_vcpu)->i32;
    pub fn kvm_s390_handle_lpsw(*mut kvm_vcpu)->i32;
    pub fn kvm_s390_handle_stctl(*mut kvm_vcpu)->i32;
    pub fn kvm_s390_handle_lctl(*mut kvm_vcpu)->i32;
    pub fn kvm_s390_handle_eb(*mut kvm_vcpu)->i32;
    pub fn kvm_s390_skey_check_enable(*mut kvm_vcpu)->i32;
    pub fn kvm_s390_handle_per_ifetch_icpt(*mut kvm_vcpu)->i32;
    pub fn kvm_s390_handle_per_event(*mut kvm_vcpu)->i32;
    pub fn kvm_s390_reinject_machine_check(*mut kvm_vcpu,*mut mcck_volatile_info);
    pub fn kvm_s390_vcpu_crypto_reset_all(*mut kvm);
    pub fn kvm_s390_vcpu_pci_enable_interp(*mut kvm);
    pub static mut diag9c_forwarding_hz: u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
