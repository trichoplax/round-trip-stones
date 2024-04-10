use clap::Parser;

const INITIAL_STONES: usize = 3;

/// The probability for a code golf challenge
#[derive(Parser)]
#[clap(version, about)]
struct Args {
    /// n, the number of vessels and initial stones per vessel
    #[clap(short)]
    n: usize,
}

#[derive(Debug)]
struct Transfer {
    depart: usize,
    arrive: usize,
}

fn main() {
    let n = Args::parse().n;
    for number in 1..=n {
        print_probabilities(number)
    }
}

fn print_probabilities(n: usize) {
    let first_vessel_departing_stones: Vec<usize> = (0..=INITIAL_STONES).collect();

    let mut transfers: Vec<Transfer> = first_vessel_departing_stones
        .iter()
        .map(|stones| Transfer {
            depart: *stones,
            arrive: *stones,
        })
        .collect();

    for _vessel in 1..n {
        transfers = transfers
            .iter()
            .map(|transfer| {
                (0..=(INITIAL_STONES + transfer.arrive)).map(|stones| Transfer {
                    depart: transfer.depart,
                    arrive: stones,
                })
            })
            .flatten()
            .collect();
    }

    let total_ways = transfers.len();
    let relevant_ways = transfers
        .iter()
        .filter(|transfer| transfer.arrive >= transfer.depart)
        .count();
    println!("{} / {}", relevant_ways, total_ways);

    let probability_with_f64 = relevant_ways as f64 / total_ways as f64;
    let probability_with_f32 = relevant_ways as f32 / total_ways as f32;
    let rounded_probability = (probability_with_f64 * 1_000_000.0).round() / 1_000_000.0;
    println!("{} : {}", n, rounded_probability);
    if (probability_with_f64 - probability_with_f32 as f64).abs() > 0.0000005 {
        println!(
            "WARNING: {} != {}",
            probability_with_f32, probability_with_f64
        );
    }
}
