use crate::system::{
    linux::{
        commands::{
            install_packages, install_tools_of_rust, run_command, verificatio_package_installed,
            verification_compilator_c,
        },
        verification_distribution::verification_shell,
    },
    types::{LinuxDistro, OperatingSystem, SystemInformation},
};

pub fn install_rust(system_information: SystemInformation) {
    match system_information.os {
        OperatingSystem::Linux => install_rust_in_linux(&system_information),
        OperatingSystem::Windows => install_rust_in_windows(),
        OperatingSystem::MacOS => install_rust_in_macos(),
        OperatingSystem::Unknown => println!("System operating not found"),
    }
}

fn install_rust_in_linux(information_system: &SystemInformation) {
    let needs_cc = !verification_compilator_c()
        .map_err(|e| e.to_string())
        .unwrap();
    let needs_curl = !verificatio_package_installed("curl");
    if needs_curl | needs_cc {
        install_deps_linux(&information_system.distro.unwrap(), needs_cc, needs_curl);
    }
    if verificatio_package_installed("rustup") {
        println!("update rustup");
        run_command("rustup", &["update"]);
    } else {
        println!("Installing rustup");
        run_command(
            "sh",
            &[
                "-c",
                "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y",
            ],
        );
    }

    let _ = verification_shell();
    println!("Installing rust-analyzer and rust-src...");
    let _ = install_tools_of_rust();
}

fn install_deps_linux(linux_distribution: &LinuxDistro, need_cc: bool, need_curl: bool) {
    match linux_distribution {
        LinuxDistro::Arch => {
            let mut args = vec!["pacman", "-S", "--needed", "--noconfirm"];
            if need_cc {
                args.push("base-devel");
            }
            if need_curl {
                args.push("curl");
            }
            args.extend(["gdb", "lldb", "pkgconf", "binutils"]);
            install_packages("sudo", &args);
        }
        LinuxDistro::Fedora => {
            let mut args = vec!["dnf", "install"];
            if need_cc {
                args.push("base-devel");
            }
            if need_curl {
                args.push("curl");
            }
            args.extend(["gdb", "lldb", "pkgconf-pkg-config", "binutils"]);
            install_packages("sudo", &args);
        }
        _ => print!(""),
    }
}
fn install_rust_in_windows() {}
fn install_rust_in_macos() {}
