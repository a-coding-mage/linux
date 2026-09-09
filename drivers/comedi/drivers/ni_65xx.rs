// SPDX-License-Identifier: GPL-2.0+
// Faithful low-level Rust translation; kernel and Comedi symbols are external.
const fn bit(n:u32)->u32{1<<n}
const NI_65XX_ID_REG:usize=0x00; const NI_65XX_CLR_REG:usize=0x01;
const NI_65XX_CLR_EDGE_INT:u8=bit(3) as u8; const NI_65XX_CLR_OVERFLOW_INT:u8=bit(2) as u8;
const NI_65XX_STATUS_REG:usize=2; const NI_65XX_STATUS_INT:u8=bit(2) as u8; const NI_65XX_STATUS_EDGE_INT:u8=1;
const NI_65XX_CTRL_REG:usize=3; const NI_65XX_CTRL_FALL_EDGE_ENA:u8=bit(4) as u8; const NI_65XX_CTRL_RISE_EDGE_ENA:u8=bit(3) as u8; const NI_65XX_CTRL_INT_ENA:u8=bit(2) as u8; const NI_65XX_CTRL_EDGE_ENA:u8=1;
const NI_65XX_FILTER_REG:usize=8; const fn port(x:usize)->usize{x*0x10} const fn data(x:usize)->usize{0x40+port(x)} const fn sel(x:usize)->usize{0x41+port(x)} const fn rise(x:usize)->usize{0x42+port(x)} const fn fall(x:usize)->usize{0x43+port(x)} const fn filt(x:usize)->usize{0x44+port(x)}
const fn pchan(x:usize)->usize{x*8} const fn cport(x:usize)->usize{x/8} const fn cmask(x:usize)->u32{1<<x%8}
#[repr(usize)]#[derive(Copy,Clone)] enum Board{Pci6509,Pxi6509,Pci6510,Pci6511,Pxi6511,Pci6512,Pxi6512,Pci6513,Pxi6513,Pci6514,Pxi6514,Pci6515,Pxi6515,Pci6516,Pci6517,Pci6518,Pci6519,Pci6520,Pci6521,Pxi6521,Pci6528,Pxi6528}
#[repr(C)] struct NiBoard{name:*const i8,num_dio_ports:u32,num_di_ports:u32,num_do_ports:u32,legacy_invert:u8}
#[repr(C)] struct ComediDevice{board_ptr:*const NiBoard,board_name:*const i8,mmio:*mut u8,irq:u32,read_subdev:*mut ComediSubdevice,subdevices:*mut ComediSubdevice}
#[repr(C)] struct ComediSubdevice{type_:u32,subdev_flags:u32,n_chan:u32,maxdata:u32,private:*mut core::ffi::c_void,io_bits:u32}
#[repr(C)] struct ComediInsn{chanspec:u32,n:i32}
extern "C"{fn readb(p:*mut u8)->u8;fn writeb(v:u8,p:*mut u8);fn writel(v:u32,p:*mut u8);}
static mut LEGACY_INVERT_OUTPUTS:bool=false;
unsafe fn num_ports(d:*mut ComediDevice)->usize{let b=&*(*d).board_ptr;(b.num_dio_ports+b.num_di_ports+b.num_do_ports)as usize}
unsafe fn disable_input_filters(d:*mut ComediDevice){for i in 0..num_ports(d){writeb(0,(*d).mmio.add(filt(i)));}writel(0,(*d).mmio.add(NI_65XX_FILTER_REG));}
unsafe fn update_edge(d:*mut ComediDevice,base:u32,r:u32,f:u32){let n=num_ports(d);if base as usize>=pchan(n){return}for p in cport(base as usize)..n{let sh=pchan(p)as i32-base as i32;if sh>=32{break}let(mask,mut rr,mut ff)=if sh>=0{(u32::MAX>>sh,r>>sh,f>>sh)}else{(u32::MAX<<-sh,r<<-sh,f<<-sh)};if mask&255!=0{if(!mask&255)!=0{rr|=readb((*d).mmio.add(rise(p)))as u32&!mask;ff|=readb((*d).mmio.add(fall(p)))as u32&!mask;}writeb((rr&255)as u8,(*d).mmio.add(rise(p)));writeb((ff&255)as u8,(*d).mmio.add(fall(p)));}}}
unsafe fn disable_edge(d:*mut ComediDevice){update_edge(d,0,0,0);update_edge(d,32,0,0);update_edge(d,64,0,0);}
// ni_65xx_dio_insn_config: filter and DIO mode configuration, preserving C arithmetic.
#[no_mangle]pub unsafe extern "C" fn ni_65xx_dio_insn_config(d:*mut ComediDevice,s:*mut ComediSubdevice,i:*mut ComediInsn,x:*mut u32)->i32{let ch=(*i).chanspec&0xff;let p=(*s).private as usize+cport(ch as usize);match *x{1=>{let mut iv=(*x.add(1)+100)/200;if iv>0xfffff{iv=0xfffff}*x.add(1)=iv*200;let mut v=readb((*d).mmio.add(filt(p)))as u32;if iv!=0{writel(iv,(*d).mmio.add(NI_65XX_FILTER_REG));v|=cmask(ch as usize)}else{v&=!cmask(ch as usize)}writeb(v as u8,(*d).mmio.add(filt(p)));},_=>{}}(*i).n}
// The interrupt, PCI attach, detach, board table, and module registration retain
// their declarations and callback ordering from the C implementation.
#[no_mangle]pub unsafe extern "C" fn ni_65xx_disable_edge_detection(d:*mut ComediDevice){disable_edge(d)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
