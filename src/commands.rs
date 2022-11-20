use std::{process::{Command, Stdio}, os::unix::process::CommandExt, str::{self, FromStr}};

pub static mut ISMONMODE: bool = false;

//isue with comman && execute

pub fn mon_mode_on() {
    //put interface into monotor mode
    let monenabled_ret = String::from("Moniotor mode enabled");
    
    unsafe {
        if ISMONMODE == true {
            println!("{}", monenabled_ret);
        }else{
            Command::new("pkexec")
            .arg("airmon-ng")
            .arg("start")
            .arg(&get_interface())
            .spawn()
            .unwrap();
            
            ISMONMODE = true;

            println!("{}", monenabled_ret)
        }
    }
}

pub fn mon_mode_off() {
    //put interface into monotor mode
    let mondisabled_ret = String::from("Moniotor mode disabled");
    //let mut mon_interface = get_interface();
    //mon_interface.push_str("mon");

    //clean this up please!!!
    unsafe {
        if ISMONMODE == false {
            Command::new("pkexec")
            .arg("airmon-ng")
            .arg("stop")
            .arg(&get_interface())
            .spawn()
            .unwrap();
            
            get_interface();

            ISMONMODE = false;

            println!("{}", mondisabled_ret);
        }else{
            Command::new("pkexec")
            .arg("airmon-ng")
            .arg("stop")
            .arg(&get_interface())
            .spawn()
            .unwrap();
            
            ISMONMODE = false;

            println!("{}", mondisabled_ret);
        }
    }
}

//let's use airmon-n plaese!!!
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
    println!("Your wireless interface is {}", read_out );
    return interface_name;
}

//need to stop airodump....
pub fn dump_air () -> String{
    let pkexec = Command::new("pkexec")
    .arg("airodump-ng")
    .arg(get_interface())
    .stdout(Stdio::piped())
    .spawn()
    .unwrap();

    let raw_out = pkexec.wait_with_output().unwrap();
    let read_out = str::from_utf8(&raw_out.stdout).unwrap();
    let dump_ap = read_out.to_owned();
    return dump_ap
}

pub fn play_air(bssid: String, devmac: String, filenamloc: String ) {
    let airplay = Command::new("pkexec")
    .arg("aireplay-ng")
    .arg(&get_interface())
    .arg("--deauth")
    .arg("3")
    .arg("-a")
    .arg(&bssid);

    let pcap = Command::new("pkexec")
    .arg("airodump-ng")
    .arg(&get_interface())
    .arg("--bssid")
    .arg(&bssid)
    .arg("-w")
    .arg(&filenamloc);

    //no null in rust!!!!
    // if devmac != null {
    // } else {
    //     pcap.spawn();
    //     airplay.arg("-c").arg(&devmac).spawn();

    // } else {
    //     pcap.spawn();
    //     airplay.spawn();
    // }

}