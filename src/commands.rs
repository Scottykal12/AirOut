use core::time;
use std::{
    os::unix::process::CommandExt,
    process::{Command, Stdio, Output},
    str::{self, FromStr},
    thread, fs::read,
};

pub fn is_mon_mode(interface: &String) -> String {
    // return true if iw $int info type monitor
    // iw dev wlp0s20f3 info | grep type | awk -F " " '{print $2}'
    let iw_cmd = Command::new("iw")
        .arg("dev")
        .arg(interface)
        .arg("info")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let grep_child = Command::new("grep")
        .arg("type")
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
    let interface_mon = read_out.to_owned();
    return interface_mon;

}

//issue with command && execute

pub fn mon_mode_on(interface: &String) {
    //put interface into monotor mode
    if is_mon_mode(interface) == "monitor".to_string() {
        println!("{} is in {} mode", interface , is_mon_mode(interface));
    } else {
        Command::new("pkexec")
            .arg("airmon-ng")
            .arg("start")
            .arg(interface)
            .output()
            .unwrap();

        get_interface();
        println!("{} is in {} mode", interface , is_mon_mode(interface));
    }
}

pub fn mon_mode_off(interface: &String) {
    // use ip link set
    if is_mon_mode(interface) == "managed".to_string() {
        Command::new("pkexec")
            .arg("airmon-ng")
            .arg("stop")
            .arg(interface)
            .output()
            .unwrap();

        println!("{} is in {} mode", interface , is_mon_mode(interface));
    } else {
        Command::new("pkexec")
            .arg("airmon-ng")
            .arg("stop")
            .arg(interface)
            .output()
            .unwrap();

        println!("{} is in {} mode", interface , is_mon_mode(interface));
    }
}

// This only prints one item needs to be an array.
pub fn get_interface() -> Vec<String> {
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

        // We get a panic here for some reason.

    let raw_out = xargs_child.wait_with_output().unwrap();
    let read_out = str::from_utf8(&raw_out.stdout).unwrap();
    let interface_name = read_out.to_owned();
    let ifaces_str = interface_name.split_whitespace().collect::<Vec<&str>>();
    let mut ifaces: Vec<String> = vec![];

    // Values converted from str to String
    for i in ifaces_str {
        ifaces.push(i.to_string());
    }

    // return interface_name;
    return ifaces;
}

//need to stop airodump....
//child.kll().expect("This might Stop it?")
pub fn dump_air() -> &'static str {
    let mut airodump = Command::new("pkexec")
        .arg("airodump-ng")
        .arg(&get_interface()[0])
        // .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let raw_out = airodump.wait_with_output().unwrap();
    let read_out = str::from_utf8(&raw_out.stdout).unwrap();
    // let dump_ap = read_out.to_owned();
    println!("{}", read_out);
    return "test";

    // thread::sleep(time::Duration::from_secs(10));    
    // airodump.kill().unwrap();
    //return dump_ap
}

//what happens if no devmac
pub fn play_air(bssid: String, clientmac: String, filenamloc: String) {
    let airplay = Command::new("pkexec")
        .arg("aireplay-ng")
        .arg(&get_interface()[0])
        .arg("--deauth")
        .arg("3")
        .arg("-a")
        .arg(&bssid);

    let pcap = Command::new("pkexec")
        .arg("airodump-ng")
        .arg(&get_interface()[0])
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
