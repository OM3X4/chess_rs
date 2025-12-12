
export type Bitboard = bigint;

function deepCopyBitboards(bitboards: any): any {
    return {
        whitePawns: bitboards.whitePawns,
        whiteKnights: bitboards.whiteKnights,
        whiteBishops: bitboards.whiteBishops,
        whiteRooks: bitboards.whiteRooks,
        whiteQueens: bitboards.whiteQueens,
        whiteKing: bitboards.whiteKing,
        blackPawns: bitboards.blackPawns,
        blackKnights: bitboards.blackKnights,
        blackBishops: bitboards.blackBishops,
        blackRooks: bitboards.blackRooks,
        blackQueens: bitboards.blackQueens,
        blackKing: bitboards.blackKing,
    };
}
function stringifyBitBoards(bitboards: Record<string, bigint>, turn: 'w' | 'b'): string {
    const entries = Object.entries(bitboards).map(
        ([key, value]) => `${key}:${value.toString()}_${turn}`
    );
    return entries.join('|');
}

export interface Move {
    from: number; // index 0–63
    to: number;   // index 0–63
    capture?: boolean;
    promotion?: string; // optional for now
}

const RANK_4 = 0x00000000FF000000n;
const RANK_5 = 0x000000FF00000000n;
const RANK_2 = 0x000000000000FF00n;
const RANK_7 = 0x00FF000000000000n;
const RANK_8 = 0xFF00000000000000n;
const RANK_1 = 0x00000000000000FFn;

const FILE_A = 0x0101010101010101n;
const FILE_H = 0x8080808080808080n;


function generateBitBoardWithOnePiece(i: number): Bitboard {
    return 1n << BigInt(i);
}

function extractBits(bb: Bitboard): number[] {
    const bits = [];
    while (bb) {
        const lsb = bb & -bb;
        const index = Number(BigInt.asUintN(64, BigInt(Math.log2(Number(lsb)))));
        bits.push(index);
        bb &= ~lsb;
    }
    return bits;
}

function generateBishopMoves(square: number, ownPieces: Bitboard, enemyPieces: Bitboard): Move[] {
    const moves: Move[] = [];
    const blockers = ownPieces | enemyPieces;

    const add = (to: number) => {
        const toMask = generateBitBoardWithOnePiece(to);
        const capture = (enemyPieces & toMask) !== 0n;
        moves.push({ from: square, to, capture });
    };

    // NORTH EAST
    for (let to = square + 9; (to <= 63) && (to % 8) !== 0; to += 9) {
        const toMask = generateBitBoardWithOnePiece(to);
        if (ownPieces & toMask) break;
        add(to);
        if (blockers & toMask) break;
    }

    // NORTH WEST
    for (let to = square + 7; (to <= 63) && (to % 8) !== 7; to += 7) {
        const toMask = generateBitBoardWithOnePiece(to);
        if (ownPieces & toMask) break;
        add(to);
        if (blockers & toMask) break;
    }

    // SOUTH EAST
    for (let to = square - 7; (to >= 0) && (to % 8) !== 0; to -= 7) {
        const toMask = generateBitBoardWithOnePiece(to);
        if (ownPieces & toMask) break;
        add(to);
        if (blockers & toMask) break;
    }

    // SOUNT WEST
    for (let to = square - 9; (to >= 0) && (to % 8) !== 7; to -= 9) {
        const toMask = generateBitBoardWithOnePiece(to);
        if (ownPieces & toMask) break;
        add(to);
        if (blockers & toMask) break;
    }

    return moves;
}

function generateWhitePawnMoves(
    square: number,
    ownPieces: Bitboard,
    enemyPieces: Bitboard
) {
    const moves: Move[] = [];
    const blockers = ownPieces | enemyPieces;

    const add = (to: number) => {
        const toMask = generateBitBoardWithOnePiece(to);
        const capture = (enemyPieces & toMask) !== 0n;
        moves.push({ from: square, to, capture });
    }

    const pawnBit = generateBitBoardWithOnePiece(square);

    // Single and doulbe Push Forward
    if (square + 8 <= 63 && !(blockers & generateBitBoardWithOnePiece(square + 8))) {
        add(square + 8)
        if ((pawnBit & RANK_2) !== 0n) {
            if (square + 16 <= 63 && !(blockers & generateBitBoardWithOnePiece(square + 16))) {
                add(square + 16);
            }
        }
    }

    if (square + 7 <= 63 && (enemyPieces & generateBitBoardWithOnePiece(square + 7)) !== 0n && !(pawnBit & FILE_H)) {
        add(square + 7);
    }
    if (square + 9 <= 63 && (enemyPieces & generateBitBoardWithOnePiece(square + 9)) !== 0n && !(pawnBit & FILE_A)) {
        add(square + 9);
    }

    return moves
}

