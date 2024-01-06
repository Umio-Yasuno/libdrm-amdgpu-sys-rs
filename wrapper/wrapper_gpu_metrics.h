/* from linux-6.2.7 drivers/gpu/drm/amd/include/kgd_pp_interface.h */

/*
 * Copyright 2017 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 */

#define NUM_HBM_INSTANCES 4
#define NUM_XGMI_LINKS 8
#define MAX_GFX_CLKS 8
#define MAX_CLKS 4
#define NUM_VCN 4
#define NUM_JPEG_ENG 32

struct metrics_table_header {
	uint16_t			structure_size;
	uint8_t				format_revision;
	uint8_t				content_revision;
};

/*
 * gpu_metrics_v1_0 is not recommended as it's not naturally aligned.
 * Use gpu_metrics_v1_1 or later instead.
 */
struct gpu_metrics_v1_0 {
	struct metrics_table_header	common_header;

	/* Driver attached timestamp (in ns) */
	uint64_t			system_clock_counter;

	/* Temperature */
	uint16_t			temperature_edge;
	uint16_t			temperature_hotspot;
	uint16_t			temperature_mem;
	uint16_t			temperature_vrgfx;
	uint16_t			temperature_vrsoc;
	uint16_t			temperature_vrmem;

	/* Utilization */
	uint16_t			average_gfx_activity;
	uint16_t			average_umc_activity; // memory controller
	uint16_t			average_mm_activity; // UVD or VCN

	/* Power/Energy */
	uint16_t			average_socket_power;
	uint32_t			energy_accumulator;

	/* Average clocks */
	uint16_t			average_gfxclk_frequency;
	uint16_t			average_socclk_frequency;
	uint16_t			average_uclk_frequency;
	uint16_t			average_vclk0_frequency;
	uint16_t			average_dclk0_frequency;
	uint16_t			average_vclk1_frequency;
	uint16_t			average_dclk1_frequency;

	/* Current clocks */
	uint16_t			current_gfxclk;
	uint16_t			current_socclk;
	uint16_t			current_uclk;
	uint16_t			current_vclk0;
	uint16_t			current_dclk0;
	uint16_t			current_vclk1;
	uint16_t			current_dclk1;

	/* Throttle status */
	uint32_t			throttle_status;

	/* Fans */
	uint16_t			current_fan_speed;

	/* Link width/speed */
	uint8_t				pcie_link_width;
	uint8_t				pcie_link_speed; // in 0.1 GT/s
};

struct gpu_metrics_v1_1 {
	struct metrics_table_header	common_header;

	/* Temperature */
	uint16_t			temperature_edge;
	uint16_t			temperature_hotspot;
	uint16_t			temperature_mem;
	uint16_t			temperature_vrgfx;
	uint16_t			temperature_vrsoc;
	uint16_t			temperature_vrmem;

	/* Utilization */
	uint16_t			average_gfx_activity;
	uint16_t			average_umc_activity; // memory controller
	uint16_t			average_mm_activity; // UVD or VCN

	/* Power/Energy */
	uint16_t			average_socket_power;
	uint64_t			energy_accumulator;

	/* Driver attached timestamp (in ns) */
	uint64_t			system_clock_counter;

	/* Average clocks */
	uint16_t			average_gfxclk_frequency;
	uint16_t			average_socclk_frequency;
	uint16_t			average_uclk_frequency;
	uint16_t			average_vclk0_frequency;
	uint16_t			average_dclk0_frequency;
	uint16_t			average_vclk1_frequency;
	uint16_t			average_dclk1_frequency;

	/* Current clocks */
	uint16_t			current_gfxclk;
	uint16_t			current_socclk;
	uint16_t			current_uclk;
	uint16_t			current_vclk0;
	uint16_t			current_dclk0;
	uint16_t			current_vclk1;
	uint16_t			current_dclk1;

	/* Throttle status */
	uint32_t			throttle_status;

	/* Fans */
	uint16_t			current_fan_speed;

	/* Link width/speed */
	uint16_t			pcie_link_width;
	uint16_t			pcie_link_speed; // in 0.1 GT/s

	uint16_t			padding;

	uint32_t			gfx_activity_acc;
	uint32_t			mem_activity_acc;

	uint16_t			temperature_hbm[NUM_HBM_INSTANCES];
};

struct gpu_metrics_v1_2 {
	struct metrics_table_header	common_header;

