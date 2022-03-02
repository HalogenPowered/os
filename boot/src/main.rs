use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};
use std::time::Duration;

const RUN_ARGS: &[&str] = &["-no-reboot", "-no-shutdown"];
const TEST_ARGS: &[&str] = &[
    "-device",
    "isa-debug-exit,iobase=0xF4,iosize=0x04",
    "-serial",
    "stdio",
    "-display",
    "none",
    "--no-reboot"
];
const TEST_TIMEOUT_SECONDS: u64 = 10;

fn main() {
    let mut args = std::env::args().skip(1); // Skip executable name

    let kernel_binary_path = {
        let path = PathBuf::from(args.next().unwrap());
        path.canonicalize().unwrap()
    };
    // This is a bit of a hack to allow kimage to use `cargo run` without running QEMU.
    let no_boot = if let Some(arg) = args.next() {
        match arg.as_str() {
            "--no-run" => true,
            other => panic!("Unexpected argument {}!", other),
        }
    } else {
        false
    };
    let uefi = if let Some(arg) = args.next() {
        match arg.as_str() {
            "--uefi" => true,
            other => panic!("Unexpected argument {}!", other)
        }
    } else {
        false
    };

    let image = create_disk_images(&kernel_binary_path, uefi);

    if no_boot {
        println!("Created disk image at `{}`", image.display());
        return;
    }

    let mut run_command = Command::new("qemu-system-x86_64");
    run_command
        .arg("-drive")
        .arg(format!("format=raw,file={}", image.display()));

    let binary_kind = runner_utils::binary_kind(&kernel_binary_path);
    if binary_kind.is_test() {
        run_command.args(TEST_ARGS);
        let exit_status = run_test_command(run_command);
        match exit_status.code() {
            Some(33) => {},
            other => panic!("Test failed! Exit code: {:?}", other)
        }
    } else {
        run_command.args(RUN_ARGS);
        let exit_status = run_command.status().unwrap();
        if !exit_status.success() {
            std::process::exit(exit_status.code().unwrap_or(1));
        }
    }
}

fn run_test_command(mut command: Command) -> ExitStatus {
    runner_utils::run_with_timeout(&mut command, Duration::from_secs(TEST_TIMEOUT_SECONDS)).unwrap()
}

pub fn create_disk_images(kernel_binary_path: &Path, uefi: bool) -> PathBuf {
    let bootloader_manifest_path = bootloader_locator::locate_bootloader("bootloader").unwrap();
    let kernel_manifest_path = locate_cargo_manifest::locate_manifest().unwrap();

    let mut build_command = Command::new(env!("CARGO"));
    build_command
        .current_dir(bootloader_manifest_path.parent().unwrap())
        .arg("builder")
        .arg("--kernel-manifest")
        .arg(&kernel_manifest_path)
        .arg("--kernel-binary")
        .arg(&kernel_binary_path)
        .arg("--target-dir")
        .arg(kernel_manifest_path.parent().unwrap().join("target"))
        .arg("--out-dir")
        .arg(kernel_binary_path.parent().unwrap())
        .arg("--quiet");

    if !build_command.status().unwrap().success() {
        panic!("Build failed!");
    }

    let kernel_binary_name = kernel_binary_path.file_name().unwrap().to_str().unwrap();
    let firmware_interface_name = if uefi { "uefi" } else { "bios" };
    let disk_image = kernel_binary_path
        .parent()
        .unwrap()
        .join(format!("boot-{}-{}.img", firmware_interface_name, kernel_binary_name));
    if !disk_image.exists() {
        panic!(
            "Disk image does not exist at {} after bootloader build",
            disk_image.display()
        );
    }
    disk_image
}