function generateRookMoves(
    square: number,
    ownPieces: Bitboard,
    enemyPieces: Bitboard
): Move[] {
    const moves: Move[] = [];
    const blockers = ownPieces | enemyPieces;

    const add = (to: number) => {
        const toMask = generateBitBoardWithOnePiece(to);
        const capture = (enemyPieces & toMask) !== 0n;
        moves.push({ from: square, to, capture });
    };

    // NORTH
    for (let to = square + 8; to <= 63; to += 8) {
        const toMask = generateBitBoardWithOnePiece(to);
        if (ownPieces & toMask) break;
        add(to);
        if (blockers & toMask) break;
    }

    // SOUTH
    for (let to = square - 8; to >= 0; to -= 8) {
        const toMask = generateBitBoardWithOnePiece(to);
        if (ownPieces & toMask) break;
        add(to);
        if (blockers & toMask) break;
    }

    // EAST (check file wrap)
    for (let to = square + 1; to % 8 !== 0; to++) {
        const toMask = generateBitBoardWithOnePiece(to);
        if (ownPieces & toMask) break;
        add(to);
        if (blockers & toMask) break;
    }

    // WEST
    for (let to = square - 1; to % 8 !== 7 && to >= 0; to--) {
        const toMask = generateBitBoardWithOnePiece(to);
        if (ownPieces & toMask) break;
        add(to);
        if (blockers & toMask) break;
    }

    return moves;
}

function generateKingMoves(
    square: number,
    ownPieces: Bitboard,
    enemyPieces: Bitboard
): Move[] {
    const moves: Move[] = [];
    const own = ownPieces;

    // Precompute king's possible move offsets
    const offsets = [8, -8, 1, -1, 9, 7, -7, -9];

    for (const offset of offsets) {
        const to = square + offset;

        // Make sure 'to' is on board (0..63)
        if (to < 0 || to > 63) continue;

        // Prevent wraparound for horizontal moves:
        // Check file difference between from and to
        const fromFile = square % 8;
        const toFile = to % 8;
        if (Math.abs(fromFile - toFile) > 1) continue;

        const toMask = 1n << BigInt(to);

        // Can't move onto your own piece
        if ((own & toMask) !== 0n) continue;

        // Capture if enemy piece, else normal move
        const capture = (enemyPieces & toMask) !== 0n;
        moves.push({ from: square, to, capture });
    }

    return moves;
}

function generateKnightMoves(
    knights: Bitboard,
    ownPieces: Bitboard,
    enemyPieces: Bitboard
): Move[] {
    const moves: Move[] = [];
    const knightSquares = extractBits(knights);

    for (const from of knightSquares) {
        const attacks = knightAttackTable[from] & ~ownPieces;
        for (const to of extractBits(attacks)) {
            const capture = (enemyPieces >> BigInt(to)) & 1n ? true : false;
            moves.push({ from, to, capture });
        }
    }

    return moves;
}


export class ChessGame3 {
    // Piece bitboards
    private BitBoards = {
        whitePawns: 0n,
        whiteKnights: 0n,
        whiteBishops: 0n,
        whiteRooks: 0n,
        whiteQueens: 0n,
        whiteKing: 0n,

        blackPawns: 0n,
        blackKnights: 0n,
        blackBishops: 0n,
        blackRooks: 0n,
        blackQueens: 0n,
        blackKing: 0n,
    }
    private prevBitBoards = { ...this.BitBoards };
    private memoTable = new Map<string, Move[]>();

    turn: 'w' | 'b' = 'w';

    constructor();
    constructor(fen: string);

    constructor(fen?: string) {
        this.resetToDefault();

        if (fen) {
            this.loadFEN(fen);
        }
    }


