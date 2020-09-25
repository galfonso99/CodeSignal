/* Given a string, your task is to replace each of its characters by the next one in the English alphabet; i.e. replace a with b, replace b with c, etc (z would be replaced by a).

Example

For inputString = "crazy", the output should be alphabeticShift(inputString) = "dsbaz". */

fn alphabeticShift(s: String) -> String {
    s.chars().map(|c| if c=='z' {'a'} else {(c as u8 + 1) as char}).collect()
}