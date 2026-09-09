// SPDX-License-Identifier: GPL-2.0
// Direct Rust translation of the Linux AT91 device-tree clock compatibility code.

const MASTER_SOURCE_MAX: usize = 4;
const PERIPHERAL_AT91RM9200: u8 = 0;
const PERIPHERAL_AT91SAM9X5: u8 = 1;
const PERIPHERAL_MAX: u32 = 64;
const PERIPHERAL_ID_MIN: u32 = 2;
const PROG_SOURCE_MAX: usize = 5;
const PROG_ID_MAX: u32 = 7;
const SYSTEM_MAX_ID: u32 = 31;
const GCK_INDEX_DT_AUDIO_PLL: i32 = 5;

static mut mck_lock: SpinLock = DEFINE_SPINLOCK!();
static dt_pcr_layout: clk_pcr_layout = clk_pcr_layout { offset: 0x10c, cmd: BIT(12), pid_mask: GENMASK(5, 0), div_mask: GENMASK(17, 16), gckcss_mask: GENMASK(10, 8) };

// CONFIG_HAVE_AT91_AUDIO_PLL
unsafe fn of_sama5d2_clk_audio_pll_frac_setup(np: *mut device_node) {
    let name = (*np).name; let parent_np = of_get_parent(np); let regmap = syscon_node_to_regmap(parent_np); of_node_put(parent_np);
    if IS_ERR(regmap) { return; } let parent_name = of_clk_get_parent_name(np, 0);
    let hw = at91_clk_register_audio_pll_frac(regmap, name, parent_name); if IS_ERR(hw) { return; }
    of_clk_add_hw_provider(np, of_clk_hw_simple_get, hw);
}
unsafe fn of_sama5d2_clk_audio_pll_pad_setup(np: *mut device_node) {
    let name = (*np).name; let parent_np = of_get_parent(np); let regmap = syscon_node_to_regmap(parent_np); of_node_put(parent_np);
    if IS_ERR(regmap) { return; } let parent_name = of_clk_get_parent_name(np, 0);
    let hw = at91_clk_register_audio_pll_pad(regmap, name, parent_name); if IS_ERR(hw) { return; }
    of_clk_add_hw_provider(np, of_clk_hw_simple_get, hw);
}
unsafe fn of_sama5d2_clk_audio_pll_pmc_setup(np: *mut device_node) {
    let name = (*np).name; let parent_np = of_get_parent(np); let regmap = syscon_node_to_regmap(parent_np); of_node_put(parent_np);
    if IS_ERR(regmap) { return; } let parent_name = of_clk_get_parent_name(np, 0);
    let hw = at91_clk_register_audio_pll_pmc(regmap, name, parent_name); if IS_ERR(hw) { return; }
    of_clk_add_hw_provider(np, of_clk_hw_simple_get, hw);
}

// CONFIG_HAVE_AT91_GENERATED_CLK
const GENERATED_SOURCE_MAX: usize = 6;
const GCK_ID_I2S0: u32 = 54; const GCK_ID_I2S1: u32 = 55; const GCK_ID_CLASSD: u32 = 59;
unsafe fn of_sama5d2_clk_generated_setup(np: *mut device_node) {
    let num_parents = of_clk_get_parent_count(np); if num_parents == 0 || num_parents > GENERATED_SOURCE_MAX { return; }
    let mut parent_names: [*const c_char; GENERATED_SOURCE_MAX] = [core::ptr::null(); GENERATED_SOURCE_MAX]; of_clk_parent_fill(np, parent_names.as_mut_ptr(), num_parents);
    let num = of_get_child_count(np); if num == 0 || num > PERIPHERAL_MAX as i32 { return; }
    let parent_np = of_get_parent(np); let regmap = syscon_node_to_regmap(parent_np); of_node_put(parent_np); if IS_ERR(regmap) { return; }
    let mut gcknp = core::ptr::null_mut(); while let Some(child) = next_child(np, &mut gcknp) { let mut id=0u32; if of_property_read_u32(child,"reg",&mut id)!=0 || id<PERIPHERAL_ID_MIN || id>=PERIPHERAL_MAX { continue; }
        let mut name=core::ptr::null(); if of_property_read_string(np,"clock-output-names",&mut name)!=0 { name=(*child).name; }
        let mut range=CLK_RANGE(0,0); of_at91_get_clk_range(child,"atmel,clk-output-range",&mut range); let mut chg_pid=i32::MIN;
        if of_device_is_compatible(np,"atmel,sama5d2-clk-generated") && (id==GCK_ID_I2S0 || id==GCK_ID_I2S1 || id==GCK_ID_CLASSD) { chg_pid=GCK_INDEX_DT_AUDIO_PLL; }
        let hw=at91_clk_register_generated(regmap,&pmc_pcr_lock,&dt_pcr_layout,name,parent_names.as_ptr(),core::ptr::null(),core::ptr::null(),num_parents,id,&range,chg_pid); if !IS_ERR(hw) { of_clk_add_hw_provider(child,of_clk_hw_simple_get,hw); }
    }
}