    resetToDefault() {
        // Setup white
        this.BitBoards.whitePawns = 0x000000000000FF00n; // Rank 2
        this.BitBoards.whiteRooks = 0x0000000000000081n; // a1, h1
        this.BitBoards.whiteKnights = 0x0000000000000042n; // b1, g1
        this.BitBoards.whiteBishops = 0x0000000000000024n; // c1, f1
        this.BitBoards.whiteQueens = 0x0000000000000008n; // d1
        this.BitBoards.whiteKing = 0x0000000000000010n; // e1

        // Setup black
        this.BitBoards.blackPawns = 0x00FF000000000000n; // Rank 7
        this.BitBoards.blackRooks = 0x8100000000000000n; // a8, h8
        this.BitBoards.blackKnights = 0x4200000000000000n; // b8, g8
        this.BitBoards.blackBishops = 0x2400000000000000n; // c8, f8
        this.BitBoards.blackQueens = 0x0800000000000000n; // d8
        this.BitBoards.blackKing = 0x1000000000000000n; // e8
    }
    resetToZero() {
        this.BitBoards.whitePawns = 0n;
        this.BitBoards.whiteKnights = 0n;
        this.BitBoards.whiteBishops = 0n;
        this.BitBoards.whiteRooks = 0n;
        this.BitBoards.whiteQueens = 0n;
        this.BitBoards.whiteKing = 0n;

        this.BitBoards.blackPawns = 0n;
        this.BitBoards.blackKnights = 0n;
        this.BitBoards.blackBishops = 0n;
        this.BitBoards.blackRooks = 0n;
        this.BitBoards.blackQueens = 0n;
        this.BitBoards.blackKing = 0n;
    }

    get allWhite(): Bitboard {
        return this.BitBoards.whitePawns | this.BitBoards.whiteKnights | this.BitBoards.whiteBishops |
            this.BitBoards.whiteRooks | this.BitBoards.whiteQueens | this.BitBoards.whiteKing;
    }

    get allBlack(): Bitboard {
        return this.BitBoards.blackPawns | this.BitBoards.blackKnights | this.BitBoards.blackBishops |
            this.BitBoards.blackRooks | this.BitBoards.blackQueens | this.BitBoards.blackKing;
    }

    get allPieces(): Bitboard {
        return this.allWhite | this.allBlack;
    }
    get empty(): Bitboard {
        return ~this.allPieces & 0xFFFFFFFFFFFFFFFFn; // 64-bit mask
    }


    public loadFEN(fen: string) {
        this.resetToZero()

        const pieceMap = {
            p: "blackPawns",
            r: "blackRooks",
            n: "blackKnights",
            b: "blackBishops",
            q: "blackQueens",
            k: "blackKing",

            P: "whitePawns",
            R: "whiteRooks",
            N: "whiteKnights",
            B: "whiteBishops",
            Q: "whiteQueens",
            K: "whiteKing",
        } as const;

        const [position, turn] = fen.split(" ");
        if (turn === "w" || turn === "b") {
            this.turn = turn;
        } else {
            this.turn = "w"; // default to white if invalid
        }

        const rows = position.split("/");

        for (let rank = 0; rank < 8; rank++) {
            let file = 0;
            for (const char of rows[rank]) {
                if (Number.isInteger(Number(char))) {
                    file += Number(char);
                } else {
                    const squareIndex = (7 - rank) * 8 + file;
                    const bit = generateBitBoardWithOnePiece(squareIndex);
                    if (char in pieceMap) {
                        const targetBoard = pieceMap[char as keyof typeof pieceMap];
                        this.BitBoards[targetBoard] |= bit;
                        file++;
                    }
                }
            }
        }

    }

