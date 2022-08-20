from utility import *

# Pieces: [URF, URB, ULF, ULB, DRF, DRB, DLF], DLB fixed

U = Move(
    p = Permutation([3,1,4,2,5,6,7]),
    o = None,
    name="U"
)

U2 = copy(U) @ copy(U)
U2.name = "U2"
Ui = copy(U2) @ copy(U)
Ui.name = "U'"

R = Move(
    p = Permutation([2,6,3,4,1,5,7]),
    o = Orientation([1,2,0,0,2,1,0],3),
    name="R"
)

R2 = copy(R) @ copy(R)
R2.name = "R2"
Ri = copy(R2) @ copy(R)
Ri.name = "Ri"

F = Move(
    p = Permutation([5,2,1,4,7,6,3]),
    o = Orientation([2,0,1,0,1,0,2],3),
    name="F"
)

F2 = copy(F) @ copy(F)
F2.name = "F2"
Fi = copy(F2) @ copy(F)
Fi.name = "F'"


class Cube222:
    def __init__(self):
        self.corners = PieceStateFast(7, 3, [U, Ui, U2, F, Fi, F2, R, Ri, R2])
        self.moves = {
            "R": R,
            "R'": Ri,
            "R2": R2,
            "F": F,
            "F'": Fi,
            "F2": F2,
            "U": U,
            "U'": Ui,
            "U2": U2
        }
    def _apply_move(self, move: str):
        self.corners.apply_move(self.moves[move])
    def _apply_algorithm(self, moves: List[str]):
        for move in moves:
            self._apply_move(move)


t_perm = "R U R' U' R' F R2 U' R' U' R U R' F'".split()
cube = Cube222()
cube._apply_algorithm(t_perm)



