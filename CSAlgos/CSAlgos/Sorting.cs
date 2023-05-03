namespace CSAlgos;
public static class Sorting
{
    public static int? Linear<T>(List<T> items, T item) where T : IEquatable<T> {
        for (int i = 0; i < items.Count; i++)
            if (items[i].Equals(item))
                return i;
        return null;
    } 
}
