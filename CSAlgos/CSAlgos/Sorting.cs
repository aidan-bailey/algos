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
     * Insertion Sort (Inplace)
     *
     * @param items a list to sort
     * @return reference to list
     */
    public static IList<T> Insertion<T>(IList<T> items) where T : IComparable<T>
    {
        if (items.Count < 2)
            return items;

        for (int currentIndex = 1; currentIndex < items.Count; currentIndex++)
            for (int insertionIndex = currentIndex; insertionIndex > 0; insertionIndex--)
            {
                var comparison = items[insertionIndex].CompareTo(items[insertionIndex - 1]);
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

    /**
     * Merge Sort
     *
     * @param items a list to sort
     * @return sorted list
     */
    public static IList<T> Merge<T>(IList<T> items) where T : IComparable<T>
    {
        if (items.Count < 2)
            return items;

        var mid = items.Count / 2;
        var lSortedItems = Merge(new List<T>(items.Take(mid)));
        var rSortedItems = Merge(new List<T>(items.TakeLast(items.Count - mid)));

        var sortedItems = new List<T>();

        while (lSortedItems.Count > 0 && rSortedItems.Count > 0)
        {
            var comparison = lSortedItems.First().CompareTo(rSortedItems.First());
            if (comparison < 0)
            {
                sortedItems.Add(lSortedItems.First());
                lSortedItems.RemoveAt(0);
            }
            else if (comparison > 0)
            {
                sortedItems.Add(rSortedItems.First());
                rSortedItems.RemoveAt(0);
            }
            else
            {
                sortedItems.Add(lSortedItems.First());
                sortedItems.Add(rSortedItems.First());
                lSortedItems.RemoveAt(0);
                rSortedItems.RemoveAt(0);
            }
        }
        sortedItems.AddRange(lSortedItems);
        sortedItems.AddRange(rSortedItems);
        return sortedItems;
    }

    /**
     * Quick Sort (Inplace)
     *
     * @param items a list to sort
     * @return reference to list
     */
    public static IList<T> Quick<T>(IList<T> items) where T : IComparable<T>
    {
        if (items.Count < 2)
            return items;

        void Partition(IList<T> items, int startIndex, int endIndex)
        {
            if ((endIndex - startIndex + 1) < 2)
                return;
            int pivot = endIndex;
            int lIndex = startIndex;
            int rIndex = pivot;
            while (lIndex < rIndex)
            {
                var lComparison = items[lIndex].CompareTo(items[pivot]);
                if (lComparison > 0){
                    var rComparison = items[rIndex].CompareTo(items[pivot]);
                    if (rComparison < 0) {
                        Swap(items, lIndex, rIndex);
                        Swap(items, rIndex, pivot);
                        pivot = rIndex;
                    }
                    rIndex--;
                }
                else
                    lIndex++;
            }
            var comparison = items[lIndex].CompareTo(items[pivot]);
            if (comparison > 0){
                Swap(items, pivot, lIndex);
                pivot = lIndex;
            }

            Partition(items, startIndex, pivot - 1);
            Partition(items, pivot + 1, endIndex);
        }

        Partition(items, 0, items.Count - 1);

        return items;
    }
}