extern crate systemstat;

use simple_websockets::{Event, Responder};
use std::collections::HashMap;
use serde::Serialize;
use serde_json;
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
            power: Power {
                percent, 
                ac_power
            },
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
    let event_hub = simple_websockets::launch(8080)
        .expect("failed to listen on port 8080");
    let mut clients: HashMap<u64, Responder> = HashMap::new();

    let stat = StatServer::new();

    
    loop {
        let mut s = stat.stats();

        match event_hub.poll_event() {
            Event::Connect(client_id, responder) => {
                println!("A client connected with id #{}", client_id);
                clients.insert(client_id, responder);
            },
            Event::Disconnect(client_id) => {
                println!("Client #{} disconnected.", client_id);
                clients.remove(&client_id);
            },
            Event::Message(client_id, message) => {
                println!("Received a message from client #{}: {:?}", client_id, message);
                let responder = clients.get(&client_id).unwrap();
                responder.send(simple_websockets::Message::Text(to_json(&mut s)));
            },
        }
    }
}

fn to_json(stats: &mut Stats) -> String {
    serde_json::to_string(stats).expect("Serialization failed")
}
