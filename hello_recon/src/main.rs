// use std::env;

// fn main() {
//     // Compile-time constants from the Rust standard library
//     let os = env::consts::OS; // "windows", "linux", "macos"
//     let arch = env::consts::ARCH; // "x86_64", "aarch64"

//     // Runtime environment lookups
//     let user = env::var("USER").unwrap_or_else(|_| "unknown".to_string());
//     let cwd = env::current_dir()
//         .map(|p| p.display().to_string())
//         .unwrap_or_else(|_| "unknown".to_string());

//     println!("=== Host Info ===");
//     println!("OS:      {os}");
//     println!("Arch:    {arch}");
//     println!("User:    {user}");
//     println!("CWD:     {cwd}");
// }

// -------------------- Environment Variables [Easy Way] --------------------- //

// use std::env;

// fn main() {
//     println!("OS: {}", env::consts::OS);
//     println!("Arch: {}", env::consts::ARCH);

//     let user = env::var("USER");
//     println!("User: {:?}", user);

//     let cwd = env::current_dir();
//     println!("CWD: {:?}", cwd);
// }

// -------------------- Environment Variables [With Ok(..)] --------------------- //

// use std::env;

// fn main() {
//     let user = env::var("USER").unwrap_or(String::from("unknown"));
//     // let user = env::var("USER").unwrap_or(String::from("unknown"));

//     let cwd = env::current_dir().unwrap().display().to_string();
//     // let cwd = env::current_dir()
//     //     .unwrap()
//     //     .display()
//     //     .to_string();

//     println!("OS: {}", env::consts::OS);
//     println!("Arch: {}", env::consts::ARCH);
//     println!("User: {}", user);
//     println!("CWD: {}", cwd);

// -------------------- Challenge --------------------- //

// Standard library imports
use std::env;
use std::ffi::{CStr, CString};
use std::ptr::{null, null_mut};

// Windows API imports
use windows_sys::Win32::Foundation::LocalFree;
use windows_sys::Win32::Security::Authorization::ConvertSidToStringSidA;
use windows_sys::Win32::Security::{LookupAccountNameA, SID_NAME_USE};

fn main() {
    // ============================================================
    // Challenge 1 - Computer Name
    // ============================================================

    // Read the COMPUTERNAME environment variable.
    // If it doesn't exist, use "unknown".
    let hostname = env::var("COMPUTERNAME").unwrap_or_else(|_| "unknown".to_string());

    println!("Hostname : {}", hostname);

    // ============================================================
    // Challenge 2 - Domain Membership
    // ============================================================

    // Get the local computer name.
    let computer_name = env::var("COMPUTERNAME").unwrap_or_else(|_| "unknown".to_string());

    // Get the domain/workgroup name.
    let user_domain = env::var("USERDOMAIN").unwrap_or_else(|_| "unknown".to_string());

    // If USERDOMAIN and COMPUTERNAME are the same,
    // the machine is probably not domain joined.
    if computer_name.eq_ignore_ascii_case(&user_domain) {
        println!("Domain   : workgroup");
    } else {
        println!("Domain   : domain-joined ({})", user_domain);
    }

    // ============================================================
    // Challenge 3 - Current Executable Path
    // ============================================================

    // Get the full path of the currently running executable.
    let exe_path = env::current_exe()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    println!("Exe Path : {}", exe_path);

    // ============================================================
    // Challenge 5 - JSON Output
    // ============================================================

    // Compile-time constants provided by Rust.
    let os = env::consts::OS;
    let arch = env::consts::ARCH;

    // Current username.
    let user = env::var("USERNAME").unwrap_or_else(|_| "unknown".to_string());

    // Current working directory.
    let cwd = env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    // Build a JSON string manually.
    // Backslashes must be escaped because JSON uses '\' as an escape character.
    let json = format!(
        "{{\"os\":\"{}\",\"arch\":\"{}\",\"user\":\"{}\",\"cwd\":\"{}\"}}",
        os,
        arch,
        user,
        cwd.replace('\\', "\\\\")
    );

    println!("{}", json);

    // ============================================================
    // Bonus Challenge - Get Current User SID
    // ============================================================

    unsafe {
        // Get the current username from Windows.
        let username = env::var("USERNAME").unwrap();

        // Convert Rust String -> C String
        // Windows API expects C-style strings.
        let username_c = CString::new(username).unwrap();

        // These variables will receive the required buffer sizes.
        let mut sid_size = 0u32;
        let mut domain_size = 0u32;

        // Receives the SID type (User, Group, etc.)
        let mut sid_type: SID_NAME_USE = 0;

        // --------------------------------------------------------
        // First call
        //
        // We intentionally pass NULL buffers.
        // Windows tells us how large the SID and Domain buffers
        // need to be.
        // --------------------------------------------------------
        LookupAccountNameA(
            null(),
            username_c.as_ptr() as *const u8,
            null_mut(),
            &mut sid_size,
            null_mut(),
            &mut domain_size,
            &mut sid_type,
        );

        // Allocate buffers using the sizes Windows returned.
        let mut sid = vec![0u8; sid_size as usize];
        let mut domain = vec![0u8; domain_size as usize];

        // --------------------------------------------------------
        // Second call
        //
        // Now that we have correctly sized buffers,
        // Windows fills them with the SID information.
        // --------------------------------------------------------
        let success = LookupAccountNameA(
            null(),
            username_c.as_ptr() as *const u8,
            sid.as_mut_ptr() as *mut _,
            &mut sid_size,
            domain.as_mut_ptr(),
            &mut domain_size,
            &mut sid_type,
        );

        if success != 0 {
            // Windows will allocate memory and return
            // a pointer to a SID string.
            let mut sid_string = null_mut();

            // Convert binary SID -> string SID
            //
            // Example:
            // S-1-5-21-123456789-987654321-1111111111-1001
            if ConvertSidToStringSidA(sid.as_mut_ptr() as *mut _, &mut sid_string) != 0 {
                // Convert C string -> Rust string.
                let sid_str = CStr::from_ptr(sid_string as *const i8).to_string_lossy();

                println!("SID: {}", sid_str);

                // Free memory allocated by ConvertSidToStringSidA.
                LocalFree(sid_string as _);
            }
        }
    }
}
