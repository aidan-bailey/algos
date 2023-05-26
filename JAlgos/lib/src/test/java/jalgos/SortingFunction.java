package jalgos;

@FunctionalInterface
interface SortingFunction<Items, Result> {
    public Result sort(Items items);
}
