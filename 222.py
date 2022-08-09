from utility import *

# Pieces: [URF, URB, ULF, ULB, DRF, DRB, DLF], DLB fixed

U = Move(
    p = Permutation([3,1,4,2,5,6,7]),
    o = None,
    name="U"
)

U2 = copy(U) @ copy(U)
Ui = copy(U2) @ copy(U)

R = Move(
    p = Permutation([2,6,3,4,1,5,7]),
    o = Orientation([1,2,0,0,2,1,0],3),
    name="R"
)

R2 = copy(R) @ copy(R)
Ri = copy(R2) @ copy(R)

F = Move(
    p = Permutation([5,2,1,4,7,6,3]),
    o = Orientation([2,0,1,0,1,0,2],3),
    name="F"
)

F2 = copy(F) @ copy(F)
Fi = copy(F2) @ copy(F)





