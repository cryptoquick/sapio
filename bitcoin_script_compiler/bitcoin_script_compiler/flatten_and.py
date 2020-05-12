from functools import singledispatchmethod
from typing import TYPE_CHECKING, List, Union

from .clause import (DNF, AfterClause, AndClause, CheckTemplateVerifyClause,
                     Clause, DNFClause, OrClause, PreImageCheckClause,
                     SatisfiedClause, SignatureCheckClause)

class FlattenPass:
    """
    Flattenpass takes a tree of clauses which have already been "normalized" and
    turns it into a DNF. E.g. OrClause(OrClause(a,b),AndClause(c,d)) ==> [[a],[b],[c,d]].

    FlattenPass checks that there is no OrClause which follows and AndClause, otherwise
    the flattening may only be shallow (and a true DNF would not be returned).
    """
    def __call__(self, arg: Clause, or_allowed: bool = True) -> DNF:
        if TYPE_CHECKING:
            assert callable(self.flatten)
        r: DNF = self.flatten(arg, or_allowed)
        return r

    @singledispatchmethod
    def flatten(self, arg: Clause, or_allowed: bool = True) -> DNF:
        raise NotImplementedError("Cannot Compile Arg", arg)

    @flatten.register
    def flatten_and(self, arg: AndClause, or_allowed: bool = False) -> DNF:
        if TYPE_CHECKING:
            assert callable(self.flatten)
        l: DNF = self.flatten(arg.a, or_allowed)
        l2: DNF = self.flatten(arg.b, or_allowed)
        assert len(l) == 1
        assert len(l2) == 1
        l[0].extend(l2[0])
        return l

    @flatten.register
    def flatten_sat(self, arg: SatisfiedClause, or_allowed: bool = False) -> DNF:
        return [[]]

    @flatten.register
    def flatten_or(self, arg: OrClause, or_allowed: bool = True) -> DNF:
        if TYPE_CHECKING:
            assert callable(self.flatten)
        assert or_allowed
        l: DNF = self.flatten(arg.a, or_allowed)
        l2: DNF = self.flatten(arg.b, or_allowed)
        return l + l2

    @flatten.register(AfterClause)
    @flatten.register(CheckTemplateVerifyClause)
    @flatten.register(PreImageCheckClause)
    @flatten.register(SignatureCheckClause)
    def flatten_after(
        self,
        arg: Union[
            AfterClause,
            CheckTemplateVerifyClause,
            PreImageCheckClause,
            SignatureCheckClause,
        ],
        or_allowed: bool = False,
    ) -> DNF:
        return [[arg]]


# TODO: Move to a test...
try:
    f = FlattenPass()
    assert callable(f.flatten)
    f.flatten(
        AndClause(
            OrClause(SatisfiedClause(), SatisfiedClause()),
            OrClause(SatisfiedClause(), SatisfiedClause()),
        )
    )
    raise AssertionError("this sanity check should fail")
except AssertionError:
    pass
