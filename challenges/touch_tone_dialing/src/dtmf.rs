use hound::WavReader;
use std::f64::consts::PI;

const LOW_1: f64 = 697.0;
const LOW_2: f64 = 770.0;
const LOW_3: f64 = 852.0;
const LOW_4: f64 = 941.0;
const HIGH_1: f64 = 1209.0;
const HIGH_2: f64 = 1336.0;
const HIGH_3: f64 = 1477.0;

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
            (LOW_1, HIGH_1) => Tone::One,
            (LOW_1, HIGH_2) => Tone::Two,
            (LOW_1, HIGH_3) => Tone::Three,
            (LOW_2, HIGH_1) => Tone::Four,
            (LOW_2, HIGH_2) => Tone::Five,
            (LOW_2, HIGH_3) => Tone::Six,
            (LOW_3, HIGH_1) => Tone::Seven,
            (LOW_3, HIGH_2) => Tone::Eight,
            (LOW_3, HIGH_3) => Tone::Nine,
            (LOW_4, HIGH_1) => Tone::Star,
            (LOW_4, HIGH_2) => Tone::Zero,
            (LOW_4, HIGH_3) => Tone::Pound,
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

    let samples: Vec<i16> = reader.samples::<i16>().map(|s| s.unwrap()).collect();
    let splited = split_samples(&samples);

    for stream in splited {
        let stream = preprocess_samples(&stream);
        let c = decode_one(&stream, sample_rate);
        res.push(c);
    }
    let res = res.into_iter().collect::<String>();
    return res;
}


fn split_samples(samples: &Vec<i16>) -> Vec<Vec<i16>> {
    let n = 30;
    let mut result = Vec::new();
    let mut prev = 0;
    let mut i = 0;

    while i < samples.len() {
        if samples[i] == 0 {
            let mut j = i;
            while j < samples.len() && samples[j] == 0 {
                j += 1;
            }

            let zero = j - i;
            if zero >= n {
                if prev < i {
                    result.push(samples[prev..i].to_owned());
                }
                prev = j;
                i = j;
                continue;
            }
        }
        i += 1;
    }

    if prev < samples.len() {
        result.push(samples[prev..].to_owned());
    }

    result
}


fn preprocess_samples(samples: &Vec<i16>) -> Vec<f64> {
    let samples = samples.iter().map(|&s| s as f64).collect::<Vec<f64>>();
    let a = samples[0];
    let b = (samples[samples.len() - 1] - a) / (samples.len() as f64 - 1.);
    let mut res = Vec::with_capacity(samples.len());
    for i in 0..samples.len() {
        res.push(samples[i] - (a + b * (i as f64  - 1.)));
    }
    return res;
}

fn decode_one(samples: &Vec<f64>, sample_rate: u32) -> char {
    let low_freqs: [f64; 4] = [LOW_1, LOW_2, LOW_3, LOW_4];
    let high_freqs: [f64; 3] = [HIGH_1, HIGH_2, HIGH_3];
    let lf = goertzel(samples, sample_rate, &low_freqs);
    let hf = goertzel(samples, sample_rate, &high_freqs);
    Tone::from_freq((lf, hf)).value()
}

fn goertzel(samples: &Vec<f64>, sample_rate: u32, target_freqs: &[f64]) -> f64 {
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
