#ifndef __GENERATED_CSR_H
#define __GENERATED_CSR_H
#include <hw/common.h>

/* rtio */
#define CSR_RTIO_BASE 0xa0000000
#define CSR_RTIO_TARGET_ADDR 0xa0000000
#define CSR_RTIO_TARGET_SIZE 1
static inline unsigned int rtio_target_read(void) {
	unsigned int r = MMPTR(0xa0000000);
	return r;
}
static inline void rtio_target_write(unsigned int value) {
	MMPTR(0xa0000000) = value;
}
#define CSR_RTIO_NOW_HI_ADDR 0xa0000004
#define CSR_RTIO_NOW_HI_SIZE 1
static inline unsigned int rtio_now_hi_read(void) {
	unsigned int r = MMPTR(0xa0000004);
	return r;
}
static inline void rtio_now_hi_write(unsigned int value) {
	MMPTR(0xa0000004) = value;
}
#define CSR_RTIO_NOW_LO_ADDR 0xa0000008
#define CSR_RTIO_NOW_LO_SIZE 1
static inline unsigned int rtio_now_lo_read(void) {
	unsigned int r = MMPTR(0xa0000008);
	return r;
}
static inline void rtio_now_lo_write(unsigned int value) {
	MMPTR(0xa0000008) = value;
}
#define CSR_RTIO_O_DATA_ADDR 0xa000000c
#define CSR_RTIO_O_DATA_SIZE 16
#define CSR_RTIO_O_STATUS_ADDR 0xa000004c
#define CSR_RTIO_O_STATUS_SIZE 1
static inline unsigned int rtio_o_status_read(void) {
	unsigned int r = MMPTR(0xa000004c);
	return r;
}
#define CSR_RTIO_I_TIMEOUT_ADDR 0xa0000050
#define CSR_RTIO_I_TIMEOUT_SIZE 2
static inline unsigned long long int rtio_i_timeout_read(void) {
	unsigned long long int r = MMPTR(0xa0000050);
	r <<= 32;
	r |= MMPTR(0xa0000054);
	return r;
}
static inline void rtio_i_timeout_write(unsigned long long int value) {
	MMPTR(0xa0000050) = value >> 32;
	MMPTR(0xa0000054) = value;
}
#define CSR_RTIO_I_DATA_ADDR 0xa0000058
#define CSR_RTIO_I_DATA_SIZE 1
static inline unsigned int rtio_i_data_read(void) {
	unsigned int r = MMPTR(0xa0000058);
	return r;
}
#define CSR_RTIO_I_TIMESTAMP_ADDR 0xa000005c
#define CSR_RTIO_I_TIMESTAMP_SIZE 2
static inline unsigned long long int rtio_i_timestamp_read(void) {
	unsigned long long int r = MMPTR(0xa000005c);
	r <<= 32;
	r |= MMPTR(0xa0000060);
	return r;
}
#define CSR_RTIO_I_STATUS_ADDR 0xa0000064
#define CSR_RTIO_I_STATUS_SIZE 1
static inline unsigned int rtio_i_status_read(void) {
	unsigned int r = MMPTR(0xa0000064);
	return r;
}
#define CSR_RTIO_I_OVERFLOW_RESET_ADDR 0xa0000068
#define CSR_RTIO_I_OVERFLOW_RESET_SIZE 1
static inline unsigned int rtio_i_overflow_reset_read(void) {
	unsigned int r = MMPTR(0xa0000068);
	return r;
}
static inline void rtio_i_overflow_reset_write(unsigned int value) {
	MMPTR(0xa0000068) = value;
}
#define CSR_RTIO_COUNTER_ADDR 0xa000006c
#define CSR_RTIO_COUNTER_SIZE 2
static inline unsigned long long int rtio_counter_read(void) {
	unsigned long long int r = MMPTR(0xa000006c);
	r <<= 32;
	r |= MMPTR(0xa0000070);
	return r;
}
#define CSR_RTIO_COUNTER_UPDATE_ADDR 0xa0000074
#define CSR_RTIO_COUNTER_UPDATE_SIZE 1
static inline unsigned int rtio_counter_update_read(void) {
	unsigned int r = MMPTR(0xa0000074);
	return r;
}
static inline void rtio_counter_update_write(unsigned int value) {
	MMPTR(0xa0000074) = value;
}

/* rtio_dma */
#define CSR_RTIO_DMA_BASE 0xb0000000
#define CSR_RTIO_DMA_ENABLE_ADDR 0xb0000000
#define CSR_RTIO_DMA_ENABLE_SIZE 1
static inline unsigned int rtio_dma_enable_read(void) {
	unsigned int r = MMPTR(0xb0000000);
	return r;
}
static inline void rtio_dma_enable_write(unsigned int value) {
	MMPTR(0xb0000000) = value;
}
#define CSR_RTIO_DMA_BASE_ADDRESS_ADDR 0xb0000004
#define CSR_RTIO_DMA_BASE_ADDRESS_SIZE 2
static inline unsigned long long int rtio_dma_base_address_read(void) {
	unsigned long long int r = MMPTR(0xb0000004);
	r <<= 32;
	r |= MMPTR(0xb0000008);
	return r;
}
static inline void rtio_dma_base_address_write(unsigned long long int value) {
	MMPTR(0xb0000004) = value >> 32;
	MMPTR(0xb0000008) = value;
}
#define CSR_RTIO_DMA_TIME_OFFSET_ADDR 0xb000000c
#define CSR_RTIO_DMA_TIME_OFFSET_SIZE 2
static inline unsigned long long int rtio_dma_time_offset_read(void) {
	unsigned long long int r = MMPTR(0xb000000c);
	r <<= 32;
	r |= MMPTR(0xb0000010);
	return r;
}
static inline void rtio_dma_time_offset_write(unsigned long long int value) {
	MMPTR(0xb000000c) = value >> 32;
	MMPTR(0xb0000010) = value;
}
#define CSR_RTIO_DMA_ERROR_ADDR 0xb0000014
#define CSR_RTIO_DMA_ERROR_SIZE 1
static inline unsigned int rtio_dma_error_read(void) {
	unsigned int r = MMPTR(0xb0000014);
	return r;
}
static inline void rtio_dma_error_write(unsigned int value) {
	MMPTR(0xb0000014) = value;
}
#define CSR_RTIO_DMA_ERROR_CHANNEL_ADDR 0xb0000018
#define CSR_RTIO_DMA_ERROR_CHANNEL_SIZE 1
static inline unsigned int rtio_dma_error_channel_read(void) {
	unsigned int r = MMPTR(0xb0000018);
	return r;
}
#define CSR_RTIO_DMA_ERROR_TIMESTAMP_ADDR 0xb000001c
#define CSR_RTIO_DMA_ERROR_TIMESTAMP_SIZE 2
static inline unsigned long long int rtio_dma_error_timestamp_read(void) {
	unsigned long long int r = MMPTR(0xb000001c);
	r <<= 32;
	r |= MMPTR(0xb0000020);
	return r;
}
#define CSR_RTIO_DMA_ERROR_ADDRESS_ADDR 0xb0000024
#define CSR_RTIO_DMA_ERROR_ADDRESS_SIZE 1
static inline unsigned int rtio_dma_error_address_read(void) {
	unsigned int r = MMPTR(0xb0000024);
	return r;
}

/* cri_con */
#define CSR_CRI_CON_BASE 0x90000000
#define CSR_CRI_CON_SELECTED_ADDR 0x90000000
#define CSR_CRI_CON_SELECTED_SIZE 1
static inline unsigned int cri_con_selected_read(void) {
	unsigned int r = MMPTR(0x90000000);
	return r;
}
static inline void cri_con_selected_write(unsigned int value) {
	MMPTR(0x90000000) = value;
}

