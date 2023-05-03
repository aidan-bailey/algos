namespace CSAlgosTest;
using CSAlgos;

[TestClass]
public class SearchingTests
{

    private static List<int> items = TestUtils.GenerateList(1000, true);

    public void SearchTemplate(Func<List<int>, int, int?> func)
    {
        Assert.AreEqual(0, func(items, items[0]));
        Assert.AreEqual(items.Count - 1, func(items, items[items.Count - 1]));
        Assert.AreEqual(500, func(items, items[500]));
        Assert.AreEqual(null, func(items, -1));
    }

    [TestMethod]
    public void LinearTest() => SearchTemplate(Searching.Linear<int>);

    [TestMethod]
    public void BinaryTest() => SearchTemplate(Searching.Binary<int>);
}
