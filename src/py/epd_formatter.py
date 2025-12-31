import re

def square_to_index(sq: str) -> int:
    file = ord(sq[0]) - ord('a')   # a-h -> 0-7
    rank = int(sq[1]) - 1          # 1-8 -> 0-7
    return rank * 8 + file


def parse_sts_to_indices(text: str):
    results = []

    for line in text.strip().splitlines():
        line = line.strip()
        if not line:
            continue

        # Extract FEN (before " bm ")
        fen = line.split(" bm ")[0].strip()

        # Extract c9 moves
        match = re.search(r'c9\s+"([^"]+)"', line)
        if not match:
            continue

        moves = []
        for m in match.group(1).split():
            if len(m) == 4:
                frm = square_to_index(m[:2])
                to = square_to_index(m[2:])
                moves.append((frm, to))

        results.append((fen, moves))

    return results


# -----------------------------
# Example usage
# -----------------------------
if __name__ == "__main__":
    with open("sts.epd", "r", encoding="utf-8") as f:
        text = f.read()

    data = parse_sts_to_indices(text)


    for fen, moves in data:
        moves_string = ", ".join(f"[{move[0]}, {move[1]}]" for move in moves)
        print("(")
        print(f"\t\"{fen}\" ,")
        print(f"\tvec![{moves_string}]")
        print("),")
        print()
