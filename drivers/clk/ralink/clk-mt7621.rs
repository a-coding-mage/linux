// SPDX-License-Identifier: GPL-2.0
/* Mediatek MT7621 Clock Driver
 * Author: Sergio Paracuellos <sergio.paracuellos@gmail.com>
 */

// Linux dependencies supplied by the surrounding kernel/Rust bindings.
use crate::*;

const SYSC_REG_SYSTEM_CONFIG0: u32 = 0x10;
const SYSC_REG_SYSTEM_CONFIG1: u32 = 0x14;
const SYSC_REG_CLKCFG0: u32 = 0x2c;
const SYSC_REG_CLKCFG1: u32 = 0x30;
const SYSC_REG_RESET_CTRL: u32 = 0x34;
const SYSC_REG_CUR_CLK_STS: u32 = 0x44;
const MEMC_REG_CPU_PLL: u32 = 0x648;
const XTAL_MODE_SEL_MASK: u32 = 0x1c0;
const CPU_CLK_SEL_MASK: u32 = 0xc0000000;
const CUR_CPU_FDIV_MASK: u32 = 0x1f00;
const CUR_CPU_FFRAC_MASK: u32 = 0x1f;
const CPU_PLL_PREDIV_MASK: u32 = 0x3000;
const CPU_PLL_FBDIV_MASK: u32 = 0x7f0;

#[repr(C)]
struct mt7621_clk_priv { sysc: *mut regmap, memc: *mut regmap }
#[repr(C)]
struct mt7621_clk { hw: clk_hw, priv_: *mut mt7621_clk_priv }
#[repr(C)]
struct mt7621_fixed_clk {
    idx: u8, name: *const c_char, parent_name: *const c_char, rate: c_ulong,
    hw: *mut clk_hw,
}
#[repr(C)]
struct mt7621_gate {
    idx: u8, name: *const c_char, parent_name: *const c_char,
    priv_: *mut mt7621_clk_priv, bit_idx: u32, hw: clk_hw,
}

static mut MT7621_GATES: [mt7621_gate; 20] = [unsafe { core::mem::zeroed() }; 20];
static mut MT7621_FIXED_CLKS: [mt7621_fixed_clk; 5] = [unsafe { core::mem::zeroed() }; 5];
static mut MT7621_CLKS_BASE: [mt7621_clk; 3] = [unsafe { core::mem::zeroed() }; 3];
static mut MT7621_CLK_EARLY: [*mut clk_hw; MT7621_CLK_MAX] = [core::ptr::null_mut(); MT7621_CLK_MAX];

unsafe fn to_mt7621_gate(hw: *mut clk_hw) -> *mut mt7621_gate {
    (hw as *mut u8).sub(core::mem::offset_of!(mt7621_gate, hw)) as *mut mt7621_gate
}
unsafe extern "C" fn mt7621_gate_enable(hw: *mut clk_hw) -> c_int {
    let g = &*to_mt7621_gate(hw); regmap_update_bits((*g.priv_).sysc, SYSC_REG_CLKCFG1, g.bit_idx, g.bit_idx)
}
unsafe extern "C" fn mt7621_gate_disable(hw: *mut clk_hw) {
    let g = &*to_mt7621_gate(hw); regmap_update_bits((*g.priv_).sysc, SYSC_REG_CLKCFG1, g.bit_idx, 0);
}
unsafe extern "C" fn mt7621_gate_is_enabled(hw: *mut clk_hw) -> c_int {
    let g = &*to_mt7621_gate(hw); let mut val = 0u32;
    if regmap_read((*g.priv_).sysc, SYSC_REG_CLKCFG1, &mut val) != 0 { return 0; }
    (val & g.bit_idx) as c_int
}

unsafe fn mt7621_gate_ops_init(dev: *mut device, sclk: *mut mt7621_gate) -> c_int {
    (*sclk).hw.init = core::ptr::null_mut();
    devm_clk_hw_register(dev, &mut (*sclk).hw)
}
unsafe fn mt7621_register_gates(dev: *mut device, data: *mut clk_hw_onecell_data,
                                priv_: *mut mt7621_clk_priv) -> c_int {
    let mut i = 0; while i < MT7621_GATES.len() {
        let s = &mut MT7621_GATES[i]; s.priv_ = priv_;
        let ret = mt7621_gate_ops_init(dev, s); if ret != 0 {
            dev_err(dev, cstr!("Couldn't register clock %s\n"), s.name); while i > 0 { i -= 1; clk_hw_unregister(&mut MT7621_GATES[i].hw); } return ret;
        }
        (*data).hws.add(s.idx as usize).write(&mut s.hw); i += 1;
    } 0
}

