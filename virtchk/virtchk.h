#include <stdlib.h>

typedef unsigned __int8		u8;
typedef unsigned __int16	u16;
typedef unsigned __int32	u32;
typedef unsigned __int64	u64;

typedef signed __int8		i8;
typedef signed __int16		i16;
typedef signed __int32		i32;
typedef signed __int64		i64;

#if defined(_WIN64)
typedef unsigned __int64	ulong_ptr;
typedef signed __int64		long_ptr;
#else
typedef unsigned __int32	ulong_ptr;
typedef signed __int32		long_ptr;
#endif

typedef enum
{
	false=0,
	true=1
}bool;

#define inline	__forceinline

#define null	(void*)0

#define intel_processor		0
#define amd_processor		1
#define unknown_processor	0xff

#define vtc_bt		_bittest

typedef union _svm_feat_info
{
	struct
	{
		u32 npt:1;
		u32 lbr_virt:1;
		u32 svm_lock:1;
		u32 next_rip:1;
		u32 tsc_rate:1;
		u32 vmcb_clean:1;
		u32 flush_asid:1;
		u32 decoder:1;
		u32 reserved0:2;
		u32 pause_filter:1;
		u32 reserved1:1;
		u32 pause_filter_threshold:1;
		u32 avic:1;
		u32 reserved2:1;
		u32 svm_ins_virt:1;
		u32 svm_gif_virt:1;
		u32 reserved:15;
	};
	u32 value;
}svm_feat_info,*svm_feat_info_p;

#define ia32_vmx_basic					0x480
#define ia32_vmx_pinbased_ctrl			0x481
#define ia32_vmx_procbased_ctrl			0x482
#define ia32_vmx_exit_ctrl				0x483
#define ia32_vmx_entry_ctrl				0x484
#define ia32_vmx_misc					0x485
#define ia32_vmx_cr0_fixed0				0x486
#define ia32_vmx_cr0_fixed1				0x487
#define ia32_vmx_cr4_fixed0				0x488
#define ia32_vmx_cr4_fixed1				0x489
#define ia32_vmx_vmcs_enum				0x48A
#define ia32_vmx_procbased2_ctrl		0x48B
#define ia32_vmx_ept_vpid_cap			0x48C
#define ia32_vmx_true_pinbased_ctrl		0x48D
#define ia32_vmx_true_procbased_ctrl	0x48E
#define ia32_vmx_true_exit_ctrl			0x48F
#define ia32_vmx_true_entry_ctrl		0x490
#define ia32_vmx_vmfunc_ctrl			0x491

typedef union _vmx_basic_msr
{
	struct
	{
		u64 revision_id:31;
		u64 reserved0:1;
		u64 vmxon_region_size:13;
		u64 reserved1:3;
		u64 pa_width:1;
		u64 dual_monitor:1;
		u64 memory_type:4;
		u64 advanced_ios_exit_info:1;
		u64 use_true_msr:1;
		u64 reserved2:8;
	};
	u64 value;
}vmx_basic_msr,*vmx_basic_msr_p;

typedef union _vmx_pinbased_ctrl
{
	struct
	{
		u32 external_interrupt:1;					//Bit	0
		u32 reserved0:2;							//Bits	1-2
		u32 nmi_exiting:1;							//Bit	3
		u32 reserved1:1;							//Bit	4
		u32 virtual_nmi:1;							//Bit	5
		u32 activate_vmx_preemption_timer:1;		//Bit	6
		u32 process_posted_interrupts:1;			//Bit	7
		u32 reserved2:24;							//Bits	8-31
	};
	u32 value;
}vmx_pinbased_ctrl,*vmx_pinbased_ctrl_p;

typedef union _vmx_procbased_ctrl
{
	struct
	{
		u32 reserved0:2;					//Bits	0-1
		u32 interrupt_window_exiting:1;		//Bit	2
		u32 use_tsc_offseting:1;			//Bit	3
		u32 reserved1:3;					//Bits	4-6
		u32 hlt_exiting:1;					//Bit	7
		u32 reserved2:1;					//Bit	8
		u32 invlpg_exiting:1;				//Bit	9
		u32 mwait_exiting:1;				//Bit	10
		u32 rdpmc_exiting:1;				//Bit	11
		u32 rdtsc_exiting:1;				//Bit	12
		u32 reserved3:2;					//Bits	13-14
		u32 cr3_load_exiting:1;				//Bit	15
		u32 cr3_store_exiting:1;			//Bit	16
		u32 reserved4:2;					//Bits	17-18
		u32 cr8_load_exiting:1;				//Bit	19
		u32 cr8_store_exiting:1;			//Bit	20
		u32 use_tpr_shadow:1;				//Bit	21
		u32 nmi_window_exiting:1;			//Bit	22
		u32 mov_dr_exiting:1;				//Bit	23
		u32 io_exiting:1;					//Bit	24
		u32 use_io_bitmap:1;				//Bit	25
		u32 reserved5:1;					//Bit	26
		u32 monitor_trap_flag:1;			//Bit	27
		u32 use_msr_bitmap:1;				//Bit	28
		u32 monitor_exiting:1;				//Bit	29
		u32 pause_exiting:1;				//Bit	30
		u32 activate_secondary_control:1;	//Bit	31
	};
	u32 value;
}vmx_procbased_ctrl,*vmx_procbased_ctrl_p;

