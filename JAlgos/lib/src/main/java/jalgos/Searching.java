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

}
