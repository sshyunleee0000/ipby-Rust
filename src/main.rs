#![deny(warnings)]
#[cfg(not(target_arch = "wasm32"))]

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::process::Command;
    println!("
    ___ ___ _                    _
   |_ _| _ | |__ _  _   _ _  ___| |_
    | ||  _| '_ | || | | ' \\/ -_|  _|
   |___|_| |_.__/\\_, | |_||_\\___|\\__|
                 |__/");                          // logo: IPbynet

    
    // GET PRIVATE IP COMMAND FROM HOST COMPUTER
    let macos_ip = Command::new("ipconfig")         // macOS version
        .args(&["getifaddr", "en0"])
        .output().unwrap_or_else(|error| {
            panic!("failed to execute process: {}", error)
    });
    let windows_ip = Command::new("ipconfig")       // Windows version
        .args(&["|", "grep", "d"])
        .output().unwrap_or_else(|error| {
            panic!("failed to execute process: {}", error)
    });
    let linux_ip = Command::new("hostname")         // Linux version
        .arg("-I")
        .output().unwrap_or_else(|error| {
            panic!("failed to execute process: {}", error)
    });


    // GET PUBLIC IP FROM ipify API (https://www.ipify.org)
    let ipv4_api = reqwest::get("https://api.ipify.org").await?;    // Get public IPv4
    let ipv6_api = reqwest::get("https://api64.ipify.org").await?;  // Get public IPv6
    let ipv4 = ipv4_api.text().await?;
    let ipv6 = ipv6_api.text().await?;


    // OUTPUT THE IP RESULT: PRIVATE IP
    println!("\n\nAbout Private Internet Protocol\n");      // Private IP information
    if cfg!(target_os = "macos") {              // If the Operation Systen is macOS
        if macos_ip.status.success() {
            let privateip = String::from_utf8_lossy(&macos_ip.stdout);
            print!("\t• IP: {}", privateip);
        } else {
            let privateip = String::from_utf8_lossy(&macos_ip.stderr);
            print!("\t• macOS rustc failed and stderr was:\n\n{}", privateip);
        }
    } else if cfg!(target_os = "windows") {     // If the Operation Systen is Windows
        if windows_ip.status.success() {
            let privateip = String::from_utf8_lossy(&windows_ip.stdout);
            print!("{}\n", privateip);
        } else {
            let privateip = String::from_utf8_lossy(&windows_ip.stderr);
            print!("\t• Windows rustc failed and stderr was:\n\n{}", privateip);
        }
    } else if cfg!(target_os = "linux") {       // If the Operation Systen is Linux
        if linux_ip.status.success() {
            let privateip = String::from_utf8_lossy(&linux_ip.stdout);
            print!("\t• IP: {}", privateip);
        } else {                                // Else, just not print the Private IP
            let privateip = String::from_utf8_lossy(&linux_ip.stderr);
            print!("\t• Linux rustc failed and stderr was:\n\n{}", privateip);
        }
    } else {
        println!("• IP: Not support this OS\n");
    };

    
    // OUTPUT THE IP RESULT: PUBLIC IP
    println!("\n\nAbout Public Internet Protocol\n");     // Public IP information
    println!("\t• IPv4: {}", ipv4);
    if ipv4 != ipv6 {                       // If not support IPv6, doesn't print
        println!("\t• IPv6: {}\n\n\n", ipv6);   // Note: If not support IPv6, it will get the same result as IPv4
    } else {
        print!("\t• IPv6: Not activated\n\n\n");
    }
    
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() {}