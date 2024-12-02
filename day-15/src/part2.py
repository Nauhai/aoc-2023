import util
import sys


def process(steps):
    boxes = []
    for i in range(256):
        boxes.append({})

    for step in steps:
        if step.endswith("-"):
            label = step[:-1]
            box = util.hash(label)
            if label in boxes[box]:
                del boxes[box][label]
        
        else:
            label, length = step.split("=")
            box = util.hash(label)
            boxes[box][label] = int(length)

    return compute_power(boxes)


def compute_power(boxes):
    result = 0
    for i in range(len(boxes)):
        for j, v in enumerate(boxes[i].values()):
            power = (i+1)*(j+1)*v
            result += power
    return result


if __name__ == "__main__":
    with open(sys.argv[1], "r") as file:
        steps = util.parse_steps(file.read())
        print("Part 2:", process(steps))
