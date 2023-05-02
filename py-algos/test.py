import random
from functools import partial
from py_algos import searching, sorting

def get_list(n: int, sort: bool):
    result = [ random.randint(0, 2**32) for _ in range(n) ]
    if sort:
        result.sort()
    return result

test_size = 1000

def search_template(func):
    ####################
    ### Event Length ###
    ####################

    items = get_list(1000, True)

    # First item
    assert(func(items=items, item=items[0]) == 0)

    # Last item
    assert(func(items=items, item=items[-1]) == (len(items) - 1))

    # Exists
    assert(func(items=items, item=items[500]) == 500)

    # Not Exists
    assert(func(items=items, item=-1) == None)

def sort_template(func):

    items = get_list(10, False)
    items_sorted = sorted(items.copy())

    assert(func(items.copy()) == items_sorted)


def test_linear():
    search_template(searching.linear)

def test_binary():
    search_template(searching.binary)

def test_ternary():
    search_template(searching.ternary)

def test_1ary():
    func = partial(searching.kary, k=1)
    search_template(func)

def test_2ary():
    func = partial(searching.kary, k=2)
    search_template(func)

def test_3ary():
    func = partial(searching.kary, k=3)
    search_template(func)

def test_5ary():
    func = partial(searching.kary, k=5)
    search_template(func)

def test_8ary():
    func = partial(searching.kary, k=8)
    search_template(func)

def test_13ary():
    func = partial(searching.kary, k=13)
    search_template(func)

def test_insertion():
    sort_template(sorting.insertion)

def test_merge():
    sort_template(sorting.merge)

def test_selection():
    sort_template(sorting.selection)

def test_quick():
    sort_template(sorting.quick)

def test_bubble():
    sort_template(sorting.bubble)
