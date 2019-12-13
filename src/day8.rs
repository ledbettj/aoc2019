const INPUT : &'static str = include_str!("../inputs/day8.txt");

pub fn decode_layers(input: &str, w: usize, h: usize) -> Vec<Vec<usize>> {
    let per_layer = w * h;
    let digits : Vec<usize> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).expect("Failed to parse digit") as usize)
        .collect();

    digits[..]
        .chunks(per_layer)
        .map(|chunk|{
            chunk
                .iter()
                .cloned()
                .collect::<Vec<usize>>()
        })
        .collect()
}

pub fn print_layer(layer: &Vec<usize>, w: usize, _h: usize) {
    layer
        .chunks(w)
        .for_each(|chunk|{
            let line : Vec<String> = chunk.iter()
                .map(|c|{
                    match c {
                        1 => "#",
                        _ => " "
                    }.to_string()
                })
                .collect();
            println!("{}", line.join(""));
        });

}

pub fn collapse_layers(layers: &Vec<Vec<usize>>) -> Vec<usize> {
    let len = layers[0].len();
    let mut result = vec![];

    for i in 0..len {
        for j in 0..layers.len() {
            match layers[j][i] {
                2 => {  },
                v => { result.push(v); break },
            };
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_examples() {
        let layers = decode_layers("123456789012", 3, 2);
        println!("{:?}", layers);
    }

    #[test]
    fn p1_solution() {
        let layers = decode_layers(INPUT, 25, 6);

        let l = layers
            .iter()
            .min_by_key(|layer|{
                layer
                    .iter()
                    .filter(|&&v| v == 0)
                    .count()
            })
            .unwrap();

        let ones = l
            .iter()
            .filter(|&&v| v == 1)
            .count();

        let twos = l
            .iter()
            .filter(|&&v| v == 2)
            .count();

        assert_eq!(ones * twos, 1064);
    }

    #[test]
    fn p2_solution() {
        let layers = decode_layers(INPUT, 25, 6);
        let layer = collapse_layers(&layers);

        print_layer(&layer, 25, 6);
    }
}