unsafe fn mt7621_register_fixed_clocks(dev: *mut device, data: *mut clk_hw_onecell_data) -> c_int {
    let mut i = 0; while i < MT7621_FIXED_CLKS.len() {
        let s = &mut MT7621_FIXED_CLKS[i]; s.hw = clk_hw_register_fixed_rate(dev, s.name, s.parent_name, 0, s.rate);
        if is_err(s.hw) { let ret = ptr_err(s.hw); while i > 0 { i -= 1; clk_hw_unregister_fixed_rate(MT7621_FIXED_CLKS[i].hw); } return ret; }
        (*data).hws.add(s.idx as usize).write(s.hw); i += 1;
    } 0
}

unsafe fn to_mt7621_clk(hw: *mut clk_hw) -> *mut mt7621_clk {
    (hw as *mut u8).sub(core::mem::offset_of!(mt7621_clk, hw)) as *mut mt7621_clk
}
unsafe extern "C" fn mt7621_xtal_recalc_rate(hw: *mut clk_hw, _parent_rate: c_ulong) -> c_ulong {
    let c = &*to_mt7621_clk(hw); let mut val = 0; regmap_read((*c.priv_).sysc, SYSC_REG_SYSTEM_CONFIG0, &mut val);
    let v = (val & XTAL_MODE_SEL_MASK) >> 6; if v <= 2 { 20_000_000 } else if v <= 5 { 40_000_000 } else { 25_000_000 }
}
unsafe extern "C" fn mt7621_cpu_recalc_rate(hw: *mut clk_hw, xtal_clk: c_ulong) -> c_ulong {
    let c = &*to_mt7621_clk(hw); let mut clkcfg=0; let mut curclk=0; let mut pll=0;
    regmap_read((*c.priv_).sysc, SYSC_REG_CLKCFG0, &mut clkcfg); let sel=(clkcfg&CPU_CLK_SEL_MASK)>>30;
    regmap_read((*c.priv_).sysc, SYSC_REG_CUR_CLK_STS, &mut curclk); let ffiv=(curclk&CUR_CPU_FDIV_MASK)>>8; let ffrac=curclk&CUR_CPU_FFRAC_MASK;
    let cpu = match sel { 0 => 500_000_000, 1 => { regmap_read((*c.priv_).memc, MEMC_REG_CPU_PLL, &mut pll); let fb=((pll&CPU_PLL_FBDIV_MASK)>>4)+1; let pre=(pll&CPU_PLL_PREDIV_MASK)>>12; let tbl=[0,1,2,2]; (fb as c_ulong * xtal_clk) >> tbl[pre as usize] }, _ => xtal_clk };
    cpu / ffiv as c_ulong * ffrac as c_ulong
}
unsafe extern "C" fn mt7621_bus_recalc_rate(_hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong { parent_rate / 4 }

unsafe fn mt7621_assert_device(rcdev: *mut reset_controller_dev, id: c_ulong) -> c_int { let d=to_mt7621_rst(rcdev); regmap_update_bits((*d).sysc, SYSC_REG_RESET_CTRL, 1u32.wrapping_shl(id as u32), 1u32.wrapping_shl(id as u32)) }
unsafe fn mt7621_deassert_device(rcdev: *mut reset_controller_dev, id: c_ulong) -> c_int { let d=to_mt7621_rst(rcdev); regmap_update_bits((*d).sysc, SYSC_REG_RESET_CTRL, 1u32.wrapping_shl(id as u32), 0) }
unsafe fn mt7621_reset_device(rcdev: *mut reset_controller_dev, id: c_ulong) -> c_int { let r=mt7621_assert_device(rcdev,id); if r<0 { r } else { mt7621_deassert_device(rcdev,id) } }
#[repr(C)] struct mt7621_rst { rcdev: reset_controller_dev, sysc: *mut regmap }
unsafe fn to_mt7621_rst(d:*mut reset_controller_dev)->*mut mt7621_rst { (d as *mut u8).sub(core::mem::offset_of!(mt7621_rst,rcdev)) as *mut mt7621_rst }
unsafe fn mt7621_rst_xlate(rcdev:*mut reset_controller_dev, args:*const of_phandle_args)->c_int { let id=(*args).args[0]; if id==MT7621_RST_SYS || id>=(*rcdev).nr_resets { -EINVAL } else { id as c_int } }

unsafe fn mt7621_register_early_clocks(np:*mut device_node, data:*mut clk_hw_onecell_data, priv_:*mut mt7621_clk_priv)->c_int {
    let mut i=0; while i<3 { MT7621_CLKS_BASE[i].priv_=priv_; let r=of_clk_hw_register(np,&mut MT7621_CLKS_BASE[i].hw); if r!=0 { while i>0 { i-=1; clk_hw_unregister(&mut MT7621_CLKS_BASE[i].hw); } return r; } (*data).hws.add(i).write(&mut MT7621_CLKS_BASE[i].hw); MT7621_CLK_EARLY[i]=&mut MT7621_CLKS_BASE[i].hw; i+=1; }
    while i<MT7621_CLK_MAX { MT7621_CLK_EARLY[i]=err_ptr(-EPROBE_DEFER); i+=1; } 0
}
unsafe fn mt7621_reset_init(dev:*mut device, sysc:*mut regmap)->c_int {
    let r=devm_kzalloc(dev,core::mem::size_of::<mt7621_rst>(),GFP_KERNEL) as *mut mt7621_rst; if r.is_null(){return -ENOMEM;} (*r).sysc=sysc; devm_reset_controller_register(dev,&mut (*r).rcdev)
}
unsafe extern "C" fn mt7621_clk_init(node:*mut device_node) {
    let p=kzalloc(core::mem::size_of::<mt7621_clk_priv>(),GFP_KERNEL) as *mut mt7621_clk_priv; if p.is_null(){return;}
    (*p).sysc=syscon_node_to_regmap(node); if is_err((*p).sysc){kfree(p as *mut c_void);return;}
    (*p).memc=syscon_regmap_lookup_by_phandle(node,cstr!("ralink,memctl")); if is_err((*p).memc){kfree(p as *mut c_void);return;}
    let count=3+5+20; let d=kzalloc_flex(count) as *mut clk_hw_onecell_data; if d.is_null(){kfree(p as *mut c_void);return;}
    if mt7621_register_early_clocks(node,d,p)!=0 { kfree(d as *mut c_void); kfree(p as *mut c_void); return; } (*d).num=count;
    if of_clk_add_hw_provider(node,of_clk_hw_onecell_get,d)!=0 { for i in 0..3 { clk_hw_unregister(&mut MT7621_CLKS_BASE[i].hw); } kfree(d as *mut c_void); kfree(p as *mut c_void); }
}
unsafe extern "C" fn mt7621_clk_probe(pdev:*mut platform_device)->c_int {
    let dev=&mut (*pdev).dev; let np=dev.of_node; let p=devm_kzalloc(dev,core::mem::size_of::<mt7621_clk_priv>(),GFP_KERNEL) as *mut mt7621_clk_priv; if p.is_null(){return -ENOMEM;}
    (*p).sysc=syscon_node_to_regmap(np); if is_err((*p).sysc){return ptr_err((*p).sysc);} (*p).memc=syscon_regmap_lookup_by_phandle(np,cstr!("ralink,memctl")); if is_err((*p).memc){return ptr_err((*p).memc);}
    let r=mt7621_reset_init(dev,(*p).sysc); if r!=0{return r;} let d=devm_kzalloc(dev,core::mem::size_of::<clk_hw_onecell_data>(),GFP_KERNEL) as *mut clk_hw_onecell_data; if d.is_null(){return -ENOMEM;} (*d).num=28;
    for i in 0..3 {(*d).hws.add(i).write(MT7621_CLK_EARLY[i]);} let r=mt7621_register_fixed_clocks(dev,d); if r!=0{return r;} mt7621_register_gates(dev,d,p)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
