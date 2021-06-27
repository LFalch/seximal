use seximal::to_seximal_words;

fn test(number: &str, pronunciation: &str) {
     assert_eq!(to_seximal_words(number).unwrap().trim_end(), pronunciation);
}
fn test_bad(number: &str) {
     assert!(to_seximal_words(number).is_err());
}

#[test]
fn zero() {
    test("0", "zero");
}
#[test]
fn negative_zero() {
    test("-0", "negative zero");
}
#[test]
fn one_to_twelve() {
    test("1", "one");
    test("2", "two");
    test("3", "three");
    test("4", "four");
    test("5", "five");
    test("10", "six");
    test("11", "seven");
    test("12", "eight");
    test("13", "nine");
    test("14", "ten");
    test("15", "eleven");
    test("20", "twelve");
}

#[test]
fn dozens() {
    test("21", "dozen one");
    test("22", "dozen two");
    test("23", "dozen three");
    test("24", "dozen four");
    test("25", "dozen five");
}

#[test]
fn thirsy_to_fifsy_five() {
    test("31", "thirsy one");
    test("32", "thirsy two");
    test("33", "thirsy three");
    test("34", "thirsy four");
    test("35", "thirsy five");
    test("41", "foursy one");
    test("42", "foursy two");
    test("43", "foursy three");
    test("44", "foursy four");
    test("45", "foursy five");
    test("51", "fifsy one");
    test("52", "fifsy two");
    test("53", "fifsy three");
    test("54", "fifsy four");
    test("55", "fifsy five");
}

#[test]
fn nif_to_115() {
    test("100", "nif");
    test("101", "nif one");
    test("102", "nif two");
    test("103", "nif three");
    test("104", "nif four");
    test("105", "nif five");
    test("110", "nif six");
    test("111", "nif seven");
    test("112", "nif eight");
    test("113", "nif nine");
    test("114", "nif ten");
    test("115", "nif eleven");
}
#[test]
fn nifs() {
    test("100", "nif");
    test("200", "two nif");
    test("300", "three nif");
    test("400", "four nif");
    test("500", "five nif");
    test("1000", "six nif");
    test("1100", "seven nif");
    test("1200", "eight nif");
    test("1300", "nine nif");
    test("1400", "ten nif");
    test("1500", "eleven nif");
}
#[test]
fn exians() {
    // Maybe shouldn't need to say `1`
    test("10000", "one unexian");
    test("100000000", "one biexian");
    test("1000000000000", "one triexian");
}

#[test]
fn leading_zeroes() {
    test("00", "zero");
    test("0000000", "zero");
    test("000000012", "eight");
    test("0000000012", "eight");
}

#[test]
fn nonsense_strings() {
    test_bad("");
    test_bad("æå");
    test_bad("five");
    test_bad("џіз стрің із нөт э нымбэ");
}