/* ddrphy */
#define CSR_DDRPHY_BASE 0xe0003800
#define CSR_DDRPHY_WLEVEL_EN_ADDR 0xe0003800
#define CSR_DDRPHY_WLEVEL_EN_SIZE 1
static inline unsigned char ddrphy_wlevel_en_read(void) {
	unsigned char r = MMPTR(0xe0003800);
	return r;
}
static inline void ddrphy_wlevel_en_write(unsigned char value) {
	MMPTR(0xe0003800) = value;
}
#define CSR_DDRPHY_WLEVEL_STROBE_ADDR 0xe0003804
#define CSR_DDRPHY_WLEVEL_STROBE_SIZE 1
static inline unsigned char ddrphy_wlevel_strobe_read(void) {
	unsigned char r = MMPTR(0xe0003804);
	return r;
}
static inline void ddrphy_wlevel_strobe_write(unsigned char value) {
	MMPTR(0xe0003804) = value;
}
#define CSR_DDRPHY_DLY_SEL_ADDR 0xe0003808
#define CSR_DDRPHY_DLY_SEL_SIZE 1
static inline unsigned char ddrphy_dly_sel_read(void) {
	unsigned char r = MMPTR(0xe0003808);
	return r;
}
static inline void ddrphy_dly_sel_write(unsigned char value) {
	MMPTR(0xe0003808) = value;
}
#define CSR_DDRPHY_RDLY_DQ_RST_ADDR 0xe000380c
#define CSR_DDRPHY_RDLY_DQ_RST_SIZE 1
static inline unsigned char ddrphy_rdly_dq_rst_read(void) {
	unsigned char r = MMPTR(0xe000380c);
	return r;
}
static inline void ddrphy_rdly_dq_rst_write(unsigned char value) {
	MMPTR(0xe000380c) = value;
}
#define CSR_DDRPHY_RDLY_DQ_INC_ADDR 0xe0003810
#define CSR_DDRPHY_RDLY_DQ_INC_SIZE 1
static inline unsigned char ddrphy_rdly_dq_inc_read(void) {
	unsigned char r = MMPTR(0xe0003810);
	return r;
}
static inline void ddrphy_rdly_dq_inc_write(unsigned char value) {
	MMPTR(0xe0003810) = value;
}
#define CSR_DDRPHY_RDLY_DQ_BITSLIP_ADDR 0xe0003814
#define CSR_DDRPHY_RDLY_DQ_BITSLIP_SIZE 1
static inline unsigned char ddrphy_rdly_dq_bitslip_read(void) {
	unsigned char r = MMPTR(0xe0003814);
	return r;
}
static inline void ddrphy_rdly_dq_bitslip_write(unsigned char value) {
	MMPTR(0xe0003814) = value;
}
#define CSR_DDRPHY_WDLY_DQ_RST_ADDR 0xe0003818
#define CSR_DDRPHY_WDLY_DQ_RST_SIZE 1
static inline unsigned char ddrphy_wdly_dq_rst_read(void) {
	unsigned char r = MMPTR(0xe0003818);
	return r;
}
static inline void ddrphy_wdly_dq_rst_write(unsigned char value) {
	MMPTR(0xe0003818) = value;
}
#define CSR_DDRPHY_WDLY_DQ_INC_ADDR 0xe000381c
#define CSR_DDRPHY_WDLY_DQ_INC_SIZE 1
static inline unsigned char ddrphy_wdly_dq_inc_read(void) {
	unsigned char r = MMPTR(0xe000381c);
	return r;
}
static inline void ddrphy_wdly_dq_inc_write(unsigned char value) {
	MMPTR(0xe000381c) = value;
}
#define CSR_DDRPHY_WDLY_DQS_RST_ADDR 0xe0003820
#define CSR_DDRPHY_WDLY_DQS_RST_SIZE 1
static inline unsigned char ddrphy_wdly_dqs_rst_read(void) {
	unsigned char r = MMPTR(0xe0003820);
	return r;
}
static inline void ddrphy_wdly_dqs_rst_write(unsigned char value) {
	MMPTR(0xe0003820) = value;
}
#define CSR_DDRPHY_WDLY_DQS_INC_ADDR 0xe0003824
#define CSR_DDRPHY_WDLY_DQS_INC_SIZE 1
static inline unsigned char ddrphy_wdly_dqs_inc_read(void) {
	unsigned char r = MMPTR(0xe0003824);
	return r;
}
static inline void ddrphy_wdly_dqs_inc_write(unsigned char value) {
	MMPTR(0xe0003824) = value;
}

