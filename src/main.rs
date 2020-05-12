use std::net::UdpSocket;

fn print_hex(hex: &[u8]) {
    print!("[");

    for x in 0..hex.len() -1 {
        print!("{:#04x}, ", hex[x]);
    }

    print!("{:#04x}]\n", hex[hex.len() - 1]);
}

fn prepare_answer(query: &[u8]) -> std::vec::Vec<u8> {

    // print_hex(query);
    if query.len() < 13 {
        return vec![0u8, 1]; //Return 0x00
    }

    let mut answer: std::vec::Vec<u8> = Vec::new();

    //Access first two bytes - query[0..2]
    answer.extend_from_slice(&query[0..2]); //Copy the two bytes transaction ID.
    answer.extend_from_slice(&[0x81, 0x80]); //Copy hardcoded flags
    answer.extend_from_slice(&query[4..6]); //Number of questions
    answer.extend_from_slice(&u16::to_be_bytes(1)); //No of answers - ONLY 1
    answer.extend_from_slice(&u32::to_be_bytes(0)); //No nameserver / resource records.
    //Finished the header.

    let mut position = 12;

    while position < query.len() && query[position] != 0x00 {
        position += 1;
    }

    if position + 5 >= query.len() {
        return vec![0u8, 1]; //Return 0x00
    }

    position +=1;
    let qname = &query[12..position];

    let qtype = &query[position..position+2];
    position += 2;

    let qclass = &query[position..position+2];

    answer.extend_from_slice(&[qname, qtype, qclass].concat());
    answer.extend_from_slice(&[qname, qtype, qclass].concat());

    answer.extend_from_slice(&u32::to_be_bytes(1000)); //TTL - 1000 seconds
    answer.extend_from_slice(&u16::to_be_bytes(4));

    let ip: [u8; 4] = [127, 0, 0, 1];
    answer.extend_from_slice(&ip);

    // print_hex(answer.as_slice());

    return answer;
}

fn main(){

    let port = 53; //DNS is on 53
    let socket = UdpSocket::bind(("127.0.0.1", port)).expect("Failed to bind.");

    println!("Server listening on port {}", port);

    loop {

        let mut buf = [0u8; 1024]; 

        match socket.recv_from(&mut buf) {
            Ok ((size, src)) => {
                let answer = prepare_answer(&buf[0..size]);
                socket.send_to(answer.as_slice(), src).unwrap();
            },
            Err (e) => {
                println!("Had a problem receiving data.");
            }
        }
    }
}