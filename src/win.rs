// Windows runtime
use core::{fmt, ptr::{null, null_mut}};

use static_collections::{string::StaticString, vec::StaticVec};
use windows_sys::Win32::{Foundation::{HANDLE, NTSTATUS, STATUS_SUCCESS}, System::{Console::WriteConsoleW, Threading::PEB}};

use crate::main;

static mut STDOUT_HANDLE:HANDLE=null_mut();

pub fn internal_print(args:fmt::Arguments)
{
	// Use on-stack printer.
	let mut w:StaticString<1024>=StaticString::new();
	if fmt::write(&mut w,args).is_ok()
	{
		// Convert UTF-8 to UTF-16.
		let mut s:StaticVec<1024,u16>=StaticVec::new();
		// Do not use `.collect()` method so that we may avoid heap allocation.
		for x in w.as_str().encode_utf16()
		{
			s.push(x);
		}
		unsafe
		{
			WriteConsoleW(STDOUT_HANDLE,s.as_ptr(),s.len() as u32,null_mut(),null());
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

#[unsafe(no_mangle)] extern "C" fn msvc_start(peb:*const PEB)->NTSTATUS
{
	unsafe
	{
		// Obtain stdout handle from PEB. Just to avoid GetStdHandle call.
		STDOUT_HANDLE=(*(*peb).ProcessParameters).Reserved2[3];
	}
	main();
	STATUS_SUCCESS
}

#[unsafe(no_mangle)] extern "C" fn __CxxFrameHandler3()
{
	
}