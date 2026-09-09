// SPDX-License-Identifier: GPL-2.0-only
/* POWER LPAR Platform KeyStore(PLPKS) */

// Kernel and architecture dependencies are supplied by the surrounding tree.

const PLPKS_WRAPKEY_COMPONENT: &[u8] = b"PLPKSWR\0";
const PLPKS_WRAPKEY_NAME: &[u8] = b"default-wrapping-key\0";
const PLPKS_WRAPPING_BUF_ALIGN: usize = 4096;
const PLPKS_WRAPPING_BUF_DIFF: u32 = 1024;
const PLPKS_WRAP_INTERFACE_BIT: u32 = 3;
const PLPKS_WRAPPING_KEY_LENGTH: u32 = 32;

#[repr(C, packed, align(16))]
pub struct PlpksAuth {
    pub version: u8,
    pub consumer: u8,
    pub rsvd0: u64,
    pub rsvd1: u32,
    pub passwordlength: u16,
    pub password: [u8; 0],
}

#[repr(C)]
pub struct LabelAttr { pub prefix: [u8; 8], pub version: u8, pub os: u8, pub length: u8, pub reserved: [u8; 5] }
#[repr(C)]
pub struct Label { pub attr: LabelAttr, pub name: [u8; PLPKS_MAX_NAME_SIZE], pub size: usize }

static mut ospassword: *mut u8 = core::ptr::null_mut();
static mut ospasswordlength: u16 = 0;
static mut version: u8 = 0;
static mut objoverhead: u16 = 0;
static mut maxpwsize: u16 = 0;
static mut maxobjsize: u16 = 0;
static mut maxobjlabelsize: i16 = 0;
static mut totalsize: u32 = 0;
static mut usedspace: u32 = 0;
static mut supportedpolicies: u32 = 0;
static mut maxlargeobjectsize: u32 = 0;
static mut signedupdatealgorithms: u64 = 0;
static mut wrappingfeatures: u64 = 0;
static mut wrapsupport: bool = false;

unsafe fn pseries_status_to_err(rc: i32) -> i32 {
    let err = match rc { H_SUCCESS => 0, H_FUNCTION => -ENXIO, H_PARAMETER|H_P2|H_P3|H_P4|H_P5|H_P6 => -EINVAL,
        H_NOT_FOUND => -ENOENT, H_BUSY|H_LONG_BUSY_ORDER_1_MSEC|H_LONG_BUSY_ORDER_10_MSEC|H_LONG_BUSY_ORDER_100_MSEC|H_LONG_BUSY_ORDER_1_SEC|H_LONG_BUSY_ORDER_10_SEC|H_LONG_BUSY_ORDER_100_SEC => -EBUSY,
        H_AUTHORITY => -EPERM, H_NO_MEM => -ENOMEM, H_RESOURCE|H_IN_USE => -EEXIST, H_TOO_BIG => -EFBIG, H_STATE|H_R_STATE|H_ABORTED => -EIO, _ => -EINVAL };
    pr_debug!("Converted hypervisor code {} to Linux {}\n", rc, err); err
}

unsafe fn plpks_gen_password() -> i32 {
    let mut retbuf = [0usize; PLPAR_HCALL_BUFSIZE]; let consumer = PLPKS_OS_OWNER;
    if !ospassword.is_null() { pr_debug!("Password of length {} already in use\n", ospasswordlength); return 0; }
    let password = kzalloc(roundup_pow_of_two(maxpwsize as usize), GFP_KERNEL);
    if password.is_null() { return -ENOMEM; }
    let mut rc = plpar_hcall(H_PKS_GEN_PASSWORD, retbuf.as_mut_ptr(), consumer as usize, 0, virt_to_phys(password), maxpwsize as usize);
    if rc == 0 { ospasswordlength=maxpwsize; ospassword=kzalloc(maxpwsize as usize,GFP_KERNEL); if ospassword.is_null(){ kfree_sensitive(password); return -ENOMEM; } memcpy(ospassword,password,ospasswordlength as usize); }
    else if rc == H_IN_USE { pr_warn!("Password already set - authenticated operations will fail\n"); rc=0; }
    else { kfree_sensitive(password); return pseries_status_to_err(rc); }
    kfree_sensitive(password); pseries_status_to_err(rc)
}

unsafe fn construct_auth(consumer: u8) -> *mut PlpksAuth {
    if consumer > PLPKS_OS_OWNER { return ERR_PTR(-EINVAL); }
    let auth = kzalloc(roundup_pow_of_two(core::mem::size_of::<PlpksAuth>() + maxpwsize as usize), GFP_KERNEL) as *mut PlpksAuth;
    if auth.is_null() { return ERR_PTR(-ENOMEM); }
    (*auth).version=1; (*auth).consumer=consumer;
    if consumer != PLPKS_FW_OWNER && consumer != PLPKS_BOOTLOADER_OWNER { memcpy((*auth).password.as_mut_ptr(),ospassword,ospasswordlength as usize); (*auth).passwordlength=ospasswordlength.to_be(); }
    auth
}

