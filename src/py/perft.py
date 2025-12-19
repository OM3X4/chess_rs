import chess
import time

def perft(board: chess.Board, depth: int , max_depth: int) -> int:
    # Base case
    if depth == max_depth:
        return 1

    moves = list(board.pseudo_legal_moves)

    print(f"{board.fen()} || {depth} , {len(moves)}")

    # Matches your Rust behavior:
    # if no moves, still count as 1 node
    if not moves:
        return 1

    nodes = 0

    for move in moves:
        before = board.fen()          # snapshot for assertion
        board.push(move)              # make_move

        nodes += perft(board, depth + 1 , max_depth)

        board.pop()                   # unmake_move
        assert board.fen() == before  # equivalent to assert_eq!(*self, before)

    return nodes



# start = time.perf_counter()

print(perft(chess.Board(), 0 , 3))

# end = time.perf_counter()
# print(f"Time: {end - start}")