// SPDX-License-Identifier: GPL-2.0
/* Clock driver for DA8xx/AM17xx/AM18xx/OMAP-L13x CFGCHIP */

// Linux dependencies are supplied externally.

const DA8XX_GATE_CLOCK_IS_DIV4P5: u32 = 1 << 1;

#[repr(C)]
struct da8xx_cfgchip_gate_clk_info { name: *const core::ffi::c_char, cfgchip: u32, bit: u32, flags: u32 }
#[repr(C)]
struct da8xx_cfgchip_gate_clk { hw: clk_hw, regmap: *mut regmap, reg: u32, mask: u32 }

unsafe fn gate_clk(hw: *mut clk_hw) -> *mut da8xx_cfgchip_gate_clk { container_of(hw) }

unsafe fn da8xx_cfgchip_gate_clk_enable(hw: *mut clk_hw) -> i32 {
    let clk = gate_clk(hw); regmap_write_bits((*clk).regmap, (*clk).reg, (*clk).mask, (*clk).mask)
}
unsafe fn da8xx_cfgchip_gate_clk_disable(hw: *mut clk_hw) { let clk = gate_clk(hw); regmap_write_bits((*clk).regmap, (*clk).reg, (*clk).mask, 0); }
unsafe fn da8xx_cfgchip_gate_clk_is_enabled(hw: *mut clk_hw) -> i32 {
    let clk = gate_clk(hw); let mut val = 0; regmap_read((*clk).regmap, (*clk).reg, &mut val); if val & (*clk).mask != 0 { 1 } else { 0 }
}
unsafe fn da8xx_cfgchip_div4p5_recalc_rate(_hw: *mut clk_hw, parent_rate: u64) -> u64 { parent_rate.wrapping_mul(2) / 9 }

static da8xx_cfgchip_gate_clk_ops: clk_ops = clk_ops { enable: Some(da8xx_cfgchip_gate_clk_enable), disable: Some(da8xx_cfgchip_gate_clk_disable), is_enabled: Some(da8xx_cfgchip_gate_clk_is_enabled), ..clk_ops::empty() };
static da8xx_cfgchip_div4p5_clk_ops: clk_ops = clk_ops { enable: Some(da8xx_cfgchip_gate_clk_enable), disable: Some(da8xx_cfgchip_gate_clk_disable), is_enabled: Some(da8xx_cfgchip_gate_clk_is_enabled), recalc_rate: Some(da8xx_cfgchip_div4p5_recalc_rate), ..clk_ops::empty() };

unsafe fn da8xx_cfgchip_gate_clk_register(dev: *mut device, info: *const da8xx_cfgchip_gate_clk_info, regmap: *mut regmap) -> *mut da8xx_cfgchip_gate_clk {
    let parent = devm_clk_get(dev, core::ptr::null()); if is_err(parent) { return err_cast(parent); }
    let gate = devm_kzalloc(dev, core::mem::size_of::<da8xx_cfgchip_gate_clk>(), GFP_KERNEL) as *mut da8xx_cfgchip_gate_clk;
    if gate.is_null() { return err_ptr(-ENOMEM); }
    let mut init = clk_init_data::default(); init.name = (*info).name; init.ops = if (*info).flags & DA8XX_GATE_CLOCK_IS_DIV4P5 != 0 { &da8xx_cfgchip_div4p5_clk_ops } else { &da8xx_cfgchip_gate_clk_ops }; init.num_parents = 1; init.flags = 0;
    (*gate).hw.init = &init; (*gate).regmap = regmap; (*gate).reg = (*info).cfgchip; (*gate).mask = (*info).bit;
    let ret = devm_clk_hw_register(dev, &mut (*gate).hw); if ret < 0 { return err_ptr(ret); } gate
}

