package jalgos;

import java.util.ArrayList;
import java.util.List;
import java.util.Random;

public class Util {

    public static List<Integer> GenerateList(int n, boolean sorted){
        var rnd = new Random();
        var result = new ArrayList<Integer>();
        for (int i = 0; i < n; i++)
            result.add(rnd.nextInt(2147483647));
        if (sorted)
            result.sort(null);
        return result;
    }

}
