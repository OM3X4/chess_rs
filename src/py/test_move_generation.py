import chess
import random
import subprocess


def safe_random_fen():
    while True:
        board = chess.Board()
        for _ in range(random.randint(5, 40)):
            if board.is_game_over():
                break
            board.push(random.choice(list(board.legal_moves)))

        fen = board.fen()
        parts = fen.split()
        board2 = chess.Board(fen)

        if board2.has_kingside_castling_rights(chess.WHITE): continue
        if board2.has_queenside_castling_rights(chess.WHITE): continue
        if board2.has_kingside_castling_rights(chess.BLACK): continue
        if board2.has_queenside_castling_rights(chess.BLACK): continue

        # Reject promotions
        for sq in board2.pieces(chess.PAWN, chess.WHITE):
            if chess.square_rank(sq) == 6:
                break
        else:
            for sq in board2.pieces(chess.PAWN, chess.BLACK):
                if chess.square_rank(sq) == 1:
                    break
            else:
                # Remove castling + ep
                parts[2] = "-"
                parts[3] = "-"
                return " ".join(parts)

def reference_moves(board: chess.Board):
    ref = []

    for m in board.legal_moves:
        # Skip promotions
        if m.promotion is not None:
            continue

        # Skip castling
        if board.is_castling(m):
            continue

        # Skip en-passant
        if board.is_en_passant(m):
            continue

        ref.append((m.from_square, m.to_square))

    return ref

command = ["cargo", "build", "--release"]
try:
    # Use subprocess.run() to execute the command
    # cwd sets the current working directory for the command
    # check=True raises an exception if the command fails
    subprocess.run(
        command,
        cwd="C:/Learn/LearnRust/chess",
        check=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True # Decodes output as string
    )
    print("Cargo build successful!")
except subprocess.CalledProcessError as e:
    print("Cargo build failed!")
    print("Return code:", e.returncode)
    print("Output:", e.output)
    print("Error:", e.stderr)
    exit(1)



counter = 0
while True:
    fen = safe_random_fen()
    # fen = "rnb2b1r/pp2kp2/6p1/2p1p1Pp/2P1n3/2QPB2B/qP2KP1P/RN4NR b - - 2 13"
    # fen = "1rb3kr/p2p4/np6/2p1qppp/5PnP/PPPp4/1B2P1KR/RN3BN1 w - - 0 21"
    # fen = "2kr1bnB/pppN4/6p1/1N3p2/1np2P1r/P3Q3/4P1PP/1bK2BR1 w - - 2 20"
    # fen = "8/7n/3r1B1P/4Nk2/b7/5QB1/pKN1q1Pb/8 b - - 0 16"
    minimal_fen = fen.split(" ")[0:2]
    print("Random FEN:")
    print(fen)

    board = chess.Board(fen)

    moves = set(reference_moves(board))


    p = subprocess.Popen(
        ["C:/Learn/LearnRust/chess/target/release/py_api.exe"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        text=True
    )

    out, _ = p.communicate(minimal_fen[0] + " " + minimal_fen[1])
    rust_moves = set(tuple(map(int, line.split())) for line in out.splitlines())

    print(rust_moves)

    if moves != rust_moves:
        print("Counter:", counter)
        print("FEN:", fen)
        print("Missing:", moves - rust_moves)
        print("Extra:", rust_moves - moves)
        exit(1)
    counter += 1