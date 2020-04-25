use std::fmt::{Display, Error, Formatter};

pub fn step1(input: &str) {
    let layers = parse_img(input, 25, 6);
    let min0layer = layers.iter().min_by_key(|l| l.count_char('0')).unwrap();
    println!("{}", min0layer.count_char('1') * min0layer.count_char('2'));
}
pub fn step2(input: &str) {
    let layers = parse_img(input, 25, 6);
    let final_img = merge_layers(&layers);
    println!("{}", final_img);
}

#[derive(Clone)]
struct Layer {
    rows: Vec<Vec<char>>,
}

impl Layer {
    fn count_char(&self, c: char) -> usize {
        self.rows
            .iter()
            .map(|row| row.iter().filter(|&&c0| c0 == c).count())
            .sum()
    }
}

impl Display for Layer {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for row in &self.rows {
            for col in row {
                match col {
                    '0' => write!(f, "#")?,
                    '1' => write!(f, ".")?,
                    '2' => write!(f, " ")?,
                    _ => write!(f, "?")?,
                };
            }
            writeln!(f)?;
        }
        Result::Ok(())
    }
}

fn parse_img(input: &str, w: usize, h: usize) -> Vec<Layer> {
    let mut current_row = Vec::new();
    let mut current_layer_rows = Vec::new();
    let mut layers = Vec::new();

    for c in input.chars() {
        current_row.push(c);

        if current_row.len() == w {
            current_layer_rows.push(current_row);
            current_row = Vec::new();
        }
        if current_layer_rows.len() == h {
            layers.push(Layer {
                rows: current_layer_rows,
            });
            current_layer_rows = Vec::new();
        }
    }

    layers
}

fn merge_layers(layers: &[Layer]) -> Layer {
    let mut dest = layers[0].clone();
    for i in 0..dest.rows.len() {
        for j in 0..dest.rows[0].len() {
            let mut l = 0;
            while dest.rows[i][j] == '2' && l < layers.len() {
                l += 1;
                dest.rows[i][j] = layers[l].rows[i][j];
            }
        }
    }
    dest
}