	/* Temperature */
	uint16_t			temperature_edge;
	uint16_t			temperature_hotspot;
	uint16_t			temperature_mem;
	uint16_t			temperature_vrgfx;
	uint16_t			temperature_vrsoc;
	uint16_t			temperature_vrmem;

	/* Utilization */
	uint16_t			average_gfx_activity;
	uint16_t			average_umc_activity; // memory controller
	uint16_t			average_mm_activity; // UVD or VCN

	/* Power/Energy */
	uint16_t			average_socket_power;
	uint64_t			energy_accumulator;

	/* Driver attached timestamp (in ns) */
	uint64_t			system_clock_counter;

	/* Average clocks */
	uint16_t			average_gfxclk_frequency;
	uint16_t			average_socclk_frequency;
	uint16_t			average_uclk_frequency;
	uint16_t			average_vclk0_frequency;
	uint16_t			average_dclk0_frequency;
	uint16_t			average_vclk1_frequency;
	uint16_t			average_dclk1_frequency;

	/* Current clocks */
	uint16_t			current_gfxclk;
	uint16_t			current_socclk;
	uint16_t			current_uclk;
	uint16_t			current_vclk0;
	uint16_t			current_dclk0;
	uint16_t			current_vclk1;
	uint16_t			current_dclk1;

	/* Throttle status (ASIC dependent) */
	uint32_t			throttle_status;

	/* Fans */
	uint16_t			current_fan_speed;

	/* Link width/speed */
	uint16_t			pcie_link_width;
	uint16_t			pcie_link_speed; // in 0.1 GT/s

	uint16_t			padding;

	uint32_t			gfx_activity_acc;
	uint32_t			mem_activity_acc;

	uint16_t			temperature_hbm[NUM_HBM_INSTANCES];

	/* PMFW attached timestamp (10ns resolution) */
	uint64_t			firmware_timestamp;
};

struct gpu_metrics_v1_3 {
	struct metrics_table_header	common_header;

	/* Temperature */
	uint16_t			temperature_edge;
	uint16_t			temperature_hotspot;
	uint16_t			temperature_mem;
	uint16_t			temperature_vrgfx;
	uint16_t			temperature_vrsoc;
	uint16_t			temperature_vrmem;

	/* Utilization */
	uint16_t			average_gfx_activity;
	uint16_t			average_umc_activity; // memory controller
	uint16_t			average_mm_activity; // UVD or VCN

	/* Power/Energy */
	uint16_t			average_socket_power;
	uint64_t			energy_accumulator;

	/* Driver attached timestamp (in ns) */
	uint64_t			system_clock_counter;

	/* Average clocks */
	uint16_t			average_gfxclk_frequency;
	uint16_t			average_socclk_frequency;
	uint16_t			average_uclk_frequency;
	uint16_t			average_vclk0_frequency;
	uint16_t			average_dclk0_frequency;
	uint16_t			average_vclk1_frequency;
	uint16_t			average_dclk1_frequency;

	/* Current clocks */
	uint16_t			current_gfxclk;
	uint16_t			current_socclk;
	uint16_t			current_uclk;
	uint16_t			current_vclk0;
	uint16_t			current_dclk0;
	uint16_t			current_vclk1;
	uint16_t			current_dclk1;

	/* Throttle status */
	uint32_t			throttle_status;

	/* Fans */
	uint16_t			current_fan_speed;

	/* Link width/speed */
	uint16_t			pcie_link_width;
	uint16_t			pcie_link_speed; // in 0.1 GT/s

	uint16_t			padding;

	uint32_t			gfx_activity_acc;
	uint32_t			mem_activity_acc;

	uint16_t			temperature_hbm[NUM_HBM_INSTANCES];

	/* PMFW attached timestamp (10ns resolution) */
	uint64_t			firmware_timestamp;

	/* Voltage (mV) */
	uint16_t			voltage_soc;
	uint16_t			voltage_gfx;
	uint16_t			voltage_mem;

	uint16_t			padding1;

	/* Throttle status (ASIC independent) */
	uint64_t			indep_throttle_status;
};

struct gpu_metrics_v1_4 {
	struct metrics_table_header	common_header;

	/* Temperature (Celsius) */
	uint16_t			temperature_hotspot;
	uint16_t			temperature_mem;
	uint16_t			temperature_vrsoc;

	/* Power (Watts) */
	uint16_t			curr_socket_power;

	/* Utilization (%) */
	uint16_t			average_gfx_activity;
	uint16_t			average_umc_activity; // memory controller
	uint16_t			vcn_activity[NUM_VCN];