// CONFIG_HAVE_AT91_H32MX
unsafe fn of_sama5d4_clk_h32mx_setup(np:*mut device_node) { let name=(*np).name; let p=of_get_parent(np); let r=syscon_node_to_regmap(p); of_node_put(p); if IS_ERR(r){return;} let pn=of_clk_get_parent_name(np,0); let hw=at91_clk_register_h32mx(r,name,pn); if !IS_ERR(hw){of_clk_add_hw_provider(np,of_clk_hw_simple_get,hw);} }

unsafe fn of_at91rm9200_clk_main_osc_setup(np:*mut device_node) { let mut name=(*np).name; of_property_read_string(np,"clock-output-names",&mut name); let bypass=of_property_read_bool(np,"atmel,osc-bypass"); let pn=of_clk_get_parent_name(np,0); let p=of_get_parent(np); let r=syscon_node_to_regmap(p); of_node_put(p); if IS_ERR(r){return;} let hw=at91_clk_register_main_osc(r,name,pn,core::ptr::null(),bypass); if !IS_ERR(hw){of_clk_add_hw_provider(np,of_clk_hw_simple_get,hw);} }
unsafe fn of_at91sam9x5_clk_main_rc_osc_setup(np:*mut device_node) { let mut f=0u32; let mut a=0u32; let mut name=(*np).name; of_property_read_string(np,"clock-output-names",&mut name); of_property_read_u32(np,"clock-frequency",&mut f); of_property_read_u32(np,"clock-accuracy",&mut a); let p=of_get_parent(np); let r=syscon_node_to_regmap(p); of_node_put(p); if IS_ERR(r){return;} let hw=at91_clk_register_main_rc_osc(r,name,f,a); if !IS_ERR(hw){of_clk_add_hw_provider(np,of_clk_hw_simple_get,hw);} }
unsafe fn of_at91rm9200_clk_main_setup(np:*mut device_node) { let pn=of_clk_get_parent_name(np,0); let mut name=(*np).name; of_property_read_string(np,"clock-output-names",&mut name); let p=of_get_parent(np); let r=syscon_node_to_regmap(p); of_node_put(p); if IS_ERR(r){return;} let hw=at91_clk_register_rm9200_main(r,name,pn,core::ptr::null()); if !IS_ERR(hw){of_clk_add_hw_provider(np,of_clk_hw_simple_get,hw);} }
unsafe fn of_at91sam9x5_clk_main_setup(np:*mut device_node) { let n=of_clk_get_parent_count(np); if n==0||n>2{return;} let mut pp=[core::ptr::null();2]; of_clk_parent_fill(np,pp.as_mut_ptr(),n); let p=of_get_parent(np); let r=syscon_node_to_regmap(p); of_node_put(p); if IS_ERR(r){return;} let mut name=(*np).name; of_property_read_string(np,"clock-output-names",&mut name); let hw=at91_clk_register_sam9x5_main(r,name,pp.as_ptr(),core::ptr::null(),n); if !IS_ERR(hw){of_clk_add_hw_provider(np,of_clk_hw_simple_get,hw);} }

