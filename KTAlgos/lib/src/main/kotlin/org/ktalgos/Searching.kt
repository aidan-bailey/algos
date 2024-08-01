package org.ktalgos.searching

public class Searching {

    /**
     * Linear search
     *
     * @param <T> the type of item
     * @param needle the value to search for.
     * @param haystack the list to search in.
     * @return The index of the needle in the haystack, or null if not found.
     */
    fun <T : Comparable<T>> linear(needle: T, haystack: Iterable<T>): Int? {
        for ((i, v) in haystack.withIndex()) {
            if (v.equals(needle)) {
                return i
            }
        }
        return null
    }
}
