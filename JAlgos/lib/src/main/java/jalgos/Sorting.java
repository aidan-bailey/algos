package jalgos;

import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Type;
import java.util.ArrayList;
import java.util.List;
import java.util.stream.Collector;
import java.util.stream.Collectors;
import java.util.stream.StreamSupport;

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

    /**
     * Selection Sort (Inplace)
     * 
     * @param <T>   the type of the items
     * @param items the list of items to be sorted
     * @return reference to the sorted list
     */
    public static <T extends Comparable<T>> List<T> Selection(List<T> items) {
        if (items.size() < 2)
            return items;

        for (int insertionIndex = 0; insertionIndex < items.size() - 1; insertionIndex++) {
            var minIndex = insertionIndex;
            for (int selectionIndex = minIndex + 1; selectionIndex < items.size(); selectionIndex++) {
                var comparison = items.get(selectionIndex).compareTo(items.get(minIndex));
                if (comparison < 0) {
                    minIndex = selectionIndex;
                }
            }
            Swap(items, insertionIndex, minIndex);
        }
        return items;
    }

    /**
     * Merge Sort (Inplace)
     * 
     * @param <T>   the type of the items
     * @param items the list of items to be sorted
     * @return reference to the sorted list
     */
    public static <T extends Comparable<T>> List<T> Merge(List<T> items) {
        if (items.size() < 2)
            return items;

        var lSpliterator = items.spliterator();
        var rSpliterator = lSpliterator.trySplit();
        var lSorted = Merge(StreamSupport.stream(lSpliterator, false).collect(Collectors.toList()));
        var rSorted = Merge(StreamSupport.stream(rSpliterator, false).collect(Collectors.toList()));

        items.clear();

        while (!lSorted.isEmpty() && !rSorted.isEmpty()) {
            var comparison = lSorted.get(0).compareTo(rSorted.get(0));
            if (comparison < 0) {
                items.add(lSorted.get(0));
                lSorted.remove(0);
            } else if (comparison > 0) {
                items.add(rSorted.get(0));
                rSorted.remove(0);
            } else {
                items.add(lSorted.get(0));
                items.add(rSorted.get(0));
                lSorted.remove(0);
                rSorted.remove(0);
            }
        }

        items.addAll(lSorted);
        items.addAll(rSorted);

        return items;
    }

}