    public getwhitePawnsMoves() {
        const moves: Move[] = [];

        const pawns = extractBits(this.BitBoards.whitePawns);
        for (const pawn of pawns) {
            moves.push(...generateWhitePawnMoves(pawn, this.allWhite, this.allBlack))
        }
        return moves
    }
    public getblackPawnsMoves() {
        const moves: Move[] = [];

        const singlePushTargets = (this.BitBoards.blackPawns >> 8n) & this.empty;
        const doublePushTargets = (singlePushTargets >> 8n) & this.empty & RANK_5;

        const notHFile = ~FILE_H;
        const notAFile = ~FILE_A;

        const pawnsEligibleForRightCapture = this.BitBoards.blackPawns & notHFile;
        const rightCaptureTargets = (pawnsEligibleForRightCapture >> 7n) & this.BitBoards.whitePawns;

        const pawnsEligibleForLeftCapture = this.BitBoards.blackPawns & notAFile;
        const leftCaptureTargets = (pawnsEligibleForLeftCapture >> 9n) & this.BitBoards.whitePawns;

        for (let to = 0; to < 64; to++) {
            const toMask = generateBitBoardWithOnePiece(to);
            if ((singlePushTargets & toMask) !== 0n) {
                const from = to + 8;
                moves.push({ from, to });
            } else if ((doublePushTargets & toMask) !== 0n) {
                const from = to + 16;
                moves.push({ from, to });
            } else if ((rightCaptureTargets & toMask) !== 0n) {
                const from = to + 7;
                moves.push({ from, to, capture: true });
            } else if ((leftCaptureTargets & toMask) !== 0n) {
                const from = to + 9;
                moves.push({ from, to, capture: true });
            }
        }

        return moves;
    }
    public getwhiteRooksMoves() {
        const moves: Move[] = [];

        const rocks = extractBits(this.BitBoards.whiteRooks);
        for (const rock of rocks) {
            moves.push(...generateRookMoves(rock, this.allWhite, this.allBlack));
        }
        return moves
    }
    public getBlackRocksMoves() {
        const moves: Move[] = [];

        const rocks = extractBits(this.BitBoards.blackRooks);
        for (const rock of rocks) {
            moves.push(...generateRookMoves(rock, this.allBlack, this.allWhite));
        }
        return moves
    }
    public getWhiteBishopMoves() {
        const moves: Move[] = [];

        const rocks = extractBits(this.BitBoards.whiteBishops);
        for (const rock of rocks) {
            moves.push(...generateBishopMoves(rock, this.allWhite, this.allBlack));
        }
        return moves
    }
    public getBlackBishopMoves() {
        const moves: Move[] = [];

        const rocks = extractBits(this.BitBoards.blackBishops);
        for (const rock of rocks) {
            moves.push(...generateBishopMoves(rock, this.allBlack, this.allWhite));
        }
        return moves
    }
    public getWhiteQueenMoves() {
        const moves: Move[] = [];

        const rocks = extractBits(this.BitBoards.whiteQueens);
        for (const rock of rocks) {
            moves.push(...generateBishopMoves(rock, this.allWhite, this.allBlack));
            moves.push(...generateRookMoves(rock, this.allWhite, this.allBlack));
        }
        return moves
    }
    public getBlackQueenMoves() {
        const moves: Move[] = [];

        const rocks = extractBits(this.BitBoards.blackQueens);
        for (const rock of rocks) {
            moves.push(...generateBishopMoves(rock, this.allBlack, this.allWhite));
            moves.push(...generateRookMoves(rock, this.allBlack, this.allWhite));
        }
        return moves
    }
    public getwhiteKingMoves() {
        return generateKingMoves(extractBits(this.BitBoards.whiteKing)[0], this.allWhite, this.allBlack);
    }
    public getBlackKingMoves() {
        return generateKingMoves(extractBits(this.BitBoards.blackKing)[0], this.allBlack, this.allWhite);
    }
    public getWhiteKnightMoves() {
        const moves: Move[] = [];
        moves.push(...generateKnightMoves(this.BitBoards.whiteKnights, this.allWhite, this.allBlack));
        return moves
    }
    public getBlackKnightMoves() {
        const moves: Move[] = [];
        moves.push(...generateKnightMoves(this.BitBoards.blackKnights, this.allBlack, this.allWhite));
        return moves
    }

    public generateMoves() {
        if (this.memoTable.has(stringifyBitBoards(this.BitBoards, this.turn))) {
            return this.memoTable.get(stringifyBitBoards(this.BitBoards, this.turn));
        }
        const moves: Move[] = [];
        if (this.turn == 'w') {
            const totalTimeStart = performance.now();

            const startPawns = performance.now();
            moves.push(...this.getwhitePawnsMoves());
            const pawnsTime = performance.now() - startPawns;

            const startRocks = performance.now();
            moves.push(...this.getwhiteRooksMoves());
            const rocksTime = performance.now() - startRocks;

            const startBishops = performance.now();
            moves.push(...this.getWhiteBishopMoves());
            const bishopsTime = performance.now() - startBishops;

            const startQueens = performance.now();
            moves.push(...this.getWhiteQueenMoves());
            const queensTime = performance.now() - startQueens;

            const startKings = performance.now();
            moves.push(...this.getwhiteKingMoves());
            const kingsTime = performance.now() - startKings;

            const startKnights = performance.now();
            moves.push(...this.getWhiteKnightMoves());
            const knightsTime = performance.now() - startKnights;

            const totalTime = performance.now() - totalTimeStart;

            // console.log("Percentage pawns: ", (pawnsTime / totalTime) * 100, "%");
            // console.log("Percentage rooks: ", (rocksTime / totalTime) * 100, "%");
            // console.log("Percentage bishops: ", (bishopsTime / totalTime) * 100, "%");
            // console.log("Percentage queens: ", (queensTime / totalTime) * 100, "%");
            // console.log("Percentage kings: ", (kingsTime / totalTime) * 100, "%");
            // console.log("Percentage knights: ", (knightsTime / totalTime) * 100, "%");
        } else {
            moves.push(...this.getblackPawnsMoves());
            moves.push(...this.getBlackRocksMoves());
            moves.push(...this.getBlackBishopMoves());
            moves.push(...this.getBlackQueenMoves());
            moves.push(...this.getBlackKingMoves());
            moves.push(...this.getBlackKnightMoves());
        }
        this.memoTable.set(stringifyBitBoards(this.BitBoards, this.turn), moves);
        return moves;
    }
    private generateMovesForBothSides() {
        const moves: Move[] = [];
        moves.push(...this.getwhitePawnsMoves());
        moves.push(...this.getwhiteRooksMoves());
        moves.push(...this.getWhiteBishopMoves());
        moves.push(...this.getWhiteQueenMoves());
        moves.push(...this.getwhiteKingMoves());
        moves.push(...this.getWhiteKnightMoves());
        moves.push(...this.getblackPawnsMoves());
        moves.push(...this.getBlackRocksMoves());
        moves.push(...this.getBlackBishopMoves());
        moves.push(...this.getBlackQueenMoves());
        moves.push(...this.getBlackKingMoves());
        moves.push(...this.getBlackKnightMoves());
        return moves;
    }

