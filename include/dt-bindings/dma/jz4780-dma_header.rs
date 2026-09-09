/*
 * Request type numbers for the JZ4780 DMA controller (written to the DRTn
 * register for the channel).
 */
pub const JZ4780_DMA_I2S1_TX: u32 = 0x4;
pub const JZ4780_DMA_I2S1_RX: u32 = 0x5;
pub const JZ4780_DMA_I2S0_TX: u32 = 0x6;
pub const JZ4780_DMA_I2S0_RX: u32 = 0x7;
pub const JZ4780_DMA_AUTO: u32 = 0x8;
pub const JZ4780_DMA_SADC_RX: u32 = 0x9;
pub const JZ4780_DMA_UART4_TX: u32 = 0xc;
pub const JZ4780_DMA_UART4_RX: u32 = 0xd;
pub const JZ4780_DMA_UART3_TX: u32 = 0xe;
pub const JZ4780_DMA_UART3_RX: u32 = 0xf;
pub const JZ4780_DMA_UART2_TX: u32 = 0x10;
pub const JZ4780_DMA_UART2_RX: u32 = 0x11;
pub const JZ4780_DMA_UART1_TX: u32 = 0x12;
pub const JZ4780_DMA_UART1_RX: u32 = 0x13;
pub const JZ4780_DMA_UART0_TX: u32 = 0x14;
pub const JZ4780_DMA_UART0_RX: u32 = 0x15;
pub const JZ4780_DMA_SSI0_TX: u32 = 0x16;
pub const JZ4780_DMA_SSI0_RX: u32 = 0x17;
pub const JZ4780_DMA_SSI1_TX: u32 = 0x18;
pub const JZ4780_DMA_SSI1_RX: u32 = 0x19;
pub const JZ4780_DMA_MSC0_TX: u32 = 0x1a;
pub const JZ4780_DMA_MSC0_RX: u32 = 0x1b;
pub const JZ4780_DMA_MSC1_TX: u32 = 0x1c;
pub const JZ4780_DMA_MSC1_RX: u32 = 0x1d;
pub const JZ4780_DMA_MSC2_TX: u32 = 0x1e;
pub const JZ4780_DMA_MSC2_RX: u32 = 0x1f;
pub const JZ4780_DMA_PCM0_TX: u32 = 0x20;
pub const JZ4780_DMA_PCM0_RX: u32 = 0x21;
pub const JZ4780_DMA_SMB0_TX: u32 = 0x24;
pub const JZ4780_DMA_SMB0_RX: u32 = 0x25;
pub const JZ4780_DMA_SMB1_TX: u32 = 0x26;
pub const JZ4780_DMA_SMB1_RX: u32 = 0x27;
pub const JZ4780_DMA_SMB2_TX: u32 = 0x28;
pub const JZ4780_DMA_SMB2_RX: u32 = 0x29;
pub const JZ4780_DMA_SMB3_TX: u32 = 0x2a;
pub const JZ4780_DMA_SMB3_RX: u32 = 0x2b;
pub const JZ4780_DMA_SMB4_TX: u32 = 0x2c;
pub const JZ4780_DMA_SMB4_RX: u32 = 0x2d;
pub const JZ4780_DMA_DES_TX: u32 = 0x2e;
pub const JZ4780_DMA_DES_RX: u32 = 0x2f;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