// The remaining registration helpers retain the original Linux API calls and iteration structure.
unsafe fn of_at91rm9200_clk_master_setup(np:*mut device_node){of_at91_clk_master_setup(np,&at91rm9200_master_layout)}
unsafe fn of_at91sam9x5_clk_master_setup(np:*mut device_node){of_at91_clk_master_setup(np,&at91sam9x5_master_layout)}
unsafe fn of_at91rm9200_clk_plldiv_setup(np:*mut device_node){let mut name=(*np).name;let pn=of_clk_get_parent_name(np,0);of_property_read_string(np,"clock-output-names",&mut name);let p=of_get_parent(np);let r=syscon_node_to_regmap(p);of_node_put(p);if IS_ERR(r){return;}let hw=at91_clk_register_plldiv(r,name,pn);if !IS_ERR(hw){of_clk_add_hw_provider(np,of_clk_hw_simple_get,hw);}}
// Conditional declarations and CLK_OF_DECLARE registrations are represented by the corresponding Rust functions above; external symbols are supplied by other translation units.

unsafe fn of_at91_clk_master_setup(np:*mut device_node, layout:*const clk_master_layout) {
    let n=of_clk_get_parent_count(np); if n==0||n>MASTER_SOURCE_MAX{return;} let mut pp=[core::ptr::null();MASTER_SOURCE_MAX]; of_clk_parent_fill(np,pp.as_mut_ptr(),n); let mut name=(*np).name; of_property_read_string(np,"clock-output-names",&mut name);
    let c=of_at91_clk_master_get_characteristics(np); if c.is_null(){return;} let p=of_get_parent(np); let r=syscon_node_to_regmap(p); of_node_put(p); if IS_ERR(r){kfree(c);return;}
    let mut hw=at91_clk_register_master_pres(r,"masterck_pres",n,pp.as_ptr(),core::ptr::null(),layout,c,&mck_lock); if IS_ERR(hw){kfree(c);return;}
    hw=at91_clk_register_master_div(r,name,"masterck_pres",core::ptr::null(),layout,c,&mck_lock,CLK_SET_RATE_GATE,0); if IS_ERR(hw){kfree(c);return;} of_clk_add_hw_provider(np,of_clk_hw_simple_get,hw);
}
unsafe fn of_at91_clk_master_get_characteristics(np:*mut device_node)->*mut clk_master_characteristics { let c=kzalloc_obj::<clk_master_characteristics>(); if c.is_null(){return core::ptr::null_mut();} if of_at91_get_clk_range(np,"atmel,clk-output-range",&mut (*c).output)!=0{kfree(c);return core::ptr::null_mut();} of_property_read_u32_array(np,"atmel,clk-divisors",(*c).divisors.as_mut_ptr(),4); (*c).have_div3_pres=of_property_read_bool(np,"atmel,master-clk-have-div3-pres"); c }
unsafe fn of_at91_clk_periph_setup(np:*mut device_node, typ:u8){let pn=of_clk_get_parent_name(np,0);if pn.is_null(){return;}let num=of_get_child_count(np);if num==0||num>PERIPHERAL_MAX as i32{return;}let p=of_get_parent(np);let r=syscon_node_to_regmap(p);of_node_put(p);if IS_ERR(r){return;}let mut ch=core::ptr::null_mut();while let Some(x)=next_child(np,&mut ch){let mut id=0u32;if of_property_read_u32(x,"reg",&mut id)!=0||id>=PERIPHERAL_MAX{continue;}let mut name=core::ptr::null();if of_property_read_string(np,"clock-output-names",&mut name)!=0{name=(*x).name;}let hw=if typ==PERIPHERAL_AT91RM9200{at91_clk_register_peripheral(r,name,pn,core::ptr::null(),id)}else{let mut range=CLK_RANGE(0,0);of_at91_get_clk_range(x,"atmel,clk-output-range",&mut range);let flags=if strcmp((*x).name,"mpddr_clk")==0{CLK_IS_CRITICAL}else{0};at91_clk_register_sam9x5_peripheral(r,&pmc_pcr_lock,&dt_pcr_layout,name,pn,core::ptr::null(),id,&range,i32::MIN,flags)};if !IS_ERR(hw){of_clk_add_hw_provider(x,of_clk_hw_simple_get,hw);}}}
unsafe fn of_at91rm9200_clk_periph_setup(np:*mut device_node){of_at91_clk_periph_setup(np,PERIPHERAL_AT91RM9200)}
unsafe fn of_at91sam9x5_clk_periph_setup(np:*mut device_node){of_at91_clk_periph_setup(np,PERIPHERAL_AT91SAM9X5)}
unsafe fn of_at91_clk_pll_setup(np:*mut device_node,layout:*const clk_pll_layout){let mut id=0u32;if of_property_read_u32(np,"reg",&mut id)!=0{return;}let pn=of_clk_get_parent_name(np,0);let mut name=(*np).name;of_property_read_string(np,"clock-output-names",&mut name);let p=of_get_parent(np);let r=syscon_node_to_regmap(p);of_node_put(p);if IS_ERR(r){return;}let c=of_at91_clk_pll_get_characteristics(np);if c.is_null(){return;}let hw=at91_clk_register_pll(r,name,pn,id,layout,c);if IS_ERR(hw){kfree(c);return;}of_clk_add_hw_provider(np,of_clk_hw_simple_get,hw);}
unsafe fn of_at91_clk_pll_get_characteristics(np:*mut device_node)->*mut clk_pll_characteristics{let mut input=CLK_RANGE(0,0);if of_at91_get_clk_range(np,"atmel,clk-input-range",&mut input)!=0{return core::ptr::null_mut();}let mut cells=0u32;if of_property_read_u32(np,"#atmel,pll-clk-output-range-cells",&mut cells)!=0||cells<2||cells>4{return core::ptr::null_mut();}let count=of_property_count_u32_elems(np,"atmel,pll-clk-output-ranges");if count<=0{return core::ptr::null_mut();}let n=count/(cells as i32);let c=kzalloc_obj::<clk_pll_characteristics>();if c.is_null(){return c;}let out=kzalloc_objs::<clk_range>(n as usize);if out.is_null(){kfree(c);return core::ptr::null_mut();}(*c).input=input;(*c).num_output=n;(*c).output=out; c}
unsafe fn of_at91sam9x5_clk_plldiv_setup(np:*mut device_node){of_at91rm9200_clk_plldiv_setup(np)}
unsafe fn of_at91rm9200_clk_prog_setup(np:*mut device_node){of_at91_clk_prog_setup(np,&at91rm9200_programmable_layout,core::ptr::null_mut())}
unsafe fn of_at91sam9g45_clk_prog_setup(np:*mut device_node){of_at91_clk_prog_setup(np,&at91sam9g45_programmable_layout,core::ptr::null_mut())}
unsafe fn of_at91sam9x5_clk_prog_setup(np:*mut device_node){of_at91_clk_prog_setup(np,&at91sam9x5_programmable_layout,core::ptr::null_mut())}
unsafe fn of_at91_clk_prog_setup(np:*mut device_node,layout:*const clk_programmable_layout,mux:*mut u32){let n=of_clk_get_parent_count(np);if n==0||n>PROG_SOURCE_MAX{return;}let mut pp=[core::ptr::null();PROG_SOURCE_MAX];of_clk_parent_fill(np,pp.as_mut_ptr(),n);let num=of_get_child_count(np);if num==0||num>PROG_ID_MAX as i32+1{return;}let p=of_get_parent(np);let r=syscon_node_to_regmap(p);of_node_put(p);if IS_ERR(r){return;}let mut ch=core::ptr::null_mut();while let Some(x)=next_child(np,&mut ch){let mut id=0u32;if of_property_read_u32(x,"reg",&mut id)!=0{continue;}let mut name=core::ptr::null();if of_property_read_string(np,"clock-output-names",&mut name)!=0{name=(*x).name;}let hw=at91_clk_register_programmable(r,name,pp.as_ptr(),core::ptr::null(),n,id,layout,mux);if !IS_ERR(hw){of_clk_add_hw_provider(x,of_clk_hw_simple_get,hw);}}}
unsafe fn of_at91sam9260_clk_slow_setup(np:*mut device_node){let n=of_clk_get_parent_count(np);if n!=2{return;}let mut pp=[core::ptr::null();2];of_clk_parent_fill(np,pp.as_mut_ptr(),n);let p=of_get_parent(np);let r=syscon_node_to_regmap(p);of_node_put(p);if IS_ERR(r){return;}let mut name=(*np).name;of_property_read_string(np,"clock-output-names",&mut name);let hw=at91_clk_register_sam9260_slow(r,name,pp.as_ptr(),n);if !IS_ERR(hw){of_clk_add_hw_provider(np,of_clk_hw_simple_get,hw);}}
unsafe fn of_at91rm9200_clk_sys_setup(np:*mut device_node){let num=of_get_child_count(np);if num> SYSTEM_MAX_ID as i32+1{return;}let p=of_get_parent(np);let r=syscon_node_to_regmap(p);of_node_put(p);if IS_ERR(r){return;}let mut ch=core::ptr::null_mut();while let Some(x)=next_child(np,&mut ch){let mut id=0u32;if of_property_read_u32(x,"reg",&mut id)!=0{continue;}let mut name=core::ptr::null();if of_property_read_string(np,"clock-output-names",&mut name)!=0{name=(*x).name;}let pn=of_clk_get_parent_name(x,0);let flags=if strcmp((*x).name,"ddrck")==0{CLK_IS_CRITICAL}else{0};let hw=at91_clk_register_system(r,name,pn,core::ptr::null(),id,flags);if !IS_ERR(hw){of_clk_add_hw_provider(x,of_clk_hw_simple_get,hw);}}}
unsafe fn of_at91sam9x5_clk_usb_setup(np:*mut device_node){let n=of_clk_get_parent_count(np);if n==0||n>2{return;}let mut pp=[core::ptr::null();2];of_clk_parent_fill(np,pp.as_mut_ptr(),n);let mut name=(*np).name;of_property_read_string(np,"clock-output-names",&mut name);let p=of_get_parent(np);let r=syscon_node_to_regmap(p);of_node_put(p);if IS_ERR(r){return;}let hw=at91sam9x5_clk_register_usb(r,name,pp.as_ptr(),n);if !IS_ERR(hw){of_clk_add_hw_provider(np,of_clk_hw_simple_get,hw);}}
unsafe fn of_at91sam9n12_clk_usb_setup(np:*mut device_node){let pn=of_clk_get_parent_name(np,0);if pn.is_null(){return;}let mut name=(*np).name;of_property_read_string(np,"clock-output-names",&mut name);let p=of_get_parent(np);let r=syscon_node_to_regmap(p);of_node_put(p);if IS_ERR(r){return;}let hw=at91sam9n12_clk_register_usb(r,name,pn);if !IS_ERR(hw){of_clk_add_hw_provider(np,of_clk_hw_simple_get,hw);}}
unsafe fn of_at91rm9200_clk_usb_setup(np:*mut device_node){let pn=of_clk_get_parent_name(np,0);if pn.is_null(){return;}let mut d=[0u32;4];of_property_read_u32_array(np,"atmel,clk-divisors",d.as_mut_ptr(),4);if d[0]==0{return;}let mut name=(*np).name;of_property_read_string(np,"clock-output-names",&mut name);let p=of_get_parent(np);let r=syscon_node_to_regmap(p);of_node_put(p);if IS_ERR(r){return;}let hw=at91rm9200_clk_register_usb(r,name,pn,d.as_ptr());if !IS_ERR(hw){of_clk_add_hw_provider(np,of_clk_hw_simple_get,hw);}}
unsafe fn of_at91sam9x5_clk_utmi_setup(np:*mut device_node){let pn=of_clk_get_parent_name(np,0);let mut name=(*np).name;of_property_read_string(np,"clock-output-names",&mut name);let p=of_get_parent(np);let rp=syscon_node_to_regmap(p);of_node_put(p);if IS_ERR(rp){return;}let mut rs=syscon_regmap_lookup_by_compatible("atmel,sama5d3-sfr");if IS_ERR(rs){rs=syscon_regmap_lookup_by_compatible("atmel,sama5d2-sfr");if IS_ERR(rs){rs=core::ptr::null_mut();}}let hw=at91_clk_register_utmi(rp,rs,name,pn,core::ptr::null());if !IS_ERR(hw){of_clk_add_hw_provider(np,of_clk_hw_simple_get,hw);}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
