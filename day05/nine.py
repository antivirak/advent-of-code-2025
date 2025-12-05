

def main_2() -> int:
    with open("test_input.txt", "r") as f:
        db = f.readlines()
    delim_pos = db.index("\n")
    ranges = map(
        lambda x: tuple(map(int, x.split("-"))),
        db[:delim_pos]
    )

    return 42


if __name__ == "__main__":
    print(main_2())
