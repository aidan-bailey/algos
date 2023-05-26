package jalgos;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

import java.util.ArrayList;
import java.util.List;

class SortingTest {

    private static List<Integer> items = Util.GenerateList(1000, false);

    public void SortTemplate(SortingFunction<List<Integer>, List<Integer>> func)
    {
        var itemsCopy = new ArrayList<Integer>(items);
        var itemsSorted = new ArrayList<Integer>(itemsCopy);
        itemsSorted.sort(null);
        var result = func.sort(itemsCopy);
        assertTrue(itemsSorted.equals(result));
    }

    @Test void insertionTest() {
        SortTemplate((items) -> Sorting.Insertion(items));
    }
}
