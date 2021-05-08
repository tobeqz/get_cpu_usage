use std::process::Command;

fn main() {
    // Get threads
    let threads_output = Command::new("sh")
        .arg("-c")
        .arg("ls /sys/class/cpuid")
        .output()
        .expect("Failed");

    let threads_str = String::from_utf8(threads_output.stdout).expect("Failed to parse bytes as strign");
    let thread_split = threads_str.split("\n");
    let mut total_threads = -1;

    for _ in thread_split {
        total_threads += 1;
    }

    let mut total_usage: f64 = 0_f64;
    let processes = Command::new("sh")
        .arg("-c")
        .arg("ps aux --no-header | awk '{ print $3 }'")
        .output()
        .expect("Failed");
    
    let processes_raw = String::from_utf8(processes.stdout).expect("Failed to parse bytes as string");
    let processes_split = processes_raw.split("\n");


    for process_data in processes_split {
        match process_data.parse::<f64>() {
            Ok(usage) => total_usage += usage,
            Err(_) => continue
        }
    }

    println!("{}", total_usage / total_threads as f64);
}