typedef union _vmx_procbased2_ctrl
{
	struct
	{
		u32 virtualize_apic:1;				//Bit	0
		u32 enable_ept:1;					//Bit	1
		u32 descriptor_table_exiting:1;		//Bit	2
		u32 enable_rdtscp:1;				//Bit	3
		u32 virtualize_x2apic:1;			//Bit	4
		u32 enable_vpid:1;					//Bit	5
		u32 wbinvd_exiting:1;				//Bit	6
		u32	unrestricted_guest:1;			//Bit	7
		u32 apic_register_virtualization:1;	//Bit	8
		u32 virtual_interrupt_delivery:1;	//Bit	9
		u32 pause_loop_exiting:1;			//Bit	10
		u32 rdrand_exiting:1;				//Bit	11
		u32 enable_invpcid:1;				//Bit	12
		u32 enable_vmfunc:1;				//Bit	13
		u32 vmcs_shadowing:1;				//Bit	14
		u32 enable_encls_exiting:1;			//Bit	15
		u32 rdseed_exiting:1;				//Bit	16
		u32 enable_pml:1;					//Bit	17
		u32 virtualization_exception:1;		//Bit	18
		u32 conceal_vmx_from_ipt:1;			//Bit	19
		u32 enable_xsave_xrstors:1;			//Bit	20
		u32 reserved0:1;					//Bit	21
		u32 ept_mode_based_control:1;		//Bit	22
		u32 reserved1:2;					//Bits	23-24
		u32 use_tsc_scaling:1;				//Bit	25
		u32 reserved2:6;					//Bits	26-31
	};
	u32 value;
}vmx_procbased2_ctrl,*vmx_procbased2_ctrl_p;

typedef union _vmx_exit_ctrl
{
	struct
	{
		u32 reserved0:2;						//Bits	0-1
		u32 save_debug_controls:1;				//Bit	2
		u32 reserved1:6;						//Bits	3-8
		u32 host_address_space_size:1;			//Bit	9
		u32 reserved2:2;						//Bits	10-11
		u32 load_ia32_perf_global_ctrl:1;		//Bit	12
		u32 reserved3:2;						//Bits	13-14
		u32 acknowledge_interrupt_on_exit:1;	//Bit	15
		u32 reserved4:2;						//Bits	16-17
		u32 save_ia32_pat:1;					//Bit	18
		u32 load_ia32_pat:1;					//Bit	19
		u32 save_ia32_efer:1;					//Bit	20
		u32 load_ia32_efer:1;					//Bit	21
		u32 save_vmx_preemption_timer_value:1;	//Bit	22
		u32 clear_ia32_bndcfgs:1;				//Bit	23
		u32 conceal_vmx_from_ipt:1;				//Bit	24
		u32 reserved5:7;						//Bits	25-31
	};
	u32 value;
}vmx_exit_ctrl,*vmx_exit_ctrl_p;

typedef union _vmx_entry_ctrl
{
	struct
	{
		u32 reserved0:2;						//Bits	0-1
		u32 load_debug_controls:1;				//Bit	2
		u32 reserved1:6;						//Bits	3-8
		u32 ia32e_mode_guest:1;					//Bit	9
		u32 entry_to_smm:1;						//Bit	10
		u32 deactivate_dual_monitor:1;			//Bit	11
		u32 reserved2:1;						//Bit	12
		u32 load_ia32_perf_global_ctrl:1;		//Bit	13
		u32 load_ia32_pat:1;					//Bit	14
		u32 load_ia32_efer:1;					//Bit	15
		u32 load_ia32_bndcfgs:1;				//Bit	16
		u32 conceal_vmx_from_ipt:1;				//Bit	17
		u32 reserved3:14;						//Bits	18-31
	};
	u32 value;
}vmx_entry_ctrl,*vmx_entry_ctrl_p;

