#! /usr/bin/env python3

import sys

# our rust library
import calc


def main():
    _content = sys.stdin.read().strip()

    # do necessary data parsing here?
    content = _content.strip().split("\n")
    # result = calc.calculate(content)
    result = calc.p2(content)
    print(result)


if __name__ == "__main__":
    main()
