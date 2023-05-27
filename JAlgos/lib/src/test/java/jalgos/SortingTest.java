package jalgos;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;

class SortingTest {

    private static List<Integer> items = Util.GenerateList(1000, false);

    public void SortTemplate(SortingFunction<List<Integer>, List<Integer>> func)
    {
        var itemsCopy = new LinkedList<Integer>(items);
        var itemsSorted = new ArrayList<Integer>(itemsCopy);
        itemsSorted.sort(null);
        var result = func.sort(itemsCopy);
        assertTrue(itemsSorted.equals(result));
    }

    @Test void insertionTest() {
        SortTemplate((items) -> Sorting.Insertion(items));
    }

    @Test void selectionTest() {
        SortTemplate((items) -> Sorting.Selection(items));
    }

    @Test void mergeTest() {
        SortTemplate((items) -> Sorting.Merge(items));
    }

    @Test void quickTest() {
        SortTemplate((items) -> Sorting.Quick(items));
    }
}