	/* Energy (15.259uJ (2^-16) units) */
	uint64_t			energy_accumulator;

	/* Driver attached timestamp (in ns) */
	uint64_t			system_clock_counter;

	/* Throttle status */
	uint32_t			throttle_status;

	/* Clock Lock Status. Each bit corresponds to clock instance */
	uint32_t			gfxclk_lock_status;

	/* Link width (number of lanes) and speed (in 0.1 GT/s) */
	uint16_t			pcie_link_width;
	uint16_t			pcie_link_speed;

	/* XGMI bus width and bitrate (in Gbps) */
	uint16_t			xgmi_link_width;
	uint16_t			xgmi_link_speed;

	/* Utilization Accumulated (%) */
	uint32_t			gfx_activity_acc;
	uint32_t			mem_activity_acc;

	/*PCIE accumulated bandwidth (GB/sec) */
	uint64_t			pcie_bandwidth_acc;

	/*PCIE instantaneous bandwidth (GB/sec) */
	uint64_t			pcie_bandwidth_inst;

	/* PCIE L0 to recovery state transition accumulated count */
	uint64_t			pcie_l0_to_recov_count_acc;

	/* PCIE replay accumulated count */
	uint64_t			pcie_replay_count_acc;

	/* PCIE replay rollover accumulated count */
	uint64_t			pcie_replay_rover_count_acc;

	/* XGMI accumulated data transfer size(KiloBytes) */
	uint64_t			xgmi_read_data_acc[NUM_XGMI_LINKS];
	uint64_t			xgmi_write_data_acc[NUM_XGMI_LINKS];

	/* PMFW attached timestamp (10ns resolution) */
	uint64_t			firmware_timestamp;

	/* Current clocks (Mhz) */
	uint16_t			current_gfxclk[MAX_GFX_CLKS];
	uint16_t			current_socclk[MAX_CLKS];
	uint16_t			current_vclk0[MAX_CLKS];
	uint16_t			current_dclk0[MAX_CLKS];
	uint16_t			current_uclk;

	uint16_t			padding;
};

struct gpu_metrics_v1_5 {
	struct metrics_table_header	common_header;

	/* Temperature (Celsius) */
	uint16_t			temperature_hotspot;
	uint16_t			temperature_mem;
	uint16_t			temperature_vrsoc;

	/* Power (Watts) */
	uint16_t			curr_socket_power;

	/* Utilization (%) */
	uint16_t			average_gfx_activity;
	uint16_t			average_umc_activity; // memory controller
	uint16_t			vcn_activity[NUM_VCN];
	uint16_t			jpeg_activity[NUM_JPEG_ENG];

	/* Energy (15.259uJ (2^-16) units) */
	uint64_t			energy_accumulator;

	/* Driver attached timestamp (in ns) */
	uint64_t			system_clock_counter;

	/* Throttle status */
	uint32_t			throttle_status;

	/* Clock Lock Status. Each bit corresponds to clock instance */
	uint32_t			gfxclk_lock_status;

	/* Link width (number of lanes) and speed (in 0.1 GT/s) */
	uint16_t			pcie_link_width;
	uint16_t			pcie_link_speed;

	/* XGMI bus width and bitrate (in Gbps) */
	uint16_t			xgmi_link_width;
	uint16_t			xgmi_link_speed;

	/* Utilization Accumulated (%) */
	uint32_t			gfx_activity_acc;
	uint32_t			mem_activity_acc;

	/*PCIE accumulated bandwidth (GB/sec) */
	uint64_t			pcie_bandwidth_acc;

	/*PCIE instantaneous bandwidth (GB/sec) */
	uint64_t			pcie_bandwidth_inst;

	/* PCIE L0 to recovery state transition accumulated count */
	uint64_t			pcie_l0_to_recov_count_acc;

	/* PCIE replay accumulated count */
	uint64_t			pcie_replay_count_acc;

	/* PCIE replay rollover accumulated count */
	uint64_t			pcie_replay_rover_count_acc;

	/* PCIE NAK sent  accumulated count */
	uint32_t			pcie_nak_sent_count_acc;

	/* PCIE NAK received accumulated count */
	uint32_t			pcie_nak_rcvd_count_acc;

	/* XGMI accumulated data transfer size(KiloBytes) */
	uint64_t			xgmi_read_data_acc[NUM_XGMI_LINKS];
	uint64_t			xgmi_write_data_acc[NUM_XGMI_LINKS];

	/* PMFW attached timestamp (10ns resolution) */
	uint64_t			firmware_timestamp;

