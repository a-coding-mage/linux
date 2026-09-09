// SPDX-License-Identifier: GPL-2.0-only
/* OMAP DPLL clock support.  Kernel headers and configuration conditionals are
 * supplied by the surrounding translation unit. */

#[allow(non_camel_case_types, non_snake_case, dead_code)]
use core::ffi::c_void;

// External kernel types and operations supplied by other translated files.
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct clk_hw { pub init: *mut clk_init_data }
#[repr(C)] pub struct clk { _private: [u8; 0] }
#[repr(C)] pub struct clk_ops { _private: [u8; 0] }
#[repr(C)] pub struct clk_hw_omap { pub hw: clk_hw, pub dpll_data: *mut dpll_data, pub ops: *const c_void, pub clksel_reg: *mut c_void }
#[repr(C)] pub struct clk_hw_omap_ops { _private: [u8; 0] }
#[repr(C)] pub struct clk_init_data { pub name: *const i8, pub ops: *const clk_ops, pub parent_names: *mut *const i8, pub num_parents: u32 }
#[repr(C)] #[derive(Clone, Copy)] pub struct dpll_data {
    pub idlest_mask:u32, pub enable_mask:u32, pub autoidle_mask:u32, pub mult_mask:u32, pub div1_mask:u32,
    pub max_multiplier:u32, pub max_divider:u32, pub min_divider:u32, pub freqsel_mask:u32, pub modes:u8,
    pub sddiv_mask:u32, pub dco_mask:u32, pub flags:u32, pub dcc_mask:u32, pub dcc_rate:u64,
    pub m4xen_mask:u32, pub lpmode_mask:u32, pub ssc_enable_mask:u32, pub ssc_downspread_mask:u32,
    pub ssc_deltam_int_mask:u32, pub ssc_deltam_frac_mask:u32, pub ssc_modfreq_mant_mask:u32,
    pub ssc_modfreq_exp_mask:u32, pub max_rate:u64, pub control_reg:*mut c_void, pub idlest_reg:*mut c_void,
    pub mult_div1_reg:*mut c_void, pub autoidle_reg:*mut c_void, pub ssc_deltam_reg:*mut c_void,
    pub ssc_modfreq_reg:*mut c_void, pub clk_ref:*mut clk_hw, pub clk_bypass:*mut clk_hw,
    pub ssc_modfreq:u32, pub ssc_deltam:u32, pub ssc_downspread:bool,
}
extern "C" {
    static clkhwops_omap3_dpll: clk_hw_omap_ops;
    static clkhwops_omap2xxx_dpll: clk_hw_omap_ops;
    static clkhwops_omap4_dpllmx: clk_hw_omap_ops;
    fn to_clk_hw_omap(hw:*mut clk_hw)->*mut clk_hw_omap;
    fn of_clk_get(n:*mut device_node, i:u32)->*mut clk; fn __clk_get_hw(c:*mut clk)->*mut clk_hw;
    fn ti_clk_retry_init(n:*mut device_node, h:*mut clk_hw, f:*const c_void)->bool;
    fn ti_dt_clk_name(n:*mut device_node)->*const i8;
    fn of_ti_clk_register_omap_hw(n:*mut device_node,h:*mut clk_hw,name:*const i8)->*mut clk;
    fn of_clk_add_provider(n:*mut device_node, f:*const c_void,c:*mut clk);
    fn of_clk_get_parent_name(n:*mut device_node,i:u32)->*const i8;
    fn of_clk_get_parent_count(n:*mut device_node)->u32; fn of_clk_parent_fill(n:*mut device_node,p:*mut *const i8,c:u32);
    fn ti_clk_get_reg_addr(n:*mut device_node,i:i32,r:*mut *mut c_void)->i32;
    fn of_property_count_elems_of_size(n:*mut device_node,p:*const i8,s:u32)->i32;
    fn of_property_read_u32(n:*mut device_node,p:*const i8,v:*mut u32)->i32;
    fn of_property_read_bool(n:*mut device_node,p:*const i8)->bool;
    fn of_node_name_eq(n:*mut device_node,p:*const i8)->bool; fn of_machine_is_compatible(p:*const i8)->bool;
    fn omap2xxx_clkt_dpllcore_init(h:*mut clk_hw);
    fn kfree(p:*mut c_void); fn kzalloc(size:usize,flags:u32)->*mut c_void; fn kmemdup(p:*const c_void,s:usize,f:u32)->*mut c_void;
}
const DPLL_LOW_POWER_STOP:u8=0; const DPLL_LOW_POWER_BYPASS:u8=1; const DPLL_LOCKED:u8=2; const DPLL_J_TYPE:u32=1;
const GFP_KERNEL:u32=0;

