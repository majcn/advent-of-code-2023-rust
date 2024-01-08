advent_of_code::solution!(24);

use advent_of_code::maneatingape::iter::*;
use advent_of_code::maneatingape::parse::*;

struct Hailstone {
    x: i64,
    y: i64,
    z: i64,
    vx: i64,
    vy: i64,
    vz: i64,
}

fn parse_data(input: &str) -> Vec<Hailstone> {
    let mut result = vec![];

    for line in input.lines() {
        let (position_str, velocity_str) = line.split_once('@').unwrap();
        let position = position_str.iter_signed().chunk::<3>().next().unwrap();
        let velocity = velocity_str.iter_signed().chunk::<3>().next().unwrap();

        result.push(Hailstone {
            x: position[0],
            y: position[1],
            z: position[2],
            vx: velocity[0],
            vy: velocity[1],
            vz: velocity[2],
        });
    }

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let mut result = 0;
    for i in 0..data.len() {
        for j in (i + 1)..data.len() {
            let x1 = data[i].x;
            let x2 = data[i].x + data[i].vx;
            let x3 = data[j].x;
            let x4 = data[j].x + data[j].vx;

            let y1 = data[i].y;
            let y2 = data[i].y + data[i].vy;
            let y3 = data[j].y;
            let y4 = data[j].y + data[j].vy;

            let det = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
            if det == 0 {
                continue;
            }

            let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / det;
            let u = ((x1 - x3) * (y1 - y2) - (y1 - y3) * (x1 - x2)) / det;

            let x = x1 + t * (x2 - x1);
            let y = y1 + t * (y2 - y1);

            if (200000000000000..=400000000000000).contains(&x)
                && (200000000000000..=400000000000000).contains(&y)
                && t > 0
                && u > 0
            {
                result += 1;
            }
        }
    }

    Some(result)
}

fn gaussian_elimination<const N: usize>(
    coefficients: &mut [[f64; N]; N],
    rhs: &mut [f64; N],
) -> [f64; N] {
    for p in 0..N {
        let mut max_i = p;
        for i in p + 1..N {
            if coefficients[i][p].abs() > coefficients[max_i][p].abs() {
                max_i = i;
            }
        }

        coefficients.swap(p, max_i);
        rhs.swap(p, max_i);

        for i in p + 1..N {
            let ratio = coefficients[i][p] / coefficients[p][p];
            for j in p..N {
                coefficients[i][j] -= coefficients[p][j] * ratio;
            }
            rhs[i] -= rhs[p] * ratio;
        }
    }

    let mut resut = *rhs;
    for i in (0..N).rev() {
        let s = (i + 1..N)
            .map(|j| coefficients[i][j] * resut[j])
            .sum::<f64>();
        resut[i] = (resut[i] - s) / coefficients[i][i];
    }
    resut
}

pub fn part_two(input: &str) -> Option<u64> {
    let data = parse_data(input);

    let mut coefficients = [[0f64; 4]; 4];
    let mut rhs = [0f64; 4];
    for i in 0..4 {
        let h1 = &data[i];
        let h2 = &data[i + 1];

        coefficients[i][0] = (h2.vy - h1.vy) as f64;
        coefficients[i][1] = (h1.y - h2.y) as f64;
        coefficients[i][2] = (h1.vx - h2.vx) as f64;
        coefficients[i][3] = (h2.x - h1.x) as f64;
        rhs[i] = (h1.y * h1.vx - h1.x * h1.vy + h2.x * h2.vy - h2.y * h2.vx) as f64;
    }
    let [rock_x, rock_vx, rock_y, ..] =
        gaussian_elimination(&mut coefficients, &mut rhs).map(|x| x.round() as i64);

    let mut coefficients = [[0f64; 2]; 2];
    let mut rhs = [0f64; 2];
    for i in 0..2 {
        let h1 = &data[i];

        let t = (h1.x - rock_x) as f64 / (rock_vx - h1.vx) as f64;

        coefficients[i][0] = 1f64;
        coefficients[i][1] = t;
        rhs[i] = h1.z as f64 + t * h1.vz as f64;
    }
    let [rock_z, ..] = gaussian_elimination(&mut coefficients, &mut rhs).map(|x| x.round() as i64);

    let result = rock_x + rock_y + rock_z;
    let result = result as u64;

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(47));
    }
}