    public move({ from, to }: { from: number, to: number }) {
        const fromBit = generateBitBoardWithOnePiece(from);
        const toMask = generateBitBoardWithOnePiece(to); // mask with 0 at to, 1 everywhere elses
        this.prevBitBoards = deepCopyBitboards(this.BitBoards)

        this.BitBoards.whitePawns &= ~toMask;
        this.BitBoards.whiteKnights &= ~toMask;
        this.BitBoards.whiteBishops &= ~toMask;
        this.BitBoards.whiteRooks &= ~toMask;
        this.BitBoards.whiteQueens &= ~toMask;
        this.BitBoards.whiteKing &= ~toMask;

        this.BitBoards.blackPawns &= ~toMask;
        this.BitBoards.blackKnights &= ~toMask;
        this.BitBoards.blackBishops &= ~toMask;
        this.BitBoards.blackRooks &= ~toMask;
        this.BitBoards.blackQueens &= ~toMask;
        this.BitBoards.blackKing &= ~toMask;

        if (this.turn == 'w') {
            if (fromBit & this.BitBoards.whitePawns) {
                this.BitBoards.whitePawns = (this.BitBoards.whitePawns & ~fromBit) | toMask;
            } else if (fromBit & this.BitBoards.whiteRooks) {
                this.BitBoards.whiteRooks = (this.BitBoards.whiteRooks & ~fromBit) | toMask;
            } else if (fromBit & this.BitBoards.whiteKnights) {
                this.BitBoards.whiteKnights = (this.BitBoards.whiteKnights & ~fromBit) | toMask;
            } else if (fromBit & this.BitBoards.whiteBishops) {
                this.BitBoards.whiteBishops = (this.BitBoards.whiteBishops & ~fromBit) | toMask;
            } else if (fromBit & this.BitBoards.whiteQueens) {
                this.BitBoards.whiteQueens = (this.BitBoards.whiteQueens & ~fromBit) | toMask;
            } else if (fromBit & this.BitBoards.whiteKing) {
                this.BitBoards.whiteKing = (this.BitBoards.whiteKing & ~fromBit) | toMask;
            }
        } else {
            if (fromBit & this.BitBoards.blackPawns) {
                this.BitBoards.blackPawns = (this.BitBoards.blackPawns & ~fromBit) | toMask;
            } else if (fromBit & this.BitBoards.blackRooks) {
                this.BitBoards.blackRooks = (this.BitBoards.blackRooks & ~fromBit) | toMask;
            } else if (fromBit & this.BitBoards.blackKnights) {
                this.BitBoards.blackKnights = (this.BitBoards.blackKnights & ~fromBit) | toMask;
            } else if (fromBit & this.BitBoards.blackBishops) {
                this.BitBoards.blackBishops = (this.BitBoards.blackBishops & ~fromBit) | toMask;
            } else if (fromBit & this.BitBoards.blackQueens) {
                this.BitBoards.blackQueens = (this.BitBoards.blackQueens & ~fromBit) | toMask;
            } else if (fromBit & this.BitBoards.blackKing) {
                this.BitBoards.blackKing = (this.BitBoards.blackKing & ~fromBit) | toMask;
            }
        }

        return true;
    }

