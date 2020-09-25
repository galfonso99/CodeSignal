/* Correct variable names consist only of English letters, digits and underscores and they can't start with a digit.

Check if the given string is a correct variable name.

Example

For name = "var_1__Int", the output should be
variableName(name) = true;
For name = "qq-q", the output should be
variableName(name) = false;
For name = "2w2", the output should be
variableName(name) = false. */

fn variableName(name: String) -> bool {
    name.chars().all(|c| c.is_alphanumeric() || c == '_')
    && !name.chars().next().unwrap().is_numeric()
}
