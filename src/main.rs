use monoio::io::{AsyncReadRent, AsyncWriteRentExt};
use monoio::net::{TcpListener, TcpStream};
use monoio::time::{sleep, Duration};
use nix::sys::resource::{getrlimit, setrlimit, Resource};

const MIN_TCP_PORT: u16 = 1;
const MAX_TCP_PORT: u16 = u16::MAX;

#[monoio::main(timer_enabled = true)]
async fn main() {
    let (soft_limit, hard_limit) = getrlimit(Resource::RLIMIT_NOFILE).unwrap();
    println!("current soft_limit: {}", soft_limit);
    println!("current hard_limit: {}", hard_limit);
    if hard_limit < MAX_TCP_PORT.into() {
        println!("detected insufficient number of files allowed");
        println!("you're going to want to make friends with ulimit(1)");
        std::process::exit(1);
    } else if soft_limit < MAX_TCP_PORT.into() {
        setrlimit(Resource::RLIMIT_NOFILE, hard_limit, hard_limit).expect("failed setrlimit");
        println!("adjusted ulimit for number of files to bump soft limit to match hard limit");
    }

    // we start at port 1 as 0 is used for getting a random aka ephemeral port.
    // the "as u32" is to because to include u16::MAX we need u16::MAX+1
    // as the range is non-inclusive, and the +1 would overflow a u16
    for port in MIN_TCP_PORT as u32..MAX_TCP_PORT as u32 + 1 {
        print!("binding to port: {} ", port);
        if let Ok(listener) = TcpListener::bind(format!("0.0.0.0:{}", port)) {
            println!("successfully!");
            monoio::spawn(async move {
                loop {
                    let incoming = listener.accept().await;
                    match incoming {
                        Ok((stream, _addr)) => {
                            println!("accepted a connection to {}", port);
                            monoio::spawn(echo(stream, port as u16));
                        }
                        Err(e) => {
                            println!("accepted connection failed: {}", e);
                            return;
                        }
                    }
                }
            });
        } else {
            println!("unsuccessfully :(");
        }
    }

    // TODO: is there a nice way to run something like a join here instead ?
    loop {
        sleep(Duration::from_millis(5000)).await;
    }
}

async fn echo(mut stream: TcpStream, port: u16) -> std::io::Result<()> {
    let mut buf: Vec<u8> = Vec::with_capacity(8 * 1024);
    let mut res;
    // greet
    let _ = stream
        .write_all(format!("welcome to port {}\r\n", port).into_bytes())
        .await;
    loop {
        // read
        (res, buf) = stream.read(buf).await;
        if res? == 0 {
            return Ok(());
        }

        // write all
        (res, buf) = stream.write_all(buf).await;
        res?;

        // clear
        buf.clear();
    }
}
