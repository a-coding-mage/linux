


$2{
  ATOM_MAJOR_VERSION        =0x0003,
  ATOM_MINOR_VERSION        =0x0003,
};


    $2: $1;


    $2: $1;


    $2: $1;



$2{
  ATOM_CRTC1      =0,
  ATOM_CRTC2      =1,
  ATOM_CRTC3      =2,
  ATOM_CRTC4      =3,
  ATOM_CRTC5      =4,
  ATOM_CRTC6      =5,
  ATOM_CRTC_INVALID  =0xff,
};

$2{
  ATOM_PPLL0          =2,
  ATOM_GCK_DFS        =8,
  ATOM_FCH_CLK        =9,
  ATOM_DP_DTO         =11,
  ATOM_COMBOPHY_PLL0  =20,
  ATOM_COMBOPHY_PLL1  =21,
  ATOM_COMBOPHY_PLL2  =22,
  ATOM_COMBOPHY_PLL3  =23,
  ATOM_COMBOPHY_PLL4  =24,
  ATOM_COMBOPHY_PLL5  =25,
  ATOM_PPLL_INVALID   =0xff,
};


$2{
  ASIC_INT_DIG1_ENCODER_ID  =0x03,
  ASIC_INT_DIG2_ENCODER_ID  =0x09,
  ASIC_INT_DIG3_ENCODER_ID  =0x0a,
  ASIC_INT_DIG4_ENCODER_ID  =0x0b,
  ASIC_INT_DIG5_ENCODER_ID  =0x0c,
  ASIC_INT_DIG6_ENCODER_ID  =0x0d,
  ASIC_INT_DIG7_ENCODER_ID  =0x0e,
};


$2
{
  ATOM_ENCODER_MODE_DP          =0,
  ATOM_ENCODER_MODE_DP_SST      =0,
  ATOM_ENCODER_MODE_LVDS        =1,
  ATOM_ENCODER_MODE_DVI         =2,
  ATOM_ENCODER_MODE_HDMI        =3,
  ATOM_ENCODER_MODE_DP_AUDIO    =5,
  ATOM_ENCODER_MODE_DP_MST      =5,
  ATOM_ENCODER_MODE_CRT         =15,
  ATOM_ENCODER_MODE_DVO         =16,
};

$2{
  ENCODER_REFCLK_SRC_P1PLL      =0,
  ENCODER_REFCLK_SRC_P2PLL      =1,
  ENCODER_REFCLK_SRC_P3PLL      =2,
  ENCODER_REFCLK_SRC_EXTCLK     =3,
  ENCODER_REFCLK_SRC_INVALID    =0xff,
};

$2{
  ATOM_SCALER_DISABLE          =0,  
  ATOM_SCALER_CENTER           =1,  
  ATOM_SCALER_EXPANSION        =2,  
};

$2{
  ATOM_DISABLE             = 0,
  ATOM_ENABLE              = 1,
  ATOM_INIT                = 7,
  ATOM_GET_STATUS          = 8,
};

$2{
  ATOM_LCD_BL_OFF                = 2,
  ATOM_LCD_BL_OM                 = 3,
  ATOM_LCD_BL_BRIGHTNESS_CONTROL = 4,
  ATOM_LCD_SELFTEST_START        = 5,
  ATOM_LCD_SELFTEST_STOP         = 6,
};

$2{
  ATOM_SS_CENTER_OR_DOWN_MODE_MASK  = 0x01,
  ATOM_SS_DOWN_SPREAD_MODE          = 0x00,
  ATOM_SS_CENTRE_SPREAD_MODE        = 0x01,
  ATOM_INT_OR_EXT_SS_MASK           = 0x02,
  ATOM_INTERNAL_SS_MASK             = 0x00,
  ATOM_EXTERNAL_SS_MASK             = 0x02,
};


$2{
  PANEL_BPC_UNDEFINE     =0x00,
  PANEL_6BIT_PER_COLOR   =0x01,
  PANEL_8BIT_PER_COLOR   =0x02,
  PANEL_10BIT_PER_COLOR  =0x03,
  PANEL_12BIT_PER_COLOR  =0x04,
  PANEL_16BIT_PER_COLOR  =0x05,
};


$2
{
  VOLTAGE_TYPE_VDDC = 1,
  VOLTAGE_TYPE_MVDDC = 2,
  VOLTAGE_TYPE_MVDDQ = 3,
  VOLTAGE_TYPE_VDDCI = 4,
  VOLTAGE_TYPE_VDDGFX = 5,
  VOLTAGE_TYPE_PCC = 6,
  VOLTAGE_TYPE_MVPP = 7,
  VOLTAGE_TYPE_LEDDPM = 8,
  VOLTAGE_TYPE_PCC_MVDD = 9,
  VOLTAGE_TYPE_PCIE_VDDC = 10,
  VOLTAGE_TYPE_PCIE_VDDR = 11,
  VOLTAGE_TYPE_GENERIC_I2C_1 = 0x11,
  VOLTAGE_TYPE_GENERIC_I2C_2 = 0x12,
  VOLTAGE_TYPE_GENERIC_I2C_3 = 0x13,
  VOLTAGE_TYPE_GENERIC_I2C_4 = 0x14,
  VOLTAGE_TYPE_GENERIC_I2C_5 = 0x15,
  VOLTAGE_TYPE_GENERIC_I2C_6 = 0x16,
  VOLTAGE_TYPE_GENERIC_I2C_7 = 0x17,
  VOLTAGE_TYPE_GENERIC_I2C_8 = 0x18,
  VOLTAGE_TYPE_GENERIC_I2C_9 = 0x19,
  VOLTAGE_TYPE_GENERIC_I2C_10 = 0x1A,
};

$2 {
  ATOM_DGPU_VRAM_TYPE_GDDR5 = 0x50,
  ATOM_DGPU_VRAM_TYPE_HBM2  = 0x60,
  ATOM_DGPU_VRAM_TYPE_HBM2E = 0x61,
  ATOM_DGPU_VRAM_TYPE_GDDR6 = 0x70,
  ATOM_DGPU_VRAM_TYPE_HBM3 = 0x80,
	ATOM_DGPU_VRAM_TYPE_HBM3E = 0x81,
};

$2{
  DP_VS_LEVEL0_PREEMPH_LEVEL0 = 0x00,
  DP_VS_LEVEL1_PREEMPH_LEVEL0 = 0x01,
  DP_VS_LEVEL2_PREEMPH_LEVEL0 = 0x02,
  DP_VS_LEVEL3_PREEMPH_LEVEL0 = 0x03,
  DP_VS_LEVEL0_PREEMPH_LEVEL1 = 0x08,
  DP_VS_LEVEL1_PREEMPH_LEVEL1 = 0x09,
  DP_VS_LEVEL2_PREEMPH_LEVEL1 = 0x0a,
  DP_VS_LEVEL0_PREEMPH_LEVEL2 = 0x10,
  DP_VS_LEVEL1_PREEMPH_LEVEL2 = 0x11,
  DP_VS_LEVEL0_PREEMPH_LEVEL3 = 0x18,
};





$2{
  OFFSET_TO_ATOM_ROM_HEADER_POINTER          = 0x00000048,
  OFFSET_TO_ATOM_ROM_IMAGE_SIZE              = 0x00000002,
  OFFSET_TO_ATOMBIOS_ASIC_BUS_MEM_TYPE       = 0x94,
  MAXSIZE_OF_ATOMBIOS_ASIC_BUS_MEM_TYPE      = 20,  
  OFFSET_TO_GET_ATOMBIOS_NUMBER_OF_STRINGS   = 0x2f,
  OFFSET_TO_GET_ATOMBIOS_STRING_START        = 0x6e,
  OFFSET_TO_VBIOS_PART_NUMBER                = 0x80,
  OFFSET_TO_VBIOS_DATE                       = 0x50,
};

   

$2
{
  $2: $1;
  $2: $1;   
  $2: $1;  
};

   
$2
{
  $2 table_header;
  u8  $1: [u8; 4];        
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;      
  $2: $1;       
  $2: $1;
  $2: $1;
};




   
$2{
  $2: $1;                   
  $2: $1;               
  $2: $1;               
  $2: $1;               
  $2: $1;          
  $2: $1;               
  $2: $1;               
  $2: $1;               
  $2: $1;               
  $2: $1;               
  $2: $1;              
  $2: $1;              
  $2: $1;               
  $2: $1;       
  $2: $1;              
  $2: $1;              
  $2: $1;              
  $2: $1;              
  $2: $1;              
  $2: $1;              
  $2: $1;              
  $2: $1;              
  $2: $1;              
  $2: $1;              
  $2: $1;              
  $2: $1;              
  $2: $1;              
  $2: $1;              
  $2: $1;              
  $2: $1;              
  $2: $1;              
  $2: $1;              
  $2: $1;              
  $2: $1;              
  $2: $1;                   
  $2: $1;                  
  $2: $1;              
  $2: $1;              
  $2: $1;              
  $2: $1;              
  $2: $1;              
  $2: $1;             
  $2: $1;           
  $2: $1;              
  $2: $1;              
  $2: $1;              
  $2: $1;                 
  $2: $1;              
  $2: $1;              
  $2: $1;      
  $2: $1;      
  $2: $1;              
  $2: $1;              
  $2: $1;              
  $2: $1;
  $2: $1;              
  $2: $1;              
  $2: $1;              
  $2: $1;              
  $2: $1;              
  $2: $1;        
  $2: $1;              
  $2: $1;              
  $2: $1;       
  $2: $1;              
  $2: $1;              
  $2: $1;              
  $2: $1;                  
  $2: $1;              
  $2: $1;              
  $2: $1;              
  $2: $1;              
  $2: $1;              
  $2: $1;              
  $2: $1;              
  $2: $1;              
  $2: $1;      
  $2: $1;              
  $2: $1;
  $2: $1;              
  $2: $1;              
};

$2
{
  $2  table_header;
  $2 listofcmdfunctions;
};

   
$2
{
  u16  ws_in_bytes:8;            
  u16  ps_in_bytes:7;            
  u16  updated_by_util:1;        
};


   
$2
{
  $2 func_header;
  $2 func_attrib;  
};