	/* Current clocks (Mhz) */
	uint16_t			current_gfxclk[MAX_GFX_CLKS];
	uint16_t			current_socclk[MAX_CLKS];
	uint16_t			current_vclk0[MAX_CLKS];
	uint16_t			current_dclk0[MAX_CLKS];
	uint16_t			current_uclk;

	uint16_t			padding;
};

/*
 * gpu_metrics_v2_0 is not recommended as it's not naturally aligned.
 * Use gpu_metrics_v2_1 or later instead.
 */
struct gpu_metrics_v2_0 {
	struct metrics_table_header	common_header;

	/* Driver attached timestamp (in ns) */
	uint64_t			system_clock_counter;

	/* Temperature */
	uint16_t			temperature_gfx; // gfx temperature on APUs
	uint16_t			temperature_soc; // soc temperature on APUs
	uint16_t			temperature_core[8]; // CPU core temperature on APUs
	uint16_t			temperature_l3[2];

	/* Utilization */
	uint16_t			average_gfx_activity;
	uint16_t			average_mm_activity; // UVD or VCN

	/* Power/Energy */
	uint16_t			average_socket_power; // dGPU + APU power on A + A platform
	uint16_t			average_cpu_power;
	uint16_t			average_soc_power;
	uint16_t			average_gfx_power;
	uint16_t			average_core_power[8]; // CPU core power on APUs

	/* Average clocks */
	uint16_t			average_gfxclk_frequency;
	uint16_t			average_socclk_frequency;
	uint16_t			average_uclk_frequency;
	uint16_t			average_fclk_frequency;
	uint16_t			average_vclk_frequency;
	uint16_t			average_dclk_frequency;

	/* Current clocks */
	uint16_t			current_gfxclk;
	uint16_t			current_socclk;
	uint16_t			current_uclk;
	uint16_t			current_fclk;
	uint16_t			current_vclk;
	uint16_t			current_dclk;
	uint16_t			current_coreclk[8]; // CPU core clocks
	uint16_t			current_l3clk[2];

	/* Throttle status */
	uint32_t			throttle_status;

	/* Fans */
	uint16_t			fan_pwm;

	uint16_t			padding;
};

struct gpu_metrics_v2_1 {
	struct metrics_table_header	common_header;

	/* Temperature */
	uint16_t			temperature_gfx; // gfx temperature on APUs
	uint16_t			temperature_soc; // soc temperature on APUs
	uint16_t			temperature_core[8]; // CPU core temperature on APUs
	uint16_t			temperature_l3[2];

	/* Utilization */
	uint16_t			average_gfx_activity;
	uint16_t			average_mm_activity; // UVD or VCN

	/* Driver attached timestamp (in ns) */
	uint64_t			system_clock_counter;

	/* Power/Energy */
	uint16_t			average_socket_power; // dGPU + APU power on A + A platform
	uint16_t			average_cpu_power;
	uint16_t			average_soc_power;
	uint16_t			average_gfx_power;
	uint16_t			average_core_power[8]; // CPU core power on APUs

	/* Average clocks */
	uint16_t			average_gfxclk_frequency;
	uint16_t			average_socclk_frequency;
	uint16_t			average_uclk_frequency;
	uint16_t			average_fclk_frequency;
	uint16_t			average_vclk_frequency;
	uint16_t			average_dclk_frequency;

	/* Current clocks */
	uint16_t			current_gfxclk;
	uint16_t			current_socclk;
	uint16_t			current_uclk;
	uint16_t			current_fclk;
	uint16_t			current_vclk;
	uint16_t			current_dclk;
	uint16_t			current_coreclk[8]; // CPU core clocks
	uint16_t			current_l3clk[2];

	/* Throttle status */
	uint32_t			throttle_status;

	/* Fans */
	uint16_t			fan_pwm;

	uint16_t			padding[3];
};

struct gpu_metrics_v2_2 {
	struct metrics_table_header	common_header;

	/* Temperature */
	uint16_t			temperature_gfx; // gfx temperature on APUs
	uint16_t			temperature_soc; // soc temperature on APUs
	uint16_t			temperature_core[8]; // CPU core temperature on APUs
	uint16_t			temperature_l3[2];

	/* Utilization */
	uint16_t			average_gfx_activity;
	uint16_t			average_mm_activity; // UVD or VCN

	/* Driver attached timestamp (in ns) */
	uint64_t			system_clock_counter;

