extern crate systemstat;

use serde::Serialize;
use serde_json;
use simple_websockets::{Event, Message, Responder};
use std::collections::HashMap;
use systemstat::{saturating_sub_bytes, Platform, System};

type WrappedFloat = Result<f64, String>;
type WrappedBool = Result<bool, String>;
type WrappedInt = Result<u64, String>;

struct StatServer {
    sys: System,
}

#[derive(Serialize)]
struct Stats {
    power: Power,
    memory: Memory,
    uptime: WrappedInt,
    cpu_temp: WrappedFloat,
}

#[derive(Serialize)]
struct Power {
    percent: WrappedFloat,
    ac_power: WrappedBool,
}

#[derive(Serialize)]
struct Memory {
    mem_used: WrappedFloat,
    mem_total: WrappedFloat,
}

impl StatServer {
    fn new() -> Self {
        let sys = System::new();

        StatServer { sys }
    }

    fn stats(self: &Self) -> Stats {
        let percent: WrappedFloat = match self.sys.battery_life() {
            Ok(battery) => Ok(battery.remaining_capacity as f64 * 100.0),
            Err(x) => Err(x.to_string()),
        };

        let ac_power: WrappedBool = match self.sys.on_ac_power() {
            Ok(power) => Ok(power),
            Err(x) => Err(x.to_string()),
        };

        let mem_used: WrappedFloat = match self.sys.memory() {
            Ok(mem) => Ok(saturating_sub_bytes(mem.total, mem.free)
                .to_string()
                .replace(" GB", "")
                .parse::<f64>()
                .unwrap()),
            Err(x) => Err(x.to_string()),
        };

        let mem_total: WrappedFloat = match self.sys.memory() {
            Ok(mem) => Ok(mem
                .total
                .to_string()
                .replace(" GB", "")
                .parse::<f64>()
                .unwrap()),
            Err(x) => Err(x.to_string()),
        };

        let uptime: WrappedInt = match self.sys.uptime() {
            Ok(uptime) => Ok(uptime.as_secs()),
            Err(x) => Err(x.to_string()),
        };

        let cpu_temp: WrappedFloat = match self.sys.cpu_temp() {
            Ok(cpu_temp) => Ok(cpu_temp.into()),
            Err(x) => Err(x.to_string()),
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
    let event_hub = simple_websockets::launch(8080).expect("failed to listen on port 8080");
    let mut clients: HashMap<u64, Responder> = HashMap::new();

    let stat = StatServer::new();

    loop {
        let mut s = stat.stats();

        match event_hub.poll_event() {
            Event::Connect(client_id, responder) => {
                println!("A client connected with id #{}", client_id);
                clients.insert(client_id, responder);
            }
            Event::Disconnect(client_id) => {
                println!("Client #{} disconnected.", client_id);
                clients.remove(&client_id);
            }
            Event::Message(client_id, message) => {
                if let Message::Text(str) = message {
                    println!("Received a message from client #{}: {}", client_id, str);
                    let responder = clients.get(&client_id).unwrap();
                    responder.send(simple_websockets::Message::Text(to_json(&mut s)));
                }
            }
        }
    }
}

fn to_json(stats: &mut Stats) -> String {
    serde_json::to_string(stats).expect("Serialization failed")
}