    public unmove() {
        this.BitBoards = deepCopyBitboards(this.prevBitBoards)
        this.turn = this.turn == 'w' ? 'b' : 'w'
    }

    public isAttacked(square: number) {
        let moves;
        if (this.memoTable.has(stringifyBitBoards(this.BitBoards, this.turn))) {
            moves = this.memoTable.get(stringifyBitBoards(this.BitBoards, this.turn));
        } else {
            moves = this.generateMoves()
        }
        return moves?.filter(move => move.to == square).length ?? 0 > 0 ? true : false
    }

    public getFEN() {
        const boardInStrings = new Array(64).fill('1');

        function fillPieces(indices: number[], symbol: string) {
            for (const sq of indices) {
                boardInStrings[sq] = symbol;
            }
        }

        fillPieces(extractBits(this.BitBoards.whitePawns), 'P');
        fillPieces(extractBits(this.BitBoards.whiteKnights), 'N');
        fillPieces(extractBits(this.BitBoards.whiteBishops), 'B');
        fillPieces(extractBits(this.BitBoards.whiteRooks), 'R');
        fillPieces(extractBits(this.BitBoards.whiteQueens), 'Q');
        fillPieces(extractBits(this.BitBoards.whiteKing), 'K');
        fillPieces(extractBits(this.BitBoards.blackPawns), 'p');
        fillPieces(extractBits(this.BitBoards.blackKnights), 'n');
        fillPieces(extractBits(this.BitBoards.blackBishops), 'b');
        fillPieces(extractBits(this.BitBoards.blackRooks), 'r');
        fillPieces(extractBits(this.BitBoards.blackQueens), 'q');
        fillPieces(extractBits(this.BitBoards.blackKing), 'k');

        let fen = '';

        for (let rank = 7; rank >= 0; rank--) {
            let emptyCount = 0;

            for (let file = 0; file < 8; file++) {
                const sq = rank * 8 + file;
                const piece = boardInStrings[sq];

                if (piece === '1') {
                    emptyCount++;
                } else {
                    if (emptyCount > 0) {
                        fen += emptyCount.toString();
                        emptyCount = 0;
                    }
                    fen += piece;
                }
            }

            // End of rank: flush any remaining empty squares
            if (emptyCount > 0) {
                fen += emptyCount.toString();
            }

            if (rank > 0) fen += '/';
        }

        return fen;
    }

    public printBoard() {
        const pieceMap: [bigint, string][] = [
            [this.BitBoards.whitePawns, 'P'],
            [this.BitBoards.whiteKnights, 'N'],
            [this.BitBoards.whiteBishops, 'B'],
            [this.BitBoards.whiteRooks, 'R'],
            [this.BitBoards.whiteQueens, 'Q'],
            [this.BitBoards.whiteKing, 'K'],
            [this.BitBoards.blackPawns, 'p'],
            [this.BitBoards.blackKnights, 'n'],
            [this.BitBoards.blackBishops, 'b'],
            [this.BitBoards.blackRooks, 'r'],
            [this.BitBoards.blackQueens, 'q'],
            [this.BitBoards.blackKing, 'k'],
        ];

        const files = '  a   b   c   d   e   f   g   h';

        const horizontalLine = '  +---+---+---+---+---+---+---+---+';

        console.log(files);
        console.log(horizontalLine);

        for (let rank = 7; rank >= 0; rank--) {
            let row = `${rank + 1} |`;

            for (let file = 0; file < 8; file++) {
                const square = rank * 8 + file;
                const bit = 1n << BigInt(square);

                let char = ' ';
                for (const [bb, symbol] of pieceMap) {
                    if ((bb & bit) !== 0n) {
                        char = symbol;
                        break;
                    }
                }

                row += ` ${char} |`;
            }

            console.log(row);
            console.log(horizontalLine);
        }

        console.log(files);
    }

}

