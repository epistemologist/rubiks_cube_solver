from math import factorial
from typing import *
from copy import copy
import tqdm as tqdm_

TIMING = False
tqdm = tqdm_.tqdm if TIMING else lambda x: x

# Most of this code from https://www.jaapsch.net/puzzles/compindx.htm

class Permutation:
    def __init__(self, perm: List[int]):
        self.pv = perm
        self.states = len(perm)
    def __repr__(self):
        return repr(self.pv)
    def __iter__(self):
        return iter(self.pv)
    def __getitem__(self, key):
        return self.pv[key]
    def __len__(self):
        return self.states
    def __eq__(self, other):
        return isinstance(other, Permutation) and self.pv == other.pv
    def max(self) -> int:
        return factorial(self.states)
    def __int__(self):
        N = self.states
        t = 0
        for i in range(1, N):
            t *= (N - i + 1)
            for j in range(i+1, N+1):
                if self.pv[i-1] > self.pv[j-1]:
                    t += 1
        return t
    def __matmul__(self, other):
        assert isinstance(other, Permutation) and len(other.pv) == len(self.pv)
        new_perm = [None for i in range(len(self.pv))]
        for i in range(len(self.pv)):
            new_perm[i] = self.pv[other.pv[i] - 1]
        self.pv = new_perm
        return self
    def __copy__(self):
        return self.__class__(self.pv)

def permutation_from_int(N: int, length: int) -> Permutation:
    assert N < factorial(length)
    perm = [1 for i in range(length)]
    for i in reversed(range(1, length)):
        perm[i-1] = 1 + (N % (length - i + 1))
        N //= length - i + 1
        for j in range(i+1, length+1):
            if perm[j-1] >= perm[i-1]: perm[j-1] += 1
    return Permutation(perm)

class Orientation:
    def __init__(self, ov: List[int], states):
        self.ov = ov
        self.states = states
    def __repr__(self):
        return repr(self.ov)
    def __iter__(self):
        return iter(self.ov)
    def __getitem__(self, key):
        return self.ov[key]
    def __eq__(self, other):
        return isinstance(other, Orientation) and self.ov == other.ov and self.states == other.states
    def size(self):
        return (len(self.ov), self.states)
    def max(self) -> int:
        return self.states ** len(self.ov)
    def __int__(self):
        return sum([i*self.states**n for n,i in enumerate(self.ov)])
    def __matmul__(self, other):
        assert isinstance(other, Orientation) and len(self.ov) == len(other.ov) and self.states == other.states
        self.ov = [(i+j) % self.states for i,j in zip(self.ov, other.ov)]
        return self
    def __copy__(self):
        return self.__class__(self.ov, self.states)

def orientation_from_int(N: int, length: int, orientations: int) -> Orientation:
    ov = [0 for i in range(length)]
    for i in range(length):
        ov[i] = N % orientations
        N //= orientations
    return Orientation(ov, orientations)

class Move:
    def __init__(self, p: Permutation, o: Optional[Orientation], name: Optional[str]):
        assert o is None or len(o.ov) == len(p.pv)
        self.o = o
        self.p = p
        self.name = name
    def __repr__(self):
        return self.name
    def __matmul__(self, other):
        print("mat_mul: self, other: ", self, (self.p, self.o),  other, (other.p, other.o))
        assert isinstance(other, Move) and len(self.p) == len(other.p)
        new_p = copy(self.p)
        new_p @= other.p
        self.p = new_p
        if self.o:
            if other.o is None:
                other.o = orientation_from_int(0, len(self.p), self.o.states)
            new_ov = [None for i in range(len(self.p))]
            for i in range(len(self.p)):
                new_ov[i] = self.o[other.p[i] - 1]
            self.o = Orientation(new_ov, self.o.states) @ other.o
#            self.o @= other.o
        return self
    def __eq__(self, other):
        return isinstance(other, Move) and self.p == other.p and self.o == other.o
    def __int__(self):
        return (int(self.o) if self.o else 0)*self.p.max()+int(self.p)


def gen_move_table(move: Move):
    p_table, o_table = [], []
    num_pieces = len(move.p)
    print("Calcuating permutation table...")
    # Calculate permutation table
    for i in tqdm(range(move.p.max())):
        p_tmp = permutation_from_int(i, num_pieces)
        new_p = copy(p_tmp) @ move.p
        p_table.append(int(new_p))
    print("Calculating orientation table...")
    # Calculate orientation table
    if move.o:
        num_orientations = move.o.states
        for i in tqdm(range(move.o.max())):
            o_tmp = orientation_from_int(i, num_pieces, num_orientations)
            new_ov = [o_tmp[move.p[j]-1] for j in range(num_pieces)]
            new_o = Orientation(new_ov, num_orientations) @ move.o
            o_table.append(int(new_o))
    return p_table, o_table

def move_from_int(N: int, num_orientations: int, num_pieces: int) -> Move:
    o_coord = N // factorial(num_pieces)
    p_coord = N % factorial(num_pieces)
    o = orientation_from_int(o_coord, num_pieces, num_orientations)
    p = permutation_from_int(p_coord, num_pieces)
    return Move(p, o)

# Without move tables
class PieceStateSlow:
    def __init__(self, num_pieces: int, num_orientations: int):
        p = permutation_from_int(0, num_pieces)
        o = orientation_from_int(0, num_pieces, num_orientations)
        self.state = Move(p, o, name="state")
    def __repr__(self):
        return f"p: {self.state.p}, o: {self.state.o}"
    def apply_move(self, move: Move):
        self.state @= move

# With move tables
class PieceStateFast:
    def __init__(self, num_pieces: int, num_orientations: int, moves: List[Move]):
        self.num_pieces = num_pieces
        self.num_orientations = num_orientations
        assert all([len(move.p.pv) == num_pieces for move in moves])
        assert all([move.o is None or move.o.states == num_orientations for move in moves])
        self.p = 0
        self.o = 0
        self.p_table = dict()
        self.o_table = dict()
        for move in moves:
            p_table, o_table = gen_move_table(move)
            self.p_table[move.name] = p_table
            self.o_table[move.name] = o_table
    def __repr__(self):
        p_ = permutation_from_int(self.p, self.num_pieces)
        o_ = orientation_from_int(self.o, self.num_pieces, self.num_orientations)
        return f"p: {p_.pv}, o: {o_.ov}"
    def apply_move(self, move: Move):
        self.p = self.p_table[move.name][self.p]
        self.o = self.o if not self.o_table[move.name] else self.o_table[move.name][self.o]


def sanity_test():
    # Test if permutation to int and inverse are actually inverses
    for i in range(factorial(7)):
        assert int(permutation_from_int(i, 7)) == i
    # Test if orientation to int and inverse are actually inverses
    for i in range(pow(6,4)):
        assert int(orientation_from_int(i,4,6)) == i

