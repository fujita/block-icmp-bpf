#![no_std]
#![no_main]

use redbpf_probes::xdp::prelude::*;
use redbpf_probes::helpers::bpf_trace_printk;

program!(0xFFFFFFFE, "GPL");

#[xdp]
pub fn block_icmp(ctx: XdpContext) -> XdpResult {
    if let Ok(iph) = ctx.ip() {
        unsafe {
            if (*iph).protocol as u32 == 1 {
                bpf_trace_printk(b"PING\0");
                return Ok(XdpAction::Drop);
            }
        }
    }
    Ok(XdpAction::Pass)
}
