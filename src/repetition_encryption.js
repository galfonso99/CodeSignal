function solution(letter) {
    var pattern = /([a-zA-Z]+)[^a-zA-Z]+(\1)\b/gmi;
    var matches = letter.match(pattern);
    if (matches) {
      return matches.length;
    }
    else {
      return 0;
    }
  }