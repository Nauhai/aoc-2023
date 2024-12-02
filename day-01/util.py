import re


def get_calibration_value(line, pattern):
    matches = re.findall(pattern, line)
    digits = list(map(to_digit, matches))
    
    return digits[0]*10 + digits[-1]


def to_digit(str):
    if len(str) == 1:
        return int(str)
    
    return ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].index(str)
