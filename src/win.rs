// Windows runtime
#[cfg(target_arch="x86_64")]
use core::arch::x86_64::_bittest;
#[cfg(target_arch="x86")]
use core::arch::x86::_bittest;
use core::{ffi::{c_void, CStr}, fmt, mem::{offset_of, transmute}, ptr::{null, null_mut}, slice, sync::atomic::{AtomicPtr,Ordering}};

use static_collections::{ffi::wstring::StaticWString, string::StaticString};
use windows_sys::{core::HRESULT, Win32::{Foundation::{HANDLE, NTSTATUS, STATUS_SUCCESS, S_OK}, System::{Console::WriteConsoleW, Diagnostics::Debug::*, Hypervisor::{WHvCapabilityCodeExceptionExitBitmap, WHvCapabilityCodeExtendedVmExits, WHvCapabilityCodeFeatures, WHvCapabilityCodeHypervisorPresent, WHvCapabilityCodeX64MsrExitBitmap, WHV_CAPABILITY, WHV_CAPABILITY_CODE}, LibraryLoader::LoadLibraryA, SystemServices::{IMAGE_DOS_HEADER, IMAGE_DOS_SIGNATURE, IMAGE_EXPORT_DIRECTORY, IMAGE_NT_SIGNATURE}, Threading::PEB, WindowsProgramming::LDR_DATA_TABLE_ENTRY}}};

use crate::{main, println};

static mut STDOUT_HANDLE:HANDLE=null_mut();
static GLOBAL_PEB:Peb=Peb(AtomicPtr::new(null_mut()));

struct Peb(AtomicPtr<PEB>);

impl Peb
{
	fn set_base(&self,value:*mut PEB)
	{
		self.0.store(value,Ordering::Relaxed);
	}

	#[allow(dead_code)]
	fn get_module_handle(&self,name:&str)->ModuleHandle
	{
		let ldr=unsafe{&mut *(*self.0.load(Ordering::Relaxed)).Ldr};
		let anchor=&raw mut ldr.InMemoryOrderModuleList;
		let mut cursor=ldr.InMemoryOrderModuleList.Flink;
		while cursor!=anchor
		{
			let cur_ldr:&mut LDR_DATA_TABLE_ENTRY=unsafe{&mut *cursor.byte_sub(offset_of!(LDR_DATA_TABLE_ENTRY,InMemoryOrderLinks)).cast()};
			let mut s:StaticString<128>=StaticString::new();
			let dll_name:&[u16]=unsafe{slice::from_raw_parts(cur_ldr.Reserved5[0].cast(),(*cur_ldr.Reserved4.as_ptr().cast::<u16>() as usize)>>1)};
			for &c in dll_name
			{
				let _=s.push(unsafe{char::from_u32_unchecked(c as u32)});
			}
			println!("Current DLL Name: {s}");
			if s==name
			{
				return ModuleHandle(cur_ldr.DllBase.cast());
			}
			cursor=cur_ldr.InMemoryOrderLinks.Flink;
		}
		ModuleHandle(null_mut())
	}
}

struct ModuleHandle(*mut IMAGE_DOS_HEADER);

impl ModuleHandle
{
	fn get_proc_address(&self,name:&CStr)->*mut c_void
	{
		let dos_head=unsafe{&*self.0};
		if dos_head.e_magic==IMAGE_DOS_SIGNATURE
		{
			#[cfg(target_pointer_width="64")]
			let nt_head:&IMAGE_NT_HEADERS64=unsafe{&*self.0.byte_add(dos_head.e_lfanew as usize).cast()};
			#[cfg(target_pointer_width="32")]
			let nt_head:&IMAGE_NT_HEADERS32=unsafe{&*self.0.byte_add(dos_head.e_lfanew as usize).cast()};
			if nt_head.Signature==IMAGE_NT_SIGNATURE
			{
				let expdir_offset=nt_head.OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_EXPORT as usize].VirtualAddress as usize;
				if expdir_offset!=0
				{
					let exp_dir:&IMAGE_EXPORT_DIRECTORY=unsafe{&*self.0.byte_add(expdir_offset).cast()};
					let name_rvas:&[u32]=unsafe{slice::from_raw_parts(self.0.byte_add(exp_dir.AddressOfNames as usize).cast(),exp_dir.NumberOfNames as usize)};
					let func_rvas:&[u32]=unsafe{slice::from_raw_parts(self.0.byte_add(exp_dir.AddressOfFunctions as usize).cast(),exp_dir.NumberOfFunctions as usize)};
					let ord_rvas:&[u16]=unsafe{slice::from_raw_parts(self.0.byte_add(exp_dir.AddressOfNameOrdinals as usize).cast(),exp_dir.NumberOfNames as usize)};
					let mut lo:isize=0;
					let mut hi:isize=(exp_dir.NumberOfNames as isize)-1;
					while hi>=lo
					{
						use core::cmp::Ordering;
						let mid=(hi+lo)>>1;
						let name_offset=name_rvas[mid as usize] as usize;
						let func_name=unsafe{CStr::from_ptr(self.0.byte_add(name_offset).cast())};
						match func_name.cmp(name)
						{
							Ordering::Equal=>
							{
								let ordinal=ord_rvas[mid as usize] as usize;
								return unsafe{self.0.byte_add(func_rvas[ordinal] as usize).cast()};
							}
							Ordering::Greater=>hi=mid-1,
							Ordering::Less=>lo=mid+1
						}
					}
				}
			}
		}
		null_mut()
	}
}