/* dfii */
#define CSR_DFII_BASE 0xe0002800
#define CSR_DFII_CONTROL_ADDR 0xe0002800
#define CSR_DFII_CONTROL_SIZE 1
static inline unsigned char dfii_control_read(void) {
	unsigned char r = MMPTR(0xe0002800);
	return r;
}
static inline void dfii_control_write(unsigned char value) {
	MMPTR(0xe0002800) = value;
}
#define CSR_DFII_PI0_COMMAND_ADDR 0xe0002804
#define CSR_DFII_PI0_COMMAND_SIZE 1
static inline unsigned char dfii_pi0_command_read(void) {
	unsigned char r = MMPTR(0xe0002804);
	return r;
}
static inline void dfii_pi0_command_write(unsigned char value) {
	MMPTR(0xe0002804) = value;
}
#define CSR_DFII_PI0_COMMAND_ISSUE_ADDR 0xe0002808
#define CSR_DFII_PI0_COMMAND_ISSUE_SIZE 1
static inline unsigned char dfii_pi0_command_issue_read(void) {
	unsigned char r = MMPTR(0xe0002808);
	return r;
}
static inline void dfii_pi0_command_issue_write(unsigned char value) {
	MMPTR(0xe0002808) = value;
}
#define CSR_DFII_PI0_ADDRESS_ADDR 0xe000280c
#define CSR_DFII_PI0_ADDRESS_SIZE 2
static inline unsigned short int dfii_pi0_address_read(void) {
	unsigned short int r = MMPTR(0xe000280c);
	r <<= 8;
	r |= MMPTR(0xe0002810);
	return r;
}
static inline void dfii_pi0_address_write(unsigned short int value) {
	MMPTR(0xe000280c) = value >> 8;
	MMPTR(0xe0002810) = value;
}
#define CSR_DFII_PI0_BADDRESS_ADDR 0xe0002814
#define CSR_DFII_PI0_BADDRESS_SIZE 1
static inline unsigned char dfii_pi0_baddress_read(void) {
	unsigned char r = MMPTR(0xe0002814);
	return r;
}
static inline void dfii_pi0_baddress_write(unsigned char value) {
	MMPTR(0xe0002814) = value;
}
#define CSR_DFII_PI0_WRDATA_ADDR 0xe0002818
#define CSR_DFII_PI0_WRDATA_SIZE 16
#define CSR_DFII_PI0_RDDATA_ADDR 0xe0002858
#define CSR_DFII_PI0_RDDATA_SIZE 16
#define CSR_DFII_PI1_COMMAND_ADDR 0xe0002898
#define CSR_DFII_PI1_COMMAND_SIZE 1
static inline unsigned char dfii_pi1_command_read(void) {
	unsigned char r = MMPTR(0xe0002898);
	return r;
}
static inline void dfii_pi1_command_write(unsigned char value) {
	MMPTR(0xe0002898) = value;
}
#define CSR_DFII_PI1_COMMAND_ISSUE_ADDR 0xe000289c
#define CSR_DFII_PI1_COMMAND_ISSUE_SIZE 1
static inline unsigned char dfii_pi1_command_issue_read(void) {
	unsigned char r = MMPTR(0xe000289c);
	return r;
}
static inline void dfii_pi1_command_issue_write(unsigned char value) {
	MMPTR(0xe000289c) = value;
}
#define CSR_DFII_PI1_ADDRESS_ADDR 0xe00028a0
#define CSR_DFII_PI1_ADDRESS_SIZE 2
static inline unsigned short int dfii_pi1_address_read(void) {
	unsigned short int r = MMPTR(0xe00028a0);
	r <<= 8;
	r |= MMPTR(0xe00028a4);
	return r;
}
static inline void dfii_pi1_address_write(unsigned short int value) {
	MMPTR(0xe00028a0) = value >> 8;
	MMPTR(0xe00028a4) = value;
}
#define CSR_DFII_PI1_BADDRESS_ADDR 0xe00028a8
#define CSR_DFII_PI1_BADDRESS_SIZE 1
static inline unsigned char dfii_pi1_baddress_read(void) {
	unsigned char r = MMPTR(0xe00028a8);
	return r;
}
static inline void dfii_pi1_baddress_write(unsigned char value) {
	MMPTR(0xe00028a8) = value;
}
#define CSR_DFII_PI1_WRDATA_ADDR 0xe00028ac
#define CSR_DFII_PI1_WRDATA_SIZE 16
#define CSR_DFII_PI1_RDDATA_ADDR 0xe00028ec
#define CSR_DFII_PI1_RDDATA_SIZE 16
#define CSR_DFII_PI2_COMMAND_ADDR 0xe000292c
#define CSR_DFII_PI2_COMMAND_SIZE 1
static inline unsigned char dfii_pi2_command_read(void) {
	unsigned char r = MMPTR(0xe000292c);
	return r;
}
static inline void dfii_pi2_command_write(unsigned char value) {
	MMPTR(0xe000292c) = value;
}
#define CSR_DFII_PI2_COMMAND_ISSUE_ADDR 0xe0002930
#define CSR_DFII_PI2_COMMAND_ISSUE_SIZE 1
static inline unsigned char dfii_pi2_command_issue_read(void) {
	unsigned char r = MMPTR(0xe0002930);
	return r;
}
static inline void dfii_pi2_command_issue_write(unsigned char value) {
	MMPTR(0xe0002930) = value;
}
#define CSR_DFII_PI2_ADDRESS_ADDR 0xe0002934
#define CSR_DFII_PI2_ADDRESS_SIZE 2
static inline unsigned short int dfii_pi2_address_read(void) {
	unsigned short int r = MMPTR(0xe0002934);
	r <<= 8;
	r |= MMPTR(0xe0002938);
	return r;
}
static inline void dfii_pi2_address_write(unsigned short int value) {
	MMPTR(0xe0002934) = value >> 8;
	MMPTR(0xe0002938) = value;
}
#define CSR_DFII_PI2_BADDRESS_ADDR 0xe000293c
#define CSR_DFII_PI2_BADDRESS_SIZE 1
static inline unsigned char dfii_pi2_baddress_read(void) {
	unsigned char r = MMPTR(0xe000293c);
	return r;
}
static inline void dfii_pi2_baddress_write(unsigned char value) {
	MMPTR(0xe000293c) = value;
}
#define CSR_DFII_PI2_WRDATA_ADDR 0xe0002940
#define CSR_DFII_PI2_WRDATA_SIZE 16
#define CSR_DFII_PI2_RDDATA_ADDR 0xe0002980
#define CSR_DFII_PI2_RDDATA_SIZE 16
#define CSR_DFII_PI3_COMMAND_ADDR 0xe00029c0
#define CSR_DFII_PI3_COMMAND_SIZE 1
static inline unsigned char dfii_pi3_command_read(void) {
	unsigned char r = MMPTR(0xe00029c0);
	return r;
}
static inline void dfii_pi3_command_write(unsigned char value) {
	MMPTR(0xe00029c0) = value;
}
#define CSR_DFII_PI3_COMMAND_ISSUE_ADDR 0xe00029c4
#define CSR_DFII_PI3_COMMAND_ISSUE_SIZE 1
static inline unsigned char dfii_pi3_command_issue_read(void) {
	unsigned char r = MMPTR(0xe00029c4);
	return r;
}
static inline void dfii_pi3_command_issue_write(unsigned char value) {
	MMPTR(0xe00029c4) = value;
}
#define CSR_DFII_PI3_ADDRESS_ADDR 0xe00029c8
#define CSR_DFII_PI3_ADDRESS_SIZE 2
static inline unsigned short int dfii_pi3_address_read(void) {
	unsigned short int r = MMPTR(0xe00029c8);
	r <<= 8;
	r |= MMPTR(0xe00029cc);
	return r;
}
static inline void dfii_pi3_address_write(unsigned short int value) {
	MMPTR(0xe00029c8) = value >> 8;
	MMPTR(0xe00029cc) = value;
}
#define CSR_DFII_PI3_BADDRESS_ADDR 0xe00029d0
#define CSR_DFII_PI3_BADDRESS_SIZE 1
static inline unsigned char dfii_pi3_baddress_read(void) {
	unsigned char r = MMPTR(0xe00029d0);
	return r;
}
static inline void dfii_pi3_baddress_write(unsigned char value) {
	MMPTR(0xe00029d0) = value;
}
#define CSR_DFII_PI3_WRDATA_ADDR 0xe00029d4
#define CSR_DFII_PI3_WRDATA_SIZE 16
#define CSR_DFII_PI3_RDDATA_ADDR 0xe0002a14
#define CSR_DFII_PI3_RDDATA_SIZE 16

