use base64::prelude::*;
use celes::Country;
use core::str::FromStr;
use openssl::{
    asn1::{Asn1Integer, Asn1Time},
    bn::BigNum,
    hash::MessageDigest,
    pkey::PKey,
    rsa::Rsa,
    x509::extension::{BasicConstraints, KeyUsage, SubjectAlternativeName, SubjectKeyIdentifier},
    x509::{X509, X509Name},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
struct Output {
    certificate: String,
}

#[derive(Deserialize, Debug)]
struct RequiredData {
    domain: String,
    serial_number: String,
    country: String,
}

#[derive(Deserialize, Debug)]
struct Input {
    private_key: String,
    required_data: RequiredData,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_data = util::get_problem::<Input>("tales_of_ssl").await?;

    let decoded_private_key = BASE64_STANDARD.decode(json_data.private_key.as_bytes())?;
    let rsa_key = Rsa::private_key_from_der(&decoded_private_key)?;
    let pkey = PKey::from_rsa(rsa_key)?;

    let mut subject_name_builder = X509Name::builder()?;
    let country_code = get_country_code(json_data.required_data.country);
    subject_name_builder.append_entry_by_text("C", &country_code)?;
    subject_name_builder.append_entry_by_text("CN", &json_data.required_data.domain)?;
    let subject_name = subject_name_builder.build();

    let mut builder = X509::builder()?;
    builder.set_version(2)?;

    let serial_hex = json_data
        .required_data
        .serial_number
        .strip_prefix("0x")
        .unwrap_or(&json_data.required_data.serial_number);
    let serial_bn = BigNum::from_hex_str(serial_hex)?;
    let serial_asn1 = Asn1Integer::from_bn(&serial_bn)?;
    builder.set_serial_number(&serial_asn1)?;

    builder.set_not_before(&Asn1Time::days_from_now(0).unwrap())?;
    builder.set_not_after(&Asn1Time::days_from_now(365).unwrap())?;
    builder.set_pubkey(&pkey)?;
    builder.set_subject_name(&subject_name)?;
    builder.set_issuer_name(&subject_name)?;

    let basic_constraints = BasicConstraints::new().ca().build()?;
    let key_usage = KeyUsage::new().digital_signature().key_encipherment().build()?;

    let context = builder.x509v3_context(None, None);
    let subj_key_identifier = SubjectKeyIdentifier::new().build(&context)?;

    let mut san = SubjectAlternativeName::new();
    san.dns(&json_data.required_data.domain);
    let san_extension = san.build(&context)?;

    builder.append_extension(basic_constraints)?;
    builder.append_extension(key_usage)?;
    builder.append_extension(subj_key_identifier)?;
    builder.append_extension(san_extension)?;

    builder.sign(&pkey, MessageDigest::sha256())?;

    let cert = builder.build();
    let cert_der = cert.to_der()?;
    let certificate = BASE64_STANDARD.encode(&cert_der);
    let result = Output { certificate };
    util::post_answer::<Output>("tales_of_ssl", result, false).await?;
    Ok(())
}

// There're some irregular, like "Tokelau Island" does not exist by default, while "Tokelau" does.
fn get_country_code(coutry: String) -> String {
    let v: Vec<String> = coutry.split_whitespace().map(str::to_string).collect();
    let all = v.iter().fold(String::new(), |acc, s| format!("{acc}{s}"));
    if let Ok(country_code) = Country::from_str(all.as_str()) {
        return country_code.alpha2.to_string();
    }
    for k in v {
        if let Ok(country_code) = Country::from_str(k.as_str()) {
            return country_code.alpha2.to_string();
        }
    }

    println!("Country not found");
    "XX".to_string()
}