export const knightAttackTable: bigint[] = [
    0x0000000000020400n, 0x0000000000050800n, 0x00000000000a1100n, 0x0000000000142200n,
    0x0000000000284400n, 0x0000000000508800n, 0x0000000000a01000n, 0x0000000000402000n,
    0x0000000002040004n, 0x0000000005080008n, 0x000000000a110011n, 0x0000000014220022n,
    0x0000000028440044n, 0x0000000050880088n, 0x00000000a0100010n, 0x0000000040200020n,
    0x0000000204000402n, 0x0000000508000805n, 0x0000000a1100110an, 0x0000001422002214n,
    0x0000002844004428n, 0x0000005088008850n, 0x000000a0100010a0n, 0x0000004020002040n,
    0x0000020400040200n, 0x0000050800080500n, 0x00000a1100110a00n, 0x0000142200221400n,
    0x0000284400442800n, 0x0000508800885000n, 0x0000a0100010a000n, 0x0000402000204000n,
    0x0002040004020000n, 0x0005080008050000n, 0x000a1100110a0000n, 0x0014220022140000n,
    0x0028440044280000n, 0x0050880088500000n, 0x00a0100010a00000n, 0x0040200020400000n,
    0x0204000402000000n, 0x0508000805000000n, 0x0a1100110a000000n, 0x1422002214000000n,
    0x2844004428000000n, 0x5088008850000000n, 0xa0100010a0000000n, 0x4020002040000000n,
    0x0400040200000000n, 0x0800080500000000n, 0x1100110a00000000n, 0x2200221400000000n,
    0x4400442800000000n, 0x8800885000000000n, 0x100010a000000000n, 0x2000204000000000n,
    0x0004020000000000n, 0x0008050000000000n, 0x00110a0000000000n, 0x0022140000000000n,
    0x0044280000000000n, 0x0088500000000000n, 0x0010a00000000000n, 0x0020400000000000n,
];

export const bishopRaysNE: bigint[] = [
    0x0000000000000000n, 0x0000000000000000n, 0x0000000000000000n, 0x0000000000000000n,
    0x0000000000000000n, 0x0000000000000000n, 0x0000000000000000n, 0x0000000000000000n,
    0x0000000000000200n, 0x0000000000000400n, 0x0000000000000800n, 0x0000000000001000n,
    0x0000000000002000n, 0x0000000000004000n, 0x0000000000008000n, 0x0000000000000000n,
    0x0000000000020400n, 0x0000000000040800n, 0x0000000000081000n, 0x0000000000102000n,
    0x0000000000204000n, 0x0000000000408000n, 0x0000000000810000n, 0x0000000000000000n,
    0x0000000002040800n, 0x0000000004081000n, 0x0000000008102000n, 0x0000000010204000n,
    0x0000000020408000n, 0x0000000040810000n, 0x0000000081020000n, 0x0000000000000000n,
    0x0000000204081000n, 0x0000000408102000n, 0x0000000810204000n, 0x0000001020408000n,
    0x0000002040810000n, 0x0000004081020000n, 0x0000008102040000n, 0x0000000000000000n,
    0x0000020408102000n, 0x0000040810204000n, 0x0000081020408000n, 0x0000102040810000n,
    0x0000204081020000n, 0x0000408102040000n, 0x0000810204080000n, 0x0000000000000000n,
    0x0002040810204000n, 0x0004081020408000n, 0x0008102040810000n, 0x0010204081020000n,
    0x0020408102040000n, 0x0040810204080000n, 0x0081020408000000n, 0x0000000000000000n,
    0x0204081020408000n, 0x0408102040810000n, 0x0810204081020000n, 0x1020408102040000n,
    0x2040810204080000n, 0x4081020408000000n, 0x8102040800000000n, 0x0000000000000000n,
];

export const bishopRaysNW: bigint[] = [
    0x0000000000000000n, 0x0000000000000002n, 0x0000000000000004n, 0x0000000000000008n,
    0x0000000000000010n, 0x0000000000000020n, 0x0000000000000040n, 0x0000000000000000n,
    0x0000000000000200n, 0x0000000000000402n, 0x0000000000000804n, 0x0000000000001008n,
    0x0000000000002010n, 0x0000000000004020n, 0x0000000000008000n, 0x0000000000000000n,
    0x0000000000020400n, 0x0000000000040802n, 0x0000000000081010n, 0x0000000000102010n,
    0x0000000000204020n, 0x0000000000408000n, 0x0000000000800000n, 0x0000000000000000n,
    0x0000000002040800n, 0x0000000004081010n, 0x0000000008102010n, 0x0000000010204020n,
    0x0000000020408000n, 0x0000000040800000n, 0x0000000080000000n, 0x0000000000000000n,
    0x0000000204081010n, 0x0000000408102010n, 0x0000000810204020n, 0x0000001020408000n,
    0x0000002040800000n, 0x0000004080000000n, 0x0000008000000000n, 0x0000000000000000n,
    0x0000020408102010n, 0x0000040810204020n, 0x0000081020408000n, 0x0000102040800000n,
    0x0000204080000000n, 0x0000408000000000n, 0x0000800000000000n, 0x0000000000000000n,
    0x0002040810204020n, 0x0004081020408000n, 0x0008102040800000n, 0x0010204080000000n,
    0x0020408000000000n, 0x0040800000000000n, 0x0080000000000000n, 0x0000000000000000n,
    0x0204081020408000n, 0x0408102040800000n, 0x0810204080000000n, 0x1020408000000000n,
    0x2040800000000000n, 0x4080000000000000n, 0x8000000000000000n, 0x0000000000000000n,
];

