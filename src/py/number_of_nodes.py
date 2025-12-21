import chess
import chess.engine

# ===============================
# CONFIGURATION
# ===============================
STOCKFISH_PATH = r"C:\Program Files\stockfish\stockfish-windows-x86-64-avx2.exe"

FEN = "2b2r2/rp1nb1p1/1q1p1n1k/p1p1Np2/1PQPp3/P1N1P3/2P2PPP/2RK1B1R w"
SEARCH_DEPTH = 30

# ===============================
# MAIN LOGIC
# ===============================
def analyze_fen(fen: str, depth: int):
    board = chess.Board(fen)

    with chess.engine.SimpleEngine.popen_uci(STOCKFISH_PATH) as engine:
        info = engine.analyse(
            board,
            chess.engine.Limit(depth=depth),
            info=chess.engine.INFO_ALL
        )

        best_move = info.get("pv")[0] if "pv" in info else None
        score = info.get("score")
        nodes = info.get("nodes")

        # Normalize evaluation
        if score is not None:
            score = score.white().score(mate_score=100000)

        print("FEN:", fen)
        print("Depth:", depth)
        print("Best move:", best_move)
        print("Evaluation (centipawns):", score)
        print("Searched nodes:", nodes)


if __name__ == "__main__":
    analyze_fen(FEN, SEARCH_DEPTH)
