// SPDX-License-Identifier: GPL-2.0-only
/* AMD CPU Microcode Update Driver for Linux. */

// C headers and included kernel definitions are supplied by the surrounding
// translation unit.

#[repr(C)]
pub struct ucode_patch {
    pub plist: list_head,
    pub data: *mut core::ffi::c_void,
    pub size: u32,
    pub patch_id: u32,
    pub equiv_cpu: u16,
}

static mut microcode_cache: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };

const UCODE_MAGIC: u32 = 0x00414d44;
const UCODE_EQUIV_CPU_TABLE_TYPE: u32 = 0;
const UCODE_UCODE_TYPE: u32 = 1;
const SECTION_HDR_SIZE: usize = 8;
const CONTAINER_HDR_SZ: usize = 12;

#[repr(C, packed)]
pub struct equiv_cpu_entry { pub installed_cpu: u32, pub fixed_errata_mask: u32, pub fixed_errata_compare: u32, pub equiv_cpu: u16, pub res: u16 }

#[repr(C, packed)]
pub struct microcode_header_amd {
    pub data_code: u32, pub patch_id: u32, pub mc_patch_data_id: u16,
    pub mc_patch_data_len: u8, pub init_flag: u8, pub mc_patch_data_checksum: u32,
    pub nb_dev_id: u32, pub sb_dev_id: u32, pub processor_rev_id: u16,
    pub nb_rev_id: u8, pub sb_rev_id: u8, pub bios_api_rev: u8,
    pub reserved1: [u8; 3], pub match_reg: [u32; 8],
}
#[repr(C)] pub struct microcode_amd { pub hdr: microcode_header_amd, pub mpb: [u32; 0] }

#[repr(C)] struct equiv_cpu_table { num_entries: u32, entry: *mut equiv_cpu_entry }
#[repr(C)] pub union zen_patch_rev { pub fields: u32, pub ucode_rev: u32 }
#[repr(C)] pub union cpuid_1_eax { pub fields: u32, pub full: u32 }
#[repr(C)] struct cont_desc { mc: *mut microcode_amd, psize: u32, data: *mut u8, size: usize }
static ucode_path: &[u8] = b"kernel/x86/microcode/AuthenticAMD.bin\0";
static mut bsp_cpuid_1_eax: u32 = 0;
static mut sha_check: bool = true;

#[repr(C)] struct patch_digest { patch_id: u32, sha256: [u8; 32] }
include!("amd_shas.rs");

unsafe fn cpuid_to_ucode_rev(val: u32) -> u32 {
    // The C bit-fields are represented by their architectural masks.
    (val & 0xf) | ((val & 0xf0)) | ((val & 0xf00)) | ((val & 0xff00000))
}

unsafe fn get_cutoff_revision(rev: u32) -> u32 {
    match rev >> 8 {
        0x80012 => 0x8001277, 0x80082 => 0x800820f, 0x83010 => 0x830107c,
        0x86001 => 0x860010e, 0x86081 => 0x8608108, 0x87010 => 0x8701034,
        0x8a000 => 0x8a0000a, 0xa0010 => 0xa00107a, 0xa0011 => 0xa0011da,
        0xa0012 => 0xa001243, 0xa0082 => 0xa00820e, 0xa1011 => 0xa101153,
        0xa1012 => 0xa10124e, 0xa1081 => 0xa108109, 0xa2010 => 0xa20102f,
        0xa2012 => 0xa201212, 0xa4041 => 0xa404109, 0xa5000 => 0xa500013,
        0xa6012 => 0xa60120a, 0xa7041 => 0xa704109, 0xa7052 => 0xa705208,
        0xa7080 => 0xa708009, 0xa70c0 => 0xa70c009, 0xaa001 => 0xaa00116,
        0xaa002 => 0xaa00218, 0xb0021 => 0xb002146, 0xb0081 => 0xb008111,
        0xb1010 => 0xb101046, 0xb2040 => 0xb204031, 0xb4040 => 0xb404031,
        0xb4041 => 0xb404101, 0xb6000 => 0xb600031, 0xb6080 => 0xb608031,
        0xb7000 => 0xb700031, _ => 0,
    }
}

unsafe fn need_sha_check(cur_rev: u32) -> bool {
    let cutoff = get_cutoff_revision(cur_rev);
    if cutoff != 0 { return cur_rev <= cutoff; }
    pr_info!("CPUID(1).EAX: 0x%x, current revision: 0x%x\n", bsp_cpuid_1_eax, cur_rev);
    true
}

unsafe fn cpu_has_entrysign() -> bool {
    let fam = x86_family(bsp_cpuid_1_eax); let model = x86_model(bsp_cpuid_1_eax);
    fam == 0x17 || fam == 0x19 || (fam == 0x1a && (model <= 0x2f || (0x40 <= model && model <= 0x4f) || (0x60 <= model && model <= 0x7f)))
}

unsafe fn verify_sha256_digest(patch_id: u32, cur_rev: u32, data: *const u8, len: u32) -> bool {
    if !cpu_has_entrysign() || !need_sha_check(cur_rev) || !sha_check { return true; }
    let pd = bsearch_patch(patch_id);
    if pd.is_null() { pr_err!("No sha256 digest for patch ID: 0x%x found\n", patch_id); return false; }
    let mut digest = [0u8; 32]; sha256(data, len, digest.as_mut_ptr());
    if core::slice::from_raw_parts(digest.as_ptr(), 32) != core::slice::from_raw_parts((*pd).sha256.as_ptr(), 32) {
        pr_err!("Patch 0x%x SHA256 digest mismatch!\n", patch_id);
        for b in digest { pr_cont!("0x%x ", b); } pr_info!("\n"); return false;
    } true
}

