package jalgos;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

import java.util.List;
import java.util.Optional;

class SearchingTest {

    private static List<Integer> items = Util.GenerateList(1000, true);

    public void SearchTemplate(SearchingFunction<List<Integer>, Integer, Optional<Integer>> func)
    {
        assertEquals(0, func.search(items, items.get(0)).get());
        assertEquals(items.size() - 1, func.search(items, items.get(items.size() - 1)).get());
        assertEquals(500, func.search(items, items.get(500)).get());
        assertTrue(func.search(items, -1).isEmpty());
    }

    @Test void linearTest() {
        SearchingFunction<List<Integer>, Integer, Optional<Integer>> func = (items, item) -> Searching.Linear(items, item);
        SearchTemplate(func);
    }

    @Test void binaryTest() {
        SearchingFunction<List<Integer>, Integer, Optional<Integer>> func = (items, item) -> Searching.Binary(items, item);
        SearchTemplate(func);
    }

    @Test void ternaryTest() {
        SearchingFunction<List<Integer>, Integer, Optional<Integer>> func = (items, item) -> Searching.Ternary(items, item);
        SearchTemplate(func);
    }
}
