use std::ffi::OsString;
use std::process::{Command, exit};
use std::ptr::null_mut;

#[cfg(windows)]
use std::os::windows::ffi::OsStringExt;
#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;
#[cfg(windows)]
use std::os::windows::process::CommandExt;
#[cfg(windows)]
use winapi::um::shellapi::CommandLineToArgvW;
#[cfg(windows)]
use winapi::um::winuser::{MessageBoxW, MB_ICONERROR, MB_OK};
#[cfg(windows)]
use winapi::um::processthreadsapi::{CreateProcessW, PROCESS_INFORMATION, STARTUPINFOW};
#[cfg(windows)]
use winapi::um::handleapi::CloseHandle;
#[cfg(windows)]
use winapi::um::synchapi::WaitForSingleObject;
#[cfg(windows)]
use winapi::um::winbase::{CREATE_NO_WINDOW, INFINITE};

#[cfg(windows)]
fn get_message(message: &str) -> Vec<u16> {
    OsString::from(message).encode_wide().chain(Some(0)).collect()
}

#[cfg(windows)]
fn show_error_and_exit(message: &str) {
    let wide_message: Vec<u16> = get_message(message);
    let wide_title: Vec<u16> = get_message(message);
    unsafe {
        MessageBoxW(null_mut(), wide_message.as_ptr(), wide_title.as_ptr(), MB_ICONERROR | MB_OK);
    }
    exit(1);
}

#[cfg(windows)]
fn os_string_from_wide(arg: *const u16) -> OsString {
    let len = (0..).take_while(|&i| unsafe { *arg.offset(i) != 0 }).count();
    OsString::from_wide(unsafe { std::slice::from_raw_parts(arg, len) })
}

fn is_java_in_path() -> bool {
    let output = if cfg!(windows) {
        Command::new("java").arg("-version").creation_flags(CREATE_NO_WINDOW).output()
    } else {
        Command::new("java").arg("-version").output()
    };
    output.map_or(false, |o| o.status.success())
}

#[cfg(windows)]
fn main() {
    if is_java_in_path() {
        let mut argc: i32 = 0;
        let cmd_line = unsafe { CommandLineToArgvW(winapi::um::processenv::GetCommandLineW(), &mut argc) };
        if cmd_line.is_null() {
            show_error_and_exit("Failed to get command line arguments");
        }

        let args: Vec<OsString> = unsafe {
            let slice = std::slice::from_raw_parts(cmd_line, argc as usize);
            slice.iter().map(|&arg| os_string_from_wide(arg)).collect()
        };

        if args.len() > 1 {
            let mut command = OsString::from("javaw -jar ");
            command.push(&args[1]);
            let command_wide: Vec<u16> = command.encode_wide().chain(Some(0)).collect();
            let command_str: *mut u16 = command_wide.as_ptr() as *mut u16;
            let mut si: STARTUPINFOW = unsafe { std::mem::zeroed() };
            si.cb = std::mem::size_of::<STARTUPINFOW>() as u32;
            let mut pi: PROCESS_INFORMATION = unsafe { std::mem::zeroed() };

            let success = unsafe {
                CreateProcessW(
                    null_mut(),
                    command_str,
                    null_mut(),
                    null_mut(),
                    false as i32,
                    0,
                    null_mut(),
                    null_mut(),
                    &mut si,
                    &mut pi
                )
            };

            if success == 0 {
                show_error_and_exit("Failed to launch javaw.exe. Please check your Java installation and try again.");
            }

            unsafe {
                WaitForSingleObject(pi.hProcess, INFINITE);
                CloseHandle(pi.hProcess);
                CloseHandle(pi.hThread);
            }
        } else {
            show_error_and_exit("No file specified!");
        }
    } else {
        show_error_and_exit("Java not found!\n\nDownload and install a JRE.");
    }
}

#[cfg(unix)]
fn main() {
    if is_java_in_path() {
        let args: Vec<String> = std::env::args().collect();
        if args.len() > 1 {
            let mut java_cmd = vec!["java".to_string(), "-jar".to_string()];
            java_cmd.extend_from_slice(&args[1..]);

            let status = Command::new("java").args(&java_cmd).status();
            if let Err(e) = status {
                eprintln!("Error: java not found or error executing command: {}", e);
                exit(1);
            }
        } else {
            eprintln!("No file specified!");
            exit(1);
        }
    } else {
        eprintln!("Java not found! Download and install a JRE.");
        exit(1);
    }
}
