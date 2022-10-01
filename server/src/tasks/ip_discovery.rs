use log::{debug, info};
use tokio::net::UdpSocket;
use tokio::task::JoinHandle;

const HANDSHAKE_BEGIN_PORT: u16 = 31337;
const HANDSHAKE_END_PORT: u16 = 31338;

const HANDSHAKE_REQUEST: &str = "HelloDearRusty";
const HANDSHAKE_RESPONSE: &str = "HeyoDearClient";

pub(super) fn spawn() -> JoinHandle<Option<()>> {
    tokio::spawn(async {
        let socket = UdpSocket::bind(format!("0.0.0.0:{HANDSHAKE_BEGIN_PORT}"))
            .await
            .expect("Failed binding");
        info!("Binding on {}", HANDSHAKE_BEGIN_PORT);

        socket.set_broadcast(true).unwrap();

        loop {
            let mut packet = [0; 16];

            let recv = socket.recv_from(&mut packet).await;

            if recv.is_err() {
                continue;
            }

            let (_, mut src) = recv.unwrap();

            let ascii_packet = String::from_utf8_lossy(&packet[..14]);

            if !ascii_packet.starts_with(HANDSHAKE_REQUEST) {
                debug!("Received a packet that's not the Rusty handshake");
                continue;
            }

            info!(
                "Received Rusty handshake begin from {}:{}!",
                src.ip(),
                src.port()
            );

            src.set_port(HANDSHAKE_END_PORT);

            info!("Sending handshake end to {}:{}", src.ip(), src.port());

            socket
                .send_to(&HANDSHAKE_RESPONSE.as_bytes(), &src)
                .await
                .expect("Failed sending response");

            info!("Handshake with {} finished", src.ip());
        }
    })
}
