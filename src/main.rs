use std::time::{Duration, SystemTime};
use std::process::{Command, Stdio};
use std::thread::sleep;

fn main() {
    println!("This is RSMONITOR");
    let sys_time = SystemTime::now();
    println!("Starting engines {sys_time:?}");
    get_status();
    
}


fn get_status() {
    loop {
        // let mut send_cmd = Command::new("echo");
        // send_cmd.arg("docker stats --no-stream");
        // send_cmd.arg("|");
        // send_cmd.arg("pipenames/statuspipe");
        

        let mut read_cmd = Command::new("cat");
        read_cmd.arg("pipenames/status.txt");

        match read_cmd.output() {
            Ok(o) =>{
                unsafe {

                    println!("{}", String::from_utf8_unchecked(o.stdout));
                }
            },
            Err(e) => {
                println!("Error: {}", e);
            }
        }
        
        //let status = String::from(output);
        //println!("{}", status);
        //println!("{:?}",output);
        sleep(Duration::new(5, 0));

    }
}
