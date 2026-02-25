// UEFI runtime

use core::{fmt, ptr::null_mut, slice, sync::atomic::{AtomicPtr, Ordering}};

use r_efi::{efi::{BootServices, Handle, Status, SystemTable}, protocols::{simple_text_input::{self, InputKey}, simple_text_output}};
use static_collections::ffi::wstring::StaticWString;

use crate::{main, println};

static STDIN_PROTOCOL:AtomicPtr<simple_text_input::Protocol>=AtomicPtr::new(null_mut());
static STDOUT_PROTOCOL:AtomicPtr<simple_text_output::Protocol>=AtomicPtr::new(null_mut());
pub static BS_TABLE:AtomicPtr<BootServices>=AtomicPtr::new(null_mut());

pub fn internal_print(args:fmt::Arguments)
{
	let mut w:StaticWString<1024>=StaticWString::new();
	if fmt::write(&mut w,args).is_ok()
	{
		// Implicit CR for every LF because VGA console won't do that for us.
		let mut i:usize=0;
		while i<w.len()
		{
			if w[i]==b'\n' as u16
			{
				w.insert_char(i,'\r');
				i+=1;
			}
			i+=1;
		}
		// In Rust, strings are not terminated with 0.
		// Append null-terminator.
		if w.len()==w.capacity()
		{
			let end=w.len()-1;
			w[end-1]=0;
		}
		else
		{
			w.push_char('\0');
		}
		// Perform the output.
		let stdout_ptr=STDOUT_PROTOCOL.load(Ordering::Relaxed);
		unsafe
		{
			let stdout=&*stdout_ptr;
			(stdout.output_string)(stdout_ptr,w.as_mut_ptr());
		}
	}
}

#[macro_export]
macro_rules! print
{
	($($arg:tt)*)=>
	{
		$crate::uefi::internal_print(format_args!($($arg)*))
	};
}

fn block_until_keystroke(unicode:u16)
{
	let bs=unsafe{&*BS_TABLE.load(Ordering::Relaxed)};
	let stdin=unsafe{&mut *STDIN_PROTOCOL.load(Ordering::Relaxed)};
	let mut incoming=InputKey::default();
	while incoming.unicode_char!=unicode
	{
		let mut fi=0;
		unsafe
		{
		(bs.wait_for_event)(1,&raw mut stdin.wait_for_key,&raw mut fi);
			(stdin.read_key_stroke)(stdin,&raw mut incoming);
		}
	}
}

#[unsafe(no_mangle)] extern "efiapi" fn uefi_start(_image_handle:Handle,system_table:*mut SystemTable)->Status
{
	unsafe
	{
		STDIN_PROTOCOL.store((*system_table).con_in,Ordering::Relaxed);
		STDOUT_PROTOCOL.store((*system_table).con_out,Ordering::Relaxed);
		BS_TABLE.store((*system_table).boot_services,Ordering::Relaxed);
	}
	main();
	println!("Press Enter key to continue...");
	block_until_keystroke(b'\r' as u16);
	Status::SUCCESS
}

// The static-collections crate requires some CRT functions.
#[unsafe(no_mangle)] extern "C" fn strnlen(string:*const u8,limit:usize)->usize
{
	let s:&[u8]=unsafe{slice::from_raw_parts(string,limit)};
	s.iter().position(|v| *v==0).unwrap_or(limit)
}