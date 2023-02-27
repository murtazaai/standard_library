// use std::{net::TcpListener, io::{Read, Write}, fs};

// pub fn tcp_listener() {
//     let listener = TcpListener::bind("127.0.0.1:8787").unwrap();

//     for stream in listener.incoming() {
        
//         let mut tcp_stream = stream.unwrap();

//         // let buf = BufReader::new(tcp_stream);

//         let mut http_request_buf = String::new();
        
//         // let http_request = 
//         let result = tcp_stream
//                                             .read_to_string(&mut http_request_buf);
//         // Error handling
//         if result.is_err() {
//             // let buf = result.err().unwrap().to_string();

//             // let buf_as_bytes = buf.as_bytes();

//             // tcp_stream.write_all(buf_as_bytes);

//             let http_error_message = String::from("HTTP Error Code: 999");

//             let error_message = http_error_message.as_bytes();

//             let buf = error_message;

//             tcp_stream.write_all(buf);
//         }
//                     // lines()
//                     // .map(|f| {
//                     //     f.unwrap()
//                     // }).filter(|f| { 
//                     //     !f.is_empty()})
//                     // .collect();
        
//         let expected_http_request = vec![
//             "GET / HTTP/1.1",
//             "Host: 127.0.0.1:8787",
//             "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:99.0) Gecko/20100101 Firefox/99.0",
//             "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
//             "Accept-Language: en-US,en;q=0.5",  
//             "Accept-Encoding: gzip, deflate, br",
//             "DNT: 1",
//             "Connection: keep-alive",
//             "Upgrade-Insecure-Requests: 1",
//             "Sec-Fetch-Dest: document",
//             "Sec-Fetch-Mode: navigate",
//             "Sec-Fetch-Site: none",
//             "Sec-Fetch-User: ?1",
//             "Cache-Control: max-age=0",
//         ];

//         // assert_eq!(http_request_buf, expected_http_request);

//         let status_line = "HTTP/1.1 200 OK".to_string();

//         let path = "../ipfs/index.html".to_string();

//         let content = fs::read_to_string(path).unwrap();

//         let len = content.len();
        
//         // let response = format!("{}, {}, {}", &status_line, content, len).as_bytes();
        
//         // let x = tcp_stream.write_all(response);
//     }
// }