use std::fs::File;
use std::io;
use std::io::*;
use tokio_modbus::prelude::*;
#[tokio::main(flavor = "current_thread")]
async fn main() {
    println!("Enter IP address of your heatpump:");
    let mut ip = String::new();
    io::stdin()
        .read_line(&mut ip)
        .expect("error: unable to read user input");
    ip = ip.trim().to_string();
    let ip = ip + ":502";
    println!("Connecting to {}", ip);
    let socket_addr: std::net::SocketAddr = ip.parse().expect("failed to parse IP address");
    let mut connection = Some(tcp::connect(socket_addr).await.expect("Failed to connect"));
    println!("Connected to heatpump");

    let mut f = File::create("weishaupt_modbus_scan.txt").expect("Error creating file");
    let mut text = "".to_string();
    for addr in 30001..=39999 {
        let val = connection
            .as_mut()
            .unwrap()
            .read_input_registers(addr, 1)
            .await;

        match val {
            Ok(val) => match val {
                Ok(val) => {
                    text = val[0].to_string();
                    let text = addr.to_string() + ", " + text.as_str() + "\n";
                    f.write_all(text.as_bytes()).expect("Error writing to file");
                }
                Err(_) => {}
            },
            Err(_) => {}
        }
    }

    for addr in 40001..=49999 {
        let val = connection
            .as_mut()
            .unwrap()
            .read_holding_registers(addr, 1)
            .await;

        match val {
            Ok(val) => match val {
                Ok(val) => {
                    text = val[0].to_string();
                    let text = addr.to_string() + ", " + text.as_str() + "\n";
                    f.write_all(text.as_bytes()).expect("Error writing to file");
                }
                Err(_) => {}
            },
            Err(_) => {}
        }
    }
}
