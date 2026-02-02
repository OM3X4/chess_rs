**QueenFish** is a chess engine written entirely in Rust. It uses bitboards, magic move generation, and a deterministic alpha–beta search with pruning and caching to achieve high performance while remaining fully reproducible.

### Motivation
**QueenFish** was built as a deliberate learning project, exploring how competitive a complete, non-trivial chess engine can become when constructed from low-level board representation to UCI under controlled complexity growth.

### High-Level
The engine is centered around an alpha–beta search implemented in Negamax form, which coordinates all core subsystems. Legal move generation and validation are performed using magic bitboards, while Zobrist hashing is integrated into the search to cache previously evaluated positions and avoid redundant work.

Board state is maintained through incremental updates, allowing moves to be made and unmade with minimal overhead. Performance-critical structures are restored incrementally rather than reallocated, ensuring stability and efficiency at deeper search depths. Moves are encoded into a compact `u32` representation to reduce memory traffic during search.

The engine core is independent of external control and communicates with graphical user interfaces via the Universal Chess Interface (UCI), enabling consistent use across different environments.

# Search Design
QueenFish is built around [**Alpha–Beta pruning**](https://www.chessprogramming.org/Alpha-Beta) , using a [Negamax]() formulation. The search aggressively cuts branches once a move is proven inferior, allowing the engine to explore significantly deeper positions within a fixed time budget.

Search efficiency is driven primarily by **move ordering**. Moves are evaluated in the following priority:
1. **Transposition Table move** — the best move found at a previous depth for the same position.
2. **Captures**, ordered by **MVV-LVA**, to surface high-impact tactical moves early.
3. **Killer moves** — quiet moves that caused cutoffs in sibling nodes.
4. **Remaining quiet moves**.

This ordering maximizes early cutoffs and stabilizes principal variation selection across iterative deepening.

To further reduce the search tree, the engine applies [**Null Move Pruning**](https://www.chessprogramming.org/Null_Move_Pruning) and **Late Move Reductions ([LMR](https://www.chessprogramming.org/Late_Move_Reductions))** under carefully constrained conditions, trading depth for speed while preserving tactical reliability.

A [**quiescence search**](https://www.chessprogramming.org/Quiescence_Search) is performed at leaf nodes to mitigate the [horizon effect](https://www.chessprogramming.org/Horizon_Effect), extending the search through capture sequences until the position becomes tactically stable.

For move generation, QueenFish intentionally operates on [**pseudo-legal moves**](https://www.chessprogramming.org/Pseudo-Legal_Move) during search. Moves are generated without legality filtering, then validated by making the move and checking for self-check. Illegal moves are immediately undone and discarded. Since make/unmake operations are already required by the search, this approach avoids a separate legal-move pass and improves overall throughput without sacrificing correctness.

# Openings
QueenFish uses a **static opening book** embedded directly into the engine binary. The book was generated offline using Stockfish analysis and consists of approximately **65,000 positions**, each mapped to a best move. This approach provides strong early-game guidance while keeping the runtime engine logic simple, deterministic, and free from external dependencies.
# Performance
QueenFish achieves an estimated playing strength in the **1700–2000 Elo** range based on a sample of **300 games** played locally against **Stockfish 1800** (150 games as White, 150 as Black). All games were run on a **Ryzen 5 4650G** under identical time controls, providing a consistent and reproducible testing environment.

# Usage
Run
``` bash
cargo build --release
```

then run uci.exe in the target/release folder, you can communicate with it using [UCI](https://www.chessprogramming.org/UCI).

An Example on UCI
``` bash
PS C:\QueenFish> .\target\release\uci.exe
uci #user
id name QueenFish 2.0 [Egypt]
id author Omar Emad (om3x4)
option name UseTT type check default true
option name UseLMR type check default true
option name UseNullMove type check default true
option name UseQuiesense type check default true
option name UseMoveOrder type check default true
uciok
position fen r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1 #user
go
info depth 1 score cp -306 nodes 224940 time 34 pv c4c5
info depth 2 score cp -306 nodes 383742 time 58 pv c4c5
info depth 3 score cp -321 nodes 518773 time 80 pv c4c5
info depth 4 score cp -321 nodes 950964 time 143 pv c4c5
info depth 5 score cp -367 nodes 1932459 time 300 pv c4c5
info depth 6 score cp -377 nodes 3340221 time 520 pv c4c5
info depth 7 score cp -367 nodes 7281210 time 1167 pv c4c5
info depth 8 score cp -357 nodes 17790187 time 2733 pv c4c5
info depth 9 score cp -402 nodes 38908632 time 6518 pv c4c5
stop #user

bestmove c4c5
```





