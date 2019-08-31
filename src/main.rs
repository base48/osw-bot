use std::io::prelude::*;
use std::{thread, time};
use std::net::TcpStream;
use std::io::{BufReader, BufRead};

static ADDR: &str = "irc.freenode.org:6667";
static CHAN: &str = "#testbot32";
static NICK: &str = "osw-switch";

fn main() -> std::io::Result<()> {
	loop{
		let mut s = TcpStream::connect(ADDR)?;
		s.set_read_timeout(Some(time::Duration::new(360, 0)))?;
		let mut r = BufReader::new(s.try_clone()?);

		s.write(format!("NICK {}\n", NICK).as_ref())?;
		s.write(format!("USER {} 0 * :test bot\n", NICK).as_ref())?;
		s.write(format!("JOIN {}\n", CHAN).as_ref())?;

		loop{
			let mut data = String::new();
			match r.read_line(&mut data) {
				Ok(0) => { thread::sleep(time::Duration::new(180, 0)); break; }
				Ok(_) => { eval(data.trim_end().to_string(), s.try_clone()?)?; }
				Err(_) => { thread::sleep(time::Duration::new(180, 0)); break; }
			}
		}
	}
}

fn eval(mut data: String, mut s: TcpStream) -> std::io::Result<()> {
	println!("{:#?}", data);

	if data.starts_with("PING :") {
		let (_,last) = data.split_at(6);
		s.write(format!("PONG :{}\n", last).as_ref())?;
		println!("{:#?}", format!("PONG :{}\n", last));
	}
	else{
		data.remove(0);
		let (_,last) = data.split_at(data.find(' ').unwrap() + 1);
		if last.starts_with("332 ") || last.starts_with("TOPIC ") {
			let (_,last) = data.split_at(data.find(':').unwrap() + 1);
			println!("TOPIC JE: {:#?}", last);
		}
		if last.starts_with("PRIVMSG ") {
			let (_,last) = data.split_at(data.find(':').unwrap() + 1);
			if last == ".beacon on" { println!("BEACON ON"); }
			if last == ".beacon off" { println!("BEACON OFF"); }
		}
	}
	Ok(())
}
