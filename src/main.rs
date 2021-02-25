use std::{thread, time};
use rppal::pwm::{Channel, Pwm};
use std::io::Write;
use std::io::Read;
use serde_json::{Result, Value};
use std::fs;

fn main() {
    println!("Hello, world!");
    //let mut duty = 0.0;
    let pwm = Pwm::new(Channel::Pwm0).unwrap();
    let pwm2 = Pwm::new(Channel::Pwm1).unwrap();
    pwm.set_frequency(25000.0, 0.0).unwrap();
    pwm2.set_frequency(25000.0, 0.0).unwrap();
    pwm.enable().unwrap();
    pwm2.enable().unwrap();
    loop
    {
        thread::sleep(time::Duration::from_secs(5));
        let lock_present = std::path::Path::new("gpio.lock").exists();
        if !lock_present {
            let mut lock = std::fs::File::create("gpio.lock").expect("create failed");
            lock.write_all("Busy".as_bytes()).expect("write failed");
            let path = "./duty.json";
            let data = fs::read_to_string(path).expect("Unable to read file");
            let res: Value = serde_json::from_str(&data).expect("Unable to parse");
            let duty: Option<f64> = res["duty"].as_f64();
            match duty {
                Some(val) => {
                    pwm.set_duty_cycle(val/100.0).unwrap();
                    println!("{}", val);
                },
                None => println!("No duty cycle"),
            }
            fs::remove_file("gpio.lock").expect("could not remove file");
            println!("file is removed");
        }
        else {
            println!("Lock busy, trying again");
        }
    }
}
