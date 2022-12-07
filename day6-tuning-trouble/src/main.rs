use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let datastream_buffer = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("solution to part 1: {}", chars_to_end_of_marker(datastream_buffer.clone(), 4));
    println!("solution to part 2: {}", chars_to_end_of_marker(datastream_buffer, 14));
}

fn chars_to_end_of_marker(datastream_buffer: String, num_distinct: usize) -> u32{
    datastream_buffer
        .chars()
        .collect::<Vec<char>>()
        .windows(num_distinct)
        .enumerate()
        .find(|(_i, window)| {
            window.iter()
                .enumerate()
                .all(|(i, c)| !window[..i].contains(c))
        }).unwrap()
        .0 as u32 + num_distinct as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chars_to_end_of_packet_marker() {
        let test_map = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];

        test_map
            .iter()
            .for_each(|(k, v)| assert_eq!(chars_to_end_of_marker(k.to_string(), 4), *v as u32));
    }

    #[test]
    fn test_chars_to_end_of_message_marker() {
        let test_map = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];

        test_map
            .iter()
            .for_each(|(k, v)| assert_eq!(chars_to_end_of_marker(k.to_string(), 14), *v as u32));
    }
}