$2{
  $2: $1;               
  $2: $1;               
  $2: $1;
  $2: $1;                 
  $2: $1;                  
  $2: $1;
  $2: $1;                      
  $2: $1;
  $2: $1;                 
  $2: $1;
  $2: $1; 
  $2: $1;          
  $2: $1;                  
  $2: $1; 
  $2: $1;
  $2: $1;                 
  $2: $1;                
  $2: $1;
  $2: $1;
  $2: $1;                
  $2: $1;
  $2: $1;
  $2: $1;             
  $2: $1;			  
  $2: $1;                      
  $2: $1;
  $2: $1;
  $2: $1;                      
  $2: $1;                     
  $2: $1;
  $2: $1;          
  $2: $1;           
  $2: $1;            
  $2: $1;
  $2: $1;
};


$2
{ 
  $2 table_header;
  $2 listOfdatatables;
};


$2
{
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
};


$2{
  ATOM_HSYNC_POLARITY    = 0x0002,
  ATOM_VSYNC_POLARITY    = 0x0004,
  ATOM_H_REPLICATIONBY2  = 0x0010,
  ATOM_V_REPLICATIONBY2  = 0x0020,
  ATOM_INTERLACE         = 0x0080,
  ATOM_COMPOSITESYNC     = 0x0040,
};







$2
{
  $2 table_header;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;             
  $2: $1;          
  $2: $1;      
  $2: $1;
  $2: $1; 
  $2: $1;
  $2: $1;
  $2: $1;       
  $2: $1;              
  u8  $1: [u8; 2];
  $2: $1;
  $2: $1;
  u32 $1: [u8; 6];
};


$2
{
	ATOM_FIRMWARE_CAP_FIRMWARE_POSTED = 0x00000001,
	ATOM_FIRMWARE_CAP_GPU_VIRTUALIZATION  = 0x00000002,
	ATOM_FIRMWARE_CAP_WMI_SUPPORT  = 0x00000040,
	ATOM_FIRMWARE_CAP_HWEMU_ENABLE  = 0x00000080,
	ATOM_FIRMWARE_CAP_HWEMU_UMC_CFG = 0x00000100,
	ATOM_FIRMWARE_CAP_SRAM_ECC      = 0x00000200,
	ATOM_FIRMWARE_CAP_ENABLE_2STAGE_BIST_TRAINING  = 0x00000400,
	ATOM_FIRMWARE_CAP_ENABLE_2ND_USB20PORT = 0x0008000,
	ATOM_FIRMWARE_CAP_DYNAMIC_BOOT_CFG_ENABLE = 0x0020000,
};

$2{
  AIR_COOLING    = 0x00,
  LIQUID_COOLING = 0x01
};

$2 {
  $2 table_header;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;             
  $2: $1;          
  $2: $1;      
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;              
  u8  $1: [u8; 2];
  $2: $1;
  $2: $1;
  $2: $1;            
  $2: $1;       
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  u32 $1: [u8; 3];
};

$2
{
  $2 table_header;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;             
  $2: $1;          
  $2: $1;      
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;              
  u8  $1: [u8; 2];
  $2: $1;
  $2: $1;
  $2: $1;            
  $2: $1;       
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;                
  u32 $1: [u8; 2];
};

$2 {
	$2 table_header;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;             
	$2: $1;          
	$2: $1;      
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;              
	u8  $1: [u8; 2];
	$2: $1;
	$2: $1;
	$2: $1;            
	$2: $1;       
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;                
	$2: $1;                      
	$2: $1;             
	$2: $1;               
	$2: $1;              
	$2: $1;              
	$2: $1;                
	$2: $1;                 
	$2: $1;          
        $2: $1;
        $2: $1;
        $2: $1;   
        u32 $1: [u8; 2];
};

$2 {
  $2 table_header;
  $2: $1;
  u32 $1: [u8; 2];
  $2: $1;             
  $2: $1;    
  $2: $1;      
  u32 $1: [u8; 2];
  $2: $1;
  $2: $1;              
  $2: $1;                     
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;            
  $2: $1;       
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  
  $2: $1;
  u32 $1: [u8; 3];
  $2: $1;                
  $2: $1;                 
  $2: $1;          
  u32 $1: [u8; 3];
  $2: $1;                    
  $2: $1;
  $2: $1;
  u32 $1: [u8; 16];
};


$2
{
  $2 table_header;
  $2  lcd_timing;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;          
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  u32 $1: [u8; 8];
};


$2{
  ATOM_PANEL_MISC_FPDI            =0x0002,
};


$2
{
  eDP_TO_LVDS_RX_DISABLE                 = 0x00,       
  eDP_TO_LVDS_COMMON_ID                  = 0x01,       
  eDP_TO_LVDS_REALTEK_ID                 = 0x02,       
};

    


$2
{
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
};


$2 {
  I2C_HW_LANE_MUX        =0x0f, 
  I2C_HW_ENGINE_ID_MASK  =0x70,  
  I2C_HW_CAP             =0x80, 

  
  
  PCIE_VDDC_CONTROL_GPIO_PINID = 56,
  
  PP_AC_DC_SWITCH_GPIO_PINID = 60,
  
  VDDC_VRHOT_GPIO_PINID = 61,
  
  VDDC_PCC_GPIO_PINID = 62,
  
  EFUSE_CUT_ENABLE_GPIO_PINID = 63,
  
  DRAM_SELF_REFRESH_GPIO_PINID = 64,
  
  THERMAL_INT_OUTPUT_GPIO_PINID =65,
};


$2
{
  $2  table_header;
  
  $2  gpio_pin[];
};




$2
{
	$2  table_header;
	$2: $1;
	$2: $1;
	$2: $1;
};

$2 {
	$2  table_header;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	u32  $1: [u8; 7];
};



$2 {
	ATOM_I2C_RECORD_TYPE = 1,
	ATOM_HPD_INT_RECORD_TYPE = 2,
	ATOM_CONNECTOR_CAP_RECORD_TYPE = 3,
	ATOM_CONNECTOR_SPEED_UPTO = 4,
	ATOM_OBJECT_GPIO_CNTL_RECORD_TYPE = 9,
	ATOM_CONNECTOR_HPDPIN_LUT_RECORD_TYPE = 16,
	ATOM_CONNECTOR_AUXDDC_LUT_RECORD_TYPE = 17,
	ATOM_ENCODER_CAP_RECORD_TYPE = 20,
	ATOM_BRACKET_LAYOUT_RECORD_TYPE = 21,
	ATOM_CONNECTOR_FORCED_TMDS_CAP_RECORD_TYPE = 22,
	ATOM_DISP_CONNECTOR_CAPS_RECORD_TYPE = 23,
	ATOM_BRACKET_LAYOUT_V2_RECORD_TYPE = 25,
	ATOM_RECORD_END_TYPE = 0xFF,
};

$2
{
  $2: $1;                      
  $2: $1;                      
};

$2
{
  $2 record_header;   
  $2: $1; 
  $2: $1;                   
};

$2
{
  $2 record_header;  
  $2: $1;              
  $2: $1;
};

$2 {
	$2
		record_header; 
	$2: $1; 
};

$2 {
	$2
		record_header; 
	$2: $1; 
	$2: $1;
};


$2
{
  ATOM_ENCODER_CAP_RECORD_HBR2                  =0x01,         
  ATOM_ENCODER_CAP_RECORD_MST_EN                =0x01,         
  ATOM_ENCODER_CAP_RECORD_HBR2_EN               =0x02,         
  ATOM_ENCODER_CAP_RECORD_HDMI6Gbps_EN          =0x04,         
  ATOM_ENCODER_CAP_RECORD_HBR3_EN               =0x08,         
  ATOM_ENCODER_CAP_RECORD_DP2                   =0x10,         
  ATOM_ENCODER_CAP_RECORD_UHBR10_EN             =0x20,         
  ATOM_ENCODER_CAP_RECORD_UHBR13_5_EN           =0x40,         
  ATOM_ENCODER_CAP_RECORD_UHBR20_EN             =0x80,         
  ATOM_ENCODER_CAP_RECORD_USB_C_TYPE            =0x100,        
};

$2
{
  $2 record_header;  
  $2: $1;
};

$2
{
  ATOM_CONNECTOR_CAP_INTERNAL_DISPLAY         = 0x01,        
  ATOM_CONNECTOR_CAP_INTERNAL_DISPLAY_BL      = 0x02,        
  ATOM_CONNECTOR_CAP_DP_PLUS_PLUS_TYPE2_ONLY  = 0x10,        
};

$2
{
  $2 record_header;
  $2: $1;                          
};


$2
{
  $2: $1;               
  $2: $1;         
};

$2
{
  $2 record_header;
  $2: $1;                   
  $2: $1;         
  $2 $1: [u8; 1];              
};


$2
{
  GPIO_PIN_TYPE_INPUT             = 0x00,
  GPIO_PIN_TYPE_OUTPUT            = 0x10,
  GPIO_PIN_TYPE_HW_CONTROL        = 0x20,


  GPIO_PIN_OUTPUT_STATE_MASK      = 0x01,
  GPIO_PIN_OUTPUT_STATE_SHIFT     = 0,
  GPIO_PIN_STATE_ACTIVE_LOW       = 0x0,
  GPIO_PIN_STATE_ACTIVE_HIGH      = 0x1,
};



$2
{
  ATOM_GPIO_INDEX_GLSYNC_REFCLK    = 0,
  ATOM_GPIO_INDEX_GLSYNC_HSYNC     = 1,
  ATOM_GPIO_INDEX_GLSYNC_VSYNC     = 2,
  ATOM_GPIO_INDEX_GLSYNC_SWAP_REQ  = 3,
  ATOM_GPIO_INDEX_GLSYNC_SWAP_GNT  = 4,
  ATOM_GPIO_INDEX_GLSYNC_INTERRUPT = 5,
  ATOM_GPIO_INDEX_GLSYNC_V_RESET   = 6,
  ATOM_GPIO_INDEX_GLSYNC_SWAP_CNTL = 7,
  ATOM_GPIO_INDEX_GLSYNC_SWAP_SEL  = 8,
  ATOM_GPIO_INDEX_GLSYNC_MAX       = 9,
};


$2     
{
  $2 record_header;
  u8 $1: [u8; 8];             
};

$2     
{
  $2 record_header;
  u8 $1: [u8; 8];
};

$2
{
  $2 record_header;
  
  $2: $1;
  $2: $1;
};    

$2
{
  $2: $1;
  $2: $1;
  $2: $1;
};


$2
{
  CONNECTOR_TYPE_DVI_D                 = 1,
 
  CONNECTOR_TYPE_HDMI                  = 4,
  CONNECTOR_TYPE_DISPLAY_PORT          = 5,
  CONNECTOR_TYPE_MINI_DISPLAY_PORT     = 6,
};

