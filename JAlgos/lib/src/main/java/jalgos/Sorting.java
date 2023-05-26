package jalgos;

import java.util.List;

public class Sorting {

    private static <T> void Swap(List<T> items, int index1, int index2) {
        var tmp = items.get(index1);
        items.set(index1, items.get(index2));
        items.set(index2, tmp);
    }

    /**
     * Insertion Sort (Inplace)
     * 
     * @param <T>   the type of the items
     * @param items the list of items to be sorted
     * @return reference to the sorted list
     */
    public static <T extends Comparable<T>> List<T> Insertion(List<T> items) {
        if (items.size() < 2)
            return items;

        for (int currentIndex = 1; currentIndex < items.size(); currentIndex++)
            for (int insertionIndex = currentIndex; insertionIndex > 0; insertionIndex--) {
                var comparison = items.get(insertionIndex).compareTo(items.get(insertionIndex - 1));
                if (comparison < 0)
                    Swap(items, insertionIndex, insertionIndex - 1);
                else
                    break;
            }
        return items;
    }

}
