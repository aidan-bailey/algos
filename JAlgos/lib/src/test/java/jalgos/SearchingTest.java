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
        SearchTemplate((items, item) -> Searching.Linear(items, item));
    }

    @Test void binaryTest() {
        SearchTemplate((items, item) -> Searching.Binary(items, item));
    }

    @Test void ternaryTest() {
        SearchTemplate((items, item) -> Searching.Ternary(items, item));
    }

    @Test void k1aryTest() {
        SearchTemplate((items, item) -> Searching.Kary(1, items, item));
    }

    @Test void k2aryTest() {
        SearchTemplate((items, item) -> Searching.Kary(2, items, item));
    }

    @Test void k3aryTest() {
        SearchTemplate((items, item) -> Searching.Kary(3, items, item));
    }

    @Test void k5aryTest() {
        SearchTemplate((items, item) -> Searching.Kary(5, items, item));
    }

    @Test void k8aryTest() {
        SearchTemplate((items, item) -> Searching.Kary(8, items, item));
    }

    @Test void k13aryTest() {
        SearchTemplate((items, item) -> Searching.Kary(13, items, item));
    }
}
