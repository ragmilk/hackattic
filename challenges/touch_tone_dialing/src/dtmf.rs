use hound::WavReader;
use std::f64::consts::PI;

enum Tone {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Star,
    Zero,
    Pound,
}

#[allow(dead_code)]
impl Tone {
    fn value(&self) -> char {
        match self {
            Tone::One => '1',
            Tone::Two => '2',
            Tone::Three => '3',
            Tone::Four => '4',
            Tone::Five => '5',
            Tone::Six => '6',
            Tone::Seven => '7',
            Tone::Eight => '8',
            Tone::Nine => '9',
            Tone::Star => '*',
            Tone::Zero => '0',
            Tone::Pound => '#',
        }
    }

    fn from_freq(pair: (f64, f64)) -> Self {
        match pair {
            (697.0, 1209.0) => Tone::One,
            (697.0, 1336.0) => Tone::Two,
            (697.0, 1477.0) => Tone::Three,
            (770.0, 1209.0) => Tone::Four,
            (770.0, 1336.0) => Tone::Five,
            (770.0, 1477.0) => Tone::Six,
            (852.0, 1209.0) => Tone::Seven,
            (852.0, 1336.0) => Tone::Eight,
            (852.0, 1477.0) => Tone::Nine,
            (941.0, 1209.0) => Tone::Star,
            (941.0, 1336.0) => Tone::Zero,
            (941.0, 1477.0) => Tone::Pound,
            _ => panic!("Invalid frequency pair"),
        }
    }
}

pub fn decode(file_path: &str) -> String {
    let mut res: Vec<char> = Vec::new();
    let mut reader = WavReader::open(file_path).unwrap();
    let spec = reader.spec();
    println!("{spec:?}");
    let sample_rate = spec.sample_rate;

    let samples: Vec<i32> = reader.samples::<i32>().map(|s| s.unwrap()).collect();
    let splited = split_samples(&samples, sample_rate);
    let len = splited.len();
    println!("num: {len}, sample_len: {}", samples.len());
    println!("sample_len % num: {}, sample_len / num: {}", samples.len() % len, samples.len() / len);

    for stream in splited {
        let c = decode_one(&stream, sample_rate);
        res.push(c);
    }

    let res = res.into_iter().collect::<String>();
    return res;
}


//TODO: split proper
fn split_samples(samples: &Vec<i32>, _sample_rate: u32) -> Vec<Vec<i32>> {
    return samples.split(|&s| s == 0)
        .map(|s| s.to_vec())
        .filter(|s| !s.is_empty())
        .collect();
}


fn decode_one(samples: &Vec<i32>, sample_rate: u32) -> char {
    let low_freqs: [f64; 4] = [697.0, 770.0, 852.0, 941.0];
    let high_freqs: [f64; 3] = [1209.0, 1336.0, 1477.0];
    let lf = goertzel(samples, sample_rate, &low_freqs);
    let hf = goertzel(samples, sample_rate, &high_freqs);
    Tone::from_freq((lf, hf)).value()
}

fn goertzel(samples: &Vec<i32>, sample_rate: u32, target_freqs: &[f64]) -> f64 {
    let sample_rate = sample_rate as f64;
    let mut res = 0.0;
    let mut max_power = 0.0;
    for &target in target_freqs {
        let omega = 2.0 * PI * target / sample_rate;
        let coeff = 2.0 * omega.cos();
        let (mut q1, mut q2) = (0.0, 0.0);
        for &sample in samples {
            let sample = sample as f64;
            let q0 = coeff * q1 - q2 + sample;
            q2 = q1;
            q1 = q0;
        }
        let power = q1.powi(2) + q2.powi(2) - coeff * q1 * q2;
        if power > max_power {
            max_power = power;
            res = target;
        }
    }
    return res;
}
