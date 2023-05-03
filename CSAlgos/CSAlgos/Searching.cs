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
    public static int? Linear<T>(List<T> items, T item) where T : IEquatable<T> {
        for (int i = 0; i < items.Count; i++)
            if (items[i].Equals(item))
                return i;
        return null;
    } 
}