export const bishopRaysSE: bigint[] = [
    0x0000000000000000n, 0x0000000000000000n, 0x0000000000000000n, 0x0000000000000000n,
    0x0000000000000000n, 0x0000000000000000n, 0x0000000000000000n, 0x0000000000000000n,
    0x0000000000000001n, 0x0000000000000002n, 0x0000000000000004n, 0x0000000000000008n,
    0x0000000000000010n, 0x0000000000000020n, 0x0000000000000040n, 0x0000000000000000n,
    0x0000000000010200n, 0x0000000000020400n, 0x0000000000040800n, 0x0000000000081000n,
    0x0000000000102000n, 0x0000000000204000n, 0x0000000000408000n, 0x0000000000000000n,
    0x0000000001020400n, 0x0000000002040800n, 0x0000000004081000n, 0x0000000008102000n,
    0x0000000010204000n, 0x0000000020408000n, 0x0000000040810000n, 0x0000000000000000n,
    0x0000000102040800n, 0x0000000204081000n, 0x0000000408102000n, 0x0000000810204000n,
    0x0000001020408000n, 0x0000002040810000n, 0x0000004081020000n, 0x0000000000000000n,
    0x0000010204081000n, 0x0000020408102000n, 0x0000040810204000n, 0x0000081020408000n,
    0x0000102040810000n, 0x0000204081020000n, 0x0000408102040000n, 0x0000000000000000n,
    0x0001020408102000n, 0x0002040810204000n, 0x0004081020408000n, 0x0008102040810000n,
    0x0010204081020000n, 0x0020408102040000n, 0x0040810204080000n, 0x0000000000000000n,
    0x0102040810204000n, 0x0204081020408000n, 0x0408102040810000n, 0x0810204081020000n,
    0x1020408102040000n, 0x2040810204080000n, 0x4081020408000000n, 0x0000000000000000n,
];

export const bishopRaysSW: bigint[] = [
    0x0000000000000000n, 0x0000000000000000n, 0x0000000000000000n, 0x0000000000000000n,
    0x0000000000000000n, 0x0000000000000000n, 0x0000000000000000n, 0x0000000000000000n,
    0x0000000000000000n, 0x0000000000000001n, 0x0000000000000003n, 0x0000000000000006n,
    0x000000000000000cn, 0x0000000000000018n, 0x0000000000000030n, 0x0000000000000000n,
    0x0000000000000102n, 0x0000000000000306n, 0x000000000000070en, 0x0000000000000e1cn,
    0x0000000000001c38n, 0x0000000000003830n, 0x0000000000007000n, 0x0000000000000000n,
    0x0000000000010306n, 0x000000000003070en, 0x0000000000070e1cn, 0x00000000000e1c38n,
    0x00000000001c3870n, 0x0000000000383070n, 0x0000000000700000n, 0x0000000000000000n,
    0x000000000103070en, 0x0000000003070e1cn, 0x00000000070e1c38n, 0x000000000e1c3870n,
    0x000000001c3870e0n, 0x0000000038307060n, 0x0000000070000000n, 0x0000000000000000n,
    0x0000000103070e1cn, 0x00000003070e1c38n, 0x000000070e1c3870n, 0x0000000e1c3870e0n,
    0x0000001c3870e0c0n, 0x0000003830706040n, 0x0000007000000000n, 0x0000000000000000n,
    0x000001070e1c3870n, 0x0000030e1c3870e0n, 0x0000071c3870e0c0n, 0x00000e3870e0c080n,
    0x00001c3870e0c080n, 0x0000383070604000n, 0x0000700000000000n, 0x0000000000000000n,
    0x0000070e1c3870e0n, 0x00000e1c3870e0c0n, 0x00001c3870e0c080n, 0x0000383070604000n,
    0x0000700000000000n, 0x0000e00000000000n, 0x0000000000000000n, 0x0000000000000000n,
];