$2
{
  $2 record_header;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2  $1: [u8; 1];
};
$2 {
	$2
		record_header; 
	$2: $1; 
	$2: $1; 
	$2: $1; 
	$2: $1; 
	$2: $1;
	$2: $1;
};

$2 {
	MINI_TYPE_NORMAL = 0,
	MINI_TYPE_MINI = 1,
};

$2{
  ATOM_DISPLAY_LCD1_SUPPORT            = 0x0002, 
  ATOM_DISPLAY_LCD2_SUPPORT            = 0x0020, 
  ATOM_DISPLAY_DFP1_SUPPORT            = 0x0008,
  ATOM_DISPLAY_DFP2_SUPPORT            = 0x0080,
  ATOM_DISPLAY_DFP3_SUPPORT            = 0x0200,
  ATOM_DISPLAY_DFP4_SUPPORT            = 0x0400,
  ATOM_DISPLAY_DFP5_SUPPORT            = 0x0800,
  ATOM_DISPLAY_DFP6_SUPPORT            = 0x0040,
  ATOM_DISPLAY_DFPx_SUPPORT            = 0x0ec8,
};

$2
{
  $2: $1;                  
  $2: $1;
  $2: $1;                   
  $2: $1;                
  $2: $1;
  $2: $1;
  $2: $1;                     
  $2: $1;
  $2: $1;
};

$2 {
	$2: $1; 
	$2: $1;
	$2: $1; 
	$2: $1; 
	$2: $1; 
	$2: $1; 
	
	
	$2: $1;
	$2: $1; 
};

$2
{
  $2  table_header;
  $2: $1;
  $2: $1;
  $2: $1;
  $2 display_path[];   
};

$2 {
	$2 table_header;
	$2: $1;
	$2: $1;
	$2: $1;
	
	
	$2 display_path[];
};


$2
{
  $2  table_header;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;       
  $2: $1;        
  $2: $1;      
  $2: $1;
  $2: $1;        
  $2: $1;
  $2: $1;             
  $2: $1;            
  $2: $1;              
  $2: $1;
  $2: $1;       
  u8  $1: [u8; 3];
  $2: $1;  
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  u8  $1: [u8; 8];
};

$2
{
  $2  table_header;
  $2: $1;            
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;       
  $2: $1;
  $2: $1;      
  $2: $1;
  $2: $1;        
  $2: $1;
  $2: $1;             
  $2: $1;            
  $2: $1;              
  $2: $1;
  $2: $1;   
  $2: $1;
  $2: $1;   
  $2: $1;
  $2: $1;  
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  u8  $1: [u8; 8];
};

$2
{
  $2  table_header;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;       
  $2: $1;
  $2: $1;      
  $2: $1;
  $2: $1;        
  $2: $1;
  $2: $1;             
  $2: $1;            
  $2: $1;              
  $2: $1;
  $2: $1;   
  $2: $1;
  $2: $1;   
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  u8  $1: [u8; 8];
};

$2 {
	$2 table_header;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;	 
	$2: $1;
	$2: $1;	 
	$2: $1;
	$2: $1;	 
	$2: $1;
	$2: $1;		 
	$2: $1;		 
	$2: $1;		 
	$2: $1;
	$2: $1;	 
	$2: $1;
	$2: $1;	 
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;	 
	$2: $1;	 
	$2: $1;	 
	$2: $1; 
	$2: $1;
	u32 $1: [u8; 3];
};

$2
{
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	u32 $1: [u8; 23];
};

$2 {
	
	DCE_INFO_CAPS_FORCE_DISPDEV_CONNECTED = 0x02,
	
	DCE_INFO_CAPS_DISABLE_DFP_DP_HBR2 = 0x04,
	
	DCE_INFO_CAPS_ENABLE_INTERLAC_TIMING = 0x08,
	
	DCE_INFO_CAPS_LTTPR_SUPPORT_ENABLE = 0x20,
	DCE_INFO_CAPS_VBIOS_LTTPR_TRANSPARENT_ENABLE = 0x40,
};

$2
{
  $2  table_header;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;       
  $2: $1;
  $2: $1;      
  $2: $1;
  $2: $1;        
  $2: $1;
  $2: $1;             
  $2: $1;            
  $2: $1;              
  $2: $1;
  
  $2: $1;
  
  $2: $1;
  
  $2: $1;
  
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  
  $2: $1;
  
  $2: $1;
  
  $2: $1;
  $2: $1;  
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  u32 $1: [u8; 26];
};


$2
{
  $2: $1;                      
  $2: $1;                
  $2: $1;                  
  $2: $1;                 
  $2: $1;                    
  $2: $1;               
  $2: $1;                  
  $2: $1;                      
  $2: $1;
  $2: $1; 
};


$2 {
  EXT_DISPLAY_PATH_CAPS__EXT_CHIP_MASK =		0x007E,
  AMD_EXT_DISPLAY_PATH_CAPS__EXT_CHIP_MASK =		0x007E,
  AMD_EXT_DISPLAY_PATH_CAPS__DP_FIXED_VS_EN =		(0x01 << 1),
  AMD_EXT_DISPLAY_PATH_CAPS__HDMI20_PI3EQX1204 =	(0x02 << 1),
  AMD_EXT_DISPLAY_PATH_CAPS__DP_EARLY_8B10B_TPS2 =	(0x03 << 1),
  AMD_EXT_DISPLAY_PATH_CAPS__HDMI20_TISN65DP159RSBT =	(0x04 << 1),
  AMD_EXT_DISPLAY_PATH_CAPS__HDMI20_PARADE_PS175 =	(0x06 << 1),
  EXT_DISPLAY_PATH_CAPS__DP_FIXED_VS_EN =		(0x07 << 1),
  EXT_DISPLAY_PATH_CAPS__HDMI20_PI3EQX1204 =		(0x08 << 1),   
  EXT_DISPLAY_PATH_CAPS__HDMI20_TISN65DP159RSBT =	(0x09 << 1),   
  EXT_DISPLAY_PATH_CAPS__AMD_INTERNAL =		(0x0a << 1),   
};

$2
{
  $2  table_header;
  u8                  $1: [u8; 16];                                  
  $2 $1: [u8; 7];                               
  $2: $1;                                  
  $2: $1;                               
  $2: $1;
  $2: $1;
  $2: $1;                         
  u8                  $1: [u8; 3];                               
};



$2
{
  $2: $1;       
  $2: $1;
};

$2
{
  u16 $1: [u8; 3];
};

$2
{
  $2: $1;                    
  u8 $1: [u8; 8];
  $2 $1: [u8; 6]; 
};

$2
{
  $2: $1;                
  u8 $1: [u8; 8];
};

$2
{
  $2: $1;
  $2 $1: [u8; 3];      
  $2 flashInfo;      
  $2 dphy_param;
  $2: $1;         
};


$2
{
  $2: $1;
  $2: $1;            
  $2: $1;                 
  $2: $1;           
  $2: $1;            
  $2: $1;                
  $2: $1;    
  $2: $1;              
};

$2{
  $2: $1;       
  $2: $1;           
  $2: $1;            
  $2: $1;                
};

$2{
  $2: $1;                 
  $2: $1;
  $2: $1;             
  $2: $1;
  $2 $1: [u8; 10];
};

$2{  
  $2  table_header;  
  $2: $1;     
  $2: $1;           
  $2: $1;            
};

$2
{
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;         
};

$2
{
  $2: $1;
  $2: $1;           
  $2: $1;                
  $2: $1;             
  $2: $1;              
  $2: $1;             
  $2: $1;
  $2: $1;          
  $2: $1;
};

$2{
  $2: $1;       
  $2: $1;             
  $2: $1;              
  $2: $1;             
  $2: $1;          
};

$2{
  $2: $1;                 
  $2: $1;
  $2: $1;             
  $2: $1;
  $2 $1: [u8; 10];
};

$2 {
  $2: $1;
  $2: $1;
};

$2 {
  $2: $1;
  $2: $1;
  $2: $1;
  $2 $1: [u8; 9];        
  $2 $1: [u8; 3];    
};

$2
{
  $2  table_header;
  $2: $1;                       
  $2: $1;                       
  $2: $1;                    
  $2: $1;
  $2: $1;             
  $2: $1;
  $2: $1;               
  $2: $1;
  $2: $1;               
  $2: $1;
  $2: $1;                
  $2: $1;
  $2: $1;                   
  $2: $1;                        
  $2: $1;                 
  $2: $1;                       
  $2: $1;                 
  $2: $1;               
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2 extdispconninfo;
  $2 dvi_tuningset;
  $2 hdmi_tuningset;
  $2 hdmi6g_tuningset;
  $2 dp_tuningset;        
  $2 dp_hbr3_tuningset;   
  $2  camera_info;
  $2 dp0_retimer_set;   
  $2 dp1_retimer_set;   
  $2 dp2_retimer_set;   
  $2 dp3_retimer_set;   
  $2 dp_hbr_tuningset;    
  $2 dp_hbr2_tuningset;   
  $2 edp_tuningset;       
  u32  $1: [u8; 66];
};

$2
{
  $2  table_header;
  $2: $1;                       
  $2: $1;                       
  $2: $1;                    
  $2: $1;
  $2: $1;             
  $2: $1;
  $2: $1;               
  $2: $1;
  $2: $1;               
  $2: $1;
  $2: $1;                
  $2: $1;
  $2: $1;                   
  $2: $1;                        
  $2: $1;                 
  $2: $1;                       
  $2: $1;                 
  $2: $1;               
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2 extdispconninfo;
  $2  TMDS_tuningset;
  $2  hdmiCLK5_tuningset;
  $2  hdmiCLK8_tuningset;
  $2 rbr_tuningset;        
  $2 hbr3_tuningset;   
  $2  camera_info;
  $2 dp0_retimer_set;   
  $2 dp1_retimer_set;   
  $2 dp2_retimer_set;   
  $2 dp3_retimer_set;   
  $2 hbr_tuningset;    
  $2 hbr2_tuningset;   
  $2 edp_tunings;       
  $2  hdmiCLK6_tuningset;
  u32  $1: [u8; 63];
};

$2
{
        $2: $1;
        $2: $1;
        $2: $1;
        $2: $1;
        $2: $1;
        $2: $1;
        $2: $1;
        $2: $1;
        $2: $1;
        $2: $1;
        u8  $1: [u8; 3];
        u32 $1: [u8; 3];
};

