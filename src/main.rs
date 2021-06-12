/*
 * Copyright (C) 2021 The block-icmp-bpf Authors.
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License as
 * published by the Free Software Foundation, version 2 of the
 * License.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA
 * 02110-1301 USA
 */
 
 use clap::{App, Arg};
use redbpf::{xdp, Module};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time};

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .unwrap();

    let args = App::new("block-icmp-bpf")
        .arg(
            Arg::with_name("DEV")
                .long("dev")
                .takes_value(true)
                .required(true)
                .help("specify device name (e.g, eth0)"),
        )
        .get_matches();

    let dev_name = args.value_of("DEV").unwrap();

    let code = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/target/bpf/programs/block_icmp/block_icmp.elf"
    ));
    let mut module = Module::parse(code).unwrap();
    for prog in module.programs.iter_mut() {
        prog.load(module.version, module.license.clone()).unwrap();
    }

    for prog in module.xdps_mut() {
        prog.attach_xdp(dev_name, xdp::Flags::default()).unwrap();
    }

    while running.load(Ordering::SeqCst) {
        thread::sleep(time::Duration::from_secs(1));
    }
}