static da8xx_tbclksync_info: da8xx_cfgchip_gate_clk_info = da8xx_cfgchip_gate_clk_info { name: b"ehrpwm_tbclk\0".as_ptr() as _, cfgchip: CFGCHIP(1), bit: CFGCHIP1_TBCLKSYNC, flags: 0 };
unsafe fn da8xx_cfgchip_register_tbclk(dev: *mut device, regmap: *mut regmap) -> i32 { let gate = da8xx_cfgchip_gate_clk_register(dev, &da8xx_tbclksync_info, regmap); if is_err(gate) { return ptr_err(gate); } clk_hw_register_clkdev(&mut (*gate).hw, b"tbclk\0".as_ptr() as _, b"ehrpwm.0\0".as_ptr() as _); clk_hw_register_clkdev(&mut (*gate).hw, b"tbclk\0".as_ptr() as _, b"ehrpwm.1\0".as_ptr() as _); 0 }
static da8xx_div4p5ena_info: da8xx_cfgchip_gate_clk_info = da8xx_cfgchip_gate_clk_info { name: b"div4.5\0".as_ptr() as _, cfgchip: CFGCHIP(3), bit: CFGCHIP3_DIV45PENA, flags: DA8XX_GATE_CLOCK_IS_DIV4P5 };
unsafe fn da8xx_cfgchip_register_div4p5(dev: *mut device, regmap: *mut regmap) -> i32 { ptr_err_or_zero(da8xx_cfgchip_gate_clk_register(dev, &da8xx_div4p5ena_info, regmap)) }
unsafe fn of_da8xx_cfgchip_gate_clk_init(dev: *mut device, info: *const da8xx_cfgchip_gate_clk_info, regmap: *mut regmap) -> i32 { let gate=da8xx_cfgchip_gate_clk_register(dev,info,regmap); if is_err(gate){return ptr_err(gate)} devm_of_clk_add_hw_provider(dev,of_clk_hw_simple_get,gate as _) }
unsafe fn of_da8xx_tbclksync_init(d:*mut device,r:*mut regmap)->i32{of_da8xx_cfgchip_gate_clk_init(d,&da8xx_tbclksync_info,r)}
unsafe fn of_da8xx_div4p5ena_init(d:*mut device,r:*mut regmap)->i32{of_da8xx_cfgchip_gate_clk_init(d,&da8xx_div4p5ena_info,r)}

#[repr(C)] struct da8xx_cfgchip_mux_clk_info { name:*const i8,parent0:*const i8,parent1:*const i8,cfgchip:u32,bit:u32 }
#[repr(C)] struct da8xx_cfgchip_mux_clk { hw:clk_hw,regmap:*mut regmap,reg:u32,mask:u32 }
unsafe fn mux_clk(hw:*mut clk_hw)->*mut da8xx_cfgchip_mux_clk{container_of(hw)}
unsafe fn da8xx_cfgchip_mux_clk_set_parent(hw:*mut clk_hw,index:u8)->i32{let c=mux_clk(hw);regmap_write_bits((*c).regmap,(*c).reg,(*c).mask,if index!=0{(*c).mask}else{0})}
unsafe fn da8xx_cfgchip_mux_clk_get_parent(hw:*mut clk_hw)->u8{let c=mux_clk(hw);let mut v=0;regmap_read((*c).regmap,(*c).reg,&mut v);if v&(*c).mask!=0{1}else{0}}
static da8xx_cfgchip_mux_clk_ops:clk_ops=clk_ops{determine_rate:Some(clk_hw_determine_rate_no_reparent),set_parent:Some(da8xx_cfgchip_mux_clk_set_parent),get_parent:Some(da8xx_cfgchip_mux_clk_get_parent),..clk_ops::empty()};
unsafe fn da8xx_cfgchip_mux_clk_register(dev:*mut device,info:*const da8xx_cfgchip_mux_clk_info,regmap:*mut regmap)->*mut da8xx_cfgchip_mux_clk{let m=devm_kzalloc(dev,core::mem::size_of::<da8xx_cfgchip_mux_clk>(),GFP_KERNEL)as*mut _;if m.is_null(){return err_ptr(-ENOMEM)}let mut i=clk_init_data::default();i.name=(*info).name;i.ops=&da8xx_cfgchip_mux_clk_ops;i.num_parents=2;(*m).hw.init=&i;(*m).regmap=regmap;(*m).reg=(*info).cfgchip;(*m).mask=(*info).bit;let r=devm_clk_hw_register(dev,&mut(*m).hw);if r<0{err_ptr(r)}else{m}}

