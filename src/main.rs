#![allow(non_snake_case)]

use noldr::{get_dll_address, get_function_address, get_teb};
use winapi::{shared::ntdef::NTSTATUS, um::processthreadsapi::GetProcessId};
mod func;
use crate::func::{CreateSuspendedProcess, CreateUserProcess};
use winapi::shared::ntdef::HANDLE as winapi_HANDLE;

fn main() {
    let teb = get_teb();
    //println!("teb: {:?}", teb);

    //need to add error handling
    let ntdll = get_dll_address("ntdll.dll".to_string(), teb).unwrap();
    
    println!("Choose process creation mode:");
    println!("1. Create suspended process");
    println!("2. Create normal process");
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    
    let handles = match input.trim() {
        "1" => CreateSuspendedProcess(ntdll),
        "2" => CreateUserProcess(ntdll),
        _ => CreateSuspendedProcess(ntdll)
    };

    let process_handle = handles.process_handle;
    let thread_handle = handles.thread_handle;
    let is_suspended = input.trim() == "1" || input.trim() != "2";

    //get pid
    let pid = unsafe { GetProcessId(process_handle.0 as *mut _) };
    println!("pid: {:?}", pid);

    //wait for user input
    println!("press enter to continue");
    let _ = std::io::stdin().read_line(&mut String::new()).unwrap();

    //resume the thread only if process was created suspended
    if is_suspended {
        println!("Resuming thread");
        let resume_result = func::ResumeThread(thread_handle.0 as winapi_HANDLE, ntdll);
        println!("Resume result: {:x?}", resume_result);
    }

    //locate NTTerminateProcess
    let ntdll = get_dll_address("ntdll.dll".to_string(), teb).unwrap();
    let nt_terminate_process_addr = get_function_address(ntdll, "NtTerminateProcess").unwrap();
    let nt_terminate_process: unsafe fn(winapi_HANDLE, u32) -> NTSTATUS = unsafe { std::mem::transmute(nt_terminate_process_addr) };

    unsafe {
        nt_terminate_process(process_handle.0 as winapi_HANDLE, 0);
    }
    
}