/* ethmac */
#define CSR_ETHMAC_BASE 0xe0005000
#define CSR_ETHMAC_SRAM_WRITER_SLOT_ADDR 0xe0005000
#define CSR_ETHMAC_SRAM_WRITER_SLOT_SIZE 1
static inline unsigned char ethmac_sram_writer_slot_read(void) {
	unsigned char r = MMPTR(0xe0005000);
	return r;
}
#define CSR_ETHMAC_SRAM_WRITER_LENGTH_ADDR 0xe0005004
#define CSR_ETHMAC_SRAM_WRITER_LENGTH_SIZE 4
static inline unsigned int ethmac_sram_writer_length_read(void) {
	unsigned int r = MMPTR(0xe0005004);
	r <<= 8;
	r |= MMPTR(0xe0005008);
	r <<= 8;
	r |= MMPTR(0xe000500c);
	r <<= 8;
	r |= MMPTR(0xe0005010);
	return r;
}
#define CSR_ETHMAC_SRAM_WRITER_ERRORS_ADDR 0xe0005014
#define CSR_ETHMAC_SRAM_WRITER_ERRORS_SIZE 4
static inline unsigned int ethmac_sram_writer_errors_read(void) {
	unsigned int r = MMPTR(0xe0005014);
	r <<= 8;
	r |= MMPTR(0xe0005018);
	r <<= 8;
	r |= MMPTR(0xe000501c);
	r <<= 8;
	r |= MMPTR(0xe0005020);
	return r;
}
#define CSR_ETHMAC_SRAM_WRITER_EV_STATUS_ADDR 0xe0005024
#define CSR_ETHMAC_SRAM_WRITER_EV_STATUS_SIZE 1
static inline unsigned char ethmac_sram_writer_ev_status_read(void) {
	unsigned char r = MMPTR(0xe0005024);
	return r;
}
static inline void ethmac_sram_writer_ev_status_write(unsigned char value) {
	MMPTR(0xe0005024) = value;
}
#define CSR_ETHMAC_SRAM_WRITER_EV_PENDING_ADDR 0xe0005028
#define CSR_ETHMAC_SRAM_WRITER_EV_PENDING_SIZE 1
static inline unsigned char ethmac_sram_writer_ev_pending_read(void) {
	unsigned char r = MMPTR(0xe0005028);
	return r;
}
static inline void ethmac_sram_writer_ev_pending_write(unsigned char value) {
	MMPTR(0xe0005028) = value;
}
#define CSR_ETHMAC_SRAM_WRITER_EV_ENABLE_ADDR 0xe000502c
#define CSR_ETHMAC_SRAM_WRITER_EV_ENABLE_SIZE 1
static inline unsigned char ethmac_sram_writer_ev_enable_read(void) {
	unsigned char r = MMPTR(0xe000502c);
	return r;
}
static inline void ethmac_sram_writer_ev_enable_write(unsigned char value) {
	MMPTR(0xe000502c) = value;
}
#define CSR_ETHMAC_SRAM_READER_START_ADDR 0xe0005030
#define CSR_ETHMAC_SRAM_READER_START_SIZE 1
static inline unsigned char ethmac_sram_reader_start_read(void) {
	unsigned char r = MMPTR(0xe0005030);
	return r;
}
static inline void ethmac_sram_reader_start_write(unsigned char value) {
	MMPTR(0xe0005030) = value;
}
#define CSR_ETHMAC_SRAM_READER_READY_ADDR 0xe0005034
#define CSR_ETHMAC_SRAM_READER_READY_SIZE 1
static inline unsigned char ethmac_sram_reader_ready_read(void) {
	unsigned char r = MMPTR(0xe0005034);
	return r;
}
#define CSR_ETHMAC_SRAM_READER_SLOT_ADDR 0xe0005038
#define CSR_ETHMAC_SRAM_READER_SLOT_SIZE 1
static inline unsigned char ethmac_sram_reader_slot_read(void) {
	unsigned char r = MMPTR(0xe0005038);
	return r;
}
static inline void ethmac_sram_reader_slot_write(unsigned char value) {
	MMPTR(0xe0005038) = value;
}
#define CSR_ETHMAC_SRAM_READER_LENGTH_ADDR 0xe000503c
#define CSR_ETHMAC_SRAM_READER_LENGTH_SIZE 2
static inline unsigned short int ethmac_sram_reader_length_read(void) {
	unsigned short int r = MMPTR(0xe000503c);
	r <<= 8;
	r |= MMPTR(0xe0005040);
	return r;
}
static inline void ethmac_sram_reader_length_write(unsigned short int value) {
	MMPTR(0xe000503c) = value >> 8;
	MMPTR(0xe0005040) = value;
}
#define CSR_ETHMAC_SRAM_READER_EV_STATUS_ADDR 0xe0005044
#define CSR_ETHMAC_SRAM_READER_EV_STATUS_SIZE 1
static inline unsigned char ethmac_sram_reader_ev_status_read(void) {
	unsigned char r = MMPTR(0xe0005044);
	return r;
}
static inline void ethmac_sram_reader_ev_status_write(unsigned char value) {
	MMPTR(0xe0005044) = value;
}
#define CSR_ETHMAC_SRAM_READER_EV_PENDING_ADDR 0xe0005048
#define CSR_ETHMAC_SRAM_READER_EV_PENDING_SIZE 1
static inline unsigned char ethmac_sram_reader_ev_pending_read(void) {
	unsigned char r = MMPTR(0xe0005048);
	return r;
}
static inline void ethmac_sram_reader_ev_pending_write(unsigned char value) {
	MMPTR(0xe0005048) = value;
}
#define CSR_ETHMAC_SRAM_READER_EV_ENABLE_ADDR 0xe000504c
#define CSR_ETHMAC_SRAM_READER_EV_ENABLE_SIZE 1
static inline unsigned char ethmac_sram_reader_ev_enable_read(void) {
	unsigned char r = MMPTR(0xe000504c);
	return r;
}
static inline void ethmac_sram_reader_ev_enable_write(unsigned char value) {
	MMPTR(0xe000504c) = value;
}
#define CSR_ETHMAC_PREAMBLE_ERRORS_ADDR 0xe0005050
#define CSR_ETHMAC_PREAMBLE_ERRORS_SIZE 4
static inline unsigned int ethmac_preamble_errors_read(void) {
	unsigned int r = MMPTR(0xe0005050);
	r <<= 8;
	r |= MMPTR(0xe0005054);
	r <<= 8;
	r |= MMPTR(0xe0005058);
	r <<= 8;
	r |= MMPTR(0xe000505c);
	return r;
}
#define CSR_ETHMAC_CRC_ERRORS_ADDR 0xe0005060
#define CSR_ETHMAC_CRC_ERRORS_SIZE 4
static inline unsigned int ethmac_crc_errors_read(void) {
	unsigned int r = MMPTR(0xe0005060);
	r <<= 8;
	r |= MMPTR(0xe0005064);
	r <<= 8;
	r |= MMPTR(0xe0005068);
	r <<= 8;
	r |= MMPTR(0xe000506c);
	return r;
}

/* ethphy */
#define CSR_ETHPHY_BASE 0xe0004800
#define CSR_ETHPHY_MODE_DETECTION_MODE_ADDR 0xe0004800
#define CSR_ETHPHY_MODE_DETECTION_MODE_SIZE 1
static inline unsigned char ethphy_mode_detection_mode_read(void) {
	unsigned char r = MMPTR(0xe0004800);
	return r;
}
#define CSR_ETHPHY_CRG_RESET_ADDR 0xe0004804
#define CSR_ETHPHY_CRG_RESET_SIZE 1
static inline unsigned char ethphy_crg_reset_read(void) {
	unsigned char r = MMPTR(0xe0004804);
	return r;
}
static inline void ethphy_crg_reset_write(unsigned char value) {
	MMPTR(0xe0004804) = value;
}

/* i2c */
#define CSR_I2C_BASE 0xe0007000
#define CSR_I2C_IN_ADDR 0xe0007000
#define CSR_I2C_IN_SIZE 1
static inline unsigned char i2c_in_read(void) {
	unsigned char r = MMPTR(0xe0007000);
	return r;
}
#define CSR_I2C_OUT_ADDR 0xe0007004
#define CSR_I2C_OUT_SIZE 1
static inline unsigned char i2c_out_read(void) {
	unsigned char r = MMPTR(0xe0007004);
	return r;
}
static inline void i2c_out_write(unsigned char value) {
	MMPTR(0xe0007004) = value;
}
#define CSR_I2C_OE_ADDR 0xe0007008
#define CSR_I2C_OE_SIZE 1
static inline unsigned char i2c_oe_read(void) {
	unsigned char r = MMPTR(0xe0007008);
	return r;
}
static inline void i2c_oe_write(unsigned char value) {
	MMPTR(0xe0007008) = value;
}

/* identifier */
#define CSR_IDENTIFIER_BASE 0xe0001000
#define CSR_IDENTIFIER_ADDRESS_ADDR 0xe0001000
#define CSR_IDENTIFIER_ADDRESS_SIZE 1
static inline unsigned char identifier_address_read(void) {
	unsigned char r = MMPTR(0xe0001000);
	return r;
}
static inline void identifier_address_write(unsigned char value) {
	MMPTR(0xe0001000) = value;
}
#define CSR_IDENTIFIER_DATA_ADDR 0xe0001004
#define CSR_IDENTIFIER_DATA_SIZE 1
static inline unsigned char identifier_data_read(void) {
	unsigned char r = MMPTR(0xe0001004);
	return r;
}

/* kernel_cpu */
#define CSR_KERNEL_CPU_BASE 0xe0005800
#define CSR_KERNEL_CPU_RESET_ADDR 0xe0005800
#define CSR_KERNEL_CPU_RESET_SIZE 1
static inline unsigned char kernel_cpu_reset_read(void) {
	unsigned char r = MMPTR(0xe0005800);
	return r;
}
static inline void kernel_cpu_reset_write(unsigned char value) {
	MMPTR(0xe0005800) = value;
}