	/* Power/Energy */
	uint16_t			average_socket_power; // dGPU + APU power on A + A platform
	uint16_t			average_cpu_power;
	uint16_t			average_soc_power;
	uint16_t			average_gfx_power;
	uint16_t			average_core_power[8]; // CPU core power on APUs

	/* Average clocks */
	uint16_t			average_gfxclk_frequency;
	uint16_t			average_socclk_frequency;
	uint16_t			average_uclk_frequency;
	uint16_t			average_fclk_frequency;
	uint16_t			average_vclk_frequency;
	uint16_t			average_dclk_frequency;

	/* Current clocks */
	uint16_t			current_gfxclk;
	uint16_t			current_socclk;
	uint16_t			current_uclk;
	uint16_t			current_fclk;
	uint16_t			current_vclk;
	uint16_t			current_dclk;
	uint16_t			current_coreclk[8]; // CPU core clocks
	uint16_t			current_l3clk[2];

	/* Throttle status (ASIC dependent) */
	uint32_t			throttle_status;

	/* Fans */
	uint16_t			fan_pwm;

	uint16_t			padding[3];

	/* Throttle status (ASIC independent) */
	uint64_t			indep_throttle_status;
};

struct gpu_metrics_v2_3 {
	struct metrics_table_header	common_header;

	/* Temperature */
	uint16_t			temperature_gfx; // gfx temperature on APUs
	uint16_t			temperature_soc; // soc temperature on APUs
	uint16_t			temperature_core[8]; // CPU core temperature on APUs
	uint16_t			temperature_l3[2];

	/* Utilization */
	uint16_t			average_gfx_activity;
	uint16_t			average_mm_activity; // UVD or VCN

	/* Driver attached timestamp (in ns) */
	uint64_t			system_clock_counter;

	/* Power/Energy */
	uint16_t			average_socket_power; // dGPU + APU power on A + A platform
	uint16_t			average_cpu_power;
	uint16_t			average_soc_power;
	uint16_t			average_gfx_power;
	uint16_t			average_core_power[8]; // CPU core power on APUs

	/* Average clocks */
	uint16_t			average_gfxclk_frequency;
	uint16_t			average_socclk_frequency;
	uint16_t			average_uclk_frequency;
	uint16_t			average_fclk_frequency;
	uint16_t			average_vclk_frequency;
	uint16_t			average_dclk_frequency;

	/* Current clocks */
	uint16_t			current_gfxclk;
	uint16_t			current_socclk;
	uint16_t			current_uclk;
	uint16_t			current_fclk;
	uint16_t			current_vclk;
	uint16_t			current_dclk;
	uint16_t			current_coreclk[8]; // CPU core clocks
	uint16_t			current_l3clk[2];

	/* Throttle status (ASIC dependent) */
	uint32_t			throttle_status;

	/* Fans */
	uint16_t			fan_pwm;

	uint16_t			padding[3];

	/* Throttle status (ASIC independent) */
	uint64_t			indep_throttle_status;

	/* Average Temperature */
	uint16_t			average_temperature_gfx; // average gfx temperature on APUs
	uint16_t			average_temperature_soc; // average soc temperature on APUs
	uint16_t			average_temperature_core[8]; // average CPU core temperature on APUs
	uint16_t			average_temperature_l3[2];
};

struct gpu_metrics_v2_4 {
	struct metrics_table_header	common_header;

	/* Temperature (unit: centi-Celsius) */
	uint16_t			temperature_gfx;
	uint16_t			temperature_soc;
	uint16_t			temperature_core[8];
	uint16_t			temperature_l3[2];

	/* Utilization (unit: centi) */
	uint16_t			average_gfx_activity;
	uint16_t			average_mm_activity;

	/* Driver attached timestamp (in ns) */
	uint64_t			system_clock_counter;

	/* Power/Energy (unit: mW) */
	uint16_t			average_socket_power;
	uint16_t			average_cpu_power;
	uint16_t			average_soc_power;
	uint16_t			average_gfx_power;
	uint16_t			average_core_power[8];

	/* Average clocks (unit: MHz) */
	uint16_t			average_gfxclk_frequency;
	uint16_t			average_socclk_frequency;
	uint16_t			average_uclk_frequency;
	uint16_t			average_fclk_frequency;
	uint16_t			average_vclk_frequency;
	uint16_t			average_dclk_frequency;

	/* Current clocks (unit: MHz) */
	uint16_t			current_gfxclk;
	uint16_t			current_socclk;
	uint16_t			current_uclk;
	uint16_t			current_fclk;
	uint16_t			current_vclk;
	uint16_t			current_dclk;
	uint16_t			current_coreclk[8];
	uint16_t			current_l3clk[2];

