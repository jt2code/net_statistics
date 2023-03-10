use chrono::Local;
use std::{env, fs::read_to_string, ops::Index, path::Path, process::exit, thread, time::Duration};

fn main() {
    let args = Argument::parse();
    let _path = format!("/sys/class/net/{}/statistics", args.iface);
    let path = Path::new(&_path);
    if !path.is_dir() {
        eprintln!("Error: not found interface {:?}", "en");
        exit(1);
    }

    let get_file_bytes = |name: &str| -> u64 {
        if let Ok(s) = read_to_string(path.join(name)) {
            if let Some(byte) = s.strip_suffix("\n") {
                return byte.parse::<u64>().unwrap();
            }
        }
        return 0;
    };

    let fmt = "%m/%d %T";

    loop {
        let before = Local::now();
        let tx_1 = get_file_bytes("tx_bytes");
        let rx_1 = get_file_bytes("rx_bytes");

        thread::sleep(Duration::from_secs(args.interval));

        let now = Local::now();
        println!(
            "{0} - {1} TX {2} ({4}s)\n{0} - {1} RX {3} ({4}s)",
            before.format(fmt),
            now.format(fmt),
            get_unit_bytes(get_file_bytes("tx_bytes") - tx_1),
            get_unit_bytes(get_file_bytes("rx_bytes") - rx_1),
            (now - before).num_seconds(),
        );
        println!("-");
    }
}

fn print_help_exit(exec: &str) {
    eprintln!("Usage: {} <interface> [interval]", exec);
    exit(1);
}

fn get_unit_bytes(n: u64) -> String {
    let kb = 1000.0;
    let mb = kb * 1000.0;
    let gb = mb * 1000.0;

    let n2 = n as f64;
    if n2 < kb {
        return format!("{:6}  B", n);
    } else if n2 < mb {
        return format!("{:6.2} KB", n2 / kb);
    } else if n2 < gb {
        return format!("{:6.2} MB", n2 / mb);
    }
    return format!("{:6.2} GB", n2 / gb);
}

struct Argument {
    iface: String,
    interval: u64,
}

impl Argument {
    fn parse() -> Argument {
        let args: Vec<String> = env::args().collect();

        let mut ret = Argument {
            iface: String::new(),
            interval: 1,
        };

        match args.len() {
            2 => ret.iface = args.index(1).to_string(),
            3 => {
                ret.iface = args.index(1).to_string();
                if let Ok(n) = args.index(2).parse::<u64>() {
                    if n > 0 {
                        ret.interval = n;
                    }
                } else {
                    print_help_exit(args.index(0));
                }
            }
            _ => print_help_exit(args.index(0)),
        }
        return ret;
    }
}
