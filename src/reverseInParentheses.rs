/* Write a function that reverses characters in (possibly nested) parentheses in the input string.

Input strings will always be well-formed with matching ()s.

Example

For inputString = "(bar)", the output should be
reverseInParentheses(inputString) = "rab";
For inputString = "foo(bar)baz", the output should be */

fn reverseInParentheses(inputString: String) -> String {
    let mut input = inputString.chars().collect::<Vec<_>>();
    let mut stack = Vec::new();
    
    for i in 0..inputString.len() {
        match input[i] {
            '(' => stack.push(i),
            ')' => input[stack.pop().unwrap() + 1..i].reverse(),
            _ => {}
        }
    }
    
    input.into_iter().filter(|&c| c != '(' && c != ')').collect()
}