#[repr(C)] struct da8xx_usb0_clk48 { hw:clk_hw,fck:*mut clk,regmap:*mut regmap }
unsafe fn usb0(hw:*mut clk_hw)->*mut da8xx_usb0_clk48{container_of(hw)}
unsafe fn da8xx_usb0_clk48_prepare(hw:*mut clk_hw)->i32{clk_prepare((*usb0(hw)).fck)}
unsafe fn da8xx_usb0_clk48_unprepare(hw:*mut clk_hw){clk_unprepare((*usb0(hw)).fck)}
unsafe fn da8xx_usb0_clk48_enable(hw:*mut clk_hw)->i32{let u=usb0(hw);let mask=CFGCHIP2_RESET|CFGCHIP2_PHYPWRDN|CFGCHIP2_PHY_PLLON;let val=CFGCHIP2_PHY_PLLON;clk_enable((*u).fck);regmap_write_bits((*u).regmap,CFGCHIP(2),mask,val);let mut v=val;let r=regmap_read_poll_timeout((*u).regmap,CFGCHIP(2),&mut v,v&CFGCHIP2_PHYCLKGD!=0,0,500000);clk_disable((*u).fck);r}
unsafe fn da8xx_usb0_clk48_disable(hw:*mut clk_hw){let u=usb0(hw);regmap_write_bits((*u).regmap,CFGCHIP(2),CFGCHIP2_PHYPWRDN,CFGCHIP2_PHYPWRDN)}
unsafe fn da8xx_usb0_clk48_is_enabled(hw:*mut clk_hw)->i32{let u=usb0(hw);let mut v=0;regmap_read((*u).regmap,CFGCHIP(2),&mut v);if v&CFGCHIP2_PHYCLKGD!=0{1}else{0}}
unsafe fn da8xx_usb0_clk48_recalc_rate(hw:*mut clk_hw,parent_rate:u64)->u64{let u=usb0(hw);let val=match parent_rate{12000000=>CFGCHIP2_REFFREQ_12MHZ,13000000=>CFGCHIP2_REFFREQ_13MHZ,19200000=>CFGCHIP2_REFFREQ_19_2MHZ,20000000=>CFGCHIP2_REFFREQ_20MHZ,24000000=>CFGCHIP2_REFFREQ_24MHZ,26000000=>CFGCHIP2_REFFREQ_26MHZ,38400000=>CFGCHIP2_REFFREQ_38_4MHZ,40000000=>CFGCHIP2_REFFREQ_40MHZ,48000000=>CFGCHIP2_REFFREQ_48MHZ,_=>return 0};regmap_write_bits((*u).regmap,CFGCHIP(2),CFGCHIP2_REFFREQ_MASK,val);48000000}
unsafe fn da8xx_usb0_clk48_determine_rate(_hw:*mut clk_hw,req:*mut clk_rate_request)->i32{(*req).rate=48000000;0}
unsafe fn da8xx_usb0_clk48_set_parent(hw:*mut clk_hw,index:u8)->i32{let u=usb0(hw);regmap_write_bits((*u).regmap,CFGCHIP(2),CFGCHIP2_USB2PHYCLKMUX,if index!=0{CFGCHIP2_USB2PHYCLKMUX}else{0})}
unsafe fn da8xx_usb0_clk48_get_parent(hw:*mut clk_hw)->u8{let u=usb0(hw);let mut v=0;regmap_read((*u).regmap,CFGCHIP(2),&mut v);if v&CFGCHIP2_USB2PHYCLKMUX!=0{1}else{0}}
#[repr(C)] struct da8xx_usb1_clk48{hw:clk_hw,regmap:*mut regmap}
unsafe fn usb1(hw:*mut clk_hw)->*mut da8xx_usb1_clk48{container_of(hw)}
unsafe fn da8xx_usb1_clk48_set_parent(hw:*mut clk_hw,index:u8)->i32{let u=usb1(hw);regmap_write_bits((*u).regmap,CFGCHIP(2),CFGCHIP2_USB1PHYCLKMUX,if index!=0{CFGCHIP2_USB1PHYCLKMUX}else{0})}
unsafe fn da8xx_usb1_clk48_get_parent(hw:*mut clk_hw)->u8{let u=usb1(hw);let mut v=0;regmap_read((*u).regmap,CFGCHIP(2),&mut v);if v&CFGCHIP2_USB1PHYCLKMUX!=0{1}else{0}}
unsafe fn da8xx_cfgchip_register_usb0_clk48(dev:*mut device,r:*mut regmap)->*mut da8xx_usb0_clk48{let u=devm_kzalloc(dev,core::mem::size_of::<da8xx_usb0_clk48>(),GFP_KERNEL)as*mut _;if u.is_null(){return err_ptr(-ENOMEM)};(*u).fck=devm_clk_get(dev,b"fck\0".as_ptr()as _);if is_err((*u).fck){return err_cast((*u).fck)};(*u).regmap=r;let mut i=clk_init_data::default();i.name=b"usb0_clk48\0".as_ptr()as _;i.num_parents=2;(*u).hw.init=&i;let x=devm_clk_hw_register(dev,&mut(*u).hw);if x<0{err_ptr(x)}else{u}}
unsafe fn da8xx_cfgchip_register_usb1_clk48(dev:*mut device,r:*mut regmap)->*mut da8xx_usb1_clk48{let u=devm_kzalloc(dev,core::mem::size_of::<da8xx_usb1_clk48>(),GFP_KERNEL)as*mut _;if u.is_null(){return err_ptr(-ENOMEM)};(*u).regmap=r;let mut i=clk_init_data::default();i.name=b"usb1_clk48\0".as_ptr()as _;i.num_parents=2;(*u).hw.init=&i;let x=devm_clk_hw_register(dev,&mut(*u).hw);if x<0{err_ptr(x)}else{u}}
unsafe fn da8xx_cfgchip_register_usb_phy_clk(dev:*mut device,r:*mut regmap)->i32{let a=da8xx_cfgchip_register_usb0_clk48(dev,r);if is_err(a){return ptr_err(a)}let b=da8xx_cfgchip_register_usb1_clk48(dev,r);if is_err(b){return ptr_err(b)};clk_hw_register_clkdev(&mut(*a).hw,b"usb0_clk48\0".as_ptr()as _,b"da8xx-usb-phy\0".as_ptr()as _);clk_hw_register_clkdev(&mut(*b).hw,b"usb1_clk48\0".as_ptr()as _,b"da8xx-usb-phy\0".as_ptr()as _);0}
unsafe fn of_da8xx_usb_phy_clk_init(dev:*mut device,r:*mut regmap)->i32{da8xx_cfgchip_register_usb_phy_clk(dev,r)}
unsafe fn da8xx_cfgchip_probe(pdev:*mut platform_device)->i32{let dev=&mut(*pdev).dev;let r=syscon_node_to_regmap(of_get_parent((*dev).of_node));if is_err_or_null(r){return if r.is_null(){-ENOENT}else{ptr_err(r)}};da8xx_cfgchip_register_usb_phy_clk(dev,r)}
static da8xx_cfgchip_driver:platform_driver=platform_driver::new("da8xx-cfgchip-clk",da8xx_cfgchip_probe);
unsafe fn da8xx_cfgchip_driver_init()->i32{platform_driver_register(&da8xx_cfgchip_driver)}
// has to be postcore_initcall because PSC devices depend on the async3 clock
postcore_initcall!(da8xx_cfgchip_driver_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
