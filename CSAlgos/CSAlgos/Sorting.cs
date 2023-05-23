namespace CSAlgos;
public static class Sorting
{

    private static void Swap<T>(IList<T> items, int index1, int index2)
    {
        var tmp = items[index1];
        items[index1] = items[index2];
        items[index2] = tmp;
    }

    /**
     * Selection Sort (Inplace)
     *
     * @param items a list to sort
     * @return reference to list
     */
    public static IList<T> Selection<T>(IList<T> items) where T : IComparable<T>
    {
        if (items.Count < 2)
            return items;

        for (int insertionIndex = 0; insertionIndex < items.Count - 1; insertionIndex++)
        {
            var minIndex = insertionIndex;
            for (int searchIndex = insertionIndex + 1; searchIndex < items.Count; searchIndex++)
            {
                var comparison = items[searchIndex].CompareTo(items[minIndex]);
                if (comparison < 0)
                    minIndex = searchIndex;
            }
            Swap(items, minIndex, insertionIndex);
        }

        return items;
    }
}