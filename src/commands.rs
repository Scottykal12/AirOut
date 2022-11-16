use std::{process::{Command, Stdio}, os::unix::process::CommandExt, str::{self, FromStr}};

static mut ISMONMODE: bool = false;

pub fn mon_mode_on() {
    //put interface into monotor mode
    let monenabled_ret = String::from("Moniotor mode enabled");
    
    unsafe {
        if ISMONMODE == true {
            println!("{}", monenabled_ret);
            //return monenabled_ret;
        }else{
            //airmon-ng start $wint
            Command::new("pkexec")
            .arg("airmon-ng")
            .arg("start")
            .arg(&get_interface())
            .exec();
            
            ISMONMODE = true;

            println!("{}", monenabled_ret)

            //return monenabled_ret;
        }
    }
}

pub fn mon_mode_off() {
    //put interface into monotor mode
    let mondisabled_ret = String::from("Moniotor mode disabled");
    let mut mon_interface = get_interface();
    mon_interface.push_str("mon");

    unsafe {
        if ISMONMODE == false {
            println!("{}", mondisabled_ret);
            //return monenabled_ret;
        }else{
            //airmon-ng start $wint
            Command::new("pkexec")
            .arg("airmon-ng")
            .arg("stop")
            .arg(&mon_interface)
            .exec();
            
            ISMONMODE = false;

            println!("{}", mondisabled_ret);

            //return monenabled_ret;
        }
    }
}

pub fn get_interface() -> String {
    let iw_cmd = Command::new("iw")
    .arg("dev")
    .stdout(Stdio::piped())
    .spawn()
    .unwrap();
    let grep_child = Command::new("grep")
    .arg("Interface")
    .stdin(Stdio::from(iw_cmd.stdout.unwrap()))
    .stdout(Stdio::piped())
    .spawn()
    .unwrap();
    let awk_child = Command::new("awk")
    .arg("-F")
    .arg(" ")
    .arg("{print $2}")
    .stdin(Stdio::from(grep_child.stdout.unwrap()))
    .stdout(Stdio::piped())
    .spawn()
    .unwrap();
    let xargs_child = Command::new("xargs")
    .arg("echo")
    .arg("-n")
    .stdin(Stdio::from(awk_child.stdout.unwrap()))
    .stdout(Stdio::piped())
    .spawn()
    .unwrap();

    let raw_out = xargs_child.wait_with_output().unwrap();
    let read_out = str::from_utf8(&raw_out.stdout).unwrap();
    let interface_name = read_out.to_owned();
    print!("Your wireless interface is {}", read_out );
    return interface_name;
}