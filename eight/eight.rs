use std::fs::read;

fn main() {

    let raw_input = match read("input.txt") {
        Err(_) => panic!("Can't read input.txt!"),
        Ok(file) => file,
    };

    let input_string = String::from_utf8_lossy(&raw_input);
    let layer_size = 25 * 6;

    let mut fewest_zeros = None;
    let mut ones_times_twos = 0;
    let mut rendered_image = String::new();

    for layer_index in 0..(input_string.len() / layer_size) {
        let layer = &input_string[(layer_index * layer_size)..((layer_index + 1) * layer_size)];

        rendered_image = stack_layers(rendered_image, String::from(layer));

        let zeros = layer.chars()
            .filter(|chr| *chr == '0')
            .count();
        
        if fewest_zeros.is_none() || zeros < fewest_zeros.unwrap() {
            fewest_zeros = Some(zeros);

            let ones_count = layer.chars().filter(|chr| *chr == '1').count();
            let twos_count = layer.chars().filter(|chr| *chr == '2').count();

            ones_times_twos = ones_count * twos_count;
        }
    }

    println!("Fewest zeros: {}, ones*twos: {}", fewest_zeros.unwrap(), ones_times_twos);
    print_image(rendered_image, 25);
}

fn stack_layers(top: String, bottom: String) -> String {
    if top.is_empty() {
        return bottom.clone();
    }

    let mut together = String::new();

    for pixels in top.chars().zip(bottom.chars()) {
        together.push(match pixels.0 {
            '2' => pixels.1,
            other => other,
        });
    }

    return together;
}

fn print_image(image: String, row_size: usize) {
    for row_index in 0..(image.len() / row_size) {
        let row = &image[(row_index * row_size)..((row_index + 1) * row_size)];
        println!("{}", row.replace("0", " ").replace("1", "$"));
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_stack() {
        let top = String::from("2222210220122222");
        let bot = String::from("0000000011111111");

        assert_eq!(stack_layers(top, bot), String::from("0000010010111111"));
    }

    #[test]
    fn test_empty() {
        let top = String::new();
        let bot = String::from("0000000011111111");

        assert_eq!(stack_layers(top, bot), String::from("0000000011111111"));
    }
}