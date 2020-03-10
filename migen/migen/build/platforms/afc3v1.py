# Founded by Creotech Instruments S.A.
# www.creotech.pl

from migen.build.generic_platform import *
from migen.build.xilinx import XilinxPlatform

_io = [

    # Clock inputs
    # =========================================================================

    ("clk20", 0, Pins("AE5"), IOStandard("LVCMOS15")),

    ("fpga_clk", 0,
        Subsignal("p", Pins("AK7"), Misc("IN_TERM=UNTUNED_SPLIT_50")),
        Subsignal("n", Pins("AL7")),
        Misc("DIFF_TERM=FALSE"),
        IOStandard("DIFF_SSTL15")),

    # FMC1 Clocks

    ("fmc1_clk0_m2c", 0,
        Subsignal("p", Pins("H7")),
        Subsignal("n", Pins("H6")),
        IOStandard("LVDS_25")),

    ("fmc1_clk1_m2c", 0,
        Subsignal("p", Pins("P5")),
        Subsignal("n", Pins("N4")),
        IOStandard("LVDS_25")),

    ("fmc1_clk2_bidir", 0,
        Subsignal("p", Pins("G5")),
        Subsignal("n", Pins("G4")),
        IOStandard("LVDS_25")),

    ("fmc1_clk3_bidir", 0,
        Subsignal("p", Pins("R6")),
        Subsignal("n", Pins("R5")),
        IOStandard("LVDS_25")),

    # FMC2 Clocks

    ("fmc2_clk0_m2c", 0,
        Subsignal("p", Pins("AA30")),
        Subsignal("n", Pins("AB30")),
        IOStandard("LVDS_25")),

    ("fmc2_clk1_m2c", 0,
        Subsignal("p", Pins("AG29")),
        Subsignal("n", Pins("AG30")),
        IOStandard("LVDS_25")),

    ("fmc2_clk2_bidir", 0,
        Subsignal("p", Pins("AH28")),
        Subsignal("n", Pins("AH29")),
        IOStandard("LVDS_25")),

    ("fmc2_clk3_bidir", 0,
        Subsignal("p", Pins("Y30")),
        Subsignal("n", Pins("Y31")),
        IOStandard("LVDS_25")),

    # MGT Clocks

    ("mgt113_clk0", 0,
        Subsignal("p", Pins("AG14")),
        Subsignal("n", Pins("AH14"))),

    ("mgt113_clk1", 0,
         Subsignal("p", Pins("AG16")),
         Subsignal("n", Pins("AH16"))),

    ("mgt216_clk0", 0,
        Subsignal("p", Pins("H20")),
        Subsignal("n", Pins("G20"))),

    ("mgt216_clk1", 0,
         Subsignal("p", Pins("H18")),
         Subsignal("n", Pins("G18"))),

    ("mgt213_clk0", 0,
        Subsignal("p", Pins("AG20")),
        Subsignal("n", Pins("AH20"))),

    ("mgt213_clk1", 0,
         Subsignal("p", Pins("AG18")),
         Subsignal("n", Pins("AH18"))),

    ("mgt116_clk0", 0,
        Subsignal("p", Pins("H16")),
        Subsignal("n", Pins("G16"))),

    ("mgt116_clk1", 0,
         Subsignal("p", Pins("H14")),
         Subsignal("n", Pins("G14"))),

    # GTP Transcivers
    # =========================================================================

    # MGT113

    ("mgt113", 0,
        Subsignal("txp", Pins("AN17")),
        Subsignal("txn", Pins("AP17")),
        Subsignal("rxp", Pins("AJ17")),
        Subsignal("rxn", Pins("AK17"))),

    ("mgt113", 1,
        Subsignal("txp", Pins("AN15")),
        Subsignal("txn", Pins("AP15")),
        Subsignal("rxp", Pins("AL16")),
        Subsignal("rxn", Pins("AM16"))),

    ("mgt113", 2,
        Subsignal("txp", Pins("AL14")),
        Subsignal("txn", Pins("AM14")),
        Subsignal("rxp", Pins("AJ15")),
        Subsignal("rxn", Pins("AK15"))),

    ("mgt113", 3,
        Subsignal("txp", Pins("AN13")),
        Subsignal("txn", Pins("AP13")),
        Subsignal("rxp", Pins("AJ13")),
        Subsignal("rxn", Pins("AK13"))),

    # MGT 116

    ("mgt116", 0,
         Subsignal("txp", Pins("B13")),
         Subsignal("txn", Pins("A13")),
         Subsignal("rxp", Pins("F13")),
         Subsignal("rxn", Pins("E13"))),

    ("mgt116", 1,
         Subsignal("txp", Pins("D14")),
         Subsignal("txn", Pins("C14")),
         Subsignal("rxp", Pins("F15")),
         Subsignal("rxn", Pins("E15"))),

    ("mgt116", 2,
         Subsignal("txp", Pins("B15")),
         Subsignal("txn", Pins("A15")),
         Subsignal("rxp", Pins("D16")),
         Subsignal("rxn", Pins("C16"))),

    ("mgt116", 3,
         Subsignal("txp", Pins("B17")),
         Subsignal("txn", Pins("A17")),
         Subsignal("rxp", Pins("F17")),
         Subsignal("rxn", Pins("E17"))),

    # MGT 213

    ("mgt213", 0,
         Subsignal("txp", Pins("AN19")),
         Subsignal("txn", Pins("AP19")),
         Subsignal("rxp", Pins("AL18")),
         Subsignal("rxn", Pins("AM18"))),

    ("mgt213", 1,
         Subsignal("txp", Pins("AN21")),
         Subsignal("txn", Pins("AP21")),
         Subsignal("rxp", Pins("AJ19")),
         Subsignal("rxn", Pins("AK19"))),

    ("mgt213", 2,
         Subsignal("txp", Pins("AL22")),
         Subsignal("txn", Pins("AM22")),
         Subsignal("rxp", Pins("AL20")),
         Subsignal("rxn", Pins("AM20"))),

    ("mgt213", 3,
         Subsignal("txp", Pins("AN23")),
         Subsignal("txn", Pins("AP23")),
         Subsignal("rxp", Pins("AJ21")),
         Subsignal("rxn", Pins("AK21"))),

    # MGT 216

    ("mgt216", 0,
         Subsignal("txp", Pins("B23")),
         Subsignal("txn", Pins("A23")),
         Subsignal("rxp", Pins("F21")),
         Subsignal("rxn", Pins("E21"))),

    ("mgt216", 1,
         Subsignal("txp", Pins("D22")),
         Subsignal("txn", Pins("C22")),
         Subsignal("rxp", Pins("D20")),
         Subsignal("rxn", Pins("C20"))),

    ("mgt216", 2,
         Subsignal("txp", Pins("B21")),
         Subsignal("txn", Pins("A21")),
         Subsignal("rxp", Pins("F19")),
         Subsignal("rxn", Pins("E19"))),

    ("mgt216", 3,
         Subsignal("txp", Pins("B19")),
         Subsignal("txn", Pins("A19")),
         Subsignal("rxp", Pins("D18")),
         Subsignal("rxn", Pins("C18"))),

    # RTM IO
    # =========================================================================

    ("rtm_io", 0, Pins("M26"), IOStandard("LVCMOS25")),
    ("rtm_io", 1, Pins("L23"), IOStandard("LVCMOS25")),
    ("rtm_io", 2, Pins("W6"), IOStandard("LVCMOS25")),
    ("rtm_io", 3, Pins("AA8"), IOStandard("LVCMOS25")),
    ("rtm_io", 4, Pins("U11"), IOStandard("LVCMOS25")),
    ("rtm_io", 5, Pins("R8"), IOStandard("LVCMOS25")),
    ("rtm_io", 6, Pins("L4"), IOStandard("LVCMOS25")),
    ("rtm_io", 7, Pins("AE32"), IOStandard("LVCMOS25")),
    ("rtm_io", 8, Pins("AC28"), IOStandard("LVCMOS25")),
    ("rtm_io", 9, Pins("Y26"), IOStandard("LVCMOS25")),
    ("rtm_io", 10, Pins("T32"), IOStandard("LVCMOS25")),
    ("rtm_io", 11, Pins("AF10"), IOStandard("LVCMOS15")),
    ("rtm_io", 12, Pins("AC11"), IOStandard("LVCMOS15")),
    ("rtm_io", 13, Pins("AN7"), IOStandard("LVCMOS15")),
    ("rtm_io", 14, Pins("AN6"), IOStandard("LVCMOS15")),

    ("rtm_lvds", 0,
        Subsignal("p", Pins("N9")),
        Subsignal("n", Pins("M9")),
        IOStandard("LVDS_25")),

    ("rtm_lvds", 1,
        Subsignal("p", Pins("U10")),
        Subsignal("n", Pins("T10")),
        IOStandard("LVDS_25")),

    ("rtm_lvds", 2,
        Subsignal("p", Pins("U9")),
        Subsignal("n", Pins("T9")),
        IOStandard("LVDS_25")),

    ("rtm_lvds", 3,
        Subsignal("p", Pins("L12")),
        Subsignal("n", Pins("K12")),
        IOStandard("LVDS_25")),

    ("rtm_lvds", 4,
        Subsignal("p", Pins("H11")),
        Subsignal("n", Pins("G11")),
        IOStandard("LVDS_25")),

    ("rtm_lvds", 5,
        Subsignal("p", Pins("AD31")),
        Subsignal("n", Pins("AE31")),
        IOStandard("LVDS_25")),

    ("rtm_lvds", 6,
        Subsignal("p", Pins("AD26")),
        Subsignal("n", Pins("AE26")),
        IOStandard("LVDS_25")),

    ("rtm_lvds", 7,
        Subsignal("p", Pins("AC24")),
        Subsignal("n", Pins("AD24")),
        IOStandard("LVDS_25")),

    # AMC MLVDS
    # =========================================================================

    ("amc_mlvds_io", 0, Pins("AM9"), IOStandard("LVCMOS15")),
    ("amc_mlvds_io", 1, Pins("AP11"), IOStandard("LVCMOS15")),
    ("amc_mlvds_io", 2, Pins("AP10"), IOStandard("LVCMOS15")),
    ("amc_mlvds_io", 3, Pins("AM11"), IOStandard("LVCMOS15")),
    ("amc_mlvds_io", 4, Pins("AN8"), IOStandard("LVCMOS15")),
    ("amc_mlvds_io", 5, Pins("AP8"), IOStandard("LVCMOS15")),
    ("amc_mlvds_io", 6, Pins("AL8"), IOStandard("LVCMOS15")),
    ("amc_mlvds_io", 7, Pins("AL9"), IOStandard("LVCMOS15")),

    ("amc_mlvds_dir", 0, Pins("AJ10"), IOStandard("LVCMOS15")),
    ("amc_mlvds_dir", 1, Pins("AK11"), IOStandard("LVCMOS15")),
    ("amc_mlvds_dir", 2, Pins("AJ11"), IOStandard("LVCMOS15")),
    ("amc_mlvds_dir", 3, Pins("AL10"), IOStandard("LVCMOS15")),
    ("amc_mlvds_dir", 4, Pins("AM10"), IOStandard("LVCMOS15")),
    ("amc_mlvds_dir", 5, Pins("AN11"), IOStandard("LVCMOS15")),
    ("amc_mlvds_dir", 6, Pins("AN9"), IOStandard("LVCMOS15")),
    ("amc_mlvds_dir", 7, Pins("AP9"), IOStandard("LVCMOS15")),

    # Carrier Peripherals
    # =========================================================================

    ("serial", 0,
        Subsignal("rx", Pins("AB11")),
        Subsignal("tx", Pins("Y11")),
        IOStandard("LVCMOS25")),

    ("i2c", 0,
        Subsignal("scl", Pins("P6")),
        Subsignal("sda", Pins("R11")),
        IOStandard("LVCMOS25")),

    ("spiflash", 0,
        # CLK is on CCLK
        Subsignal("cs_n", Pins("Y27")),
        Subsignal("dq", Pins("V28 V29 V26 V27")),
        IOStandard("LVCMOS25")
     ),

    ("spiflash2x", 0,
        Subsignal("cs_n", Pins("Y27")),
        Subsignal("dq", Pins("V28 V29")),
        Subsignal("wp", Pins("V26")),
        Subsignal("hold", Pins("V27")),
        # "clk" is on CCLK
        IOStandard("LVCMOS25")
    ),

    ("led", 0, Pins("K10"), IOStandard("LVCMOS25")),
    ("led", 1, Pins("L7"), IOStandard("LVCMOS25")),
    ("led", 2, Pins("H12"), IOStandard("LVCMOS25")),

    ("si570_en", 0, Pins("AD23"), IOStandard("LVCMOS25")),
    ("adn4604_nupdate", 0, Pins("U24"), IOStandard("LVCMOS25")),

    ("ddram", 0,
     
            # FIXME: MiSOC RAM controller does not seem to cooperate with multiple memory devices

            Subsignal("a", Pins("AP4 AJ6 AM4 AM6 AN4 AK3 AK5 AP6 AK6 AL4 AN2 AJ8 AP5 AM7 AK8 AP3"),
                IOStandard("SSTL15"), Misc("SLEW=FAST")),
            Subsignal("ba", Pins("AM2 AK2 AK1"), IOStandard("SSTL15"), Misc("SLEW=FAST")),
            Subsignal("ras_n", Pins("AN1"), IOStandard("SSTL15"), Misc("SLEW=FAST")),
            Subsignal("cas_n", Pins("AL2"), IOStandard("SSTL15"), Misc("SLEW=FAST")),
            Subsignal("we_n", Pins("AM1"), IOStandard("SSTL15"), Misc("SLEW=FAST")),
            Subsignal("cs_n", Pins("AN3"), IOStandard("SSTL15"), Misc("SLEW=FAST")),
            Subsignal("dm", Pins("AG4 AH1"), IOStandard("SSTL15"), Misc("SLEW=FAST")),
            Subsignal("dq", Pins(
                                 "AF4 AF5 AD3 AG5 AD5 AG6 AD4 AE3 AG1 AG2 AE1 AF3 AE2 AH3 AD1 AF2"),
            # Subsignal("dq", Pins("AD11 AE10 AF12 AG11 AE11 AH11 AG12 AH9 AD6 AG7 AF9 AH7 AE8 AF8 AE7 AF7 "
            #                      "AF4 AF5 AD3 AG5 AD5 AG6 AD4 AE3 AG1 AG2 AE1 AF3 AE2 AH3 AD1 AF2"),
                IOStandard("SSTL15"), Misc("SLEW=FAST"), Misc("IN_TERM=UNTUNED_SPLIT_50")),
            Subsignal("dqs_p", Pins("AH4 AH2"), IOStandard("DIFF_SSTL15"), Misc("SLEW=FAST")),
            Subsignal("dqs_n", Pins("AJ4 AJ1"), IOStandard("DIFF_SSTL15"), Misc("SLEW=FAST")),
            Subsignal("clk_p", Pins("AL5"), IOStandard("DIFF_SSTL15"), Misc("SLEW=FAST")),
            Subsignal("clk_n", Pins("AM5"), IOStandard("DIFF_SSTL15"), Misc("SLEW=FAST")),
            Subsignal("cke", Pins("AJ5"), IOStandard("SSTL15"), Misc("SLEW=FAST")),
            Subsignal("odt", Pins("AP1"), IOStandard("SSTL15"), Misc("SLEW=FAST")),
            Subsignal("reset_n", Pins("AJ9"), IOStandard("LVCMOS15"), Misc("SLEW=FAST")),
            
            # Subsignal("a", Pins("AP4 AJ6 AM4 AM6 AN4 AK3 AK5 AP6 AK6 AL4 AN2 AJ8 AP5 AM7 AK8 AP3"),
            #     IOStandard("SSTL15")),
            # Subsignal("ba", Pins("AM2 AK2 AK1"), IOStandard("SSTL15")),
            # Subsignal("ras_n", Pins("AN1"), IOStandard("SSTL15")),
            # Subsignal("cas_n", Pins("AL2"), IOStandard("SSTL15")),
            # Subsignal("we_n", Pins("AM1"), IOStandard("SSTL15")),
            # Subsignal("cs_n", Pins("AN3"), IOStandard("SSTL15")),
            # Subsignal("dm", Pins("AD10 AH6 AG4 AH1"), IOStandard("SSTL15")),
            # Subsignal("dq", Pins("AD11 AE10 AF12 AG11 AE11 AH11 AG12 AH9 AD6 AG7 AF9 AH7 AE8 AF8 AE7 AF7 AF4 AF5 AD3 "
            #                      "AG5 AD5 AG6 AD4 AE3 AG1 AG2 AE1 AF3 AE2 AH3 AD1 AF2"),
            #     IOStandard("SSTL15")),
            # Subsignal("dqs_p", Pins("AG10 AD9 AH4 AH2"), IOStandard("DIFF_SSTL15")),
            # Subsignal("dqs_n", Pins("AG9 AD8 AJ4 AJ1"), IOStandard("DIFF_SSTL15")),
            # Subsignal("clk_p", Pins("AL5"), IOStandard("DIFF_SSTL15")),
            # Subsignal("clk_n", Pins("AM5"), IOStandard("DIFF_SSTL15")),
            # Subsignal("cke", Pins("AJ5"), IOStandard("SSTL15")),
            # Subsignal("odt", Pins("AP1"), IOStandard("SSTL15")),
            # Subsignal("reset_n", Pins("AJ9"), IOStandard("LVCMOS15")),
        ),


]