$2
{
        $2  table_header;
        $2: $1;                       
        $2: $1;                       
        $2: $1;
        $2: $1;
        $2: $1;             
        $2: $1;
        $2: $1;                   
        $2: $1;                       
        $2: $1;                 
        $2: $1;
        $2: $1;
        $2: $1;
        $2: $1;
        $2 edp1_info;
        $2 edp2_info;
        u32  $1: [u8; 8];
        $2 extdispconninfo;
        $2  TMDS_tuningset;
        $2  hdmiCLK5_tuningset; 
        $2  hdmiCLK6_tuningset;
        $2  hdmiCLK8_tuningset;
        u32 $1: [u8; 6];
        $2 rbr_tuningset;        
        $2 hbr_tuningset;    
        $2 hbr2_tuningset;   
        $2 hbr3_tuningset;   
        $2 edp_tunings;       
        u32 $1: [u8; 28];
        $2 dp0_retimer_set;   
        $2 dp1_retimer_set;   
        $2 dp2_retimer_set;   
        $2 dp3_retimer_set;   
        u32 $1: [u8; 30];
        u32 $1: [u8; 32];

};

$2 {
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
};

$2 {
	$2 table_header;
	$2 $1: [u8; 1];
};

$2
{
	$2  table_header;
	$2: $1;                       
	$2: $1;                       
	$2: $1;
	$2: $1;
	$2: $1;             
	$2: $1;
	$2: $1;                   
	$2: $1;                       
	$2: $1;                 
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2 edp1_info;
	$2 edp2_info;
	u32  $1: [u8; 8];
	$2 extdispconninfo;

	u32  $1: [u8; 189];
};

$2 {
  char       $1: [u8; 29];        
  $2: $1;        
  $2: $1;     
  union {
    $2 {
      u8 Auto     : 1;
      u8 Custom   : 1;
      u8 Reserved : 6;
    } flags;
    $2: $1;
  } uma_carveout_option_flags;
};

$2 {
  $2 table_header;
  $2: $1; 
  $2: $1; 
  $2: $1;
  $2: $1;
  $2: $1; 
  $2: $1;
  $2: $1;  
  $2: $1;       
  $2: $1; 
  $2: $1;
  $2: $1;
  $2: $1; 
  $2: $1;
  $2  edp1_info;
  $2  edp2_info;
  $2: $1;
  $2: $1;
  u32 $1: [u8; 6];
  $2 extdispconninfo;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;           
  $2: $1;
  $2 $1: [u8; 20];
  u8 $1: [u8; 110];
};


$2{
  INTEGRATED_SYSTEM_INFO__GET_EDID_CALLBACK_FUNC_SUPPORT = 0x01,
};



$2{
  SYS_INFO_GPUCAPS__ENABLE_DFS_BYPASS  = 0x10,
};


$2{
  ATOM_ENABLE_DVI_TUNINGSET   = 0x01,
  ATOM_ENABLE_HDMI_TUNINGSET  = 0x02,
  ATOM_ENABLE_HDMI6G_TUNINGSET  = 0x04,
  ATOM_ENABLE_DP_TUNINGSET  = 0x08,
  ATOM_ENABLE_DP_HBR3_TUNINGSET  = 0x10,  
};


$2
{
  SYS_INFO_LVDS_MISC_888_FPDI_MODE                 =0x01,
  SYS_INFO_LVDS_MISC_888_BPC_MODE                  =0x04,
  SYS_INFO_LVDS_MISC_OVERRIDE_EN                   =0x08,
};



$2{
  OtherMemType = 0x01,                                  
  UnknownMemType,                                       
  DramMemType,                                          
  EdramMemType,                                         
  VramMemType,                                          
  SramMemType,                                          
  RamMemType,                                           
  RomMemType,                                           
  FlashMemType,                                         
  EepromMemType,                                        
  FepromMemType,                                        
  EpromMemType,                                         
  CdramMemType,                                         
  ThreeDramMemType,                                     
  SdramMemType,                                         
  SgramMemType,                                         
  RdramMemType,                                         
  DdrMemType,                                           
  Ddr2MemType,                                          
  Ddr2FbdimmMemType,                                    
  Ddr3MemType = 0x18,                                   
  Fbd2MemType,                                          
  Ddr4MemType,                                          
  LpDdrMemType,                                         
  LpDdr2MemType,                                        
  LpDdr3MemType,                                        
  LpDdr4MemType,                                        
  GDdr6MemType,                                         
  HbmMemType,                                           
  Hbm2MemType,                                          
  Ddr5MemType,                                          
  LpDdr5MemType,                                        
  LpDdr5xMemType,                                       
};



$2
{
  $2   sysinfo;           
  u32   $1: [u8; 256];                                
}; 




$2
{
  $2  table_header;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1; 
  $2: $1;
  $2: $1;
  $2: $1; 
};

$2 {
  $2  table_header;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
};

$2
{
  $2  table_header;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
};

$2 {
	$2 table_header;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	u8 $1: [u8; 2];
	$2: $1;
	u8 $1: [u8; 8];
	u32 $1: [u8; 6];
};

$2 {
	$2 table_header;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	u8 $1: [u8; 16];
	u8 $1: [u8; 16];
	$2: $1;
	$2: $1;
	u32 $1: [u8; 8];
};


$2
{
  $2  table_header;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;           
  $2: $1;
  $2: $1;
  $2: $1;    
  $2: $1;
  $2: $1;
  $2: $1;          
  $2: $1;          
  $2: $1;         
  $2: $1;         
  $2: $1;         
  $2: $1;         
  $2: $1;         
  $2: $1;         
};

$2 {
  $2  table_header;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;    
  $2: $1;
  $2: $1;
  $2: $1;          
  $2: $1;          
  $2: $1;         
  $2: $1;         
  $2: $1;         
  $2: $1;         
  $2: $1;         
  $2: $1;         
  $2: $1;            
  $2: $1;       
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  u32 $1: [u8; 5];
};

$2 {
  $2  table_header;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;    
  $2: $1;
  $2: $1;
  $2: $1;          
  $2: $1;          
  $2: $1;         
  $2: $1;         
  $2: $1;         
  $2: $1;         
  $2: $1;         
  $2: $1;         
  $2: $1;            
  $2: $1;       
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;    
  $2: $1;
  $2: $1;
};

$2
{
  $2  table_header;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;    
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;            
  $2: $1;       
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;    
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  u32 $1: [u8; 16];
};

$2
{
	$2  table_header;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	u32 $1: [u8; 12];
};

$2 {
	$2 table_header;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	u32 $1: [u8; 16];
};


$2
{
  $2  table_header;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;
  u8  $1: [u8; 2];

  $2: $1;
  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;

	$2: $1;
	$2: $1;
	$2: $1;

  $2: $1;
  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;
  $2: $1;

	$2: $1;
	$2: $1;
	$2: $1;

	$2: $1;
	u8 $1: [u8; 3];

	u32 $1: [u8; 9];
};


$2
{
  $2  table_header;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;
  $2: $1;

  u32 $1: [u8; 10];
};

$2 {
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
};

$2
{
  $2  table_header;
  u32  $1: [u8; 3];

  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;
  $2: $1;


  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;

  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;


  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;


  $2: $1;
  $2: $1;
  $2: $1;


  $2: $1;
  $2: $1;
  $2: $1;


  $2: $1;
  $2: $1;
  $2: $1;


  $2: $1;
  $2: $1;
  $2: $1;


  $2  $1: [u8; 7];


  u32 $1: [u8; 10];
};

$2{
    SMC_V4_5_I2C_CONTROLLER_NAME_VR_GFX = 0,
    SMC_V4_5_I2C_CONTROLLER_NAME_VR_SOC,
    SMC_V4_5_I2C_CONTROLLER_NAME_VR_VDDCI,
    SMC_V4_5_I2C_CONTROLLER_NAME_VR_MVDD,
    SMC_V4_5_I2C_CONTROLLER_NAME_LIQUID0,
    SMC_V4_5_I2C_CONTROLLER_NAME_LIQUID1,
    SMC_V4_5_I2C_CONTROLLER_NAME_PLX,
    SMC_V4_5_I2C_CONTROLLER_NAME_SPARE,
    SMC_V4_5_I2C_CONTROLLER_NAME_COUNT,
};

$2{
    SMC_V4_5_I2C_CONTROLLER_THROTTLER_TYPE_NONE = 0,
    SMC_V4_5_I2C_CONTROLLER_THROTTLER_VR_GFX,
    SMC_V4_5_I2C_CONTROLLER_THROTTLER_VR_SOC,
    SMC_V4_5_I2C_CONTROLLER_THROTTLER_VR_VDDCI,
    SMC_V4_5_I2C_CONTROLLER_THROTTLER_VR_MVDD,
    SMC_V4_5_I2C_CONTROLLER_THROTTLER_LIQUID0,
    SMC_V4_5_I2C_CONTROLLER_THROTTLER_LIQUID1,
    SMC_V4_5_I2C_CONTROLLER_THROTTLER_PLX,
    SMC_V4_5_I2C_CONTROLLER_THROTTLER_COUNT,
};

$2{
    SMC_V4_5_I2C_CONTROLLER_PROTOCOL_VR_0,
    SMC_V4_5_I2C_CONTROLLER_PROTOCOL_VR_1,
    SMC_V4_5_I2C_CONTROLLER_PROTOCOL_TMP_0,
    SMC_V4_5_I2C_CONTROLLER_PROTOCOL_TMP_1,
    SMC_V4_5_I2C_CONTROLLER_PROTOCOL_SPARE_0,
    SMC_V4_5_I2C_CONTROLLER_PROTOCOL_SPARE_1,
    SMC_V4_5_I2C_CONTROLLER_PROTOCOL_COUNT,
};

$2
{
    $2: $1;
    $2: $1;
    u8   $1: [u8; 2];
    $2: $1;
    $2: $1;
    $2: $1;
    $2: $1;
    $2: $1;
};

