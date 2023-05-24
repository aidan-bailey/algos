namespace CSAlgosTest;
using CSAlgos;

[TestClass]
public class SortingTests
{
    private static List<int> items = TestUtils.GenerateList(1000, false);

    public void SortTemplate(Func<IList<int>, IList<int>> func)
    {
        var itemsCopy = new List<int>(items);
        var itemsSorted = new List<int>(itemsCopy);
        itemsSorted.Sort();
        var result = func(itemsCopy);
        Assert.IsTrue(itemsSorted.SequenceEqual(result));
    }

    [TestMethod]
    public void InsertionTest() => SortTemplate(Sorting.Insertion<int>);

    [TestMethod]
    public void SelectionTest() => SortTemplate(Sorting.Selection<int>);

    [TestMethod]
    public void MergeTest() => SortTemplate(Sorting.Merge<int>);
}