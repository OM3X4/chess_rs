import chess
import chess.engine
import chess.pgn
import subprocess
import time

# ==================================================
# PATHS (CHANGE IF NEEDED)
# ==================================================
STOCKFISH_PATH = r"C:\Program Files\stockfish\stockfish-windows-x86-64-avx2.exe"
ENGINE_PATH = r"C:/Learn/LearnRust/chess/target/release/test_best_move.exe"

# ==================================================
# CONFIG
# ==================================================
STOCKFISH_ELO = 1800
STOCKFISH_DEPTH = 10
ENGINE_PLAYS_WHITE = True
MOVE_DELAY_SEC = 0.2

# ==================================================
# STOCKFISH TOP-5
# ==================================================
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

        # Enforce same constraints as your engine
        if move.promotion is not None:
            continue
        if board.is_castling(move):
            continue
        if board.is_en_passant(move):
            continue

        eval_cp = score.pov(board.turn).score(mate_score=10_000)
        scored_moves.append((eval_cp, move))

    scored_moves.sort(reverse=True, key=lambda x: x[0])

    return [move.uci() for _, move in scored_moves[:5]]

# ==================================================
# RUN YOUR ENGINE FOR ONE MOVE
# ==================================================
def engine_move(board: chess.Board):
    start = time.time()

    # Minimal FEN: pieces + side to move
    fen_parts = board.fen().split(" ")
    fen_input = fen_parts[0] + " " + fen_parts[1]

    p = subprocess.Popen(
        [ENGINE_PATH],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )

    out, err = p.communicate(fen_input)
    if err.strip():
        print("ENGINE STDERR:", err)

    frm, to = map(int, out.strip().split())
    mv = chess.Move(frm, to)

    print(f"Engine took {time.time() - start:.3f}s")
    return mv

# ==================================================
# MAIN GAME LOOP + PGN
# ==================================================
def play_game():
    board = chess.Board()

    # ---------- PGN SETUP ----------
    game = chess.pgn.Game()
    game.headers["Event"] = "Engine vs Stockfish"
    game.headers["White"] = "YourEngine" if ENGINE_PLAYS_WHITE else "Stockfish"
    game.headers["Black"] = "Stockfish" if ENGINE_PLAYS_WHITE else "YourEngine"
    game.headers["Result"] = "*"

    node = game

    with chess.engine.SimpleEngine.popen_uci(STOCKFISH_PATH) as sf:
        sf.configure({
            "UCI_LimitStrength": True,
            "UCI_Elo": STOCKFISH_ELO
        })

        print(f"\n=== ENGINE vs STOCKFISH ({STOCKFISH_ELO} ELO) ===\n")

        while not board.is_game_over():
            print(board)
            print("FEN:", board.fen())

            # ------------------------------------------
            # ENGINE MOVE
            # ------------------------------------------
            if (board.turn == chess.WHITE and ENGINE_PLAYS_WHITE) or \
               (board.turn == chess.BLACK and not ENGINE_PLAYS_WHITE):

                top_5 = stockfish_top5(board, sf)

                mv = engine_move(board)
                print(f"\nEngine plays: {mv.uci()}")
                print("Stockfish top-5:", top_5)

                if mv.uci() not in top_5:
                    print("❌ MISMATCH DETECTED")

                board.push(mv)
                node = node.add_variation(mv)

            # ------------------------------------------
            # STOCKFISH MOVE
            # ------------------------------------------
            else:
                result = sf.play(
                    board,
                    chess.engine.Limit(depth=STOCKFISH_DEPTH)
                )
                mv = result.move
                print(f"\nStockfish plays: {mv.uci()}")

                board.push(mv)
                node = node.add_variation(mv)

            print("-" * 50)
            time.sleep(MOVE_DELAY_SEC)

        # ---------- FINALIZE PGN ----------
        game.headers["Result"] = board.result()

        print("\n=== GAME OVER ===")
        print("Result:", board.result())
        print("\n=== PGN ===\n")
        print(game)

        # Optional: save PGN
        with open("game.pgn", "w") as f:
            print(game, file=f)

# ==================================================
# BUILD ENGINE
# ==================================================
subprocess.run(
    ["cargo", "build", "--release"],
    cwd="C:/Learn/LearnRust/chess",
    check=True
)

# ==================================================
# ENTRY POINT
# ==================================================
if __name__ == "__main__":
    play_game()
