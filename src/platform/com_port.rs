
use std::ffi::CString;
use std::ptr;
use std::mem::{size_of, zeroed};

use winapi::um::{winnt, winreg};
use winapi::shared::{winerror::ERROR_SUCCESS, minwindef::FILETIME, minwindef::DWORD};

pub fn EnumerateAvailableComPorts()
{
	unsafe
	{
		let keyCStr = CString::new("HARDWARE\\DEVICEMAP\\SERIALCOMM").unwrap(); //You'll find this under HKEY_LOCAL_MACHINE in regedit
		let mut regHandle = ptr::null_mut();
		
		let openResult = winreg::RegOpenKeyExA(
			winreg::HKEY_LOCAL_MACHINE,
			keyCStr.as_ptr(),
			0,
			winnt::KEY_READ,
			&mut regHandle
		);
		
		if (openResult == ERROR_SUCCESS as i32)
		{
			let mut classNameBuffer = vec![0u8;256];
			let mut classNameLength: DWORD = classNameBuffer.len() as DWORD;
			let mut numSubkeys: DWORD = 0;
			let mut numSubkeysLength: DWORD = 0;
			let mut maxClassLength: DWORD = 0;
			let mut numValues: DWORD = 0;
			let mut maxValueNameLength: DWORD = 0;
			let mut maxValuesLength: DWORD = 0;
			let mut securityDescriptor: DWORD = 0;
			let mut lastWriteTime: FILETIME = zeroed();
			
			let queryResult = winreg::RegQueryInfoKeyA(
				regHandle,
				classNameBuffer.as_mut_ptr() as *mut winnt::CHAR,
				&mut classNameLength as *mut DWORD,
				ptr::null_mut(),
				&mut numSubkeys as *mut DWORD,
				&mut numSubkeysLength as *mut DWORD,
				&mut maxClassLength as *mut DWORD,
				&mut numValues as *mut DWORD,
				&mut maxValueNameLength as *mut DWORD,
				&mut maxValuesLength as *mut DWORD,
				&mut securityDescriptor as *mut DWORD,
				&mut lastWriteTime as *mut FILETIME
			);
			
			if (queryResult == ERROR_SUCCESS as i32)
			{
				//numSubkeys = 0 numSubkeysLength = 0 maxClassLength = 0 numValues = 2 maxValueNameLength = 17 maxValuesLength = 10 securityDescriptor = 220
				// println!("numSubkeys = {} numSubkeysLength = {} maxClassLength = {} numValues = {} maxValueNameLength = {} maxValuesLength = {} securityDescriptor = {}",
				// 	numSubkeys,
				// 	numSubkeysLength,
				// 	maxClassLength,
				// 	numValues,
				// 	maxValueNameLength,
				// 	maxValuesLength,
				// 	securityDescriptor);
				
				let mut valueBuffer = vec![0u8;256];
				let mut valueBufferLength = valueBuffer.len() as DWORD;
				let mut dataBuffer = vec![0u8;256];
				let mut dataBufferLength = dataBuffer.len() as DWORD;
				
				for vIndex in 0..numValues
				{
					println!("Checking value[{}]", vIndex);
				}
			}
			else
			{
				println!("QueryResult: {}", queryResult);
			}
		}
		else { println!("Failed to open registry key: {}", openResult); }
	}
}