/* leds */
#define CSR_LEDS_BASE 0xe0006800
#define CSR_LEDS_OUT_ADDR 0xe0006800
#define CSR_LEDS_OUT_SIZE 1
static inline unsigned char leds_out_read(void) {
	unsigned char r = MMPTR(0xe0006800);
	return r;
}
static inline void leds_out_write(unsigned char value) {
	MMPTR(0xe0006800) = value;
}

/* rtio_analyzer */
#define CSR_RTIO_ANALYZER_BASE 0xe0009000
#define CSR_RTIO_ANALYZER_ENABLE_ADDR 0xe0009000
#define CSR_RTIO_ANALYZER_ENABLE_SIZE 1
static inline unsigned char rtio_analyzer_enable_read(void) {
	unsigned char r = MMPTR(0xe0009000);
	return r;
}
static inline void rtio_analyzer_enable_write(unsigned char value) {
	MMPTR(0xe0009000) = value;
}
#define CSR_RTIO_ANALYZER_BUSY_ADDR 0xe0009004
#define CSR_RTIO_ANALYZER_BUSY_SIZE 1
static inline unsigned char rtio_analyzer_busy_read(void) {
	unsigned char r = MMPTR(0xe0009004);
	return r;
}
#define CSR_RTIO_ANALYZER_MESSAGE_ENCODER_OVERFLOW_ADDR 0xe0009008
#define CSR_RTIO_ANALYZER_MESSAGE_ENCODER_OVERFLOW_SIZE 1
static inline unsigned char rtio_analyzer_message_encoder_overflow_read(void) {
	unsigned char r = MMPTR(0xe0009008);
	return r;
}
#define CSR_RTIO_ANALYZER_MESSAGE_ENCODER_OVERFLOW_RESET_ADDR 0xe000900c
#define CSR_RTIO_ANALYZER_MESSAGE_ENCODER_OVERFLOW_RESET_SIZE 1
static inline unsigned char rtio_analyzer_message_encoder_overflow_reset_read(void) {
	unsigned char r = MMPTR(0xe000900c);
	return r;
}
static inline void rtio_analyzer_message_encoder_overflow_reset_write(unsigned char value) {
	MMPTR(0xe000900c) = value;
}
#define CSR_RTIO_ANALYZER_DMA_RESET_ADDR 0xe0009010
#define CSR_RTIO_ANALYZER_DMA_RESET_SIZE 1
static inline unsigned char rtio_analyzer_dma_reset_read(void) {
	unsigned char r = MMPTR(0xe0009010);
	return r;
}
static inline void rtio_analyzer_dma_reset_write(unsigned char value) {
	MMPTR(0xe0009010) = value;
}
#define CSR_RTIO_ANALYZER_DMA_BASE_ADDRESS_ADDR 0xe0009014
#define CSR_RTIO_ANALYZER_DMA_BASE_ADDRESS_SIZE 5
static inline unsigned long long int rtio_analyzer_dma_base_address_read(void) {
	unsigned long long int r = MMPTR(0xe0009014);
	r <<= 8;
	r |= MMPTR(0xe0009018);
	r <<= 8;
	r |= MMPTR(0xe000901c);
	r <<= 8;
	r |= MMPTR(0xe0009020);
	r <<= 8;
	r |= MMPTR(0xe0009024);
	return r;
}
static inline void rtio_analyzer_dma_base_address_write(unsigned long long int value) {
	MMPTR(0xe0009014) = value >> 32;
	MMPTR(0xe0009018) = value >> 24;
	MMPTR(0xe000901c) = value >> 16;
	MMPTR(0xe0009020) = value >> 8;
	MMPTR(0xe0009024) = value;
}
#define CSR_RTIO_ANALYZER_DMA_LAST_ADDRESS_ADDR 0xe0009028
#define CSR_RTIO_ANALYZER_DMA_LAST_ADDRESS_SIZE 5
static inline unsigned long long int rtio_analyzer_dma_last_address_read(void) {
	unsigned long long int r = MMPTR(0xe0009028);
	r <<= 8;
	r |= MMPTR(0xe000902c);
	r <<= 8;
	r |= MMPTR(0xe0009030);
	r <<= 8;
	r |= MMPTR(0xe0009034);
	r <<= 8;
	r |= MMPTR(0xe0009038);
	return r;
}
static inline void rtio_analyzer_dma_last_address_write(unsigned long long int value) {
	MMPTR(0xe0009028) = value >> 32;
	MMPTR(0xe000902c) = value >> 24;
	MMPTR(0xe0009030) = value >> 16;
	MMPTR(0xe0009034) = value >> 8;
	MMPTR(0xe0009038) = value;
}
#define CSR_RTIO_ANALYZER_DMA_BYTE_COUNT_ADDR 0xe000903c
#define CSR_RTIO_ANALYZER_DMA_BYTE_COUNT_SIZE 8
static inline unsigned long long int rtio_analyzer_dma_byte_count_read(void) {
	unsigned long long int r = MMPTR(0xe000903c);
	r <<= 8;
	r |= MMPTR(0xe0009040);
	r <<= 8;
	r |= MMPTR(0xe0009044);
	r <<= 8;
	r |= MMPTR(0xe0009048);
	r <<= 8;
	r |= MMPTR(0xe000904c);
	r <<= 8;
	r |= MMPTR(0xe0009050);
	r <<= 8;
	r |= MMPTR(0xe0009054);
	r <<= 8;
	r |= MMPTR(0xe0009058);
	return r;
}

/* rtio_core */
#define CSR_RTIO_CORE_BASE 0xe0008000
#define CSR_RTIO_CORE_RESET_ADDR 0xe0008000
#define CSR_RTIO_CORE_RESET_SIZE 1
static inline unsigned char rtio_core_reset_read(void) {
	unsigned char r = MMPTR(0xe0008000);
	return r;
}
static inline void rtio_core_reset_write(unsigned char value) {
	MMPTR(0xe0008000) = value;
}
#define CSR_RTIO_CORE_RESET_PHY_ADDR 0xe0008004
#define CSR_RTIO_CORE_RESET_PHY_SIZE 1
static inline unsigned char rtio_core_reset_phy_read(void) {
	unsigned char r = MMPTR(0xe0008004);
	return r;
}
static inline void rtio_core_reset_phy_write(unsigned char value) {
	MMPTR(0xe0008004) = value;
}
#define CSR_RTIO_CORE_ASYNC_ERROR_ADDR 0xe0008008
#define CSR_RTIO_CORE_ASYNC_ERROR_SIZE 1
static inline unsigned char rtio_core_async_error_read(void) {
	unsigned char r = MMPTR(0xe0008008);
	return r;
}
static inline void rtio_core_async_error_write(unsigned char value) {
	MMPTR(0xe0008008) = value;
}
#define CSR_RTIO_CORE_COLLISION_CHANNEL_ADDR 0xe000800c
#define CSR_RTIO_CORE_COLLISION_CHANNEL_SIZE 2
static inline unsigned short int rtio_core_collision_channel_read(void) {
	unsigned short int r = MMPTR(0xe000800c);
	r <<= 8;
	r |= MMPTR(0xe0008010);
	return r;
}
#define CSR_RTIO_CORE_BUSY_CHANNEL_ADDR 0xe0008014
#define CSR_RTIO_CORE_BUSY_CHANNEL_SIZE 2
static inline unsigned short int rtio_core_busy_channel_read(void) {
	unsigned short int r = MMPTR(0xe0008014);
	r <<= 8;
	r |= MMPTR(0xe0008018);
	return r;
}
#define CSR_RTIO_CORE_SEQUENCE_ERROR_CHANNEL_ADDR 0xe000801c
#define CSR_RTIO_CORE_SEQUENCE_ERROR_CHANNEL_SIZE 2
static inline unsigned short int rtio_core_sequence_error_channel_read(void) {
	unsigned short int r = MMPTR(0xe000801c);
	r <<= 8;
	r |= MMPTR(0xe0008020);
	return r;
}

