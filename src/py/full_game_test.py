import chess
import chess.engine
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
STOCKFISH_ELO = 1800     # <<< CHANGE THIS
STOCKFISH_DEPTH = 10
ENGINE_PLAYS_WHITE = True
MOVE_DELAY_SEC = 0.2    # readability

# ==================================================
# RUN YOUR ENGINE FOR ONE MOVE
# ==================================================
def engine_move(board: chess.Board):
    minimal_fen = board.fen().split(" ")[0:2]
    fen_input = minimal_fen[0] + " " + minimal_fen[1]

    p = subprocess.Popen(
        [ENGINE_PATH],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )

    out, err = p.communicate(fen_input)
    if err:
        print("ENGINE STDERR:", err)

    frm, to = map(int, out.strip().split())
    return chess.Move(frm, to)

# ==================================================
# MAIN GAME LOOP
# ==================================================
def play_game():
    board = chess.Board()

    with chess.engine.SimpleEngine.popen_uci(STOCKFISH_PATH) as sf:
        # Enable ELO limiting
        sf.configure({
            "UCI_LimitStrength": True,
            "UCI_Elo": STOCKFISH_ELO
        })

        move_number = 1

        print(f"\n=== ENGINE vs STOCKFISH ({STOCKFISH_ELO} ELO) ===\n")

        while not board.is_game_over():
            print(board)
            print("FEN:", board.fen())

            # ------------------------------------------
            # ENGINE MOVE
            # ------------------------------------------
            if board.turn == chess.WHITE and ENGINE_PLAYS_WHITE or \
               board.turn == chess.BLACK and not ENGINE_PLAYS_WHITE:

                mv = engine_move(board)
                print(f"\nEngine plays: {mv.uci()}")
                board.push(mv)

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

            print("-" * 50)
            time.sleep(MOVE_DELAY_SEC)

            move_number += 1

        print("\n=== GAME OVER ===")
        print("Result:", board.result())
        print("Final FEN:", board.fen())

# ==================================================
# ENTRY POINT
# ==================================================
if __name__ == "__main__":
    play_game()
