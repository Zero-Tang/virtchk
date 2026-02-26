#![no_std]
#![no_main]

use core::arch::{asm,naked_asm};
#[cfg(target_arch="x86_64")]
use core::arch::x86_64::{__cpuid, _bittest};
#[cfg(target_arch="x86")]
use core::arch::x86::{__cpuid, _bittest};

use static_collections::string::StaticString;

#[cfg(windows)] mod win;
#[cfg(unix)] mod linux;
#[cfg(target_os="uefi")] mod uefi;

#[macro_export]
macro_rules! println
{
	()=>
	{
		$crate::print!("\n")
	};
	($($arg:tt)*)=>
	{
		$crate::print!("{}\n",format_args!($($arg)*))
	};
}

fn get_vendor_string()->StaticString<12>
{
	let r=__cpuid(0);
	let mut vstr:StaticString<12>=StaticString::new();
	macro_rules! push_u32
	{
		($reg:tt) =>
		{
			{
				let s=r.$reg.to_le_bytes();
				let _=vstr.push_str(unsafe{str::from_utf8_unchecked(&s)});
			}
		};
	}
	push_u32!(ebx);
	push_u32!(edx);
	push_u32!(ecx);
	vstr.truncate_to_nul();
	vstr
}

fn get_processor_brand()->StaticString<48>
{
	let mut pstr:StaticString<48>=StaticString::new();
	for a in 0x80000002..=0x80000004
	{
		let r=__cpuid(a);
		macro_rules! push_u32
		{
			($reg:tt) =>
			{
				{
					let s=r.$reg.to_le_bytes();
					let _=pstr.push_str(unsafe{str::from_utf8_unchecked(&s)});
				}
			};
		}
		push_u32!(eax);
		push_u32!(ebx);
		push_u32!(ecx);
		push_u32!(edx);
	}
	pstr.truncate_to_nul();
	pstr
}

fn has_vmx_and_hv()->(bool,bool)
{
	let r=__cpuid(1);
	let vmx=unsafe{_bittest((&raw const r.ecx).cast(),5)}!=0;
	let hv=unsafe{_bittest((&raw const r.ecx).cast(),31)}!=0;
	(vmx,hv)
}

fn has_svm()->bool
{
	let r=__cpuid(0x80000001);
	unsafe
	{
		_bittest((&raw const r.ecx).cast(),2)!=0
	}
}

fn get_hypervisor_vendor()->(u32,StaticString<12>)
{
	let mut vstr:StaticString<12>=StaticString::new();
	let r=__cpuid(0x40000000);
	macro_rules! push_u32
	{
		($reg:tt) =>
		{
			{
				let s=r.$reg.to_le_bytes();
				let _=vstr.push_str(unsafe{str::from_utf8_unchecked(&s)});
			}
		};
	}
	push_u32!(ebx);
	push_u32!(ecx);
	push_u32!(edx);
	vstr.truncate_to_nul();
	(r.eax,vstr)
}

#[unsafe(naked)] extern "win64" fn rdmsr(index:u32)->u64
{
	naked_asm!
	(
		"rdmsr",
		"shl rdx,32",
		"or rax,rdx",
		"ret"
	);
}