/* rtio_crg */
#define CSR_RTIO_CRG_BASE 0xe0007800
#define CSR_RTIO_CRG_CLOCK_SEL_ADDR 0xe0007800
#define CSR_RTIO_CRG_CLOCK_SEL_SIZE 1
static inline unsigned char rtio_crg_clock_sel_read(void) {
	unsigned char r = MMPTR(0xe0007800);
	return r;
}
static inline void rtio_crg_clock_sel_write(unsigned char value) {
	MMPTR(0xe0007800) = value;
}
#define CSR_RTIO_CRG_PLL_RESET_ADDR 0xe0007804
#define CSR_RTIO_CRG_PLL_RESET_SIZE 1
static inline unsigned char rtio_crg_pll_reset_read(void) {
	unsigned char r = MMPTR(0xe0007804);
	return r;
}
static inline void rtio_crg_pll_reset_write(unsigned char value) {
	MMPTR(0xe0007804) = value;
}
#define CSR_RTIO_CRG_PLL_LOCKED_ADDR 0xe0007808
#define CSR_RTIO_CRG_PLL_LOCKED_SIZE 1
static inline unsigned char rtio_crg_pll_locked_read(void) {
	unsigned char r = MMPTR(0xe0007808);
	return r;
}

/* rtio_moninj */
#define CSR_RTIO_MONINJ_BASE 0xe0008800
#define CSR_RTIO_MONINJ_MON_CHAN_SEL_ADDR 0xe0008800
#define CSR_RTIO_MONINJ_MON_CHAN_SEL_SIZE 1
static inline unsigned char rtio_moninj_mon_chan_sel_read(void) {
	unsigned char r = MMPTR(0xe0008800);
	return r;
}
static inline void rtio_moninj_mon_chan_sel_write(unsigned char value) {
	MMPTR(0xe0008800) = value;
}
#define CSR_RTIO_MONINJ_MON_PROBE_SEL_ADDR 0xe0008804
#define CSR_RTIO_MONINJ_MON_PROBE_SEL_SIZE 1
static inline unsigned char rtio_moninj_mon_probe_sel_read(void) {
	unsigned char r = MMPTR(0xe0008804);
	return r;
}
static inline void rtio_moninj_mon_probe_sel_write(unsigned char value) {
	MMPTR(0xe0008804) = value;
}
#define CSR_RTIO_MONINJ_MON_VALUE_UPDATE_ADDR 0xe0008808
#define CSR_RTIO_MONINJ_MON_VALUE_UPDATE_SIZE 1
static inline unsigned char rtio_moninj_mon_value_update_read(void) {
	unsigned char r = MMPTR(0xe0008808);
	return r;
}
static inline void rtio_moninj_mon_value_update_write(unsigned char value) {
	MMPTR(0xe0008808) = value;
}
#define CSR_RTIO_MONINJ_MON_VALUE_ADDR 0xe000880c
#define CSR_RTIO_MONINJ_MON_VALUE_SIZE 4
static inline unsigned int rtio_moninj_mon_value_read(void) {
	unsigned int r = MMPTR(0xe000880c);
	r <<= 8;
	r |= MMPTR(0xe0008810);
	r <<= 8;
	r |= MMPTR(0xe0008814);
	r <<= 8;
	r |= MMPTR(0xe0008818);
	return r;
}
#define CSR_RTIO_MONINJ_INJ_CHAN_SEL_ADDR 0xe000881c
#define CSR_RTIO_MONINJ_INJ_CHAN_SEL_SIZE 1
static inline unsigned char rtio_moninj_inj_chan_sel_read(void) {
	unsigned char r = MMPTR(0xe000881c);
	return r;
}
static inline void rtio_moninj_inj_chan_sel_write(unsigned char value) {
	MMPTR(0xe000881c) = value;
}
#define CSR_RTIO_MONINJ_INJ_OVERRIDE_SEL_ADDR 0xe0008820
#define CSR_RTIO_MONINJ_INJ_OVERRIDE_SEL_SIZE 1
static inline unsigned char rtio_moninj_inj_override_sel_read(void) {
	unsigned char r = MMPTR(0xe0008820);
	return r;
}
static inline void rtio_moninj_inj_override_sel_write(unsigned char value) {
	MMPTR(0xe0008820) = value;
}
#define CSR_RTIO_MONINJ_INJ_VALUE_ADDR 0xe0008824
#define CSR_RTIO_MONINJ_INJ_VALUE_SIZE 1
static inline unsigned char rtio_moninj_inj_value_read(void) {
	unsigned char r = MMPTR(0xe0008824);
	return r;
}
static inline void rtio_moninj_inj_value_write(unsigned char value) {
	MMPTR(0xe0008824) = value;
}

/* spiflash */
#define CSR_SPIFLASH_BASE 0xe0004000
#define CSR_SPIFLASH_BITBANG_ADDR 0xe0004000
#define CSR_SPIFLASH_BITBANG_SIZE 1
static inline unsigned char spiflash_bitbang_read(void) {
	unsigned char r = MMPTR(0xe0004000);
	return r;
}
static inline void spiflash_bitbang_write(unsigned char value) {
	MMPTR(0xe0004000) = value;
}
#define CSR_SPIFLASH_MISO_ADDR 0xe0004004
#define CSR_SPIFLASH_MISO_SIZE 1
static inline unsigned char spiflash_miso_read(void) {
	unsigned char r = MMPTR(0xe0004004);
	return r;
}
#define CSR_SPIFLASH_BITBANG_EN_ADDR 0xe0004008
#define CSR_SPIFLASH_BITBANG_EN_SIZE 1
static inline unsigned char spiflash_bitbang_en_read(void) {
	unsigned char r = MMPTR(0xe0004008);
	return r;
}
static inline void spiflash_bitbang_en_write(unsigned char value) {
	MMPTR(0xe0004008) = value;
}

