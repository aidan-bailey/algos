namespace CSAlgosTest;
using CSAlgos;

[TestClass]
public class SearchingTests
{

    private static List<int> items = TestUtils.GenerateList(1000, false);

    public void SearchTemplate(Func<List<int>, int, int?> func)
    {
        Assert.AreEqual(func(items, items[0]), 0);
        Assert.AreEqual(func(items, items[items.Count - 1]), items.Count - 1);
        Assert.AreEqual(func(items, items[500]), 500);
        Assert.AreEqual(func(items, -1), null);
    }

    [TestMethod]
    public void LinearTest() => SearchTemplate(Searching.Linear<int>);
}
