#### NtCreateUserProcess Rust Example

This project demonstrates working examples of using the undocumented `NtCreateUserProcess` Windows API function in Rust. It provides implementations for both suspended and normal process creation, which can be valuable for developers working on process manipulation or security tools.

*note* that this version of the PoC is for x64. It also uses https://github.com/Teach2Breach/noldr to locate ntdll.dll and dynamically locate the function addresses. 

*also note* that this PoC launches cmd.exe by default. It's hardcoded in the func.rs file. This is because apps like calc.exe are now just stubs for the UWP apps and create issues with handle consistency. If you want to test with a different executable, you can change it in the func.rs file. But if you test with a UWP app, you'll have to modify the code to get the correct process and thread handles. This would probably require calling NtOpenProcess to obtain a handle to the UWP app (ie: CalculatorApp.exe) that is launched by the initial calc.exe stub.

##### Overview

The project includes two main process creation methods:
- Creating a suspended process (`CreateSuspendedProcess`)
- Creating a normal process (`CreateUserProcess`)

Both methods demonstrate proper initialization of the required structures and parameters for `NtCreateUserProcess`, including:
- Process parameters setup using `RtlCreateProcessParametersEx`
- Unicode string handling with `RtlInitUnicodeString`
- Proper attribute list configuration

##### Testing

1. Compile the project using `cargo build`
2. Run the compiled executable
3. Choose the process creation mode by entering `1` for suspended process or `2` for normal process
4. cmd.exe is used as the default executable. there is a breakpoint in the code so you can check the pid in task manager
5. Press Enter to continue and observe the output

##### Using as a library in your own project

Add this to your `Cargo.toml`:

```toml
[dependencies]
NtCreateUserProcess_rs = { git = "https://github.com/Teach2Breach/NtCreateUserProcess_rs.git" }
```

##### Usage
example using from your own program:
```rust
use NtCreateUserProcess_rs::CreateSuspendedProcess;
use noldr::{get_dll_address, get_function_address, get_teb};

fn main() {
    let teb = get_teb();
    //println!("teb: {:?}", teb);

    let ntdll = get_dll_address("ntdll.dll".to_string(), teb).unwrap();

    //set the process name to cmd.exe for testing
    let process_name = "cmd.exe".to_string();

    let process_path = if !process_name.contains('\\') {
        format!("C:\\Windows\\System32\\{}", process_name)
    } else {
        process_name
    };

    // Create a suspended CMD process
    let process_handles = CreateSuspendedProcess(ntdll, &process_path);
    println!("process_handles: ProcessHandles {{ process_handle: HANDLE(0x{:x}), thread_handle: HANDLE(0x{:x}) }}", 
        process_handles.process_handle.0,
        process_handles.thread_handle.0
    );
}
```

##### Features

- Creates processes using the native API instead of Win32 APIs
- Proper handling of process and thread handles
- Support for suspended process creation
- Thread resuming functionality
- Process termination with `NtTerminateProcess`

##### Why This Exists

Finding working examples of `NtCreateUserProcess` usage in Rust is challenging, as most documentation and examples are in C/C++. This project aims to provide a reference implementation for Rust developers who need to work with low-level process creation on Windows.

##### Credits

Thanks to [kudaes](https://github.com/Kudaes) for helping me fix a couple issues with the original PoC.
