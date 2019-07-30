extern crate clap;

use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead};
use std::net::Ipv4Addr;

struct IPRange {
    first: u32,
    last: u32,
}

fn set_bit(original: u32, bit: u32) -> u32 {
    let mask = 1 << bit;
    return original | mask;
}

fn get_ip_range(cidr: &str) -> Option<IPRange> {
    let ip_and_mask = cidr.split("/").collect::<Vec<&str>>();
    if ip_and_mask.len() < 2 {
        return None;
    }
    let ip = ip_and_mask[0]; // IP Address
    let mask: u32 = ip_and_mask[1].parse::<u32>().unwrap(); // Subnet mask
    let addr: Ipv4Addr = ip.parse().unwrap(); // IPv4

    let mut ip_mask_long: u32 = 0;
    let mut inverse_ip_mask_long: u32 = 0;
    for i in 0..32 {
        if i < mask {
            ip_mask_long = set_bit(ip_mask_long, 31 - i);
        } else {
            inverse_ip_mask_long = set_bit(inverse_ip_mask_long, 31 - i)
        }
    }

    //println!("{} {}", ip_mask_long, Ipv4Addr::from(ip_mask_long));
    //println!("{} {}", inverse_ip_mask_long, Ipv4Addr::from(inverse_ip_mask_long));
    let ip_long = u32::from(addr); // u32 of the IP address
                                   //println!("{}", Ipv4Addr::from(ip_long));

    let network = ip_long & ip_mask_long;
    let start = network + 1; // ignore network IP, i.e. 192.168.2.0
    let end = (network | inverse_ip_mask_long) - 1; // ignore broadcast IP, i.e. 192.168.2.255

    let res = IPRange {
        first: start,
        last: end,
    };
    return Some(res);
}

fn file_to_string_vec(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let buf = io::BufReader::new(file);
    let results: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect::<Vec<String>>();
    return results;
}

fn get_each_ip_in_range(cidr: &str) -> Option<Vec<u32>> {
    let range = get_ip_range(cidr);
    match range {
        Some(range) => {
            let mut res: Vec<u32> = Vec::new();
            for ip in range.first..=range.last {
                res.push(ip);
            }
            return Some(res);
        }
        None => {
            return None;
        }
    }
}

fn main() {
    let matches = App::new("cidr-to-iplist")
        .version("1.0")
        .author("Valerio Santinelli <santinelli@altralogica.it>")
        .about("Convert a list of subnets to the corrisponding list of IP addresses")
        .arg(
            Arg::with_name("input")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let filename = matches.value_of("input").unwrap();
    let lines = file_to_string_vec(filename);
    for line in lines {
        let ips = get_each_ip_in_range(&line);
        match ips {
            Some(ips) => {
                for ip in ips {
                    println!("{}", Ipv4Addr::from(ip));
                }
            }
            None => {}
        }
    }
}
