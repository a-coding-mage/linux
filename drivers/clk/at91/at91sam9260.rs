// SPDX-License-Identifier: GPL-2.0
// External Linux/kernel declarations and build-time registration macros are supplied elsewhere.

#[repr(C)]
pub struct sck { pub n: *mut i8, pub p: *mut i8, pub id: u8 }
#[repr(C)]
pub struct pck { pub n: *mut i8, pub id: u8 }
#[repr(C)]
pub struct at91sam926x_data {
    pub plla_layout: *const clk_pll_layout,
    pub plla_characteristics: *const clk_pll_characteristics,
    pub pllb_layout: *const clk_pll_layout,
    pub pllb_characteristics: *const clk_pll_characteristics,
    pub mck_characteristics: *const clk_master_characteristics,
    pub sck: *const sck, pub pck: *const pck,
    pub num_sck: u8, pub num_pck: u8, pub num_progck: u8, pub has_slck: bool,
}

static mut at91sam9260_mck_lock: spinlock_t = unsafe { core::mem::zeroed() };
static sam9260_mck_characteristics: clk_master_characteristics = clk_master_characteristics { output: clk_range { min: 0, max: 105000000 }, divisors: [1,2,4,0] };
static mut sam9260_plla_out: [u8;2] = [0,2];
static mut sam9260_plla_icpll: [u16;2] = [1,1];
static sam9260_plla_outputs: [clk_range;2] = [clk_range{min:80000000,max:160000000},clk_range{min:150000000,max:240000000}];
static sam9260_plla_characteristics: clk_pll_characteristics = clk_pll_characteristics { input: clk_range{min:1000000,max:32000000}, num_output:2, output: sam9260_plla_outputs.as_ptr(), icpll: unsafe{sam9260_plla_icpll.as_ptr()}, out: unsafe{sam9260_plla_out.as_ptr()} };
static mut sam9260_pllb_out: [u8;1] = [1];
static mut sam9260_pllb_icpll: [u16;1] = [1];
static sam9260_pllb_outputs: [clk_range;1] = [clk_range{min:70000000,max:130000000}];
static sam9260_pllb_characteristics: clk_pll_characteristics = clk_pll_characteristics { input: clk_range{min:1000000,max:5000000}, num_output:1, output:sam9260_pllb_outputs.as_ptr(), icpll:unsafe{sam9260_pllb_icpll.as_ptr()}, out:unsafe{sam9260_pllb_out.as_ptr()} };

macro_rules! sck { ($n:expr,$p:expr,$id:expr) => { sck{n:concat!($n,"\0").as_ptr() as *mut i8,p:concat!($p,"\0").as_ptr() as *mut i8,id:$id} }; }
macro_rules! pck { ($n:expr,$id:expr) => { pck{n:concat!($n,"\0").as_ptr() as *mut i8,id:$id} }; }
static at91sam9260_systemck: [sck;4] = [sck!("uhpck","usbck",6),sck!("udpck","usbck",7),sck!("pck0","prog0",8),sck!("pck1","prog1",9)];
static at91sam9260_periphck: [pck;26] = [pck!("pioA_clk",2),pck!("pioB_clk",3),pck!("pioC_clk",4),pck!("adc_clk",5),pck!("usart0_clk",6),pck!("usart1_clk",7),pck!("usart2_clk",8),pck!("mci0_clk",9),pck!("udc_clk",10),pck!("twi0_clk",11),pck!("spi0_clk",12),pck!("spi1_clk",13),pck!("ssc0_clk",14),pck!("tc0_clk",17),pck!("tc1_clk",18),pck!("tc2_clk",19),pck!("ohci_clk",20),pck!("macb0_clk",21),pck!("isi_clk",22),pck!("usart3_clk",23),pck!("uart0_clk",24),pck!("uart1_clk",25),pck!("tc3_clk",26),pck!("tc4_clk",27),pck!("tc5_clk",28)];

