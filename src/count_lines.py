import glob
import argparse


def is_preamble(line):
    if line.startswith("pub mod"):
        return True
    if line.startswith("mod"):
        return True
    elif line.startswith("use"):
        return True
    elif line.startswith("extern"):
        return True
    elif line.startswith("pub(crate) mod"):
        return True
    elif line.startswith("pub(crate) use"):
        return True
    return False


def is_prusti_annotation(annotation):
    if annotation[2:6] == "pure":
        return True
    elif annotation[2:9] == "trusted":
        return True
    elif annotation[2:9] == "ensures":
        return True
    elif annotation[2:10] == "requires":
        return True
    return False


def is_flux_annotation(annotation):
    return "flux::" in annotation


def count_files(pattern):
    counts = {
        "loc": 0,
        "annot": 0,
        "spec": 0,
    }

    for file in glob.iglob(pattern, recursive=True):
        counts = {k: v + counts[k] for (k, v) in count_file(file).items()}

    return counts


def count_file(path):
    file = open(path, 'r')
    lines = file.readlines()

    counts = {
        "loc": 0,
        "annot": 0,
        "spec": 0,
    }

    line_number = 0
    in_contract = False
    in_body_invariant = False
    in_predicate = False
    predicate_ident = 0

    # Strips the newline character
    for line in lines:
        line_number += 1
        stripped = line.strip()
        ident = len(line.rstrip()) - len(stripped)
        if stripped:
            if (stripped[0] != "/" or stripped[1] != "/"
                ) and stripped[:16] != "pub fn main() {}" and not is_preamble(
                    stripped):
                if stripped[0] == "#":
                    if is_prusti_annotation(stripped) or is_flux_annotation(
                            stripped):
                        counts['spec'] = counts['spec'] + 1
                        in_contract = True

                        if stripped.endswith("]"):
                            in_contract = False

                elif in_contract:
                    counts['spec'] += 1
                    if stripped.endswith(")]"):
                        in_contract = False

                elif stripped.startswith("predicate!"):
                    predicate_ident = ident
                    counts['spec'] += 1
                    in_predicate = True
                    if stripped.endswith("}"):
                        in_predicate = False

                elif in_predicate:
                    counts['spec'] += 1
                    if predicate_ident == ident:
                        in_predicate = False

                elif stripped.startswith("body_invariant!"):
                    counts['annot'] = counts['annot'] + 1
                    in_body_invariant = True

                    if stripped.endswith(");"):
                        in_body_invariant = False

                elif in_body_invariant:
                    counts['annot'] += 1
                    if stripped == ");":
                        in_body_invariant = False
                else:
                    counts['loc'] = counts['loc'] + 1
    return counts

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("glob")
    args = parser.parse_args()

    print(count_files(args.glob))
    
