package jalgos;

import java.util.List;
import java.util.Optional;

public class Searching {

    /**
     * Linear Search
     * 
     * @param <T>   the type of item
     * @param items the list of items to be search
     * @param item  the item to search for
     * @return index of item if found, empty if not
     */
    public static <T extends Comparable<T>> Optional<Integer> Linear(List<T> items, T item) {
        for (int i = 0; i < items.size(); i++) {
            if (items.get(i).equals(item)) {
                return Optional.of(i);
            }
        }
        return Optional.empty();
    }

    /**
     * Binary Search
     * 
     * @param <T>   the type of item
     * @param items the list of items to be search
     * @param item  the item to search for
     * @return index of item if found, empty if not
     */
    public static <T extends Comparable<T>> Optional<Integer> Binary(List<T> items, T item) {

        var l = 0;
        var r = items.size() - 1;

        while (l <= r) {
            var mid = l + (r - l) / 2;
            var comparison = item.compareTo(items.get(mid));
            if (comparison == 0)
                return Optional.of(mid);
            if (comparison < 0)
                r = mid - 1;
            else
                l = mid + 1;
        }

        return Optional.empty();
    }

}
