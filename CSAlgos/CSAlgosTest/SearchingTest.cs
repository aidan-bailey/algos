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

    [TestMethod]
    public void TernaryTest() => SearchTemplate(Searching.Ternary<int>);

    [TestMethod]
    public void K1aryTest() => SearchTemplate((items, item) => Searching.Kary<int>(1, items, item));

    [TestMethod]
    public void K2aryTest() => SearchTemplate((items, item) => Searching.Kary<int>(2, items, item));

    [TestMethod]
    public void K3aryTest() => SearchTemplate((items, item) => Searching.Kary<int>(3, items, item));

    [TestMethod]
    public void K5aryTest() => SearchTemplate((items, item) => Searching.Kary<int>(5, items, item));

    [TestMethod]
    public void K8aryTest() => SearchTemplate((items, item) => Searching.Kary<int>(8, items, item));

    [TestMethod]
    public void K13aryTest() => SearchTemplate((items, item) => Searching.Kary<int>(13, items, item));

}
