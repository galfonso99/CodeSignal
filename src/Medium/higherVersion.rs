/* Given two version strings composed of several non-negative decimal fields separated by periods ("."), both strings contain equal number of numeric fields. Return true if the first version is higher than the second version and false otherwise.

The syntax follows the regular semver ordering rules:

1.0.5 < 1.1.0 < 1.1.5 < 1.1.10 < 1.2.0 < 1.2.2
< 1.2.10 < 1.10.2 < 2.0.0 < 10.0.0
There are no leading zeros in any of the numeric fields, i.e. you do not have to handle inputs like 100.020.003 (it would instead be given as 100.20.3) */

fn higherVersion(v1: String, v2: String) -> bool {
    let tup = v1.split('.').zip(v2.split('.')).map(|(a,b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
    .find(|t| t.0!=t.1).unwrap_or((0,1));
    tup.0 > tup.1
}