$2
{
  $2  table_header;
    
    
  $2  $1: [u8; 8];

  
  $2: $1; 
  $2: $1; 

  $2: $1;   
  $2: $1;   
  $2: $1;  
  $2: $1;  

  $2: $1; 
  $2: $1; 
  $2: $1; 
  $2: $1;

  
  $2: $1;   
  $2: $1;       
  $2: $1;
  $2: $1;   
  $2: $1;       
  $2: $1;

  $2: $1;   
  $2: $1;       
  $2: $1;

  $2: $1;   
  $2: $1;       
  $2: $1;

  
  $2: $1;        
  $2: $1;    
  $2: $1;      
  $2: $1;  

  $2: $1;      
  $2: $1;  
  $2: $1;        
  $2: $1;    

  
  $2: $1;         
  $2: $1;         
  $2: $1;         
  $2: $1;

  
  $2: $1;   
  $2: $1;   
  $2: $1;      

  
  $2: $1;   
  $2: $1;   
  $2: $1;      

  
  $2: $1;   
  $2: $1;   
  $2: $1;      

  
  $2: $1;   
  $2: $1;   
  $2: $1;      

  
  $2: $1;     
  $2: $1; 

  
  $2: $1; 
  
  u32     $1: [u8; 9];

};

$2
{
  $2  table_header;
  
  u32     $1: [u8; 3];   

  $2: $1; 
  $2: $1; 

  $2: $1;     
  $2: $1;     
  $2: $1;     
  $2: $1;      

  $2: $1; 
  $2: $1; 
  u8      $1: [u8; 2];

  
  $2: $1;   
  $2: $1;       
  $2: $1;

  $2: $1;   
  $2: $1;       
  $2: $1;

  $2: $1;   
  $2: $1;       
  $2: $1;

  $2: $1;   
  $2: $1;       
  $2: $1;

  
  $2: $1;      
  $2: $1;  
  $2: $1;      
  $2: $1;  

 
  $2: $1;	
  $2: $1;	
  $2: $1;		

 
  $2: $1;   
  $2: $1;   
  $2: $1;	   

 
  $2: $1;   
  $2: $1;   
  $2: $1;	   


  
  $2: $1;   
  $2: $1;   
  $2: $1;      

  
  $2 $1: [u8; 8];

  
  $2: $1; 

  $2: $1; 
  u8 	 $1: [u8; 3];

	
  $2: $1;	  
  $2: $1;

	
  u8 	 $1: [u8; 4];
  u8 	 $1: [u8; 4];

  u16	 $1: [u8; 4];
  u16	 $1: [u8; 4];

  
  u32   $1: [u8; 10];
};

$2
{
  $2  table_header;
    
    
  $2  $1: [u8; 8];

  
  $2: $1; 
  $2: $1; 

  $2: $1;   
  $2: $1;   
  $2: $1;  
  $2: $1;  

  $2: $1; 
  $2: $1; 
  $2: $1; 
  $2: $1;

  
  $2: $1;   
  $2: $1;       
  $2: $1;
  $2: $1;   
  $2: $1;       
  $2: $1;

  $2: $1;   
  $2: $1;       
  $2: $1;

  $2: $1;   
  $2: $1;       
  $2: $1;

  
  $2: $1;        
  $2: $1;    
  $2: $1;      
  $2: $1;  

  $2: $1;      
  $2: $1;  
  $2: $1;        
  $2: $1;    

  
  $2: $1;         
  $2: $1;         
  $2: $1;         
  $2: $1;

  
  $2: $1;   
  $2: $1;   
  $2: $1;      

  
  $2: $1;   
  $2: $1;   
  $2: $1;      

  
  $2: $1;   
  $2: $1;   
  $2: $1;      

  
  $2: $1;   
  $2: $1;   
  $2: $1;      

  
  $2: $1;     
  $2: $1;

  
  $2: $1; 

  
  $2: $1;          
  $2: $1;          
  $2: $1;

  
  $2: $1;         
  $2: $1;         
  $2: $1;

  
  u8      $1: [u8; 4];    

  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;

  u32     $1: [u8; 5];
};

$2
{
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
};

$2
{
  $2  table_header;

  
  

  
  $2  $1: [u8; 16];     

  $2: $1;  
  $2: $1;  
  $2: $1; 
  $2: $1;

  
  $2: $1;   
  $2: $1;   
  $2: $1;  
  $2: $1;  

  $2: $1; 
  $2: $1; 
  $2: $1; 
  $2: $1; 

  
  $2: $1;   
  $2: $1;       
  $2: $1;

  $2: $1;   
  $2: $1;       
  $2: $1;

  $2: $1;   
  $2: $1;       
  $2: $1;
  
  $2: $1;   
  $2: $1;       
  $2: $1;

  $2: $1; 
  
  
  $2: $1;        
  $2: $1;    
  $2: $1;      
  $2: $1;  

  $2: $1;      
  $2: $1;  
  $2: $1;        
  $2: $1;    

  
  $2: $1;         
  $2: $1;         
  $2: $1;         
  $2: $1;

  $2: $1;        
  $2: $1;       
  u8      $1: [u8; 2];

  
  
  
  $2: $1;   
  $2: $1;   
  $2: $1;      

  
  $2: $1;   
  $2: $1;   
  $2: $1;      
  
  
  $2: $1;   
  $2: $1;   
  $2: $1;      

  
  $2: $1;   
  $2: $1;   
  $2: $1;      
  
  
  $2: $1; 
  
  $2: $1; 
  u8      $1: [u8; 3];

  
  $2: $1;     
  $2: $1; 
  
  
  u8      $1: [u8; 4];
  u8      $1: [u8; 4];

  u16     $1: [u8; 4];
  u16     $1: [u8; 4];

  

  u32     $1: [u8; 16];

};

$2
{
  $2  table_header;

  
  
  $2: $1; 
  $2: $1;     
  $2: $1;

  $2: $1; 
  $2: $1;     
  $2: $1;

  $2: $1; 
  $2: $1;     
  $2: $1;

  $2: $1; 
  $2: $1;     
  $2: $1;

  
  $2: $1; 
  $2: $1; 

  
  $2: $1;     
  $2: $1; 
  $2: $1;     
  $2: $1; 

  
  $2: $1; 
  $2: $1; 
  $2: $1;    

  
  $2: $1; 
  $2: $1; 
  $2: $1;    

  
  $2  $1: [u8; 8];

  
  $2: $1; 
  $2: $1; 
  $2: $1;

  u32 $1: [u8; 16];
};


$2
{
  $2  table_header;
  $2: $1;                 
  $2: $1;               
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;	
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
};

$2 {
	$2  table_header;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
};


$2
{
  $2  table_header;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1; 
  $2: $1;           
  $2: $1;           
};



$2
{
  $2  table_header;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;              
  $2: $1;
  $2: $1;
};


$2 {
  UMC_CONFIG__ENABLE_1KB_INTERLEAVE_MODE  =   0x00000001,
  UMC_CONFIG__DEFAULT_MEM_ECC_ENABLE      =   0x00000002,
  UMC_CONFIG__ENABLE_HBM_LANE_REPAIR      =   0x00000004,
  UMC_CONFIG__ENABLE_BANK_HARVESTING      =   0x00000008,
  UMC_CONFIG__ENABLE_PHY_REINIT           =   0x00000010,
  UMC_CONFIG__DISABLE_UCODE_CHKSTATUS     =   0x00000020,
};

$2
{
  $2  table_header;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;              
  $2: $1;
  $2: $1;
  u32 $1: [u8; 4];
  $2: $1;
  $2: $1;
};

$2
{
  $2  table_header;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;              
  $2: $1;
  $2: $1;
  u32 $1: [u8; 4];
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  u32 $1: [u8; 2];
};

$2 {
	UMC_CONFIG1__ENABLE_PSTATE_PHASE_STORE_TRAIN = 0x00000001,
	UMC_CONFIG1__ENABLE_AUTO_FRAMING = 0x00000002,
	UMC_CONFIG1__ENABLE_RESTORE_BIST_DATA = 0x00000004,
	UMC_CONFIG1__DISABLE_STROBE_MODE = 0x00000008,
	UMC_CONFIG1__DEBUG_DATA_PARITY_EN = 0x00000010,
	UMC_CONFIG1__ENABLE_ECC_CAPABLE = 0x00010000,
};

$2 {
	$2 table_header;
	u32 $1: [u8; 5];
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	u32 $1: [u8; 4];
	$2: $1;
	$2: $1;
	u32 $1: [u8; 2];
	$2: $1;
	$2: $1;
	u8 $1: [u8; 2];
	u8 $1: [u8; 16];
};


$2 {
  
  $2: $1;                   
  $2: $1;                
  $2: $1;                   
  u16  $1: [u8; 3];
  $2: $1;                   
  $2: $1;              
  $2: $1;                 
  $2: $1;                   
  $2: $1;                   
  $2: $1;                 
  $2: $1;                       
  $2: $1;                 
  $2: $1;                 
  $2: $1;                   
  $2: $1;		   
  $2: $1;			   
  char    $1: [u8; 20];               
};

$2 {
  $2 table_header;
  $2: $1;                         
  $2: $1;                      
  $2: $1;                  
  $2: $1;                         
  $2: $1;                    
  $2: $1;                              
  $2: $1;                       
  $2: $1;
  $2: $1;                              
  $2: $1;
  $2: $1;
  $2: $1;                              
  $2  $1: [u8; 16];         
};


$2 {
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	u32 $1: [u8; 3];
	char $1: [u8; 40];
};

$2 {
	$2 table_header;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	u16 $1: [u8; 2];
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	u32 $1: [u8; 4];
	$2 $1: [u8; 8];
};

$2{
  u32  umc_register_addr:24;
  u32  umc_reg_type_ind:1;
  u32  umc_reg_rsvd:7;
};


$2{
  b3ATOM_UMC_REG_ADD_INFO_INDIRECT_ACCESS  =0x01,
};

$2
{
  $2 umc_reg_addr;
  $2: $1;
};

$2{
  u32 memclockrange:24;
  u32 mem_blk_id:8;
};

$2
{
  $2 umc_id_access;
  $2: $1;
};

$2{
  $2  block_id;
  u32 $1: [u8; 1];                       
};

$2{
  $2: $1;
  $2: $1;    
  $2 $1: [u8; 1];     
  $2 $1: [u8; 1];
};

$2 {
  
  $2: $1;                   
  $2: $1;                
  $2: $1;                   
  u16  $1: [u8; 3];
  $2: $1;                   
  $2: $1;              
  $2: $1;                 
  $2: $1;                   
  $2: $1;                   
  $2: $1;                 
  $2: $1;                       
  $2: $1;                 
  $2: $1;                 
  $2: $1;                   
  $2: $1;			   
  $2: $1;			   
  $2: $1;                    
  $2: $1;                     
  $2: $1;                     
  $2: $1;                     
  char    $1: [u8; 20];               
};

