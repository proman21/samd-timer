var N=null,E="",T="t",U="u",searchIndex={};
var R=["Type marker for `Timer` specific configuration.","Marker type for the TC count mode.","Prescaler","Default","Count16","Count8","Count32","Build and configure a TCC instance.","tcc0tcc1clock","controltimer","tc0tc1clock","count8","tc2tc3clock","count16","count32","tc4tc5clock","tcc2tcc3clock","capturemode","Select the capture trigger of channel 0.","capturetrigger","channels","Returns an empty set of flags","Returns the set containing all flags.","dtienable","Returns the raw value of the flags currently stored.","from_bits","Convert from underlying bit representation, unless that…","from_bits_truncate","Convert from underlying bit representation, dropping any…","from_bits_unchecked","Convert from underlying bit representation, preserving all…","is_empty","Returns `true` if no flags are currently stored.","Returns `true` if all flags are currently set.","intersects","Returns `true` if there are flags common to both `self`…","contains","Returns `true` all of the flags in `other` are contained…","Inserts the specified flags in-place.","Removes the specified flags in-place.","Toggles the specified flags in-place.","Inserts or removes the specified flags depending on the…","waveformselect","prescaler","timerconfig","samd_timer","WaveformSelect","Bitfield used to select waveform outputs.","Normal PWM","registerblock","interrupts","option","get_count","set_count","direction","Get the capture/compare register value for a given channel.","Interrupts","A bitfield that describes the interrupt flags that a timer…","force_buffer_update","Force the values stored in buffer registers to be written…","enable_double_buffering","disable_double_buffering","Get the current value of the counter.","get_period_buffer","set_period_buffer","Overflow","is_all","insert","remove","toggle","Enable the timer.","Disable the timer.","reconfigure","Reset and reconfigure the timer.","retrigger","is_stopped","set_oneshot","enable_interrupts","Enable or disable the timer interrupts specified by the…","interrupt_status","Get the timer instance's interrupt flags.","clear_interrupts","Clear the timer instance's interrupt flags as specified by…","set_direction","Set the direction of counting for the timer.","toggle_direction","Reverse the counting direction of the timer.","get_direction","Get the counting direction of the timer.","Set the value of the timer counter.","get_period","set_period","samd_timer::config","try_from","try_into","borrow_mut","result","type_id","borrow","typeid","samd_timer::control","master_rb","samd_timer::timer","get_cc0_buf","set_cc0_buf","get_cc1_buf","set_cc1_buf","slave_rb","formatter","Returns the set difference of the two sets of flags.","ordering","partial_cmp","sub_assign","Disables all flags enabled in the set.","Returns the complement of this set of flags.","Returns the intersection between the two sets of flags.","Returns the union of the two sets of flags.","Returns the left flags, but with all the right flags…","bitand_assign","Disables all flags disabled in the set.","bitor_assign","Adds the set of flags.","bitxor_assign","Toggles the set of flags.","from_iter","intoiterator","synchronization","outputmatrixconfig","timerwavegen","default","Direction","Synchronization","CaptureMode","CaptureTrigger","OutputMatrixConfig","TimerMode","TimerWaveGen","TimerConfig","DTIEnable","CountMode","WaveformGen","TCCInstance","ControlTimer","tcc2tc3clock"];
searchIndex["samd_timer"]={"doc":"samd-timer","i":[[3,"TC",R[45],R[0],N,N],[0,"config",E,"Configuration types for `Timer` and `ControlTimer`.",N,N],[3,"TC",R[92],R[0],N,N],[3,"Channels",E,"Bitfield for channel polarity.",N,N],[3,R[138],E,"Dead-time insertion generator bitfield.",N,N],[3,R[46],E,R[47],N,N],[3,R[137],E,"Common initialisation time configuration for `Timer` and…",N,N],[3,"TCC",E,"Type marker for `ControlTimer` specific configuration.",N,N],[3,R[5],E,R[1],N,N],[3,R[4],E,R[1],N,N],[3,R[6],E,R[1],N,N],[4,R[130],E,"Counting directions for timers.",N,N],[13,"Up",E,E,0,N],[13,"Down",E,E,0,N],[4,"Dither",E,"Dither widths for PWM dithering.",N,N],[13,"None",E,E,1,N],[13,"Dith4",E,E,1,N],[13,"Dith5",E,E,1,N],[13,"Dith6",E,E,1,N],[4,R[2],E,"Set of clock prescalers.",N,N],[13,"Div1",E,E,2,N],[13,"Div2",E,E,2,N],[13,"Div4",E,E,2,N],[13,"Div8",E,E,2,N],[13,"Div16",E,E,2,N],[13,"Div64",E,E,2,N],[13,"Div256",E,E,2,N],[13,"Div1024",E,E,2,N],[4,R[131],E,"Prescaler and counter synchronisation options.",N,N],[13,"Clock",E,E,3,N],[13,R[2],E,E,3,N],[13,"Resync",E,E,3,N],[4,"Ramp",E,"Ramp operation modes.",N,N],[13,"Ramp1",E,E,4,N],[13,"Ramp2Alternative",E,E,4,N],[13,"Ramp2",E,E,4,N],[13,"Ramp2Critical",E,E,4,N],[4,R[132],E,"Capture modes for Timer.",N,N],[13,R[3],E,E,5,N],[13,"MinCapture",E,E,5,N],[13,"MaxCapture",E,E,5,N],[4,R[133],E,"Capture triggers for Timer.",N,N],[13,"Event",E,E,6,N],[13,"Pin",E,E,6,N],[4,R[134],E,"Output matrix configuration options.",N,N],[13,R[3],E,E,7,N],[13,"Modulo",E,E,7,N],[13,"AllZero",E,E,7,N],[13,"OneZeroRestOne",E,E,7,N],[4,R[135],E,"Bit-width modes for `Timer`.",N,N],[13,R[4],E,E,8,N],[13,R[5],E,E,8,N],[13,R[6],E,E,8,N],[4,R[136],E,"Wave generation modes for `Timer`.",N,N],[13,"NFRQ",E,"Normal Frequency PWM (Default)",9,N],[13,"MFRQ",E,"Match Frequency PWM",9,N],[13,"NPWM",E,R[48],9,N],[13,"MPWM",E,"Match PWM",9,N],[11,"tcc0",E,R[7],10,[[["pm"],[R[8]],["self"],["tcc0"]],[[R[9],["tcc0"]],["tcc0"]]]],[11,"tcc1",E,R[7],10,[[["tcc1"],["pm"],[R[8]],["self"]],[[R[9],["tcc1"]],["tcc1"]]]],[11,"tcc2",E,R[7],10,[[["tcc2"],["pm"],["self"],[R[143]]],[[R[9],["tcc2"]],["tcc2"]]]],[11,"tc3",E,E,10,[[["tc3"],["pm"],["self"],[R[143]]],[[R[11]],["timer",["tc3",R[11]]],["tc3"]]]],[11,"tc4",E,E,10,[[[R[15]],["pm"],["tc4"],["self"]],[["tc4"],["timer",["tc4",R[11]]],[R[11]]]]],[11,"tc5",E,E,10,[[[R[15]],["pm"],["self"],["tc5"]],[["timer",["tc5",R[11]]],["tc5"],[R[11]]]]],[11,"tc3",E,E,10,[[["tc3"],["pm"],["self"],[R[143]]],[[R[13]],["timer",["tc3",R[13]]],["tc3"]]]],[11,"tc4",E,E,10,[[[R[15]],["pm"],["tc4"],["self"]],[["timer",["tc4",R[13]]],["tc4"],[R[13]]]]],[11,"tc5",E,E,10,[[[R[15]],["pm"],["self"],["tc5"]],[[R[13]],["tc5"],["timer",["tc5",R[13]]]]]],[11,"tc4_5",E,E,10,[[[R[15]],["tc4_5"],["pm"],["self"]],[["timer",["tc4_5",R[14]]],[R[14]],["tc4_5"]]]],[11,"wave_gen",E,E,10,[[[R[128]],["self"]],["self"]]],[11,"reverse",E,E,0,[[],[R[54]]]],[18,"CHAN_0",E,E,11,N],[18,"CHAN_1",E,E,11,N],[18,"CHAN_2",E,E,11,N],[18,"CHAN_3",E,E,11,N],[18,"CHAN_4",E,E,11,N],[18,"CHAN_5",E,E,11,N],[11,"empty",E,R[21],11,[[],[R[20]]]],[11,"all",E,R[22],11,[[],[R[20]]]],[11,"bits",E,R[24],11,[[["self"]],["u8"]]],[11,R[25],E,R[26],11,[[["u8"]],[[R[51],[R[20]]],[R[20]]]]],[11,R[27],E,R[28],11,[[["u8"]],[R[20]]]],[11,R[29],E,R[30],11,[[["u8"]],[R[20]]]],[11,R[31],E,R[32],11,[[["self"]],["bool"]]],[11,R[66],E,R[33],11,[[["self"]],["bool"]]],[11,R[34],E,R[35],11,[[["self"],[R[20]]],["bool"]]],[11,R[36],E,R[37],11,[[["self"],[R[20]]],["bool"]]],[11,R[67],E,R[38],11,[[["self"],[R[20]]]]],[11,R[68],E,R[39],11,[[["self"],[R[20]]]]],[11,R[69],E,R[40],11,[[["self"],[R[20]]]]],[11,"set",E,R[41],11,[[["self"],[R[20]],["bool"]]]],[18,"GEN_0",E,E,12,N],[18,"GEN_1",E,E,12,N],[18,"GEN_2",E,E,12,N],[18,"GEN_3",E,E,12,N],[11,"empty",E,R[21],12,[[],[R[23]]]],[11,"all",E,R[22],12,[[],[R[23]]]],[11,"bits",E,R[24],12,[[["self"]],["u8"]]],[11,R[25],E,R[26],12,[[["u8"]],[[R[51],[R[23]]],[R[23]]]]],[11,R[27],E,R[28],12,[[["u8"]],[R[23]]]],[11,R[29],E,R[30],12,[[["u8"]],[R[23]]]],[11,R[31],E,R[32],12,[[["self"]],["bool"]]],[11,R[66],E,R[33],12,[[["self"]],["bool"]]],[11,R[34],E,R[35],12,[[["self"],[R[23]]],["bool"]]],[11,R[36],E,R[37],12,[[["self"],[R[23]]],["bool"]]],[11,R[67],E,R[38],12,[[["self"],[R[23]]]]],[11,R[68],E,R[39],12,[[["self"],[R[23]]]]],[11,R[69],E,R[40],12,[[["self"],[R[23]]]]],[11,"set",E,R[41],12,[[["self"],[R[23]],["bool"]]]],[18,"WO_0",E,E,13,N],[18,"WO_1",E,E,13,N],[18,"WO_2",E,E,13,N],[18,"WO_3",E,E,13,N],[18,"WO_4",E,E,13,N],[18,"WO_5",E,E,13,N],[18,"WO_6",E,E,13,N],[18,"WO_7",E,E,13,N],[11,"empty",E,R[21],13,[[],[R[42]]]],[11,"all",E,R[22],13,[[],[R[42]]]],[11,"bits",E,R[24],13,[[["self"]],["u8"]]],[11,R[25],E,R[26],13,[[["u8"]],[[R[42]],[R[51],[R[42]]]]]],[11,R[27],E,R[28],13,[[["u8"]],[R[42]]]],[11,R[29],E,R[30],13,[[["u8"]],[R[42]]]],[11,R[31],E,R[32],13,[[["self"]],["bool"]]],[11,R[66],E,R[33],13,[[["self"]],["bool"]]],[11,R[34],E,R[35],13,[[["self"],[R[42]]],["bool"]]],[11,R[36],E,R[37],13,[[["self"],[R[42]]],["bool"]]],[11,R[67],E,R[38],13,[[["self"],[R[42]]]]],[11,R[68],E,R[39],13,[[["self"],[R[42]]]]],[11,R[69],E,R[40],13,[[["self"],[R[42]]]]],[11,"set",E,R[41],13,[[["bool"],["self"],[R[42]]]]],[8,R[139],E,"Trait for the different counter widths of the `Timer`…",N,N],[18,"MODE",E,E,14,N],[16,"Size",E,E,14,N],[10,R[52],E,E,14,[[["tc_rb"]]]],[10,R[53],E,E,14,[[["tc_rb"]]]],[10,"get_cc0",E,E,14,[[["tc_rb"]]]],[10,"set_cc0",E,E,14,[[["tc_rb"]]]],[10,"get_cc1",E,E,14,[[["tc_rb"]]]],[10,"set_cc1",E,E,14,[[["tc_rb"]]]],[11,"dma_oneshot",E,"Enable or disable the DMA oneshot trigger.",10,[[["self"],["bool"]],["self"]]],[11,"auto_lock",E,"If set, double buffering will be disabled on overflow,…",10,[[["self"],["bool"]],["self"]]],[11,"sync",E,"Chose which clock source is used to reset or reload the…",10,[[["self"],[R[126]]],["self"]]],[11,"run_standby",E,"If set, the timer will continue operation in standby mode.",10,[[["self"],["bool"]],["self"]]],[11,R[43],E,"Select which factor is used for the timer prescaler.",10,[[["self"],[R[43]]],["self"]]],[11,"halt_on_debug",E,"Halt the timer when the device enters debug mode.",10,[[["self"]],["self"]]],[11,"continue_on_debug",E,"The timer will continue in debug mode.",10,[[["self"]],["self"]]],[11,"control_timer",E,"Create a configuration for a `ControlTimer`.",10,[[],[["tcc"],[R[44],["tcc"]]]]],[11,"dither",E,"Select how many PWM frames are between dithered values.",10,[[["self"],["dither"]],["self"]]],[11,"no_fault_on_debug",E,"Do not generate faults when device is in debug mode.",10,[[["self"]],["self"]]],[11,"fault_on_debug",E,"Generate a non-recoverable fault when device is in debug…",10,[[["self"]],["self"]]],[11,"dti_output_cycles",E,"Set the number of cycles for the dead-time generator's…",10,[[["self"]],["self"]]],[11,"dti_enable",E,"Enable the dead-time insertion generator for the given…",10,[[["self"],[R[23]]],["self"]]],[11,"output_config",E,"Set the matrix routing mode of the waveform generation…",10,[[["self"],[R[127]]],["self"]]],[11,"output_inversion",E,"Select which waveform outputs are inverted.",10,[[["self"],[R[42]]],["self"]]],[11,R[11],E,"Create a configuration for `Timer` with an 8-bit counter.",10,[[],[["tc",[R[11]]],[R[44],["tc"]]]]],[11,R[13],E,"Create a configuration for `Timer` with a 16-bit counter.",10,[[],[[R[44],["tc"]],["tc",[R[13]]]]]],[11,R[14],E,"Create a configuration for `Timer` with a 32-bit counter.",10,[[],[["tc",[R[14]]],[R[44],["tc"]]]]],[0,"control",R[45],"Timer Counter for Control applications.",N,N],[3,R[46],R[100],R[47],N,N],[3,R[56],E,R[57],N,N],[3,R[142],E,"`ControlTimer` is used to work with a TCC peripheral…",N,N],[4,R[140],E,"Possible waveform generation modes for a TCC instance.",N,N],[13,"NFRQ",E,"Normal Frequency (Default)",15,N],[13,"MFRQ",E,"Match Frequency",15,N],[13,"NPWM",E,R[48],15,N],[13,"DSCritical",E,"Dual-Slope Critical PWM",15,N],[13,"DSBottom",E,"Dual-Slope Bottom PWM",15,N],[13,"DSBoth",E,"Dual-Slope Both PWM",15,N],[13,"DSTop",E,"Dual-Slope Top PWM",15,N],[18,"MC3",E,"Match or Capture Channel 3",16,N],[18,"MC2",E,"Match or Capture Channel 2",16,N],[18,"MC1",E,"Match or Capture Channel 1",16,N],[18,"MC0",E,"Match or Capture Channel 0",16,N],[18,"FAULT1",E,"Non-Recoverable Fault 1",16,N],[18,"FAULT0",E,"Non-Recoverable Fault 0",16,N],[18,"FAULTB",E,"Recoverable Fault B",16,N],[18,"FAULTA",E,"Recoverable Fault A",16,N],[18,"DFS",E,"Non-Recoverable Debug Fault State",16,N],[18,"UFS",E,"Non-Recoverable Update Fault",16,N],[18,"ERR",E,"Error",16,N],[18,"CNT",E,"Counter",16,N],[18,"TRG",E,"Retrigger",16,N],[18,"OVF",E,R[65],16,N],[11,"empty",E,R[21],16,[[],[R[50]]]],[11,"all",E,R[22],16,[[],[R[50]]]],[11,"bits",E,R[24],16,[[["self"]],["u32"]]],[11,R[25],E,R[26],16,[[["u32"]],[[R[50]],[R[51],[R[50]]]]]],[11,R[27],E,R[28],16,[[["u32"]],[R[50]]]],[11,R[29],E,R[30],16,[[["u32"]],[R[50]]]],[11,R[31],E,R[32],16,[[["self"]],["bool"]]],[11,R[66],E,R[33],16,[[["self"]],["bool"]]],[11,R[34],E,R[35],16,[[[R[50]],["self"]],["bool"]]],[11,R[36],E,R[37],16,[[[R[50]],["self"]],["bool"]]],[11,R[67],E,R[38],16,[[[R[50]],["self"]]]],[11,R[68],E,R[39],16,[[[R[50]],["self"]]]],[11,R[69],E,R[40],16,[[[R[50]],["self"]]]],[11,"set",E,R[41],16,[[[R[50]],["self"],["bool"]]]],[11,"eject",E,"Reset and eject the TCC instance.",17,[[],[T]]],[11,"enable",E,R[70],17,[[["self"]]]],[11,"disable",E,R[71],17,[[["self"]]]],[11,"reset",E,"Disable the timer and issue a software reset. This clears…",17,[[["self"]]]],[11,R[72],E,R[73],17,[[["self"],[R[44]]]]],[11,"stop",E,"Stop the timer counting.",17,[[["self"]]]],[11,R[74],E,"Restart the timer count.",17,[[["self"]]]],[11,R[75],E,"Returns whether the timer is stopped or not.",17,[[["self"]],["bool"]]],[11,"enabled_interrupts",E,"Get the enabled interrupts for the timer instance.",17,[[["self"]],[R[50]]]],[11,R[77],E,R[78],17,[[[R[50]],["self"]]]],[11,R[79],E,R[80],17,[[["self"]],[R[50]]]],[11,R[81],E,R[82],17,[[[R[50]],["self"]]]],[11,R[60],E,"Enable double buffering for period, pattern, and compare…",17,[[["self"]]]],[11,R[61],E,"Disable double buffering for period, pattern, and compare…",17,[[["self"]]]],[11,R[58],E,R[59],17,[[["self"]]]],[11,R[76],E,"Set whether or not the timer is a oneshot timer.",17,[[["self"],["bool"]]]],[11,R[52],E,R[62],17,[[["self"]],["u32"]]],[11,R[53],E,R[89],17,[[["self"],["u32"]]]],[11,"set_wave_gen",E,"Set the wave generation mode of the timer.",17,[[["self"],["waveformgen"]]]],[11,"set_channel_polarity",E,"Set the channel polarity using the given channel bitfield.",17,[[["self"],[R[20]]]]],[11,R[83],E,R[84],17,[[[R[54]],["self"]]]],[11,R[85],E,R[86],17,[[["self"]]]],[11,R[87],E,R[88],17,[[["self"]],[R[54]]]],[11,"set_cc",E,"Set the capture/compare register value for a given channel.",17,[[["self"],["usize"],["u32"]]]],[11,"get_cc",E,R[55],17,[[["self"],["usize"]],["u32"]]],[11,"set_cc_dither",E,"Set the capture/compare register dither for a given channel.",17,[[["self"],["usize"],["u8"]]]],[11,"get_cc_dither",E,R[55],17,[[["self"],["usize"]],["u8"]]],[11,"set_cc_buffer",E,"Set the capture/compare buffer register value for a given…",17,[[["self"],["usize"],["u32"]]]],[11,"get_cc_buffer",E,"Get the capture/compare register buffer value for a given…",17,[[["self"],["usize"]],["u32"]]],[11,"set_cc_buffer_dither",E,"Set the capture/compare buffer register dither for a given…",17,[[["self"],["usize"],["u8"]]]],[11,"get_cc_buffer_dither",E,"Get the capture/compare register buffer dither for a given…",17,[[["self"],["usize"]],["u8"]]],[11,R[91],E,"Set the period value of the timer.",17,[[["self"],["u32"]]]],[11,R[90],E,"Get the period value of the timer.",17,[[["self"]],["u32"]]],[11,R[64],E,"Set the period buffer value of the timer.",17,[[["self"],["u32"]]]],[11,R[63],E,"Get the period buffer value of the timer.",17,[[["self"]],["u32"]]],[11,"set_period_buffer_dither",E,"Set the period buffer dither value of the timer.",17,[[["self"],["u8"]]]],[11,"get_period_buffer_dither",E,"Get the period buffer dither value of the timer.",17,[[["self"]],["u8"]]],[11,"set_pattern_value",E,"Set the pattern value of each waveform output as specified…",17,[[["self"],[R[42]]]]],[11,"get_pattern_value",E,"Get the pattern value of each waveform output as a bitfield.",17,[[["self"]],[R[42]]]],[11,"set_pattern_enable",E,"Set the enable pattern for each waveform output using the…",17,[[["self"],[R[42]]]]],[11,"get_pattern_enable",E,"Get the enable pattern for each waveform output as a…",17,[[["self"]],[R[42]]]],[11,"set_pattern_value_buffer",E,"Set the pattern value buffer of each waveform output as…",17,[[["self"],[R[42]]]]],[11,"get_pattern_value_buffer",E,"Get the pattern value buffer of each waveform output as a…",17,[[["self"]],[R[42]]]],[11,"set_pattern_enable_buffer",E,"Set the pattern enable buffer of each waveform output as…",17,[[["self"],[R[42]]]]],[11,"get_pattern_enable_buffer",E,"Get the pattern enable buffer of each waveform output as a…",17,[[["self"]],[R[42]]]],[0,"timer",R[45],"Timer Counter.",N,N],[3,"TC4_5",R[102],E,N,N],[12,"0",E,E,18,N],[12,"1",E,E,18,N],[3,R[56],E,R[57],N,N],[3,"Timer",E,"Timer is used to work with an instance of a TC peripheral.",N,N],[11,R[52],E,R[62],19,[[["self"]]]],[11,"get_cc0",E,"Get the compare/capture value of channel 0.",19,[[["self"]]]],[11,"get_cc1",E,"Get the compare/capture value of channel 1.",19,[[["self"]]]],[18,"MC1",E,"Match or Compare Channel 1",20,N],[18,"MC0",E,"Match or Compare Channel 0",20,N],[18,"ERR",E,"Error",20,N],[18,"OVF",E,R[65],20,N],[11,"empty",E,R[21],20,[[],[R[50]]]],[11,"all",E,R[22],20,[[],[R[50]]]],[11,"bits",E,R[24],20,[[["self"]],["u8"]]],[11,R[25],E,R[26],20,[[["u8"]],[[R[50]],[R[51],[R[50]]]]]],[11,R[27],E,R[28],20,[[["u8"]],[R[50]]]],[11,R[29],E,R[30],20,[[["u8"]],[R[50]]]],[11,R[31],E,R[32],20,[[["self"]],["bool"]]],[11,R[66],E,R[33],20,[[["self"]],["bool"]]],[11,R[34],E,R[35],20,[[[R[50]],["self"]],["bool"]]],[11,R[36],E,R[37],20,[[[R[50]],["self"]],["bool"]]],[11,R[67],E,R[38],20,[[[R[50]],["self"]]]],[11,R[68],E,R[39],20,[[[R[50]],["self"]]]],[11,R[69],E,R[40],20,[[[R[50]],["self"]]]],[11,"set",E,R[41],20,[[[R[50]],["self"],["bool"]]]],[11,"eject",E,"Reset and eject the TC instance.",19,[[],[T]]],[11,"enable",E,R[70],19,[[["self"]]]],[11,"disable",E,R[71],19,[[["self"]]]],[11,"reset",E,"Disable and reset the timer.",19,[[["self"]]]],[11,R[72],E,R[73],19,[[["self"],[R[44]]]]],[11,R[74],E,"Retrigger the timer.",19,[[["self"]]]],[11,"stop",E,"Stop the timer.",19,[[["self"]]]],[11,R[75],E,"Return true if the timer is stopped.",19,[[["self"]],["bool"]]],[11,"get_oneshot",E,"Return true if the timer is in oneshot mode.",19,[[["self"]],["bool"]]],[11,R[76],E,"Turn onehsot mode on and off.",19,[[["self"],["bool"]]]],[11,R[77],E,R[78],19,[[[R[50]],["self"]]]],[11,R[79],E,R[80],19,[[["self"]],[R[50]]]],[11,R[81],E,R[82],19,[[[R[50]],["self"]]]],[11,R[83],E,R[84],19,[[[R[54]],["self"]]]],[11,R[85],E,R[86],19,[[["self"]]]],[11,R[87],E,R[88],19,[[["self"]],[R[54]]]],[11,R[53],E,R[89],19,[[["self"]]]],[11,"set_cc0",E,"Set the compare/capture value of channel 0.",19,[[["self"]]]],[11,"set_cc1",E,"Set the compare/capture value of channel 1.",19,[[["self"]]]],[11,R[90],E,"Get period value",19,[[["self"]],["u8"]]],[11,R[91],E,"Set period value",19,[[["self"],["u8"]]]],[11,"from",R[45],E,21,[[[T]],[T]]],[11,R[93],E,E,21,[[[U]],[R[96]]]],[11,"into",E,E,21,[[],[U]]],[11,R[94],E,E,21,[[],[R[96]]]],[11,R[98],E,E,21,[[["self"]],[T]]],[11,R[95],E,E,21,[[["self"]],[T]]],[11,R[97],E,E,21,[[["self"]],[R[99]]]],[11,"from",R[92],E,11,[[[T]],[T]]],[11,R[93],E,E,11,[[[U]],[R[96]]]],[11,"into",E,E,11,[[],[U]]],[11,R[94],E,E,11,[[],[R[96]]]],[11,R[98],E,E,11,[[["self"]],[T]]],[11,R[95],E,E,11,[[["self"]],[T]]],[11,R[97],E,E,11,[[["self"]],[R[99]]]],[11,"from",E,E,12,[[[T]],[T]]],[11,R[93],E,E,12,[[[U]],[R[96]]]],[11,"into",E,E,12,[[],[U]]],[11,R[94],E,E,12,[[],[R[96]]]],[11,R[98],E,E,12,[[["self"]],[T]]],[11,R[95],E,E,12,[[["self"]],[T]]],[11,R[97],E,E,12,[[["self"]],[R[99]]]],[11,"from",E,E,13,[[[T]],[T]]],[11,R[93],E,E,13,[[[U]],[R[96]]]],[11,"into",E,E,13,[[],[U]]],[11,R[94],E,E,13,[[],[R[96]]]],[11,R[98],E,E,13,[[["self"]],[T]]],[11,R[95],E,E,13,[[["self"]],[T]]],[11,R[97],E,E,13,[[["self"]],[R[99]]]],[11,"from",E,E,10,[[[T]],[T]]],[11,R[93],E,E,10,[[[U]],[R[96]]]],[11,"into",E,E,10,[[],[U]]],[11,R[94],E,E,10,[[],[R[96]]]],[11,R[98],E,E,10,[[["self"]],[T]]],[11,R[95],E,E,10,[[["self"]],[T]]],[11,R[97],E,E,10,[[["self"]],[R[99]]]],[11,"from",E,E,22,[[[T]],[T]]],[11,R[93],E,E,22,[[[U]],[R[96]]]],[11,"into",E,E,22,[[],[U]]],[11,R[94],E,E,22,[[],[R[96]]]],[11,R[98],E,E,22,[[["self"]],[T]]],[11,R[95],E,E,22,[[["self"]],[T]]],[11,R[97],E,E,22,[[["self"]],[R[99]]]],[11,"from",E,E,23,[[[T]],[T]]],[11,R[93],E,E,23,[[[U]],[R[96]]]],[11,"into",E,E,23,[[],[U]]],[11,R[94],E,E,23,[[],[R[96]]]],[11,R[98],E,E,23,[[["self"]],[T]]],[11,R[95],E,E,23,[[["self"]],[T]]],[11,R[97],E,E,23,[[["self"]],[R[99]]]],[11,"from",E,E,24,[[[T]],[T]]],[11,R[93],E,E,24,[[[U]],[R[96]]]],[11,"into",E,E,24,[[],[U]]],[11,R[94],E,E,24,[[],[R[96]]]],[11,R[98],E,E,24,[[["self"]],[T]]],[11,R[95],E,E,24,[[["self"]],[T]]],[11,R[97],E,E,24,[[["self"]],[R[99]]]],[11,"from",E,E,25,[[[T]],[T]]],[11,R[93],E,E,25,[[[U]],[R[96]]]],[11,"into",E,E,25,[[],[U]]],[11,R[94],E,E,25,[[],[R[96]]]],[11,R[98],E,E,25,[[["self"]],[T]]],[11,R[95],E,E,25,[[["self"]],[T]]],[11,R[97],E,E,25,[[["self"]],[R[99]]]],[11,"from",E,E,0,[[[T]],[T]]],[11,R[93],E,E,0,[[[U]],[R[96]]]],[11,"into",E,E,0,[[],[U]]],[11,R[94],E,E,0,[[],[R[96]]]],[11,R[98],E,E,0,[[["self"]],[T]]],[11,R[95],E,E,0,[[["self"]],[T]]],[11,R[97],E,E,0,[[["self"]],[R[99]]]],[11,"from",E,E,1,[[[T]],[T]]],[11,R[93],E,E,1,[[[U]],[R[96]]]],[11,"into",E,E,1,[[],[U]]],[11,R[94],E,E,1,[[],[R[96]]]],[11,R[98],E,E,1,[[["self"]],[T]]],[11,R[95],E,E,1,[[["self"]],[T]]],[11,R[97],E,E,1,[[["self"]],[R[99]]]],[11,"from",E,E,2,[[[T]],[T]]],[11,R[93],E,E,2,[[[U]],[R[96]]]],[11,"into",E,E,2,[[],[U]]],[11,R[94],E,E,2,[[],[R[96]]]],[11,R[98],E,E,2,[[["self"]],[T]]],[11,R[95],E,E,2,[[["self"]],[T]]],[11,R[97],E,E,2,[[["self"]],[R[99]]]],[11,"from",E,E,3,[[[T]],[T]]],[11,R[93],E,E,3,[[[U]],[R[96]]]],[11,"into",E,E,3,[[],[U]]],[11,R[94],E,E,3,[[],[R[96]]]],[11,R[98],E,E,3,[[["self"]],[T]]],[11,R[95],E,E,3,[[["self"]],[T]]],[11,R[97],E,E,3,[[["self"]],[R[99]]]],[11,"from",E,E,4,[[[T]],[T]]],[11,R[93],E,E,4,[[[U]],[R[96]]]],[11,"into",E,E,4,[[],[U]]],[11,R[94],E,E,4,[[],[R[96]]]],[11,R[98],E,E,4,[[["self"]],[T]]],[11,R[95],E,E,4,[[["self"]],[T]]],[11,R[97],E,E,4,[[["self"]],[R[99]]]],[11,"from",E,E,5,[[[T]],[T]]],[11,R[93],E,E,5,[[[U]],[R[96]]]],[11,"into",E,E,5,[[],[U]]],[11,R[94],E,E,5,[[],[R[96]]]],[11,R[98],E,E,5,[[["self"]],[T]]],[11,R[95],E,E,5,[[["self"]],[T]]],[11,R[97],E,E,5,[[["self"]],[R[99]]]],[11,"from",E,E,6,[[[T]],[T]]],[11,R[93],E,E,6,[[[U]],[R[96]]]],[11,"into",E,E,6,[[],[U]]],[11,R[94],E,E,6,[[],[R[96]]]],[11,R[98],E,E,6,[[["self"]],[T]]],[11,R[95],E,E,6,[[["self"]],[T]]],[11,R[97],E,E,6,[[["self"]],[R[99]]]],[11,"from",E,E,7,[[[T]],[T]]],[11,R[93],E,E,7,[[[U]],[R[96]]]],[11,"into",E,E,7,[[],[U]]],[11,R[94],E,E,7,[[],[R[96]]]],[11,R[98],E,E,7,[[["self"]],[T]]],[11,R[95],E,E,7,[[["self"]],[T]]],[11,R[97],E,E,7,[[["self"]],[R[99]]]],[11,"from",E,E,8,[[[T]],[T]]],[11,R[93],E,E,8,[[[U]],[R[96]]]],[11,"into",E,E,8,[[],[U]]],[11,R[94],E,E,8,[[],[R[96]]]],[11,R[98],E,E,8,[[["self"]],[T]]],[11,R[95],E,E,8,[[["self"]],[T]]],[11,R[97],E,E,8,[[["self"]],[R[99]]]],[11,"from",E,E,9,[[[T]],[T]]],[11,R[93],E,E,9,[[[U]],[R[96]]]],[11,"into",E,E,9,[[],[U]]],[11,R[94],E,E,9,[[],[R[96]]]],[11,R[98],E,E,9,[[["self"]],[T]]],[11,R[95],E,E,9,[[["self"]],[T]]],[11,R[97],E,E,9,[[["self"]],[R[99]]]],[11,"from",R[100],E,16,[[[T]],[T]]],[11,R[93],E,E,16,[[[U]],[R[96]]]],[11,"into",E,E,16,[[],[U]]],[11,R[94],E,E,16,[[],[R[96]]]],[11,R[98],E,E,16,[[["self"]],[T]]],[11,R[95],E,E,16,[[["self"]],[T]]],[11,R[97],E,E,16,[[["self"]],[R[99]]]],[11,"from",E,E,17,[[[T]],[T]]],[11,R[93],E,E,17,[[[U]],[R[96]]]],[11,"into",E,E,17,[[],[U]]],[11,R[94],E,E,17,[[],[R[96]]]],[11,R[98],E,E,17,[[["self"]],[T]]],[11,R[95],E,E,17,[[["self"]],[T]]],[11,R[97],E,E,17,[[["self"]],[R[99]]]],[11,"from",E,E,15,[[[T]],[T]]],[11,R[93],E,E,15,[[[U]],[R[96]]]],[11,"into",E,E,15,[[],[U]]],[11,R[94],E,E,15,[[],[R[96]]]],[11,R[98],E,E,15,[[["self"]],[T]]],[11,R[95],E,E,15,[[["self"]],[T]]],[11,R[97],E,E,15,[[["self"]],[R[99]]]],[11,"from",R[102],E,18,[[[T]],[T]]],[11,R[93],E,E,18,[[[U]],[R[96]]]],[11,"into",E,E,18,[[],[U]]],[11,R[94],E,E,18,[[],[R[96]]]],[11,R[98],E,E,18,[[["self"]],[T]]],[11,R[95],E,E,18,[[["self"]],[T]]],[11,R[97],E,E,18,[[["self"]],[R[99]]]],[11,"from",E,E,20,[[[T]],[T]]],[11,R[93],E,E,20,[[[U]],[R[96]]]],[11,"into",E,E,20,[[],[U]]],[11,R[94],E,E,20,[[],[R[96]]]],[11,R[98],E,E,20,[[["self"]],[T]]],[11,R[95],E,E,20,[[["self"]],[T]]],[11,R[97],E,E,20,[[["self"]],[R[99]]]],[11,"from",E,E,19,[[[T]],[T]]],[11,R[93],E,E,19,[[[U]],[R[96]]]],[11,"into",E,E,19,[[],[U]]],[11,R[94],E,E,19,[[],[R[96]]]],[11,R[98],E,E,19,[[["self"]],[T]]],[11,R[95],E,E,19,[[["self"]],[T]]],[11,R[97],E,E,19,[[["self"]],[R[99]]]],[11,R[52],R[92],E,23,[[["tc_rb"]]]],[11,R[53],E,E,23,[[["tc_rb"]]]],[11,"get_cc0",E,E,23,[[["tc_rb"]]]],[11,"set_cc0",E,E,23,[[["tc_rb"]]]],[11,"get_cc1",E,E,23,[[["tc_rb"]]]],[11,"set_cc1",E,E,23,[[["tc_rb"]]]],[11,R[52],E,E,24,[[["tc_rb"]]]],[11,R[53],E,E,24,[[["tc_rb"]]]],[11,"get_cc0",E,E,24,[[["tc_rb"]]]],[11,"set_cc0",E,E,24,[[["tc_rb"]]]],[11,"get_cc1",E,E,24,[[["tc_rb"]]]],[11,"set_cc1",E,E,24,[[["tc_rb"]]]],[11,R[52],E,E,25,[[["tc_rb"]]]],[11,R[53],E,E,25,[[["tc_rb"]]]],[11,"get_cc0",E,E,25,[[["tc_rb"]]]],[11,"set_cc0",E,E,25,[[["tc_rb"]]]],[11,"get_cc1",E,E,25,[[["tc_rb"]]]],[11,"set_cc1",E,E,25,[[["tc_rb"]]]],[11,"fmt",E,E,11,[[[R[108]],["self"]],[R[96]]]],[11,"fmt",E,E,12,[[[R[108]],["self"]],[R[96]]]],[11,"fmt",E,E,13,[[[R[108]],["self"]],[R[96]]]],[11,"fmt",R[100],E,16,[[[R[108]],["self"]],[R[96]]]],[11,"fmt",E,E,15,[[["self"],[R[108]]],[R[96]]]],[11,"fmt",R[102],E,20,[[[R[108]],["self"]],[R[96]]]],[11,"sub",R[92],R[109],11,[[[R[20]]],[R[20]]]],[11,"sub",E,R[109],12,[[[R[23]]],[R[23]]]],[11,"sub",E,R[109],13,[[[R[42]]],[R[42]]]],[11,"sub",R[100],R[109],16,[[[R[50]]],[R[50]]]],[11,"sub",R[102],R[109],20,[[[R[50]]],[R[50]]]],[11,"eq",R[92],E,11,[[["self"],[R[20]]],["bool"]]],[11,"ne",E,E,11,[[["self"],[R[20]]],["bool"]]],[11,"eq",E,E,12,[[[R[23]],["self"]],["bool"]]],[11,"ne",E,E,12,[[[R[23]],["self"]],["bool"]]],[11,"eq",E,E,13,[[["self"],[R[42]]],["bool"]]],[11,"ne",E,E,13,[[["self"],[R[42]]],["bool"]]],[11,"eq",R[100],E,16,[[["self"],[R[50]]],["bool"]]],[11,"ne",E,E,16,[[["self"],[R[50]]],["bool"]]],[11,"eq",R[102],E,20,[[["self"],[R[50]]],["bool"]]],[11,"ne",E,E,20,[[["self"],[R[50]]],["bool"]]],[11,"cmp",R[92],E,11,[[["self"],[R[20]]],[R[110]]]],[11,"cmp",E,E,12,[[[R[23]],["self"]],[R[110]]]],[11,"cmp",E,E,13,[[["self"],[R[42]]],[R[110]]]],[11,"cmp",R[100],E,16,[[["self"],[R[50]]],[R[110]]]],[11,"cmp",R[102],E,20,[[["self"],[R[50]]],[R[110]]]],[11,R[111],R[92],E,11,[[["self"],[R[20]]],[[R[51],[R[110]]],[R[110]]]]],[11,"lt",E,E,11,[[["self"],[R[20]]],["bool"]]],[11,"le",E,E,11,[[["self"],[R[20]]],["bool"]]],[11,"gt",E,E,11,[[["self"],[R[20]]],["bool"]]],[11,"ge",E,E,11,[[["self"],[R[20]]],["bool"]]],[11,R[111],E,E,12,[[[R[23]],["self"]],[[R[51],[R[110]]],[R[110]]]]],[11,"lt",E,E,12,[[[R[23]],["self"]],["bool"]]],[11,"le",E,E,12,[[[R[23]],["self"]],["bool"]]],[11,"gt",E,E,12,[[[R[23]],["self"]],["bool"]]],[11,"ge",E,E,12,[[[R[23]],["self"]],["bool"]]],[11,R[111],E,E,13,[[["self"],[R[42]]],[[R[51],[R[110]]],[R[110]]]]],[11,"lt",E,E,13,[[["self"],[R[42]]],["bool"]]],[11,"le",E,E,13,[[["self"],[R[42]]],["bool"]]],[11,"gt",E,E,13,[[["self"],[R[42]]],["bool"]]],[11,"ge",E,E,13,[[["self"],[R[42]]],["bool"]]],[11,R[111],R[100],E,16,[[["self"],[R[50]]],[[R[51],[R[110]]],[R[110]]]]],[11,"lt",E,E,16,[[["self"],[R[50]]],["bool"]]],[11,"le",E,E,16,[[["self"],[R[50]]],["bool"]]],[11,"gt",E,E,16,[[["self"],[R[50]]],["bool"]]],[11,"ge",E,E,16,[[["self"],[R[50]]],["bool"]]],[11,R[111],R[102],E,20,[[["self"],[R[50]]],[[R[51],[R[110]]],[R[110]]]]],[11,"lt",E,E,20,[[["self"],[R[50]]],["bool"]]],[11,"le",E,E,20,[[["self"],[R[50]]],["bool"]]],[11,"gt",E,E,20,[[["self"],[R[50]]],["bool"]]],[11,"ge",E,E,20,[[["self"],[R[50]]],["bool"]]],[11,R[112],R[92],R[113],11,[[["self"],[R[20]]]]],[11,R[112],E,R[113],12,[[["self"],[R[23]]]]],[11,R[112],E,R[113],13,[[["self"],[R[42]]]]],[11,R[112],R[100],R[113],16,[[[R[50]],["self"]]]],[11,R[112],R[102],R[113],20,[[[R[50]],["self"]]]],[11,"not",R[92],R[114],11,[[],[R[20]]]],[11,"not",E,R[114],12,[[],[R[23]]]],[11,"not",E,R[114],13,[[],[R[42]]]],[11,"not",R[100],R[114],16,[[],[R[50]]]],[11,"not",R[102],R[114],20,[[],[R[50]]]],[11,"bitand",R[92],R[115],11,[[[R[20]]],[R[20]]]],[11,"bitand",E,R[115],12,[[[R[23]]],[R[23]]]],[11,"bitand",E,R[115],13,[[[R[42]]],[R[42]]]],[11,"bitand",R[100],R[115],16,[[[R[50]]],[R[50]]]],[11,"bitand",R[102],R[115],20,[[[R[50]]],[R[50]]]],[11,"bitor",R[92],R[116],11,[[[R[20]]],[R[20]]]],[11,"bitor",E,R[116],12,[[[R[23]]],[R[23]]]],[11,"bitor",E,R[116],13,[[[R[42]]],[R[42]]]],[11,"bitor",R[100],R[116],16,[[[R[50]]],[R[50]]]],[11,"bitor",R[102],R[116],20,[[[R[50]]],[R[50]]]],[11,"bitxor",R[92],R[117],11,[[[R[20]]],[R[20]]]],[11,"bitxor",E,R[117],12,[[[R[23]]],[R[23]]]],[11,"bitxor",E,R[117],13,[[[R[42]]],[R[42]]]],[11,"bitxor",R[100],R[117],16,[[[R[50]]],[R[50]]]],[11,"bitxor",R[102],R[117],20,[[[R[50]]],[R[50]]]],[11,R[118],R[92],R[119],11,[[["self"],[R[20]]]]],[11,R[118],E,R[119],12,[[["self"],[R[23]]]]],[11,R[118],E,R[119],13,[[["self"],[R[42]]]]],[11,R[118],R[100],R[119],16,[[[R[50]],["self"]]]],[11,R[118],R[102],R[119],20,[[[R[50]],["self"]]]],[11,R[120],R[92],R[121],11,[[["self"],[R[20]]]]],[11,R[120],E,R[121],12,[[["self"],[R[23]]]]],[11,R[120],E,R[121],13,[[["self"],[R[42]]]]],[11,R[120],R[100],R[121],16,[[[R[50]],["self"]]]],[11,R[120],R[102],R[121],20,[[[R[50]],["self"]]]],[11,R[122],R[92],R[123],11,[[["self"],[R[20]]]]],[11,R[122],E,R[123],12,[[["self"],[R[23]]]]],[11,R[122],E,R[123],13,[[["self"],[R[42]]]]],[11,R[122],R[100],R[123],16,[[[R[50]],["self"]]]],[11,R[122],R[102],R[123],20,[[[R[50]],["self"]]]],[11,"deref",E,E,18,[[["self"]],[R[49]]]],[11,"hash",R[92],E,11,[[["self"],["__h"]]]],[11,"hash",E,E,12,[[["self"],["__h"]]]],[11,"hash",E,E,13,[[["self"],["__h"]]]],[11,"hash",R[100],E,16,[[["self"],["__h"]]]],[11,"hash",R[102],E,20,[[["self"],["__h"]]]],[11,R[124],R[92],E,11,[[[R[125]]],[R[20]]]],[11,R[124],E,E,12,[[[R[125]]],[R[23]]]],[11,R[124],E,E,13,[[[R[125]]],[R[42]]]],[11,R[124],R[100],E,16,[[[R[125]]],[R[50]]]],[11,R[124],R[102],E,20,[[[R[125]]],[R[50]]]],[11,"extend",R[92],E,11,[[["self"],[R[125]]]]],[11,"extend",E,E,12,[[["self"],[R[125]]]]],[11,"extend",E,E,13,[[["self"],[R[125]]]]],[11,"extend",R[100],E,16,[[["self"],[R[125]]]]],[11,"extend",R[102],E,20,[[["self"],[R[125]]]]],[11,"fmt",R[92],E,11,[[[R[108]],["self"]],[R[96]]]],[11,"fmt",E,E,12,[[[R[108]],["self"]],[R[96]]]],[11,"fmt",E,E,13,[[[R[108]],["self"]],[R[96]]]],[11,"fmt",R[100],E,16,[[[R[108]],["self"]],[R[96]]]],[11,"fmt",R[102],E,20,[[[R[108]],["self"]],[R[96]]]],[11,"fmt",R[92],E,11,[[[R[108]],["self"]],[R[96]]]],[11,"fmt",E,E,12,[[[R[108]],["self"]],[R[96]]]],[11,"fmt",E,E,13,[[[R[108]],["self"]],[R[96]]]],[11,"fmt",R[100],E,16,[[[R[108]],["self"]],[R[96]]]],[11,"fmt",R[102],E,20,[[[R[108]],["self"]],[R[96]]]],[11,"fmt",R[92],E,11,[[[R[108]],["self"]],[R[96]]]],[11,"fmt",E,E,12,[[[R[108]],["self"]],[R[96]]]],[11,"fmt",E,E,13,[[[R[108]],["self"]],[R[96]]]],[11,"fmt",R[100],E,16,[[[R[108]],["self"]],[R[96]]]],[11,"fmt",R[102],E,20,[[[R[108]],["self"]],[R[96]]]],[11,"fmt",R[92],E,11,[[[R[108]],["self"]],[R[96]]]],[11,"fmt",E,E,12,[[[R[108]],["self"]],[R[96]]]],[11,"fmt",E,E,13,[[[R[108]],["self"]],[R[96]]]],[11,"fmt",R[100],E,16,[[[R[108]],["self"]],[R[96]]]],[11,"fmt",R[102],E,20,[[[R[108]],["self"]],[R[96]]]],[11,"clone",R[92],E,1,[[["self"]],["dither"]]],[11,"clone",E,E,2,[[["self"]],[R[43]]]],[11,"clone",E,E,3,[[["self"]],[R[126]]]],[11,"clone",E,E,5,[[["self"]],[R[17]]]],[11,"clone",E,E,11,[[["self"]],[R[20]]]],[11,"clone",E,E,12,[[["self"]],[R[23]]]],[11,"clone",E,E,13,[[["self"]],[R[42]]]],[11,"clone",E,E,7,[[["self"]],[R[127]]]],[11,"clone",E,E,9,[[["self"]],[R[128]]]],[11,"clone",E,E,10,[[["self"]],[R[44]]]],[11,"clone",R[100],E,16,[[["self"]],[R[50]]]],[11,"clone",R[102],E,20,[[["self"]],[R[50]]]],[11,R[129],R[45],E,21,[[],["tc"]]],[11,R[129],R[92],"Return `Dither::None`",1,[[],["self"]]],[11,R[129],E,"Return `Prescaler::Div1`",2,[[],["self"]]],[11,R[129],E,"Return `Synchronization::Clock`",3,[[],["self"]]],[11,R[129],E,"Return `Ramp::Ramp1`",4,[[],["self"]]],[11,R[129],E,"Return `CaptureMode::Default`",5,[[],["self"]]],[11,R[129],E,"Return `CaptureTrigger::Event`",6,[[],["self"]]],[11,R[129],E,E,11,[[],[R[20]]]],[11,R[129],E,E,12,[[],[R[23]]]],[11,R[129],E,E,13,[[],[R[42]]]],[11,R[129],E,"Return `OutputMatrixConfig::Default`",7,[[],["self"]]],[11,R[129],E,"Return `TimerMode::Count16`",8,[[],["self"]]],[11,R[129],E,"Return `TimerWaveGen::NFRQ`",9,[[],["self"]]],[11,R[129],E,E,10,[[],[R[44]]]],[11,R[129],E,E,22,[[],["tcc"]]],[11,R[129],E,E,23,[[],[R[11]]]],[11,R[129],E,E,24,[[],[R[13]]]],[11,R[129],E,E,25,[[],[R[14]]]],[11,R[129],R[100],E,16,[[],[R[50]]]],[11,R[129],E,"Return `WaveformGen::NFRQ`",15,[[],["self"]]],[11,R[129],R[102],E,20,[[],[R[50]]]]],"p":[[4,R[130]],[4,"Dither"],[4,R[2]],[4,R[131]],[4,"Ramp"],[4,R[132]],[4,R[133]],[4,R[134]],[4,R[135]],[4,R[136]],[3,R[137]],[3,"Channels"],[3,R[138]],[3,R[46]],[8,R[139]],[4,R[140]],[3,R[56]],[3,R[142]],[3,"TC4_5"],[3,"Timer"],[3,R[56]],[3,"TC"],[3,"TCC"],[3,R[5]],[3,R[4]],[3,R[6]]]};
initSearch(searchIndex);addSearchOptions(searchIndex);