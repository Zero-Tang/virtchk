#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "virtchk.h"

u8 vtc_confirm_cpu()
{
	char vs_str[13];
	vtc_cpuid(0,0,null,(u32*)&vs_str[0],(u32*)&vs_str[8],(u32*)&vs_str[4]);
	vs_str[12]=0;
	if(strcmp(vs_str,"GenuineIntel")==0)
		return intel_processor;
	else if(strcmp(vs_str,"AuthenticAMD")==0)
		return amd_processor;
	else
		return unknown_processor;
}

void vtc_print_vmx_info()
{
	char vs_str[13];
	u32 max;
	vtc_cpuid(0,0,&max,(u32*)&vs_str[0],(u32*)&vs_str[8],(u32*)&vs_str[4]);
	if(strcmp(vs_str,"GenuineIntel")==0 && max>=1)
	{
		u32 feat_info;
		printf("This processor is manufactured by Intel Corporation!\n");
		vtc_cpuid(1,0,null,null,&feat_info,null);
		if(vtc_bt(&feat_info,5))
		{
			printf("To detect Intel VT-x capability, virtchk needs to load driver!\n");
			printf("You have to make sure virtchk is started with administrator privilege!\n");
			system("pause");
			if(load_driver())
			{
				vmx_basic_msr vtbas;
				vmx_pinbased_ctrl_msr pin_ctrl;
				vmx_procbased_ctrl_msr proc_ctrl;
				vmx_exit_ctrl_msr exit_ctrl;
				vmx_entry_ctrl_msr entry_ctrl;
				vmx_misc_msr vtmisc;
				printf("This processor supports Intel VT-x!\n");
				vtbas.value=vtc_rdmsr(ia32_vmx_basic);
				printf("VMCS Revision ID of This Processor: %d\n",vtbas.revision_id);
				if(vtbas.memory_type==6)
					printf("This processor recommends Write-Back Memory for VMCS!\n");
				else
					printf("This processor recommends Non-Cached Memory for VMCS!\n");
				if(vtbas.use_true_msr)
				{
					pin_ctrl.value=vtc_rdmsr(ia32_vmx_true_pinbased_ctrl);
					proc_ctrl.value=vtc_rdmsr(ia32_vmx_true_procbased_ctrl);
					exit_ctrl.value=vtc_rdmsr(ia32_vmx_true_exit_ctrl);
					entry_ctrl.value=vtc_rdmsr(ia32_vmx_true_entry_ctrl);
				}
				else
				{
					pin_ctrl.value=vtc_rdmsr(ia32_vmx_pinbased_ctrl);
					proc_ctrl.value=vtc_rdmsr(ia32_vmx_procbased_ctrl);
					exit_ctrl.value=vtc_rdmsr(ia32_vmx_exit_ctrl);
					entry_ctrl.value=vtc_rdmsr(ia32_vmx_entry_ctrl);
				}
				if(proc_ctrl.allowed1_settings.activate_secondary_control)
				{
					vmx_procbased2_ctrl_msr proc2_ctrl;
					proc2_ctrl.value=vtc_rdmsr(ia32_vmx_procbased2_ctrl);
					if(proc2_ctrl.allowed1_settings.enable_ept)
					{
						vmx_ept_vpid_cap_msr ept_cap;
						printf("Intel EPT is supported!\n");
						ept_cap.value=vtc_rdmsr(ia32_vmx_ept_vpid_cap);
						if(ept_cap.map_2mb_page)
							printf("Intel EPT 2MB-Paging is supported!\n");
						if(ept_cap.map_1gb_page)
							printf("Intel EPT 1GB-Paging is supported!\n");
						if(ept_cap.uncacheable_ept)
							printf("Non-Cached EPT Paging Structure is supported!\n");
						if(ept_cap.writeback_ept)
							printf("Write-Back EPT Paging Structure is supported!\n");
						if(ept_cap.exec_only_trans)
							printf("Execute-Only Translation is supported!\n");
					}
					if(proc2_ctrl.allowed1_settings.vmcs_shadowing)
						printf("Hardware-Accelerated Nesting Virtualization is supported!\n");
					if(proc2_ctrl.allowed1_settings.enable_encls_exiting)
						printf("Intel SGX Virtualization is supported!\n");
				}
				if(proc_ctrl.allowed1_settings.use_msr_bitmap)
					printf("MSR-Bitmap is supported!\n");
				if(pin_ctrl.allowed1_settings.external_interrupt)
					printf("External-Interrupt Interception is supported!\n");
				vtmisc.value=vtc_rdmsr(ia32_vmx_misc);
				printf("Maximum supported Auto-Load MSR Count: %d\n",(vtmisc.max_auto_load_msr+1)<<9);
				unload_driver();
			}
		}
	}
}

void vtc_print_svm_info()
{
	char vs_str[13];
	u32 max;
	vtc_cpuid(0x80000000,0,&max,(u32*)&vs_str[0],(u32*)&vs_str[8],(u32*)&vs_str[4]);
	if(strcmp(vs_str,"AuthenticAMD")==0 && max>=0x8000000A)
	{
		u32 feat_info;
		printf("This processor is manufactured by AMD Incorporated!\n");
		vtc_cpuid(0x80000001,0,null,&feat_info,null,null);
		if(vtc_bt(&feat_info,2))
		{
			u32 nasid,rev;
			svm_feat_info svi;
			printf("This processor supports AMD-V!\n");
			vtc_cpuid(0x8000000A,0,&rev,&nasid,null,(u32*)&svi);
			printf("SVM Revision Id: %d\n",rev);
			printf("Number of available ASID: %d\n",nasid);
			if(svi.npt)
				printf("Nested Paging (NPT/RVI) is supported!\n");
			if(svi.lbr_virt)
				printf("LBR Virtualization is supported!\n");
			if(svi.svm_lock)
				printf("SVM Lock is supported!\n");
			if(svi.next_rip)
				printf("Next-RIP Saving is supported!\n");
			if(svi.tsc_rate)
				printf("TSC Ratio Control is supported!\n");
			if(svi.vmcb_clean)
				printf("VMCB Clean-Bit is supported!\n");
			if(svi.flush_asid)
				printf("Flush-By-ASID is supported!\n");
			if(svi.decoder)
				printf("Decode Assist is supported!\n");
			if(svi.pause_filter)
				printf("Pause Filter is supported!\n");
			if(svi.pause_filter_threshold)
				printf("Pause Filter Threshold is supported!\n");
			if(svi.avic)
				printf("Advanced Virtual Interrupt Controller is supported!\n");
			if(svi.svm_ins_virt)
				printf("Hardware-Accelerated SVM Instruction Virtualization is supported!\n");
			if(svi.svm_gif_virt)
				printf("Hardware-Accelerated SVM Global Interrupt Flag Virtualization is supported!\n");
		}
	}
}

void vtc_main()
{
	u8 cpu_manuf=vtc_confirm_cpu();
	switch(cpu_manuf)
	{
		case intel_processor:
		{
			vtc_print_vmx_info();
			break;
		}
		case amd_processor:
		{
			vtc_print_svm_info();
			break;
		}
		default:
		{
			printf("Your processor is unknown!\n");
			break;
		}
	}
	system("pause");
}