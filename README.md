This is just an example of how to write **both** eBPF and user-space code in Rust with [RedBPF](https://github.com/foniod/redbpf).

```bash
cargo build
sudo ./target/debug/block-icmp-bpf --dev eth0
```

The above command attaches the XDP code that drops ICMP packets and prints messages to the common trace_pipe (/sys/kernel/debug/tracing/trace_pipe).

[libbpf-rs](https://github.com/libbpf/libbpf-rs) is another option for implemeting eBPF in Rust; writing eBPF code in **C** and user-space in Rust.
