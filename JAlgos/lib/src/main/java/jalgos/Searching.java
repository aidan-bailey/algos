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

    /**
     * Ternary Search
     * 
     * @param <T>   the type of item
     * @param items the list of items to be search
     * @param item  the item to search for
     * @return index of item if found, empty if not
     */
    public static <T extends Comparable<T>> Optional<Integer> Ternary(List<T> items, T item) {
        var l = 0;
        var r = items.size() - 1;

        while (l <= r) {
            var mid1 = l + (r - l) / 3;
            var mid2 = l + 2 * (r - l) / 3;
            var comparison1 = item.compareTo(items.get(mid1));
            if (comparison1 == 0)
                return Optional.of(mid1);
            var comparison2 = item.compareTo(items.get(mid2));
            if (comparison2 == 0)
                return Optional.of(mid2);
            if (comparison1 < 0)
                r = mid1 - 1;
            else if (comparison2 < 0) {
                l = mid1 + 1;
                r = mid2 - 1;
            } else {
                l = mid2 + 1;
            }
        }

        return Optional.empty();
    }

}