$2 {
  $2 table_header;
  $2: $1;                         
  $2: $1;                      
  $2: $1;                  
  $2: $1;                         
  $2: $1;                    
  $2: $1;                                     
  $2: $1;                       
  $2: $1;
  $2: $1;                              
  $2: $1;
  $2: $1;
  $2: $1;                              
  $2  $1: [u8; 16];        
};

$2 {
	
	$2: $1;                   
	$2: $1;                
	$2: $1;                   
	$2: $1;              
	$2: $1;                 
	$2: $1;                   
	$2: $1;                   
	$2: $1;                 
	$2: $1;                       
	$2: $1;                 
	u16  $1: [u8; 4];                   
	$2: $1;                 
	$2: $1;			 
	$2: $1;			 
	$2: $1;			 
	$2: $1;                    
	$2: $1;                     
	$2: $1;                     
	$2: $1;                     
	$2: $1;                     
	$2: $1;                     
	$2: $1;                     
	char    $1: [u8; 40];               
};

$2 {
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;

	$2: $1;
	$2: $1;
	$2: $1;

	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;

	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;

	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;

	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;

	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;

	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;

	$2: $1;

	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;

	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;

	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;

	$2: $1;
	$2: $1;
	$2: $1;
	u8  $1: [u8; 9];
};

$2 {
	$2: $1;    
	$2: $1;    
	$2: $1;    
	$2: $1;    
	$2: $1;    
	$2: $1;    
	$2: $1;          
};

$2 {
	$2: $1;
	u8 $1: [u8; 8];     
	$2 $1: [u8; 16];
};

$2 {
	$2 table_header;
	$2: $1;                         
	$2: $1;                     
	$2: $1;                  
	$2: $1;                         
	$2: $1;                    
	$2: $1;                                     
	$2: $1;                       
	$2: $1;                  
	$2: $1;                              
	$2: $1;
	$2: $1;
	$2: $1;                              
	$2  $1: [u8; 16];        
};

$2 {
	$2 table_header;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2: $1;
	$2 $1: [u8; 16];
};

$2
{
  $2: $1;               
  $2: $1;                
};

$2{
  $2: $1;                           
  $2: $1;                           
  $2: $1;                            
};


$2 
{
   VOLTAGE_OBJ_GPIO_LUT              =  0,        
   VOLTAGE_OBJ_VR_I2C_INIT_SEQ       =  3,        
   VOLTAGE_OBJ_PHASE_LUT             =  4,        
   VOLTAGE_OBJ_SVID2                 =  7,        
   VOLTAGE_OBJ_EVV                   =  8, 
   VOLTAGE_OBJ_MERGED_POWER          =  9,
};

$2
{
   $2 header;  
   $2: $1;                        
   $2: $1;
   $2: $1;
   $2: $1;       
   $2: $1;                            
   $2: $1;                           
   u8  $1: [u8; 2];
   $2 $1: [u8; 1];     
};


$2
{
   VOLTAGE_DATA_ONE_BYTE = 0,
   VOLTAGE_DATA_TWO_BYTE = 1,
};


$2
{
  $2: $1;              
  $2: $1;                  
};

$2
{
   $2 header;  
   $2: $1;                     
   $2: $1;                      
   $2: $1;                      
   $2: $1;   
   $2: $1;                         
   $2 voltage_gpio_lut[] __counted_by(gpio_entry_num);
};

$2
{
   $2 header;  
   $2: $1;                        
   $2: $1;                    
   $2: $1;                          
   $2: $1;
   $2: $1;
   $2: $1; 
   $2: $1;
};

$2
{
  $2 header;  
  $2: $1;               
  u8  $1: [u8; 3];
};

$2{
  $2 gpio_voltage_obj;
  $2 i2c_voltage_obj;
  $2 svid2_voltage_obj;
  $2 merged_voltage_obj;
};

$2
{
  $2 table_header; 
  $2 $1: [u8; 1];   
};


   

   

$2
{
  u32 sclkfreqin10khz:24;
  u32 engineflag:8;              
};

$2
{
  u32 mclkfreqin10khz:24;
  u32 memflag:8;                 
};

$2
{
  $2 engineparam;
  $2 memparam;
};

$2
{
  $2 param;
  u32 $1: [u8; 16];
};


$2
{
  b3NORMAL_ENGINE_INIT = 0,
  b3SRIOV_SKIP_ASIC_INIT = 0x02,
  b3SRIOV_LOAD_UCODE = 0x40,
};

$2
{
  b3NORMAL_MEM_INIT = 0,
  b3DRAM_SELF_REFRESH_EXIT =0x20,
};

   

$2
{
  u32 sclkfreqin10khz:24;
  u32 sclkflag:8;              
  u32 $1: [u8; 10];
};

$2
{
  $2 clockinfo;
  u32 $1: [u8; 10];
};


$2
{
  b3NORMAL_CHANGE_CLOCK = 0,
  b3FIRST_TIME_CHANGE_CLOCK = 0x08,
  b3STORE_DPM_TRAINGING = 0x40,         
};

   
$2
{
  $2: $1;          
  $2: $1;
};

   
$2
{
  u32 mclkfreqin10khz:24;
  u32 mclkflag:8;              
  u32 $1: [u8; 10];
};

$2
{
  $2 clockinfo;
  u32 $1: [u8; 10];
};


   
$2
{
  $2: $1;          
  $2: $1;
};



   

$2
{
  $2: $1;                
  $2: $1;                    
  $2: $1;                  
};


$2{
  ATOM_SET_VOLTAGE  = 0,
  ATOM_INIT_VOLTAGE_REGULATOR = 3,
  ATOM_SET_VOLTAGE_PHASE = 4,
  ATOM_GET_LEAKAGE_ID    = 8,
};

$2
{
  $2 setvoltageparam;
  u32 $1: [u8; 10];
};


   


$2 
{
  COMPUTE_GPUCLK_INPUT_FLAG_DEFAULT_GPUCLK =0x00,
  COMPUTE_GPUCLK_INPUT_FLAG_GFXCLK =0x01,
  COMPUTE_GPUCLK_INPUT_FLAG_UCLK =0x02,
};

$2
{
  u32  gpuclock_10khz:24;         
  u32  gpu_clock_type:8;          
  u32  $1: [u8; 5];
};


$2
{
  u32  gpuclock_10khz:24;              
  u32  dfs_did:8;                      
  $2: $1;                    
  $2: $1;                 
  $2: $1;
  $2: $1;
  $2: $1;
  u32  $1: [u8; 2];
};



   

$2
{
  $2: $1;
  $2: $1;
  $2: $1;
};


$2
{
  $2 efuse_info;
  $2: $1;
};


   
$2
{
  $2: $1;          
  $2: $1;             
  $2: $1;            
  $2: $1;             
};

$2 
{
  GET_SMU_CLOCK_INFO_V3_1_GET_CLOCK_FREQ       = 0,
  GET_SMU_CLOCK_INFO_V3_1_GET_PLLVCO_FREQ      = 1,
  GET_SMU_CLOCK_INFO_V3_1_GET_PLLREFCLK_FREQ   = 2,
};

$2
{
  SMU9_SYSPLL0_SMNCLK_ID   = 0,       
  SMU9_SYSPLL0_SOCCLK_ID   = 1,       
  SMU9_SYSPLL0_MP0CLK_ID   = 2,       
  SMU9_SYSPLL0_MP1CLK_ID   = 3,       
  SMU9_SYSPLL0_LCLK_ID     = 4,       
  SMU9_SYSPLL0_DCLK_ID     = 5,       
  SMU9_SYSPLL0_VCLK_ID     = 6,       
  SMU9_SYSPLL0_ECLK_ID     = 7,       
  SMU9_SYSPLL0_DCEFCLK_ID  = 8,       
  SMU9_SYSPLL0_DPREFCLK_ID = 10,      
  SMU9_SYSPLL0_DISPCLK_ID  = 11,      
};

$2 {
  SMU11_SYSPLL0_ID            = 0,
  SMU11_SYSPLL1_0_ID          = 1,
  SMU11_SYSPLL1_1_ID          = 2,
  SMU11_SYSPLL1_2_ID          = 3,
  SMU11_SYSPLL2_ID            = 4,
  SMU11_SYSPLL3_0_ID          = 5,
  SMU11_SYSPLL3_1_ID          = 6,
};

$2 {
  SMU11_SYSPLL0_ECLK_ID     = 0,       
  SMU11_SYSPLL0_SOCCLK_ID   = 1,       
  SMU11_SYSPLL0_MP0CLK_ID   = 2,       
  SMU11_SYSPLL0_DCLK_ID     = 3,       
  SMU11_SYSPLL0_VCLK_ID     = 4,       
  SMU11_SYSPLL0_DCEFCLK_ID  = 5,       
};

$2 {
  SMU11_SYSPLL1_0_UCLKA_ID   = 0,       
};

$2 {
  SMU11_SYSPLL1_0_UCLKB_ID   = 0,       
};

$2 {
  SMU11_SYSPLL1_0_FCLK_ID   = 0,        
};

$2 {
  SMU11_SYSPLL2_GFXCLK_ID   = 0,        
};

$2 {
  SMU11_SYSPLL3_0_WAFCLK_ID = 0,       
  SMU11_SYSPLL3_0_DISPCLK_ID = 1,      
  SMU11_SYSPLL3_0_DPREFCLK_ID = 2,     
};

$2 {
  SMU11_SYSPLL3_1_MP1CLK_ID = 0,       
  SMU11_SYSPLL3_1_SMNCLK_ID = 1,       
  SMU11_SYSPLL3_1_LCLK_ID = 2,         
};

$2 {
  SMU12_SYSPLL0_ID          = 0,
  SMU12_SYSPLL1_ID          = 1,
  SMU12_SYSPLL2_ID          = 2,
  SMU12_SYSPLL3_0_ID        = 3,
  SMU12_SYSPLL3_1_ID        = 4,
};

$2 {
  SMU12_SYSPLL0_SMNCLK_ID   = 0,			
  SMU12_SYSPLL0_SOCCLK_ID   = 1,			
  SMU12_SYSPLL0_MP0CLK_ID   = 2,			
  SMU12_SYSPLL0_MP1CLK_ID   = 3,			
  SMU12_SYSPLL0_MP2CLK_ID   = 4,			
  SMU12_SYSPLL0_VCLK_ID     = 5,			
  SMU12_SYSPLL0_LCLK_ID     = 6,			
  SMU12_SYSPLL0_DCLK_ID     = 7,			
  SMU12_SYSPLL0_ACLK_ID     = 8,			
  SMU12_SYSPLL0_ISPCLK_ID   = 9,			
  SMU12_SYSPLL0_SHUBCLK_ID  = 10,			
};

