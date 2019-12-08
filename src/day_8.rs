use anyhow::Error;
use ndarray::{Array, Array2, Array3};

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn get_image(input: &str) -> Result<Array3<u32>, Error> {
    let num_layers = input.len() / (WIDTH * HEIGHT);
    Ok(Array::from(
        input
            .chars()
            .map(|c| c.to_digit(10).ok_or_else(|| anyhow!("char is not digit")))
            .collect::<Result<Vec<_>, Error>>()?,
    )
        .into_shape((num_layers, HEIGHT, WIDTH))?)
}

pub fn solve_day_8_1(input: &str) -> Result<u32, Error> {
    let image = get_image(input)?;
    let min_zero_layer = (0..image.shape()[0])
        .map(|i| {
            (
                image
                    .slice(s![i, .., ..])
                    .iter()
                    .filter(|x| **x == 0)
                    .count(),
                i,
            )
        })
        .min_by(|a, b| a.0.cmp(&b.0))
        .unwrap()
        .1;
    let (mut ones, mut twos) = (0, 0);
    for x in image.slice(s![min_zero_layer, .., ..]).iter() {
        if *x == 1 {
            ones += 1;
        } else if *x == 2 {
            twos += 1;
        }
    }
    Ok(ones * twos)
}

fn plot(array: &Array2<u32>) -> String {
    (0..array.shape()[0])
        .flat_map(|i| {
            (0..array.shape()[1])
                .map(move |j| if array[(i, j)] == 0 { ' ' } else { '█' })
                .chain(vec!['\n'].into_iter())
        })
        .collect()
}

pub fn solve_day_8_2(input: &str) -> Result<String, Error> {
    let image = get_image(input)?;
    let mut visible = Array2::zeros((HEIGHT, WIDTH));
    visible.fill(2);
    for layer in 0..image.shape()[0] {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                if visible[(i, j)] == 2 && image[(layer, i, j)] != 2 {
                    visible[(i, j)] = image[(layer, i, j)]
                }
            }
        }
    }
    Ok(plot(&visible))
}
