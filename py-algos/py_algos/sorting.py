from typing import Any, List

def swap(items: List[Any], i1: int, i2: int):
    temp = items[i1]
    items[i1] = items[i2]
    items[i2] = temp

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
                swap(items, insertion_index, insertion_index - 1)
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
        swap(items, insertion_index, min_index)

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

def quick(items: List[Any]) -> List[Any]:
    """Quicl Sort (Inplace)

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

    def partition(items, first, last):
        if (last - first + 1) < 2:
            return
        pivot = last
        from_left_index = first
        from_right_index = last
        while from_left_index < from_right_index:
            if items[from_left_index] > items[pivot]:
                if items[from_right_index] < items[pivot]:
                    swap(items, from_right_index, from_left_index)
                    swap(items, from_right_index, pivot)
                    pivot = from_right_index
                else:
                    swap(items, from_right_index, pivot)
                    pivot = from_right_index
                from_right_index -= 1
            else:
                from_left_index += 1
        if items[from_left_index] > items[pivot]:
            swap(items, pivot, from_left_index)
            pivot = from_left_index

        partition(items, first, pivot - 1)
        partition(items, pivot + 1, last)

    partition(items, 0, len(items) - 1)
    return items


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