unsafe fn construct_label(component: *mut i8, varos: u8, name: *mut u8, namelen: u16) -> *mut Label {
    if name.is_null() || namelen as usize > PLPKS_MAX_NAME_SIZE { return ERR_PTR(-EINVAL); }
    let slen=if component.is_null(){0}else{strlen(component)}; if slen>8{return ERR_PTR(-EINVAL);}
    let label=kzalloc(roundup_pow_of_two(core::mem::size_of::<Label>()),GFP_KERNEL) as *mut Label; if label.is_null(){return ERR_PTR(-ENOMEM);}
    if !component.is_null(){memcpy((*label).attr.prefix.as_mut_ptr(),component,slen);} (*label).attr.version=PLPKS_LABEL_VERSION; (*label).attr.os=varos; (*label).attr.length=PLPKS_MAX_LABEL_ATTR_SIZE; memcpy((*label).name.as_mut_ptr(),name,namelen as usize); (*label).size=core::mem::size_of::<LabelAttr>()+namelen as usize; label
}

unsafe fn _plpks_get_config() -> i32 {
    let mut retbuf=[0usize;PLPAR_HCALL_BUFSIZE]; let config=kzalloc(512,GFP_KERNEL) as *mut u8; if config.is_null(){return -ENOMEM;}
    let mut rc=plpar_hcall(H_PKS_GET_CONFIG,retbuf.as_mut_ptr(),virt_to_phys(config),512); if rc!=H_SUCCESS{rc=pseries_status_to_err(rc);kfree(config);return rc;}
    version=*config.add(0); objoverhead=u16::from_be(*(config.add(4) as *const u16)); maxpwsize=u16::from_be(*(config.add(6) as *const u16)); maxobjlabelsize=u16::from_be(*(config.add(8) as *const u16)) as i16; maxobjsize=u16::from_be(*(config.add(10) as *const u16)); totalsize=u32::from_be(*(config.add(12) as *const u32)); usedspace=u32::from_be(*(config.add(16) as *const u32)); supportedpolicies=u32::from_be(*(config.add(20) as *const u32)); maxlargeobjectsize=u32::from_be(*(config.add(24) as *const u32)); signedupdatealgorithms=u64::from_be(*(config.add(28) as *const u64)); wrappingfeatures=u64::from_be(*(config.add(36) as *const u64)); wrapsupport=*config.add(1)&PPC_BIT8(PLPKS_WRAP_INTERFACE_BIT)!=0;
    if maxpwsize<32||maxobjlabelsize<255||totalsize<4096||(version>=3&&maxlargeobjectsize>=65536&&maxobjsize!=0xffff){kfree(config);return -EIO;} kfree(config);0
}

pub unsafe fn plpks_get_version()->u8{version}
pub unsafe fn plpks_get_objoverhead()->u16{objoverhead}
pub unsafe fn plpks_get_maxpwsize()->u16{maxpwsize}
pub unsafe fn plpks_get_maxobjectsize()->u16{maxobjsize}
pub unsafe fn plpks_get_maxobjectlabelsize()->i16{maxobjlabelsize}
pub unsafe fn plpks_get_totalsize()->u32{totalsize}
pub unsafe fn plpks_get_usedspace()->u32{if _plpks_get_config()!=0{0}else{usedspace}}
pub unsafe fn plpks_get_supportedpolicies()->u32{supportedpolicies}
pub unsafe fn plpks_get_maxlargeobjectsize()->u32{maxlargeobjectsize}
pub unsafe fn plpks_get_signedupdatealgorithms()->u64{signedupdatealgorithms}
pub unsafe fn plpks_get_wrappingfeatures()->u64{wrappingfeatures}
pub unsafe fn plpks_get_passwordlen()->u16{ospasswordlength}
pub unsafe fn plpks_is_available()->bool{firmware_has_feature(FW_FEATURE_PLPKS)&&_plpks_get_config()==0}

pub unsafe fn plpks_wrapping_is_supported()->bool{wrapsupport}

