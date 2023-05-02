from typing import Any, List


def insertion(items: List) -> List:
    """Insertion Sort (inplace)

    Parameters
    ----------
    items: List
        List to sort

    Returns
    -------
    Reference to sorted list.
    """
    if len(items) == 0:
        return items

    for current_index in range(1, len(items)):
        for insertion_index in range(current_index, 0, -1):
            if items[insertion_index] < items[insertion_index - 1]:
                temp = items[insertion_index]
                items[insertion_index] = items[insertion_index - 1]
                items[insertion_index - 1] = temp
            else:
                break

    return items


def merge(items: List[Any]) -> List[Any]:
    """Merge Sort

    Parameters
    ----------
    items: List
        List to sort

    Returns
    -------
    Sorted list.
    """
    if len(items) < 2:
        return items.copy()

    m = len(items) // 2
    l_sorted = merge(items[:m])
    r_sorted = merge(items[m:])
    result = []

    while len(l_sorted) > 0 and len(r_sorted) > 0:
        if l_sorted[0] < r_sorted[0]:
            result.append(l_sorted.pop(0))
        else:
            result.append(r_sorted.pop(0))

    while len(l_sorted) > 0:
        result.append(l_sorted.pop(0))

    while len(r_sorted) > 0:
        result.append(r_sorted.pop(0))

    return result
