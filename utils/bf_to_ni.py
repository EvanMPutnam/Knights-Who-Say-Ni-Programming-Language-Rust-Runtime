import sys
from pathlib import Path


def convert_char_to_ni_language(char):
    if char == ">": return "Ni"
    if char == "<": return "ni"
    if char == "-": return "ni!"
    if char == "+": return "Ni!"
    if char == "[": return "Niii"
    if char == "]": return "niii"
    if char == ",": return "nii"
    if char == ".": return "Nii"
    return ""


def parse_bf_file(file_path):
    if not file_path.endswith(".b"):
        raise Exception("Invalid bf extension.  Needs to end with .b extension.")
    ni_text = ""
    with open(file_path) as fp:
        for line in fp.readlines():
            for char in line:
                ni_text += (convert_char_to_ni_language(char) + " ")
            ni_text += "\n"
    return ni_text


def write_ni_text_to_file(ni_text, file_path):
    with open(file_path, "w+") as fp:
        fp.write(ni_text)


def main():
    args = sys.argv
    print(args)
    if len(args) != 2:
        raise Exception("Invalid number of arguments.  Only provide a path file.")
    file_arg = args[1]
    ni_text = parse_bf_file(file_arg)
    ni_path = Path(file_arg)
    ni_path = ni_path.stem + ".ni"
    write_ni_text_to_file(ni_text, ni_path)


if __name__ == "__main__":
    main()