	/* Throttle status (ASIC dependent) */
	uint32_t			throttle_status;

	/* Fans */
	uint16_t			fan_pwm;

	uint16_t			padding[3];

	/* Throttle status (ASIC independent) */
	uint64_t			indep_throttle_status;

	/* Average Temperature (unit: centi-Celsius) */
	uint16_t			average_temperature_gfx;
	uint16_t			average_temperature_soc;
	uint16_t			average_temperature_core[8];
	uint16_t			average_temperature_l3[2];

	/* Power/Voltage (unit: mV) */
	uint16_t			average_cpu_voltage;
	uint16_t			average_soc_voltage;
	uint16_t			average_gfx_voltage;

	/* Power/Current (unit: mA) */
	uint16_t			average_cpu_current;
	uint16_t			average_soc_current;
	uint16_t			average_gfx_current;
};

struct gpu_metrics_v3_0 {
	struct metrics_table_header	common_header;

	/* Temperature */
	/* gfx temperature on APUs */
	uint16_t			temperature_gfx;
	/* soc temperature on APUs */
	uint16_t			temperature_soc;
	/* CPU core temperature on APUs */
	uint16_t			temperature_core[16];
	/* skin temperature on APUs */
	uint16_t			temperature_skin;

	/* Utilization */
	/* time filtered GFX busy % [0-100] */
	uint16_t			average_gfx_activity;
	/* time filtered VCN busy % [0-100] */
	uint16_t			average_vcn_activity;
	/* time filtered IPU per-column busy % [0-100] */
	uint16_t			average_ipu_activity[8];
	/* time filtered per-core C0 residency % [0-100]*/
	uint16_t			average_core_c0_activity[16];
	/* time filtered DRAM read bandwidth [MB/sec] */
	uint16_t			average_dram_reads;
	/* time filtered DRAM write bandwidth [MB/sec] */
	uint16_t			average_dram_writes;
	/* time filtered IPU read bandwidth [MB/sec] */
	uint16_t			average_ipu_reads;
	/* time filtered IPU write bandwidth [MB/sec] */
	uint16_t			average_ipu_writes;

	/* Driver attached timestamp (in ns) */
	uint64_t			system_clock_counter;

	/* Power/Energy */
	/* time filtered power used for PPT/STAPM [APU+dGPU] [mW] */
	uint32_t			average_socket_power;
	/* time filtered IPU power [mW] */
	uint16_t			average_ipu_power;
	/* time filtered APU power [mW] */
	uint32_t			average_apu_power;
	/* time filtered GFX power [mW] */
	uint32_t			average_gfx_power;
	/* time filtered dGPU power [mW] */
	uint32_t			average_dgpu_power;
	/* time filtered sum of core power across all cores in the socket [mW] */
	uint32_t			average_all_core_power;
	/* calculated core power [mW] */
	uint16_t			average_core_power[16];
	/* time filtered total system power [mW] */
	uint16_t			average_sys_power;
	/* maximum IRM defined STAPM power limit [mW] */
	uint16_t			stapm_power_limit;
	/* time filtered STAPM power limit [mW] */
	uint16_t			current_stapm_power_limit;

	/* time filtered clocks [MHz] */
	uint16_t			average_gfxclk_frequency;
	uint16_t			average_socclk_frequency;
	uint16_t			average_vpeclk_frequency;
	uint16_t			average_ipuclk_frequency;
	uint16_t			average_fclk_frequency;
	uint16_t			average_vclk_frequency;
	uint16_t			average_uclk_frequency;
	uint16_t			average_mpipu_frequency;

	/* Current clocks */
	/* target core frequency [MHz] */
	uint16_t			current_coreclk[16];
	/* CCLK frequency limit enforced on classic cores [MHz] */
	uint16_t			current_core_maxfreq;
	/* GFXCLK frequency limit enforced on GFX [MHz] */
	uint16_t			current_gfx_maxfreq;

	/* Throttle Residency (ASIC dependent) */
	uint32_t			throttle_residency_prochot;
	uint32_t			throttle_residency_spl;
	uint32_t			throttle_residency_fppt;
	uint32_t			throttle_residency_sppt;
	uint32_t			throttle_residency_thm_core;
	uint32_t			throttle_residency_thm_gfx;
	uint32_t			throttle_residency_thm_soc;

	/* Metrics table alpha filter time constant [us] */
	uint32_t			time_filter_alphavalue;
};
