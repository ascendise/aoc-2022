mod signal_processor;

fn main() {
    let stream = std::fs::read_to_string("stream.txt").unwrap();
    let start_of_packet_index = signal_processor::get_start_index(&stream, 4);
    println!("Start of packet is at {start_of_packet_index}");
    let start_of_message_index = signal_processor::get_start_index(&stream, 14);
    println!("Start of message is at {start_of_message_index}");

}