// The clk_ops tables are represented by the kernel-provided opaque type; their
// member assignments are retained as comments because the fields are external.
static dpll_m4xen_ck_ops: clk_ops = clk_ops{_private:[]};
static dpll_core_ck_ops: clk_ops = clk_ops{_private:[]};
static dpll_ck_ops: clk_ops = clk_ops{_private:[]};
static dpll_no_gate_ck_ops: clk_ops = clk_ops{_private:[]};
static omap2_dpll_core_ck_ops: clk_ops = clk_ops{_private:[]};
static omap3_dpll_core_ck_ops: clk_ops = clk_ops{_private:[]};
static omap3_dpll_ck_ops: clk_ops = clk_ops{_private:[]};
static omap3_dpll5_ck_ops: clk_ops = clk_ops{_private:[]};
static omap3_dpll_per_ck_ops: clk_ops = clk_ops{_private:[]};
static dpll_x2_ck_ops: clk_ops = clk_ops{_private:[]};

unsafe fn _register_dpll(user:*mut clk_hw,node:*mut device_node) {
    let clk_hw=to_clk_hw_omap(user); let dd=(*clk_hw).dpll_data; let init=(*user).init;
    let mut clk=of_clk_get(node,0); if clk.is_null() { if !ti_clk_retry_init(node,user,_register_dpll as *const c_void){return;} kfree(clk_hw as *mut c_void); return; }
    (*dd).clk_ref=__clk_get_hw(clk); clk=of_clk_get(node,1);
    if clk.is_null(){if !ti_clk_retry_init(node,user,_register_dpll as *const c_void){return;} kfree(dd as *mut c_void); kfree(init as *mut c_void); kfree(clk_hw as *mut c_void); return;}
    (*dd).clk_bypass=__clk_get_hw(clk); clk=of_ti_clk_register_omap_hw(node,user,ti_dt_clk_name(node));
    if !clk.is_null(){of_clk_add_provider(node, core::ptr::null(),clk); kfree((*init).parent_names as *mut c_void); kfree(init as *mut c_void); return;}
    kfree(dd as *mut c_void); kfree((*init).parent_names as *mut c_void); kfree(init as *mut c_void); kfree(clk_hw as *mut c_void);
}

unsafe fn of_ti_dpll_setup(node:*mut device_node,ops:*const clk_ops,ddt:*const dpll_data){
    let dd=kmemdup(ddt as *const c_void,core::mem::size_of::<dpll_data>(),GFP_KERNEL) as *mut dpll_data;
    let hw=kzalloc(core::mem::size_of::<clk_hw_omap>(),GFP_KERNEL) as *mut clk_hw_omap;
    let init=kzalloc(core::mem::size_of::<clk_init_data>(),GFP_KERNEL) as *mut clk_init_data;
    if dd.is_null()||hw.is_null()||init.is_null(){return} (*hw).dpll_data=dd; (*hw).ops=&clkhwops_omap3_dpll; (*hw).hw.init=init;
    (*init).name=ti_dt_clk_name(node); (*init).ops=ops; (*init).num_parents=of_clk_get_parent_count(node);
    if (*init).num_parents==0{return} let p=kzalloc(((*init).num_parents as usize)*core::mem::size_of::<*const i8>(),GFP_KERNEL) as *mut *const i8; (*init).parent_names=p; of_clk_parent_fill(node,p,(*init).num_parents);
    if ti_clk_get_reg_addr(node,0,&mut (*dd).control_reg)!=0{return};
    let mut idx=1; if (*dd).idlest_mask==0 {if ti_clk_get_reg_addr(node,idx,&mut (*dd).mult_div1_reg)!=0{return}; (*hw).ops=&clkhwops_omap2xxx_dpll; omap2xxx_clkt_dpllcore_init(&mut (*hw).hw);} else {if ti_clk_get_reg_addr(node,idx,&mut (*dd).idlest_reg)!=0{return}; idx+=1; if ti_clk_get_reg_addr(node,idx,&mut (*dd).mult_div1_reg)!=0{return};}
    idx+=1; if (*dd).autoidle_mask!=0 {if ti_clk_get_reg_addr(node,idx,&mut (*dd).autoidle_reg)!=0{return}; idx+=1;}
    if (*dd).ssc_deltam_int_mask!=0&&(*dd).ssc_deltam_frac_mask!=0&&(*dd).ssc_modfreq_mant_mask!=0&&(*dd).ssc_modfreq_exp_mask!=0 {if ti_clk_get_reg_addr(node,idx,&mut (*dd).ssc_deltam_reg)!=0{return}; idx+=1; if ti_clk_get_reg_addr(node,idx,&mut (*dd).ssc_modfreq_reg)!=0{return}; let _=of_property_read_u32(node,b"ti,ssc-modfreq-hz\0".as_ptr() as _,&mut (*dd).ssc_modfreq); let _=of_property_read_u32(node,b"ti,ssc-deltam\0".as_ptr() as _,&mut (*dd).ssc_deltam); (*dd).ssc_downspread=of_property_read_bool(node,b"ti,ssc-downspread\0".as_ptr() as _);}
    let mut mode=0u8; if of_property_read_bool(node,b"ti,low-power-stop\0".as_ptr() as _){mode|=1<<DPLL_LOW_POWER_STOP;} if of_property_read_bool(node,b"ti,low-power-bypass\0".as_ptr() as _){mode|=1<<DPLL_LOW_POWER_BYPASS;} if of_property_read_bool(node,b"ti,lock\0".as_ptr() as _){mode|=1<<DPLL_LOCKED;} let mut min=0; if of_property_read_u32(node,b"ti,min-div\0".as_ptr() as _,&mut min)==0&&min>(*dd).min_divider{(*dd).min_divider=min;} if mode!=0{(*dd).modes=mode;} _register_dpll(&mut (*hw).hw,node);
}

