extern crate systemstat;

//use std::thread;
//use std::time::Duration;
use serde::Serialize;
use serde_json;
use std::time::Duration;
use systemstat::{saturating_sub_bytes, Platform, System};

struct StatServer {
    sys: System,
}

#[derive(Serialize)]
struct Stats {
    power: Power,
    memory: Memory,
    uptime: String,
    cpu_temp: String,
}

#[derive(Serialize)]
struct Power {
    percent: String,
    ac_power: String,
}

#[derive(Serialize)]
struct Memory {
    mem_used: String,
    mem_total: String,
}

impl StatServer {
    fn new() -> Self {
        let sys = System::new();

        StatServer { sys }
    }

    fn stats(self: &Self) -> Stats {
        let percent = match self.sys.battery_life() {
            Ok(battery) => (battery.remaining_capacity * 100.0).to_string(),
            Err(x) => x.to_string(),
        };

        let ac_power = match self.sys.on_ac_power() {
            Ok(power) => power.to_string(),
            Err(x) => x.to_string(),
        };

        let mem_used = match self.sys.memory() {
            Ok(mem) => saturating_sub_bytes(mem.total, mem.free).to_string(),
            Err(x) => x.to_string(),
        };

        let mem_total = match self.sys.memory() {
            Ok(mem) => mem.total.to_string(),
            Err(x) => x.to_string(),
        };

        let uptime = match self.sys.uptime() {
            Ok(uptime) => format!("{:?}", uptime),
            Err(x) => x.to_string(),
        };

        let cpu_temp = match self.sys.cpu_temp() {
            Ok(cpu_temp) => cpu_temp.to_string(),
            Err(x) => x.to_string(),
        };

        Stats {
            power: Power { percent, ac_power },
            memory: Memory {
                mem_used,
                mem_total,
            },
            uptime,
            cpu_temp,
        }
    }
}

fn main() {
    let stat = StatServer::new();

    loop {
        let mut s = stat.stats();
        
        println!("{}", to_json(&mut s));
        
        std::thread::sleep(Duration::new(1, 0));
    }
}

fn to_json(stats: &mut Stats) -> String {
    serde_json::to_string(stats).expect("Serialization failed")
}

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
