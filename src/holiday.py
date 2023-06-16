from calendar import monthrange
from datetime import datetime
import time
def solution(x, weekDay, month, year):
    month_max = monthrange(year, datetime.strptime(month, '%B').month)[1]
    first_day = (7 * (x - 1)) + 1
    if first_day > month_max:
        return -1
    date = datetime.strptime("{} {} {}".format(month, first_day, year), '%B %d %Y')
    first_weekday = date.isoweekday()
    day = first_day + (time.strptime(weekDay, "%A").tm_wday + 1 - first_weekday) % 7
    return day if day <= month_max else -1