typedef union _vmx_pinbased_ctrl_msr
{
	struct
	{
		vmx_pinbased_ctrl allowed0_settings;
		vmx_pinbased_ctrl allowed1_settings;
	};
	u64 value;
}vmx_pinbased_ctrl_msr,*vmx_pinbased_ctrl_msr_p;

typedef union _vmx_procbased_ctrl_msr
{
	struct
	{
		vmx_procbased_ctrl allowed0_settings;
		vmx_procbased_ctrl allowed1_settings;
	};
	u64 value;
}vmx_procbased_ctrl_msr,*vmx_procbased_ctrl_msr_p;

typedef union _vmx_procbased2_ctrl_msr
{
	struct
	{
		vmx_procbased2_ctrl allowed0_settings;
		vmx_procbased2_ctrl allowed1_settings;
	};
	u64 value;
}vmx_procbased2_ctrl_msr,*vmx_procbased2_ctrl_msr_p;

typedef union _vmx_exit_ctrl_msr
{
	struct
	{
		vmx_exit_ctrl allowed0_settings;
		vmx_exit_ctrl allowed1_settings;
	};
	u64 value;
}vmx_exit_ctrl_msr,*vmx_exit_ctrl_msr_p;

typedef union _vmx_entry_ctrl_msr
{
	struct
	{
		vmx_entry_ctrl allowed0_settings;
		vmx_entry_ctrl allowed1_settings;
	};
	u64 value;
}vmx_entry_ctrl_msr,*vmx_entry_ctrl_msr_p;

typedef union _vmx_misc_msr
{
	struct
	{
		u64 vmx_tsc_rate:5;				//Bits	0-4
		u64	auto_set_lma:1;				//Bit	5
		u64 support_hlt_state:1;		//Bit	6
		u64 support_shudown_state:1;	//Bit	7
		u64 support_wait_sipi_state:1;	//Bit	8
		u64 reserved0:5;				//Bits	9-13
		u64 allow_ipt_in_vmx:1;			//Bit	14
		u64 allow_read_smbase:1;		//Bit	15
		u64 max_cr3_target_value:9;		//Bits	16-24
		u64 max_auto_load_msr:3;		//Bits	25-27
		u64 vmxoff_unblock_smi:1;		//Bit	28
		u64 vmwrite_to_readonly:1;		//Bit	29
		u64 allow_len0_injection:1;		//Bit	30
		u64 reserved1:1;				//Bit	31
		u64 mseg_revision_id:32;		//Bits	32-63
	};
	u64 value;
}vmx_misc_msr,*vmx_misc_msr_p;

typedef union _vmx_ept_vpid_cap_msr
{
	struct
	{
		u64 exec_only_trans:1;			//Bit	0
		u64 reserved0:5;				//Bits	1-5
		u64 len4_page_walk:1;			//Bit	6
		u64 reserved1:1;				//Bit	7
		u64 uncacheable_ept:1;			//Bit	8
		u64 reserved2:5;				//Bits	9-13
		u64 writeback_ept:1;			//Bit	14
		u64 reserved3:1;				//Bit	15
		u64 map_2mb_page:1;				//Bit	16
		u64 map_1gb_page:1;				//Bit	17
		u64 reserved4:2;				//Bits	18-19
		u64 support_invept:1;			//Bit	20
		u64 support_dirty_flag:1;		//Bit	21
		u64 advanced_ept_exit_info:1;	//Bit	22
		u64 reserved5:2;				//Bits	23-24
		u64 support_sc_invept:1;		//Bit	25
		u64 support_ac_invept:1;		//Bit	26
		u64 reserved6:5;				//Bits	27-31
		u64 support_invvpid:1;			//Bit	32
		u64 reserved7:7;				//Bits	33-39
		u64 support_ia_invvpid:1;		//Bit	40
		u64 support_sc_invvpid:1;		//Bit	41
		u64 support_ac_invvpid:1;		//Bit	42
		u64 support_scrg_invvpid:1;		//Bit	43
		u64 reserved8:20;				//Bits	44-63
	};
	u64 value;
}vmx_ept_vpid_cap_msr,*vmx_ept_vpid_cap_msr_p;

bool load_driver();
void unload_driver();
u64 vtc_rdmsr(u32 index);

void inline vtc_cpuid(u32 ta,u32 tc,u32* a,u32* b,u32* c,u32* d)
{
	u32 id[4];
	__cpuidex(id,ta,tc);
	if(a)*a=id[0];
	if(b)*b=id[1];
	if(c)*c=id[2];
	if(d)*d=id[3];
}