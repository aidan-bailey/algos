package jalgos;

@FunctionalInterface
interface SearchingFunction<Items, Item, Result> {
    public Result search(Items items, Item item);
}
