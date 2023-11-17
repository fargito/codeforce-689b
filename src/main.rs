use std::{cmp::min, io};

fn main() -> io::Result<()> {
    let mut first_line = String::new();
    let mut second_line = String::new();
    io::stdin().read_line(&mut first_line)?;
    io::stdin().read_line(&mut second_line)?;

    let res = handle(first_line, second_line);

    println!("{res}");

    Ok(())
}

fn handle(first_line: String, second_line: String) -> String {
    // in the first line, we have the number of streets
    let streets_count: usize = first_line
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .next()
        .unwrap();

    let shortcuts: Vec<usize> = second_line
        .split_whitespace()
        .map(|c| c.parse().unwrap())
        .collect();

    // initialize result array
    let mut energies: Vec<usize> = Vec::new();

    // initialize the intersections array
    let mut intersections: Vec<usize> = Vec::new();

    // TODO cleaner init
    for i in 0..streets_count {
        energies.push(usize::MAX - 1);
        intersections.push(streets_count - 1 - i);
    }

    // algo
    // for each intersection
    while let Some(i) = intersections.pop() {
        // first set the energy at the current position
        // special case if O or last element
        let current_energy = if i == 0 {
            0
        } else if i == streets_count - 1 {
            min(energies[i], energies[i - 1] + 1)
        } else {
            let current_energy = min(energies[i], min(energies[i - 1] + 1, energies[i + 1] + 1));

            // if greater than previous, put the previous index back for analysis
            if current_energy < energies[i - 1] + 1 {
                intersections.push(i - 1);
            }

            current_energy
        };
        energies[i] = current_energy;

        // jump to the shortcut and set its new value
        let shortcut_destination = shortcuts[i] - 1;

        let new_shortcut_destination_energy =
            min(current_energy + 1, energies[shortcut_destination]);

        energies[shortcut_destination] = new_shortcut_destination_energy;
    }

    energies
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use crate::handle;

    #[test]
    fn test_1() {
        let res = handle("3".to_string(), "2 2 3".to_string());

        assert_eq!(res, "0 1 2");
    }

    #[test]
    fn test_2() {
        let res = handle("5".to_string(), "1 2 3 4 5".to_string());

        assert_eq!(res, "0 1 2 3 4");
    }

    #[test]
    fn test_3() {
        let res = handle("7".to_string(), "4 4 4 4 7 7 7".to_string());

        assert_eq!(res, "0 1 2 1 2 3 3");
    }
}
