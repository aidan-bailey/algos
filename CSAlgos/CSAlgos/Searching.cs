namespace CSAlgos;
public static class Searching
{
    /**
     * Linear Search
     *
     * @param items a list of items to search
     * @param item an item to search for
     * @return the index of the item if found, null if not
     */
    public static int? Linear<T>(List<T> items, T item) where T : IEquatable<T>
    {
        for (int i = 0; i < items.Count; i++)
            if (items[i].Equals(item))
                return i;
        return null;
    }

    /**
     * Binary Search
     *
     * @param items a list of items to search
     * @param item an item to search for
     * @return the index of the item if found, null if not
     */
    public static int? Binary<T>(List<T> items, T item) where T : IComparable<T>
    {
        if (items.Count == 0)
            return null;

        var l = 0;
        var r = items.Count - 1;

        while (l <= r) {
            var m = l + (r - l) / 2;
            var comparison = items[m].CompareTo(item);
            if (comparison == 0)
                return m;
            else if (comparison < 0)
                l = m + 1;
            else
                r = m - 1;
        }

        return null;
    }
}