/* timer0 */
#define CSR_TIMER0_BASE 0xe0001800
#define CSR_TIMER0_LOAD_ADDR 0xe0001800
#define CSR_TIMER0_LOAD_SIZE 8
static inline unsigned long long int timer0_load_read(void) {
	unsigned long long int r = MMPTR(0xe0001800);
	r <<= 8;
	r |= MMPTR(0xe0001804);
	r <<= 8;
	r |= MMPTR(0xe0001808);
	r <<= 8;
	r |= MMPTR(0xe000180c);
	r <<= 8;
	r |= MMPTR(0xe0001810);
	r <<= 8;
	r |= MMPTR(0xe0001814);
	r <<= 8;
	r |= MMPTR(0xe0001818);
	r <<= 8;
	r |= MMPTR(0xe000181c);
	return r;
}
static inline void timer0_load_write(unsigned long long int value) {
	MMPTR(0xe0001800) = value >> 56;
	MMPTR(0xe0001804) = value >> 48;
	MMPTR(0xe0001808) = value >> 40;
	MMPTR(0xe000180c) = value >> 32;
	MMPTR(0xe0001810) = value >> 24;
	MMPTR(0xe0001814) = value >> 16;
	MMPTR(0xe0001818) = value >> 8;
	MMPTR(0xe000181c) = value;
}
#define CSR_TIMER0_RELOAD_ADDR 0xe0001820
#define CSR_TIMER0_RELOAD_SIZE 8
static inline unsigned long long int timer0_reload_read(void) {
	unsigned long long int r = MMPTR(0xe0001820);
	r <<= 8;
	r |= MMPTR(0xe0001824);
	r <<= 8;
	r |= MMPTR(0xe0001828);
	r <<= 8;
	r |= MMPTR(0xe000182c);
	r <<= 8;
	r |= MMPTR(0xe0001830);
	r <<= 8;
	r |= MMPTR(0xe0001834);
	r <<= 8;
	r |= MMPTR(0xe0001838);
	r <<= 8;
	r |= MMPTR(0xe000183c);
	return r;
}
static inline void timer0_reload_write(unsigned long long int value) {
	MMPTR(0xe0001820) = value >> 56;
	MMPTR(0xe0001824) = value >> 48;
	MMPTR(0xe0001828) = value >> 40;
	MMPTR(0xe000182c) = value >> 32;
	MMPTR(0xe0001830) = value >> 24;
	MMPTR(0xe0001834) = value >> 16;
	MMPTR(0xe0001838) = value >> 8;
	MMPTR(0xe000183c) = value;
}
#define CSR_TIMER0_EN_ADDR 0xe0001840
#define CSR_TIMER0_EN_SIZE 1
static inline unsigned char timer0_en_read(void) {
	unsigned char r = MMPTR(0xe0001840);
	return r;
}
static inline void timer0_en_write(unsigned char value) {
	MMPTR(0xe0001840) = value;
}
#define CSR_TIMER0_UPDATE_VALUE_ADDR 0xe0001844
#define CSR_TIMER0_UPDATE_VALUE_SIZE 1
static inline unsigned char timer0_update_value_read(void) {
	unsigned char r = MMPTR(0xe0001844);
	return r;
}
static inline void timer0_update_value_write(unsigned char value) {
	MMPTR(0xe0001844) = value;
}
#define CSR_TIMER0_VALUE_ADDR 0xe0001848
#define CSR_TIMER0_VALUE_SIZE 8
static inline unsigned long long int timer0_value_read(void) {
	unsigned long long int r = MMPTR(0xe0001848);
	r <<= 8;
	r |= MMPTR(0xe000184c);
	r <<= 8;
	r |= MMPTR(0xe0001850);
	r <<= 8;
	r |= MMPTR(0xe0001854);
	r <<= 8;
	r |= MMPTR(0xe0001858);
	r <<= 8;
	r |= MMPTR(0xe000185c);
	r <<= 8;
	r |= MMPTR(0xe0001860);
	r <<= 8;
	r |= MMPTR(0xe0001864);
	return r;
}
#define CSR_TIMER0_EV_STATUS_ADDR 0xe0001868
#define CSR_TIMER0_EV_STATUS_SIZE 1
static inline unsigned char timer0_ev_status_read(void) {
	unsigned char r = MMPTR(0xe0001868);
	return r;
}
static inline void timer0_ev_status_write(unsigned char value) {
	MMPTR(0xe0001868) = value;
}
#define CSR_TIMER0_EV_PENDING_ADDR 0xe000186c
#define CSR_TIMER0_EV_PENDING_SIZE 1
static inline unsigned char timer0_ev_pending_read(void) {
	unsigned char r = MMPTR(0xe000186c);
	return r;
}
static inline void timer0_ev_pending_write(unsigned char value) {
	MMPTR(0xe000186c) = value;
}
#define CSR_TIMER0_EV_ENABLE_ADDR 0xe0001870
#define CSR_TIMER0_EV_ENABLE_SIZE 1
static inline unsigned char timer0_ev_enable_read(void) {
	unsigned char r = MMPTR(0xe0001870);
	return r;
}
static inline void timer0_ev_enable_write(unsigned char value) {
	MMPTR(0xe0001870) = value;
}

/* timer1 */
#define CSR_TIMER1_BASE 0xe0006000
#define CSR_TIMER1_LOAD_ADDR 0xe0006000
#define CSR_TIMER1_LOAD_SIZE 8
static inline unsigned long long int timer1_load_read(void) {
	unsigned long long int r = MMPTR(0xe0006000);
	r <<= 8;
	r |= MMPTR(0xe0006004);
	r <<= 8;
	r |= MMPTR(0xe0006008);
	r <<= 8;
	r |= MMPTR(0xe000600c);
	r <<= 8;
	r |= MMPTR(0xe0006010);
	r <<= 8;
	r |= MMPTR(0xe0006014);
	r <<= 8;
	r |= MMPTR(0xe0006018);
	r <<= 8;
	r |= MMPTR(0xe000601c);
	return r;
}
static inline void timer1_load_write(unsigned long long int value) {
	MMPTR(0xe0006000) = value >> 56;
	MMPTR(0xe0006004) = value >> 48;
	MMPTR(0xe0006008) = value >> 40;
	MMPTR(0xe000600c) = value >> 32;
	MMPTR(0xe0006010) = value >> 24;
	MMPTR(0xe0006014) = value >> 16;
	MMPTR(0xe0006018) = value >> 8;
	MMPTR(0xe000601c) = value;
}
#define CSR_TIMER1_RELOAD_ADDR 0xe0006020
#define CSR_TIMER1_RELOAD_SIZE 8
static inline unsigned long long int timer1_reload_read(void) {
	unsigned long long int r = MMPTR(0xe0006020);
	r <<= 8;
	r |= MMPTR(0xe0006024);
	r <<= 8;
	r |= MMPTR(0xe0006028);
	r <<= 8;
	r |= MMPTR(0xe000602c);
	r <<= 8;
	r |= MMPTR(0xe0006030);
	r <<= 8;
	r |= MMPTR(0xe0006034);
	r <<= 8;
	r |= MMPTR(0xe0006038);
	r <<= 8;
	r |= MMPTR(0xe000603c);
	return r;
}
static inline void timer1_reload_write(unsigned long long int value) {
	MMPTR(0xe0006020) = value >> 56;
	MMPTR(0xe0006024) = value >> 48;
	MMPTR(0xe0006028) = value >> 40;
	MMPTR(0xe000602c) = value >> 32;
	MMPTR(0xe0006030) = value >> 24;
	MMPTR(0xe0006034) = value >> 16;
	MMPTR(0xe0006038) = value >> 8;
	MMPTR(0xe000603c) = value;
}
#define CSR_TIMER1_EN_ADDR 0xe0006040
#define CSR_TIMER1_EN_SIZE 1
static inline unsigned char timer1_en_read(void) {
	unsigned char r = MMPTR(0xe0006040);
	return r;
}
static inline void timer1_en_write(unsigned char value) {
	MMPTR(0xe0006040) = value;
}
#define CSR_TIMER1_UPDATE_VALUE_ADDR 0xe0006044
#define CSR_TIMER1_UPDATE_VALUE_SIZE 1
static inline unsigned char timer1_update_value_read(void) {
	unsigned char r = MMPTR(0xe0006044);
	return r;
}
static inline void timer1_update_value_write(unsigned char value) {
	MMPTR(0xe0006044) = value;
}
#define CSR_TIMER1_VALUE_ADDR 0xe0006048
#define CSR_TIMER1_VALUE_SIZE 8
static inline unsigned long long int timer1_value_read(void) {
	unsigned long long int r = MMPTR(0xe0006048);
	r <<= 8;
	r |= MMPTR(0xe000604c);
	r <<= 8;
	r |= MMPTR(0xe0006050);
	r <<= 8;
	r |= MMPTR(0xe0006054);
	r <<= 8;
	r |= MMPTR(0xe0006058);
	r <<= 8;
	r |= MMPTR(0xe000605c);
	r <<= 8;
	r |= MMPTR(0xe0006060);
	r <<= 8;
	r |= MMPTR(0xe0006064);
	return r;
}
#define CSR_TIMER1_EV_STATUS_ADDR 0xe0006068
#define CSR_TIMER1_EV_STATUS_SIZE 1
static inline unsigned char timer1_ev_status_read(void) {
	unsigned char r = MMPTR(0xe0006068);
	return r;
}
static inline void timer1_ev_status_write(unsigned char value) {
	MMPTR(0xe0006068) = value;
}
#define CSR_TIMER1_EV_PENDING_ADDR 0xe000606c
#define CSR_TIMER1_EV_PENDING_SIZE 1
static inline unsigned char timer1_ev_pending_read(void) {
	unsigned char r = MMPTR(0xe000606c);
	return r;
}
static inline void timer1_ev_pending_write(unsigned char value) {
	MMPTR(0xe000606c) = value;
}
#define CSR_TIMER1_EV_ENABLE_ADDR 0xe0006070
#define CSR_TIMER1_EV_ENABLE_SIZE 1
static inline unsigned char timer1_ev_enable_read(void) {
	unsigned char r = MMPTR(0xe0006070);
	return r;
}
static inline void timer1_ev_enable_write(unsigned char value) {
	MMPTR(0xe0006070) = value;
}

