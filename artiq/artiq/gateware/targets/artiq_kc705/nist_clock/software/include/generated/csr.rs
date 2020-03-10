// Include this file as:
//     include!(concat!(env!("BUILDINC_DIRECTORY"), "/generated/csr.rs"));
#[allow(dead_code)]
pub mod csr {
  pub const RTIO_BASE: *mut u32 = 0xa0000000 as *mut u32;

  pub mod rtio {
    #[allow(unused_imports)]
    use core::ptr::{read_volatile, write_volatile};

    pub const TARGET_ADDR: *mut u32 = 0xa0000000 as *mut u32;
    pub const TARGET_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn target_read() -> u32 {
      read_volatile(TARGET_ADDR) as u32
    }

    #[inline(always)]
    pub unsafe fn target_write(w: u32) {
      write_volatile(TARGET_ADDR.offset(0), (w) as u32);
    }

    pub const NOW_HI_ADDR: *mut u32 = 0xa0000004 as *mut u32;
    pub const NOW_HI_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn now_hi_read() -> u32 {
      read_volatile(NOW_HI_ADDR) as u32
    }

    #[inline(always)]
    pub unsafe fn now_hi_write(w: u32) {
      write_volatile(NOW_HI_ADDR.offset(0), (w) as u32);
    }

    pub const NOW_LO_ADDR: *mut u32 = 0xa0000008 as *mut u32;
    pub const NOW_LO_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn now_lo_read() -> u32 {
      read_volatile(NOW_LO_ADDR) as u32
    }

    #[inline(always)]
    pub unsafe fn now_lo_write(w: u32) {
      write_volatile(NOW_LO_ADDR.offset(0), (w) as u32);
    }

    pub const O_DATA_ADDR: *mut u32 = 0xa000000c as *mut u32;
    pub const O_DATA_SIZE: usize = 16;

    pub const O_STATUS_ADDR: *mut u32 = 0xa000004c as *mut u32;
    pub const O_STATUS_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn o_status_read() -> u8 {
      read_volatile(O_STATUS_ADDR) as u8
    }

    pub const I_TIMEOUT_ADDR: *mut u32 = 0xa0000050 as *mut u32;
    pub const I_TIMEOUT_SIZE: usize = 2;

    #[inline(always)]
    pub unsafe fn i_timeout_read() -> u64 {
      let r = read_volatile(I_TIMEOUT_ADDR) as u64;
      let r = r << 32 | read_volatile(I_TIMEOUT_ADDR.offset(1)) as u64;
      r
    }

    #[inline(always)]
    pub unsafe fn i_timeout_write(w: u64) {
      write_volatile(I_TIMEOUT_ADDR.offset(0), (w >> 32) as u32);
      write_volatile(I_TIMEOUT_ADDR.offset(1), (w) as u32);
    }

    pub const I_DATA_ADDR: *mut u32 = 0xa0000058 as *mut u32;
    pub const I_DATA_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn i_data_read() -> u32 {
      read_volatile(I_DATA_ADDR) as u32
    }

    pub const I_TIMESTAMP_ADDR: *mut u32 = 0xa000005c as *mut u32;
    pub const I_TIMESTAMP_SIZE: usize = 2;

    #[inline(always)]
    pub unsafe fn i_timestamp_read() -> u64 {
      let r = read_volatile(I_TIMESTAMP_ADDR) as u64;
      let r = r << 32 | read_volatile(I_TIMESTAMP_ADDR.offset(1)) as u64;
      r
    }

    pub const I_STATUS_ADDR: *mut u32 = 0xa0000064 as *mut u32;
    pub const I_STATUS_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn i_status_read() -> u8 {
      read_volatile(I_STATUS_ADDR) as u8
    }