$2 {
  SMU12_SYSPLL1_DISPCLK_ID  = 0,      
  SMU12_SYSPLL1_DPPCLK_ID   = 1,      
  SMU12_SYSPLL1_DPREFCLK_ID = 2,      
  SMU12_SYSPLL1_DCFCLK_ID   = 3,      
};

$2 {
  SMU12_SYSPLL2_Pre_GFXCLK_ID = 0,   
};

$2 {
  SMU12_SYSPLL3_0_FCLK_ID = 0,      
};

$2 {
  SMU12_SYSPLL3_1_UMCCLK_ID = 0,    
};

$2
{
  union {
    $2: $1;
    $2: $1;
    $2: $1;
  }atom_smu_outputclkfreq;
};



   

$2 
{
  COMPUTE_MEMORY_PLL_PARAM = 1,
  COMPUTE_ENGINE_PLL_PARAM = 2,
  ADJUST_MC_SETTING_PARAM = 3,
};


$2
{
  u32  mclk_10khz:24;         
  u32  command:8;             
  $2: $1;
};


$2
{
  u32  sclk_10khz:24;         
  u32  command:8;             
  $2: $1;
  $2: $1;
};

$2
{
  $2 mclk_setting;
  $2 sclk_setting;
};



   

$2
{
  UMC60_UCODE_FUNC_ID_REINIT                 = 0,
  UMC60_UCODE_FUNC_ID_ENTER_SELFREFRESH      = 1,
  UMC60_UCODE_FUNC_ID_EXIT_SELFREFRESH       = 2,
};


$2
{
  $2: $1;
  u8 $1: [u8; 3];
  u32 $1: [u8; 5];
};


   

$2
{
    $2: $1;               

    $2: $1;                     
    $2: $1;               
                                         
    $2: $1;               
    $2: $1;                   
    $2: $1;                    
    $2: $1;           
    u8  $1: [u8; 2];    
    $2: $1;
};


$2
{
  PIXEL_CLOCK_V7_MISC_FORCE_PROG_PPLL         = 0x01,
  PIXEL_CLOCK_V7_MISC_PROG_PHYPLL             = 0x02,
  PIXEL_CLOCK_V7_MISC_YUV420_MODE             = 0x04,
  PIXEL_CLOCK_V7_MISC_DVI_DUALLINK_EN         = 0x08,
  PIXEL_CLOCK_V7_MISC_REF_DIV_SRC             = 0x30,
  PIXEL_CLOCK_V7_MISC_REF_DIV_SRC_XTALIN      = 0x00,
  PIXEL_CLOCK_V7_MISC_REF_DIV_SRC_PCIE        = 0x10,
  PIXEL_CLOCK_V7_MISC_REF_DIV_SRC_GENLK       = 0x20,
  PIXEL_CLOCK_V7_MISC_REF_DIV_SRC_REFPAD      = 0x30, 
  PIXEL_CLOCK_V7_MISC_ATOMIC_UPDATE           = 0x40,
  PIXEL_CLOCK_V7_MISC_FORCE_SS_DIS            = 0x80,
};


$2
{
  PIXEL_CLOCK_V7_DEEPCOLOR_RATIO_DIS          = 0x00,      
  PIXEL_CLOCK_V7_DEEPCOLOR_RATIO_5_4          = 0x01,      
  PIXEL_CLOCK_V7_DEEPCOLOR_RATIO_3_2          = 0x02,      
  PIXEL_CLOCK_V7_DEEPCOLOR_RATIO_2_1          = 0x03,      
};

   


$2
{
  $2: $1;                               
  $2: $1;                                 
  $2: $1;                                  
  $2: $1;                                 
  $2: $1;                                    
};


$2
{
  DCE_CLOCK_TYPE_DISPCLK                      = 0,
  DCE_CLOCK_TYPE_DPREFCLK                     = 1,
  DCE_CLOCK_TYPE_PIXELCLK                     = 2,        
};


$2
{
  DCE_CLOCK_FLAG_PLL_REFCLK_SRC_MASK          = 0x03,
  DCE_CLOCK_FLAG_PLL_REFCLK_SRC_GENERICA      = 0x00,
  DCE_CLOCK_FLAG_PLL_REFCLK_SRC_GENLK         = 0x01,
  DCE_CLOCK_FLAG_PLL_REFCLK_SRC_PCIE          = 0x02,
  DCE_CLOCK_FLAG_PLL_REFCLK_SRC_XTALIN        = 0x03,
};


$2
{
  DCE_CLOCK_FLAG_PCLK_DEEPCOLOR_RATIO_MASK    = 0x03,
  DCE_CLOCK_FLAG_PCLK_DEEPCOLOR_RATIO_DIS     = 0x00,      
  DCE_CLOCK_FLAG_PCLK_DEEPCOLOR_RATIO_5_4     = 0x01,      
  DCE_CLOCK_FLAG_PCLK_DEEPCOLOR_RATIO_3_2     = 0x02,      
  DCE_CLOCK_FLAG_PCLK_DEEPCOLOR_RATIO_2_1     = 0x03,      
  DCE_CLOCK_FLAG_PIXCLK_YUV420_MODE           = 0x04,
};

$2
{
  $2 param;
  u32 $1: [u8; 2];
};


   

   
$2
{
  $2: $1;                   
  $2: $1;                  
  $2: $1;
  $2: $1;
};

$2
{
  ATOM_BLANKING         = 1,
  ATOM_BLANKING_OFF     = 0,
};

   

   
$2
{
  $2: $1;                    
  $2: $1;                     
  u8 $1: [u8; 2];
};


   

   
$2
{
  $2: $1;                
  $2: $1;                     
  u8 $1: [u8; 2];
};

$2 
{
  $2 param;
  u32 $1: [u8; 4];
};

   

   
$2
{
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;
  $2: $1;  
  $2: $1;
  $2: $1;
  $2: $1;                   
  $2: $1;			   
  u8   $1: [u8; 2];
};


   

   
$2
{
  $2: $1;
  union {
    $2: $1;
    $2: $1;                  
  } regind_status;
  $2: $1;
  $2: $1;                    
  $2: $1;
  $2: $1;
  $2: $1;
};


$2
{
  HW_I2C_WRITE          = 1,
  HW_I2C_READ           = 0,
  I2C_2BYTE_ADDR        = 0x02,
  HW_I2C_SMBUS_BYTE_WR  = 0x04,
};


$2
{
  HW_ASSISTED_I2C_STATUS_FAILURE     =2,
  HW_ASSISTED_I2C_STATUS_SUCCESS     =1,
};


   

   

$2
{
  $2: $1;
  $2: $1;
  $2: $1;
  union {
    $2: $1;
    $2: $1;
  } aux_status_delay;
  $2: $1;
  $2: $1;                                       
};


   

   

$2
{
  $2: $1;                        
  $2: $1;                     
  $2: $1;                    
  $2: $1;                        
};


   

   


$2
{
  ATOM_ENCODER_CMD_DISABLE_DIG                  = 0,
  ATOM_ENCODER_CMD_ENABLE_DIG                   = 1,
  ATOM_ENCODER_CMD_DP_LINK_TRAINING_START       = 0x08,
  ATOM_ENCODER_CMD_DP_LINK_TRAINING_PATTERN1    = 0x09,
  ATOM_ENCODER_CMD_DP_LINK_TRAINING_PATTERN2    = 0x0a,
  ATOM_ENCODER_CMD_DP_LINK_TRAINING_PATTERN3    = 0x13,
  ATOM_ENCODER_CMD_DP_LINK_TRAINING_COMPLETE    = 0x0b,
  ATOM_ENCODER_CMD_DP_VIDEO_OFF                 = 0x0c,
  ATOM_ENCODER_CMD_DP_VIDEO_ON                  = 0x0d,
  ATOM_ENCODER_CMD_SETUP_PANEL_MODE             = 0x10,
  ATOM_ENCODER_CMD_DP_LINK_TRAINING_PATTERN4    = 0x14,
  ATOM_ENCODER_CMD_STREAM_SETUP                 = 0x0F, 
  ATOM_ENCODER_CMD_LINK_SETUP                   = 0x11, 
  ATOM_ENCODER_CMD_ENCODER_BLANK                = 0x12,
};


$2
{
  DP_PANEL_MODE_DISABLE                        = 0x00,
  DP_PANEL_MODE_ENABLE_eDP_MODE                = 0x01,
  DP_PANEL_MODE_ENABLE_LVLINK_MODE             = 0x11,
};


$2
{
  ATOM_ENCODER_CONFIG_V5_DIG0_ENCODER           = 0x00,
  ATOM_ENCODER_CONFIG_V5_DIG1_ENCODER           = 0x01,
  ATOM_ENCODER_CONFIG_V5_DIG2_ENCODER           = 0x02,
  ATOM_ENCODER_CONFIG_V5_DIG3_ENCODER           = 0x03,
  ATOM_ENCODER_CONFIG_V5_DIG4_ENCODER           = 0x04,
  ATOM_ENCODER_CONFIG_V5_DIG5_ENCODER           = 0x05,
  ATOM_ENCODER_CONFIG_V5_DIG6_ENCODER           = 0x06,
  ATOM_ENCODER_CONFIG_V5_DIG7_ENCODER           = 0x07,
};

$2
{
  $2: $1;            
  $2: $1;           
  $2: $1;          
  $2: $1;          
  $2: $1;      
  $2: $1;
  $2: $1;
  u8 $1: [u8; 2];
};

$2
{
  $2: $1;           
  $2: $1;          
  $2: $1;         
  $2: $1;         
  $2: $1;    
  $2: $1;
  $2: $1;       
  u8 $1: [u8; 2];
};

$2
{
  $2: $1;              
  $2: $1;             
  $2: $1;      
  $2: $1;    
  u32 $1: [u8; 2];
};

$2 
{
  $2: $1;           
  $2: $1;          
  u8 $1: [u8; 2];    
  u32 $1: [u8; 2];
};