static SVM_EDX_FEATURE_NAMES:[Option<&'static str>;32]=
[
	Some("Nested Paging"),
	Some("LBR Virtualization"),
	Some("SVM Lock"),
	Some("Next RIP Saving"),
	Some("TSC Rate MSR"),
	Some("VMCB Clean Bits"),
	Some("Flush by ASID"),
	Some("Decode Assists"),
	Some("PMC Virtualization"),
	None,
	Some("Pause Filter"),
	None,
	Some("Pause Filter Threshold"),
	Some("AVIC (Advanced Virtual Interrupt Controller)"),
	None,
	Some("VMSAVE/VMLOAD Virtualization"),
	Some("Virtual GIF"),
	Some("Guest Mode Execution Trap"),
	Some("x2AVIC (x2APIC Virtualization)"),
	Some("SVM Supervisor Shadow Stack Restrictions"),
	Some("SPEC_CTRL Virtualization"),
	Some("Read-Only Guest Page Tables"),
	None,
	Some("Host MCE Override"),
	Some("INVLPGB/TLBSYNC Support and Interception"),
	Some("NMI Virtualization"),
	Some("IBS Virtualization"),
	Some("Extended Interrupt LVT AVIC Access Changes"),
	Some("Guest VMCB Address Check"),
	Some("Bus Lock Threshold"),
	Some("Idle HLT Intercept"),
	None
];

static VT_PIN_BASED_FEATURES:[Option<&'static str>;32]=
{
	let mut a=[None;32];
	a[0]=Some("External-Interrupt Exiting");
	a[3]=Some("NMI Exiting");
	a[5]=Some("Virtual NMIs");
	a[6]=Some("Activate VMX-preemption Timer");
	a[7]=Some("Process Posted Interrupts");
	a
};

static VT_PRIMARY_PROCESSOR_FEATURES:[Option<&'static str>;32]=
{
	let mut a=[None;32];
	a[2]=Some("Interrupt-Window Exiting");
	a[3]=Some("Use TSC-Offsetting");
	a[7]=Some("HLT Exiting");
	a[9]=Some("INVLPG Exiting");
	a[10]=Some("MWAIT Exiting");
	a[11]=Some("RDPMC Exiting");
	a[12]=Some("RDTSC Exiting");
	a[15]=Some("CR3-Load Exiting");
	a[16]=Some("CR3-Store Exiting");
	a[17]=Some("Activate Tertiary Controls");
	a[19]=Some("CR8-Load Exiting");
	a[20]=Some("CR8-Store Exiting");
	a[21]=Some("Use TPR Shadow");
	a[22]=Some("NMI-Window Exiting");
	a[23]=Some("MOV-DR Exiting");
	a[24]=Some("Unconditional I/O Exiting");
	a[25]=Some("Use I/O Bitmaps");
	a[27]=Some("Monitor Trap Flag");
	a[28]=Some("Use MSR Bitmap");
	a[29]=Some("MONITOR Exiting");
	a[30]=Some("PAUSE Exiting");
	a[31]=Some("Activate Secondary Controls");
	a
};

static VT_SECONDARY_PROCESSOR_FEATURES:[Option<&'static str>;32]=
[
	Some("Virtualize APIC Accesses"),
	Some("Enable EPT"),
	Some("Descriptor-Table Exiting"),
	Some("Enable RDTSCP"),
	Some("Virtualize x2APIC Mode"),
	Some("Enable VPID"),
	Some("WBINVD Exiting"),
	Some("Unrestricted Guest"),
	Some("APIC-Register Virtualization"),
	Some("Virtual-Interrupt Delivery"),
	Some("PAUSE-Loop Exiting"),
	Some("RDRAND Exiting"),
	Some("Enable INVPCID"),
	Some("Enable VMFUNC"),
	Some("VMCS Shadowing"),
	Some("ENCLS Exiting"),
	Some("RDSEED Exiting"),
	Some("Enable PML"),
	Some("Convert EPT-Violation to #VE"),
	Some("Conceal VMX from PT"),
	Some("Enable XSAVES/XRSTORS"),
	Some("PASID Translation"),
	Some("EPT Mode-based Execution Control"),
	Some("EPT Sub-Page Write Permissions"),
	Some("PT Use GPA"),
	Some("Use TSC Scaling"),
	Some("Enable UMWAIT/UMONITOR/TPAUSE"),
	Some("Enable PCONFIG"),
	None,
	None,
	Some("VMM Bus-Lock Detection"),
	Some("Instruction Timeout")
];

static VT_TERTIARY_PROCESSOR_FEATURES:[Option<&'static str>;64]=
{
	let mut a=[None;64];
	a[0]=Some("LOADIWKEY Exiting");
	a[1]=Some("Enable HLAT");
	a[2]=Some("EPT Paging-Write Control");
	a[3]=Some("Guest-Paging Verification");
	a[4]=Some("IPI Virtualization");
	a[5]=Some("SEAM GPA Width");
	a[6]=Some("Enable MSR-List");
	a[7]=Some("Virtualize IA32_SPEC_CTRL");
	a[8]=Some("Enable PBNDKB");
	a
};

// Use naked assembly to avoid optimization.
#[cfg(target_arch="x86_64")]
#[unsafe(naked)] extern "win64" fn cpuid_latency()->u64
{
	naked_asm!
	(
		// rdtscp would wait until all previous instructions retire,
		// then obtains the Time-Stamp Counter.
		"rdtscp",
		"mov r8d,eax",
		"mov r9d,edx",
		"cpuid",
		"rdtscp",
		"shl rdx,32",
		"shl r9,32",
		"or rax,rdx",
		"or r8,r9",
		"sub rax,r8",
		"ret"
	)
}

#[cfg(target_arch="x86")]
#[unsafe(naked)] extern "stdcall" fn cpuid_latency()->u64
{
	naked_asm!
	(
		// rdtscp would wait until all previous instructions retire,
		// then obtains the Time-Stamp Counter.
		"rdtscp",
		"push edx",
		"push eax",
		"cpuid",
		"rdtscp",
		"pop ecx",
		"sub eax,ecx",
		"pop ecx",
		"sbb edx,ecx",
		"ret"
	)
}

const CPUID_TEST_COUNT:u64=1000000;

#[inline(never)]
fn test_cpuid_latency()
{
	let mut sum=0u64;
	let mut min=u64::MAX;
	let mut max=0u64;
	let mut i:u64=0;
	while i<CPUID_TEST_COUNT
	{
		let t=cpuid_latency();
		sum+=t;
		if t>max
		{
			max=t;
		}
		if t<min
		{
			min=t;
		}
		i+=1;
	}
	let avg=sum/CPUID_TEST_COUNT;
	println!("CPUID Average TSC: {avg}, Minimum TSC: {min}, Maximum TSC: {max}");
}

fn main()
{
	let vstr=get_vendor_string();
	let pstr=get_processor_brand();
	println!("CPU Vendor: {}",vstr);
	println!("CPU Brand: {}",pstr);
	let (vmx,hv)=has_vmx_and_hv();
	let svm=has_svm();
	if vmx
	{
		println!("Intel VT-x is supported!");
		let cs:u16;
		unsafe
		{
			asm!
			(
				"mov {:x},cs",
				out(reg) cs
			);
		}
		if (cs&3)==0
		{
			println!("We're in kernel-mode! Enumerating VMX capability reporting MSRs...");
			let vt_basic=rdmsr(0x480);
			let (pin,proc1)=if (vt_basic&(1<<55))!=0
			{
				(rdmsr(0x48D),rdmsr(0x48E))
			}
			else
			{
				(rdmsr(0x481),rdmsr(0x482))
			};
			for i in 0..32
			{
				let supported=unsafe{_bittest((&raw const pin).cast(),i+32)}!=0;
				if let Some(name)=VT_PIN_BASED_FEATURES[i as usize]
				{
					println!("{name} is {}supported!",if supported {""} else {"un"});
				}
			}
			for i in 0..32
			{
				let supported=unsafe{_bittest((&raw const proc1).cast(),i+32)}!=0;
				if let Some(name)=VT_PRIMARY_PROCESSOR_FEATURES[i as usize]
				{
					println!("{name} is {}supported!",if supported {""} else {"un"});
				}
			}
			if (proc1&(1<<63))!=0
			{
				let proc2=rdmsr(0x48B);
				for i in 0..32
				{
					let supported=unsafe{_bittest((&raw const proc2).cast(),i+32)}!=0;
					if let Some(name)=VT_SECONDARY_PROCESSOR_FEATURES[i as usize]
					{
						println!("{name} is {}supported!",if supported {""} else {"un"});
					}
				}
			}
			if (proc1&(1<<49))!=0
			{
				let proc3=rdmsr(0x492);
				for i in 0..64
				{
					let supported=unsafe{_bittest((&raw const proc3).cast(),i)}!=0;
					if let Some(name)=VT_TERTIARY_PROCESSOR_FEATURES[i as usize]
					{
						println!("{name} is {}supported!",if supported {""} else {"un"});
					}
				}
			}
		}
	}
	else if svm
	{
		println!("AMD-V is supported!");
		let r=__cpuid(0x8000000A);
		println!("SVM Revision Number: {}",r.eax&0xFF);
		println!("Number of available ASIDs: {}",r.ebx);
		for i in 0..32
		{
			if unsafe{_bittest((&raw const r.edx).cast(),i)}!=0
			{
				match SVM_EDX_FEATURE_NAMES[i as usize]
				{
					Some(name)=>println!("{name} is supported!"),
					None=>println!("Reserved feature #{i} is supported!")
				}
			}
		}
	}
	else
	{
		println!("No hardware-accelerated virtualization technology is supported!");
	}
	if hv
	{
		let (max_hv_leaf,hv_vstr)=get_hypervisor_vendor();
		println!("Hypervisor is detected! Maximum Leaf: 0x{max_hv_leaf:X}");
		println!("Hypervisor Vendor: {}",hv_vstr);
	}
	test_cpuid_latency();
}

#[cfg(not(test))]
mod panicking
{
	use core::panic::PanicInfo;

	#[panic_handler] fn panic(info:&PanicInfo)->!
	{
		println!("{info}");
		loop{}
	}
}