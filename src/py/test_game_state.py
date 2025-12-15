import chess
import random
import subprocess

def random_stalemate_fen():
    while True:
        board = chess.Board()

        for _ in range(random.randint(5, 80)):
            if board.is_game_over():
                break
            board.push(random.choice(list(board.legal_moves)))

        if not board.is_stalemate():
            continue

        fen = board.fen()
        parts = fen.split()
        board2 = chess.Board(fen)

        # Reject castling positions
        if board2.has_kingside_castling_rights(chess.WHITE): continue
        if board2.has_queenside_castling_rights(chess.WHITE): continue
        if board2.has_kingside_castling_rights(chess.BLACK): continue
        if board2.has_queenside_castling_rights(chess.BLACK): continue

        # Reject promotions
        if any(chess.square_rank(sq) == 6 for sq in board2.pieces(chess.PAWN, chess.WHITE)):
            continue
        if any(chess.square_rank(sq) == 1 for sq in board2.pieces(chess.PAWN, chess.BLACK)):
            continue

        # Remove castling + en-passant
        parts[2] = "-"
        parts[3] = "-"

        return " ".join(parts)

def safe_random_fen():
    while True:
        board = chess.Board()

        for _ in range(random.randint(5, 50)):
            if board.is_game_over():
                break
            board.push(random.choice(list(board.legal_moves)))

        fen = board.fen()
        parts = fen.split()

        board2 = chess.Board(fen)

        # Reject castling positions
        if board2.has_kingside_castling_rights(chess.WHITE): continue
        if board2.has_queenside_castling_rights(chess.WHITE): continue
        if board2.has_kingside_castling_rights(chess.BLACK): continue
        if board2.has_queenside_castling_rights(chess.BLACK): continue

        # Reject promotions
        if any(chess.square_rank(sq) == 6 for sq in board2.pieces(chess.PAWN, chess.WHITE)):
            continue
        if any(chess.square_rank(sq) == 1 for sq in board2.pieces(chess.PAWN, chess.BLACK)):
            continue

        # Remove castling + en-passant
        parts[2] = "-"
        parts[3] = "-"

        return " ".join(parts)


def reference_game_state(board: chess.Board) -> str:
    if board.is_checkmate():
        return "CHECKMATE"
    if board.is_stalemate():
        return "STALEMATE"
    return "IN_PROGRESS"


counter = 0

while True:
    fen = safe_random_fen()
    board = chess.Board(fen)

    expected_state = reference_game_state(board)

    minimal_fen = fen.split(" ")[0:2]

    print("\nRandom FEN:")
    print(fen)
    print("Expected:", expected_state)

    p = subprocess.Popen(
        ["C:/Learn/LearnRust/chess/target/release/test_checkmate.exe"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )

    out, err = p.communicate(minimal_fen[0] + " " + minimal_fen[1])
    rust_state = out.strip()

    if rust_state != expected_state:
        print("\n❌ MISMATCH DETECTED")
        print("FEN:", fen)
        print("Python:", expected_state)
        print("Rust:", rust_state)
        exit(1)

    counter += 1
    print("✔ OK | Tests passed:", counter)