pub fn internal_print(args:fmt::Arguments)
{
	// Use on-stack printer.
	let mut w:StaticWString<1024>=StaticWString::new();
	if fmt::write(&mut w,args).is_ok()
	{
		unsafe
		{
			WriteConsoleW(STDOUT_HANDLE,w.as_ptr(),w.len() as u32,null_mut(),null());
		}
	}
}

#[macro_export]
macro_rules! print
{
	($($arg:tt)*)=>
	{
		$crate::win::internal_print(format_args!($($arg)*))
	};
}

static WHPX_CAPABILITY_FEATURE_NAMES:[Option<&'static str>;64]=
{
	let mut names:[Option<&'static str>;64]=[None;64];
	names[0]=Some("Partial Unmap");
	names[1]=Some("Local APIC Emulation");
	names[2]=Some("XSAVE");
	names[3]=Some("Dirty Page Tracking");
	names[4]=Some("Speculation Control");
	names[5]=Some("APIC Remote Read");
	names[6]=Some("Idle Suspend");
	names[7]=Some("Virtual PCI Device");
	names[8]=Some("IOMMU");
	names[9]=Some("Virtual CPU Hot Add Remove");
	names
};

static WHPX_EXTENDED_VMEXIT_NAMES:[Option<&'static str>;64]=
{
	let mut names:[Option<&'static str>;64]=[None;64];
	names[0]=Some("X64CpuidExit");
	names[1]=Some("X64MsrExit");
	names[2]=Some("X64ExceptionExit");
	names[3]=Some("X64RdtscExit");
	names[4]=Some("X64ApicSmiExitTrap");
	names[5]=Some("HypercallExit");
	names[6]=Some("X64ApicInitSipiExitTrap");
	names[7]=Some("X64ApicWriteLint0ExitTrap");
	names[8]=Some("X64ApicWriteLint1ExitTrap");
	names[9]=Some("X64ApicWriteSvrExitTrap");
	names[10]=Some("Unknown SynIC Connection Exit");
	names[11]=Some("Retarget Unknown vPCI Device Exit");
	names[12]=Some("X64ApicWriteLdrExitTrap");
	names[13]=Some("X64ApicWriteDfrExitTrap");
	names[14]=Some("GpaAccessFaultExit");
	names
};

static WHPX_EXCEPTION_BITMAP_NAMES:[Option<&'static str>;64]=
{
	let mut names:[Option<&'static str>;64]=[None;64];
	names[0]=Some("#DE: Divide-Error Fault");
	names[1]=Some("#DB: Debug Trap or Fault");
	names[3]=Some("#BP: Breakpoint Trap");
	names[4]=Some("#OF: Overflow Trap");
	names[5]=Some("#BR: Bound Range Fault");
	names[6]=Some("#UD: Invalid Opcode Fault");
	names[7]=Some("#NM: Device Not Available Fault");
	names[8]=Some("#DF: Double Fault Abort");
	names[10]=Some("#TS: Invalid Task Segment State Fault");
	names[11]=Some("#NP: Segment Not Present Fault");
	names[12]=Some("#SS: Stack Fault");
	names[13]=Some("#GP: General Protection Fault");
	names[14]=Some("#PF: Page Fault");
	names[16]=Some("#MF: Floating-Point Error Fault");
	names[17]=Some("#AC: Alignment-Check Fault");
	names[18]=Some("#MC: Machine-Check Abort");
	names[19]=Some("#XF: SIMD Floating-Point Fault");
	names[20]=Some("#VE: Virtualization Exception Fault");
	names[21]=Some("#CP: Control-Protection Fault");
	names[28]=Some("#HV: Hypervisor Injection Exception");
	names[29]=Some("#VC: VMM Communication Exception");
	names[30]=Some("#SX: Security Exception");
	names
};

static WHPX_MSR_BITMAP_NAMES:[Option<&'static str>;64]=
{
	let mut names:[Option<&'static str>;64]=[None;64];
	names[0]=Some("Unhandled MSRs");
	names[1]=Some("TSC MSR-Write");
	names[2]=Some("TSC MSR-Read");
	names[3]=Some("APIC-Base MSR-Write");
	names[4]=Some("Misc-Enable MSR-Read");
	names[5]=Some("MC-Update Patch-Level MSR-Read");
	names
};

type WhpxGetCapabilityFn=unsafe extern "system" fn (WHV_CAPABILITY_CODE,*mut c_void,u32,*mut u32)->HRESULT;

fn print_cap_names(whpx_get_cap:WhpxGetCapabilityFn,feat_code:WHV_CAPABILITY_CODE,prefix:&str,names:&[Option<&'static str>])
{
	let mut feat=WHV_CAPABILITY::default();
	let hr:HRESULT=unsafe{whpx_get_cap(feat_code,(&raw mut feat).cast(),size_of::<WHV_CAPABILITY>() as u32,null_mut())};
	if hr==S_OK
	{
		for i in 0..(names.len() as i32)
		{
			let present=unsafe{_bittest((&raw const feat).cast(),i)!=0};
			if present
			{
				match names[i as usize]
				{
					Some(name)=>println!("WHPX {prefix} {name} is supported!"),
					None=>println!("WHPX Reserved {prefix} #{i} is supported!")
				}
			}
		}
	}
	else
	{
		println!("WHvGetCapability failed to query {prefix}!");
	}
}

fn check_whpx()
{
	// Windows Hypervisor Platform is available only in Windows 10 1809 and later.
	let p:*mut IMAGE_DOS_HEADER=unsafe{LoadLibraryA(c"WinHvPlatform.dll".as_ptr().cast()).cast()};
	if p.is_null()
	{
		println!("Cannot load WinHvPlatform.dll! Your system does not support Windows Hypervisor Platform!");
	}
	else
	{
		let whpx_mod=ModuleHandle(p);
		let whpx_get_cap:WhpxGetCapabilityFn=unsafe{transmute(whpx_mod.get_proc_address(c"WHvGetCapability"))};
		let mut cap=WHV_CAPABILITY::default();
		let hv_present=unsafe
		{
			whpx_get_cap(WHvCapabilityCodeHypervisorPresent,(&raw mut cap).cast(),size_of::<WHV_CAPABILITY>() as u32,null_mut());
			cap.HypervisorPresent!=0
		};
		println!("Windows Hypervisor Platform API is {}sent!",if hv_present {"pre"} else {"ab"});
		if hv_present
		{
			print_cap_names(whpx_get_cap,WHvCapabilityCodeFeatures,"Feature",&WHPX_CAPABILITY_FEATURE_NAMES);
			print_cap_names(whpx_get_cap,WHvCapabilityCodeExtendedVmExits,"Extended VM-Exit",&WHPX_EXTENDED_VMEXIT_NAMES);
			print_cap_names(whpx_get_cap,WHvCapabilityCodeExceptionExitBitmap,"Exception Intercept",&WHPX_EXCEPTION_BITMAP_NAMES);
			print_cap_names(whpx_get_cap,WHvCapabilityCodeX64MsrExitBitmap,"MSR Intercept",&WHPX_MSR_BITMAP_NAMES);
		}
	}
}

#[unsafe(no_mangle)] extern "C" fn msvc_start(peb:*mut PEB)->NTSTATUS
{
	unsafe
	{
		// Obtain stdout handle from PEB. Just to avoid GetStdHandle call.
		STDOUT_HANDLE=(*(*peb).ProcessParameters).Reserved2[3];
	}
	GLOBAL_PEB.set_base(peb);
	main();
	if cfg!(target_arch="x86_64")
	{
		check_whpx();
	}
	else
	{
		println!("32-bit Windows Application cannot use Windows Hypervisor Platform!");
	}
	STATUS_SUCCESS
}

#[unsafe(no_mangle)] extern "C" fn __CxxFrameHandler3()
{
	
}