unsafe fn plpks_confirm_object_flushed(label:*mut Label,auth:*mut PlpksAuth)->i32 {
    let mut retbuf=[0usize;PLPAR_HCALL_BUFSIZE]; let mut timeout:u64=0;
    loop { let mut rc=plpar_hcall(H_PKS_CONFIRM_OBJECT_FLUSHED,retbuf.as_mut_ptr(),virt_to_phys(auth),virt_to_phys(label),(*label).size); let status=retbuf[0];
        if rc!=0 { if rc==H_NOT_FOUND&&status==1{rc=0;} return pseries_status_to_err(rc); } if status==1{return 0;} fsleep(PLPKS_FLUSH_SLEEP); timeout=timeout.wrapping_add(PLPKS_FLUSH_SLEEP as u64); if timeout>=PLPKS_MAX_TIMEOUT{return -ETIMEDOUT;} }
}

pub unsafe fn plpks_read_os_var(var:*mut PlpksVar)->i32{plpks_read_var(PLPKS_OS_OWNER,var)}
pub unsafe fn plpks_read_fw_var(var:*mut PlpksVar)->i32{plpks_read_var(PLPKS_FW_OWNER,var)}
pub unsafe fn plpks_read_bootloader_var(var:*mut PlpksVar)->i32{plpks_read_var(PLPKS_BOOTLOADER_OWNER,var)}
unsafe fn plpks_read_var(consumer:u8,var:*mut PlpksVar)->i32 { if (*var).namelen>PLPKS_MAX_NAME_SIZE||(*var).policy&PLPKS_WRAPPINGKEY!=0{return -EINVAL;} let auth=construct_auth(consumer); if IS_ERR(auth){return PTR_ERR(auth);} let label=if consumer==PLPKS_OS_OWNER{construct_label((*var).component,(*var).os,(*var).name,(*var).namelen)}else{core::ptr::null_mut()}; if consumer==PLPKS_OS_OWNER&&IS_ERR(label){kfree(auth);return PTR_ERR(label);} let output=kzalloc(maxobjsize as usize,GFP_KERNEL); if output.is_null(){kfree(label);kfree(auth);return -ENOMEM;} let mut rb=[0usize;PLPAR_HCALL_BUFSIZE]; let rc=if consumer==PLPKS_OS_OWNER{plpar_hcall(H_PKS_READ_OBJECT,rb.as_mut_ptr(),virt_to_phys(auth),virt_to_phys(label),(*label).size,virt_to_phys(output),maxobjsize as usize)}else{plpar_hcall(H_PKS_READ_OBJECT,rb.as_mut_ptr(),virt_to_phys(auth),virt_to_phys((*var).name),(*var).namelen as usize,virt_to_phys(output),maxobjsize as usize)}; let result=if rc!=H_SUCCESS{pseries_status_to_err(rc)}else{if (*var).data.is_null()||(*var).datalen>rb[0] as u32{(*var).datalen=rb[0] as u32;}(*var).policy=rb[1] as u32;if !(*var).data.is_null(){memcpy((*var).data,output,(*var).datalen as usize);}0}; kfree(output);kfree(label);kfree(auth);result }

pub unsafe fn plpks_populate_fdt(fdt:*mut core::ffi::c_void)->i32 { let chosen=fdt_path_offset(fdt,b"/chosen\0".as_ptr() as _); if chosen<0{return chosen;} fdt_setprop(fdt,chosen,b"ibm,plpks-pw\0".as_ptr() as _,ospassword,ospasswordlength as usize) }

pub unsafe fn plpks_early_init_devtree(){let fdt=initial_boot_params;let chosen=fdt_path_offset(fdt,b"/chosen\0".as_ptr() as _);if chosen<0{return;}let mut len=0;let password=fdt_getprop(fdt,chosen,b"ibm,plpks-pw\0".as_ptr() as _,&mut len);if len<=0{return;}ospassword=memblock_alloc_raw(len as usize,SMP_CACHE_BYTES);if !ospassword.is_null(){memcpy(ospassword,password,len as usize);ospasswordlength=len as u16;}fdt_nop_property(fdt,chosen,b"ibm,plpks-pw\0".as_ptr() as _);early_init_dt_verify(fdt,__pa(fdt));}

// These public entry points use externally defined PLPKS variable layouts and H_CALL bindings.
// Their declarations preserve the source interfaces for the surrounding translation unit.
extern "C" {
    pub fn plpks_signed_update_var(var:*mut PlpksVar,flags:u64)->i32;
    pub fn plpks_write_var(var:PlpksVar)->i32;
    pub fn plpks_remove_var(component:*mut i8,varos:u8,vname:PlpksVarName)->i32;
    pub fn plpks_gen_wrapping_key()->i32;
    pub fn plpks_wrap_object(input_buf:*mut *mut u8,input_len:u32,wrap_flags:u16,output_buf:*mut *mut u8,output_len:*mut u32)->i32;
    pub fn plpks_unwrap_object(input_buf:*mut *mut u8,input_len:u32,output_buf:*mut *mut u8,output_len:*mut u32)->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