_connectors = [
    ("fmc1", {
        "HA00_N": "H29",
        "HA00_P": "J29",
        "HA01_N": "K28",
        "HA01_P": "L28",
        "HA02_N": "J34",
        "HA02_P": "K33",
        "HA03_N": "J30",
        "HA03_P": "K30",
        "HA04_N": "L34",
        "HA04_P": "L33",
        "HA05_N": "H34",
        "HA05_P": "J33",
        "HA06_N": "K27",
        "HA06_P": "L27",
        "HA07_N": "K32",
        "HA07_P": "L32",
        "HA08_N": "L30",
        "HA08_P": "L29",
        "HA09_N": "J31",
        "HA09_P": "K31",
        "HA10_N": "G32",
        "HA10_P": "H32",
        "HA11_N": "L25",
        "HA11_P": "M25",
        "HA12_N": "G34",
        "HA12_P": "H33",
        "HA13_N": "J25",
        "HA13_P": "K25",
        "HA14_N": "L24",
        "HA14_P": "M24",
        "HA15_N": "G30",
        "HA15_P": "G29",
        "HA16_N": "G31",
        "HA16_P": "H31",
        "HA17_N": "H28",
        "HA17_P": "J28",
        "HA18_N": "G27",
        "HA18_P": "H27",
        "HA19_N": "G26",
        "HA19_P": "H26",
        "HA20_N": "J26",
        "HA20_P": "K26",
        "HA21_N": "G25",
        "HA21_P": "G24",
        "HA22_N": "H24",
        "HA22_P": "J24",
        "HA23_N": "J23",
        "HA23_P": "K23",
        "HB00_N": "Y5",
        "HB00_P": "W5",
        "HB01_N": "W8",
        "HB01_P": "W9",
        "HB02_N": "V8",
        "HB02_P": "V9",
        "HB03_N": "Y10",
        "HB03_P": "W10",
        "HB04_N": "Y7",
        "HB04_P": "Y8",
        "HB05_N": "AA9",
        "HB05_P": "AA10",
        "HB06_N": "W4",
        "HB06_P": "V4",
        "HB07_N": "AB9",
        "HB07_P": "AB10",
        "HB08_N": "W3",
        "HB08_P": "V3",
        "HB09_N": "V1",
        "HB09_P": "V2",
        "HB10_N": "V6",
        "HB10_P": "V7",
        "HB11_N": "Y1",
        "HB11_P": "W1",
        "HB12_N": "AC6",
        "HB12_P": "AC7",
        "HB13_N": "Y2",
        "HB13_P": "Y3",
        "HB14_N": "AC1",
        "HB14_P": "AC2",
        "HB15_N": "AC8",
        "HB15_P": "AC9",
        "HB16_N": "AB1",
        "HB16_P": "AB2",
        "HB17_N": "AA4",
        "HB17_P": "AA5",
        "HB18_N": "AB6",
        "HB18_P": "AB7",
        "HB19_N": "AB4",
        "HB19_P": "AB5",
        "HB20_N": "AA2",
        "HB20_P": "AA3",
        "HB21_N": "AC3",
        "HB21_P": "AC4",
        "LA00_N": "K6",
        "LA00_P": "K7",
        "LA01_N": "J5",
        "LA01_P": "J6",
        "LA02_N": "G6",
        "LA02_P": "G7",
        "LA03_N": "G1",
        "LA03_P": "H1",
        "LA04_N": "J1",
        "LA04_P": "K1",
        "LA05_N": "H3",
        "LA05_P": "H4",
        "LA06_N": "K5",
        "LA06_P": "L5",
        "LA07_N": "K2",
        "LA07_P": "K3",
        "LA08_N": "F2",
        "LA08_P": "F3",
        "LA09_N": "J3",
        "LA09_P": "J4",
        "LA10_N": "G2",
        "LA10_P": "H2",
        "LA11_N": "L2",
        "LA11_P": "M2",
        "LA12_N": "K8",
        "LA12_P": "L8",
        "LA13_N": "G9",
        "LA13_P": "G10",
        "LA14_N": "H8",
        "LA14_P": "H9",
        "LA15_N": "J11",
        "LA15_P": "K11",
        "LA16_N": "L9",
        "LA16_P": "L10",
        "LA17_N": "T4",
        "LA17_P": "T5",
        "LA18_N": "P3",
        "LA18_P": "P4",
        "LA19_N": "U4",
        "LA19_P": "U5",
        "LA20_N": "P10",
        "LA20_P": "R10",
        "LA21_N": "M6",
        "LA21_P": "M7",
        "LA22_N": "M4",
        "LA22_P": "M5",
        "LA23_N": "N2",
        "LA23_P": "N3",
        "LA24_N": "M10",
        "LA24_P": "M11",
        "LA25_N": "N7",
        "LA25_P": "N8",
        "LA26_N": "T2",
        "LA26_P": "T3",
        "LA27_N": "R2",
        "LA27_P": "R3",
        "LA28_N": "T7",
        "LA28_P": "T8",
        "LA29_N": "P8",
        "LA29_P": "P9",
        "LA30_N": "M1",
        "LA30_P": "N1",
        "LA31_N": "U6",
        "LA31_P": "U7",
        "LA32_N": "P1",
        "LA32_P": "R1",
        "LA33_N": "U1",
        "LA33_P": "U2"
    }),
    ("fmc2", {
        "HA00_N": "AM30",
        "HA00_P": "AL30",
        "HA01_N": "AL29",
        "HA01_P": "AL28",
        "HA02_N": "AP31",
        "HA02_P": "AN31",
        "HA03_N": "AN26",
        "HA03_P": "AM26",
        "HA04_N": "AK25",
        "HA04_P": "AJ25",
        "HA05_N": "AM25",
        "HA05_P": "AL25",
        "HA06_N": "AM32",
        "HA06_P": "AL32",
        "HA07_N": "AN32",
        "HA07_P": "AM31",
        "HA08_N": "AN27",
        "HA08_P": "AM27",
        "HA09_N": "AK26",
        "HA09_P": "AJ26",
        "HA10_N": "AP26",
        "HA10_P": "AP25",
        "HA11_N": "AL33",
        "HA11_P": "AK33",
        "HA12_N": "AN29",
        "HA12_P": "AM29",
        "HA13_N": "AP28",
        "HA13_P": "AN28",
        "HA14_N": "AP34",
        "HA14_P": "AN34",
        "HA15_N": "AK30",
        "HA15_P": "AJ29",
        "HA16_N": "AL27",
        "HA16_P": "AK27",
        "HA17_N": "AK28",
        "HA17_P": "AJ28",
        "HA18_N": "AP33",
        "HA18_P": "AN33",
        "HA19_N": "AK31",
        "HA19_P": "AJ30",
        "HA20_N": "AK32",
        "HA20_P": "AJ31",
        "HA21_N": "AP30",
        "HA21_P": "AP29",
        "HA22_N": "AM34",
        "HA22_P": "AL34",
        "HA23_N": "AJ34",
        "HA23_P": "AJ33",
        "HB00_N": "T29",
        "HB00_P": "U29",
        "HB01_N": "M27",
        "HB01_P": "N26",
        "HB02_N": "M34",
        "HB02_P": "N34",
        "HB03_N": "T34",
        "HB03_P": "U34",
        "HB04_N": "U27",
        "HB04_P": "U26",
        "HB05_N": "P34",
        "HB05_P": "P33",
        "HB06_N": "P30",
        "HB06_P": "R30",
        "HB07_N": "N24",
        "HB07_P": "P24",
        "HB08_N": "P26",
        "HB08_P": "R26",
        "HB09_N": "R28",
        "HB09_P": "T28",
        "HB10_N": "T25",
        "HB10_P": "U25",
        "HB11_N": "R27",
        "HB11_P": "T27",
        "HB12_N": "M32",
        "HB12_P": "N31",
        "HB13_N": "N28",
        "HB13_P": "N27",
        "HB14_N": "P31",
        "HB14_P": "R31",
        "HB15_N": "T30",
        "HB15_P": "U30",
        "HB16_N": "U32",
        "HB16_P": "U31",
        "HB17_N": "P29",
        "HB17_P": "P28",
        "HB18_N": "M29",
        "HB18_P": "N29",
        "HB19_N": "M31",
        "HB19_P": "M30",
        "HB20_N": "N33",
        "HB20_P": "N32",
        "HB21_N": "R33",
        "HB21_P": "T33",
        "LA00_N": "AF28",
        "LA00_P": "AE28",
        "LA01_N": "AF30",
        "LA01_P": "AF29",
        "LA02_N": "AH31",
        "LA02_P": "AG31",
        "LA03_N": "AH24",
        "LA03_P": "AG24",
        "LA04_N": "AC27",
        "LA04_P": "AC26",
        "LA05_N": "AH34",
        "LA05_P": "AH33",
        "LA06_N": "AF23",
        "LA06_P": "AE23",
        "LA07_N": "AH27",
        "LA07_P": "AG27",
        "LA08_N": "AE25",
        "LA08_P": "AD25",
        "LA09_N": "AG25",
        "LA09_P": "AF25",
        "LA10_N": "AH32",
        "LA10_P": "AG32",
        "LA11_N": "AE30",
        "LA11_P": "AD30",
        "LA12_N": "AF27",
        "LA12_P": "AE27",
        "LA13_N": "AG34",
        "LA13_P": "AF34",
        "LA14_N": "AF33",
        "LA14_P": "AE33",
        "LA15_N": "AD29",
        "LA15_P": "AD28",
        "LA16_N": "AD34",
        "LA16_P": "AD33",
        "LA17_N": "AB32",
        "LA17_P": "AB31",
        "LA18_N": "W31",
        "LA18_P": "W30",
        "LA19_N": "AB27",
        "LA19_P": "AB26",
        "LA20_N": "AB25",
        "LA20_P": "AB24",
        "LA21_N": "AA33",
        "LA21_P": "AA32",
        "LA22_N": "AA25",
        "LA22_P": "AA24",
        "LA23_N": "Y25",
        "LA23_P": "W25",
        "LA24_N": "Y33",
        "LA24_P": "Y32",
        "LA25_N": "AB29",
        "LA25_P": "AA29",
        "LA26_N": "AC32",
        "LA26_P": "AC31",
        "LA27_N": "AA28",
        "LA27_P": "AA27",
        "LA28_N": "W29",
        "LA28_P": "W28",
        "LA29_N": "AC34",
        "LA29_P": "AC33",
        "LA30_N": "W34",
        "LA30_P": "W33",
        "LA31_N": "V32",
        "LA31_P": "V31",
        "LA32_N": "AB34",
        "LA32_P": "AA34",
        "LA33_N": "V34",
        "LA33_P": "V33"
    })
]


class Platform(XilinxPlatform):
    default_clk_name = "clk20"
    default_clk_period = 50.0

    def __init__(self):

        XilinxPlatform.__init__(
                self, "xc7a200tffg1156-3", _io, _connectors,
                toolchain="vivado")
        self.toolchain.bitstream_commands.extend([
            "set_property BITSTREAM.CONFIG.OVERTEMPPOWERDOWN Enable [current_design]",
            "set_property BITSTREAM.GENERAL.COMPRESS TRUE [current_design]",
            "set_property CONFIG_MODE SPIx1 [current_design]",
            "set_property CFGBVS VCCO [current_design]",
            "set_property CONFIG_VOLTAGE 2.5 [current_design]",
            ])