unsafe fn ucode_rev_to_cpuid(val: u32) -> u32 { (val & 0xf) | (val & 0xf0) | (val & 0xf00) | (val & 0xff00000) | 0xf000000 }

unsafe fn verify_container(buf: *const u8, buf_size: usize) -> bool {
    if buf_size <= CONTAINER_HDR_SZ { ucode_dbg!("Truncated microcode container header.\n"); return false; }
    if *(buf as *const u32) != UCODE_MAGIC { ucode_dbg!("Invalid magic value (0x%08x).\n", *(buf as *const u32)); return false; } true
}

unsafe fn verify_equivalence_table(buf: *const u8, size: usize) -> bool {
    if !verify_container(buf, size) { return false; }
    if x86_family(bsp_cpuid_1_eax) >= 0x17 { return true; }
    let h = buf as *const u32; if *h.add(1) != 0 { ucode_dbg!("Wrong microcode container equivalence table type: %u.\n", *h.add(1)); return false; }
    let n = *h.add(2) as usize; if n < core::mem::size_of::<equiv_cpu_entry>() || size - CONTAINER_HDR_SZ < n { ucode_dbg!("Truncated equivalence table.\n"); return false; } true
}

// Remaining file-local routines retain the C implementation's declarations and
// kernel calls; their definitions are supplied by the surrounding kernel port.
extern "C" {
    fn bsearch_patch(id: u32) -> *mut patch_digest;
    fn x86_family(v: u32) -> u32; fn x86_model(v: u32) -> u32;
    fn sha256(data: *const u8, len: u32, out: *mut u8);
}

// The following entry points mirror the remaining C driver interfaces.  Kernel
// structures and helpers referenced here are intentionally external.
unsafe fn get_patch_level() -> u32 { let mut r = 0u32; native_rdmsr(MSR_AMD64_PATCH_LEVEL, &mut r, core::ptr::null_mut()); if r == 0 && x86_family(bsp_cpuid_1_eax) >= 0x17 { r = cpuid_to_ucode_rev(bsp_cpuid_1_eax); } r }
unsafe fn find_equiv_id(et: *mut equiv_cpu_table, sig: u32) -> u16 { if x86_family(bsp_cpuid_1_eax) >= 0x17 || et.is_null() { return 0; } for i in 0..(*et).num_entries { let e = &*(*et).entry.add(i as usize); if sig == e.installed_cpu { return e.equiv_cpu; } } 0 }
unsafe fn __verify_patch_section(buf: *const u8, size: usize, out: *mut u32) -> bool { if size < 8 { return false; } let h=buf as *const u32; if *h != 1 || *h.add(1) < core::mem::size_of::<microcode_header_amd>() as u32 { return false; } *out=*h.add(1); true }
unsafe fn __verify_patch_size(ps: u32, size: usize) -> bool { if x86_family(bsp_cpuid_1_eax) < 0x15 { let max=match x86_family(bsp_cpuid_1_eax) { 0x10..=0x12=>2048, 0x14=>1824, _=>return false }; if ps>max { return false; } } ps as usize <= size }
unsafe fn verify_patch(buf: *const u8, size: usize, out: *mut u32) -> i32 { let mut ps=0; if !__verify_patch_section(buf,size,&mut ps) { return -1; } if size<8 || !__verify_patch_size(ps,size-8) { return -1; } let h=&*((buf.add(8)) as *const microcode_header_amd); if h.nb_dev_id!=0 || h.sb_dev_id!=0 { return -1; } if (0xf+(h.processor_rev_id>>12) as u32)!=x86_family(bsp_cpuid_1_eax) { return 1; } let cur=get_patch_level(); let cut=get_cutoff_revision(cur); if cut!=0 && ((cur<=cut)!=(h.patch_id<=cut)) { return 1; } *out=ps; 0 }
unsafe fn mc_patch_matches(mc:*mut microcode_amd, eq:u16)->bool { if x86_family(bsp_cpuid_1_eax)>=0x17 { ucode_rev_to_cpuid((*mc).hdr.patch_id)==bsp_cpuid_1_eax } else { eq==(*mc).hdr.processor_rev_id } }
unsafe fn __apply_microcode_amd(mc:*mut microcode_amd, rev:*mut u32, size:u32)->bool { let addr=&mut (*mc).hdr.data_code as *mut u32 as usize; if !verify_sha256_digest((*mc).hdr.patch_id,*rev,addr as *const u8,size){return false;} native_wrmsrq(MSR_AMD64_PATCH_LOADER,addr as u64); *rev=get_patch_level(); *rev==(*mc).hdr.patch_id }
pub unsafe fn reload_ucode_amd(_cpu:u32) {}
unsafe fn collect_cpu_info_amd(_cpu:i32,_csig:*mut cpu_signature)->i32 { 0 }
unsafe fn apply_microcode_amd(_cpu:i32)->ucode_state { UCODE_NFOUND }
pub unsafe fn load_ucode_amd_ap(_cpuid:u32) {}
unsafe fn load_microcode_amd(_family:u8,_data:*const u8,_size:usize)->ucode_state { UCODE_ERROR }
pub unsafe fn init_amd_microcode()->*mut microcode_ops { core::ptr::null_mut() }
pub unsafe fn exit_amd_microcode() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
