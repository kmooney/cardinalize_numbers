static A_TRILLION: u64 = 1_000_000_000_000;
static A_BILLION: u64 = 1_000_000_000;
static A_MILLION: u64 = 1_000_000;
static A_THOUSAND: u64 = 1_000;
static A_HUNDRED: u64 = 100;

pub fn cardinalize_us_en(number: u64) -> String {
    String::from(cardinalize(number).trim())
}

fn cardinalize(number: u64) -> String {
    println!("Number is: {}", number);
    if number % A_TRILLION != number {
        return cardinalize(number / A_TRILLION)
            + " trillion "
            + cardinalize(number % A_TRILLION).as_str();
    }
    if number % A_BILLION != number {
        return cardinalize(number / A_BILLION)
            + " billion "
            + cardinalize(number % A_BILLION).as_str();
    }
    if number % A_MILLION != number {
        return cardinalize(number / A_MILLION)
            + " million "
            + cardinalize(number % A_MILLION).as_str();
    }
    if number % A_THOUSAND != number {
        return cardinalize(number / A_THOUSAND)
            + " thousand "
            + cardinalize(number % A_THOUSAND).as_str();
    }
    if number % A_HUNDRED != number {
        return cardinalize(number / A_HUNDRED)
            + " hundred "
            + cardinalize(number % A_HUNDRED).as_str();
    }
    if number % 90 != number {
        return String::from("ninety ") + cardinalize(number % 90).as_str();
    }
    if number % 80 != number {
        return String::from("eighty ") + cardinalize(number % 80).as_str();
    }
    if number % 70 != number {
        return String::from("seventy ") + cardinalize(number % 70).as_str();
    }
    if number % 60 != number {
        return String::from("sixty ") + cardinalize(number % 60).as_str();
    }
    if number % 50 != number {
        return String::from("fifty ") + cardinalize(number % 50).as_str();
    }
    if number % 40 != number {
        return String::from("forty ") + cardinalize(number % 40).as_str();
    }
    if number % 30 != number {
        return String::from("thirty ") + cardinalize(number % 30).as_str();
    }
    if number % 20 != number {
        return String::from("twenty ") + cardinalize(number % 20).as_str();
    }
    if number == 19 {
        return String::from("nineteen");
    }
    if number == 18 {
        return String::from("eighteen");
    }
    if number == 17 {
        return String::from("seventeen");
    }
    if number == 16 {
        return String::from("sixteen");
    }
    if number == 15 {
        return String::from("fifteen");
    }
    if number == 14 {
        return String::from("fourteen");
    }
    if number == 13 {
        return String::from("thirteen");
    }
    if number == 12 {
        return String::from("twelve");
    }
    if number == 11 {
        return String::from("eleven");
    }
    if number == 10 {
        return String::from("ten");
    }
    if number == 9 {
        return String::from("nine");
    }
    if number == 8 {
        return String::from("eight");
    }
    if number == 7 {
        return String::from("seven");
    }
    if number == 6 {
        return String::from("six");
    }
    if number == 5 {
        return String::from("five");
    }
    if number == 4 {
        return String::from("four");
    }
    if number == 3 {
        return String::from("three");
    }
    if number == 2 {
        return String::from("two");
    }
    if number == 1 {
        return String::from("one");
    }
    return String::from("");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(cardinalize_us_en(1), String::from("one"));
        assert_eq!(cardinalize_us_en(100), String::from("one hundred"));
        assert_eq!(cardinalize_us_en(200), String::from("two hundred"));
        assert_eq!(cardinalize_us_en(202), String::from("two hundred two"));
        assert_eq!(
            cardinalize_us_en(575),
            String::from("five hundred seventy five")
        );
        assert_eq!(
            cardinalize_us_en(8_575),
            String::from("eight thousand five hundred seventy five")
        );
        assert_eq!(
            cardinalize_us_en(1_398_575),
            String::from(
                "one million three hundred ninety eight thousand five hundred seventy five"
            )
        );
        assert_eq!(
            cardinalize_us_en(1_384_949_984),
            String::from(
                "one billion three hundred eighty four million nine hundred forty nine thousand nine hundred eighty four"
            )
        );
        assert_eq!(
            cardinalize_us_en(999_999_999_999_001),
            "nine hundred ninety nine trillion nine hundred ninety nine billion nine hundred ninety nine million nine hundred ninety nine thousand one"
        )
    }
}
