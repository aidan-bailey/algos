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

        while (l <= r)
        {
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

    /**
     * Ternary Search
     *
     * @param items a list of items to search
     * @param item an item to search for
     * @return the index of the item if found, null if not
     */
    public static int? Ternary<T>(List<T> items, T item) where T : IComparable<T>
    {
        if (items.Count == 0)
            return null;

        var l = 0;
        var r = items.Count - 1;

        while (l <= r)
        {
            var m1 = l + (r - l) / 3;
            var m2 = r - (r - l) / 3;
            var m1Comparison = items[m1].CompareTo(item);
            var m2Comparison = items[m2].CompareTo(item);
            if (m1Comparison == 0)
                return m1;
            else if (m2Comparison == 0)
                return m2;
            else if (m2Comparison < 0)
                l = m2 + 1;
            else if (m1Comparison < 0)
            {
                l = m1 + 1;
                r = m2 - 1;
            }
            else
                r = m1 - 1;
        }

        return null;
    }

    /**
     * Kary Search
     *
     * @param k arity of the search
     * @param items a list of items to search
     * @param item an item to search for
     * @return the index of the item if found, null if not
     */
    public static int? Kary<T>(int k, List<T> items, T item) where T : IComparable<T>
    {
        if (items.Count == 0)
            return null;

        var l = 0;
        var r = items.Count - 1;

        while (l <= r)
        {
            var lConst = l;
            var rConst = r;
            for (int i = 1; i < k + 1; i++)
            {
                var m = lConst + i * (rConst - lConst) / (k + 1);
                var mComparison = items[m].CompareTo(item);
                if (mComparison == 0)
                    return m;
                else if (mComparison > 0)
                {
                    r = m - 1;
                    break;
                }
                else
                    l = m + 1;
            }
        }

        return null;
    }

}
