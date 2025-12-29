import chess
import chess.engine
import random
import subprocess


STOCKFISH_PATH = "C:\Program Files\stockfish\stockfish-windows-x86-64-avx2.exe"  # change if needed
ENGINE_PATH = "C:/Learn/LearnRust/chess/target/release/test_best_move.exe"


# --------------------------------------------------
# Safe random FEN (same constraints as your script)
# --------------------------------------------------
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

        # Reject castling rights
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
                parts[2] = "-"
                parts[3] = "-"
                return " ".join(parts)


# --------------------------------------------------
# Stockfish top-5 moves
# --------------------------------------------------
def stockfish_top5(board: chess.Board, engine, time_sec=3):
    infos = engine.analyse(
        board,
        chess.engine.Limit(time=time_sec),
        multipv=10
    )

    scored_moves = []

    for info in infos:
        pv = info.get("pv")
        score = info.get("score")

        if not pv or score is None:
            continue

        move = pv[0]

        # Enforce constraints
        if move.promotion is not None:
            continue
        if board.is_castling(move):
            continue
        if board.is_en_passant(move):
            continue

        eval_cp = score.pov(board.turn).score(mate_score=10_000)
        scored_moves.append((eval_cp, move))

    # Sort by evaluation (best first)
    scored_moves.sort(reverse=True, key=lambda x: x[0])

    # Keep top 5
    return [
        (move.from_square, move.to_square)
        for _, move in scored_moves[:3]
    ]

def stockfish_top_near_moves(
    board: chess.Board,
    engine,
    time_sec=3,
    max_moves=5,
    threshold_cp=60,  # <-- 0.60 pawn window
):
    infos = engine.analyse(
        board,
        chess.engine.Limit(time=time_sec),
        multipv=10
    )

    scored_moves = []

    for info in infos:
        pv = info.get("pv")
        score = info.get("score")

        if not pv or score is None:
            continue

        move = pv[0]

        # Enforce constraints
        if move.promotion is not None:
            continue
        if board.is_castling(move):
            continue
        if board.is_en_passant(move):
            continue

        eval_cp = score.pov(board.turn).score(mate_score=10_000)
        if eval_cp is None:
            continue

        scored_moves.append((eval_cp, move))

    if not scored_moves:
        return []

    # Sort best first
    scored_moves.sort(reverse=True, key=lambda x: x[0])

    best_eval = scored_moves[0][0]

    # Keep moves close to best evaluation
    near_moves = [
        (mv.from_square, mv.to_square)
        for eval_cp, mv in scored_moves
        if best_eval - eval_cp <= threshold_cp
    ]

    return near_moves[:max_moves]



# --------------------------------------------------
# Build Rust engine
# --------------------------------------------------
subprocess.run(
    ["cargo", "build", "--release"],
    cwd="C:/Learn/LearnRust/chess",
    check=True
)


# --------------------------------------------------
# Main test loop
# --------------------------------------------------
with chess.engine.SimpleEngine.popen_uci(STOCKFISH_PATH) as sf:
    counter = 0
    missed = 0

    while counter <= 100:
        print("Move number " , counter)
        fen = safe_random_fen()
        board = chess.Board(fen)

        top5 = stockfish_top_near_moves(board, sf)

        minimal_fen = fen.split(" ")[0:2]

        p = subprocess.Popen(
            [ENGINE_PATH],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            text=True
        )

        out, _ = p.communicate(fen)
        engine_move = tuple(map(int, out.strip().split()))

        print("FEN:", fen)
        print("Stockfish top-5:", top5)
        print("Engine move:", engine_move)

        if engine_move not in top5:
            print("❌ ENGINE MOVE OUTSIDE STOCKFISH TOP-5")
            missed += 1
            counter += 1
            # exit(1)

        print("✅ OK\n")
        counter += 1

print("Number Of tests: " , counter)
print("Number Of missed tests: " , missed)
print("percentage: " , (counter - missed) * 100 / counter)
