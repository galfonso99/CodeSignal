/* An IP address is a numerical label assigned to each device (e.g., computer, printer) participating in a computer network that uses the Internet Protocol for communication. There are two versions of the Internet protocol, and thus two versions of addresses. One of them is the IPv4 address.

Given a string, find out if it satisfies the IPv4 address naming rules.

Example

For inputString = "172.16.254.1", the output should be
isIPv4Address(inputString) = true; */

fn isIPv4Address(mut s: String) -> bool {
    let mut ls :Vec<String> = s.split(".").map(|s| s.to_string()).collect();
    if ls.len() !=4 {return false}
    let mut cnt = 0;
    for i in ls {
        if i.parse::<u8>().is_ok() && i.parse::<u8>().unwrap().to_string() == i {
            cnt+=1;
        }
    }
    cnt == 4
    
}