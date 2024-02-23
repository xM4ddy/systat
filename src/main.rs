extern crate systemstat;

use serde::Serialize;
//use serde_json::Result;
use serde_json;
//use std::thread;
//use std::time::Duration;
use systemstat::{saturating_sub_bytes, Platform, System};

#[derive(Serialize)]
struct Stats {
    battery: String,
    ac: String,
    memory: String,
    uptime: String,
    cpu_temp: String,
}

fn main() {
    let sys = System::new();

    let battery = match sys.battery_life() {
        Ok(battery) => (battery.remaining_capacity * 100.0).to_string(),
        Err(x) => x.to_string(),
    };

    let ac = match sys.on_ac_power() {
        Ok(power) => power.to_string(),
        Err(x) => x.to_string(),
    };

    let memory = match sys.memory() {
        Ok(mem) => format!(
            "{}/{}",
            saturating_sub_bytes(mem.total, mem.free),
            mem.total
        ),
        Err(x) => x.to_string(),
    };

    let uptime = match sys.uptime() {
        Ok(uptime) => format!("{:?}", uptime),
        Err(x) => x.to_string(),
    };

    let cpu_temp = match sys.cpu_temp() {
        Ok(cpu_temp) => cpu_temp.to_string(),
        Err(x) => x.to_string(),
    };

    // match sys.cpu_load_aggregate() {
    //     Ok(cpu) => {
    //         println!("\nMeasuring CPU load...");
    //         thread::sleep(Duration::from_secs(1));
    //         let cpu = cpu.done().unwrap();
    //         println!(
    //             "CPU load: {}% user, {}% nice, {}% system, {}% intr, {}% idle ",
    //             cpu.user * 100.0,
    //             cpu.nice * 100.0,
    //             cpu.system * 100.0,
    //             cpu.interrupt * 100.0,
    //             cpu.idle * 100.0
    //         );
    //     }
    //     Err(x) => println!("\nCPU load: error: {}", x),
    // }

    // match sys.load_average() {
    //     Ok(loadavg) => println!(
    //         "\nLoad average: {} {} {}",
    //         loadavg.one, loadavg.five, loadavg.fifteen
    //     ),
    //     Err(x) => println!("\nLoad average: error: {}", x),
    // }

    // let string = match sys.networks() {
    //     Ok(netifs) => {
    //         format!("\nNetwork interface statistics:");
    //         for netif in netifs.values() {
    //             format!(
    //                 "{} statistics: ({:?})",
    //                 netif.name,
    //                 sys.network_stats(&netif.name)
    //             );
    //         }
    //     }
    //     Err(x) => println!("\nNetworks: error: {}", x),
    // }

    let json = serde_json::to_string(&Stats {
        battery,
        ac,
        memory,
        uptime,
        cpu_temp,
    })
    .expect("Serialization failed");

    println!("{}", json);
}