/* tmpu */
#define CSR_TMPU_BASE 0xe0002000
#define CSR_TMPU_ENABLE_NULL_ADDR 0xe0002000
#define CSR_TMPU_ENABLE_NULL_SIZE 1
static inline unsigned char tmpu_enable_null_read(void) {
	unsigned char r = MMPTR(0xe0002000);
	return r;
}
static inline void tmpu_enable_null_write(unsigned char value) {
	MMPTR(0xe0002000) = value;
}
#define CSR_TMPU_ENABLE_PROG_ADDR 0xe0002004
#define CSR_TMPU_ENABLE_PROG_SIZE 1
static inline unsigned char tmpu_enable_prog_read(void) {
	unsigned char r = MMPTR(0xe0002004);
	return r;
}
static inline void tmpu_enable_prog_write(unsigned char value) {
	MMPTR(0xe0002004) = value;
}
#define CSR_TMPU_PROG_ADDRESS_ADDR 0xe0002008
#define CSR_TMPU_PROG_ADDRESS_SIZE 4
static inline unsigned int tmpu_prog_address_read(void) {
	unsigned int r = MMPTR(0xe0002008);
	r <<= 8;
	r |= MMPTR(0xe000200c);
	r <<= 8;
	r |= MMPTR(0xe0002010);
	r <<= 8;
	r |= MMPTR(0xe0002014);
	return r;
}
static inline void tmpu_prog_address_write(unsigned int value) {
	MMPTR(0xe0002008) = value >> 24;
	MMPTR(0xe000200c) = value >> 16;
	MMPTR(0xe0002010) = value >> 8;
	MMPTR(0xe0002014) = value;
}

/* uart */
#define CSR_UART_BASE 0xe0000800
#define CSR_UART_RXTX_ADDR 0xe0000800
#define CSR_UART_RXTX_SIZE 1
static inline unsigned char uart_rxtx_read(void) {
	unsigned char r = MMPTR(0xe0000800);
	return r;
}
static inline void uart_rxtx_write(unsigned char value) {
	MMPTR(0xe0000800) = value;
}
#define CSR_UART_TXFULL_ADDR 0xe0000804
#define CSR_UART_TXFULL_SIZE 1
static inline unsigned char uart_txfull_read(void) {
	unsigned char r = MMPTR(0xe0000804);
	return r;
}
#define CSR_UART_RXEMPTY_ADDR 0xe0000808
#define CSR_UART_RXEMPTY_SIZE 1
static inline unsigned char uart_rxempty_read(void) {
	unsigned char r = MMPTR(0xe0000808);
	return r;
}
#define CSR_UART_EV_STATUS_ADDR 0xe000080c
#define CSR_UART_EV_STATUS_SIZE 1
static inline unsigned char uart_ev_status_read(void) {
	unsigned char r = MMPTR(0xe000080c);
	return r;
}
static inline void uart_ev_status_write(unsigned char value) {
	MMPTR(0xe000080c) = value;
}
#define CSR_UART_EV_PENDING_ADDR 0xe0000810
#define CSR_UART_EV_PENDING_SIZE 1
static inline unsigned char uart_ev_pending_read(void) {
	unsigned char r = MMPTR(0xe0000810);
	return r;
}
static inline void uart_ev_pending_write(unsigned char value) {
	MMPTR(0xe0000810) = value;
}
#define CSR_UART_EV_ENABLE_ADDR 0xe0000814
#define CSR_UART_EV_ENABLE_SIZE 1
static inline unsigned char uart_ev_enable_read(void) {
	unsigned char r = MMPTR(0xe0000814);
	return r;
}
static inline void uart_ev_enable_write(unsigned char value) {
	MMPTR(0xe0000814) = value;
}

/* uart_phy */
#define CSR_UART_PHY_BASE 0xe0000000
#define CSR_UART_PHY_TUNING_WORD_ADDR 0xe0000000
#define CSR_UART_PHY_TUNING_WORD_SIZE 4
static inline unsigned int uart_phy_tuning_word_read(void) {
	unsigned int r = MMPTR(0xe0000000);
	r <<= 8;
	r |= MMPTR(0xe0000004);
	r <<= 8;
	r |= MMPTR(0xe0000008);
	r <<= 8;
	r |= MMPTR(0xe000000c);
	return r;
}
static inline void uart_phy_tuning_word_write(unsigned int value) {
	MMPTR(0xe0000000) = value >> 24;
	MMPTR(0xe0000004) = value >> 16;
	MMPTR(0xe0000008) = value >> 8;
	MMPTR(0xe000000c) = value;
}

/* constants */
#define UART_INTERRUPT 0
static inline int uart_interrupt_read(void) {
	return 0;
}
#define TIMER0_INTERRUPT 1
static inline int timer0_interrupt_read(void) {
	return 1;
}
#define ETHMAC_INTERRUPT 2
static inline int ethmac_interrupt_read(void) {
	return 2;
}
#define TIMER1_INTERRUPT 3
static inline int timer1_interrupt_read(void) {
	return 3;
}
#define ETHMAC_CORE_PREAMBLE_CRC 1
static inline int ethmac_core_preamble_crc_read(void) {
	return 1;
}
#define ETHMAC_RX_SLOTS 4
static inline int ethmac_rx_slots_read(void) {
	return 4;
}
#define ETHMAC_TX_SLOTS 4
static inline int ethmac_tx_slots_read(void) {
	return 4;
}
#define ETHMAC_SLOT_SIZE 2048
static inline int ethmac_slot_size_read(void) {
	return 2048;
}
#define TMPU_PAGE_SIZE 4096
static inline int tmpu_page_size_read(void) {
	return 4096;
}
#define CONFIG_CLOCK_FREQUENCY 125000000
static inline int config_clock_frequency_read(void) {
	return 125000000;
}
#define CONFIG_DDRPHY_WLEVEL
#define CONFIG_HAS_DDS
#define CONFIG_HAS_RTIO_CLOCK_SWITCH
#define CONFIG_HAS_RTIO_LOG
#define CONFIG_I2C_BUS_COUNT 1
static inline int config_i2c_bus_count_read(void) {
	return 1;
}
#define CONFIG_IDENTIFIER_STR "6.7190.2a909839.beta;nist_clock"
static inline const char * config_identifier_str_read(void) {
	return "6.7190.2a909839.beta;nist_clock";
}
#define CONFIG_L2_SIZE 131072
static inline int config_l2_size_read(void) {
	return 131072;
}
#define CONFIG_RTIO_LOG_CHANNEL 28
static inline int config_rtio_log_channel_read(void) {
	return 28;
}
#define CONFIG_SOC_PLATFORM "kc705"
static inline const char * config_soc_platform_read(void) {
	return "kc705";
}
#define CONFIG_SPIFLASH_PAGE_SIZE 256
static inline int config_spiflash_page_size_read(void) {
	return 256;
}
#define CONFIG_SPIFLASH_SECTOR_SIZE 65536
static inline int config_spiflash_sector_size_read(void) {
	return 65536;
}

#endif