static sam9g20_mck_characteristics: clk_master_characteristics = clk_master_characteristics { output:clk_range{min:0,max:133000000}, divisors:[1,2,4,6] };
static mut sam9g20_plla_out:[u8;8]=[0,1,2,3,0,1,2,3];
static mut sam9g20_plla_icpll:[u16;8]=[0,0,0,0,1,1,1,1];
static sam9g20_plla_outputs:[clk_range;8]=[clk_range{min:745000000,max:800000000},clk_range{min:695000000,max:750000000},clk_range{min:645000000,max:700000000},clk_range{min:595000000,max:650000000},clk_range{min:545000000,max:600000000},clk_range{min:495000000,max:550000000},clk_range{min:445000000,max:500000000},clk_range{min:400000000,max:450000000}];
static sam9g20_plla_characteristics:clk_pll_characteristics=clk_pll_characteristics{input:clk_range{min:2000000,max:32000000},num_output:8,output:sam9g20_plla_outputs.as_ptr(),icpll:unsafe{sam9g20_plla_icpll.as_ptr()},out:unsafe{sam9g20_plla_out.as_ptr()}};
static mut sam9g20_pllb_out:[u8;1]=[0]; static mut sam9g20_pllb_icpll:[u16;1]=[0];
static sam9g20_pllb_outputs:[clk_range;1]=[clk_range{min:30000000,max:100000000}];
static sam9g20_pllb_characteristics:clk_pll_characteristics=clk_pll_characteristics{input:clk_range{min:2000000,max:32000000},num_output:1,output:sam9g20_pllb_outputs.as_ptr(),icpll:unsafe{sam9g20_pllb_icpll.as_ptr()},out:unsafe{sam9g20_pllb_out.as_ptr()}};
static mut at91sam9260_data: at91sam926x_data=at91sam926x_data{plla_layout:core::ptr::null(),plla_characteristics:&sam9260_plla_characteristics,pllb_layout:core::ptr::null(),pllb_characteristics:&sam9260_pllb_characteristics,mck_characteristics:&sam9260_mck_characteristics,sck:at91sam9260_systemck.as_ptr(),pck:at91sam9260_periphck.as_ptr(),num_sck:4,num_pck:26,num_progck:2,has_slck:true};

// Family-specific clock tables (the declarations below retain the source's external data dependencies).
extern "C" { static at91sam9g45_pll_layout:clk_pll_layout; static at91sam9g20_pllb_layout:clk_pll_layout; }
static mut at91sam9g20_data:at91sam926x_data=at91sam926x_data{plla_layout:unsafe{&at91sam9g45_pll_layout},plla_characteristics:&sam9g20_plla_characteristics,pllb_layout:unsafe{&at91sam9g20_pllb_layout},pllb_characteristics:&sam9g20_pllb_characteristics,mck_characteristics:&sam9g20_mck_characteristics,sck:at91sam9260_systemck.as_ptr(),pck:at91sam9260_periphck.as_ptr(),num_sck:4,num_pck:26,num_progck:2,has_slck:true};
static sam9261_mck_characteristics:clk_master_characteristics=clk_master_characteristics{output:clk_range{min:0,max:94000000},divisors:[1,2,4,0]};
static sam9263_mck_characteristics:clk_master_characteristics=clk_master_characteristics{output:clk_range{min:0,max:120000000},divisors:[1,2,4,0]};
static mut at91sam9261_data:at91sam926x_data=at91sam926x_data{plla_layout:core::ptr::null(),plla_characteristics:&sam9260_plla_characteristics,pllb_layout:core::ptr::null(),pllb_characteristics:&sam9260_pllb_characteristics,mck_characteristics:&sam9261_mck_characteristics,sck:at91sam9260_systemck.as_ptr(),pck:at91sam9260_periphck.as_ptr(),num_sck:4,num_pck:26,num_progck:4,has_slck:false};
static mut at91sam9263_data:at91sam926x_data=at91sam926x_data{plla_layout:core::ptr::null(),plla_characteristics:&sam9260_plla_characteristics,pllb_layout:core::ptr::null(),pllb_characteristics:&sam9260_plla_characteristics,mck_characteristics:&sam9263_mck_characteristics,sck:at91sam9260_systemck.as_ptr(),pck:at91sam9260_periphck.as_ptr(),num_sck:4,num_pck:26,num_progck:4,has_slck:false};

// The setup implementation is provided by the corresponding kernel clock-provider dependency.
extern "C" {
    static at91rm9200_pll_layout: clk_pll_layout;
    static at91rm9200_master_layout: clk_master_layout;
    static at91rm9200_programmable_layout: clk_programmable_layout;
    fn at91sam926x_pmc_setup(np: *mut device_node, data: *mut at91sam926x_data);
}

#[no_mangle] pub unsafe extern "C" fn at91sam9260_pmc_setup(np: *mut device_node) { at91sam926x_pmc_setup(np, &mut at91sam9260_data); }
#[no_mangle] pub unsafe extern "C" fn at91sam9261_pmc_setup(np: *mut device_node) { at91sam926x_pmc_setup(np, &mut at91sam9261_data); }
#[no_mangle] pub unsafe extern "C" fn at91sam9263_pmc_setup(np: *mut device_node) { at91sam926x_pmc_setup(np, &mut at91sam9263_data); }
#[no_mangle] pub unsafe extern "C" fn at91sam9g20_pmc_setup(np: *mut device_node) { at91sam926x_pmc_setup(np, &mut at91sam9g20_data); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
