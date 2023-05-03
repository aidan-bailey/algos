namespace CSAlgosTest;
using CSAlgos;

[TestClass]
public class SearchingTests
{

    private static List<int> items = TestUtils.GenerateList(1000, false);

    public void SearchTemplate(Func<List<int>, int?> func) {

    }

    [TestMethod]
    public void LinearTest()
    {
        Assert.AreEqual(Searching.Linear(new List<int>{1, 2, 3}, 1), 0);
    }
}
