advent_of_code::solution!(24);

use std::io::Write;
use std::process::Command;
use std::process::Stdio;

fn parse_data(input: &str) -> Vec<([i128; 3], [i128; 3])> {
    let mut result = vec![];

    for line in input.lines() {
        let (position_str, velocity_str) = line.split_once('@').unwrap();
        let position = position_str
            .split(',')
            .map(|x| x.trim().parse().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let velocity = velocity_str
            .split(',')
            .map(|x| x.trim().parse().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        result.push((position, velocity));
    }

    result
}

fn line_intersection(
    position1: [i128; 3],
    velocity1: [i128; 3],
    position2: [i128; 3],
    velocity2: [i128; 3],
) -> Option<(i128, i128)> {
    let line1 = (
        (position1[0], position1[1]),
        (position1[0] + velocity1[0], position1[1] + velocity1[1]),
    );

    let line2 = (
        (position2[0], position2[1]),
        (position2[0] + velocity2[0], position2[1] + velocity2[1]),
    );

    let x_diff = ((line1.0 .0 - line1.1 .0), (line2.0 .0 - line2.1 .0));
    let y_diff = ((line1.0 .1 - line1.1 .1), (line2.0 .1 - line2.1 .1));

    fn det(a: (i128, i128), b: (i128, i128)) -> i128 {
        a.0 * b.1 - a.1 * b.0
    }

    let div = det(x_diff, y_diff);
    if div == 0 {
        return None;
    }

    let d = (det(line1.0, line1.1), det(line2.0, line2.1));
    let x = det(d, x_diff) / div;
    let y = det(d, y_diff) / div;

    Some((x, y))
}

pub fn part_one(input: &str) -> Option<u32> {
    let data = parse_data(input);

    let mut result = 0;
    for i in 0..data.len() {
        for j in (i + 1)..data.len() {
            if let Some((x, y)) = line_intersection(data[i].0, data[i].1, data[j].0, data[j].1) {
                if (200000000000000..=400000000000000).contains(&x)
                    && (200000000000000..=400000000000000).contains(&y)
                    && (x - data[i].0[0]) / data[i].1[0] > 0
                    && (x - data[j].0[0]) / data[j].1[0] > 0
                {
                    result += 1;
                }
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let data = parse_data(input);

    const Z3_TEMPLATE: &str = r"
(declare-const x_rock Int)
(declare-const y_rock Int)
(declare-const z_rock Int)
(declare-const vx_rock Int)
(declare-const vy_rock Int)
(declare-const vz_rock Int)
(declare-const t_1 Int)
(declare-const t_2 Int)
(declare-const t_3 Int)
(assert (>= t_1 0))
(assert (>= t_2 0))
(assert (>= t_3 0))
(assert (= (+ @@P_1_X@@ (* t_1 @@V_1_X@@)) (+ x_rock (* t_1 vx_rock))))
(assert (= (+ @@P_1_Y@@ (* t_1 @@V_1_Y@@)) (+ y_rock (* t_1 vy_rock))))
(assert (= (+ @@P_1_Z@@ (* t_1 @@V_1_Z@@)) (+ z_rock (* t_1 vz_rock))))
(assert (= (+ @@P_2_X@@ (* t_2 @@V_2_X@@)) (+ x_rock (* t_2 vx_rock))))
(assert (= (+ @@P_2_Y@@ (* t_2 @@V_2_Y@@)) (+ y_rock (* t_2 vy_rock))))
(assert (= (+ @@P_2_Z@@ (* t_2 @@V_2_Z@@)) (+ z_rock (* t_2 vz_rock))))
(assert (= (+ @@P_3_X@@ (* t_3 @@V_3_X@@)) (+ x_rock (* t_3 vx_rock))))
(assert (= (+ @@P_3_Y@@ (* t_3 @@V_3_Y@@)) (+ y_rock (* t_3 vy_rock))))
(assert (= (+ @@P_3_Z@@ (* t_3 @@V_3_Z@@)) (+ z_rock (* t_3 vz_rock))))
(declare-const result Int)
(assert (= result (+ x_rock y_rock z_rock)))
(check-sat)
(get-value (result))
(exit)
";

    let mut z3_code = String::from(Z3_TEMPLATE);

    fn to_z3_var_string(x: i128) -> String {
        if x < 0 {
            format!("(- {})", x.abs())
        } else {
            x.to_string()
        }
    }

    for (i, item) in data.iter().enumerate().take(4).skip(1) {
        z3_code = z3_code.replace(&format!("@@P_{i}_X@@"), &to_z3_var_string(item.0[0]));
        z3_code = z3_code.replace(&format!("@@P_{i}_Y@@"), &to_z3_var_string(item.0[1]));
        z3_code = z3_code.replace(&format!("@@P_{i}_Z@@"), &to_z3_var_string(item.0[2]));
        z3_code = z3_code.replace(&format!("@@V_{i}_X@@"), &to_z3_var_string(item.1[0]));
        z3_code = z3_code.replace(&format!("@@V_{i}_Y@@"), &to_z3_var_string(item.1[1]));
        z3_code = z3_code.replace(&format!("@@V_{i}_Z@@"), &to_z3_var_string(item.1[2]));
    }

    let mut child = Command::new("z3")
        .arg("-in")
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(z3_code.as_bytes())
        .unwrap();

    let output = child.wait_with_output().unwrap();

    let raw_output = String::from_utf8(output.stdout).unwrap();

    let result = raw_output.split_ascii_whitespace().last().unwrap();
    let result = result[..result.len() - 2].parse().unwrap();

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
