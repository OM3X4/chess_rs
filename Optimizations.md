### This is the list of optimization I made in this Engine

1. TT to cache scores of every position using zobrist hashing
2. Change zobrist hashing incrementally on every make move and save it in the unmake move object
3. Used bishop and rooks magics
4. Used Attack tables to generate moves for Knights , Kings , Pawns
5. Check for "Is King In Check" By generating every piece moves from king square and check if an actual enemy piece exist in these squares
6. Store bitboards in an fixed array size 12 and index them using PieceType::type.piece_index() , they are aligned
7. In AlphaBeta I use pesudo moves and check after making the move , (I make and unmake in already in AlphaBeta , so Don't have to repeat it in generate_legal_moves)
8. Use a reference to a mutable Vector instead of creating a vector in every function
9. Set the size of the Vector in the start
10. packed moves inside a single u32