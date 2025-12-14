from stockfish import Stockfish
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


fen = safe_random_fen()
fen = "rnb2b1r/pp2kp2/6p1/2p1p1Pp/2P1n3/2QPB2B/qP2KP1P/RN4NR b - - 2 13"
minimal_fen = fen.split(" ")[0:2]
print("Random FEN:")
print(fen)

board = chess.Board(fen)

moves = set(reference_moves(board))

# print(f"\nLegal moves count: {len(moves)}")
# for move in moves:
#     print(move.from_square , move.to_square)

p = subprocess.Popen(
    ["C:/Learn/LearnRust/chess/target/release/chess.exe"],
    stdin=subprocess.PIPE,
    stdout=subprocess.PIPE,
    text=True
)

out, _ = p.communicate(minimal_fen[0] + " " + minimal_fen[1])
rust_moves = set(tuple(map(int, line.split())) for line in out.splitlines())

if moves != rust_moves:
    print("FEN:", fen)
    print("Missing:", moves - rust_moves)
    print("Extra:", rust_moves - moves)
    exit(1)

# stockfish = Stockfish(
#     path=r"C:/Program Files/stockfish/stockfish-windows-x86-64-avx2.exe",
#     parameters={
#         "Threads": 4,
#         "Minimum Thinking Time": 30,
#         "Skill Level": 20
#     }
# )

# stockfish.set_fen_position(
#     "r2q1rk1/pp1b1ppp/2np1n2/2p1p3/2P1P3/2NP1N2/PP1B1PPP/R2Q1RK1 w"
# )

# stockfish.move

# # Best move
# best_move = stockfish.get_best_move()
# print("Best move:", best_move)

# # Evaluation
# eval = stockfish.get_evaluation()
# print("Evaluation:", eval)