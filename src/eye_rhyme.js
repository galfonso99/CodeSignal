function solution(pairOfLines) {
    var re = /.*(...)\t.*(...)/;
    var match = re.exec(pairOfLines);
    return match[1] == match[2];
  }