    pub const I_OVERFLOW_RESET_ADDR: *mut u32 = 0xa0000068 as *mut u32;
    pub const I_OVERFLOW_RESET_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn i_overflow_reset_read() -> u8 {
      read_volatile(I_OVERFLOW_RESET_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn i_overflow_reset_write(w: u8) {
      write_volatile(I_OVERFLOW_RESET_ADDR.offset(0), (w) as u32);
    }

    pub const COUNTER_ADDR: *mut u32 = 0xa000006c as *mut u32;
    pub const COUNTER_SIZE: usize = 2;

    #[inline(always)]
    pub unsafe fn counter_read() -> u64 {
      let r = read_volatile(COUNTER_ADDR) as u64;
      let r = r << 32 | read_volatile(COUNTER_ADDR.offset(1)) as u64;
      r
    }

    pub const COUNTER_UPDATE_ADDR: *mut u32 = 0xa0000074 as *mut u32;
    pub const COUNTER_UPDATE_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn counter_update_read() -> u8 {
      read_volatile(COUNTER_UPDATE_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn counter_update_write(w: u8) {
      write_volatile(COUNTER_UPDATE_ADDR.offset(0), (w) as u32);
    }

  }

  pub const RTIO_DMA_BASE: *mut u32 = 0xb0000000 as *mut u32;

  pub mod rtio_dma {
    #[allow(unused_imports)]
    use core::ptr::{read_volatile, write_volatile};

    pub const ENABLE_ADDR: *mut u32 = 0xb0000000 as *mut u32;
    pub const ENABLE_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn enable_read() -> u8 {
      read_volatile(ENABLE_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn enable_write(w: u8) {
      write_volatile(ENABLE_ADDR.offset(0), (w) as u32);
    }

    pub const BASE_ADDRESS_ADDR: *mut u32 = 0xb0000004 as *mut u32;
    pub const BASE_ADDRESS_SIZE: usize = 2;

    #[inline(always)]
    pub unsafe fn base_address_read() -> u64 {
      let r = read_volatile(BASE_ADDRESS_ADDR) as u64;
      let r = r << 32 | read_volatile(BASE_ADDRESS_ADDR.offset(1)) as u64;
      r
    }

    #[inline(always)]
    pub unsafe fn base_address_write(w: u64) {
      write_volatile(BASE_ADDRESS_ADDR.offset(0), (w >> 32) as u32);
      write_volatile(BASE_ADDRESS_ADDR.offset(1), (w) as u32);
    }

    pub const TIME_OFFSET_ADDR: *mut u32 = 0xb000000c as *mut u32;
    pub const TIME_OFFSET_SIZE: usize = 2;

    #[inline(always)]
    pub unsafe fn time_offset_read() -> u64 {
      let r = read_volatile(TIME_OFFSET_ADDR) as u64;
      let r = r << 32 | read_volatile(TIME_OFFSET_ADDR.offset(1)) as u64;
      r
    }

    #[inline(always)]
    pub unsafe fn time_offset_write(w: u64) {
      write_volatile(TIME_OFFSET_ADDR.offset(0), (w >> 32) as u32);
      write_volatile(TIME_OFFSET_ADDR.offset(1), (w) as u32);
    }

    pub const ERROR_ADDR: *mut u32 = 0xb0000014 as *mut u32;
    pub const ERROR_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn error_read() -> u8 {
      read_volatile(ERROR_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn error_write(w: u8) {
      write_volatile(ERROR_ADDR.offset(0), (w) as u32);
    }

    pub const ERROR_CHANNEL_ADDR: *mut u32 = 0xb0000018 as *mut u32;
    pub const ERROR_CHANNEL_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn error_channel_read() -> u32 {
      read_volatile(ERROR_CHANNEL_ADDR) as u32
    }

    pub const ERROR_TIMESTAMP_ADDR: *mut u32 = 0xb000001c as *mut u32;
    pub const ERROR_TIMESTAMP_SIZE: usize = 2;

    #[inline(always)]
    pub unsafe fn error_timestamp_read() -> u64 {
      let r = read_volatile(ERROR_TIMESTAMP_ADDR) as u64;
      let r = r << 32 | read_volatile(ERROR_TIMESTAMP_ADDR.offset(1)) as u64;
      r
    }

    pub const ERROR_ADDRESS_ADDR: *mut u32 = 0xb0000024 as *mut u32;
    pub const ERROR_ADDRESS_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn error_address_read() -> u16 {
      read_volatile(ERROR_ADDRESS_ADDR) as u16
    }

  }

  pub const CRI_CON_BASE: *mut u32 = 0x90000000 as *mut u32;

  pub mod cri_con {
    #[allow(unused_imports)]
    use core::ptr::{read_volatile, write_volatile};

    pub const SELECTED_ADDR: *mut u32 = 0x90000000 as *mut u32;
    pub const SELECTED_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn selected_read() -> u8 {
      read_volatile(SELECTED_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn selected_write(w: u8) {
      write_volatile(SELECTED_ADDR.offset(0), (w) as u32);
    }

  }

  pub const DDRPHY_BASE: *mut u32 = 0xe0003800 as *mut u32;

  pub mod ddrphy {
    #[allow(unused_imports)]
    use core::ptr::{read_volatile, write_volatile};

    pub const WLEVEL_EN_ADDR: *mut u32 = 0xe0003800 as *mut u32;
    pub const WLEVEL_EN_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn wlevel_en_read() -> u8 {
      read_volatile(WLEVEL_EN_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn wlevel_en_write(w: u8) {
      write_volatile(WLEVEL_EN_ADDR.offset(0), (w) as u32);
    }

    pub const WLEVEL_STROBE_ADDR: *mut u32 = 0xe0003804 as *mut u32;
    pub const WLEVEL_STROBE_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn wlevel_strobe_read() -> u8 {
      read_volatile(WLEVEL_STROBE_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn wlevel_strobe_write(w: u8) {
      write_volatile(WLEVEL_STROBE_ADDR.offset(0), (w) as u32);
    }

    pub const DLY_SEL_ADDR: *mut u32 = 0xe0003808 as *mut u32;
    pub const DLY_SEL_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn dly_sel_read() -> u8 {
      read_volatile(DLY_SEL_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn dly_sel_write(w: u8) {
      write_volatile(DLY_SEL_ADDR.offset(0), (w) as u32);
    }

    pub const RDLY_DQ_RST_ADDR: *mut u32 = 0xe000380c as *mut u32;
    pub const RDLY_DQ_RST_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn rdly_dq_rst_read() -> u8 {
      read_volatile(RDLY_DQ_RST_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn rdly_dq_rst_write(w: u8) {
      write_volatile(RDLY_DQ_RST_ADDR.offset(0), (w) as u32);
    }

    pub const RDLY_DQ_INC_ADDR: *mut u32 = 0xe0003810 as *mut u32;
    pub const RDLY_DQ_INC_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn rdly_dq_inc_read() -> u8 {
      read_volatile(RDLY_DQ_INC_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn rdly_dq_inc_write(w: u8) {
      write_volatile(RDLY_DQ_INC_ADDR.offset(0), (w) as u32);
    }

    pub const RDLY_DQ_BITSLIP_ADDR: *mut u32 = 0xe0003814 as *mut u32;
    pub const RDLY_DQ_BITSLIP_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn rdly_dq_bitslip_read() -> u8 {
      read_volatile(RDLY_DQ_BITSLIP_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn rdly_dq_bitslip_write(w: u8) {
      write_volatile(RDLY_DQ_BITSLIP_ADDR.offset(0), (w) as u32);
    }

    pub const WDLY_DQ_RST_ADDR: *mut u32 = 0xe0003818 as *mut u32;
    pub const WDLY_DQ_RST_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn wdly_dq_rst_read() -> u8 {
      read_volatile(WDLY_DQ_RST_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn wdly_dq_rst_write(w: u8) {
      write_volatile(WDLY_DQ_RST_ADDR.offset(0), (w) as u32);
    }

    pub const WDLY_DQ_INC_ADDR: *mut u32 = 0xe000381c as *mut u32;
    pub const WDLY_DQ_INC_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn wdly_dq_inc_read() -> u8 {
      read_volatile(WDLY_DQ_INC_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn wdly_dq_inc_write(w: u8) {
      write_volatile(WDLY_DQ_INC_ADDR.offset(0), (w) as u32);
    }

    pub const WDLY_DQS_RST_ADDR: *mut u32 = 0xe0003820 as *mut u32;
    pub const WDLY_DQS_RST_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn wdly_dqs_rst_read() -> u8 {
      read_volatile(WDLY_DQS_RST_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn wdly_dqs_rst_write(w: u8) {
      write_volatile(WDLY_DQS_RST_ADDR.offset(0), (w) as u32);
    }

    pub const WDLY_DQS_INC_ADDR: *mut u32 = 0xe0003824 as *mut u32;
    pub const WDLY_DQS_INC_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn wdly_dqs_inc_read() -> u8 {
      read_volatile(WDLY_DQS_INC_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn wdly_dqs_inc_write(w: u8) {
      write_volatile(WDLY_DQS_INC_ADDR.offset(0), (w) as u32);
    }

  }

  pub const DFII_BASE: *mut u32 = 0xe0002800 as *mut u32;

  pub mod dfii {
    #[allow(unused_imports)]
    use core::ptr::{read_volatile, write_volatile};

    pub const CONTROL_ADDR: *mut u32 = 0xe0002800 as *mut u32;
    pub const CONTROL_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn control_read() -> u8 {
      read_volatile(CONTROL_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn control_write(w: u8) {
      write_volatile(CONTROL_ADDR.offset(0), (w) as u32);
    }

    pub const PI0_COMMAND_ADDR: *mut u32 = 0xe0002804 as *mut u32;
    pub const PI0_COMMAND_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn pi0_command_read() -> u8 {
      read_volatile(PI0_COMMAND_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn pi0_command_write(w: u8) {
      write_volatile(PI0_COMMAND_ADDR.offset(0), (w) as u32);
    }

    pub const PI0_COMMAND_ISSUE_ADDR: *mut u32 = 0xe0002808 as *mut u32;
    pub const PI0_COMMAND_ISSUE_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn pi0_command_issue_read() -> u8 {
      read_volatile(PI0_COMMAND_ISSUE_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn pi0_command_issue_write(w: u8) {
      write_volatile(PI0_COMMAND_ISSUE_ADDR.offset(0), (w) as u32);
    }

    pub const PI0_ADDRESS_ADDR: *mut u32 = 0xe000280c as *mut u32;
    pub const PI0_ADDRESS_SIZE: usize = 2;

    #[inline(always)]
    pub unsafe fn pi0_address_read() -> u16 {
      let r = read_volatile(PI0_ADDRESS_ADDR) as u16;
      let r = r << 8 | read_volatile(PI0_ADDRESS_ADDR.offset(1)) as u16;
      r
    }

    #[inline(always)]
    pub unsafe fn pi0_address_write(w: u16) {
      write_volatile(PI0_ADDRESS_ADDR.offset(0), (w >> 8) as u32);
      write_volatile(PI0_ADDRESS_ADDR.offset(1), (w) as u32);
    }

    pub const PI0_BADDRESS_ADDR: *mut u32 = 0xe0002814 as *mut u32;
    pub const PI0_BADDRESS_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn pi0_baddress_read() -> u8 {
      read_volatile(PI0_BADDRESS_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn pi0_baddress_write(w: u8) {
      write_volatile(PI0_BADDRESS_ADDR.offset(0), (w) as u32);
    }

    pub const PI0_WRDATA_ADDR: *mut u32 = 0xe0002818 as *mut u32;
    pub const PI0_WRDATA_SIZE: usize = 16;

    pub const PI0_RDDATA_ADDR: *mut u32 = 0xe0002858 as *mut u32;
    pub const PI0_RDDATA_SIZE: usize = 16;

    pub const PI1_COMMAND_ADDR: *mut u32 = 0xe0002898 as *mut u32;
    pub const PI1_COMMAND_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn pi1_command_read() -> u8 {
      read_volatile(PI1_COMMAND_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn pi1_command_write(w: u8) {
      write_volatile(PI1_COMMAND_ADDR.offset(0), (w) as u32);
    }

    pub const PI1_COMMAND_ISSUE_ADDR: *mut u32 = 0xe000289c as *mut u32;
    pub const PI1_COMMAND_ISSUE_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn pi1_command_issue_read() -> u8 {
      read_volatile(PI1_COMMAND_ISSUE_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn pi1_command_issue_write(w: u8) {
      write_volatile(PI1_COMMAND_ISSUE_ADDR.offset(0), (w) as u32);
    }

    pub const PI1_ADDRESS_ADDR: *mut u32 = 0xe00028a0 as *mut u32;
    pub const PI1_ADDRESS_SIZE: usize = 2;

    #[inline(always)]
    pub unsafe fn pi1_address_read() -> u16 {
      let r = read_volatile(PI1_ADDRESS_ADDR) as u16;
      let r = r << 8 | read_volatile(PI1_ADDRESS_ADDR.offset(1)) as u16;
      r
    }

    #[inline(always)]
    pub unsafe fn pi1_address_write(w: u16) {
      write_volatile(PI1_ADDRESS_ADDR.offset(0), (w >> 8) as u32);
      write_volatile(PI1_ADDRESS_ADDR.offset(1), (w) as u32);
    }

    pub const PI1_BADDRESS_ADDR: *mut u32 = 0xe00028a8 as *mut u32;
    pub const PI1_BADDRESS_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn pi1_baddress_read() -> u8 {
      read_volatile(PI1_BADDRESS_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn pi1_baddress_write(w: u8) {
      write_volatile(PI1_BADDRESS_ADDR.offset(0), (w) as u32);
    }

    pub const PI1_WRDATA_ADDR: *mut u32 = 0xe00028ac as *mut u32;
    pub const PI1_WRDATA_SIZE: usize = 16;

    pub const PI1_RDDATA_ADDR: *mut u32 = 0xe00028ec as *mut u32;
    pub const PI1_RDDATA_SIZE: usize = 16;

    pub const PI2_COMMAND_ADDR: *mut u32 = 0xe000292c as *mut u32;
    pub const PI2_COMMAND_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn pi2_command_read() -> u8 {
      read_volatile(PI2_COMMAND_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn pi2_command_write(w: u8) {
      write_volatile(PI2_COMMAND_ADDR.offset(0), (w) as u32);
    }

    pub const PI2_COMMAND_ISSUE_ADDR: *mut u32 = 0xe0002930 as *mut u32;
    pub const PI2_COMMAND_ISSUE_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn pi2_command_issue_read() -> u8 {
      read_volatile(PI2_COMMAND_ISSUE_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn pi2_command_issue_write(w: u8) {
      write_volatile(PI2_COMMAND_ISSUE_ADDR.offset(0), (w) as u32);
    }

    pub const PI2_ADDRESS_ADDR: *mut u32 = 0xe0002934 as *mut u32;
    pub const PI2_ADDRESS_SIZE: usize = 2;

    #[inline(always)]
    pub unsafe fn pi2_address_read() -> u16 {
      let r = read_volatile(PI2_ADDRESS_ADDR) as u16;
      let r = r << 8 | read_volatile(PI2_ADDRESS_ADDR.offset(1)) as u16;
      r
    }

    #[inline(always)]
    pub unsafe fn pi2_address_write(w: u16) {
      write_volatile(PI2_ADDRESS_ADDR.offset(0), (w >> 8) as u32);
      write_volatile(PI2_ADDRESS_ADDR.offset(1), (w) as u32);
    }

    pub const PI2_BADDRESS_ADDR: *mut u32 = 0xe000293c as *mut u32;
    pub const PI2_BADDRESS_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn pi2_baddress_read() -> u8 {
      read_volatile(PI2_BADDRESS_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn pi2_baddress_write(w: u8) {
      write_volatile(PI2_BADDRESS_ADDR.offset(0), (w) as u32);
    }

    pub const PI2_WRDATA_ADDR: *mut u32 = 0xe0002940 as *mut u32;
    pub const PI2_WRDATA_SIZE: usize = 16;

    pub const PI2_RDDATA_ADDR: *mut u32 = 0xe0002980 as *mut u32;
    pub const PI2_RDDATA_SIZE: usize = 16;

    pub const PI3_COMMAND_ADDR: *mut u32 = 0xe00029c0 as *mut u32;
    pub const PI3_COMMAND_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn pi3_command_read() -> u8 {
      read_volatile(PI3_COMMAND_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn pi3_command_write(w: u8) {
      write_volatile(PI3_COMMAND_ADDR.offset(0), (w) as u32);
    }

    pub const PI3_COMMAND_ISSUE_ADDR: *mut u32 = 0xe00029c4 as *mut u32;
    pub const PI3_COMMAND_ISSUE_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn pi3_command_issue_read() -> u8 {
      read_volatile(PI3_COMMAND_ISSUE_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn pi3_command_issue_write(w: u8) {
      write_volatile(PI3_COMMAND_ISSUE_ADDR.offset(0), (w) as u32);
    }

    pub const PI3_ADDRESS_ADDR: *mut u32 = 0xe00029c8 as *mut u32;
    pub const PI3_ADDRESS_SIZE: usize = 2;

    #[inline(always)]
    pub unsafe fn pi3_address_read() -> u16 {
      let r = read_volatile(PI3_ADDRESS_ADDR) as u16;
      let r = r << 8 | read_volatile(PI3_ADDRESS_ADDR.offset(1)) as u16;
      r
    }

    #[inline(always)]
    pub unsafe fn pi3_address_write(w: u16) {
      write_volatile(PI3_ADDRESS_ADDR.offset(0), (w >> 8) as u32);
      write_volatile(PI3_ADDRESS_ADDR.offset(1), (w) as u32);
    }

    pub const PI3_BADDRESS_ADDR: *mut u32 = 0xe00029d0 as *mut u32;
    pub const PI3_BADDRESS_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn pi3_baddress_read() -> u8 {
      read_volatile(PI3_BADDRESS_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn pi3_baddress_write(w: u8) {
      write_volatile(PI3_BADDRESS_ADDR.offset(0), (w) as u32);
    }

    pub const PI3_WRDATA_ADDR: *mut u32 = 0xe00029d4 as *mut u32;
    pub const PI3_WRDATA_SIZE: usize = 16;

    pub const PI3_RDDATA_ADDR: *mut u32 = 0xe0002a14 as *mut u32;
    pub const PI3_RDDATA_SIZE: usize = 16;

  }

  pub const ETHMAC_BASE: *mut u32 = 0xe0005000 as *mut u32;

  pub mod ethmac {
    #[allow(unused_imports)]
    use core::ptr::{read_volatile, write_volatile};

    pub const SRAM_WRITER_SLOT_ADDR: *mut u32 = 0xe0005000 as *mut u32;
    pub const SRAM_WRITER_SLOT_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn sram_writer_slot_read() -> u8 {
      read_volatile(SRAM_WRITER_SLOT_ADDR) as u8
    }

    pub const SRAM_WRITER_LENGTH_ADDR: *mut u32 = 0xe0005004 as *mut u32;
    pub const SRAM_WRITER_LENGTH_SIZE: usize = 4;

    #[inline(always)]
    pub unsafe fn sram_writer_length_read() -> u32 {
      let r = read_volatile(SRAM_WRITER_LENGTH_ADDR) as u32;
      let r = r << 8 | read_volatile(SRAM_WRITER_LENGTH_ADDR.offset(1)) as u32;
      let r = r << 8 | read_volatile(SRAM_WRITER_LENGTH_ADDR.offset(2)) as u32;
      let r = r << 8 | read_volatile(SRAM_WRITER_LENGTH_ADDR.offset(3)) as u32;
      r
    }

    pub const SRAM_WRITER_ERRORS_ADDR: *mut u32 = 0xe0005014 as *mut u32;
    pub const SRAM_WRITER_ERRORS_SIZE: usize = 4;

    #[inline(always)]
    pub unsafe fn sram_writer_errors_read() -> u32 {
      let r = read_volatile(SRAM_WRITER_ERRORS_ADDR) as u32;
      let r = r << 8 | read_volatile(SRAM_WRITER_ERRORS_ADDR.offset(1)) as u32;
      let r = r << 8 | read_volatile(SRAM_WRITER_ERRORS_ADDR.offset(2)) as u32;
      let r = r << 8 | read_volatile(SRAM_WRITER_ERRORS_ADDR.offset(3)) as u32;
      r
    }

    pub const SRAM_WRITER_EV_STATUS_ADDR: *mut u32 = 0xe0005024 as *mut u32;
    pub const SRAM_WRITER_EV_STATUS_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn sram_writer_ev_status_read() -> u8 {
      read_volatile(SRAM_WRITER_EV_STATUS_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn sram_writer_ev_status_write(w: u8) {
      write_volatile(SRAM_WRITER_EV_STATUS_ADDR.offset(0), (w) as u32);
    }

    pub const SRAM_WRITER_EV_PENDING_ADDR: *mut u32 = 0xe0005028 as *mut u32;
    pub const SRAM_WRITER_EV_PENDING_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn sram_writer_ev_pending_read() -> u8 {
      read_volatile(SRAM_WRITER_EV_PENDING_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn sram_writer_ev_pending_write(w: u8) {
      write_volatile(SRAM_WRITER_EV_PENDING_ADDR.offset(0), (w) as u32);
    }

    pub const SRAM_WRITER_EV_ENABLE_ADDR: *mut u32 = 0xe000502c as *mut u32;
    pub const SRAM_WRITER_EV_ENABLE_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn sram_writer_ev_enable_read() -> u8 {
      read_volatile(SRAM_WRITER_EV_ENABLE_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn sram_writer_ev_enable_write(w: u8) {
      write_volatile(SRAM_WRITER_EV_ENABLE_ADDR.offset(0), (w) as u32);
    }

    pub const SRAM_READER_START_ADDR: *mut u32 = 0xe0005030 as *mut u32;
    pub const SRAM_READER_START_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn sram_reader_start_read() -> u8 {
      read_volatile(SRAM_READER_START_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn sram_reader_start_write(w: u8) {
      write_volatile(SRAM_READER_START_ADDR.offset(0), (w) as u32);
    }

    pub const SRAM_READER_READY_ADDR: *mut u32 = 0xe0005034 as *mut u32;
    pub const SRAM_READER_READY_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn sram_reader_ready_read() -> u8 {
      read_volatile(SRAM_READER_READY_ADDR) as u8
    }

    pub const SRAM_READER_SLOT_ADDR: *mut u32 = 0xe0005038 as *mut u32;
    pub const SRAM_READER_SLOT_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn sram_reader_slot_read() -> u8 {
      read_volatile(SRAM_READER_SLOT_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn sram_reader_slot_write(w: u8) {
      write_volatile(SRAM_READER_SLOT_ADDR.offset(0), (w) as u32);
    }

    pub const SRAM_READER_LENGTH_ADDR: *mut u32 = 0xe000503c as *mut u32;
    pub const SRAM_READER_LENGTH_SIZE: usize = 2;

    #[inline(always)]
    pub unsafe fn sram_reader_length_read() -> u16 {
      let r = read_volatile(SRAM_READER_LENGTH_ADDR) as u16;
      let r = r << 8 | read_volatile(SRAM_READER_LENGTH_ADDR.offset(1)) as u16;
      r
    }

    #[inline(always)]
    pub unsafe fn sram_reader_length_write(w: u16) {
      write_volatile(SRAM_READER_LENGTH_ADDR.offset(0), (w >> 8) as u32);
      write_volatile(SRAM_READER_LENGTH_ADDR.offset(1), (w) as u32);
    }

    pub const SRAM_READER_EV_STATUS_ADDR: *mut u32 = 0xe0005044 as *mut u32;
    pub const SRAM_READER_EV_STATUS_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn sram_reader_ev_status_read() -> u8 {
      read_volatile(SRAM_READER_EV_STATUS_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn sram_reader_ev_status_write(w: u8) {
      write_volatile(SRAM_READER_EV_STATUS_ADDR.offset(0), (w) as u32);
    }

    pub const SRAM_READER_EV_PENDING_ADDR: *mut u32 = 0xe0005048 as *mut u32;
    pub const SRAM_READER_EV_PENDING_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn sram_reader_ev_pending_read() -> u8 {
      read_volatile(SRAM_READER_EV_PENDING_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn sram_reader_ev_pending_write(w: u8) {
      write_volatile(SRAM_READER_EV_PENDING_ADDR.offset(0), (w) as u32);
    }

    pub const SRAM_READER_EV_ENABLE_ADDR: *mut u32 = 0xe000504c as *mut u32;
    pub const SRAM_READER_EV_ENABLE_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn sram_reader_ev_enable_read() -> u8 {
      read_volatile(SRAM_READER_EV_ENABLE_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn sram_reader_ev_enable_write(w: u8) {
      write_volatile(SRAM_READER_EV_ENABLE_ADDR.offset(0), (w) as u32);
    }

    pub const PREAMBLE_ERRORS_ADDR: *mut u32 = 0xe0005050 as *mut u32;
    pub const PREAMBLE_ERRORS_SIZE: usize = 4;

    #[inline(always)]
    pub unsafe fn preamble_errors_read() -> u32 {
      let r = read_volatile(PREAMBLE_ERRORS_ADDR) as u32;
      let r = r << 8 | read_volatile(PREAMBLE_ERRORS_ADDR.offset(1)) as u32;
      let r = r << 8 | read_volatile(PREAMBLE_ERRORS_ADDR.offset(2)) as u32;
      let r = r << 8 | read_volatile(PREAMBLE_ERRORS_ADDR.offset(3)) as u32;
      r
    }

    pub const CRC_ERRORS_ADDR: *mut u32 = 0xe0005060 as *mut u32;
    pub const CRC_ERRORS_SIZE: usize = 4;

    #[inline(always)]
    pub unsafe fn crc_errors_read() -> u32 {
      let r = read_volatile(CRC_ERRORS_ADDR) as u32;
      let r = r << 8 | read_volatile(CRC_ERRORS_ADDR.offset(1)) as u32;
      let r = r << 8 | read_volatile(CRC_ERRORS_ADDR.offset(2)) as u32;
      let r = r << 8 | read_volatile(CRC_ERRORS_ADDR.offset(3)) as u32;
      r
    }

  }

  pub const ETHPHY_BASE: *mut u32 = 0xe0004800 as *mut u32;

  pub mod ethphy {
    #[allow(unused_imports)]
    use core::ptr::{read_volatile, write_volatile};

    pub const MODE_DETECTION_MODE_ADDR: *mut u32 = 0xe0004800 as *mut u32;
    pub const MODE_DETECTION_MODE_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn mode_detection_mode_read() -> u8 {
      read_volatile(MODE_DETECTION_MODE_ADDR) as u8
    }

    pub const CRG_RESET_ADDR: *mut u32 = 0xe0004804 as *mut u32;
    pub const CRG_RESET_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn crg_reset_read() -> u8 {
      read_volatile(CRG_RESET_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn crg_reset_write(w: u8) {
      write_volatile(CRG_RESET_ADDR.offset(0), (w) as u32);
    }

  }

  pub const I2C_BASE: *mut u32 = 0xe0007000 as *mut u32;

  pub mod i2c {
    #[allow(unused_imports)]
    use core::ptr::{read_volatile, write_volatile};

    pub const IN_ADDR: *mut u32 = 0xe0007000 as *mut u32;
    pub const IN_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn in_read() -> u8 {
      read_volatile(IN_ADDR) as u8
    }

    pub const OUT_ADDR: *mut u32 = 0xe0007004 as *mut u32;
    pub const OUT_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn out_read() -> u8 {
      read_volatile(OUT_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn out_write(w: u8) {
      write_volatile(OUT_ADDR.offset(0), (w) as u32);
    }

    pub const OE_ADDR: *mut u32 = 0xe0007008 as *mut u32;
    pub const OE_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn oe_read() -> u8 {
      read_volatile(OE_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn oe_write(w: u8) {
      write_volatile(OE_ADDR.offset(0), (w) as u32);
    }

  }

  pub const IDENTIFIER_BASE: *mut u32 = 0xe0001000 as *mut u32;

  pub mod identifier {
    #[allow(unused_imports)]
    use core::ptr::{read_volatile, write_volatile};

    pub const ADDRESS_ADDR: *mut u32 = 0xe0001000 as *mut u32;
    pub const ADDRESS_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn address_read() -> u8 {
      read_volatile(ADDRESS_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn address_write(w: u8) {
      write_volatile(ADDRESS_ADDR.offset(0), (w) as u32);
    }

    pub const DATA_ADDR: *mut u32 = 0xe0001004 as *mut u32;
    pub const DATA_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn data_read() -> u8 {
      read_volatile(DATA_ADDR) as u8
    }

  }

  pub const KERNEL_CPU_BASE: *mut u32 = 0xe0005800 as *mut u32;

  pub mod kernel_cpu {
    #[allow(unused_imports)]
    use core::ptr::{read_volatile, write_volatile};

    pub const RESET_ADDR: *mut u32 = 0xe0005800 as *mut u32;
    pub const RESET_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn reset_read() -> u8 {
      read_volatile(RESET_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn reset_write(w: u8) {
      write_volatile(RESET_ADDR.offset(0), (w) as u32);
    }

  }

  pub const LEDS_BASE: *mut u32 = 0xe0006800 as *mut u32;

  pub mod leds {
    #[allow(unused_imports)]
    use core::ptr::{read_volatile, write_volatile};

    pub const OUT_ADDR: *mut u32 = 0xe0006800 as *mut u32;
    pub const OUT_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn out_read() -> u8 {
      read_volatile(OUT_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn out_write(w: u8) {
      write_volatile(OUT_ADDR.offset(0), (w) as u32);
    }

  }

  pub const RTIO_ANALYZER_BASE: *mut u32 = 0xe0009000 as *mut u32;

  pub mod rtio_analyzer {
    #[allow(unused_imports)]
    use core::ptr::{read_volatile, write_volatile};

    pub const ENABLE_ADDR: *mut u32 = 0xe0009000 as *mut u32;
    pub const ENABLE_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn enable_read() -> u8 {
      read_volatile(ENABLE_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn enable_write(w: u8) {
      write_volatile(ENABLE_ADDR.offset(0), (w) as u32);
    }

    pub const BUSY_ADDR: *mut u32 = 0xe0009004 as *mut u32;
    pub const BUSY_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn busy_read() -> u8 {
      read_volatile(BUSY_ADDR) as u8
    }

    pub const MESSAGE_ENCODER_OVERFLOW_ADDR: *mut u32 = 0xe0009008 as *mut u32;
    pub const MESSAGE_ENCODER_OVERFLOW_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn message_encoder_overflow_read() -> u8 {
      read_volatile(MESSAGE_ENCODER_OVERFLOW_ADDR) as u8
    }

    pub const MESSAGE_ENCODER_OVERFLOW_RESET_ADDR: *mut u32 = 0xe000900c as *mut u32;
    pub const MESSAGE_ENCODER_OVERFLOW_RESET_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn message_encoder_overflow_reset_read() -> u8 {
      read_volatile(MESSAGE_ENCODER_OVERFLOW_RESET_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn message_encoder_overflow_reset_write(w: u8) {
      write_volatile(MESSAGE_ENCODER_OVERFLOW_RESET_ADDR.offset(0), (w) as u32);
    }

    pub const DMA_RESET_ADDR: *mut u32 = 0xe0009010 as *mut u32;
    pub const DMA_RESET_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn dma_reset_read() -> u8 {
      read_volatile(DMA_RESET_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn dma_reset_write(w: u8) {
      write_volatile(DMA_RESET_ADDR.offset(0), (w) as u32);
    }

    pub const DMA_BASE_ADDRESS_ADDR: *mut u32 = 0xe0009014 as *mut u32;
    pub const DMA_BASE_ADDRESS_SIZE: usize = 5;

    #[inline(always)]
    pub unsafe fn dma_base_address_read() -> u64 {
      let r = read_volatile(DMA_BASE_ADDRESS_ADDR) as u64;
      let r = r << 8 | read_volatile(DMA_BASE_ADDRESS_ADDR.offset(1)) as u64;
      let r = r << 8 | read_volatile(DMA_BASE_ADDRESS_ADDR.offset(2)) as u64;
      let r = r << 8 | read_volatile(DMA_BASE_ADDRESS_ADDR.offset(3)) as u64;
      let r = r << 8 | read_volatile(DMA_BASE_ADDRESS_ADDR.offset(4)) as u64;
      r
    }

    #[inline(always)]
    pub unsafe fn dma_base_address_write(w: u64) {
      write_volatile(DMA_BASE_ADDRESS_ADDR.offset(0), (w >> 32) as u32);
      write_volatile(DMA_BASE_ADDRESS_ADDR.offset(1), (w >> 24) as u32);
      write_volatile(DMA_BASE_ADDRESS_ADDR.offset(2), (w >> 16) as u32);
      write_volatile(DMA_BASE_ADDRESS_ADDR.offset(3), (w >> 8) as u32);
      write_volatile(DMA_BASE_ADDRESS_ADDR.offset(4), (w) as u32);
    }

    pub const DMA_LAST_ADDRESS_ADDR: *mut u32 = 0xe0009028 as *mut u32;
    pub const DMA_LAST_ADDRESS_SIZE: usize = 5;

    #[inline(always)]
    pub unsafe fn dma_last_address_read() -> u64 {
      let r = read_volatile(DMA_LAST_ADDRESS_ADDR) as u64;
      let r = r << 8 | read_volatile(DMA_LAST_ADDRESS_ADDR.offset(1)) as u64;
      let r = r << 8 | read_volatile(DMA_LAST_ADDRESS_ADDR.offset(2)) as u64;
      let r = r << 8 | read_volatile(DMA_LAST_ADDRESS_ADDR.offset(3)) as u64;
      let r = r << 8 | read_volatile(DMA_LAST_ADDRESS_ADDR.offset(4)) as u64;
      r
    }

    #[inline(always)]
    pub unsafe fn dma_last_address_write(w: u64) {
      write_volatile(DMA_LAST_ADDRESS_ADDR.offset(0), (w >> 32) as u32);
      write_volatile(DMA_LAST_ADDRESS_ADDR.offset(1), (w >> 24) as u32);
      write_volatile(DMA_LAST_ADDRESS_ADDR.offset(2), (w >> 16) as u32);
      write_volatile(DMA_LAST_ADDRESS_ADDR.offset(3), (w >> 8) as u32);
      write_volatile(DMA_LAST_ADDRESS_ADDR.offset(4), (w) as u32);
    }

    pub const DMA_BYTE_COUNT_ADDR: *mut u32 = 0xe000903c as *mut u32;
    pub const DMA_BYTE_COUNT_SIZE: usize = 8;

    #[inline(always)]
    pub unsafe fn dma_byte_count_read() -> u64 {
      let r = read_volatile(DMA_BYTE_COUNT_ADDR) as u64;
      let r = r << 8 | read_volatile(DMA_BYTE_COUNT_ADDR.offset(1)) as u64;
      let r = r << 8 | read_volatile(DMA_BYTE_COUNT_ADDR.offset(2)) as u64;
      let r = r << 8 | read_volatile(DMA_BYTE_COUNT_ADDR.offset(3)) as u64;
      let r = r << 8 | read_volatile(DMA_BYTE_COUNT_ADDR.offset(4)) as u64;
      let r = r << 8 | read_volatile(DMA_BYTE_COUNT_ADDR.offset(5)) as u64;
      let r = r << 8 | read_volatile(DMA_BYTE_COUNT_ADDR.offset(6)) as u64;
      let r = r << 8 | read_volatile(DMA_BYTE_COUNT_ADDR.offset(7)) as u64;
      r
    }

  }

  pub const RTIO_CORE_BASE: *mut u32 = 0xe0008000 as *mut u32;

  pub mod rtio_core {
    #[allow(unused_imports)]
    use core::ptr::{read_volatile, write_volatile};

    pub const RESET_ADDR: *mut u32 = 0xe0008000 as *mut u32;
    pub const RESET_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn reset_read() -> u8 {
      read_volatile(RESET_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn reset_write(w: u8) {
      write_volatile(RESET_ADDR.offset(0), (w) as u32);
    }

    pub const RESET_PHY_ADDR: *mut u32 = 0xe0008004 as *mut u32;
    pub const RESET_PHY_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn reset_phy_read() -> u8 {
      read_volatile(RESET_PHY_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn reset_phy_write(w: u8) {
      write_volatile(RESET_PHY_ADDR.offset(0), (w) as u32);
    }

    pub const ASYNC_ERROR_ADDR: *mut u32 = 0xe0008008 as *mut u32;
    pub const ASYNC_ERROR_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn async_error_read() -> u8 {
      read_volatile(ASYNC_ERROR_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn async_error_write(w: u8) {
      write_volatile(ASYNC_ERROR_ADDR.offset(0), (w) as u32);
    }

    pub const COLLISION_CHANNEL_ADDR: *mut u32 = 0xe000800c as *mut u32;
    pub const COLLISION_CHANNEL_SIZE: usize = 2;

    #[inline(always)]
    pub unsafe fn collision_channel_read() -> u16 {
      let r = read_volatile(COLLISION_CHANNEL_ADDR) as u16;
      let r = r << 8 | read_volatile(COLLISION_CHANNEL_ADDR.offset(1)) as u16;
      r
    }

    pub const BUSY_CHANNEL_ADDR: *mut u32 = 0xe0008014 as *mut u32;
    pub const BUSY_CHANNEL_SIZE: usize = 2;

    #[inline(always)]
    pub unsafe fn busy_channel_read() -> u16 {
      let r = read_volatile(BUSY_CHANNEL_ADDR) as u16;
      let r = r << 8 | read_volatile(BUSY_CHANNEL_ADDR.offset(1)) as u16;
      r
    }

    pub const SEQUENCE_ERROR_CHANNEL_ADDR: *mut u32 = 0xe000801c as *mut u32;
    pub const SEQUENCE_ERROR_CHANNEL_SIZE: usize = 2;

    #[inline(always)]
    pub unsafe fn sequence_error_channel_read() -> u16 {
      let r = read_volatile(SEQUENCE_ERROR_CHANNEL_ADDR) as u16;
      let r = r << 8 | read_volatile(SEQUENCE_ERROR_CHANNEL_ADDR.offset(1)) as u16;
      r
    }

  }

  pub const RTIO_CRG_BASE: *mut u32 = 0xe0007800 as *mut u32;

  pub mod rtio_crg {
    #[allow(unused_imports)]
    use core::ptr::{read_volatile, write_volatile};

    pub const CLOCK_SEL_ADDR: *mut u32 = 0xe0007800 as *mut u32;
    pub const CLOCK_SEL_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn clock_sel_read() -> u8 {
      read_volatile(CLOCK_SEL_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn clock_sel_write(w: u8) {
      write_volatile(CLOCK_SEL_ADDR.offset(0), (w) as u32);
    }

    pub const PLL_RESET_ADDR: *mut u32 = 0xe0007804 as *mut u32;
    pub const PLL_RESET_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn pll_reset_read() -> u8 {
      read_volatile(PLL_RESET_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn pll_reset_write(w: u8) {
      write_volatile(PLL_RESET_ADDR.offset(0), (w) as u32);
    }

    pub const PLL_LOCKED_ADDR: *mut u32 = 0xe0007808 as *mut u32;
    pub const PLL_LOCKED_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn pll_locked_read() -> u8 {
      read_volatile(PLL_LOCKED_ADDR) as u8
    }

  }

  pub const RTIO_MONINJ_BASE: *mut u32 = 0xe0008800 as *mut u32;

  pub mod rtio_moninj {
    #[allow(unused_imports)]
    use core::ptr::{read_volatile, write_volatile};

    pub const MON_CHAN_SEL_ADDR: *mut u32 = 0xe0008800 as *mut u32;
    pub const MON_CHAN_SEL_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn mon_chan_sel_read() -> u8 {
      read_volatile(MON_CHAN_SEL_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn mon_chan_sel_write(w: u8) {
      write_volatile(MON_CHAN_SEL_ADDR.offset(0), (w) as u32);
    }

    pub const MON_PROBE_SEL_ADDR: *mut u32 = 0xe0008804 as *mut u32;
    pub const MON_PROBE_SEL_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn mon_probe_sel_read() -> u8 {
      read_volatile(MON_PROBE_SEL_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn mon_probe_sel_write(w: u8) {
      write_volatile(MON_PROBE_SEL_ADDR.offset(0), (w) as u32);
    }

    pub const MON_VALUE_UPDATE_ADDR: *mut u32 = 0xe0008808 as *mut u32;
    pub const MON_VALUE_UPDATE_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn mon_value_update_read() -> u8 {
      read_volatile(MON_VALUE_UPDATE_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn mon_value_update_write(w: u8) {
      write_volatile(MON_VALUE_UPDATE_ADDR.offset(0), (w) as u32);
    }

    pub const MON_VALUE_ADDR: *mut u32 = 0xe000880c as *mut u32;
    pub const MON_VALUE_SIZE: usize = 4;

    #[inline(always)]
    pub unsafe fn mon_value_read() -> u32 {
      let r = read_volatile(MON_VALUE_ADDR) as u32;
      let r = r << 8 | read_volatile(MON_VALUE_ADDR.offset(1)) as u32;
      let r = r << 8 | read_volatile(MON_VALUE_ADDR.offset(2)) as u32;
      let r = r << 8 | read_volatile(MON_VALUE_ADDR.offset(3)) as u32;
      r
    }

    pub const INJ_CHAN_SEL_ADDR: *mut u32 = 0xe000881c as *mut u32;
    pub const INJ_CHAN_SEL_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn inj_chan_sel_read() -> u8 {
      read_volatile(INJ_CHAN_SEL_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn inj_chan_sel_write(w: u8) {
      write_volatile(INJ_CHAN_SEL_ADDR.offset(0), (w) as u32);
    }

    pub const INJ_OVERRIDE_SEL_ADDR: *mut u32 = 0xe0008820 as *mut u32;
    pub const INJ_OVERRIDE_SEL_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn inj_override_sel_read() -> u8 {
      read_volatile(INJ_OVERRIDE_SEL_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn inj_override_sel_write(w: u8) {
      write_volatile(INJ_OVERRIDE_SEL_ADDR.offset(0), (w) as u32);
    }

    pub const INJ_VALUE_ADDR: *mut u32 = 0xe0008824 as *mut u32;
    pub const INJ_VALUE_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn inj_value_read() -> u8 {
      read_volatile(INJ_VALUE_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn inj_value_write(w: u8) {
      write_volatile(INJ_VALUE_ADDR.offset(0), (w) as u32);
    }

  }

  pub const SPIFLASH_BASE: *mut u32 = 0xe0004000 as *mut u32;

  pub mod spiflash {
    #[allow(unused_imports)]
    use core::ptr::{read_volatile, write_volatile};

    pub const BITBANG_ADDR: *mut u32 = 0xe0004000 as *mut u32;
    pub const BITBANG_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn bitbang_read() -> u8 {
      read_volatile(BITBANG_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn bitbang_write(w: u8) {
      write_volatile(BITBANG_ADDR.offset(0), (w) as u32);
    }

    pub const MISO_ADDR: *mut u32 = 0xe0004004 as *mut u32;
    pub const MISO_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn miso_read() -> u8 {
      read_volatile(MISO_ADDR) as u8
    }

    pub const BITBANG_EN_ADDR: *mut u32 = 0xe0004008 as *mut u32;
    pub const BITBANG_EN_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn bitbang_en_read() -> u8 {
      read_volatile(BITBANG_EN_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn bitbang_en_write(w: u8) {
      write_volatile(BITBANG_EN_ADDR.offset(0), (w) as u32);
    }

  }

  pub const TIMER0_BASE: *mut u32 = 0xe0001800 as *mut u32;

  pub mod timer0 {
    #[allow(unused_imports)]
    use core::ptr::{read_volatile, write_volatile};

    pub const LOAD_ADDR: *mut u32 = 0xe0001800 as *mut u32;
    pub const LOAD_SIZE: usize = 8;

    #[inline(always)]
    pub unsafe fn load_read() -> u64 {
      let r = read_volatile(LOAD_ADDR) as u64;
      let r = r << 8 | read_volatile(LOAD_ADDR.offset(1)) as u64;
      let r = r << 8 | read_volatile(LOAD_ADDR.offset(2)) as u64;
      let r = r << 8 | read_volatile(LOAD_ADDR.offset(3)) as u64;
      let r = r << 8 | read_volatile(LOAD_ADDR.offset(4)) as u64;
      let r = r << 8 | read_volatile(LOAD_ADDR.offset(5)) as u64;
      let r = r << 8 | read_volatile(LOAD_ADDR.offset(6)) as u64;
      let r = r << 8 | read_volatile(LOAD_ADDR.offset(7)) as u64;
      r
    }

    #[inline(always)]
    pub unsafe fn load_write(w: u64) {
      write_volatile(LOAD_ADDR.offset(0), (w >> 56) as u32);
      write_volatile(LOAD_ADDR.offset(1), (w >> 48) as u32);
      write_volatile(LOAD_ADDR.offset(2), (w >> 40) as u32);
      write_volatile(LOAD_ADDR.offset(3), (w >> 32) as u32);
      write_volatile(LOAD_ADDR.offset(4), (w >> 24) as u32);
      write_volatile(LOAD_ADDR.offset(5), (w >> 16) as u32);
      write_volatile(LOAD_ADDR.offset(6), (w >> 8) as u32);
      write_volatile(LOAD_ADDR.offset(7), (w) as u32);
    }

    pub const RELOAD_ADDR: *mut u32 = 0xe0001820 as *mut u32;
    pub const RELOAD_SIZE: usize = 8;

    #[inline(always)]
    pub unsafe fn reload_read() -> u64 {
      let r = read_volatile(RELOAD_ADDR) as u64;
      let r = r << 8 | read_volatile(RELOAD_ADDR.offset(1)) as u64;
      let r = r << 8 | read_volatile(RELOAD_ADDR.offset(2)) as u64;
      let r = r << 8 | read_volatile(RELOAD_ADDR.offset(3)) as u64;
      let r = r << 8 | read_volatile(RELOAD_ADDR.offset(4)) as u64;
      let r = r << 8 | read_volatile(RELOAD_ADDR.offset(5)) as u64;
      let r = r << 8 | read_volatile(RELOAD_ADDR.offset(6)) as u64;
      let r = r << 8 | read_volatile(RELOAD_ADDR.offset(7)) as u64;
      r
    }

    #[inline(always)]
    pub unsafe fn reload_write(w: u64) {
      write_volatile(RELOAD_ADDR.offset(0), (w >> 56) as u32);
      write_volatile(RELOAD_ADDR.offset(1), (w >> 48) as u32);
      write_volatile(RELOAD_ADDR.offset(2), (w >> 40) as u32);
      write_volatile(RELOAD_ADDR.offset(3), (w >> 32) as u32);
      write_volatile(RELOAD_ADDR.offset(4), (w >> 24) as u32);
      write_volatile(RELOAD_ADDR.offset(5), (w >> 16) as u32);
      write_volatile(RELOAD_ADDR.offset(6), (w >> 8) as u32);
      write_volatile(RELOAD_ADDR.offset(7), (w) as u32);
    }

    pub const EN_ADDR: *mut u32 = 0xe0001840 as *mut u32;
    pub const EN_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn en_read() -> u8 {
      read_volatile(EN_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn en_write(w: u8) {
      write_volatile(EN_ADDR.offset(0), (w) as u32);
    }

    pub const UPDATE_VALUE_ADDR: *mut u32 = 0xe0001844 as *mut u32;
    pub const UPDATE_VALUE_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn update_value_read() -> u8 {
      read_volatile(UPDATE_VALUE_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn update_value_write(w: u8) {
      write_volatile(UPDATE_VALUE_ADDR.offset(0), (w) as u32);
    }

    pub const VALUE_ADDR: *mut u32 = 0xe0001848 as *mut u32;
    pub const VALUE_SIZE: usize = 8;

    #[inline(always)]
    pub unsafe fn value_read() -> u64 {
      let r = read_volatile(VALUE_ADDR) as u64;
      let r = r << 8 | read_volatile(VALUE_ADDR.offset(1)) as u64;
      let r = r << 8 | read_volatile(VALUE_ADDR.offset(2)) as u64;
      let r = r << 8 | read_volatile(VALUE_ADDR.offset(3)) as u64;
      let r = r << 8 | read_volatile(VALUE_ADDR.offset(4)) as u64;
      let r = r << 8 | read_volatile(VALUE_ADDR.offset(5)) as u64;
      let r = r << 8 | read_volatile(VALUE_ADDR.offset(6)) as u64;
      let r = r << 8 | read_volatile(VALUE_ADDR.offset(7)) as u64;
      r
    }

    pub const EV_STATUS_ADDR: *mut u32 = 0xe0001868 as *mut u32;
    pub const EV_STATUS_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn ev_status_read() -> u8 {
      read_volatile(EV_STATUS_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn ev_status_write(w: u8) {
      write_volatile(EV_STATUS_ADDR.offset(0), (w) as u32);
    }

    pub const EV_PENDING_ADDR: *mut u32 = 0xe000186c as *mut u32;
    pub const EV_PENDING_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn ev_pending_read() -> u8 {
      read_volatile(EV_PENDING_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn ev_pending_write(w: u8) {
      write_volatile(EV_PENDING_ADDR.offset(0), (w) as u32);
    }

    pub const EV_ENABLE_ADDR: *mut u32 = 0xe0001870 as *mut u32;
    pub const EV_ENABLE_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn ev_enable_read() -> u8 {
      read_volatile(EV_ENABLE_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn ev_enable_write(w: u8) {
      write_volatile(EV_ENABLE_ADDR.offset(0), (w) as u32);
    }

  }

  pub const TIMER1_BASE: *mut u32 = 0xe0006000 as *mut u32;

  pub mod timer1 {
    #[allow(unused_imports)]
    use core::ptr::{read_volatile, write_volatile};

    pub const LOAD_ADDR: *mut u32 = 0xe0006000 as *mut u32;
    pub const LOAD_SIZE: usize = 8;

    #[inline(always)]
    pub unsafe fn load_read() -> u64 {
      let r = read_volatile(LOAD_ADDR) as u64;
      let r = r << 8 | read_volatile(LOAD_ADDR.offset(1)) as u64;
      let r = r << 8 | read_volatile(LOAD_ADDR.offset(2)) as u64;
      let r = r << 8 | read_volatile(LOAD_ADDR.offset(3)) as u64;
      let r = r << 8 | read_volatile(LOAD_ADDR.offset(4)) as u64;
      let r = r << 8 | read_volatile(LOAD_ADDR.offset(5)) as u64;
      let r = r << 8 | read_volatile(LOAD_ADDR.offset(6)) as u64;
      let r = r << 8 | read_volatile(LOAD_ADDR.offset(7)) as u64;
      r
    }

    #[inline(always)]
    pub unsafe fn load_write(w: u64) {
      write_volatile(LOAD_ADDR.offset(0), (w >> 56) as u32);
      write_volatile(LOAD_ADDR.offset(1), (w >> 48) as u32);
      write_volatile(LOAD_ADDR.offset(2), (w >> 40) as u32);
      write_volatile(LOAD_ADDR.offset(3), (w >> 32) as u32);
      write_volatile(LOAD_ADDR.offset(4), (w >> 24) as u32);
      write_volatile(LOAD_ADDR.offset(5), (w >> 16) as u32);
      write_volatile(LOAD_ADDR.offset(6), (w >> 8) as u32);
      write_volatile(LOAD_ADDR.offset(7), (w) as u32);
    }

    pub const RELOAD_ADDR: *mut u32 = 0xe0006020 as *mut u32;
    pub const RELOAD_SIZE: usize = 8;

    #[inline(always)]
    pub unsafe fn reload_read() -> u64 {
      let r = read_volatile(RELOAD_ADDR) as u64;
      let r = r << 8 | read_volatile(RELOAD_ADDR.offset(1)) as u64;
      let r = r << 8 | read_volatile(RELOAD_ADDR.offset(2)) as u64;
      let r = r << 8 | read_volatile(RELOAD_ADDR.offset(3)) as u64;
      let r = r << 8 | read_volatile(RELOAD_ADDR.offset(4)) as u64;
      let r = r << 8 | read_volatile(RELOAD_ADDR.offset(5)) as u64;
      let r = r << 8 | read_volatile(RELOAD_ADDR.offset(6)) as u64;
      let r = r << 8 | read_volatile(RELOAD_ADDR.offset(7)) as u64;
      r
    }

    #[inline(always)]
    pub unsafe fn reload_write(w: u64) {
      write_volatile(RELOAD_ADDR.offset(0), (w >> 56) as u32);
      write_volatile(RELOAD_ADDR.offset(1), (w >> 48) as u32);
      write_volatile(RELOAD_ADDR.offset(2), (w >> 40) as u32);
      write_volatile(RELOAD_ADDR.offset(3), (w >> 32) as u32);
      write_volatile(RELOAD_ADDR.offset(4), (w >> 24) as u32);
      write_volatile(RELOAD_ADDR.offset(5), (w >> 16) as u32);
      write_volatile(RELOAD_ADDR.offset(6), (w >> 8) as u32);
      write_volatile(RELOAD_ADDR.offset(7), (w) as u32);
    }

    pub const EN_ADDR: *mut u32 = 0xe0006040 as *mut u32;
    pub const EN_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn en_read() -> u8 {
      read_volatile(EN_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn en_write(w: u8) {
      write_volatile(EN_ADDR.offset(0), (w) as u32);
    }

    pub const UPDATE_VALUE_ADDR: *mut u32 = 0xe0006044 as *mut u32;
    pub const UPDATE_VALUE_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn update_value_read() -> u8 {
      read_volatile(UPDATE_VALUE_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn update_value_write(w: u8) {
      write_volatile(UPDATE_VALUE_ADDR.offset(0), (w) as u32);
    }

    pub const VALUE_ADDR: *mut u32 = 0xe0006048 as *mut u32;
    pub const VALUE_SIZE: usize = 8;

    #[inline(always)]
    pub unsafe fn value_read() -> u64 {
      let r = read_volatile(VALUE_ADDR) as u64;
      let r = r << 8 | read_volatile(VALUE_ADDR.offset(1)) as u64;
      let r = r << 8 | read_volatile(VALUE_ADDR.offset(2)) as u64;
      let r = r << 8 | read_volatile(VALUE_ADDR.offset(3)) as u64;
      let r = r << 8 | read_volatile(VALUE_ADDR.offset(4)) as u64;
      let r = r << 8 | read_volatile(VALUE_ADDR.offset(5)) as u64;
      let r = r << 8 | read_volatile(VALUE_ADDR.offset(6)) as u64;
      let r = r << 8 | read_volatile(VALUE_ADDR.offset(7)) as u64;
      r
    }

    pub const EV_STATUS_ADDR: *mut u32 = 0xe0006068 as *mut u32;
    pub const EV_STATUS_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn ev_status_read() -> u8 {
      read_volatile(EV_STATUS_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn ev_status_write(w: u8) {
      write_volatile(EV_STATUS_ADDR.offset(0), (w) as u32);
    }

    pub const EV_PENDING_ADDR: *mut u32 = 0xe000606c as *mut u32;
    pub const EV_PENDING_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn ev_pending_read() -> u8 {
      read_volatile(EV_PENDING_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn ev_pending_write(w: u8) {
      write_volatile(EV_PENDING_ADDR.offset(0), (w) as u32);
    }

    pub const EV_ENABLE_ADDR: *mut u32 = 0xe0006070 as *mut u32;
    pub const EV_ENABLE_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn ev_enable_read() -> u8 {
      read_volatile(EV_ENABLE_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn ev_enable_write(w: u8) {
      write_volatile(EV_ENABLE_ADDR.offset(0), (w) as u32);
    }

  }

  pub const TMPU_BASE: *mut u32 = 0xe0002000 as *mut u32;

  pub mod tmpu {
    #[allow(unused_imports)]
    use core::ptr::{read_volatile, write_volatile};

    pub const ENABLE_NULL_ADDR: *mut u32 = 0xe0002000 as *mut u32;
    pub const ENABLE_NULL_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn enable_null_read() -> u8 {
      read_volatile(ENABLE_NULL_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn enable_null_write(w: u8) {
      write_volatile(ENABLE_NULL_ADDR.offset(0), (w) as u32);
    }

    pub const ENABLE_PROG_ADDR: *mut u32 = 0xe0002004 as *mut u32;
    pub const ENABLE_PROG_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn enable_prog_read() -> u8 {
      read_volatile(ENABLE_PROG_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn enable_prog_write(w: u8) {
      write_volatile(ENABLE_PROG_ADDR.offset(0), (w) as u32);
    }

    pub const PROG_ADDRESS_ADDR: *mut u32 = 0xe0002008 as *mut u32;
    pub const PROG_ADDRESS_SIZE: usize = 4;

    #[inline(always)]
    pub unsafe fn prog_address_read() -> u32 {
      let r = read_volatile(PROG_ADDRESS_ADDR) as u32;
      let r = r << 8 | read_volatile(PROG_ADDRESS_ADDR.offset(1)) as u32;
      let r = r << 8 | read_volatile(PROG_ADDRESS_ADDR.offset(2)) as u32;
      let r = r << 8 | read_volatile(PROG_ADDRESS_ADDR.offset(3)) as u32;
      r
    }

    #[inline(always)]
    pub unsafe fn prog_address_write(w: u32) {
      write_volatile(PROG_ADDRESS_ADDR.offset(0), (w >> 24) as u32);
      write_volatile(PROG_ADDRESS_ADDR.offset(1), (w >> 16) as u32);
      write_volatile(PROG_ADDRESS_ADDR.offset(2), (w >> 8) as u32);
      write_volatile(PROG_ADDRESS_ADDR.offset(3), (w) as u32);
    }

  }

  pub const UART_BASE: *mut u32 = 0xe0000800 as *mut u32;

  pub mod uart {
    #[allow(unused_imports)]
    use core::ptr::{read_volatile, write_volatile};

    pub const RXTX_ADDR: *mut u32 = 0xe0000800 as *mut u32;
    pub const RXTX_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn rxtx_read() -> u8 {
      read_volatile(RXTX_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn rxtx_write(w: u8) {
      write_volatile(RXTX_ADDR.offset(0), (w) as u32);
    }

    pub const TXFULL_ADDR: *mut u32 = 0xe0000804 as *mut u32;
    pub const TXFULL_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn txfull_read() -> u8 {
      read_volatile(TXFULL_ADDR) as u8
    }

    pub const RXEMPTY_ADDR: *mut u32 = 0xe0000808 as *mut u32;
    pub const RXEMPTY_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn rxempty_read() -> u8 {
      read_volatile(RXEMPTY_ADDR) as u8
    }

    pub const EV_STATUS_ADDR: *mut u32 = 0xe000080c as *mut u32;
    pub const EV_STATUS_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn ev_status_read() -> u8 {
      read_volatile(EV_STATUS_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn ev_status_write(w: u8) {
      write_volatile(EV_STATUS_ADDR.offset(0), (w) as u32);
    }

    pub const EV_PENDING_ADDR: *mut u32 = 0xe0000810 as *mut u32;
    pub const EV_PENDING_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn ev_pending_read() -> u8 {
      read_volatile(EV_PENDING_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn ev_pending_write(w: u8) {
      write_volatile(EV_PENDING_ADDR.offset(0), (w) as u32);
    }

    pub const EV_ENABLE_ADDR: *mut u32 = 0xe0000814 as *mut u32;
    pub const EV_ENABLE_SIZE: usize = 1;

    #[inline(always)]
    pub unsafe fn ev_enable_read() -> u8 {
      read_volatile(EV_ENABLE_ADDR) as u8
    }

    #[inline(always)]
    pub unsafe fn ev_enable_write(w: u8) {
      write_volatile(EV_ENABLE_ADDR.offset(0), (w) as u32);
    }

  }

  pub const UART_PHY_BASE: *mut u32 = 0xe0000000 as *mut u32;

  pub mod uart_phy {
    #[allow(unused_imports)]
    use core::ptr::{read_volatile, write_volatile};

    pub const TUNING_WORD_ADDR: *mut u32 = 0xe0000000 as *mut u32;
    pub const TUNING_WORD_SIZE: usize = 4;

    #[inline(always)]
    pub unsafe fn tuning_word_read() -> u32 {
      let r = read_volatile(TUNING_WORD_ADDR) as u32;
      let r = r << 8 | read_volatile(TUNING_WORD_ADDR.offset(1)) as u32;
      let r = r << 8 | read_volatile(TUNING_WORD_ADDR.offset(2)) as u32;
      let r = r << 8 | read_volatile(TUNING_WORD_ADDR.offset(3)) as u32;
      r
    }

    #[inline(always)]
    pub unsafe fn tuning_word_write(w: u32) {
      write_volatile(TUNING_WORD_ADDR.offset(0), (w >> 24) as u32);
      write_volatile(TUNING_WORD_ADDR.offset(1), (w >> 16) as u32);
      write_volatile(TUNING_WORD_ADDR.offset(2), (w >> 8) as u32);
      write_volatile(TUNING_WORD_ADDR.offset(3), (w) as u32);
    }

  }

  pub const UART_INTERRUPT: u32 = 0;
  pub const TIMER0_INTERRUPT: u32 = 1;
  pub const ETHMAC_INTERRUPT: u32 = 2;
  pub const TIMER1_INTERRUPT: u32 = 3;
  pub const ETHMAC_CORE_PREAMBLE_CRC: u32 = 1;
  pub const ETHMAC_RX_SLOTS: u32 = 4;
  pub const ETHMAC_TX_SLOTS: u32 = 4;
  pub const ETHMAC_SLOT_SIZE: u32 = 2048;
  pub const TMPU_PAGE_SIZE: u32 = 4096;
  pub const CONFIG_CLOCK_FREQUENCY: u32 = 125000000;
  pub const CONFIG_DDRPHY_WLEVEL: u32 = 1;
  pub const CONFIG_HAS_DDS: u32 = 1;
  pub const CONFIG_HAS_RTIO_CLOCK_SWITCH: u32 = 1;
  pub const CONFIG_HAS_RTIO_LOG: u32 = 1;
  pub const CONFIG_I2C_BUS_COUNT: u32 = 1;
  pub const CONFIG_IDENTIFIER_STR: &'static str = "6.7190.2a909839.beta;nist_clock";
  pub const CONFIG_L2_SIZE: u32 = 131072;
  pub const CONFIG_RTIO_LOG_CHANNEL: u32 = 28;
  pub const CONFIG_SOC_PLATFORM: &'static str = "kc705";
  pub const CONFIG_SPIFLASH_PAGE_SIZE: u32 = 256;
  pub const CONFIG_SPIFLASH_SECTOR_SIZE: u32 = 65536;
}
