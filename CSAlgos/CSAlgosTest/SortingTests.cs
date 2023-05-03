namespace CSAlgosTest;
using CSAlgos;

[TestClass]
public class SortingTests
{
    [TestMethod]
    public void LinearTest()
    {
        Assert.AreEqual(Sorting.Linear(new List<int>{1, 2, 3}, 1), 0);
    }
}
