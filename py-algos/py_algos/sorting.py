from typing import List


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
