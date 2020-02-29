set_property BITSTREAM.CONFIG.SPI_BUSWIDTH 4 [current_design]
set_property BITSTREAM.GENERAL.COMPRESS True [current_design]
write_bitstream -force top.bit 
write_cfgmem -force -format bin -interface spix4 -size 16 -loadbit "up 0x0 top.bit" -file top.bin
quit