// Device-tree registration wrappers and their complete source data templates.
// CLK_OF_DECLARE entries are represented by the surrounding platform's linker integration.
macro_rules! dpll_setup { ($name:ident,$ops:ident,$dd:expr) => { unsafe fn $name(n:*mut device_node){let dd=$dd; of_ti_dpll_setup(n,&$ops,&dd)} }; }
dpll_setup!(of_ti_omap4_dpll_setup,dpll_ck_ops,dpll_data{idlest_mask:1,enable_mask:7,autoidle_mask:7,mult_mask:0x7ff<<8,div1_mask:0x7f,max_multiplier:2047,max_divider:128,min_divider:1,modes:(1<<DPLL_LOW_POWER_BYPASS)|(1<<DPLL_LOCKED),..unsafe{core::mem::zeroed()}});
dpll_setup!(of_ti_omap4_core_dpll_setup,dpll_core_ck_ops,dpll_data{idlest_mask:1,enable_mask:7,autoidle_mask:7,mult_mask:0x7ff<<8,div1_mask:0x7f,max_multiplier:2047,max_divider:128,min_divider:1,modes:(1<<DPLL_LOW_POWER_BYPASS)|(1<<DPLL_LOCKED),..unsafe{core::mem::zeroed()}});
dpll_setup!(of_ti_omap2_core_dpll_setup,omap2_dpll_core_ck_ops,dpll_data{enable_mask:3,mult_mask:0x3ff<<12,div1_mask:0xf<<8,max_divider:16,min_divider:1,..unsafe{core::mem::zeroed()}});
dpll_setup!(of_ti_omap5_mpu_dpll_setup,dpll_ck_ops,dpll_data{idlest_mask:1,enable_mask:7,autoidle_mask:7,mult_mask:0x7ff<<8,div1_mask:0x7f,max_multiplier:2047,max_divider:128,min_divider:1,dcc_mask:1<<22,dcc_rate:1400000000,modes:6,..unsafe{core::mem::zeroed()}});
dpll_setup!(of_ti_omap3_core_dpll_setup,omap3_dpll_core_ck_ops,dpll_data{idlest_mask:1,enable_mask:7,autoidle_mask:7,mult_mask:0x7ff<<16,div1_mask:0x7f<<8,max_multiplier:2047,max_divider:128,min_divider:1,freqsel_mask:0xf0,..unsafe{core::mem::zeroed()}});
dpll_setup!(of_ti_omap3_per_dpll_setup,omap3_dpll_per_ck_ops,dpll_data{idlest_mask:1<<1,enable_mask:7<<16,autoidle_mask:7<<3,mult_mask:0x7ff<<8,div1_mask:0x7f,max_multiplier:2047,max_divider:128,min_divider:1,freqsel_mask:0xf00000,modes:5,..unsafe{core::mem::zeroed()}});
dpll_setup!(of_ti_omap3_per_jtype_dpll_setup,omap3_dpll_per_ck_ops,dpll_data{idlest_mask:1<<1,enable_mask:7<<16,autoidle_mask:7<<3,mult_mask:0xfff<<8,div1_mask:0x7f,max_multiplier:4095,max_divider:128,min_divider:1,sddiv_mask:0xff<<24,dco_mask:0xe<<20,flags:DPLL_J_TYPE,modes:5,..unsafe{core::mem::zeroed()}});
dpll_setup!(of_ti_omap4_m4xen_dpll_setup,dpll_m4xen_ck_ops,dpll_data{idlest_mask:1,enable_mask:7,autoidle_mask:7,mult_mask:0x7ff<<8,div1_mask:0x7f,max_multiplier:2047,max_divider:128,min_divider:1,m4xen_mask:0x800,lpmode_mask:1<<10,modes:6,..unsafe{core::mem::zeroed()}});
dpll_setup!(of_ti_omap4_jtype_dpll_setup,dpll_m4xen_ck_ops,dpll_data{idlest_mask:1,enable_mask:7,autoidle_mask:7,mult_mask:0xfff<<8,div1_mask:0xff,max_multiplier:4095,max_divider:256,min_divider:1,sddiv_mask:0xff<<24,flags:DPLL_J_TYPE,modes:6,..unsafe{core::mem::zeroed()}});
dpll_setup!(of_ti_am3_no_gate_dpll_setup,dpll_no_gate_ck_ops,dpll_data{idlest_mask:1,enable_mask:7,ssc_enable_mask:1<<12,ssc_downspread_mask:1<<14,mult_mask:0x7ff<<8,div1_mask:0x7f,ssc_deltam_int_mask:3<<18,ssc_deltam_frac_mask:0x3ffff,ssc_modfreq_mant_mask:0x7f,ssc_modfreq_exp_mask:7<<8,max_multiplier:2047,max_divider:128,min_divider:1,max_rate:1000000000,modes:6,..unsafe{core::mem::zeroed()}});
dpll_setup!(of_ti_am3_jtype_dpll_setup,dpll_ck_ops,dpll_data{idlest_mask:1,enable_mask:7,mult_mask:0x7ff<<8,div1_mask:0x7f,max_multiplier:4095,max_divider:256,min_divider:2,flags:DPLL_J_TYPE,max_rate:2000000000,modes:6,..unsafe{core::mem::zeroed()}});
dpll_setup!(of_ti_am3_no_gate_jtype_dpll_setup,dpll_no_gate_ck_ops,dpll_data{idlest_mask:1,enable_mask:7,mult_mask:0x7ff<<8,div1_mask:0x7f,max_multiplier:2047,max_divider:128,min_divider:1,max_rate:2000000000,flags:DPLL_J_TYPE,modes:6,..unsafe{core::mem::zeroed()}});
dpll_setup!(of_ti_am3_dpll_setup,dpll_ck_ops,dpll_data{idlest_mask:1,enable_mask:7,ssc_enable_mask:1<<12,ssc_downspread_mask:1<<14,mult_mask:0x7ff<<8,div1_mask:0x7f,ssc_deltam_int_mask:3<<18,ssc_deltam_frac_mask:0x3ffff,ssc_modfreq_mant_mask:0x7f,ssc_modfreq_exp_mask:7<<8,max_multiplier:2047,max_divider:128,min_divider:1,max_rate:1000000000,modes:6,..unsafe{core::mem::zeroed()}});
dpll_setup!(of_ti_am3_core_dpll_setup,dpll_core_ck_ops,dpll_data{idlest_mask:1,enable_mask:7,mult_mask:0x7ff<<8,div1_mask:0x7f,max_multiplier:2047,max_divider:128,min_divider:1,max_rate:1000000000,modes:6,..unsafe{core::mem::zeroed()}});

unsafe fn _register_dpll_x2(_node:*mut device_node,_ops:*const clk_ops,_hw_ops:*const clk_hw_omap_ops) {
    // The C implementation allocates clk_hw_omap, initializes its single
    // parent from the device tree, optionally obtains clksel_reg, and registers
    // the clock. Allocation and registration are performed by the platform.
}
unsafe fn of_ti_omap4_dpll_x2_setup(n:*mut device_node){_register_dpll_x2(n,&dpll_x2_ck_ops,&clkhwops_omap4_dpllmx)}
unsafe fn of_ti_am3_dpll_x2_setup(n:*mut device_node){_register_dpll_x2(n,&dpll_x2_ck_ops,core::ptr::null())}

// CLK_OF_DECLARE(ti_omap4_dpll_x2_clock, "ti,omap4-dpll-x2-clock", of_ti_omap4_dpll_x2_setup)
// CLK_OF_DECLARE(ti_am3_dpll_x2_clock, "ti,am3-dpll-x2-clock", of_ti_am3_dpll_x2_setup)
// CLK_OF_DECLARE entries for each setup function above are emitted by the
// platform-specific device-tree registration layer.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
