use std::{process::Command, time::Instant};

#[cfg(target_os = "linux")]
fn get_commands() -> Vec<String> {
    Vec::from([
        String::from("target/release/d01"),
        String::from("target/release/d02"),
        String::from("target/release/d03"),
        String::from("target/release/d04"),
        String::from("target/release/d05"),
        String::from("target/release/d06"),
        String::from("target/release/d07"),
        String::from("target/release/d08"),
        String::from("target/release/d09"),
        String::from("target/release/d10"),
        String::from("target/release/d11"),
        String::from("target/release/d12"),
        String::from("target/release/d13"),
        String::from("target/release/d14"),
        String::from("target/release/d15"),
        String::from("target/release/d16"),
        String::from("target/release/d17"),
        String::from("target/release/d18"),
        String::from("target/release/d19"),
        String::from("target/release/d20"),
        String::from("target/release/d21"),
        String::from("target/release/d22"),
        String::from("target/release/d23"),
        String::from("target/release/d24"),
        String::from("target/release/d25"),
    ])
}

#[cfg(target_os = "windows")]
fn get_commands() -> Vec<String> {
    Vec::from([
        String::from("target/release/d01.exe"),
        String::from("target/release/d02.exe"),
        String::from("target/release/d03.exe"),
        String::from("target/release/d04.exe"),
        String::from("target/release/d05.exe"),
        String::from("target/release/d06.exe"),
        String::from("target/release/d07.exe"),
        String::from("target/release/d08.exe"),
        String::from("target/release/d09.exe"),
        String::from("target/release/d10.exe"),
        String::from("target/release/d11.exe"),
        String::from("target/release/d12.exe"),
        String::from("target/release/d13.exe"),
        String::from("target/release/d14.exe"),
        String::from("target/release/d15.exe"),
        String::from("target/release/d16.exe"),
        String::from("target/release/d17.exe"),
        String::from("target/release/d18.exe"),
        String::from("target/release/d19.exe"),
        String::from("target/release/d20.exe"),
        String::from("target/release/d21.exe"),
        String::from("target/release/d22.exe"),
        String::from("target/release/d23.exe"),
        String::from("target/release/d24.exe"),
        String::from("target/release/d25.exe"),
    ])
}

fn main() {
    let commands = get_commands();

    let overall_start = Instant::now();
    for (idx, command) in commands.iter().enumerate() {
        let start = Instant::now();
        unsafe {
            Command::new(command).output().unwrap_unchecked();
        }
        let stop = start.elapsed();
        println!("Day {} took {} milliseconds", idx + 1, stop.as_millis());
    }
    let overall_stop = overall_start.elapsed();
    println!("Overall it took {} milliseconds or {} seconds", overall_stop.as_millis(), overall_stop.as_secs());
}
