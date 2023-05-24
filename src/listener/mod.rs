use std::net::TcpListener;

#[allow(dead_code, unused_variables)]
pub fn tcp_listen_string() -> String {

    let mut connected = false;
    
    let tcp_listener = TcpListener::bind("localhost:9999").unwrap();

    for incoming in tcp_listener.incoming() {
        // let result = incoming.unwrap();

        connected = true;
    }
    if connected {
        String::from("Connection established.")
    } else {
        String::from("Connection failure.")
    }

}
