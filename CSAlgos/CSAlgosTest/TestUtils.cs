namespace CSAlgosTest;

public static class TestUtils
{
    public static List<int> GenerateList(int n, bool sorted){
        Random rnd = new Random();
        var result = new List<int>();
        for (int _ = 0; _ < n; _++)
            result.Add(rnd.Next(2147483647));
        if (sorted)
            result.Sort();
        return result;
    }
}
