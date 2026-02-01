import chess
import chess.engine

def stockfish_top(board: chess.Board, engine, time_sec=3 , number_of_moves=5):
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
        for _, move in scored_moves[:number_of_moves]
    ]


def stockfish_top_near_moves(
    board: chess.Board,
    engine,
    time_sec=3,
    max_moves=10,
    threshold_cp=1000,  # <-- 0.60 pawn window
):
    infos = engine.analyse(
        board,
        chess.engine.Limit(depth=10),
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


def perft(board: chess.Board, depth: int , max_depth: int , engine) -> int:
    if depth == max_depth:
        return 1

    moves_num = 2

    if depth < 4:
        moves_num = 5
    elif depth == 5:
        moves_num = 4
    elif depth == 6:
        moves_num = 3
    elif depth == 7:
        moves_num = 2


    moves = stockfish_top_near_moves(board, engine, time_sec=3, max_moves=10, threshold_cp=60)
    # moves = stockfish_top(board, engine, time_sec=0.2 , number_of_moves=moves_num)

    moves = [chess.Move(m[0], m[1]) for m in moves]
    print(f"{board.fen()} || {[m.uci() for m in moves[0:2]]}")
    nodes = 0

    for move in moves:
        board.push(move)
        nodes += perft(board, depth + 1 , max_depth , engine)
        board.pop()

    return nodes

STOCKFISH_PATH = "C:\Program Files\stockfish\stockfish-windows-x86-64-avx2.exe"  # change if needed
with chess.engine.SimpleEngine.popen_uci(STOCKFISH_PATH) as engine:
    board = chess.Board()



    # moves = stockfish_top_near_moves(board, engine, time_sec=3, max_moves=5, threshold_cp=10)

    print(perft(board , 0 , 6 , engine))

    # print(moves)


