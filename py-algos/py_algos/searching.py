def linear(items: list, item) -> int | None:
    """Linear Search

    Parameters
    ----------
    items: list
        Items to be searched through.

    item
        Item to be searched for.

    Returns
    -------
        Index of item if it's found, `None` if it's not.
    """
    if len(items) == 0:
        return None
    for i, l_item in enumerate(items):
        if l_item == item:
            return i
    return None


def binary(items: list, item) -> int | None:
    """Binary Search

    Parameters
    ----------
    items: list
        Items to be searched through.
    item
        Item to be searched for.

    Returns
    -------
        Index of item if it's found, `None` if it's not.
    """
    if len(items) == 0:
        return None

    l = 0
    r = len(items) - 1

    while l <= r:
        m = l + (r - l) // 2
        if items[m] == item:
            return m
        elif items[m] < item:
            l = m + 1
        elif m == 0:
            return None
        else:
            r = m - 1

    return None


def ternary(items: list, item) -> int | None:
    """Ternary Search

    Parameters
    ----------
    items: list
        Items to be searched through.
    item
        Item to be searched for.

    Returns
    -------
        Index of item if it's found, `None` if it's not.
    """
    if len(items) == 0:
        return None

    l = 0
    r = len(items) - 1

    while l <= r:
        m1 = l + (r - l) // 3
        m2 = r - (r - l) // 3
        if items[m1] == item:
            return m1
        elif items[m2] == item:
            return m2
        elif items[m2] < item:
            l = m2 + 1
        elif items[m1] < item:
            l = m1 + 1
            r = m2 - 1
        elif m1 == 0:
            return None
        else:
            r = m1 - 1

    return None

def kary(k: int, items: list, item) -> int | None:
    """Kary Search

    Parameters
    ----------
    k: int
        Arity of the search.
    items: List
        Items to be searched through.
    item
        Item to be searched for.

    Returns
    -------
        Index of item if it's found, `None` if it's not.
    """
    if len(items) == 0:
        return None

    l = 0
    r = len(items) - 1

    while l <= r:
        const_l = l
        const_r = r
        for i in range(1, k + 1):
            m = const_l + i * (const_r - const_l) // (k + 1)
            if (items[m] == item):
                return m
            elif (item < items[m]):
                if (m == 0):
                    return None
                r = m - 1
                break
            else:
                l = m + 1

    return None