$2
{
  $2  cmd_param;
  $2 stream_param;
  $2   link_param;
  $2 dppanel_param;
};

   
$2
{
  $2: $1;           
  $2: $1;          
  union {
    $2: $1;        
    $2: $1;      
  } mode_laneset;
  $2: $1;        
  $2: $1;   
  $2: $1;         
  $2: $1;      
  $2: $1;     
  $2: $1;
  $2: $1;
};

$2
{
  $2 param;
  u32 $1: [u8; 4];
};


$2
{
  ATOM_TRANSMITTER_ACTION_DISABLE                 = 0,
  ATOM_TRANSMITTER_ACTION_ENABLE                  = 1,
  ATOM_TRANSMITTER_ACTION_LCD_BLOFF               = 2,
  ATOM_TRANSMITTER_ACTION_LCD_BLON                = 3,
  ATOM_TRANSMITTER_ACTION_BL_BRIGHTNESS_CONTROL   = 4,
  ATOM_TRANSMITTER_ACTION_LCD_SELFTEST_START      = 5,
  ATOM_TRANSMITTER_ACTION_LCD_SELFTEST_STOP       = 6,
  ATOM_TRANSMITTER_ACTION_INIT                    = 7,
  ATOM_TRANSMITTER_ACTION_DISABLE_OUTPUT          = 8,
  ATOM_TRANSMITTER_ACTION_ENABLE_OUTPUT           = 9,
  ATOM_TRANSMITTER_ACTION_SETUP                   = 10,
  ATOM_TRANSMITTER_ACTION_SETUP_VSEMPH            = 11,
  ATOM_TRANSMITTER_ACTION_POWER_ON                = 12,
  ATOM_TRANSMITTER_ACTION_POWER_OFF               = 13,
};


$2
{
  ATOM_TRANMSITTER_V6__DIGA_SEL                   = 0x01,
  ATOM_TRANMSITTER_V6__DIGB_SEL                   = 0x02,
  ATOM_TRANMSITTER_V6__DIGC_SEL                   = 0x04,
  ATOM_TRANMSITTER_V6__DIGD_SEL                   = 0x08,
  ATOM_TRANMSITTER_V6__DIGE_SEL                   = 0x10,
  ATOM_TRANMSITTER_V6__DIGF_SEL                   = 0x20,
  ATOM_TRANMSITTER_V6__DIGG_SEL                   = 0x40,
};



$2
{
  ATOM_TRANSMITTER_V6_NO_HPD_SEL                  = 0x00,
  ATOM_TRANSMITTER_V6_HPD1_SEL                    = 0x01,
  ATOM_TRANSMITTER_V6_HPD2_SEL                    = 0x02,
  ATOM_TRANSMITTER_V6_HPD3_SEL                    = 0x03,
  ATOM_TRANSMITTER_V6_HPD4_SEL                    = 0x04,
  ATOM_TRANSMITTER_V6_HPD5_SEL                    = 0x05,
  ATOM_TRANSMITTER_V6_HPD6_SEL                    = 0x06,
};


$2
{
  DP_LANE_SET__0DB_0_4V                           = 0x00,
  DP_LANE_SET__0DB_0_6V                           = 0x01,
  DP_LANE_SET__0DB_0_8V                           = 0x02,
  DP_LANE_SET__0DB_1_2V                           = 0x03,
  DP_LANE_SET__3_5DB_0_4V                         = 0x08, 
  DP_LANE_SET__3_5DB_0_6V                         = 0x09,
  DP_LANE_SET__3_5DB_0_8V                         = 0x0a,
  DP_LANE_SET__6DB_0_4V                           = 0x10,
  DP_LANE_SET__6DB_0_6V                           = 0x11,
  DP_LANE_SET__9_5DB_0_4V                         = 0x18, 
};



 

   

$2
{
  $2: $1;  
  $2: $1;            
  $2: $1;            
  $2: $1;       
  $2: $1;           
  $2: $1;       
  $2: $1;        
};



$2
{
  EXTERNAL_ENCODER_ACTION_V3_DISABLE_OUTPUT           = 0x00,
  EXTERNAL_ENCODER_ACTION_V3_ENABLE_OUTPUT            = 0x01,
  EXTERNAL_ENCODER_ACTION_V3_ENCODER_INIT             = 0x07,
  EXTERNAL_ENCODER_ACTION_V3_ENCODER_SETUP            = 0x0f,
  EXTERNAL_ENCODER_ACTION_V3_ENCODER_BLANKING_OFF     = 0x10,
  EXTERNAL_ENCODER_ACTION_V3_ENCODER_BLANKING         = 0x11,
  EXTERNAL_ENCODER_ACTION_V3_DACLOAD_DETECTION        = 0x12,
  EXTERNAL_ENCODER_ACTION_V3_DDC_SETUP                = 0x14,
};


$2
{
  EXTERNAL_ENCODER_CONFIG_V3_DPLINKRATE_MASK          = 0x03,
  EXTERNAL_ENCODER_CONFIG_V3_DPLINKRATE_1_62GHZ       = 0x00,
  EXTERNAL_ENCODER_CONFIG_V3_DPLINKRATE_2_70GHZ       = 0x01,
  EXTERNAL_ENCODER_CONFIG_V3_DPLINKRATE_5_40GHZ       = 0x02,
  EXTERNAL_ENCODER_CONFIG_V3_DPLINKRATE_3_24GHZ       = 0x03,  
  EXTERNAL_ENCODER_CONFIG_V3_ENCODER_SEL_MAKS         = 0x70,
  EXTERNAL_ENCODER_CONFIG_V3_ENCODER1                 = 0x00,
  EXTERNAL_ENCODER_CONFIG_V3_ENCODER2                 = 0x10,
  EXTERNAL_ENCODER_CONFIG_V3_ENCODER3                 = 0x20,
};

$2
{
  $2 sExtEncoder;
  u32 $1: [u8; 2];
};


   

$2{
  $2: $1;
  $2: $1;      
  $2: $1;
  $2: $1;
  u8  $1: [u8; 6];
  u8  $1: [u8; 8];    
  $2: $1;
  $2: $1;
  $2: $1;
};

$2{
  $2 sheader;
  u8  $1: [u8; 16];    
  $2: $1; 
  $2: $1;  
  u32 $1: [u8; 4];      
};

$2{
  $2: $1;          
  $2: $1;       
  $2: $1;     
  $2: $1;        
  $2: $1;        
  $2: $1;           
  $2: $1;            
  $2: $1;        
  $2: $1;     
};


$2 {
  $2 vbiosheader;
  u8                  $1: [u8; 1];
};

$2 {
  $2 lib1header;
  u8                  $1: [u8; 1];
};



   

$2{
  ATOM_DEVICE_CONNECT_INFO_DEF      = 0,
  ATOM_BL_BRI_LEVEL_INFO_DEF        = 2,
  ATOM_ACTIVE_INFO_DEF              = 3,
  ATOM_LCD_INFO_DEF                 = 4,
  ATOM_DEVICE_REQ_INFO_DEF          = 5,
  ATOM_ACC_CHANGE_INFO_DEF          = 6,
  ATOM_PRE_OS_MODE_INFO_DEF         = 7,
  ATOM_PRE_OS_ASSERTION_DEF      = 8,    
  ATOM_INTERNAL_TIMER_INFO_DEF      = 10,
};

$2{
  ATOM_DISPLAY_LCD1_CONNECT           =0x0002,
  ATOM_DISPLAY_DFP1_CONNECT           =0x0008,
  ATOM_DISPLAY_DFP2_CONNECT           =0x0080,
  ATOM_DISPLAY_DFP3_CONNECT           =0x0200,
  ATOM_DISPLAY_DFP4_CONNECT           =0x0400,
  ATOM_DISPLAY_DFP5_CONNECT           =0x0800,
  ATOM_DISPLAY_DFP6_CONNECT           =0x0040,
  ATOM_DISPLAY_DFPx_CONNECT           =0x0ec8,
  ATOM_CONNECT_INFO_DEVICE_MASK       =0x0fff,
};

$2{
  ATOM_CURRENT_BL_LEVEL_SHIFT         =0x8,

  ATOM_CURRENT_BL_LEVEL_MASK          =0x0000ff00,
  ATOM_DEVICE_DPMS_STATE              =0x00010000,

};

$2{
  ATOM_DISPLAY_LCD1_ACTIVE            =0x0002,
  ATOM_DISPLAY_DFP1_ACTIVE            =0x0008,
  ATOM_DISPLAY_DFP2_ACTIVE            =0x0080,
  ATOM_DISPLAY_DFP3_ACTIVE            =0x0200,
  ATOM_DISPLAY_DFP4_ACTIVE            =0x0400,
  ATOM_DISPLAY_DFP5_ACTIVE            =0x0800,
  ATOM_DISPLAY_DFP6_ACTIVE            =0x0040,
  ATOM_ACTIVE_INFO_DEVICE_MASK        =0x0fff,
};

$2{
  ATOM_DISPLAY_LCD1_REQ               =0x0002,
  ATOM_DISPLAY_DFP1_REQ               =0x0008,
  ATOM_DISPLAY_DFP2_REQ               =0x0080,
  ATOM_DISPLAY_DFP3_REQ               =0x0200,
  ATOM_DISPLAY_DFP4_REQ               =0x0400,
  ATOM_DISPLAY_DFP5_REQ               =0x0800,
  ATOM_DISPLAY_DFP6_REQ               =0x0040,
  ATOM_REQ_INFO_DEVICE_MASK           =0x0fff,
};

$2{
  ATOM_ACC_CHANGE_ACC_MODE_SHIFT    =4,
  ATOM_ACC_CHANGE_LID_STATUS_SHIFT  =6,
};

$2{
  ATOM_ACC_CHANGE_ACC_MODE          =0x00000010,
  ATOM_ACC_CHANGE_LID_STATUS        =0x00000040,
};

$2{
  ATOM_PRE_OS_MODE_MASK             =0x00000003,
  ATOM_PRE_OS_MODE_VGA              =0x00000000,
  ATOM_PRE_OS_MODE_VESA             =0x00000001,
  ATOM_PRE_OS_MODE_GOP              =0x00000002,
  ATOM_PRE_OS_MODE_PIXEL_DEPTH      =0x0000000C,
  ATOM_PRE_OS_MODE_PIXEL_FORMAT_MASK=0x000000F0,
  ATOM_PRE_OS_MODE_8BIT_PAL_EN      =0x00000100,
  ATOM_ASIC_INIT_COMPLETE           =0x00000200,

  ATOM_PRE_OS_MODE_NUMBER_MASK      =0xFFFF0000,

};





// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
