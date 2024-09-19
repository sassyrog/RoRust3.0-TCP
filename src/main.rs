fn main() -> anyhow::Result<()> {
    let tun = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buffer: [u8; 1504] = [0; 1504];
    loop {
        let nbytes = tun.recv(&mut buffer)?;
        eprintln!("Read {} bytes", nbytes);
    }
    // Ok(())
}
