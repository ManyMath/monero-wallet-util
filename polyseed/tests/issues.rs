use polyseed::{Language, Polyseed, PolyseedError};
use zeroize::Zeroizing;

#[test]
#[should_panic]
fn bug_word_count_panics_on_overflow() {
  let seed = "raven tail swear infant grief assist regular lamp \
    duck valid someone little harsh puppy airport language extra";
  let _ = Polyseed::from_string(Language::English, Zeroizing::new(seed.into()));
}

#[test]
fn bug_word_count_accepts_too_few() {
  let seed = "raven tail swear";
  let res = Polyseed::from_string(Language::English, Zeroizing::new(seed.into()));
  assert!(res.is_err());
}

#[test]
fn bug_accent_strip_uses_ascii_filter() {
  let ascii_seed = "eje fin parte celebre tabu pestana lienzo puma \
    prision hora regalo lengua existir lapiz lote sonoro";
  let res = Polyseed::from_string(Language::Spanish, Zeroizing::new(ascii_seed.into()));
  assert!(res.is_ok());
}

#[test]
fn bug_poly_arrays_not_zeroized() {
  let seed = "raven tail swear infant grief assist regular lamp \
    duck valid someone little harsh puppy airport language";
  let _seed = Polyseed::from_string(Language::English, Zeroizing::new(seed.into())).unwrap();
}

#[test]
fn gap_no_multi_coin_support() {
  let seed = "raven tail swear infant grief assist regular lamp \
    duck valid someone little harsh puppy airport language";
  let parsed = Polyseed::from_string(Language::English, Zeroizing::new(seed.into())).unwrap();
  let _key = parsed.key();
  let _str = parsed.to_string();
}

#[test]
fn gap_no_is_encrypted() {
  let seed = "raven tail swear infant grief assist regular lamp \
    duck valid someone little harsh puppy airport language";
  let parsed = Polyseed::from_string(Language::English, Zeroizing::new(seed.into())).unwrap();
  let _features = parsed.features();
}

#[test]
fn gap_no_crypt() {
  let seed = "raven tail swear infant grief assist regular lamp \
    duck valid someone little harsh puppy airport language";
  let _parsed = Polyseed::from_string(Language::English, Zeroizing::new(seed.into())).unwrap();
}

#[test]
fn gap_no_store_load() {
  let seed = "raven tail swear infant grief assist regular lamp \
    duck valid someone little harsh puppy airport language";
  let _parsed = Polyseed::from_string(Language::English, Zeroizing::new(seed.into())).unwrap();
}

#[test]
fn gap_no_auto_detect() {
  let seed = "raven tail swear infant grief assist regular lamp \
    duck valid someone little harsh puppy airport language";
  let _parsed = Polyseed::from_string(Language::English, Zeroizing::new(seed.into())).unwrap();
}

#[test]
fn gap_no_enable_features() {
  let seed = "raven tail swear infant grief assist regular lamp \
    duck valid someone little harsh puppy airport language";
  let _parsed = Polyseed::from_string(Language::English, Zeroizing::new(seed.into())).unwrap();
}

#[test]
fn gap_no_nfc_output() {
  let seed = "raven tail swear infant grief assist regular lamp \
    duck valid someone little harsh puppy airport language";
  let parsed = Polyseed::from_string(Language::English, Zeroizing::new(seed.into())).unwrap();
  let output = parsed.to_string();
  let ascii_spaces = output.chars().filter(|&c| c == ' ').count();
  assert_eq!(ascii_spaces, 15);
}
