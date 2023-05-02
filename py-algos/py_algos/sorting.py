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


def selection(items: List[Any]) -> List[Any]:
    """Selection Sort (Inplace)

    Parameters
    ----------
    items: List
        List to sort

    Returns
    -------
    Reference to sorted list.
    """
    if len(items) < 2:
        return items

    for insertion_index in range(len(items)):
        min_index = insertion_index
        for index in range(insertion_index + 1, len(items)):
            if items[index] < items[min_index]:
                min_index = index
        temp = items[insertion_index]
        items[insertion_index] = items[min_index]
        items[min_index] = temp

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


def bubble(items: List[Any]) -> List[Any]:
    """Bubble Sort (Inplace)

    Parameters
    ----------
    items: List
       List to sort

    Returns
    -------
    Reference to sorted list.
    """
    if len(items) < 2:
        return items

    issorted = False
    while not issorted:
        issorted = True
        for i in range(len(items) - 1):
            if items[i] > items[i + 1]:
                temp = items[i]
                items[i] = items[i + 1]
                items[i + 1] = temp
                issorted = False

    return items
