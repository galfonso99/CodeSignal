/* Consider two following representations of a non-negative integer:

A simple decimal integer, constructed of a non-empty sequence of digits from 0 to 9;
An integer with at least one digit in a base from 2 to 16 (inclusive), enclosed between # characters, and preceded by the base, which can only be a number between 2 and 16 in the first representation. For digits from 10 to 15 characters a, b, ..., f and A, B, ..., F are used.
Additionally, both representations may contain underscore (_) characters; they are used only as separators for improving legibility of numbers and can be ignored while processing a number.

Your task is to determine whether the given string is a valid integer representation. 

For line = "123_456_789", the output should be
adaNumber(line) = true;
For line = "16#123abc#", the output should be
adaNumber(line) = true;  */

fn adaNumber(mut s: String) -> bool {
    if s.replace("_", "").parse::<u128>().is_ok() {true}
    else if s.matches('#').count() == 2 {
        let radix = s.chars().take_while(|&c| c!='#').filter(|&c| c!='_').collect::<String>().parse::<i32>();
        let digits = s.chars().skip_while(|&c| c!='#').skip(1)
        .take_while(|&c| c!='#').filter(|&c| c!='_').collect::<String>();
        if radix.is_ok() && !digits.is_empty() && radix.as_ref().unwrap() > &1 && radix.as_ref().unwrap() < &17 {
            digits.chars().all(|d| d.to_digit(*radix.as_ref().unwrap() as u32).is_some())
        }
        else {false}
    }
    else {false}
}