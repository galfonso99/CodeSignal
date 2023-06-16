function solution(code, args) {
    var argumentVariants = args.join('|');
    var re = new RegExp(`(?<![\\w\\$])(${argumentVariants})\\b`,"g");
    var sub = `$$$1`;
    return code.replace(re, sub);
}